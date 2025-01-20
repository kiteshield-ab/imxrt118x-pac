#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ArmPllControlMode {
    #[doc = "Software Mode (Default)"]
    Sw = 0x0,
    #[doc = "GPC Mode"]
    Gpc = 0x01,
}
impl ArmPllControlMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ArmPllControlMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ArmPllControlMode {
    #[inline(always)]
    fn from(val: u8) -> ArmPllControlMode {
        ArmPllControlMode::from_bits(val)
    }
}
impl From<ArmPllControlMode> for u8 {
    #[inline(always)]
    fn from(val: ArmPllControlMode) -> u8 {
        ArmPllControlMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ArmPllCtrlBypass {
    #[doc = "Function mode"]
    Func = 0x0,
    #[doc = "Bypass Mode"]
    Bypass = 0x01,
}
impl ArmPllCtrlBypass {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ArmPllCtrlBypass {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ArmPllCtrlBypass {
    #[inline(always)]
    fn from(val: u8) -> ArmPllCtrlBypass {
        ArmPllCtrlBypass::from_bits(val)
    }
}
impl From<ArmPllCtrlBypass> for u8 {
    #[inline(always)]
    fn from(val: ArmPllCtrlBypass) -> u8 {
        ArmPllCtrlBypass::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ArmPllCtrlHoldRingOff {
    #[doc = "Normal operation"]
    Normal = 0x0,
    #[doc = "Initialize PLL start up"]
    Enable = 0x01,
}
impl ArmPllCtrlHoldRingOff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ArmPllCtrlHoldRingOff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ArmPllCtrlHoldRingOff {
    #[inline(always)]
    fn from(val: u8) -> ArmPllCtrlHoldRingOff {
        ArmPllCtrlHoldRingOff::from_bits(val)
    }
}
impl From<ArmPllCtrlHoldRingOff> for u8 {
    #[inline(always)]
    fn from(val: ArmPllCtrlHoldRingOff) -> u8 {
        ArmPllCtrlHoldRingOff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ArmPllCtrlPowerup {
    #[doc = "Power down the PLL"]
    Pdown = 0x0,
    #[doc = "Power Up the PLL"]
    Pup = 0x01,
}
impl ArmPllCtrlPowerup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ArmPllCtrlPowerup {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ArmPllCtrlPowerup {
    #[inline(always)]
    fn from(val: u8) -> ArmPllCtrlPowerup {
        ArmPllCtrlPowerup::from_bits(val)
    }
}
impl From<ArmPllCtrlPowerup> for u8 {
    #[inline(always)]
    fn from(val: ArmPllCtrlPowerup) -> u8 {
        ArmPllCtrlPowerup::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ArmPllStable {
    #[doc = "ARM PLL is not stable"]
    Ns = 0x0,
    #[doc = "ARM PLL is stable"]
    Stable = 0x01,
}
impl ArmPllStable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ArmPllStable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ArmPllStable {
    #[inline(always)]
    fn from(val: u8) -> ArmPllStable {
        ArmPllStable::from_bits(val)
    }
}
impl From<ArmPllStable> for u8 {
    #[inline(always)]
    fn from(val: ArmPllStable) -> u8 {
        ArmPllStable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DivSelect {
    #[doc = "div_select=130x1: div_select=150x2"]
    Div21 = 0x0,
    #[doc = "div_select=160x3: div_select=200x4"]
    Div41 = 0x01,
    #[doc = "div_select=220x5: div_select=250x6"]
    Div81 = 0x02,
    #[doc = "div_select=300x7: div_select=240"]
    Div11 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl DivSelect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DivSelect {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DivSelect {
    #[inline(always)]
    fn from(val: u8) -> DivSelect {
        DivSelect::from_bits(val)
    }
}
impl From<DivSelect> for u8 {
    #[inline(always)]
    fn from(val: DivSelect) -> u8 {
        DivSelect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllAudioControlMode {
    #[doc = "Software Mode (Default)"]
    Sw = 0x0,
    #[doc = "GPC Mode"]
    Gpc = 0x01,
}
impl PllAudioControlMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllAudioControlMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllAudioControlMode {
    #[inline(always)]
    fn from(val: u8) -> PllAudioControlMode {
        PllAudioControlMode::from_bits(val)
    }
}
impl From<PllAudioControlMode> for u8 {
    #[inline(always)]
    fn from(val: PllAudioControlMode) -> u8 {
        PllAudioControlMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllAudioGate {
    #[doc = "No gate"]
    Nogate = 0x0,
    #[doc = "Gate the output"]
    Gated = 0x01,
}
impl PllAudioGate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllAudioGate {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllAudioGate {
    #[inline(always)]
    fn from(val: u8) -> PllAudioGate {
        PllAudioGate::from_bits(val)
    }
}
impl From<PllAudioGate> for u8 {
    #[inline(always)]
    fn from(val: PllAudioGate) -> u8 {
        PllAudioGate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PostDivSel {
    #[doc = "post_div=2"]
    Div2 = 0x0,
    #[doc = "post_div=4"]
    Div4 = 0x01,
    #[doc = "post_div=8"]
    Div8 = 0x02,
    #[doc = "post_div=1"]
    Div1 = 0x03,
}
impl PostDivSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PostDivSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PostDivSel {
    #[inline(always)]
    fn from(val: u8) -> PostDivSel {
        PostDivSel::from_bits(val)
    }
}
impl From<PostDivSel> for u8 {
    #[inline(always)]
    fn from(val: PostDivSel) -> u8 {
        PostDivSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysPll1ControlMode {
    #[doc = "Software Mode (Default)"]
    Sw = 0x0,
    #[doc = "GPC Mode"]
    Gpc = 0x01,
}
impl SysPll1ControlMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysPll1ControlMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysPll1ControlMode {
    #[inline(always)]
    fn from(val: u8) -> SysPll1ControlMode {
        SysPll1ControlMode::from_bits(val)
    }
}
impl From<SysPll1ControlMode> for u8 {
    #[inline(always)]
    fn from(val: SysPll1ControlMode) -> u8 {
        SysPll1ControlMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysPll1CtrlEnableClk {
    #[doc = "Enable"]
    Enable = 0x0,
    #[doc = "Disable"]
    Disable = 0x01,
}
impl SysPll1CtrlEnableClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysPll1CtrlEnableClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysPll1CtrlEnableClk {
    #[inline(always)]
    fn from(val: u8) -> SysPll1CtrlEnableClk {
        SysPll1CtrlEnableClk::from_bits(val)
    }
}
impl From<SysPll1CtrlEnableClk> for u8 {
    #[inline(always)]
    fn from(val: SysPll1CtrlEnableClk) -> u8 {
        SysPll1CtrlEnableClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysPll1Div2ControlMode {
    #[doc = "Software Mode (Default)"]
    Sw = 0x0,
    #[doc = "GPC Mode"]
    Gpc = 0x01,
}
impl SysPll1Div2ControlMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysPll1Div2ControlMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysPll1Div2ControlMode {
    #[inline(always)]
    fn from(val: u8) -> SysPll1Div2ControlMode {
        SysPll1Div2ControlMode::from_bits(val)
    }
}
impl From<SysPll1Div2ControlMode> for u8 {
    #[inline(always)]
    fn from(val: SysPll1Div2ControlMode) -> u8 {
        SysPll1Div2ControlMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysPll1Div5ControlMode {
    #[doc = "Software Mode (Default)"]
    Sw = 0x0,
    #[doc = "GPC Mode"]
    Gpc = 0x01,
}
impl SysPll1Div5ControlMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysPll1Div5ControlMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysPll1Div5ControlMode {
    #[inline(always)]
    fn from(val: u8) -> SysPll1Div5ControlMode {
        SysPll1Div5ControlMode::from_bits(val)
    }
}
impl From<SysPll1Div5ControlMode> for u8 {
    #[inline(always)]
    fn from(val: SysPll1Div5ControlMode) -> u8 {
        SysPll1Div5ControlMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysPll1Gate {
    #[doc = "No gate"]
    Nogate = 0x0,
    #[doc = "Gate the output"]
    Gated = 0x01,
}
impl SysPll1Gate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysPll1Gate {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysPll1Gate {
    #[inline(always)]
    fn from(val: u8) -> SysPll1Gate {
        SysPll1Gate::from_bits(val)
    }
}
impl From<SysPll1Gate> for u8 {
    #[inline(always)]
    fn from(val: SysPll1Gate) -> u8 {
        SysPll1Gate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysPll1Stable {
    #[doc = "Not Stable"]
    Ns = 0x0,
    #[doc = "Stable"]
    Stable = 0x01,
}
impl SysPll1Stable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysPll1Stable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysPll1Stable {
    #[inline(always)]
    fn from(val: u8) -> SysPll1Stable {
        SysPll1Stable::from_bits(val)
    }
}
impl From<SysPll1Stable> for u8 {
    #[inline(always)]
    fn from(val: SysPll1Stable) -> u8 {
        SysPll1Stable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysPll2ControlMode {
    #[doc = "Software Mode (Default)"]
    Sw = 0x0,
    #[doc = "GPC Mode"]
    Gpc = 0x01,
}
impl SysPll2ControlMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysPll2ControlMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysPll2ControlMode {
    #[inline(always)]
    fn from(val: u8) -> SysPll2ControlMode {
        SysPll2ControlMode::from_bits(val)
    }
}
impl From<SysPll2ControlMode> for u8 {
    #[inline(always)]
    fn from(val: SysPll2ControlMode) -> u8 {
        SysPll2ControlMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysPll2CtrlBypass {
    #[doc = "Function mode"]
    Func = 0x0,
    #[doc = "Bypass Mode"]
    Bypass = 0x01,
}
impl SysPll2CtrlBypass {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysPll2CtrlBypass {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysPll2CtrlBypass {
    #[inline(always)]
    fn from(val: u8) -> SysPll2CtrlBypass {
        SysPll2CtrlBypass::from_bits(val)
    }
}
impl From<SysPll2CtrlBypass> for u8 {
    #[inline(always)]
    fn from(val: SysPll2CtrlBypass) -> u8 {
        SysPll2CtrlBypass::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysPll2CtrlHoldRingOff {
    #[doc = "Normal operation"]
    Normal = 0x0,
    #[doc = "Initialize PLL start up"]
    Enable = 0x01,
}
impl SysPll2CtrlHoldRingOff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysPll2CtrlHoldRingOff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysPll2CtrlHoldRingOff {
    #[inline(always)]
    fn from(val: u8) -> SysPll2CtrlHoldRingOff {
        SysPll2CtrlHoldRingOff::from_bits(val)
    }
}
impl From<SysPll2CtrlHoldRingOff> for u8 {
    #[inline(always)]
    fn from(val: SysPll2CtrlHoldRingOff) -> u8 {
        SysPll2CtrlHoldRingOff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysPll2CtrlPowerup {
    #[doc = "Power down the PLL"]
    Pdown = 0x0,
    #[doc = "Power Up the PLL"]
    Pup = 0x01,
}
impl SysPll2CtrlPowerup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysPll2CtrlPowerup {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysPll2CtrlPowerup {
    #[inline(always)]
    fn from(val: u8) -> SysPll2CtrlPowerup {
        SysPll2CtrlPowerup::from_bits(val)
    }
}
impl From<SysPll2CtrlPowerup> for u8 {
    #[inline(always)]
    fn from(val: SysPll2CtrlPowerup) -> u8 {
        SysPll2CtrlPowerup::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysPll2PfdPfd0Div1Clkgate {
    #[doc = "PFD0 fractional divider clock is enabled."]
    Enable = 0x0,
    #[doc = "Fractional divider clock (reference PFD0) is off (power savings)"]
    Disable = 0x01,
}
impl SysPll2PfdPfd0Div1Clkgate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysPll2PfdPfd0Div1Clkgate {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysPll2PfdPfd0Div1Clkgate {
    #[inline(always)]
    fn from(val: u8) -> SysPll2PfdPfd0Div1Clkgate {
        SysPll2PfdPfd0Div1Clkgate::from_bits(val)
    }
}
impl From<SysPll2PfdPfd0Div1Clkgate> for u8 {
    #[inline(always)]
    fn from(val: SysPll2PfdPfd0Div1Clkgate) -> u8 {
        SysPll2PfdPfd0Div1Clkgate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysPll2PfdPfd1Div1Clkgate {
    #[doc = "PFD1 fractional divider clock is enabled."]
    Enable = 0x0,
    #[doc = "Fractional divider clock (reference PFD1) is off (power savings)"]
    Disable = 0x01,
}
impl SysPll2PfdPfd1Div1Clkgate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysPll2PfdPfd1Div1Clkgate {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysPll2PfdPfd1Div1Clkgate {
    #[inline(always)]
    fn from(val: u8) -> SysPll2PfdPfd1Div1Clkgate {
        SysPll2PfdPfd1Div1Clkgate::from_bits(val)
    }
}
impl From<SysPll2PfdPfd1Div1Clkgate> for u8 {
    #[inline(always)]
    fn from(val: SysPll2PfdPfd1Div1Clkgate) -> u8 {
        SysPll2PfdPfd1Div1Clkgate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysPll2PfdPfd2Div1Clkgate {
    #[doc = "PFD2 fractional divider clock is enabled."]
    Enable = 0x0,
    #[doc = "Fractional divider clock (reference PFD2) is off (power savings)"]
    Disable = 0x01,
}
impl SysPll2PfdPfd2Div1Clkgate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysPll2PfdPfd2Div1Clkgate {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysPll2PfdPfd2Div1Clkgate {
    #[inline(always)]
    fn from(val: u8) -> SysPll2PfdPfd2Div1Clkgate {
        SysPll2PfdPfd2Div1Clkgate::from_bits(val)
    }
}
impl From<SysPll2PfdPfd2Div1Clkgate> for u8 {
    #[inline(always)]
    fn from(val: SysPll2PfdPfd2Div1Clkgate) -> u8 {
        SysPll2PfdPfd2Div1Clkgate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysPll2PfdPfd3Div1Clkgate {
    #[doc = "PFD3 fractional divider clock is enabled."]
    Enable = 0x0,
    #[doc = "Fractional divider clock (reference PFD3) is off (power savings)"]
    Disable = 0x01,
}
impl SysPll2PfdPfd3Div1Clkgate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysPll2PfdPfd3Div1Clkgate {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysPll2PfdPfd3Div1Clkgate {
    #[inline(always)]
    fn from(val: u8) -> SysPll2PfdPfd3Div1Clkgate {
        SysPll2PfdPfd3Div1Clkgate::from_bits(val)
    }
}
impl From<SysPll2PfdPfd3Div1Clkgate> for u8 {
    #[inline(always)]
    fn from(val: SysPll2PfdPfd3Div1Clkgate) -> u8 {
        SysPll2PfdPfd3Div1Clkgate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysPll2UpdatePfd0ControlMode {
    #[doc = "Software Mode (Default)"]
    Sw = 0x0,
    #[doc = "GPC Mode"]
    Gpc = 0x01,
}
impl SysPll2UpdatePfd0ControlMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysPll2UpdatePfd0ControlMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysPll2UpdatePfd0ControlMode {
    #[inline(always)]
    fn from(val: u8) -> SysPll2UpdatePfd0ControlMode {
        SysPll2UpdatePfd0ControlMode::from_bits(val)
    }
}
impl From<SysPll2UpdatePfd0ControlMode> for u8 {
    #[inline(always)]
    fn from(val: SysPll2UpdatePfd0ControlMode) -> u8 {
        SysPll2UpdatePfd0ControlMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysPll2UpdatePfd1ControlMode {
    #[doc = "Software Mode (Default)"]
    Sw = 0x0,
    #[doc = "GPC Mode"]
    Gpc = 0x01,
}
impl SysPll2UpdatePfd1ControlMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysPll2UpdatePfd1ControlMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysPll2UpdatePfd1ControlMode {
    #[inline(always)]
    fn from(val: u8) -> SysPll2UpdatePfd1ControlMode {
        SysPll2UpdatePfd1ControlMode::from_bits(val)
    }
}
impl From<SysPll2UpdatePfd1ControlMode> for u8 {
    #[inline(always)]
    fn from(val: SysPll2UpdatePfd1ControlMode) -> u8 {
        SysPll2UpdatePfd1ControlMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysPll2UpdatePfd2ControlMode {
    #[doc = "Software Mode (Default)"]
    Sw = 0x0,
    #[doc = "GPC Mode"]
    Gpc = 0x01,
}
impl SysPll2UpdatePfd2ControlMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysPll2UpdatePfd2ControlMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysPll2UpdatePfd2ControlMode {
    #[inline(always)]
    fn from(val: u8) -> SysPll2UpdatePfd2ControlMode {
        SysPll2UpdatePfd2ControlMode::from_bits(val)
    }
}
impl From<SysPll2UpdatePfd2ControlMode> for u8 {
    #[inline(always)]
    fn from(val: SysPll2UpdatePfd2ControlMode) -> u8 {
        SysPll2UpdatePfd2ControlMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysPll2UpdatePfd3ControlMode {
    #[doc = "Software Mode (Default)"]
    Sw = 0x0,
    #[doc = "GPC Mode"]
    Gpc = 0x01,
}
impl SysPll2UpdatePfd3ControlMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysPll2UpdatePfd3ControlMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysPll2UpdatePfd3ControlMode {
    #[inline(always)]
    fn from(val: u8) -> SysPll2UpdatePfd3ControlMode {
        SysPll2UpdatePfd3ControlMode::from_bits(val)
    }
}
impl From<SysPll2UpdatePfd3ControlMode> for u8 {
    #[inline(always)]
    fn from(val: SysPll2UpdatePfd3ControlMode) -> u8 {
        SysPll2UpdatePfd3ControlMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysPll3ControlMode {
    #[doc = "Software Mode (Default)"]
    Sw = 0x0,
    #[doc = "GPC Mode"]
    Gpc = 0x01,
}
impl SysPll3ControlMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysPll3ControlMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysPll3ControlMode {
    #[inline(always)]
    fn from(val: u8) -> SysPll3ControlMode {
        SysPll3ControlMode::from_bits(val)
    }
}
impl From<SysPll3ControlMode> for u8 {
    #[inline(always)]
    fn from(val: SysPll3ControlMode) -> u8 {
        SysPll3ControlMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysPll3CtrlBypass {
    #[doc = "Function mode"]
    Func = 0x0,
    #[doc = "Bypass Mode"]
    Bypass = 0x01,
}
impl SysPll3CtrlBypass {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysPll3CtrlBypass {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysPll3CtrlBypass {
    #[inline(always)]
    fn from(val: u8) -> SysPll3CtrlBypass {
        SysPll3CtrlBypass::from_bits(val)
    }
}
impl From<SysPll3CtrlBypass> for u8 {
    #[inline(always)]
    fn from(val: SysPll3CtrlBypass) -> u8 {
        SysPll3CtrlBypass::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysPll3CtrlHoldRingOff {
    #[doc = "Normal operation"]
    Normal = 0x0,
    #[doc = "Initialize PLL start up"]
    Enable = 0x01,
}
impl SysPll3CtrlHoldRingOff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysPll3CtrlHoldRingOff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysPll3CtrlHoldRingOff {
    #[inline(always)]
    fn from(val: u8) -> SysPll3CtrlHoldRingOff {
        SysPll3CtrlHoldRingOff::from_bits(val)
    }
}
impl From<SysPll3CtrlHoldRingOff> for u8 {
    #[inline(always)]
    fn from(val: SysPll3CtrlHoldRingOff) -> u8 {
        SysPll3CtrlHoldRingOff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysPll3CtrlPowerup {
    #[doc = "Power down the PLL"]
    Pdown = 0x0,
    #[doc = "Power Up the PLL"]
    Pup = 0x01,
}
impl SysPll3CtrlPowerup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysPll3CtrlPowerup {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysPll3CtrlPowerup {
    #[inline(always)]
    fn from(val: u8) -> SysPll3CtrlPowerup {
        SysPll3CtrlPowerup::from_bits(val)
    }
}
impl From<SysPll3CtrlPowerup> for u8 {
    #[inline(always)]
    fn from(val: SysPll3CtrlPowerup) -> u8 {
        SysPll3CtrlPowerup::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysPll3Div2ControlMode {
    #[doc = "Software Mode (Default)"]
    Sw = 0x0,
    #[doc = "GPC Mode"]
    Gpc = 0x01,
}
impl SysPll3Div2ControlMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysPll3Div2ControlMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysPll3Div2ControlMode {
    #[inline(always)]
    fn from(val: u8) -> SysPll3Div2ControlMode {
        SysPll3Div2ControlMode::from_bits(val)
    }
}
impl From<SysPll3Div2ControlMode> for u8 {
    #[inline(always)]
    fn from(val: SysPll3Div2ControlMode) -> u8 {
        SysPll3Div2ControlMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysPll3PfdPfd0Div1Clkgate {
    #[doc = "PFD0 fractional divider clock is enabled"]
    On = 0x0,
    #[doc = "Fractional divider clock (reference ref_pfd0) is off (power savings"]
    Off = 0x01,
}
impl SysPll3PfdPfd0Div1Clkgate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysPll3PfdPfd0Div1Clkgate {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysPll3PfdPfd0Div1Clkgate {
    #[inline(always)]
    fn from(val: u8) -> SysPll3PfdPfd0Div1Clkgate {
        SysPll3PfdPfd0Div1Clkgate::from_bits(val)
    }
}
impl From<SysPll3PfdPfd0Div1Clkgate> for u8 {
    #[inline(always)]
    fn from(val: SysPll3PfdPfd0Div1Clkgate) -> u8 {
        SysPll3PfdPfd0Div1Clkgate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysPll3PfdPfd1Div1Clkgate {
    #[doc = "PFD1 fractional divider clock is enabled"]
    On = 0x0,
    #[doc = "Fractional divider clock (reference PFD1) is off (power savings)"]
    Off = 0x01,
}
impl SysPll3PfdPfd1Div1Clkgate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysPll3PfdPfd1Div1Clkgate {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysPll3PfdPfd1Div1Clkgate {
    #[inline(always)]
    fn from(val: u8) -> SysPll3PfdPfd1Div1Clkgate {
        SysPll3PfdPfd1Div1Clkgate::from_bits(val)
    }
}
impl From<SysPll3PfdPfd1Div1Clkgate> for u8 {
    #[inline(always)]
    fn from(val: SysPll3PfdPfd1Div1Clkgate) -> u8 {
        SysPll3PfdPfd1Div1Clkgate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysPll3PfdPfd2Div1Clkgate {
    #[doc = "PFD2 fractional divider clock is enabled"]
    On = 0x0,
    #[doc = "Fractional divider clock (reference PFD2) is off (power savings)"]
    Off = 0x01,
}
impl SysPll3PfdPfd2Div1Clkgate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysPll3PfdPfd2Div1Clkgate {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysPll3PfdPfd2Div1Clkgate {
    #[inline(always)]
    fn from(val: u8) -> SysPll3PfdPfd2Div1Clkgate {
        SysPll3PfdPfd2Div1Clkgate::from_bits(val)
    }
}
impl From<SysPll3PfdPfd2Div1Clkgate> for u8 {
    #[inline(always)]
    fn from(val: SysPll3PfdPfd2Div1Clkgate) -> u8 {
        SysPll3PfdPfd2Div1Clkgate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysPll3PfdPfd3Div1Clkgate {
    #[doc = "PFD3 fractional divider clock is enabled"]
    On = 0x0,
    #[doc = "Fractional divider clock (reference PFD3) is off (power savings)"]
    Off = 0x01,
}
impl SysPll3PfdPfd3Div1Clkgate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysPll3PfdPfd3Div1Clkgate {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysPll3PfdPfd3Div1Clkgate {
    #[inline(always)]
    fn from(val: u8) -> SysPll3PfdPfd3Div1Clkgate {
        SysPll3PfdPfd3Div1Clkgate::from_bits(val)
    }
}
impl From<SysPll3PfdPfd3Div1Clkgate> for u8 {
    #[inline(always)]
    fn from(val: SysPll3PfdPfd3Div1Clkgate) -> u8 {
        SysPll3PfdPfd3Div1Clkgate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysPll3Stable {
    #[doc = "Not Stable"]
    Ns = 0x0,
    #[doc = "Stable"]
    Stable = 0x01,
}
impl SysPll3Stable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysPll3Stable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysPll3Stable {
    #[inline(always)]
    fn from(val: u8) -> SysPll3Stable {
        SysPll3Stable::from_bits(val)
    }
}
impl From<SysPll3Stable> for u8 {
    #[inline(always)]
    fn from(val: SysPll3Stable) -> u8 {
        SysPll3Stable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysPll3UpdatePfd0ControlMode {
    #[doc = "Software Mode (Default)"]
    Sw = 0x0,
    #[doc = "GPC Mode"]
    Gpc = 0x01,
}
impl SysPll3UpdatePfd0ControlMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysPll3UpdatePfd0ControlMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysPll3UpdatePfd0ControlMode {
    #[inline(always)]
    fn from(val: u8) -> SysPll3UpdatePfd0ControlMode {
        SysPll3UpdatePfd0ControlMode::from_bits(val)
    }
}
impl From<SysPll3UpdatePfd0ControlMode> for u8 {
    #[inline(always)]
    fn from(val: SysPll3UpdatePfd0ControlMode) -> u8 {
        SysPll3UpdatePfd0ControlMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysPll3UpdatePfd1ControlMode {
    #[doc = "Software Mode (Default)"]
    Sw = 0x0,
    #[doc = "GPC Mode"]
    Gpc = 0x01,
}
impl SysPll3UpdatePfd1ControlMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysPll3UpdatePfd1ControlMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysPll3UpdatePfd1ControlMode {
    #[inline(always)]
    fn from(val: u8) -> SysPll3UpdatePfd1ControlMode {
        SysPll3UpdatePfd1ControlMode::from_bits(val)
    }
}
impl From<SysPll3UpdatePfd1ControlMode> for u8 {
    #[inline(always)]
    fn from(val: SysPll3UpdatePfd1ControlMode) -> u8 {
        SysPll3UpdatePfd1ControlMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysPll3UpdatePfd2ControlMode {
    #[doc = "Software Mode (Default)"]
    Sw = 0x0,
    #[doc = "GPC Mode"]
    Gpc = 0x01,
}
impl SysPll3UpdatePfd2ControlMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysPll3UpdatePfd2ControlMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysPll3UpdatePfd2ControlMode {
    #[inline(always)]
    fn from(val: u8) -> SysPll3UpdatePfd2ControlMode {
        SysPll3UpdatePfd2ControlMode::from_bits(val)
    }
}
impl From<SysPll3UpdatePfd2ControlMode> for u8 {
    #[inline(always)]
    fn from(val: SysPll3UpdatePfd2ControlMode) -> u8 {
        SysPll3UpdatePfd2ControlMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysPll3UpdatePfd3ControlMode {
    #[doc = "Software Mode (Default)"]
    Sw = 0x0,
    #[doc = "GPC Mode"]
    Gpc = 0x01,
}
impl SysPll3UpdatePfd3ControlMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysPll3UpdatePfd3ControlMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysPll3UpdatePfd3ControlMode {
    #[inline(always)]
    fn from(val: u8) -> SysPll3UpdatePfd3ControlMode {
        SysPll3UpdatePfd3ControlMode::from_bits(val)
    }
}
impl From<SysPll3UpdatePfd3ControlMode> for u8 {
    #[inline(always)]
    fn from(val: SysPll3UpdatePfd3ControlMode) -> u8 {
        SysPll3UpdatePfd3ControlMode::to_bits(val)
    }
}
