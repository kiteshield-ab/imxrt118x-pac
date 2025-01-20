#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AckMode {
    #[doc = "Disable ACK mode."]
    DisableAck = 0x0,
    #[doc = "Enable ACK mode."]
    EnableAck = 0x01,
}
impl AckMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AckMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AckMode {
    #[inline(always)]
    fn from(val: u8) -> AckMode {
        AckMode::from_bits(val)
    }
}
impl From<AckMode> for u8 {
    #[inline(always)]
    fn from(val: AckMode) -> u8 {
        AckMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Busy {
    #[doc = "Current observe is not busy"]
    NonBusy = 0x0,
    #[doc = "Current observe is busy"]
    Busy = 0x01,
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
pub enum ClockRootAuthenTzNs {
    #[doc = "Cannot be changed in Non-secure mode."]
    NonsecNo = 0x0,
    #[doc = "Can be changed in Non-secure mode."]
    NonsecYes = 0x01,
}
impl ClockRootAuthenTzNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClockRootAuthenTzNs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClockRootAuthenTzNs {
    #[inline(always)]
    fn from(val: u8) -> ClockRootAuthenTzNs {
        ClockRootAuthenTzNs::from_bits(val)
    }
}
impl From<ClockRootAuthenTzNs> for u8 {
    #[inline(always)]
    fn from(val: ClockRootAuthenTzNs) -> u8 {
        ClockRootAuthenTzNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClockRootAuthenTzUser {
    #[doc = "Clock Root settings cannot be changed in user mode."]
    UsrmodeN0 = 0x0,
    #[doc = "Clock Root settings can be changed in user mode."]
    UsrmodeYes = 0x01,
}
impl ClockRootAuthenTzUser {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClockRootAuthenTzUser {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClockRootAuthenTzUser {
    #[inline(always)]
    fn from(val: u8) -> ClockRootAuthenTzUser {
        ClockRootAuthenTzUser::from_bits(val)
    }
}
impl From<ClockRootAuthenTzUser> for u8 {
    #[inline(always)]
    fn from(val: ClockRootAuthenTzUser) -> u8 {
        ClockRootAuthenTzUser::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClockRootControlOff {
    #[doc = "Clock root is enabled"]
    Enabled = 0x0,
    #[doc = "Clock root is disabled"]
    Disabled = 0x01,
}
impl ClockRootControlOff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClockRootControlOff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClockRootControlOff {
    #[inline(always)]
    fn from(val: u8) -> ClockRootControlOff {
        ClockRootControlOff::from_bits(val)
    }
}
impl From<ClockRootControlOff> for u8 {
    #[inline(always)]
    fn from(val: ClockRootControlOff) -> u8 {
        ClockRootControlOff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClockRootStatus0Off {
    #[doc = "Clock root is enabled"]
    Enabled = 0x0,
    #[doc = "Clock root is disabled"]
    Disabled = 0x01,
}
impl ClockRootStatus0Off {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClockRootStatus0Off {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClockRootStatus0Off {
    #[inline(always)]
    fn from(val: u8) -> ClockRootStatus0Off {
        ClockRootStatus0Off::from_bits(val)
    }
}
impl From<ClockRootStatus0Off> for u8 {
    #[inline(always)]
    fn from(val: ClockRootStatus0Off) -> u8 {
        ClockRootStatus0Off::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FreqMeasureDone {
    #[doc = "Frequency measurement is on-going or not started"]
    FreqMeasOn = 0x0,
    #[doc = "Frequency measurement is done."]
    FreqMeasDone = 0x01,
}
impl FreqMeasureDone {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FreqMeasureDone {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FreqMeasureDone {
    #[inline(always)]
    fn from(val: u8) -> FreqMeasureDone {
        FreqMeasureDone::from_bits(val)
    }
}
impl From<FreqMeasureDone> for u8 {
    #[inline(always)]
    fn from(val: FreqMeasureDone) -> u8 {
        FreqMeasureDone::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GprPrivateAuthenTzNs {
    #[doc = "Cannot be changed in Non-secure mode."]
    NonsecNo = 0x0,
    #[doc = "Can be changed in Non-secure mode."]
    NonsecYes = 0x01,
}
impl GprPrivateAuthenTzNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GprPrivateAuthenTzNs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GprPrivateAuthenTzNs {
    #[inline(always)]
    fn from(val: u8) -> GprPrivateAuthenTzNs {
        GprPrivateAuthenTzNs::from_bits(val)
    }
}
impl From<GprPrivateAuthenTzNs> for u8 {
    #[inline(always)]
    fn from(val: GprPrivateAuthenTzNs) -> u8 {
        GprPrivateAuthenTzNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GprPrivateAuthenTzUser {
    #[doc = "Registers of private GPR cannot be changed in user mode."]
    UsrModeNo = 0x0,
    #[doc = "Registers of private GPR can be changed in user mode."]
    UsrModeYes = 0x01,
}
impl GprPrivateAuthenTzUser {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GprPrivateAuthenTzUser {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GprPrivateAuthenTzUser {
    #[inline(always)]
    fn from(val: u8) -> GprPrivateAuthenTzUser {
        GprPrivateAuthenTzUser::from_bits(val)
    }
}
impl From<GprPrivateAuthenTzUser> for u8 {
    #[inline(always)]
    fn from(val: GprPrivateAuthenTzUser) -> u8 {
        GprPrivateAuthenTzUser::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GprShared0AuthenTzNs {
    #[doc = "Cannot be changed in Non-secure mode."]
    NonsecNo = 0x0,
    #[doc = "Can be changed in Non-secure mode."]
    NonsecYes = 0x01,
}
impl GprShared0AuthenTzNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GprShared0AuthenTzNs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GprShared0AuthenTzNs {
    #[inline(always)]
    fn from(val: u8) -> GprShared0AuthenTzNs {
        GprShared0AuthenTzNs::from_bits(val)
    }
}
impl From<GprShared0AuthenTzNs> for u8 {
    #[inline(always)]
    fn from(val: GprShared0AuthenTzNs) -> u8 {
        GprShared0AuthenTzNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GprShared0AuthenTzUser {
    #[doc = "Registers of shared GPR slice cannot be changed in user mode."]
    UsrModeNo = 0x0,
    #[doc = "Registers of shared GPR slice can be changed in user mode."]
    UsrModeYes = 0x01,
}
impl GprShared0AuthenTzUser {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GprShared0AuthenTzUser {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GprShared0AuthenTzUser {
    #[inline(always)]
    fn from(val: u8) -> GprShared0AuthenTzUser {
        GprShared0AuthenTzUser::from_bits(val)
    }
}
impl From<GprShared0AuthenTzUser> for u8 {
    #[inline(always)]
    fn from(val: GprShared0AuthenTzUser) -> u8 {
        GprShared0AuthenTzUser::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GprShared10AuthenTzNs {
    #[doc = "Cannot be changed in Non-secure mode."]
    NonsecNo = 0x0,
    #[doc = "Can be changed in Non-secure mode."]
    NonsecYes = 0x01,
}
impl GprShared10AuthenTzNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GprShared10AuthenTzNs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GprShared10AuthenTzNs {
    #[inline(always)]
    fn from(val: u8) -> GprShared10AuthenTzNs {
        GprShared10AuthenTzNs::from_bits(val)
    }
}
impl From<GprShared10AuthenTzNs> for u8 {
    #[inline(always)]
    fn from(val: GprShared10AuthenTzNs) -> u8 {
        GprShared10AuthenTzNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GprShared10AuthenTzUser {
    #[doc = "Registers of shared GPR slice cannot be changed in user mode."]
    UsrModeNo = 0x0,
    #[doc = "Registers of shared GPR slice can be changed in user mode."]
    UsrModeYes = 0x01,
}
impl GprShared10AuthenTzUser {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GprShared10AuthenTzUser {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GprShared10AuthenTzUser {
    #[inline(always)]
    fn from(val: u8) -> GprShared10AuthenTzUser {
        GprShared10AuthenTzUser::from_bits(val)
    }
}
impl From<GprShared10AuthenTzUser> for u8 {
    #[inline(always)]
    fn from(val: GprShared10AuthenTzUser) -> u8 {
        GprShared10AuthenTzUser::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GprShared11AuthenTzNs {
    #[doc = "Cannot be changed in Non-secure mode."]
    NonsecNo = 0x0,
    #[doc = "Can be changed in Non-secure mode."]
    NonsecYes = 0x01,
}
impl GprShared11AuthenTzNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GprShared11AuthenTzNs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GprShared11AuthenTzNs {
    #[inline(always)]
    fn from(val: u8) -> GprShared11AuthenTzNs {
        GprShared11AuthenTzNs::from_bits(val)
    }
}
impl From<GprShared11AuthenTzNs> for u8 {
    #[inline(always)]
    fn from(val: GprShared11AuthenTzNs) -> u8 {
        GprShared11AuthenTzNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GprShared11AuthenTzUser {
    #[doc = "Registers of shared GPR slice cannot be changed in user mode."]
    UsrModeNo = 0x0,
    #[doc = "Registers of shared GPR slice can be changed in user mode."]
    UsrModeYes = 0x01,
}
impl GprShared11AuthenTzUser {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GprShared11AuthenTzUser {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GprShared11AuthenTzUser {
    #[inline(always)]
    fn from(val: u8) -> GprShared11AuthenTzUser {
        GprShared11AuthenTzUser::from_bits(val)
    }
}
impl From<GprShared11AuthenTzUser> for u8 {
    #[inline(always)]
    fn from(val: GprShared11AuthenTzUser) -> u8 {
        GprShared11AuthenTzUser::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GprShared12AuthenTzNs {
    #[doc = "Cannot be changed in Non-secure mode."]
    NonsecNo = 0x0,
    #[doc = "Can be changed in Non-secure mode."]
    NonsecYes = 0x01,
}
impl GprShared12AuthenTzNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GprShared12AuthenTzNs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GprShared12AuthenTzNs {
    #[inline(always)]
    fn from(val: u8) -> GprShared12AuthenTzNs {
        GprShared12AuthenTzNs::from_bits(val)
    }
}
impl From<GprShared12AuthenTzNs> for u8 {
    #[inline(always)]
    fn from(val: GprShared12AuthenTzNs) -> u8 {
        GprShared12AuthenTzNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GprShared12AuthenTzUser {
    #[doc = "Registers of shared GPR slice cannot be changed in user mode."]
    UsrModeNo = 0x0,
    #[doc = "Registers of shared GPR slice can be changed in user mode."]
    UsrModeYes = 0x01,
}
impl GprShared12AuthenTzUser {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GprShared12AuthenTzUser {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GprShared12AuthenTzUser {
    #[inline(always)]
    fn from(val: u8) -> GprShared12AuthenTzUser {
        GprShared12AuthenTzUser::from_bits(val)
    }
}
impl From<GprShared12AuthenTzUser> for u8 {
    #[inline(always)]
    fn from(val: GprShared12AuthenTzUser) -> u8 {
        GprShared12AuthenTzUser::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GprShared13AuthenTzNs {
    #[doc = "Cannot be changed in Non-secure mode."]
    NonsecNo = 0x0,
    #[doc = "Can be changed in Non-secure mode."]
    NonsecYes = 0x01,
}
impl GprShared13AuthenTzNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GprShared13AuthenTzNs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GprShared13AuthenTzNs {
    #[inline(always)]
    fn from(val: u8) -> GprShared13AuthenTzNs {
        GprShared13AuthenTzNs::from_bits(val)
    }
}
impl From<GprShared13AuthenTzNs> for u8 {
    #[inline(always)]
    fn from(val: GprShared13AuthenTzNs) -> u8 {
        GprShared13AuthenTzNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GprShared13AuthenTzUser {
    #[doc = "Registers of shared GPR slice cannot be changed in user mode."]
    UsrModeNo = 0x0,
    #[doc = "Registers of shared GPR slice can be changed in user mode."]
    UsrModeYes = 0x01,
}
impl GprShared13AuthenTzUser {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GprShared13AuthenTzUser {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GprShared13AuthenTzUser {
    #[inline(always)]
    fn from(val: u8) -> GprShared13AuthenTzUser {
        GprShared13AuthenTzUser::from_bits(val)
    }
}
impl From<GprShared13AuthenTzUser> for u8 {
    #[inline(always)]
    fn from(val: GprShared13AuthenTzUser) -> u8 {
        GprShared13AuthenTzUser::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GprShared14AuthenTzNs {
    #[doc = "Cannot be changed in Non-secure mode."]
    NonsecNo = 0x0,
    #[doc = "Can be changed in Non-secure mode."]
    NonsecYes = 0x01,
}
impl GprShared14AuthenTzNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GprShared14AuthenTzNs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GprShared14AuthenTzNs {
    #[inline(always)]
    fn from(val: u8) -> GprShared14AuthenTzNs {
        GprShared14AuthenTzNs::from_bits(val)
    }
}
impl From<GprShared14AuthenTzNs> for u8 {
    #[inline(always)]
    fn from(val: GprShared14AuthenTzNs) -> u8 {
        GprShared14AuthenTzNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GprShared14AuthenTzUser {
    #[doc = "Registers of shared GPR slice cannot be changed in user mode."]
    UsrModeNo = 0x0,
    #[doc = "Registers of shared GPR slice can be changed in user mode."]
    UsrModeYes = 0x01,
}
impl GprShared14AuthenTzUser {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GprShared14AuthenTzUser {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GprShared14AuthenTzUser {
    #[inline(always)]
    fn from(val: u8) -> GprShared14AuthenTzUser {
        GprShared14AuthenTzUser::from_bits(val)
    }
}
impl From<GprShared14AuthenTzUser> for u8 {
    #[inline(always)]
    fn from(val: GprShared14AuthenTzUser) -> u8 {
        GprShared14AuthenTzUser::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GprShared15AuthenTzNs {
    #[doc = "Cannot be changed in Non-secure mode."]
    NonsecNo = 0x0,
    #[doc = "Can be changed in Non-secure mode."]
    NonsecYes = 0x01,
}
impl GprShared15AuthenTzNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GprShared15AuthenTzNs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GprShared15AuthenTzNs {
    #[inline(always)]
    fn from(val: u8) -> GprShared15AuthenTzNs {
        GprShared15AuthenTzNs::from_bits(val)
    }
}
impl From<GprShared15AuthenTzNs> for u8 {
    #[inline(always)]
    fn from(val: GprShared15AuthenTzNs) -> u8 {
        GprShared15AuthenTzNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GprShared15AuthenTzUser {
    #[doc = "Registers of shared GPR slice cannot be changed in user mode."]
    UsrModeNo = 0x0,
    #[doc = "Registers of shared GPR slice can be changed in user mode."]
    UsrModeYes = 0x01,
}
impl GprShared15AuthenTzUser {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GprShared15AuthenTzUser {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GprShared15AuthenTzUser {
    #[inline(always)]
    fn from(val: u8) -> GprShared15AuthenTzUser {
        GprShared15AuthenTzUser::from_bits(val)
    }
}
impl From<GprShared15AuthenTzUser> for u8 {
    #[inline(always)]
    fn from(val: GprShared15AuthenTzUser) -> u8 {
        GprShared15AuthenTzUser::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GprShared1AuthenTzNs {
    #[doc = "Cannot be changed in Non-secure mode."]
    NonsecNo = 0x0,
    #[doc = "Can be changed in Non-secure mode."]
    NonsecYes = 0x01,
}
impl GprShared1AuthenTzNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GprShared1AuthenTzNs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GprShared1AuthenTzNs {
    #[inline(always)]
    fn from(val: u8) -> GprShared1AuthenTzNs {
        GprShared1AuthenTzNs::from_bits(val)
    }
}
impl From<GprShared1AuthenTzNs> for u8 {
    #[inline(always)]
    fn from(val: GprShared1AuthenTzNs) -> u8 {
        GprShared1AuthenTzNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GprShared1AuthenTzUser {
    #[doc = "Registers of shared GPR slice cannot be changed in user mode."]
    UsrModeNo = 0x0,
    #[doc = "Registers of shared GPR slice can be changed in user mode."]
    UsrModeYes = 0x01,
}
impl GprShared1AuthenTzUser {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GprShared1AuthenTzUser {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GprShared1AuthenTzUser {
    #[inline(always)]
    fn from(val: u8) -> GprShared1AuthenTzUser {
        GprShared1AuthenTzUser::from_bits(val)
    }
}
impl From<GprShared1AuthenTzUser> for u8 {
    #[inline(always)]
    fn from(val: GprShared1AuthenTzUser) -> u8 {
        GprShared1AuthenTzUser::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GprShared2AuthenTzNs {
    #[doc = "Cannot be changed in Non-secure mode."]
    NonsecNo = 0x0,
    #[doc = "Can be changed in Non-secure mode."]
    NonsecYes = 0x01,
}
impl GprShared2AuthenTzNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GprShared2AuthenTzNs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GprShared2AuthenTzNs {
    #[inline(always)]
    fn from(val: u8) -> GprShared2AuthenTzNs {
        GprShared2AuthenTzNs::from_bits(val)
    }
}
impl From<GprShared2AuthenTzNs> for u8 {
    #[inline(always)]
    fn from(val: GprShared2AuthenTzNs) -> u8 {
        GprShared2AuthenTzNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GprShared2AuthenTzUser {
    #[doc = "Registers of shared GPR slice cannot be changed in user mode."]
    UsrModeNo = 0x0,
    #[doc = "Registers of shared GPR slice can be changed in user mode."]
    UsrModeYes = 0x01,
}
impl GprShared2AuthenTzUser {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GprShared2AuthenTzUser {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GprShared2AuthenTzUser {
    #[inline(always)]
    fn from(val: u8) -> GprShared2AuthenTzUser {
        GprShared2AuthenTzUser::from_bits(val)
    }
}
impl From<GprShared2AuthenTzUser> for u8 {
    #[inline(always)]
    fn from(val: GprShared2AuthenTzUser) -> u8 {
        GprShared2AuthenTzUser::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GprShared3AuthenTzNs {
    #[doc = "Cannot be changed in Non-secure mode."]
    NonsecNo = 0x0,
    #[doc = "Can be changed in Non-secure mode."]
    NonsecYes = 0x01,
}
impl GprShared3AuthenTzNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GprShared3AuthenTzNs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GprShared3AuthenTzNs {
    #[inline(always)]
    fn from(val: u8) -> GprShared3AuthenTzNs {
        GprShared3AuthenTzNs::from_bits(val)
    }
}
impl From<GprShared3AuthenTzNs> for u8 {
    #[inline(always)]
    fn from(val: GprShared3AuthenTzNs) -> u8 {
        GprShared3AuthenTzNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GprShared3AuthenTzUser {
    #[doc = "Registers of shared GPR slice cannot be changed in user mode."]
    UsrModeNo = 0x0,
    #[doc = "Registers of shared GPR slice can be changed in user mode."]
    UsrModeYes = 0x01,
}
impl GprShared3AuthenTzUser {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GprShared3AuthenTzUser {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GprShared3AuthenTzUser {
    #[inline(always)]
    fn from(val: u8) -> GprShared3AuthenTzUser {
        GprShared3AuthenTzUser::from_bits(val)
    }
}
impl From<GprShared3AuthenTzUser> for u8 {
    #[inline(always)]
    fn from(val: GprShared3AuthenTzUser) -> u8 {
        GprShared3AuthenTzUser::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GprShared4AuthenTzNs {
    #[doc = "Cannot be changed in Non-secure mode."]
    NonsecNo = 0x0,
    #[doc = "Can be changed in Non-secure mode."]
    NonsecYes = 0x01,
}
impl GprShared4AuthenTzNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GprShared4AuthenTzNs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GprShared4AuthenTzNs {
    #[inline(always)]
    fn from(val: u8) -> GprShared4AuthenTzNs {
        GprShared4AuthenTzNs::from_bits(val)
    }
}
impl From<GprShared4AuthenTzNs> for u8 {
    #[inline(always)]
    fn from(val: GprShared4AuthenTzNs) -> u8 {
        GprShared4AuthenTzNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GprShared4AuthenTzUser {
    #[doc = "Registers of shared GPR slice cannot be changed in user mode."]
    UsrModeNo = 0x0,
    #[doc = "Registers of shared GPR slice can be changed in user mode."]
    UsrModeYes = 0x01,
}
impl GprShared4AuthenTzUser {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GprShared4AuthenTzUser {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GprShared4AuthenTzUser {
    #[inline(always)]
    fn from(val: u8) -> GprShared4AuthenTzUser {
        GprShared4AuthenTzUser::from_bits(val)
    }
}
impl From<GprShared4AuthenTzUser> for u8 {
    #[inline(always)]
    fn from(val: GprShared4AuthenTzUser) -> u8 {
        GprShared4AuthenTzUser::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GprShared5AuthenTzNs {
    #[doc = "Cannot be changed in Non-secure mode."]
    NonsecNo = 0x0,
    #[doc = "Can be changed in Non-secure mode."]
    NonsecYes = 0x01,
}
impl GprShared5AuthenTzNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GprShared5AuthenTzNs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GprShared5AuthenTzNs {
    #[inline(always)]
    fn from(val: u8) -> GprShared5AuthenTzNs {
        GprShared5AuthenTzNs::from_bits(val)
    }
}
impl From<GprShared5AuthenTzNs> for u8 {
    #[inline(always)]
    fn from(val: GprShared5AuthenTzNs) -> u8 {
        GprShared5AuthenTzNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GprShared5AuthenTzUser {
    #[doc = "Registers of shared GPR slice cannot be changed in user mode."]
    UsrModeNo = 0x0,
    #[doc = "Registers of shared GPR slice can be changed in user mode."]
    UsrModeYes = 0x01,
}
impl GprShared5AuthenTzUser {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GprShared5AuthenTzUser {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GprShared5AuthenTzUser {
    #[inline(always)]
    fn from(val: u8) -> GprShared5AuthenTzUser {
        GprShared5AuthenTzUser::from_bits(val)
    }
}
impl From<GprShared5AuthenTzUser> for u8 {
    #[inline(always)]
    fn from(val: GprShared5AuthenTzUser) -> u8 {
        GprShared5AuthenTzUser::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GprShared6AuthenTzNs {
    #[doc = "Cannot be changed in Non-secure mode."]
    NonsecNo = 0x0,
    #[doc = "Can be changed in Non-secure mode."]
    NonsecYes = 0x01,
}
impl GprShared6AuthenTzNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GprShared6AuthenTzNs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GprShared6AuthenTzNs {
    #[inline(always)]
    fn from(val: u8) -> GprShared6AuthenTzNs {
        GprShared6AuthenTzNs::from_bits(val)
    }
}
impl From<GprShared6AuthenTzNs> for u8 {
    #[inline(always)]
    fn from(val: GprShared6AuthenTzNs) -> u8 {
        GprShared6AuthenTzNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GprShared6AuthenTzUser {
    #[doc = "Registers of shared GPR slice cannot be changed in user mode."]
    UsrModeNo = 0x0,
    #[doc = "Registers of shared GPR slice can be changed in user mode."]
    UsrModeYes = 0x01,
}
impl GprShared6AuthenTzUser {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GprShared6AuthenTzUser {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GprShared6AuthenTzUser {
    #[inline(always)]
    fn from(val: u8) -> GprShared6AuthenTzUser {
        GprShared6AuthenTzUser::from_bits(val)
    }
}
impl From<GprShared6AuthenTzUser> for u8 {
    #[inline(always)]
    fn from(val: GprShared6AuthenTzUser) -> u8 {
        GprShared6AuthenTzUser::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GprShared7AuthenTzNs {
    #[doc = "Cannot be changed in Non-secure mode."]
    NonsecNo = 0x0,
    #[doc = "Can be changed in Non-secure mode."]
    NonsecYes = 0x01,
}
impl GprShared7AuthenTzNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GprShared7AuthenTzNs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GprShared7AuthenTzNs {
    #[inline(always)]
    fn from(val: u8) -> GprShared7AuthenTzNs {
        GprShared7AuthenTzNs::from_bits(val)
    }
}
impl From<GprShared7AuthenTzNs> for u8 {
    #[inline(always)]
    fn from(val: GprShared7AuthenTzNs) -> u8 {
        GprShared7AuthenTzNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GprShared7AuthenTzUser {
    #[doc = "Registers of shared GPR slice cannot be changed in user mode."]
    UsrModeNo = 0x0,
    #[doc = "Registers of shared GPR slice can be changed in user mode."]
    UsrModeYes = 0x01,
}
impl GprShared7AuthenTzUser {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GprShared7AuthenTzUser {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GprShared7AuthenTzUser {
    #[inline(always)]
    fn from(val: u8) -> GprShared7AuthenTzUser {
        GprShared7AuthenTzUser::from_bits(val)
    }
}
impl From<GprShared7AuthenTzUser> for u8 {
    #[inline(always)]
    fn from(val: GprShared7AuthenTzUser) -> u8 {
        GprShared7AuthenTzUser::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GprShared8AuthenTzNs {
    #[doc = "Cannot be changed in Non-secure mode."]
    NonsecNo = 0x0,
    #[doc = "Can be changed in Non-secure mode."]
    NonsecYes = 0x01,
}
impl GprShared8AuthenTzNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GprShared8AuthenTzNs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GprShared8AuthenTzNs {
    #[inline(always)]
    fn from(val: u8) -> GprShared8AuthenTzNs {
        GprShared8AuthenTzNs::from_bits(val)
    }
}
impl From<GprShared8AuthenTzNs> for u8 {
    #[inline(always)]
    fn from(val: GprShared8AuthenTzNs) -> u8 {
        GprShared8AuthenTzNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GprShared8AuthenTzUser {
    #[doc = "Registers of shared GPR slice cannot be changed in user mode."]
    UsrModeNo = 0x0,
    #[doc = "Registers of shared GPR slice can be changed in user mode."]
    UsrModeYes = 0x01,
}
impl GprShared8AuthenTzUser {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GprShared8AuthenTzUser {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GprShared8AuthenTzUser {
    #[inline(always)]
    fn from(val: u8) -> GprShared8AuthenTzUser {
        GprShared8AuthenTzUser::from_bits(val)
    }
}
impl From<GprShared8AuthenTzUser> for u8 {
    #[inline(always)]
    fn from(val: GprShared8AuthenTzUser) -> u8 {
        GprShared8AuthenTzUser::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GprShared9AuthenTzNs {
    #[doc = "Cannot be changed in Non-secure mode."]
    NonsecNo = 0x0,
    #[doc = "Can be changed in Non-secure mode."]
    NonsecYes = 0x01,
}
impl GprShared9AuthenTzNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GprShared9AuthenTzNs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GprShared9AuthenTzNs {
    #[inline(always)]
    fn from(val: u8) -> GprShared9AuthenTzNs {
        GprShared9AuthenTzNs::from_bits(val)
    }
}
impl From<GprShared9AuthenTzNs> for u8 {
    #[inline(always)]
    fn from(val: GprShared9AuthenTzNs) -> u8 {
        GprShared9AuthenTzNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GprShared9AuthenTzUser {
    #[doc = "Registers of shared GPR slice cannot be changed in user mode."]
    UsrModeNo = 0x0,
    #[doc = "Registers of shared GPR slice can be changed in user mode."]
    UsrModeYes = 0x01,
}
impl GprShared9AuthenTzUser {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GprShared9AuthenTzUser {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GprShared9AuthenTzUser {
    #[inline(always)]
    fn from(val: u8) -> GprShared9AuthenTzUser {
        GprShared9AuthenTzUser::from_bits(val)
    }
}
impl From<GprShared9AuthenTzUser> for u8 {
    #[inline(always)]
    fn from(val: GprShared9AuthenTzUser) -> u8 {
        GprShared9AuthenTzUser::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InUse {
    #[doc = "Clock Source is not being used."]
    ClksrcNotused = 0x0,
    #[doc = "Clock Source is being used."]
    ClksrcUsed = 0x01,
}
impl InUse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InUse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InUse {
    #[inline(always)]
    fn from(val: u8) -> InUse {
        InUse::from_bits(val)
    }
}
impl From<InUse> for u8 {
    #[inline(always)]
    fn from(val: InUse) -> u8 {
        InUse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpcgAuthenTzNs {
    #[doc = "Cannot be changed in Non-secure mode."]
    NonsecNo = 0x0,
    #[doc = "Can be changed in Non-secure mode."]
    NonsecYes = 0x01,
}
impl LpcgAuthenTzNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpcgAuthenTzNs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpcgAuthenTzNs {
    #[inline(always)]
    fn from(val: u8) -> LpcgAuthenTzNs {
        LpcgAuthenTzNs::from_bits(val)
    }
}
impl From<LpcgAuthenTzNs> for u8 {
    #[inline(always)]
    fn from(val: LpcgAuthenTzNs) -> u8 {
        LpcgAuthenTzNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpcgAuthenTzUser {
    #[doc = "LPCG settings cannot be changed in user mode."]
    UsrModeNo = 0x0,
    #[doc = "LPCG settings can be changed in user mode."]
    UsrModeYes = 0x01,
}
impl LpcgAuthenTzUser {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpcgAuthenTzUser {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpcgAuthenTzUser {
    #[inline(always)]
    fn from(val: u8) -> LpcgAuthenTzUser {
        LpcgAuthenTzUser::from_bits(val)
    }
}
impl From<LpcgAuthenTzUser> for u8 {
    #[inline(always)]
    fn from(val: LpcgAuthenTzUser) -> u8 {
        LpcgAuthenTzUser::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpcgLpm0LpmSettingD0 {
    #[doc = "Clock Source will be OFF in any CPU mode."]
    ClksrcOffAll = 0x0,
    #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
    ClksrcOnrun = 0x01,
    #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
    ClksrcOnrunwait = 0x02,
    #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
    ClksrcOnrunwaitstop = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl LpcgLpm0LpmSettingD0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpcgLpm0LpmSettingD0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpcgLpm0LpmSettingD0 {
    #[inline(always)]
    fn from(val: u8) -> LpcgLpm0LpmSettingD0 {
        LpcgLpm0LpmSettingD0::from_bits(val)
    }
}
impl From<LpcgLpm0LpmSettingD0> for u8 {
    #[inline(always)]
    fn from(val: LpcgLpm0LpmSettingD0) -> u8 {
        LpcgLpm0LpmSettingD0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpcgLpm0LpmSettingD1 {
    #[doc = "Clock Source will be OFF in any CPU mode."]
    ClksrcOffAll = 0x0,
    #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
    ClksrcOnrun = 0x01,
    #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
    ClksrcOnrunwait = 0x02,
    #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
    ClksrcOnrunwaitstop = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl LpcgLpm0LpmSettingD1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpcgLpm0LpmSettingD1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpcgLpm0LpmSettingD1 {
    #[inline(always)]
    fn from(val: u8) -> LpcgLpm0LpmSettingD1 {
        LpcgLpm0LpmSettingD1::from_bits(val)
    }
}
impl From<LpcgLpm0LpmSettingD1> for u8 {
    #[inline(always)]
    fn from(val: LpcgLpm0LpmSettingD1) -> u8 {
        LpcgLpm0LpmSettingD1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpcgLpm0LpmSettingD2 {
    #[doc = "Clock Source will be OFF in any CPU mode."]
    ClksrcOffAll = 0x0,
    #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
    ClksrcOnrun = 0x01,
    #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
    ClksrcOnrunwait = 0x02,
    #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
    ClksrcOnrunwaitstop = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl LpcgLpm0LpmSettingD2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpcgLpm0LpmSettingD2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpcgLpm0LpmSettingD2 {
    #[inline(always)]
    fn from(val: u8) -> LpcgLpm0LpmSettingD2 {
        LpcgLpm0LpmSettingD2::from_bits(val)
    }
}
impl From<LpcgLpm0LpmSettingD2> for u8 {
    #[inline(always)]
    fn from(val: LpcgLpm0LpmSettingD2) -> u8 {
        LpcgLpm0LpmSettingD2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpcgLpm0LpmSettingD3 {
    #[doc = "Clock Source will be OFF in any CPU mode."]
    ClksrcOffAll = 0x0,
    #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
    ClksrcOnrun = 0x01,
    #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
    ClksrcOnrunwait = 0x02,
    #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
    ClksrcOnrunwaitstop = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl LpcgLpm0LpmSettingD3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpcgLpm0LpmSettingD3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpcgLpm0LpmSettingD3 {
    #[inline(always)]
    fn from(val: u8) -> LpcgLpm0LpmSettingD3 {
        LpcgLpm0LpmSettingD3::from_bits(val)
    }
}
impl From<LpcgLpm0LpmSettingD3> for u8 {
    #[inline(always)]
    fn from(val: LpcgLpm0LpmSettingD3) -> u8 {
        LpcgLpm0LpmSettingD3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpcgLpm0LpmSettingD4 {
    #[doc = "Clock Source will be OFF in any CPU mode."]
    ClksrcOffAll = 0x0,
    #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
    ClksrcOnrun = 0x01,
    #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
    ClksrcOnrunwait = 0x02,
    #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
    ClksrcOnrunwaitstop = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl LpcgLpm0LpmSettingD4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpcgLpm0LpmSettingD4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpcgLpm0LpmSettingD4 {
    #[inline(always)]
    fn from(val: u8) -> LpcgLpm0LpmSettingD4 {
        LpcgLpm0LpmSettingD4::from_bits(val)
    }
}
impl From<LpcgLpm0LpmSettingD4> for u8 {
    #[inline(always)]
    fn from(val: LpcgLpm0LpmSettingD4) -> u8 {
        LpcgLpm0LpmSettingD4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpcgLpm0LpmSettingD5 {
    #[doc = "Clock Source will be OFF in any CPU mode."]
    ClksrcOffAll = 0x0,
    #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
    ClksrcOnrun = 0x01,
    #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
    ClksrcOnrunwait = 0x02,
    #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
    ClksrcOnrunwaitstop = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl LpcgLpm0LpmSettingD5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpcgLpm0LpmSettingD5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpcgLpm0LpmSettingD5 {
    #[inline(always)]
    fn from(val: u8) -> LpcgLpm0LpmSettingD5 {
        LpcgLpm0LpmSettingD5::from_bits(val)
    }
}
impl From<LpcgLpm0LpmSettingD5> for u8 {
    #[inline(always)]
    fn from(val: LpcgLpm0LpmSettingD5) -> u8 {
        LpcgLpm0LpmSettingD5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpcgLpm0LpmSettingD6 {
    #[doc = "Clock Source will be OFF in any CPU mode."]
    ClksrcOffAll = 0x0,
    #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
    ClksrcOnrun = 0x01,
    #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
    ClksrcOnrunwait = 0x02,
    #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
    ClksrcOnrunwaitstop = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl LpcgLpm0LpmSettingD6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpcgLpm0LpmSettingD6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpcgLpm0LpmSettingD6 {
    #[inline(always)]
    fn from(val: u8) -> LpcgLpm0LpmSettingD6 {
        LpcgLpm0LpmSettingD6::from_bits(val)
    }
}
impl From<LpcgLpm0LpmSettingD6> for u8 {
    #[inline(always)]
    fn from(val: LpcgLpm0LpmSettingD6) -> u8 {
        LpcgLpm0LpmSettingD6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpcgLpm0LpmSettingD7 {
    #[doc = "Clock Source will be OFF in any CPU mode."]
    ClksrcOffAll = 0x0,
    #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
    ClksrcOnrun = 0x01,
    #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
    ClksrcOnrunwait = 0x02,
    #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
    ClksrcOnrunwaitstop = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl LpcgLpm0LpmSettingD7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpcgLpm0LpmSettingD7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpcgLpm0LpmSettingD7 {
    #[inline(always)]
    fn from(val: u8) -> LpcgLpm0LpmSettingD7 {
        LpcgLpm0LpmSettingD7::from_bits(val)
    }
}
impl From<LpcgLpm0LpmSettingD7> for u8 {
    #[inline(always)]
    fn from(val: LpcgLpm0LpmSettingD7) -> u8 {
        LpcgLpm0LpmSettingD7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpcgLpm1LpmSettingD10 {
    #[doc = "Clock Source will be OFF in any CPU mode."]
    ClksrcOffAll = 0x0,
    #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
    ClksrcOnrun = 0x01,
    #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
    ClksrcOnrunwait = 0x02,
    #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
    ClksrcOnrunwaitstop = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl LpcgLpm1LpmSettingD10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpcgLpm1LpmSettingD10 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpcgLpm1LpmSettingD10 {
    #[inline(always)]
    fn from(val: u8) -> LpcgLpm1LpmSettingD10 {
        LpcgLpm1LpmSettingD10::from_bits(val)
    }
}
impl From<LpcgLpm1LpmSettingD10> for u8 {
    #[inline(always)]
    fn from(val: LpcgLpm1LpmSettingD10) -> u8 {
        LpcgLpm1LpmSettingD10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpcgLpm1LpmSettingD11 {
    #[doc = "Clock Source will be OFF in any CPU mode."]
    ClksrcOffAll = 0x0,
    #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
    ClksrcOnrun = 0x01,
    #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
    ClksrcOnrunwait = 0x02,
    #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
    ClksrcOnrunwaitstop = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl LpcgLpm1LpmSettingD11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpcgLpm1LpmSettingD11 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpcgLpm1LpmSettingD11 {
    #[inline(always)]
    fn from(val: u8) -> LpcgLpm1LpmSettingD11 {
        LpcgLpm1LpmSettingD11::from_bits(val)
    }
}
impl From<LpcgLpm1LpmSettingD11> for u8 {
    #[inline(always)]
    fn from(val: LpcgLpm1LpmSettingD11) -> u8 {
        LpcgLpm1LpmSettingD11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpcgLpm1LpmSettingD12 {
    #[doc = "Clock Source will be OFF in any CPU mode."]
    ClksrcOffAll = 0x0,
    #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
    ClksrcOnrun = 0x01,
    #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
    ClksrcOnrunwait = 0x02,
    #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
    ClksrcOnrunwaitstop = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl LpcgLpm1LpmSettingD12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpcgLpm1LpmSettingD12 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpcgLpm1LpmSettingD12 {
    #[inline(always)]
    fn from(val: u8) -> LpcgLpm1LpmSettingD12 {
        LpcgLpm1LpmSettingD12::from_bits(val)
    }
}
impl From<LpcgLpm1LpmSettingD12> for u8 {
    #[inline(always)]
    fn from(val: LpcgLpm1LpmSettingD12) -> u8 {
        LpcgLpm1LpmSettingD12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpcgLpm1LpmSettingD13 {
    #[doc = "Clock Source will be OFF in any CPU mode."]
    ClksrcOffAll = 0x0,
    #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
    ClksrcOnrun = 0x01,
    #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
    ClksrcOnrunwait = 0x02,
    #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
    ClksrcOnrunwaitstop = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl LpcgLpm1LpmSettingD13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpcgLpm1LpmSettingD13 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpcgLpm1LpmSettingD13 {
    #[inline(always)]
    fn from(val: u8) -> LpcgLpm1LpmSettingD13 {
        LpcgLpm1LpmSettingD13::from_bits(val)
    }
}
impl From<LpcgLpm1LpmSettingD13> for u8 {
    #[inline(always)]
    fn from(val: LpcgLpm1LpmSettingD13) -> u8 {
        LpcgLpm1LpmSettingD13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpcgLpm1LpmSettingD14 {
    #[doc = "Clock Source will be OFF in any CPU mode."]
    ClksrcOffAll = 0x0,
    #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
    ClksrcOnrun = 0x01,
    #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
    ClksrcOnrunwait = 0x02,
    #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
    ClksrcOnrunwaitstop = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl LpcgLpm1LpmSettingD14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpcgLpm1LpmSettingD14 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpcgLpm1LpmSettingD14 {
    #[inline(always)]
    fn from(val: u8) -> LpcgLpm1LpmSettingD14 {
        LpcgLpm1LpmSettingD14::from_bits(val)
    }
}
impl From<LpcgLpm1LpmSettingD14> for u8 {
    #[inline(always)]
    fn from(val: LpcgLpm1LpmSettingD14) -> u8 {
        LpcgLpm1LpmSettingD14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpcgLpm1LpmSettingD15 {
    #[doc = "Clock Source will be OFF in any CPU mode."]
    ClksrcOffAll = 0x0,
    #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
    ClksrcOnrun = 0x01,
    #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
    ClksrcOnrunwait = 0x02,
    #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
    ClksrcOnrunwaitstop = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl LpcgLpm1LpmSettingD15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpcgLpm1LpmSettingD15 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpcgLpm1LpmSettingD15 {
    #[inline(always)]
    fn from(val: u8) -> LpcgLpm1LpmSettingD15 {
        LpcgLpm1LpmSettingD15::from_bits(val)
    }
}
impl From<LpcgLpm1LpmSettingD15> for u8 {
    #[inline(always)]
    fn from(val: LpcgLpm1LpmSettingD15) -> u8 {
        LpcgLpm1LpmSettingD15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpcgLpm1LpmSettingD8 {
    #[doc = "Clock Source will be OFF in any CPU mode."]
    ClksrcOffAll = 0x0,
    #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
    ClksrcOnrun = 0x01,
    #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
    ClksrcOnrunwait = 0x02,
    #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
    ClksrcOnrunwaitstop = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl LpcgLpm1LpmSettingD8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpcgLpm1LpmSettingD8 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpcgLpm1LpmSettingD8 {
    #[inline(always)]
    fn from(val: u8) -> LpcgLpm1LpmSettingD8 {
        LpcgLpm1LpmSettingD8::from_bits(val)
    }
}
impl From<LpcgLpm1LpmSettingD8> for u8 {
    #[inline(always)]
    fn from(val: LpcgLpm1LpmSettingD8) -> u8 {
        LpcgLpm1LpmSettingD8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpcgLpm1LpmSettingD9 {
    #[doc = "Clock Source will be OFF in any CPU mode."]
    ClksrcOffAll = 0x0,
    #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
    ClksrcOnrun = 0x01,
    #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
    ClksrcOnrunwait = 0x02,
    #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
    ClksrcOnrunwaitstop = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl LpcgLpm1LpmSettingD9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpcgLpm1LpmSettingD9 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpcgLpm1LpmSettingD9 {
    #[inline(always)]
    fn from(val: u8) -> LpcgLpm1LpmSettingD9 {
        LpcgLpm1LpmSettingD9::from_bits(val)
    }
}
impl From<LpcgLpm1LpmSettingD9> for u8 {
    #[inline(always)]
    fn from(val: LpcgLpm1LpmSettingD9) -> u8 {
        LpcgLpm1LpmSettingD9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpcgLpmCurLpmSettingCur {
    #[doc = "Clock Source will be OFF in any CPU mode."]
    ClksrcOffAll = 0x0,
    #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
    ClksrcOnrun = 0x01,
    #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
    ClksrcOnrunwait = 0x02,
    #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
    ClksrcOnrunwaitstop = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl LpcgLpmCurLpmSettingCur {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpcgLpmCurLpmSettingCur {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpcgLpmCurLpmSettingCur {
    #[inline(always)]
    fn from(val: u8) -> LpcgLpmCurLpmSettingCur {
        LpcgLpmCurLpmSettingCur::from_bits(val)
    }
}
impl From<LpcgLpmCurLpmSettingCur> for u8 {
    #[inline(always)]
    fn from(val: LpcgLpmCurLpmSettingCur) -> u8 {
        LpcgLpmCurLpmSettingCur::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mux {
    #[doc = "Select clock source 0"]
    Source0 = 0x0,
    #[doc = "Select clock source 1"]
    Source1 = 0x01,
    #[doc = "Select clock source 2"]
    Source2 = 0x02,
    #[doc = "Select clock source 3"]
    Source3 = 0x03,
}
impl Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mux {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mux {
    #[inline(always)]
    fn from(val: u8) -> Mux {
        Mux::from_bits(val)
    }
}
impl From<Mux> for u8 {
    #[inline(always)]
    fn from(val: Mux) -> u8 {
        Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ObserveAuthenTzNs {
    #[doc = "Cannot be changed in non-secure mode."]
    NonsecNo = 0x0,
    #[doc = "Can be changed in non-secure mode."]
    NonsecYes = 0x01,
}
impl ObserveAuthenTzNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ObserveAuthenTzNs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ObserveAuthenTzNs {
    #[inline(always)]
    fn from(val: u8) -> ObserveAuthenTzNs {
        ObserveAuthenTzNs::from_bits(val)
    }
}
impl From<ObserveAuthenTzNs> for u8 {
    #[inline(always)]
    fn from(val: ObserveAuthenTzNs) -> u8 {
        ObserveAuthenTzNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ObserveAuthenTzUser {
    #[doc = "Observe slice settings cannot be changed in user mode."]
    UsrModeNo = 0x0,
    #[doc = "Observe slice settings can be changed in user mode."]
    UsrModeYes = 0x01,
}
impl ObserveAuthenTzUser {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ObserveAuthenTzUser {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ObserveAuthenTzUser {
    #[inline(always)]
    fn from(val: u8) -> ObserveAuthenTzUser {
        ObserveAuthenTzUser::from_bits(val)
    }
}
impl From<ObserveAuthenTzUser> for u8 {
    #[inline(always)]
    fn from(val: ObserveAuthenTzUser) -> u8 {
        ObserveAuthenTzUser::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ObserveControlOff {
    #[doc = "observe slice is on"]
    ObsSlOn = 0x0,
    #[doc = "observe slice is off"]
    ObsSlOff = 0x01,
}
impl ObserveControlOff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ObserveControlOff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ObserveControlOff {
    #[inline(always)]
    fn from(val: u8) -> ObserveControlOff {
        ObserveControlOff::from_bits(val)
    }
}
impl From<ObserveControlOff> for u8 {
    #[inline(always)]
    fn from(val: ObserveControlOff) -> u8 {
        ObserveControlOff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ObserveControlRaw {
    #[doc = "Select divided signal."]
    SelDiv = 0x0,
    #[doc = "Select raw signal."]
    SelRaw = 0x01,
}
impl ObserveControlRaw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ObserveControlRaw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ObserveControlRaw {
    #[inline(always)]
    fn from(val: u8) -> ObserveControlRaw {
        ObserveControlRaw::from_bits(val)
    }
}
impl From<ObserveControlRaw> for u8 {
    #[inline(always)]
    fn from(val: ObserveControlRaw) -> u8 {
        ObserveControlRaw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ObserveControlReset {
    #[doc = "Reset deasserts"]
    RstDeassert = 0x0,
    #[doc = "Reset asserts"]
    RstAssert = 0x01,
}
impl ObserveControlReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ObserveControlReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ObserveControlReset {
    #[inline(always)]
    fn from(val: u8) -> ObserveControlReset {
        ObserveControlReset::from_bits(val)
    }
}
impl From<ObserveControlReset> for u8 {
    #[inline(always)]
    fn from(val: ObserveControlReset) -> u8 {
        ObserveControlReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ObserveStatusOff {
    #[doc = "observe slice is on"]
    On = 0x0,
    #[doc = "observe slice is off"]
    Off = 0x01,
}
impl ObserveStatusOff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ObserveStatusOff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ObserveStatusOff {
    #[inline(always)]
    fn from(val: u8) -> ObserveStatusOff {
        ObserveStatusOff::from_bits(val)
    }
}
impl From<ObserveStatusOff> for u8 {
    #[inline(always)]
    fn from(val: ObserveStatusOff) -> u8 {
        ObserveStatusOff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ObserveStatusRaw {
    #[doc = "Select divided signal."]
    SelDiv = 0x0,
    #[doc = "Select raw signal."]
    SelRaw = 0x01,
}
impl ObserveStatusRaw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ObserveStatusRaw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ObserveStatusRaw {
    #[inline(always)]
    fn from(val: u8) -> ObserveStatusRaw {
        ObserveStatusRaw::from_bits(val)
    }
}
impl From<ObserveStatusRaw> for u8 {
    #[inline(always)]
    fn from(val: ObserveStatusRaw) -> u8 {
        ObserveStatusRaw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ObserveStatusReset {
    #[doc = "Observe divider is not in reset state"]
    DivNotRst = 0x0,
    #[doc = "Observe divider is in reset state"]
    DivRst = 0x01,
}
impl ObserveStatusReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ObserveStatusReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ObserveStatusReset {
    #[inline(always)]
    fn from(val: u8) -> ObserveStatusReset {
        ObserveStatusReset::from_bits(val)
    }
}
impl From<ObserveStatusReset> for u8 {
    #[inline(always)]
    fn from(val: ObserveStatusReset) -> u8 {
        ObserveStatusReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OscpllAuthenTzNs {
    #[doc = "Cannot be changed in Non-secure mode."]
    NonsecNo = 0x0,
    #[doc = "Can be changed in Non-secure mode."]
    NonsecYes = 0x01,
}
impl OscpllAuthenTzNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OscpllAuthenTzNs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OscpllAuthenTzNs {
    #[inline(always)]
    fn from(val: u8) -> OscpllAuthenTzNs {
        OscpllAuthenTzNs::from_bits(val)
    }
}
impl From<OscpllAuthenTzNs> for u8 {
    #[inline(always)]
    fn from(val: OscpllAuthenTzNs) -> u8 {
        OscpllAuthenTzNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OscpllAuthenTzUser {
    #[doc = "Clock Source settings cannot be changed in user mode."]
    UsrModeNo = 0x0,
    #[doc = "Clock Source settings can be changed in user mode."]
    UsrModeYes = 0x01,
}
impl OscpllAuthenTzUser {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OscpllAuthenTzUser {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OscpllAuthenTzUser {
    #[inline(always)]
    fn from(val: u8) -> OscpllAuthenTzUser {
        OscpllAuthenTzUser::from_bits(val)
    }
}
impl From<OscpllAuthenTzUser> for u8 {
    #[inline(always)]
    fn from(val: OscpllAuthenTzUser) -> u8 {
        OscpllAuthenTzUser::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OscpllDirectOn {
    #[doc = "Clock source is OFF."]
    ClksrcOff = 0x0,
    #[doc = "Clock source is ON."]
    ClksrcOn = 0x01,
}
impl OscpllDirectOn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OscpllDirectOn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OscpllDirectOn {
    #[inline(always)]
    fn from(val: u8) -> OscpllDirectOn {
        OscpllDirectOn::from_bits(val)
    }
}
impl From<OscpllDirectOn> for u8 {
    #[inline(always)]
    fn from(val: OscpllDirectOn) -> u8 {
        OscpllDirectOn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OscpllLpm0LpmSettingD0 {
    #[doc = "Clock Source will be OFF in any CPU mode."]
    ClksrcOffAll = 0x0,
    #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
    ClksrcOnrun = 0x01,
    #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
    ClksrcOnrunwait = 0x02,
    #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
    ClksrcOnrunwaitstop = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl OscpllLpm0LpmSettingD0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OscpllLpm0LpmSettingD0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OscpllLpm0LpmSettingD0 {
    #[inline(always)]
    fn from(val: u8) -> OscpllLpm0LpmSettingD0 {
        OscpllLpm0LpmSettingD0::from_bits(val)
    }
}
impl From<OscpllLpm0LpmSettingD0> for u8 {
    #[inline(always)]
    fn from(val: OscpllLpm0LpmSettingD0) -> u8 {
        OscpllLpm0LpmSettingD0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OscpllLpm0LpmSettingD1 {
    #[doc = "Clock Source will be OFF in any CPU mode."]
    ClksrcOffAll = 0x0,
    #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
    ClksrcOnrun = 0x01,
    #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
    ClksrcOnrunwait = 0x02,
    #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
    ClksrcOnrunwaitstop = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl OscpllLpm0LpmSettingD1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OscpllLpm0LpmSettingD1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OscpllLpm0LpmSettingD1 {
    #[inline(always)]
    fn from(val: u8) -> OscpllLpm0LpmSettingD1 {
        OscpllLpm0LpmSettingD1::from_bits(val)
    }
}
impl From<OscpllLpm0LpmSettingD1> for u8 {
    #[inline(always)]
    fn from(val: OscpllLpm0LpmSettingD1) -> u8 {
        OscpllLpm0LpmSettingD1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OscpllLpm0LpmSettingD2 {
    #[doc = "Clock Source will be OFF in any CPU mode."]
    ClksrcOffAll = 0x0,
    #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
    ClksrcOnrun = 0x01,
    #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
    ClksrcOnrunwait = 0x02,
    #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
    ClksrcOnrunwaitstop = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl OscpllLpm0LpmSettingD2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OscpllLpm0LpmSettingD2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OscpllLpm0LpmSettingD2 {
    #[inline(always)]
    fn from(val: u8) -> OscpllLpm0LpmSettingD2 {
        OscpllLpm0LpmSettingD2::from_bits(val)
    }
}
impl From<OscpllLpm0LpmSettingD2> for u8 {
    #[inline(always)]
    fn from(val: OscpllLpm0LpmSettingD2) -> u8 {
        OscpllLpm0LpmSettingD2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OscpllLpm0LpmSettingD3 {
    #[doc = "Clock Source will be OFF in any CPU mode."]
    ClksrcOffAll = 0x0,
    #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
    ClksrcOnrun = 0x01,
    #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
    ClksrcOnrunwait = 0x02,
    #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
    ClksrcOnrunwaitstop = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl OscpllLpm0LpmSettingD3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OscpllLpm0LpmSettingD3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OscpllLpm0LpmSettingD3 {
    #[inline(always)]
    fn from(val: u8) -> OscpllLpm0LpmSettingD3 {
        OscpllLpm0LpmSettingD3::from_bits(val)
    }
}
impl From<OscpllLpm0LpmSettingD3> for u8 {
    #[inline(always)]
    fn from(val: OscpllLpm0LpmSettingD3) -> u8 {
        OscpllLpm0LpmSettingD3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OscpllLpm0LpmSettingD4 {
    #[doc = "Clock Source will be OFF in any CPU mode."]
    ClksrcOffAll = 0x0,
    #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
    ClksrcOnrun = 0x01,
    #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
    ClksrcOnrunwait = 0x02,
    #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
    ClksrcOnrunwaitstop = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl OscpllLpm0LpmSettingD4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OscpllLpm0LpmSettingD4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OscpllLpm0LpmSettingD4 {
    #[inline(always)]
    fn from(val: u8) -> OscpllLpm0LpmSettingD4 {
        OscpllLpm0LpmSettingD4::from_bits(val)
    }
}
impl From<OscpllLpm0LpmSettingD4> for u8 {
    #[inline(always)]
    fn from(val: OscpllLpm0LpmSettingD4) -> u8 {
        OscpllLpm0LpmSettingD4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OscpllLpm0LpmSettingD5 {
    #[doc = "Clock Source will be OFF in any CPU mode."]
    ClksrcOffAll = 0x0,
    #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
    ClksrcOnrun = 0x01,
    #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
    ClksrcOnrunwait = 0x02,
    #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
    ClksrcOnrunwaitstop = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl OscpllLpm0LpmSettingD5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OscpllLpm0LpmSettingD5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OscpllLpm0LpmSettingD5 {
    #[inline(always)]
    fn from(val: u8) -> OscpllLpm0LpmSettingD5 {
        OscpllLpm0LpmSettingD5::from_bits(val)
    }
}
impl From<OscpllLpm0LpmSettingD5> for u8 {
    #[inline(always)]
    fn from(val: OscpllLpm0LpmSettingD5) -> u8 {
        OscpllLpm0LpmSettingD5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OscpllLpm0LpmSettingD6 {
    #[doc = "Clock Source will be OFF in any CPU mode."]
    ClksrcOffAll = 0x0,
    #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
    ClksrcOnrun = 0x01,
    #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
    ClksrcOnrunwait = 0x02,
    #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
    ClksrcOnrunwaitstop = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl OscpllLpm0LpmSettingD6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OscpllLpm0LpmSettingD6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OscpllLpm0LpmSettingD6 {
    #[inline(always)]
    fn from(val: u8) -> OscpllLpm0LpmSettingD6 {
        OscpllLpm0LpmSettingD6::from_bits(val)
    }
}
impl From<OscpllLpm0LpmSettingD6> for u8 {
    #[inline(always)]
    fn from(val: OscpllLpm0LpmSettingD6) -> u8 {
        OscpllLpm0LpmSettingD6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OscpllLpm0LpmSettingD7 {
    #[doc = "Clock Source will be OFF in any CPU mode."]
    ClksrcOffAll = 0x0,
    #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
    ClksrcOnrun = 0x01,
    #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
    ClksrcOnrunwait = 0x02,
    #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
    ClksrcOnrunwaitstop = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl OscpllLpm0LpmSettingD7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OscpllLpm0LpmSettingD7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OscpllLpm0LpmSettingD7 {
    #[inline(always)]
    fn from(val: u8) -> OscpllLpm0LpmSettingD7 {
        OscpllLpm0LpmSettingD7::from_bits(val)
    }
}
impl From<OscpllLpm0LpmSettingD7> for u8 {
    #[inline(always)]
    fn from(val: OscpllLpm0LpmSettingD7) -> u8 {
        OscpllLpm0LpmSettingD7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OscpllLpm1LpmSettingD10 {
    #[doc = "Clock Source will be OFF in any CPU mode."]
    ClksrcOffAll = 0x0,
    #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
    ClksrcOnrun = 0x01,
    #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
    ClksrcOnrunwait = 0x02,
    #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
    ClksrcOnrunwaitstop = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl OscpllLpm1LpmSettingD10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OscpllLpm1LpmSettingD10 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OscpllLpm1LpmSettingD10 {
    #[inline(always)]
    fn from(val: u8) -> OscpllLpm1LpmSettingD10 {
        OscpllLpm1LpmSettingD10::from_bits(val)
    }
}
impl From<OscpllLpm1LpmSettingD10> for u8 {
    #[inline(always)]
    fn from(val: OscpllLpm1LpmSettingD10) -> u8 {
        OscpllLpm1LpmSettingD10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OscpllLpm1LpmSettingD11 {
    #[doc = "Clock Source will be OFF in any CPU mode."]
    ClksrcOffAll = 0x0,
    #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
    ClksrcOnrun = 0x01,
    #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
    ClksrcOnrunwait = 0x02,
    #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
    ClksrcOnrunwaitstop = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl OscpllLpm1LpmSettingD11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OscpllLpm1LpmSettingD11 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OscpllLpm1LpmSettingD11 {
    #[inline(always)]
    fn from(val: u8) -> OscpllLpm1LpmSettingD11 {
        OscpllLpm1LpmSettingD11::from_bits(val)
    }
}
impl From<OscpllLpm1LpmSettingD11> for u8 {
    #[inline(always)]
    fn from(val: OscpllLpm1LpmSettingD11) -> u8 {
        OscpllLpm1LpmSettingD11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OscpllLpm1LpmSettingD12 {
    #[doc = "Clock Source will be OFF in any CPU mode."]
    ClksrcOffAll = 0x0,
    #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
    ClksrcOnrun = 0x01,
    #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
    ClksrcOnrunwait = 0x02,
    #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
    ClksrcOnrunwaitstop = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl OscpllLpm1LpmSettingD12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OscpllLpm1LpmSettingD12 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OscpllLpm1LpmSettingD12 {
    #[inline(always)]
    fn from(val: u8) -> OscpllLpm1LpmSettingD12 {
        OscpllLpm1LpmSettingD12::from_bits(val)
    }
}
impl From<OscpllLpm1LpmSettingD12> for u8 {
    #[inline(always)]
    fn from(val: OscpllLpm1LpmSettingD12) -> u8 {
        OscpllLpm1LpmSettingD12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OscpllLpm1LpmSettingD13 {
    #[doc = "Clock Source will be OFF in any CPU mode."]
    ClksrcOffAll = 0x0,
    #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
    ClksrcOnrun = 0x01,
    #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
    ClksrcOnrunwait = 0x02,
    #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
    ClksrcOnrunwaitstop = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl OscpllLpm1LpmSettingD13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OscpllLpm1LpmSettingD13 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OscpllLpm1LpmSettingD13 {
    #[inline(always)]
    fn from(val: u8) -> OscpllLpm1LpmSettingD13 {
        OscpllLpm1LpmSettingD13::from_bits(val)
    }
}
impl From<OscpllLpm1LpmSettingD13> for u8 {
    #[inline(always)]
    fn from(val: OscpllLpm1LpmSettingD13) -> u8 {
        OscpllLpm1LpmSettingD13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OscpllLpm1LpmSettingD14 {
    #[doc = "Clock Source will be OFF in any CPU mode."]
    ClksrcOffAll = 0x0,
    #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
    ClksrcOnrun = 0x01,
    #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
    ClksrcOnrunwait = 0x02,
    #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
    ClksrcOnrunwaitstop = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl OscpllLpm1LpmSettingD14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OscpllLpm1LpmSettingD14 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OscpllLpm1LpmSettingD14 {
    #[inline(always)]
    fn from(val: u8) -> OscpllLpm1LpmSettingD14 {
        OscpllLpm1LpmSettingD14::from_bits(val)
    }
}
impl From<OscpllLpm1LpmSettingD14> for u8 {
    #[inline(always)]
    fn from(val: OscpllLpm1LpmSettingD14) -> u8 {
        OscpllLpm1LpmSettingD14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OscpllLpm1LpmSettingD15 {
    #[doc = "Clock Source will be OFF in any CPU mode."]
    ClksrcOffAll = 0x0,
    #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
    ClksrcOnrun = 0x01,
    #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
    ClksrcOnrunwait = 0x02,
    #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
    ClksrcOnrunwaitstop = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl OscpllLpm1LpmSettingD15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OscpllLpm1LpmSettingD15 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OscpllLpm1LpmSettingD15 {
    #[inline(always)]
    fn from(val: u8) -> OscpllLpm1LpmSettingD15 {
        OscpllLpm1LpmSettingD15::from_bits(val)
    }
}
impl From<OscpllLpm1LpmSettingD15> for u8 {
    #[inline(always)]
    fn from(val: OscpllLpm1LpmSettingD15) -> u8 {
        OscpllLpm1LpmSettingD15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OscpllLpm1LpmSettingD8 {
    #[doc = "Clock Source will be OFF in any CPU mode."]
    ClksrcOffAll = 0x0,
    #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
    ClksrcOnrun = 0x01,
    #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
    ClksrcOnrunwait = 0x02,
    #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
    ClksrcOnrunwaitstop = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl OscpllLpm1LpmSettingD8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OscpllLpm1LpmSettingD8 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OscpllLpm1LpmSettingD8 {
    #[inline(always)]
    fn from(val: u8) -> OscpllLpm1LpmSettingD8 {
        OscpllLpm1LpmSettingD8::from_bits(val)
    }
}
impl From<OscpllLpm1LpmSettingD8> for u8 {
    #[inline(always)]
    fn from(val: OscpllLpm1LpmSettingD8) -> u8 {
        OscpllLpm1LpmSettingD8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OscpllLpm1LpmSettingD9 {
    #[doc = "Clock Source will be OFF in any CPU mode."]
    ClksrcOffAll = 0x0,
    #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
    ClksrcOnrun = 0x01,
    #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
    ClksrcOnrunwait = 0x02,
    #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
    ClksrcOnrunwaitstop = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl OscpllLpm1LpmSettingD9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OscpllLpm1LpmSettingD9 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OscpllLpm1LpmSettingD9 {
    #[inline(always)]
    fn from(val: u8) -> OscpllLpm1LpmSettingD9 {
        OscpllLpm1LpmSettingD9::from_bits(val)
    }
}
impl From<OscpllLpm1LpmSettingD9> for u8 {
    #[inline(always)]
    fn from(val: OscpllLpm1LpmSettingD9) -> u8 {
        OscpllLpm1LpmSettingD9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OscpllLpmCurLpmSettingCur {
    #[doc = "Clock Source will be OFF in any CPU mode."]
    ClksrcOffAll = 0x0,
    #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
    ClksrcOnrun = 0x01,
    #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
    ClksrcOnrunwait = 0x02,
    #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
    ClksrcOnrunwaitstop = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl OscpllLpmCurLpmSettingCur {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OscpllLpmCurLpmSettingCur {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OscpllLpmCurLpmSettingCur {
    #[inline(always)]
    fn from(val: u8) -> OscpllLpmCurLpmSettingCur {
        OscpllLpmCurLpmSettingCur::from_bits(val)
    }
}
impl From<OscpllLpmCurLpmSettingCur> for u8 {
    #[inline(always)]
    fn from(val: OscpllLpmCurLpmSettingCur) -> u8 {
        OscpllLpmCurLpmSettingCur::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct WhiteList(pub u16);
impl WhiteList {
    #[doc = "No domain can change."]
    pub const NO_CHANGE: Self = Self(0x0);
    #[doc = "Domain 0 can change."]
    pub const DOM0_CHANGE: Self = Self(0x01);
    #[doc = "Domain 1 can change."]
    pub const DOM1_CHANGE: Self = Self(0x02);
    #[doc = "Domain 0 and domain 1 can change."]
    pub const DOM01_CHANGE: Self = Self(0x03);
    #[doc = "Domain 2 can change."]
    pub const DOM2_CHANGE: Self = Self(0x04);
    #[doc = "All domain can change."]
    pub const ALL_CHANGE: Self = Self(0x0f);
}
impl WhiteList {
    pub const fn from_bits(val: u16) -> WhiteList {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for WhiteList {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NO_CHANGE"),
            0x01 => f.write_str("DOM0_CHANGE"),
            0x02 => f.write_str("DOM1_CHANGE"),
            0x03 => f.write_str("DOM01_CHANGE"),
            0x04 => f.write_str("DOM2_CHANGE"),
            0x0f => f.write_str("ALL_CHANGE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WhiteList {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NO_CHANGE"),
            0x01 => defmt::write!(f, "DOM0_CHANGE"),
            0x02 => defmt::write!(f, "DOM1_CHANGE"),
            0x03 => defmt::write!(f, "DOM01_CHANGE"),
            0x04 => defmt::write!(f, "DOM2_CHANGE"),
            0x0f => defmt::write!(f, "ALL_CHANGE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for WhiteList {
    #[inline(always)]
    fn from(val: u16) -> WhiteList {
        WhiteList::from_bits(val)
    }
}
impl From<WhiteList> for u16 {
    #[inline(always)]
    fn from(val: WhiteList) -> u16 {
        WhiteList::to_bits(val)
    }
}
