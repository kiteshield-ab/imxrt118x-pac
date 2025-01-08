#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc1IpgStopMode {
    #[doc = "This module is functional in Stop Mode"]
    FUNC = 0x0,
    #[doc = "This module is not functional in Stop Mode"]
    NONFUNC = 0x01,
}
impl Adc1IpgStopMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc1IpgStopMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc1IpgStopMode {
    #[inline(always)]
    fn from(val: u8) -> Adc1IpgStopMode {
        Adc1IpgStopMode::from_bits(val)
    }
}
impl From<Adc1IpgStopMode> for u8 {
    #[inline(always)]
    fn from(val: Adc1IpgStopMode) -> u8 {
        Adc1IpgStopMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc2IpgStopMode {
    #[doc = "This module is functional in Stop Mode"]
    FUNC = 0x0,
    #[doc = "This module is not functional in Stop Mode"]
    NONFUNC = 0x01,
}
impl Adc2IpgStopMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc2IpgStopMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc2IpgStopMode {
    #[inline(always)]
    fn from(val: u8) -> Adc2IpgStopMode {
        Adc2IpgStopMode::from_bits(val)
    }
}
impl From<Adc2IpgStopMode> for u8 {
    #[inline(always)]
    fn from(val: Adc2IpgStopMode) -> u8 {
        Adc2IpgStopMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Advp {
    #[doc = "ADV# is active low."]
    ADVP0 = 0x0,
    #[doc = "ADV# is active high."]
    ADVP1 = 0x01,
}
impl Advp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Advp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Advp {
    #[inline(always)]
    fn from(val: u8) -> Advp {
        Advp::from_bits(val)
    }
}
impl From<Advp> for u8 {
    #[inline(always)]
    fn from(val: Advp) -> u8 {
        Advp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Am {
    #[doc = "Address/Data MUX mode (ADMUX)"]
    AM0 = 0x0,
    #[doc = "Address/Data non-MUX mode (Non-ADMUX)"]
    AM1 = 0x01,
}
impl Am {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Am {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Am {
    #[inline(always)]
    fn from(val: u8) -> Am {
        Am::from_bits(val)
    }
}
impl From<Am> for u8 {
    #[inline(always)]
    fn from(val: Am) -> u8 {
        Am::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct AsyncEn(pub u8);
impl AsyncEn {
    #[doc = "Trigger in is synchronous"]
    pub const SYNC: Self = Self(0x0);
    #[doc = "Trigger in is asynchronous"]
    pub const ASYNC: Self = Self(0x01);
}
impl AsyncEn {
    pub const fn from_bits(val: u8) -> AsyncEn {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for AsyncEn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("SYNC"),
            0x01 => f.write_str("ASYNC"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AsyncEn {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "SYNC"),
            0x01 => defmt::write!(f, "ASYNC"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for AsyncEn {
    #[inline(always)]
    fn from(val: u8) -> AsyncEn {
        AsyncEn::from_bits(val)
    }
}
impl From<AsyncEn> for u8 {
    #[inline(always)]
    fn from(val: AsyncEn) -> u8 {
        AsyncEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BlkholeModeB {
    #[doc = "WAKEUP Domain to M7 SSI master will exit blackhole mode"]
    EXIT = 0x0,
    #[doc = "WAKEUP Domain to M7 SSI master will enter blackhole mode"]
    ENTER = 0x01,
}
impl BlkholeModeB {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BlkholeModeB {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BlkholeModeB {
    #[inline(always)]
    fn from(val: u8) -> BlkholeModeB {
        BlkholeModeB::from_bits(val)
    }
}
impl From<BlkholeModeB> for u8 {
    #[inline(always)]
    fn from(val: BlkholeModeB) -> u8 {
        BlkholeModeB::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ExcErrRespEn {
    #[doc = "OKAY response"]
    OKAY_RESPONSE = 0x0,
    #[doc = "SLVError response"]
    SLVERROR_RESPONSE = 0x01,
}
impl ExcErrRespEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ExcErrRespEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ExcErrRespEn {
    #[inline(always)]
    fn from(val: u8) -> ExcErrRespEn {
        ExcErrRespEn::from_bits(val)
    }
}
impl From<ExcErrRespEn> for u8 {
    #[inline(always)]
    fn from(val: ExcErrRespEn) -> u8 {
        ExcErrRespEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GlbRst {
    #[doc = "EtherCAT is out of reset"]
    OUT_RESET = 0x0,
    #[doc = "EtherCAT is held in reset"]
    IN_RESET = 0x01,
}
impl GlbRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GlbRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GlbRst {
    #[inline(always)]
    fn from(val: u8) -> GlbRst {
        GlbRst::from_bits(val)
    }
}
impl From<GlbRst> for u8 {
    #[inline(always)]
    fn from(val: GlbRst) -> u8 {
        GlbRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioB1SelectNasrc {
    #[doc = "Show the 4-bit PMOS compensation codes in GPIO_B1_NASRC field"]
    PMOS = 0x0,
    #[doc = "Show the 4-bit NMOS compensation codes in GPIO_B1_NASRC field"]
    NMOS = 0x01,
}
impl GpioB1SelectNasrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioB1SelectNasrc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioB1SelectNasrc {
    #[inline(always)]
    fn from(val: u8) -> GpioB1SelectNasrc {
        GpioB1SelectNasrc::from_bits(val)
    }
}
impl From<GpioB1SelectNasrc> for u8 {
    #[inline(always)]
    fn from(val: GpioB1SelectNasrc) -> u8 {
        GpioB1SelectNasrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioEmc2SelectNasrc {
    #[doc = "Show the 4-bit PMOS compensation codes in GPIO_EMC2_NASRC field"]
    PMOS = 0x0,
    #[doc = "Show the 4-bit NMOS compensation codes in GPIO_EMC2_NASRC field"]
    NMOS = 0x01,
}
impl GpioEmc2SelectNasrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioEmc2SelectNasrc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioEmc2SelectNasrc {
    #[inline(always)]
    fn from(val: u8) -> GpioEmc2SelectNasrc {
        GpioEmc2SelectNasrc::from_bits(val)
    }
}
impl From<GpioEmc2SelectNasrc> for u8 {
    #[inline(always)]
    fn from(val: GpioEmc2SelectNasrc) -> u8 {
        GpioEmc2SelectNasrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3cOnChipStrongPullDis {
    #[doc = "On-chip strong pull is enabled"]
    ENABLE = 0x0,
    #[doc = "On-chip strong pull is disabled"]
    DISABLE = 0x01,
}
impl I3cOnChipStrongPullDis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3cOnChipStrongPullDis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3cOnChipStrongPullDis {
    #[inline(always)]
    fn from(val: u8) -> I3cOnChipStrongPullDis {
        I3cOnChipStrongPullDis::from_bits(val)
    }
}
impl From<I3cOnChipStrongPullDis> for u8 {
    #[inline(always)]
    fn from(val: I3cOnChipStrongPullDis) -> u8 {
        I3cOnChipStrongPullDis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel10 {
    #[doc = "XBAR_INOUT as input"]
    IOMUX = 0x0,
    #[doc = "XBAR_INOUT as output"]
    XBAR = 0x01,
}
impl IomuxcXbarDirSel10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel10 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel10 {
        IomuxcXbarDirSel10::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel10> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel10) -> u8 {
        IomuxcXbarDirSel10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel11 {
    #[doc = "XBAR_INOUT as input"]
    IOMUX = 0x0,
    #[doc = "XBAR_INOUT as output"]
    XBAR = 0x01,
}
impl IomuxcXbarDirSel11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel11 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel11 {
        IomuxcXbarDirSel11::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel11> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel11) -> u8 {
        IomuxcXbarDirSel11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel12 {
    #[doc = "XBAR_INOUT as input"]
    IOMUX = 0x0,
    #[doc = "XBAR_INOUT as output"]
    XBAR = 0x01,
}
impl IomuxcXbarDirSel12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel12 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel12 {
        IomuxcXbarDirSel12::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel12> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel12) -> u8 {
        IomuxcXbarDirSel12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel13 {
    #[doc = "XBAR_INOUT as input"]
    IOMUX = 0x0,
    #[doc = "XBAR_INOUT as output"]
    XBAR = 0x01,
}
impl IomuxcXbarDirSel13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel13 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel13 {
        IomuxcXbarDirSel13::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel13> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel13) -> u8 {
        IomuxcXbarDirSel13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel14 {
    #[doc = "XBAR_INOUT as input"]
    IOMUX = 0x0,
    #[doc = "XBAR_INOUT as output"]
    XBAR = 0x01,
}
impl IomuxcXbarDirSel14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel14 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel14 {
        IomuxcXbarDirSel14::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel14> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel14) -> u8 {
        IomuxcXbarDirSel14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel15 {
    #[doc = "XBAR_INOUT as input"]
    IOMUX = 0x0,
    #[doc = "XBAR_INOUT as output"]
    XBAR = 0x01,
}
impl IomuxcXbarDirSel15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel15 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel15 {
        IomuxcXbarDirSel15::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel15> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel15) -> u8 {
        IomuxcXbarDirSel15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel16 {
    #[doc = "XBAR_INOUT as input"]
    IOMUX = 0x0,
    #[doc = "XBAR_INOUT as output"]
    XBAR = 0x01,
}
impl IomuxcXbarDirSel16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel16 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel16 {
        IomuxcXbarDirSel16::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel16> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel16) -> u8 {
        IomuxcXbarDirSel16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel17 {
    #[doc = "XBAR_INOUT as input"]
    IOMUX = 0x0,
    #[doc = "XBAR_INOUT as output"]
    XBAR = 0x01,
}
impl IomuxcXbarDirSel17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel17 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel17 {
        IomuxcXbarDirSel17::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel17> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel17) -> u8 {
        IomuxcXbarDirSel17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel18 {
    #[doc = "XBAR_INOUT as input"]
    IOMUX = 0x0,
    #[doc = "XBAR_INOUT as output"]
    XBAR = 0x01,
}
impl IomuxcXbarDirSel18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel18 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel18 {
        IomuxcXbarDirSel18::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel18> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel18) -> u8 {
        IomuxcXbarDirSel18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel19 {
    #[doc = "XBAR_INOUT as input"]
    IOMUX = 0x0,
    #[doc = "XBAR_INOUT as output"]
    XBAR = 0x01,
}
impl IomuxcXbarDirSel19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel19 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel19 {
        IomuxcXbarDirSel19::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel19> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel19) -> u8 {
        IomuxcXbarDirSel19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel20 {
    #[doc = "XBAR_INOUT as input"]
    IOMUX = 0x0,
    #[doc = "XBAR_INOUT as output"]
    XBAR = 0x01,
}
impl IomuxcXbarDirSel20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel20 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel20 {
        IomuxcXbarDirSel20::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel20> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel20) -> u8 {
        IomuxcXbarDirSel20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel21 {
    #[doc = "XBAR_INOUT as input"]
    IOMUX = 0x0,
    #[doc = "XBAR_INOUT as output"]
    XBAR = 0x01,
}
impl IomuxcXbarDirSel21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel21 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel21 {
        IomuxcXbarDirSel21::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel21> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel21) -> u8 {
        IomuxcXbarDirSel21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel22 {
    #[doc = "XBAR_INOUT as input"]
    IOMUX = 0x0,
    #[doc = "XBAR_INOUT as output"]
    XBAR = 0x01,
}
impl IomuxcXbarDirSel22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel22 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel22 {
        IomuxcXbarDirSel22::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel22> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel22) -> u8 {
        IomuxcXbarDirSel22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel23 {
    #[doc = "XBAR_INOUT as input"]
    IOMUX = 0x0,
    #[doc = "XBAR_INOUT as output"]
    XBAR = 0x01,
}
impl IomuxcXbarDirSel23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel23 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel23 {
        IomuxcXbarDirSel23::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel23> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel23) -> u8 {
        IomuxcXbarDirSel23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel24 {
    #[doc = "XBAR_INOUT as input"]
    IOMUX = 0x0,
    #[doc = "XBAR_INOUT as output"]
    XBAR = 0x01,
}
impl IomuxcXbarDirSel24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel24 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel24 {
        IomuxcXbarDirSel24::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel24> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel24) -> u8 {
        IomuxcXbarDirSel24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel25 {
    #[doc = "XBAR_INOUT as input"]
    IOMUX = 0x0,
    #[doc = "XBAR_INOUT as output"]
    XBAR = 0x01,
}
impl IomuxcXbarDirSel25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel25 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel25 {
        IomuxcXbarDirSel25::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel25> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel25) -> u8 {
        IomuxcXbarDirSel25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel26 {
    #[doc = "XBAR_INOUT as input"]
    IOMUX = 0x0,
    #[doc = "XBAR_INOUT as output"]
    XBAR = 0x01,
}
impl IomuxcXbarDirSel26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel26 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel26 {
        IomuxcXbarDirSel26::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel26> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel26) -> u8 {
        IomuxcXbarDirSel26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel27 {
    #[doc = "XBAR_INOUT as input"]
    IOMUX = 0x0,
    #[doc = "XBAR_INOUT as output"]
    XBAR = 0x01,
}
impl IomuxcXbarDirSel27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel27 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel27 {
        IomuxcXbarDirSel27::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel27> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel27) -> u8 {
        IomuxcXbarDirSel27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel28 {
    #[doc = "XBAR_INOUT as input"]
    IOMUX = 0x0,
    #[doc = "XBAR_INOUT as output"]
    XBAR = 0x01,
}
impl IomuxcXbarDirSel28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel28 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel28 {
        IomuxcXbarDirSel28::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel28> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel28) -> u8 {
        IomuxcXbarDirSel28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel29 {
    #[doc = "XBAR_INOUT as input"]
    IOMUX = 0x0,
    #[doc = "XBAR_INOUT as output"]
    XBAR = 0x01,
}
impl IomuxcXbarDirSel29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel29 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel29 {
        IomuxcXbarDirSel29::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel29> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel29) -> u8 {
        IomuxcXbarDirSel29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel30 {
    #[doc = "XBAR_INOUT as input"]
    IOMUX = 0x0,
    #[doc = "XBAR_INOUT as output"]
    XBAR = 0x01,
}
impl IomuxcXbarDirSel30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel30 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel30 {
        IomuxcXbarDirSel30::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel30> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel30) -> u8 {
        IomuxcXbarDirSel30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel31 {
    #[doc = "XBAR_INOUT as input"]
    IOMUX = 0x0,
    #[doc = "XBAR_INOUT as output"]
    XBAR = 0x01,
}
impl IomuxcXbarDirSel31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel31 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel31 {
        IomuxcXbarDirSel31::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel31> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel31) -> u8 {
        IomuxcXbarDirSel31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel32 {
    #[doc = "XBAR_INOUT as input"]
    IOMUX = 0x0,
    #[doc = "XBAR_INOUT as output"]
    XBAR = 0x01,
}
impl IomuxcXbarDirSel32 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel32 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel32 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel32 {
        IomuxcXbarDirSel32::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel32> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel32) -> u8 {
        IomuxcXbarDirSel32::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel33 {
    #[doc = "XBAR_INOUT as input"]
    IOMUX = 0x0,
    #[doc = "XBAR_INOUT as output"]
    XBAR = 0x01,
}
impl IomuxcXbarDirSel33 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel33 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel33 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel33 {
        IomuxcXbarDirSel33::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel33> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel33) -> u8 {
        IomuxcXbarDirSel33::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel34 {
    #[doc = "XBAR_INOUT as input"]
    IOMUX = 0x0,
    #[doc = "XBAR_INOUT as output"]
    XBAR = 0x01,
}
impl IomuxcXbarDirSel34 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel34 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel34 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel34 {
        IomuxcXbarDirSel34::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel34> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel34) -> u8 {
        IomuxcXbarDirSel34::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel35 {
    #[doc = "XBAR_INOUT as input"]
    IOMUX = 0x0,
    #[doc = "XBAR_INOUT as output"]
    XBAR = 0x01,
}
impl IomuxcXbarDirSel35 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel35 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel35 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel35 {
        IomuxcXbarDirSel35::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel35> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel35) -> u8 {
        IomuxcXbarDirSel35::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel36 {
    #[doc = "XBAR_INOUT as input"]
    IOMUX = 0x0,
    #[doc = "XBAR_INOUT as output"]
    XBAR = 0x01,
}
impl IomuxcXbarDirSel36 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel36 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel36 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel36 {
        IomuxcXbarDirSel36::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel36> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel36) -> u8 {
        IomuxcXbarDirSel36::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel37 {
    #[doc = "XBAR_INOUT as input"]
    IOMUX = 0x0,
    #[doc = "XBAR_INOUT as output"]
    XBAR = 0x01,
}
impl IomuxcXbarDirSel37 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel37 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel37 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel37 {
        IomuxcXbarDirSel37::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel37> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel37) -> u8 {
        IomuxcXbarDirSel37::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel4 {
    #[doc = "XBAR_INOUT as input"]
    IOMUX = 0x0,
    #[doc = "XBAR_INOUT as output"]
    XBAR = 0x01,
}
impl IomuxcXbarDirSel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel4 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel4 {
        IomuxcXbarDirSel4::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel4> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel4) -> u8 {
        IomuxcXbarDirSel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel5 {
    #[doc = "XBAR_INOUT as input"]
    IOMUX = 0x0,
    #[doc = "XBAR_INOUT as output"]
    XBAR = 0x01,
}
impl IomuxcXbarDirSel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel5 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel5 {
        IomuxcXbarDirSel5::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel5> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel5) -> u8 {
        IomuxcXbarDirSel5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel6 {
    #[doc = "XBAR_INOUT as input"]
    IOMUX = 0x0,
    #[doc = "XBAR_INOUT as output"]
    XBAR = 0x01,
}
impl IomuxcXbarDirSel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel6 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel6 {
        IomuxcXbarDirSel6::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel6> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel6) -> u8 {
        IomuxcXbarDirSel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel7 {
    #[doc = "XBAR_INOUT as input"]
    IOMUX = 0x0,
    #[doc = "XBAR_INOUT as output"]
    XBAR = 0x01,
}
impl IomuxcXbarDirSel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel7 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel7 {
        IomuxcXbarDirSel7::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel7> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel7) -> u8 {
        IomuxcXbarDirSel7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel8 {
    #[doc = "XBAR_INOUT as input"]
    IOMUX = 0x0,
    #[doc = "XBAR_INOUT as output"]
    XBAR = 0x01,
}
impl IomuxcXbarDirSel8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel8 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel8 {
        IomuxcXbarDirSel8::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel8> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel8) -> u8 {
        IomuxcXbarDirSel8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel9 {
    #[doc = "XBAR_INOUT as input"]
    IOMUX = 0x0,
    #[doc = "XBAR_INOUT as output"]
    XBAR = 0x01,
}
impl IomuxcXbarDirSel9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel9 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel9 {
        IomuxcXbarDirSel9::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel9> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel9) -> u8 {
        IomuxcXbarDirSel9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IrqStatus {
    #[doc = "Interrupt not asserted"]
    ENABLE = 0x0,
    #[doc = "Interrupt asserted"]
    DISABLE = 0x01,
}
impl IrqStatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IrqStatus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IrqStatus {
    #[inline(always)]
    fn from(val: u8) -> IrqStatus {
        IrqStatus::from_bits(val)
    }
}
impl From<IrqStatus> for u8 {
    #[inline(always)]
    fn from(val: IrqStatus) -> u8 {
        IrqStatus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpit1Trig0InputSel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl Lpit1Trig0InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpit1Trig0InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpit1Trig0InputSel {
    #[inline(always)]
    fn from(val: u8) -> Lpit1Trig0InputSel {
        Lpit1Trig0InputSel::from_bits(val)
    }
}
impl From<Lpit1Trig0InputSel> for u8 {
    #[inline(always)]
    fn from(val: Lpit1Trig0InputSel) -> u8 {
        Lpit1Trig0InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpit1Trig1InputSel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl Lpit1Trig1InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpit1Trig1InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpit1Trig1InputSel {
    #[inline(always)]
    fn from(val: u8) -> Lpit1Trig1InputSel {
        Lpit1Trig1InputSel::from_bits(val)
    }
}
impl From<Lpit1Trig1InputSel> for u8 {
    #[inline(always)]
    fn from(val: Lpit1Trig1InputSel) -> u8 {
        Lpit1Trig1InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpit1Trig2InputSel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl Lpit1Trig2InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpit1Trig2InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpit1Trig2InputSel {
    #[inline(always)]
    fn from(val: u8) -> Lpit1Trig2InputSel {
        Lpit1Trig2InputSel::from_bits(val)
    }
}
impl From<Lpit1Trig2InputSel> for u8 {
    #[inline(always)]
    fn from(val: Lpit1Trig2InputSel) -> u8 {
        Lpit1Trig2InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpit1Trig3InputSel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl Lpit1Trig3InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpit1Trig3InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpit1Trig3InputSel {
    #[inline(always)]
    fn from(val: u8) -> Lpit1Trig3InputSel {
        Lpit1Trig3InputSel::from_bits(val)
    }
}
impl From<Lpit1Trig3InputSel> for u8 {
    #[inline(always)]
    fn from(val: Lpit1Trig3InputSel) -> u8 {
        Lpit1Trig3InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpit2Trig0InputSel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl Lpit2Trig0InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpit2Trig0InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpit2Trig0InputSel {
    #[inline(always)]
    fn from(val: u8) -> Lpit2Trig0InputSel {
        Lpit2Trig0InputSel::from_bits(val)
    }
}
impl From<Lpit2Trig0InputSel> for u8 {
    #[inline(always)]
    fn from(val: Lpit2Trig0InputSel) -> u8 {
        Lpit2Trig0InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpit2Trig1InputSel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl Lpit2Trig1InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpit2Trig1InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpit2Trig1InputSel {
    #[inline(always)]
    fn from(val: u8) -> Lpit2Trig1InputSel {
        Lpit2Trig1InputSel::from_bits(val)
    }
}
impl From<Lpit2Trig1InputSel> for u8 {
    #[inline(always)]
    fn from(val: Lpit2Trig1InputSel) -> u8 {
        Lpit2Trig1InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpit2Trig2InputSel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl Lpit2Trig2InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpit2Trig2InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpit2Trig2InputSel {
    #[inline(always)]
    fn from(val: u8) -> Lpit2Trig2InputSel {
        Lpit2Trig2InputSel::from_bits(val)
    }
}
impl From<Lpit2Trig2InputSel> for u8 {
    #[inline(always)]
    fn from(val: Lpit2Trig2InputSel) -> u8 {
        Lpit2Trig2InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpit2Trig3InputSel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl Lpit2Trig3InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpit2Trig3InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpit2Trig3InputSel {
    #[inline(always)]
    fn from(val: u8) -> Lpit2Trig3InputSel {
        Lpit2Trig3InputSel::from_bits(val)
    }
}
impl From<Lpit2Trig3InputSel> for u8 {
    #[inline(always)]
    fn from(val: Lpit2Trig3InputSel) -> u8 {
        Lpit2Trig3InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpit3Trig0InputSel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl Lpit3Trig0InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpit3Trig0InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpit3Trig0InputSel {
    #[inline(always)]
    fn from(val: u8) -> Lpit3Trig0InputSel {
        Lpit3Trig0InputSel::from_bits(val)
    }
}
impl From<Lpit3Trig0InputSel> for u8 {
    #[inline(always)]
    fn from(val: Lpit3Trig0InputSel) -> u8 {
        Lpit3Trig0InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpit3Trig1InputSel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl Lpit3Trig1InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpit3Trig1InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpit3Trig1InputSel {
    #[inline(always)]
    fn from(val: u8) -> Lpit3Trig1InputSel {
        Lpit3Trig1InputSel::from_bits(val)
    }
}
impl From<Lpit3Trig1InputSel> for u8 {
    #[inline(always)]
    fn from(val: Lpit3Trig1InputSel) -> u8 {
        Lpit3Trig1InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpit3Trig2InputSel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl Lpit3Trig2InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpit3Trig2InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpit3Trig2InputSel {
    #[inline(always)]
    fn from(val: u8) -> Lpit3Trig2InputSel {
        Lpit3Trig2InputSel::from_bits(val)
    }
}
impl From<Lpit3Trig2InputSel> for u8 {
    #[inline(always)]
    fn from(val: Lpit3Trig2InputSel) -> u8 {
        Lpit3Trig2InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpit3Trig3InputSel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl Lpit3Trig3InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpit3Trig3InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpit3Trig3InputSel {
    #[inline(always)]
    fn from(val: u8) -> Lpit3Trig3InputSel {
        Lpit3Trig3InputSel::from_bits(val)
    }
}
impl From<Lpit3Trig3InputSel> for u8 {
    #[inline(always)]
    fn from(val: Lpit3Trig3InputSel) -> u8 {
        Lpit3Trig3InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Can2 {
    #[doc = "CAN2 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "CAN2 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Can2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Can2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Can2 {
    #[inline(always)]
    fn from(val: u8) -> M33Can2 {
        M33Can2::from_bits(val)
    }
}
impl From<M33Can2> for u8 {
    #[inline(always)]
    fn from(val: M33Can2) -> u8 {
        M33Can2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Edma4 {
    #[doc = "EDMA4 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "EDMA4 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Edma4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Edma4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Edma4 {
    #[inline(always)]
    fn from(val: u8) -> M33Edma4 {
        M33Edma4::from_bits(val)
    }
}
impl From<M33Edma4> for u8 {
    #[inline(always)]
    fn from(val: M33Edma4) -> u8 {
        M33Edma4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Flexio1 {
    #[doc = "FLEXIO1 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "FLEXIO1 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Flexio1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Flexio1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Flexio1 {
    #[inline(always)]
    fn from(val: u8) -> M33Flexio1 {
        M33Flexio1::from_bits(val)
    }
}
impl From<M33Flexio1> for u8 {
    #[inline(always)]
    fn from(val: M33Flexio1) -> u8 {
        M33Flexio1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Flexio2 {
    #[doc = "FLEXIO2 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "FLEXIO2 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Flexio2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Flexio2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Flexio2 {
    #[inline(always)]
    fn from(val: u8) -> M33Flexio2 {
        M33Flexio2::from_bits(val)
    }
}
impl From<M33Flexio2> for u8 {
    #[inline(always)]
    fn from(val: M33Flexio2) -> u8 {
        M33Flexio2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Flexpwm1 {
    #[doc = "FLEXPWM1 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "FLEXPWM1 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Flexpwm1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Flexpwm1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Flexpwm1 {
    #[inline(always)]
    fn from(val: u8) -> M33Flexpwm1 {
        M33Flexpwm1::from_bits(val)
    }
}
impl From<M33Flexpwm1> for u8 {
    #[inline(always)]
    fn from(val: M33Flexpwm1) -> u8 {
        M33Flexpwm1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Flexpwm2 {
    #[doc = "FLEXPWM2 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "FLEXPWM2 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Flexpwm2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Flexpwm2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Flexpwm2 {
    #[inline(always)]
    fn from(val: u8) -> M33Flexpwm2 {
        M33Flexpwm2::from_bits(val)
    }
}
impl From<M33Flexpwm2> for u8 {
    #[inline(always)]
    fn from(val: M33Flexpwm2) -> u8 {
        M33Flexpwm2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Flexpwm3 {
    #[doc = "FLEXPWM3 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "FLEXPWM3 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Flexpwm3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Flexpwm3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Flexpwm3 {
    #[inline(always)]
    fn from(val: u8) -> M33Flexpwm3 {
        M33Flexpwm3::from_bits(val)
    }
}
impl From<M33Flexpwm3> for u8 {
    #[inline(always)]
    fn from(val: M33Flexpwm3) -> u8 {
        M33Flexpwm3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Flexpwm4 {
    #[doc = "FLEXPWM4 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "FLEXPWM4 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Flexpwm4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Flexpwm4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Flexpwm4 {
    #[inline(always)]
    fn from(val: u8) -> M33Flexpwm4 {
        M33Flexpwm4::from_bits(val)
    }
}
impl From<M33Flexpwm4> for u8 {
    #[inline(always)]
    fn from(val: M33Flexpwm4) -> u8 {
        M33Flexpwm4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Gpt2 {
    #[doc = "GPT2 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "GPT2 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Gpt2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Gpt2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Gpt2 {
    #[inline(always)]
    fn from(val: u8) -> M33Gpt2 {
        M33Gpt2::from_bits(val)
    }
}
impl From<M33Gpt2> for u8 {
    #[inline(always)]
    fn from(val: M33Gpt2) -> u8 {
        M33Gpt2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33I3c2 {
    #[doc = "I3C2 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "I3C2 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33I3c2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33I3c2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33I3c2 {
    #[inline(always)]
    fn from(val: u8) -> M33I3c2 {
        M33I3c2::from_bits(val)
    }
}
impl From<M33I3c2> for u8 {
    #[inline(always)]
    fn from(val: M33I3c2) -> u8 {
        M33I3c2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Lpi2c3 {
    #[doc = "LPI2C3 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Lpi2c3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Lpi2c3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Lpi2c3 {
    #[inline(always)]
    fn from(val: u8) -> M33Lpi2c3 {
        M33Lpi2c3::from_bits(val)
    }
}
impl From<M33Lpi2c3> for u8 {
    #[inline(always)]
    fn from(val: M33Lpi2c3) -> u8 {
        M33Lpi2c3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Lpi2c4 {
    #[doc = "LPI2C4 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "LPI2C4 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Lpi2c4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Lpi2c4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Lpi2c4 {
    #[inline(always)]
    fn from(val: u8) -> M33Lpi2c4 {
        M33Lpi2c4::from_bits(val)
    }
}
impl From<M33Lpi2c4> for u8 {
    #[inline(always)]
    fn from(val: M33Lpi2c4) -> u8 {
        M33Lpi2c4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Lpi2c5 {
    #[doc = "LPI2C5 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "LPI2C5 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Lpi2c5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Lpi2c5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Lpi2c5 {
    #[inline(always)]
    fn from(val: u8) -> M33Lpi2c5 {
        M33Lpi2c5::from_bits(val)
    }
}
impl From<M33Lpi2c5> for u8 {
    #[inline(always)]
    fn from(val: M33Lpi2c5) -> u8 {
        M33Lpi2c5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Lpi2c6 {
    #[doc = "LPI2C6 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "LPI2C6 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Lpi2c6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Lpi2c6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Lpi2c6 {
    #[inline(always)]
    fn from(val: u8) -> M33Lpi2c6 {
        M33Lpi2c6::from_bits(val)
    }
}
impl From<M33Lpi2c6> for u8 {
    #[inline(always)]
    fn from(val: M33Lpi2c6) -> u8 {
        M33Lpi2c6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Lpit2 {
    #[doc = "LPIT2 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "LPIT2 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Lpit2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Lpit2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Lpit2 {
    #[inline(always)]
    fn from(val: u8) -> M33Lpit2 {
        M33Lpit2::from_bits(val)
    }
}
impl From<M33Lpit2> for u8 {
    #[inline(always)]
    fn from(val: M33Lpit2) -> u8 {
        M33Lpit2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Lpit3 {
    #[doc = "LPIT3 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "LPIT3 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Lpit3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Lpit3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Lpit3 {
    #[inline(always)]
    fn from(val: u8) -> M33Lpit3 {
        M33Lpit3::from_bits(val)
    }
}
impl From<M33Lpit3> for u8 {
    #[inline(always)]
    fn from(val: M33Lpit3) -> u8 {
        M33Lpit3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Lpspi3 {
    #[doc = "LPSPI3 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "LPSPI3 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Lpspi3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Lpspi3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Lpspi3 {
    #[inline(always)]
    fn from(val: u8) -> M33Lpspi3 {
        M33Lpspi3::from_bits(val)
    }
}
impl From<M33Lpspi3> for u8 {
    #[inline(always)]
    fn from(val: M33Lpspi3) -> u8 {
        M33Lpspi3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Lpspi4 {
    #[doc = "LPSPI4 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "LPSPI4 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Lpspi4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Lpspi4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Lpspi4 {
    #[inline(always)]
    fn from(val: u8) -> M33Lpspi4 {
        M33Lpspi4::from_bits(val)
    }
}
impl From<M33Lpspi4> for u8 {
    #[inline(always)]
    fn from(val: M33Lpspi4) -> u8 {
        M33Lpspi4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Lpspi5 {
    #[doc = "LPSPI5 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "LPSPI5 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Lpspi5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Lpspi5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Lpspi5 {
    #[inline(always)]
    fn from(val: u8) -> M33Lpspi5 {
        M33Lpspi5::from_bits(val)
    }
}
impl From<M33Lpspi5> for u8 {
    #[inline(always)]
    fn from(val: M33Lpspi5) -> u8 {
        M33Lpspi5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Lpspi6 {
    #[doc = "LPSPI6 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "LPSPI6 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Lpspi6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Lpspi6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Lpspi6 {
    #[inline(always)]
    fn from(val: u8) -> M33Lpspi6 {
        M33Lpspi6::from_bits(val)
    }
}
impl From<M33Lpspi6> for u8 {
    #[inline(always)]
    fn from(val: M33Lpspi6) -> u8 {
        M33Lpspi6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Lptmr2 {
    #[doc = "LPTMR2 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "LPTMR2 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Lptmr2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Lptmr2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Lptmr2 {
    #[inline(always)]
    fn from(val: u8) -> M33Lptmr2 {
        M33Lptmr2::from_bits(val)
    }
}
impl From<M33Lptmr2> for u8 {
    #[inline(always)]
    fn from(val: M33Lptmr2) -> u8 {
        M33Lptmr2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Lptmr3 {
    #[doc = "LPTMR3 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "LPTMR3 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Lptmr3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Lptmr3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Lptmr3 {
    #[inline(always)]
    fn from(val: u8) -> M33Lptmr3 {
        M33Lptmr3::from_bits(val)
    }
}
impl From<M33Lptmr3> for u8 {
    #[inline(always)]
    fn from(val: M33Lptmr3) -> u8 {
        M33Lptmr3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Mic {
    #[doc = "MIC does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "MIC enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Mic {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Mic {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Mic {
    #[inline(always)]
    fn from(val: u8) -> M33Mic {
        M33Mic::from_bits(val)
    }
}
impl From<M33Mic> for u8 {
    #[inline(always)]
    fn from(val: M33Mic) -> u8 {
        M33Mic::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Qtimer1 {
    #[doc = "QTIMER1 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "QTIMER1 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Qtimer1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Qtimer1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Qtimer1 {
    #[inline(always)]
    fn from(val: u8) -> M33Qtimer1 {
        M33Qtimer1::from_bits(val)
    }
}
impl From<M33Qtimer1> for u8 {
    #[inline(always)]
    fn from(val: M33Qtimer1) -> u8 {
        M33Qtimer1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Qtimer2 {
    #[doc = "QTIMER2 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "QTIMER2 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Qtimer2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Qtimer2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Qtimer2 {
    #[inline(always)]
    fn from(val: u8) -> M33Qtimer2 {
        M33Qtimer2::from_bits(val)
    }
}
impl From<M33Qtimer2> for u8 {
    #[inline(always)]
    fn from(val: M33Qtimer2) -> u8 {
        M33Qtimer2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Qtimer3 {
    #[doc = "QTIMER3 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "QTIMER3 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Qtimer3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Qtimer3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Qtimer3 {
    #[inline(always)]
    fn from(val: u8) -> M33Qtimer3 {
        M33Qtimer3::from_bits(val)
    }
}
impl From<M33Qtimer3> for u8 {
    #[inline(always)]
    fn from(val: M33Qtimer3) -> u8 {
        M33Qtimer3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Qtimer4 {
    #[doc = "QTIMER4 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "QTIMER4 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Qtimer4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Qtimer4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Qtimer4 {
    #[inline(always)]
    fn from(val: u8) -> M33Qtimer4 {
        M33Qtimer4::from_bits(val)
    }
}
impl From<M33Qtimer4> for u8 {
    #[inline(always)]
    fn from(val: M33Qtimer4) -> u8 {
        M33Qtimer4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Qtimer5 {
    #[doc = "QTIMER5 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "QTIMER5 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Qtimer5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Qtimer5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Qtimer5 {
    #[inline(always)]
    fn from(val: u8) -> M33Qtimer5 {
        M33Qtimer5::from_bits(val)
    }
}
impl From<M33Qtimer5> for u8 {
    #[inline(always)]
    fn from(val: M33Qtimer5) -> u8 {
        M33Qtimer5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Qtimer6 {
    #[doc = "QTIMER6 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "QTIMER6 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Qtimer6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Qtimer6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Qtimer6 {
    #[inline(always)]
    fn from(val: u8) -> M33Qtimer6 {
        M33Qtimer6::from_bits(val)
    }
}
impl From<M33Qtimer6> for u8 {
    #[inline(always)]
    fn from(val: M33Qtimer6) -> u8 {
        M33Qtimer6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Qtimer7 {
    #[doc = "QTIMER7 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "QTIMER7 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Qtimer7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Qtimer7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Qtimer7 {
    #[inline(always)]
    fn from(val: u8) -> M33Qtimer7 {
        M33Qtimer7::from_bits(val)
    }
}
impl From<M33Qtimer7> for u8 {
    #[inline(always)]
    fn from(val: M33Qtimer7) -> u8 {
        M33Qtimer7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Qtimer8 {
    #[doc = "QTIMER8 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "QTIMER8 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Qtimer8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Qtimer8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Qtimer8 {
    #[inline(always)]
    fn from(val: u8) -> M33Qtimer8 {
        M33Qtimer8::from_bits(val)
    }
}
impl From<M33Qtimer8> for u8 {
    #[inline(always)]
    fn from(val: M33Qtimer8) -> u8 {
        M33Qtimer8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Sai2 {
    #[doc = "SAI2 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "SAI2 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Sai2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Sai2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Sai2 {
    #[inline(always)]
    fn from(val: u8) -> M33Sai2 {
        M33Sai2::from_bits(val)
    }
}
impl From<M33Sai2> for u8 {
    #[inline(always)]
    fn from(val: M33Sai2) -> u8 {
        M33Sai2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Sai3 {
    #[doc = "SAI3 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "SAI3 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Sai3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Sai3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Sai3 {
    #[inline(always)]
    fn from(val: u8) -> M33Sai3 {
        M33Sai3::from_bits(val)
    }
}
impl From<M33Sai3> for u8 {
    #[inline(always)]
    fn from(val: M33Sai3) -> u8 {
        M33Sai3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Sai4 {
    #[doc = "SAI4 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "SAI4 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Sai4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Sai4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Sai4 {
    #[inline(always)]
    fn from(val: u8) -> M33Sai4 {
        M33Sai4::from_bits(val)
    }
}
impl From<M33Sai4> for u8 {
    #[inline(always)]
    fn from(val: M33Sai4) -> u8 {
        M33Sai4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Sinc1 {
    #[doc = "I3C2 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "I3C2 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Sinc1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Sinc1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Sinc1 {
    #[inline(always)]
    fn from(val: u8) -> M33Sinc1 {
        M33Sinc1::from_bits(val)
    }
}
impl From<M33Sinc1> for u8 {
    #[inline(always)]
    fn from(val: M33Sinc1) -> u8 {
        M33Sinc1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Sinc2 {
    #[doc = "SINC2 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "SINC2 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Sinc2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Sinc2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Sinc2 {
    #[inline(always)]
    fn from(val: u8) -> M33Sinc2 {
        M33Sinc2::from_bits(val)
    }
}
impl From<M33Sinc2> for u8 {
    #[inline(always)]
    fn from(val: M33Sinc2) -> u8 {
        M33Sinc2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Sinc3 {
    #[doc = "SINC3 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "SINC3 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Sinc3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Sinc3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Sinc3 {
    #[inline(always)]
    fn from(val: u8) -> M33Sinc3 {
        M33Sinc3::from_bits(val)
    }
}
impl From<M33Sinc3> for u8 {
    #[inline(always)]
    fn from(val: M33Sinc3) -> u8 {
        M33Sinc3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Tpm3 {
    #[doc = "TPM3 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "TPM3 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Tpm3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Tpm3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Tpm3 {
    #[inline(always)]
    fn from(val: u8) -> M33Tpm3 {
        M33Tpm3::from_bits(val)
    }
}
impl From<M33Tpm3> for u8 {
    #[inline(always)]
    fn from(val: M33Tpm3) -> u8 {
        M33Tpm3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Tpm4 {
    #[doc = "does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Tpm4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Tpm4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Tpm4 {
    #[inline(always)]
    fn from(val: u8) -> M33Tpm4 {
        M33Tpm4::from_bits(val)
    }
}
impl From<M33Tpm4> for u8 {
    #[inline(always)]
    fn from(val: M33Tpm4) -> u8 {
        M33Tpm4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Tpm5 {
    #[doc = "TPM5 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "TPM5 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Tpm5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Tpm5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Tpm5 {
    #[inline(always)]
    fn from(val: u8) -> M33Tpm5 {
        M33Tpm5::from_bits(val)
    }
}
impl From<M33Tpm5> for u8 {
    #[inline(always)]
    fn from(val: M33Tpm5) -> u8 {
        M33Tpm5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Tpm6 {
    #[doc = "TPM6 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "TPM6 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Tpm6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Tpm6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Tpm6 {
    #[inline(always)]
    fn from(val: u8) -> M33Tpm6 {
        M33Tpm6::from_bits(val)
    }
}
impl From<M33Tpm6> for u8 {
    #[inline(always)]
    fn from(val: M33Tpm6) -> u8 {
        M33Tpm6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Wdog3 {
    #[doc = "WDOG3 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "WDOG3 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Wdog3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Wdog3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Wdog3 {
    #[inline(always)]
    fn from(val: u8) -> M33Wdog3 {
        M33Wdog3::from_bits(val)
    }
}
impl From<M33Wdog3> for u8 {
    #[inline(always)]
    fn from(val: M33Wdog3) -> u8 {
        M33Wdog3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Wdog4 {
    #[doc = "WDOG4 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "WDOG4 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Wdog4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Wdog4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Wdog4 {
    #[inline(always)]
    fn from(val: u8) -> M33Wdog4 {
        M33Wdog4::from_bits(val)
    }
}
impl From<M33Wdog4> for u8 {
    #[inline(always)]
    fn from(val: M33Wdog4) -> u8 {
        M33Wdog4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Wdog5 {
    #[doc = "WDOG5 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "WDOG5 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Wdog5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Wdog5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Wdog5 {
    #[inline(always)]
    fn from(val: u8) -> M33Wdog5 {
        M33Wdog5::from_bits(val)
    }
}
impl From<M33Wdog5> for u8 {
    #[inline(always)]
    fn from(val: M33Wdog5) -> u8 {
        M33Wdog5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Can2 {
    #[doc = "CAN2 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "CAN2 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Can2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Can2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Can2 {
    #[inline(always)]
    fn from(val: u8) -> M7Can2 {
        M7Can2::from_bits(val)
    }
}
impl From<M7Can2> for u8 {
    #[inline(always)]
    fn from(val: M7Can2) -> u8 {
        M7Can2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Edma4 {
    #[doc = "EDMA4 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "EDMA4 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Edma4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Edma4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Edma4 {
    #[inline(always)]
    fn from(val: u8) -> M7Edma4 {
        M7Edma4::from_bits(val)
    }
}
impl From<M7Edma4> for u8 {
    #[inline(always)]
    fn from(val: M7Edma4) -> u8 {
        M7Edma4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Flexio1 {
    #[doc = "FLEXIO1 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "FLEXIO1 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Flexio1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Flexio1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Flexio1 {
    #[inline(always)]
    fn from(val: u8) -> M7Flexio1 {
        M7Flexio1::from_bits(val)
    }
}
impl From<M7Flexio1> for u8 {
    #[inline(always)]
    fn from(val: M7Flexio1) -> u8 {
        M7Flexio1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Flexio2 {
    #[doc = "FLEXIO2 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "FLEXIO2 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Flexio2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Flexio2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Flexio2 {
    #[inline(always)]
    fn from(val: u8) -> M7Flexio2 {
        M7Flexio2::from_bits(val)
    }
}
impl From<M7Flexio2> for u8 {
    #[inline(always)]
    fn from(val: M7Flexio2) -> u8 {
        M7Flexio2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Flexpwm1 {
    #[doc = "FLEXPWM1 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "FLEXPWM1 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Flexpwm1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Flexpwm1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Flexpwm1 {
    #[inline(always)]
    fn from(val: u8) -> M7Flexpwm1 {
        M7Flexpwm1::from_bits(val)
    }
}
impl From<M7Flexpwm1> for u8 {
    #[inline(always)]
    fn from(val: M7Flexpwm1) -> u8 {
        M7Flexpwm1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Flexpwm2 {
    #[doc = "FLEXPWM2 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "FLEXPWM2 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Flexpwm2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Flexpwm2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Flexpwm2 {
    #[inline(always)]
    fn from(val: u8) -> M7Flexpwm2 {
        M7Flexpwm2::from_bits(val)
    }
}
impl From<M7Flexpwm2> for u8 {
    #[inline(always)]
    fn from(val: M7Flexpwm2) -> u8 {
        M7Flexpwm2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Flexpwm3 {
    #[doc = "FLEXPWM3 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "FLEXPWM3 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Flexpwm3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Flexpwm3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Flexpwm3 {
    #[inline(always)]
    fn from(val: u8) -> M7Flexpwm3 {
        M7Flexpwm3::from_bits(val)
    }
}
impl From<M7Flexpwm3> for u8 {
    #[inline(always)]
    fn from(val: M7Flexpwm3) -> u8 {
        M7Flexpwm3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Flexpwm4 {
    #[doc = "FLEXPWM4 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "FLEXPWM4 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Flexpwm4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Flexpwm4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Flexpwm4 {
    #[inline(always)]
    fn from(val: u8) -> M7Flexpwm4 {
        M7Flexpwm4::from_bits(val)
    }
}
impl From<M7Flexpwm4> for u8 {
    #[inline(always)]
    fn from(val: M7Flexpwm4) -> u8 {
        M7Flexpwm4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Gpt2 {
    #[doc = "GPT2 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "GPT2 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Gpt2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Gpt2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Gpt2 {
    #[inline(always)]
    fn from(val: u8) -> M7Gpt2 {
        M7Gpt2::from_bits(val)
    }
}
impl From<M7Gpt2> for u8 {
    #[inline(always)]
    fn from(val: M7Gpt2) -> u8 {
        M7Gpt2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7I3c2 {
    #[doc = "I3C2 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "I3C2 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7I3c2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7I3c2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7I3c2 {
    #[inline(always)]
    fn from(val: u8) -> M7I3c2 {
        M7I3c2::from_bits(val)
    }
}
impl From<M7I3c2> for u8 {
    #[inline(always)]
    fn from(val: M7I3c2) -> u8 {
        M7I3c2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Lpi2c3 {
    #[doc = "LPI2C3 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "LPI2C3 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Lpi2c3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Lpi2c3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Lpi2c3 {
    #[inline(always)]
    fn from(val: u8) -> M7Lpi2c3 {
        M7Lpi2c3::from_bits(val)
    }
}
impl From<M7Lpi2c3> for u8 {
    #[inline(always)]
    fn from(val: M7Lpi2c3) -> u8 {
        M7Lpi2c3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Lpi2c4 {
    #[doc = "LPI2C4 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "LPI2C4 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Lpi2c4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Lpi2c4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Lpi2c4 {
    #[inline(always)]
    fn from(val: u8) -> M7Lpi2c4 {
        M7Lpi2c4::from_bits(val)
    }
}
impl From<M7Lpi2c4> for u8 {
    #[inline(always)]
    fn from(val: M7Lpi2c4) -> u8 {
        M7Lpi2c4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Lpi2c5 {
    #[doc = "LPI2C5 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "LPI2C5 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Lpi2c5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Lpi2c5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Lpi2c5 {
    #[inline(always)]
    fn from(val: u8) -> M7Lpi2c5 {
        M7Lpi2c5::from_bits(val)
    }
}
impl From<M7Lpi2c5> for u8 {
    #[inline(always)]
    fn from(val: M7Lpi2c5) -> u8 {
        M7Lpi2c5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Lpi2c6 {
    #[doc = "LPI2C6\" does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "LPI2C6\" enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Lpi2c6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Lpi2c6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Lpi2c6 {
    #[inline(always)]
    fn from(val: u8) -> M7Lpi2c6 {
        M7Lpi2c6::from_bits(val)
    }
}
impl From<M7Lpi2c6> for u8 {
    #[inline(always)]
    fn from(val: M7Lpi2c6) -> u8 {
        M7Lpi2c6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Lpit2 {
    #[doc = "LPIT2 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "LPIT2 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Lpit2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Lpit2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Lpit2 {
    #[inline(always)]
    fn from(val: u8) -> M7Lpit2 {
        M7Lpit2::from_bits(val)
    }
}
impl From<M7Lpit2> for u8 {
    #[inline(always)]
    fn from(val: M7Lpit2) -> u8 {
        M7Lpit2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Lpit3 {
    #[doc = "LPIT3 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "LPIT3 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Lpit3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Lpit3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Lpit3 {
    #[inline(always)]
    fn from(val: u8) -> M7Lpit3 {
        M7Lpit3::from_bits(val)
    }
}
impl From<M7Lpit3> for u8 {
    #[inline(always)]
    fn from(val: M7Lpit3) -> u8 {
        M7Lpit3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Lpspi3 {
    #[doc = "WDOD3 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "WDOG3 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Lpspi3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Lpspi3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Lpspi3 {
    #[inline(always)]
    fn from(val: u8) -> M7Lpspi3 {
        M7Lpspi3::from_bits(val)
    }
}
impl From<M7Lpspi3> for u8 {
    #[inline(always)]
    fn from(val: M7Lpspi3) -> u8 {
        M7Lpspi3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Lpspi4 {
    #[doc = "LPSPI4 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "LPSPI4 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Lpspi4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Lpspi4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Lpspi4 {
    #[inline(always)]
    fn from(val: u8) -> M7Lpspi4 {
        M7Lpspi4::from_bits(val)
    }
}
impl From<M7Lpspi4> for u8 {
    #[inline(always)]
    fn from(val: M7Lpspi4) -> u8 {
        M7Lpspi4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Lpspi5 {
    #[doc = "LPTMR3 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Lpspi5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Lpspi5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Lpspi5 {
    #[inline(always)]
    fn from(val: u8) -> M7Lpspi5 {
        M7Lpspi5::from_bits(val)
    }
}
impl From<M7Lpspi5> for u8 {
    #[inline(always)]
    fn from(val: M7Lpspi5) -> u8 {
        M7Lpspi5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Lpspi6 {
    #[doc = "LPSPI6 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "LPSPI6 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Lpspi6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Lpspi6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Lpspi6 {
    #[inline(always)]
    fn from(val: u8) -> M7Lpspi6 {
        M7Lpspi6::from_bits(val)
    }
}
impl From<M7Lpspi6> for u8 {
    #[inline(always)]
    fn from(val: M7Lpspi6) -> u8 {
        M7Lpspi6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Lptmr2 {
    #[doc = "LPTMR2 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "LPTMR2 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Lptmr2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Lptmr2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Lptmr2 {
    #[inline(always)]
    fn from(val: u8) -> M7Lptmr2 {
        M7Lptmr2::from_bits(val)
    }
}
impl From<M7Lptmr2> for u8 {
    #[inline(always)]
    fn from(val: M7Lptmr2) -> u8 {
        M7Lptmr2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Lptmr3 {
    #[doc = "LPTMR3 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "LPTMR3 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Lptmr3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Lptmr3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Lptmr3 {
    #[inline(always)]
    fn from(val: u8) -> M7Lptmr3 {
        M7Lptmr3::from_bits(val)
    }
}
impl From<M7Lptmr3> for u8 {
    #[inline(always)]
    fn from(val: M7Lptmr3) -> u8 {
        M7Lptmr3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Mic {
    #[doc = "MIC does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "MIC enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Mic {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Mic {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Mic {
    #[inline(always)]
    fn from(val: u8) -> M7Mic {
        M7Mic::from_bits(val)
    }
}
impl From<M7Mic> for u8 {
    #[inline(always)]
    fn from(val: M7Mic) -> u8 {
        M7Mic::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Qtimer1 {
    #[doc = "QTIMER1 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "QTIMER1 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Qtimer1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Qtimer1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Qtimer1 {
    #[inline(always)]
    fn from(val: u8) -> M7Qtimer1 {
        M7Qtimer1::from_bits(val)
    }
}
impl From<M7Qtimer1> for u8 {
    #[inline(always)]
    fn from(val: M7Qtimer1) -> u8 {
        M7Qtimer1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Qtimer2 {
    #[doc = "QTIMER2 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "QTIMER2 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Qtimer2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Qtimer2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Qtimer2 {
    #[inline(always)]
    fn from(val: u8) -> M7Qtimer2 {
        M7Qtimer2::from_bits(val)
    }
}
impl From<M7Qtimer2> for u8 {
    #[inline(always)]
    fn from(val: M7Qtimer2) -> u8 {
        M7Qtimer2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Qtimer3 {
    #[doc = "QTIMER3 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "QTIMER3 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Qtimer3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Qtimer3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Qtimer3 {
    #[inline(always)]
    fn from(val: u8) -> M7Qtimer3 {
        M7Qtimer3::from_bits(val)
    }
}
impl From<M7Qtimer3> for u8 {
    #[inline(always)]
    fn from(val: M7Qtimer3) -> u8 {
        M7Qtimer3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Qtimer4 {
    #[doc = "QTIMER4 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "QTIMER4 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Qtimer4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Qtimer4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Qtimer4 {
    #[inline(always)]
    fn from(val: u8) -> M7Qtimer4 {
        M7Qtimer4::from_bits(val)
    }
}
impl From<M7Qtimer4> for u8 {
    #[inline(always)]
    fn from(val: M7Qtimer4) -> u8 {
        M7Qtimer4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Qtimer5 {
    #[doc = "QTIMER5 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "QTIMER5 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Qtimer5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Qtimer5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Qtimer5 {
    #[inline(always)]
    fn from(val: u8) -> M7Qtimer5 {
        M7Qtimer5::from_bits(val)
    }
}
impl From<M7Qtimer5> for u8 {
    #[inline(always)]
    fn from(val: M7Qtimer5) -> u8 {
        M7Qtimer5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Qtimer6 {
    #[doc = "does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Qtimer6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Qtimer6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Qtimer6 {
    #[inline(always)]
    fn from(val: u8) -> M7Qtimer6 {
        M7Qtimer6::from_bits(val)
    }
}
impl From<M7Qtimer6> for u8 {
    #[inline(always)]
    fn from(val: M7Qtimer6) -> u8 {
        M7Qtimer6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Qtimer7 {
    #[doc = "QTIMER7 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "QTIMER7 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Qtimer7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Qtimer7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Qtimer7 {
    #[inline(always)]
    fn from(val: u8) -> M7Qtimer7 {
        M7Qtimer7::from_bits(val)
    }
}
impl From<M7Qtimer7> for u8 {
    #[inline(always)]
    fn from(val: M7Qtimer7) -> u8 {
        M7Qtimer7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Qtimer8 {
    #[doc = "QTIMER8 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "QTIMER8 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Qtimer8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Qtimer8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Qtimer8 {
    #[inline(always)]
    fn from(val: u8) -> M7Qtimer8 {
        M7Qtimer8::from_bits(val)
    }
}
impl From<M7Qtimer8> for u8 {
    #[inline(always)]
    fn from(val: M7Qtimer8) -> u8 {
        M7Qtimer8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Sai2 {
    #[doc = "SAI2 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "SAI2 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Sai2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Sai2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Sai2 {
    #[inline(always)]
    fn from(val: u8) -> M7Sai2 {
        M7Sai2::from_bits(val)
    }
}
impl From<M7Sai2> for u8 {
    #[inline(always)]
    fn from(val: M7Sai2) -> u8 {
        M7Sai2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Sai3 {
    #[doc = "SAI3 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "SAI3 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Sai3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Sai3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Sai3 {
    #[inline(always)]
    fn from(val: u8) -> M7Sai3 {
        M7Sai3::from_bits(val)
    }
}
impl From<M7Sai3> for u8 {
    #[inline(always)]
    fn from(val: M7Sai3) -> u8 {
        M7Sai3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Sai4 {
    #[doc = "SAI4 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "SAI4 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Sai4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Sai4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Sai4 {
    #[inline(always)]
    fn from(val: u8) -> M7Sai4 {
        M7Sai4::from_bits(val)
    }
}
impl From<M7Sai4> for u8 {
    #[inline(always)]
    fn from(val: M7Sai4) -> u8 {
        M7Sai4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Sinc1 {
    #[doc = "SINC1 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "SINC1 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Sinc1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Sinc1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Sinc1 {
    #[inline(always)]
    fn from(val: u8) -> M7Sinc1 {
        M7Sinc1::from_bits(val)
    }
}
impl From<M7Sinc1> for u8 {
    #[inline(always)]
    fn from(val: M7Sinc1) -> u8 {
        M7Sinc1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Sinc2 {
    #[doc = "SINC2 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "SINC2 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Sinc2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Sinc2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Sinc2 {
    #[inline(always)]
    fn from(val: u8) -> M7Sinc2 {
        M7Sinc2::from_bits(val)
    }
}
impl From<M7Sinc2> for u8 {
    #[inline(always)]
    fn from(val: M7Sinc2) -> u8 {
        M7Sinc2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Sinc3 {
    #[doc = "SINC3 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "SINC3 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Sinc3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Sinc3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Sinc3 {
    #[inline(always)]
    fn from(val: u8) -> M7Sinc3 {
        M7Sinc3::from_bits(val)
    }
}
impl From<M7Sinc3> for u8 {
    #[inline(always)]
    fn from(val: M7Sinc3) -> u8 {
        M7Sinc3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Tpm3 {
    #[doc = "TPM3 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "TPM3 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Tpm3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Tpm3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Tpm3 {
    #[inline(always)]
    fn from(val: u8) -> M7Tpm3 {
        M7Tpm3::from_bits(val)
    }
}
impl From<M7Tpm3> for u8 {
    #[inline(always)]
    fn from(val: M7Tpm3) -> u8 {
        M7Tpm3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Tpm4 {
    #[doc = "TPM4 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "TPM4 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Tpm4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Tpm4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Tpm4 {
    #[inline(always)]
    fn from(val: u8) -> M7Tpm4 {
        M7Tpm4::from_bits(val)
    }
}
impl From<M7Tpm4> for u8 {
    #[inline(always)]
    fn from(val: M7Tpm4) -> u8 {
        M7Tpm4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Tpm5 {
    #[doc = "TPM5 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "TPM5 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Tpm5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Tpm5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Tpm5 {
    #[inline(always)]
    fn from(val: u8) -> M7Tpm5 {
        M7Tpm5::from_bits(val)
    }
}
impl From<M7Tpm5> for u8 {
    #[inline(always)]
    fn from(val: M7Tpm5) -> u8 {
        M7Tpm5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Tpm6 {
    #[doc = "TPM5 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "TPM5 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Tpm6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Tpm6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Tpm6 {
    #[inline(always)]
    fn from(val: u8) -> M7Tpm6 {
        M7Tpm6::from_bits(val)
    }
}
impl From<M7Tpm6> for u8 {
    #[inline(always)]
    fn from(val: M7Tpm6) -> u8 {
        M7Tpm6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Wdog5 {
    #[doc = "WDOG5 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "WDOG5 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Wdog5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Wdog5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Wdog5 {
    #[inline(always)]
    fn from(val: u8) -> M7Wdog5 {
        M7Wdog5::from_bits(val)
    }
}
impl From<M7Wdog5> for u8 {
    #[inline(always)]
    fn from(val: M7Wdog5) -> u8 {
        M7Wdog5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcLinkCfg0IoVar {
    #[doc = "None"]
    NONE = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl NetcLinkCfg0IoVar {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcLinkCfg0IoVar {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcLinkCfg0IoVar {
    #[inline(always)]
    fn from(val: u8) -> NetcLinkCfg0IoVar {
        NetcLinkCfg0IoVar::from_bits(val)
    }
}
impl From<NetcLinkCfg0IoVar> for u8 {
    #[inline(always)]
    fn from(val: NetcLinkCfg0IoVar) -> u8 {
        NetcLinkCfg0IoVar::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcLinkCfg0MiiProt {
    #[doc = "MII"]
    MII = 0x0,
    #[doc = "RMII"]
    RMII = 0x01,
    #[doc = "RGMII"]
    RGMII = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl NetcLinkCfg0MiiProt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcLinkCfg0MiiProt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcLinkCfg0MiiProt {
    #[inline(always)]
    fn from(val: u8) -> NetcLinkCfg0MiiProt {
        NetcLinkCfg0MiiProt::from_bits(val)
    }
}
impl From<NetcLinkCfg0MiiProt> for u8 {
    #[inline(always)]
    fn from(val: NetcLinkCfg0MiiProt) -> u8 {
        NetcLinkCfg0MiiProt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcLinkCfg1IoVar {
    #[doc = "None"]
    NONE = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl NetcLinkCfg1IoVar {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcLinkCfg1IoVar {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcLinkCfg1IoVar {
    #[inline(always)]
    fn from(val: u8) -> NetcLinkCfg1IoVar {
        NetcLinkCfg1IoVar::from_bits(val)
    }
}
impl From<NetcLinkCfg1IoVar> for u8 {
    #[inline(always)]
    fn from(val: NetcLinkCfg1IoVar) -> u8 {
        NetcLinkCfg1IoVar::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcLinkCfg1MiiProt {
    #[doc = "MII"]
    MII = 0x0,
    #[doc = "RMII"]
    RMII = 0x01,
    #[doc = "RGMII"]
    RGMII = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl NetcLinkCfg1MiiProt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcLinkCfg1MiiProt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcLinkCfg1MiiProt {
    #[inline(always)]
    fn from(val: u8) -> NetcLinkCfg1MiiProt {
        NetcLinkCfg1MiiProt::from_bits(val)
    }
}
impl From<NetcLinkCfg1MiiProt> for u8 {
    #[inline(always)]
    fn from(val: NetcLinkCfg1MiiProt) -> u8 {
        NetcLinkCfg1MiiProt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcLinkCfg2IoVar {
    #[doc = "None"]
    DISABLE = 0x0,
    #[doc = "no description available"]
    RES_1 = 0x01,
    #[doc = "no description available"]
    RES_2 = 0x02,
    #[doc = "no description available"]
    RES_3 = 0x03,
    #[doc = "no description available"]
    RES_4 = 0x04,
    #[doc = "no description available"]
    RES_5 = 0x05,
    #[doc = "no description available"]
    RES_6 = 0x06,
    #[doc = "no description available"]
    RES_7 = 0x07,
    #[doc = "no description available"]
    RES_8 = 0x08,
    #[doc = "no description available"]
    RES_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl NetcLinkCfg2IoVar {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcLinkCfg2IoVar {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcLinkCfg2IoVar {
    #[inline(always)]
    fn from(val: u8) -> NetcLinkCfg2IoVar {
        NetcLinkCfg2IoVar::from_bits(val)
    }
}
impl From<NetcLinkCfg2IoVar> for u8 {
    #[inline(always)]
    fn from(val: NetcLinkCfg2IoVar) -> u8 {
        NetcLinkCfg2IoVar::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcLinkCfg2MiiProt {
    #[doc = "MII"]
    MII = 0x0,
    #[doc = "RMII"]
    RMII = 0x01,
    #[doc = "RGMII"]
    RGMII = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl NetcLinkCfg2MiiProt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcLinkCfg2MiiProt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcLinkCfg2MiiProt {
    #[inline(always)]
    fn from(val: u8) -> NetcLinkCfg2MiiProt {
        NetcLinkCfg2MiiProt::from_bits(val)
    }
}
impl From<NetcLinkCfg2MiiProt> for u8 {
    #[inline(always)]
    fn from(val: NetcLinkCfg2MiiProt) -> u8 {
        NetcLinkCfg2MiiProt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PauseMode {
    #[doc = "WAKEUP Domain to M7 SSI master will enter pause mode"]
    EXIT = 0x0,
    #[doc = "WAKEUP Domain to M7 SSI master will exit pause mode"]
    ENTER = 0x01,
}
impl PauseMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PauseMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PauseMode {
    #[inline(always)]
    fn from(val: u8) -> PauseMode {
        PauseMode::from_bits(val)
    }
}
impl From<PauseMode> for u8 {
    #[inline(always)]
    fn from(val: PauseMode) -> u8 {
        PauseMode::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct PolSel(pub u8);
impl PolSel {
    #[doc = "Same as trigger in"]
    pub const SAME: Self = Self(0x0);
    #[doc = "Invert trigger in"]
    pub const INV: Self = Self(0x01);
}
impl PolSel {
    pub const fn from_bits(val: u8) -> PolSel {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for PolSel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("SAME"),
            0x01 => f.write_str("INV"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PolSel {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "SAME"),
            0x01 => defmt::write!(f, "INV"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for PolSel {
    #[inline(always)]
    fn from(val: u8) -> PolSel {
        PolSel::from_bits(val)
    }
}
impl From<PolSel> for u8 {
    #[inline(always)]
    fn from(val: PolSel) -> u8 {
        PolSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Port1RmiiRefClkDir {
    #[doc = "Port1 RMII Reference clock is input"]
    INPUT = 0x0,
    #[doc = "Port1 RMII Reference clock is output"]
    OUTPUT = 0x01,
}
impl Port1RmiiRefClkDir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Port1RmiiRefClkDir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Port1RmiiRefClkDir {
    #[inline(always)]
    fn from(val: u8) -> Port1RmiiRefClkDir {
        Port1RmiiRefClkDir::from_bits(val)
    }
}
impl From<Port1RmiiRefClkDir> for u8 {
    #[inline(always)]
    fn from(val: Port1RmiiRefClkDir) -> u8 {
        Port1RmiiRefClkDir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Port2RmiiRefClkDir {
    #[doc = "Port2 RMII Reference clock is input"]
    INPUT = 0x0,
    #[doc = "Port2 RMII Reference clock is output"]
    OUTPUT = 0x01,
}
impl Port2RmiiRefClkDir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Port2RmiiRefClkDir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Port2RmiiRefClkDir {
    #[inline(always)]
    fn from(val: u8) -> Port2RmiiRefClkDir {
        Port2RmiiRefClkDir::from_bits(val)
    }
}
impl From<Port2RmiiRefClkDir> for u8 {
    #[inline(always)]
    fn from(val: Port2RmiiRefClkDir) -> u8 {
        Port2RmiiRefClkDir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Port3RmiiRefClkDir {
    #[doc = "Port3 RMII Reference clock is input"]
    INPUT = 0x0,
    #[doc = "Port3 RMII Reference clock is output"]
    OUTPUT = 0x01,
}
impl Port3RmiiRefClkDir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Port3RmiiRefClkDir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Port3RmiiRefClkDir {
    #[inline(always)]
    fn from(val: u8) -> Port3RmiiRefClkDir {
        Port3RmiiRefClkDir::from_bits(val)
    }
}
impl From<Port3RmiiRefClkDir> for u8 {
    #[inline(always)]
    fn from(val: Port3RmiiRefClkDir) -> u8 {
        Port3RmiiRefClkDir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Port4RmiiRefClkDir {
    #[doc = "Port4 RMII Reference clock is input"]
    INPUT = 0x0,
    #[doc = "Port4 RMII Reference clock is output"]
    OUTPUT = 0x01,
}
impl Port4RmiiRefClkDir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Port4RmiiRefClkDir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Port4RmiiRefClkDir {
    #[inline(always)]
    fn from(val: u8) -> Port4RmiiRefClkDir {
        Port4RmiiRefClkDir::from_bits(val)
    }
}
impl From<Port4RmiiRefClkDir> for u8 {
    #[inline(always)]
    fn from(val: Port4RmiiRefClkDir) -> u8 {
        Port4RmiiRefClkDir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pre {
    #[doc = "Time granularity is 1 clock cycle."]
    PRE0 = 0x0,
    #[doc = "Time granularity is 2 clock cycles."]
    PRE1 = 0x01,
    #[doc = "Time granularity is 3 clock cycles."]
    PRE2 = 0x02,
    #[doc = "Time granularity is 4 clock cycles."]
    PRE3 = 0x03,
}
impl Pre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pre {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pre {
    #[inline(always)]
    fn from(val: u8) -> Pre {
        Pre::from_bits(val)
    }
}
impl From<Pre> for u8 {
    #[inline(always)]
    fn from(val: Pre) -> u8 {
        Pre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ps {
    #[doc = "8bit"]
    PS8 = 0x0,
    #[doc = "16bit"]
    PS16 = 0x01,
}
impl Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ps {
    #[inline(always)]
    fn from(val: u8) -> Ps {
        Ps::from_bits(val)
    }
}
impl From<Ps> for u8 {
    #[inline(always)]
    fn from(val: Ps) -> u8 {
        Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer1Tmr0InputSel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl Qtimer1Tmr0InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer1Tmr0InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer1Tmr0InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer1Tmr0InputSel {
        Qtimer1Tmr0InputSel::from_bits(val)
    }
}
impl From<Qtimer1Tmr0InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer1Tmr0InputSel) -> u8 {
        Qtimer1Tmr0InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer1Tmr1InputSel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl Qtimer1Tmr1InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer1Tmr1InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer1Tmr1InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer1Tmr1InputSel {
        Qtimer1Tmr1InputSel::from_bits(val)
    }
}
impl From<Qtimer1Tmr1InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer1Tmr1InputSel) -> u8 {
        Qtimer1Tmr1InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer1Tmr2InputSel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl Qtimer1Tmr2InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer1Tmr2InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer1Tmr2InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer1Tmr2InputSel {
        Qtimer1Tmr2InputSel::from_bits(val)
    }
}
impl From<Qtimer1Tmr2InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer1Tmr2InputSel) -> u8 {
        Qtimer1Tmr2InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer1Tmr3InputSel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl Qtimer1Tmr3InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer1Tmr3InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer1Tmr3InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer1Tmr3InputSel {
        Qtimer1Tmr3InputSel::from_bits(val)
    }
}
impl From<Qtimer1Tmr3InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer1Tmr3InputSel) -> u8 {
        Qtimer1Tmr3InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer1TmrCntsFreeze {
    #[doc = "Timer counter works normally"]
    NORMAL = 0x0,
    #[doc = "Reset counter and output flags"]
    FLAGS = 0x01,
}
impl Qtimer1TmrCntsFreeze {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer1TmrCntsFreeze {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer1TmrCntsFreeze {
    #[inline(always)]
    fn from(val: u8) -> Qtimer1TmrCntsFreeze {
        Qtimer1TmrCntsFreeze::from_bits(val)
    }
}
impl From<Qtimer1TmrCntsFreeze> for u8 {
    #[inline(always)]
    fn from(val: Qtimer1TmrCntsFreeze) -> u8 {
        Qtimer1TmrCntsFreeze::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer2Tmr0InputSel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl Qtimer2Tmr0InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer2Tmr0InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer2Tmr0InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer2Tmr0InputSel {
        Qtimer2Tmr0InputSel::from_bits(val)
    }
}
impl From<Qtimer2Tmr0InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer2Tmr0InputSel) -> u8 {
        Qtimer2Tmr0InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer2Tmr1InputSel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl Qtimer2Tmr1InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer2Tmr1InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer2Tmr1InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer2Tmr1InputSel {
        Qtimer2Tmr1InputSel::from_bits(val)
    }
}
impl From<Qtimer2Tmr1InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer2Tmr1InputSel) -> u8 {
        Qtimer2Tmr1InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer2Tmr2InputSel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl Qtimer2Tmr2InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer2Tmr2InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer2Tmr2InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer2Tmr2InputSel {
        Qtimer2Tmr2InputSel::from_bits(val)
    }
}
impl From<Qtimer2Tmr2InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer2Tmr2InputSel) -> u8 {
        Qtimer2Tmr2InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer2Tmr3InputSel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl Qtimer2Tmr3InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer2Tmr3InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer2Tmr3InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer2Tmr3InputSel {
        Qtimer2Tmr3InputSel::from_bits(val)
    }
}
impl From<Qtimer2Tmr3InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer2Tmr3InputSel) -> u8 {
        Qtimer2Tmr3InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer2TmrCntsFreeze {
    #[doc = "Timer counter works normally"]
    NORMAL = 0x0,
    #[doc = "Reset counter and output flags"]
    FLAGS = 0x01,
}
impl Qtimer2TmrCntsFreeze {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer2TmrCntsFreeze {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer2TmrCntsFreeze {
    #[inline(always)]
    fn from(val: u8) -> Qtimer2TmrCntsFreeze {
        Qtimer2TmrCntsFreeze::from_bits(val)
    }
}
impl From<Qtimer2TmrCntsFreeze> for u8 {
    #[inline(always)]
    fn from(val: Qtimer2TmrCntsFreeze) -> u8 {
        Qtimer2TmrCntsFreeze::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer3Tmr0InputSel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl Qtimer3Tmr0InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer3Tmr0InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer3Tmr0InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer3Tmr0InputSel {
        Qtimer3Tmr0InputSel::from_bits(val)
    }
}
impl From<Qtimer3Tmr0InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer3Tmr0InputSel) -> u8 {
        Qtimer3Tmr0InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer3Tmr1InputSel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl Qtimer3Tmr1InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer3Tmr1InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer3Tmr1InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer3Tmr1InputSel {
        Qtimer3Tmr1InputSel::from_bits(val)
    }
}
impl From<Qtimer3Tmr1InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer3Tmr1InputSel) -> u8 {
        Qtimer3Tmr1InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer3Tmr2InputSel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl Qtimer3Tmr2InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer3Tmr2InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer3Tmr2InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer3Tmr2InputSel {
        Qtimer3Tmr2InputSel::from_bits(val)
    }
}
impl From<Qtimer3Tmr2InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer3Tmr2InputSel) -> u8 {
        Qtimer3Tmr2InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer3Tmr3InputSel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl Qtimer3Tmr3InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer3Tmr3InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer3Tmr3InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer3Tmr3InputSel {
        Qtimer3Tmr3InputSel::from_bits(val)
    }
}
impl From<Qtimer3Tmr3InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer3Tmr3InputSel) -> u8 {
        Qtimer3Tmr3InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer3TmrCntsFreeze {
    #[doc = "Timer counter works normally"]
    NORMAL = 0x0,
    #[doc = "Reset counter and ouput flags"]
    FLAGS = 0x01,
}
impl Qtimer3TmrCntsFreeze {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer3TmrCntsFreeze {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer3TmrCntsFreeze {
    #[inline(always)]
    fn from(val: u8) -> Qtimer3TmrCntsFreeze {
        Qtimer3TmrCntsFreeze::from_bits(val)
    }
}
impl From<Qtimer3TmrCntsFreeze> for u8 {
    #[inline(always)]
    fn from(val: Qtimer3TmrCntsFreeze) -> u8 {
        Qtimer3TmrCntsFreeze::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer4Tmr0InputSel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl Qtimer4Tmr0InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer4Tmr0InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer4Tmr0InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer4Tmr0InputSel {
        Qtimer4Tmr0InputSel::from_bits(val)
    }
}
impl From<Qtimer4Tmr0InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer4Tmr0InputSel) -> u8 {
        Qtimer4Tmr0InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer4Tmr1InputSel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl Qtimer4Tmr1InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer4Tmr1InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer4Tmr1InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer4Tmr1InputSel {
        Qtimer4Tmr1InputSel::from_bits(val)
    }
}
impl From<Qtimer4Tmr1InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer4Tmr1InputSel) -> u8 {
        Qtimer4Tmr1InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer4Tmr2InputSel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl Qtimer4Tmr2InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer4Tmr2InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer4Tmr2InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer4Tmr2InputSel {
        Qtimer4Tmr2InputSel::from_bits(val)
    }
}
impl From<Qtimer4Tmr2InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer4Tmr2InputSel) -> u8 {
        Qtimer4Tmr2InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer4Tmr3InputSel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl Qtimer4Tmr3InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer4Tmr3InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer4Tmr3InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer4Tmr3InputSel {
        Qtimer4Tmr3InputSel::from_bits(val)
    }
}
impl From<Qtimer4Tmr3InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer4Tmr3InputSel) -> u8 {
        Qtimer4Tmr3InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer4TmrCntsFreeze {
    #[doc = "Timer counter works normally"]
    NORMAL = 0x0,
    #[doc = "Reset counter and output flags"]
    FLAGS = 0x01,
}
impl Qtimer4TmrCntsFreeze {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer4TmrCntsFreeze {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer4TmrCntsFreeze {
    #[inline(always)]
    fn from(val: u8) -> Qtimer4TmrCntsFreeze {
        Qtimer4TmrCntsFreeze::from_bits(val)
    }
}
impl From<Qtimer4TmrCntsFreeze> for u8 {
    #[inline(always)]
    fn from(val: Qtimer4TmrCntsFreeze) -> u8 {
        Qtimer4TmrCntsFreeze::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer5Tmr0InputSel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl Qtimer5Tmr0InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer5Tmr0InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer5Tmr0InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer5Tmr0InputSel {
        Qtimer5Tmr0InputSel::from_bits(val)
    }
}
impl From<Qtimer5Tmr0InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer5Tmr0InputSel) -> u8 {
        Qtimer5Tmr0InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer5Tmr1InputSel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl Qtimer5Tmr1InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer5Tmr1InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer5Tmr1InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer5Tmr1InputSel {
        Qtimer5Tmr1InputSel::from_bits(val)
    }
}
impl From<Qtimer5Tmr1InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer5Tmr1InputSel) -> u8 {
        Qtimer5Tmr1InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer5Tmr2InputSel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl Qtimer5Tmr2InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer5Tmr2InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer5Tmr2InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer5Tmr2InputSel {
        Qtimer5Tmr2InputSel::from_bits(val)
    }
}
impl From<Qtimer5Tmr2InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer5Tmr2InputSel) -> u8 {
        Qtimer5Tmr2InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer5Tmr3InputSel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl Qtimer5Tmr3InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer5Tmr3InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer5Tmr3InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer5Tmr3InputSel {
        Qtimer5Tmr3InputSel::from_bits(val)
    }
}
impl From<Qtimer5Tmr3InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer5Tmr3InputSel) -> u8 {
        Qtimer5Tmr3InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer5TmrCntsFreeze {
    #[doc = "Timer counter works normally"]
    NORMAL_COUNTER = 0x0,
    #[doc = "Reset counter and output flags"]
    RESET_COUNTER = 0x01,
}
impl Qtimer5TmrCntsFreeze {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer5TmrCntsFreeze {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer5TmrCntsFreeze {
    #[inline(always)]
    fn from(val: u8) -> Qtimer5TmrCntsFreeze {
        Qtimer5TmrCntsFreeze::from_bits(val)
    }
}
impl From<Qtimer5TmrCntsFreeze> for u8 {
    #[inline(always)]
    fn from(val: Qtimer5TmrCntsFreeze) -> u8 {
        Qtimer5TmrCntsFreeze::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer6Tmr0InputSel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl Qtimer6Tmr0InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer6Tmr0InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer6Tmr0InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer6Tmr0InputSel {
        Qtimer6Tmr0InputSel::from_bits(val)
    }
}
impl From<Qtimer6Tmr0InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer6Tmr0InputSel) -> u8 {
        Qtimer6Tmr0InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer6Tmr1InputSel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl Qtimer6Tmr1InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer6Tmr1InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer6Tmr1InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer6Tmr1InputSel {
        Qtimer6Tmr1InputSel::from_bits(val)
    }
}
impl From<Qtimer6Tmr1InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer6Tmr1InputSel) -> u8 {
        Qtimer6Tmr1InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer6Tmr2InputSel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl Qtimer6Tmr2InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer6Tmr2InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer6Tmr2InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer6Tmr2InputSel {
        Qtimer6Tmr2InputSel::from_bits(val)
    }
}
impl From<Qtimer6Tmr2InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer6Tmr2InputSel) -> u8 {
        Qtimer6Tmr2InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer6Tmr3InputSel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl Qtimer6Tmr3InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer6Tmr3InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer6Tmr3InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer6Tmr3InputSel {
        Qtimer6Tmr3InputSel::from_bits(val)
    }
}
impl From<Qtimer6Tmr3InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer6Tmr3InputSel) -> u8 {
        Qtimer6Tmr3InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer7Tmr0InputSel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl Qtimer7Tmr0InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer7Tmr0InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer7Tmr0InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer7Tmr0InputSel {
        Qtimer7Tmr0InputSel::from_bits(val)
    }
}
impl From<Qtimer7Tmr0InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer7Tmr0InputSel) -> u8 {
        Qtimer7Tmr0InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer7Tmr1InputSel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl Qtimer7Tmr1InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer7Tmr1InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer7Tmr1InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer7Tmr1InputSel {
        Qtimer7Tmr1InputSel::from_bits(val)
    }
}
impl From<Qtimer7Tmr1InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer7Tmr1InputSel) -> u8 {
        Qtimer7Tmr1InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer7Tmr2InputSel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl Qtimer7Tmr2InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer7Tmr2InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer7Tmr2InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer7Tmr2InputSel {
        Qtimer7Tmr2InputSel::from_bits(val)
    }
}
impl From<Qtimer7Tmr2InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer7Tmr2InputSel) -> u8 {
        Qtimer7Tmr2InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer7Tmr3InputSel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl Qtimer7Tmr3InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer7Tmr3InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer7Tmr3InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer7Tmr3InputSel {
        Qtimer7Tmr3InputSel::from_bits(val)
    }
}
impl From<Qtimer7Tmr3InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer7Tmr3InputSel) -> u8 {
        Qtimer7Tmr3InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer7TmrCntsFreeze {
    #[doc = "Timer counter works normally"]
    NORMAL_COUNTER = 0x0,
    #[doc = "Reset counter and output flags"]
    RESET_COUNTER = 0x01,
}
impl Qtimer7TmrCntsFreeze {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer7TmrCntsFreeze {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer7TmrCntsFreeze {
    #[inline(always)]
    fn from(val: u8) -> Qtimer7TmrCntsFreeze {
        Qtimer7TmrCntsFreeze::from_bits(val)
    }
}
impl From<Qtimer7TmrCntsFreeze> for u8 {
    #[inline(always)]
    fn from(val: Qtimer7TmrCntsFreeze) -> u8 {
        Qtimer7TmrCntsFreeze::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer8Tmr0InputSel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl Qtimer8Tmr0InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer8Tmr0InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer8Tmr0InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer8Tmr0InputSel {
        Qtimer8Tmr0InputSel::from_bits(val)
    }
}
impl From<Qtimer8Tmr0InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer8Tmr0InputSel) -> u8 {
        Qtimer8Tmr0InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer8Tmr1InputSel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl Qtimer8Tmr1InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer8Tmr1InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer8Tmr1InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer8Tmr1InputSel {
        Qtimer8Tmr1InputSel::from_bits(val)
    }
}
impl From<Qtimer8Tmr1InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer8Tmr1InputSel) -> u8 {
        Qtimer8Tmr1InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer8Tmr2InputSel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl Qtimer8Tmr2InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer8Tmr2InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer8Tmr2InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer8Tmr2InputSel {
        Qtimer8Tmr2InputSel::from_bits(val)
    }
}
impl From<Qtimer8Tmr2InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer8Tmr2InputSel) -> u8 {
        Qtimer8Tmr2InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer8Tmr3InputSel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl Qtimer8Tmr3InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer8Tmr3InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer8Tmr3InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer8Tmr3InputSel {
        Qtimer8Tmr3InputSel::from_bits(val)
    }
}
impl From<Qtimer8Tmr3InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer8Tmr3InputSel) -> u8 {
        Qtimer8Tmr3InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer8TmrCntsFreeze {
    #[doc = "Timer counter works normally"]
    NORMAL = 0x0,
    #[doc = "Reset counter and output flags"]
    FLAGS = 0x01,
}
impl Qtimer8TmrCntsFreeze {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer8TmrCntsFreeze {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer8TmrCntsFreeze {
    #[inline(always)]
    fn from(val: u8) -> Qtimer8TmrCntsFreeze {
        Qtimer8TmrCntsFreeze::from_bits(val)
    }
}
impl From<Qtimer8TmrCntsFreeze> for u8 {
    #[inline(always)]
    fn from(val: Qtimer8TmrCntsFreeze) -> u8 {
        Qtimer8TmrCntsFreeze::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RevmiiRate {
    #[doc = "MII interface is operating at 100Mbps"]
    MII_100 = 0x0,
    #[doc = "MII interface is operating at 10Mbps"]
    MII_10 = 0x01,
}
impl RevmiiRate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RevmiiRate {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RevmiiRate {
    #[inline(always)]
    fn from(val: u8) -> RevmiiRate {
        RevmiiRate::from_bits(val)
    }
}
impl From<RevmiiRate> for u8 {
    #[inline(always)]
    fn from(val: RevmiiRate) -> u8 {
        RevmiiRate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RmiiRefClkDir0 {
    #[doc = "RMII REF_CLK is input"]
    INPUT = 0x0,
    #[doc = "RMII REF_CLK is output driven by ECAT_CLK_ROOT/2"]
    OUTPUT = 0x01,
}
impl RmiiRefClkDir0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RmiiRefClkDir0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RmiiRefClkDir0 {
    #[inline(always)]
    fn from(val: u8) -> RmiiRefClkDir0 {
        RmiiRefClkDir0::from_bits(val)
    }
}
impl From<RmiiRefClkDir0> for u8 {
    #[inline(always)]
    fn from(val: RmiiRefClkDir0) -> u8 {
        RmiiRefClkDir0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RmiiRefClkDir1 {
    #[doc = "RMII REF_CLK is input"]
    INPUT = 0x0,
    #[doc = "RMII REF_CLK is output driven by ECAT_CLK_ROOT/2"]
    OUTPUT = 0x01,
}
impl RmiiRefClkDir1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RmiiRefClkDir1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RmiiRefClkDir1 {
    #[inline(always)]
    fn from(val: u8) -> RmiiRefClkDir1 {
        RmiiRefClkDir1::from_bits(val)
    }
}
impl From<RmiiRefClkDir1> for u8 {
    #[inline(always)]
    fn from(val: RmiiRefClkDir1) -> u8 {
        RmiiRefClkDir1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RmiiSel0 {
    #[doc = "EtherCAT port0 is in MII mode"]
    MII = 0x0,
    #[doc = "EtherCAT port0 is in RMII mode"]
    RMII = 0x01,
}
impl RmiiSel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RmiiSel0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RmiiSel0 {
    #[inline(always)]
    fn from(val: u8) -> RmiiSel0 {
        RmiiSel0::from_bits(val)
    }
}
impl From<RmiiSel0> for u8 {
    #[inline(always)]
    fn from(val: RmiiSel0) -> u8 {
        RmiiSel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RmiiSel1 {
    #[doc = "EtherCAT port1 is in MII mode"]
    MII = 0x0,
    #[doc = "EtherCAT port1 is in RMII mode"]
    RMII = 0x01,
}
impl RmiiSel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RmiiSel1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RmiiSel1 {
    #[inline(always)]
    fn from(val: u8) -> RmiiSel1 {
        RmiiSel1::from_bits(val)
    }
}
impl From<RmiiSel1> for u8 {
    #[inline(always)]
    fn from(val: RmiiSel1) -> u8 {
        RmiiSel1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai2Mclk3Sel {
    #[doc = "SPDIF_CLK_ROOT"]
    CLK_ROOT = 0x0,
    #[doc = "spdif_tx_clk2"]
    TX_CLK2 = 0x01,
    #[doc = "spdif_srclk"]
    SRCLK = 0x02,
    #[doc = "spdif_outclock"]
    OUTCLK = 0x03,
}
impl Sai2Mclk3Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai2Mclk3Sel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai2Mclk3Sel {
    #[inline(always)]
    fn from(val: u8) -> Sai2Mclk3Sel {
        Sai2Mclk3Sel::from_bits(val)
    }
}
impl From<Sai2Mclk3Sel> for u8 {
    #[inline(always)]
    fn from(val: Sai2Mclk3Sel) -> u8 {
        Sai2Mclk3Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai2MclkDir {
    #[doc = "SAI2_MCLK is input signal"]
    INPUT = 0x0,
    #[doc = "SAI2_MCLK is output signal"]
    OUTPUT = 0x01,
}
impl Sai2MclkDir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai2MclkDir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai2MclkDir {
    #[inline(always)]
    fn from(val: u8) -> Sai2MclkDir {
        Sai2MclkDir::from_bits(val)
    }
}
impl From<Sai2MclkDir> for u8 {
    #[inline(always)]
    fn from(val: Sai2MclkDir) -> u8 {
        Sai2MclkDir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai3Mclk3Sel {
    #[doc = "SPDIF_CLK_ROOT"]
    CLK_ROOT = 0x0,
    #[doc = "spdif_tx_clk2"]
    TX_CLK2 = 0x01,
    #[doc = "spdif_srclk"]
    SRCLK = 0x02,
    #[doc = "spdif_outclock"]
    OUTCLK = 0x03,
}
impl Sai3Mclk3Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai3Mclk3Sel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai3Mclk3Sel {
    #[inline(always)]
    fn from(val: u8) -> Sai3Mclk3Sel {
        Sai3Mclk3Sel::from_bits(val)
    }
}
impl From<Sai3Mclk3Sel> for u8 {
    #[inline(always)]
    fn from(val: Sai3Mclk3Sel) -> u8 {
        Sai3Mclk3Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai3MclkDir {
    #[doc = "SAI3_MCLK is input signal"]
    MCLK_INPUT = 0x0,
    #[doc = "SAI3_MCLK is output signal"]
    MCLK_OUTPUT = 0x01,
}
impl Sai3MclkDir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai3MclkDir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai3MclkDir {
    #[inline(always)]
    fn from(val: u8) -> Sai3MclkDir {
        Sai3MclkDir::from_bits(val)
    }
}
impl From<Sai3MclkDir> for u8 {
    #[inline(always)]
    fn from(val: Sai3MclkDir) -> u8 {
        Sai3MclkDir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai4Mclk1Sel {
    #[doc = "SAI4_CLK_ROOT"]
    SAI4_CLK_ROOT = 0x0,
    #[doc = "SAI2_CLK_ROOT"]
    SAI2_CLK_ROOT = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "SAI3_CLK_ROOT"]
    SAI3_CLK_ROOT = 0x03,
    #[doc = "SAI4 MCLK IO pin"]
    SAI4_MCLK_IO = 0x04,
    #[doc = "SAI2 MCLK IO pin"]
    SAI2_MCLK_IO = 0x05,
    #[doc = "SAI3 MCLK IO pin"]
    SAI3_MCLK_IO = 0x06,
    _RESERVED_7 = 0x07,
}
impl Sai4Mclk1Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai4Mclk1Sel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai4Mclk1Sel {
    #[inline(always)]
    fn from(val: u8) -> Sai4Mclk1Sel {
        Sai4Mclk1Sel::from_bits(val)
    }
}
impl From<Sai4Mclk1Sel> for u8 {
    #[inline(always)]
    fn from(val: Sai4Mclk1Sel) -> u8 {
        Sai4Mclk1Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai4Mclk2Sel {
    #[doc = "SAI4_CLK_ROOT"]
    SAI4_CLK_ROOT = 0x0,
    #[doc = "SAI2_CLK_ROOT"]
    SAI2_CLK_ROOT = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "SAI3_CLK_ROOT"]
    SAI3_CLK_ROOT = 0x03,
    #[doc = "SAI4 MCLK IO pin"]
    SAI4_MCLK_IO = 0x04,
    #[doc = "SAI2 MCLK IO pin"]
    SAI2_MCLK_IO = 0x05,
    #[doc = "SAI3 MCLK IO pin"]
    SAI3_MCLK_IO = 0x06,
    _RESERVED_7 = 0x07,
}
impl Sai4Mclk2Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai4Mclk2Sel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai4Mclk2Sel {
    #[inline(always)]
    fn from(val: u8) -> Sai4Mclk2Sel {
        Sai4Mclk2Sel::from_bits(val)
    }
}
impl From<Sai4Mclk2Sel> for u8 {
    #[inline(always)]
    fn from(val: Sai4Mclk2Sel) -> u8 {
        Sai4Mclk2Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai4Mclk3Sel {
    #[doc = "SPDIF_CLK_ROOT"]
    CLK_ROOT = 0x0,
    #[doc = "spdif_tx_clk2"]
    TX_CLK2 = 0x01,
    #[doc = "spdif_srclk"]
    SRCLK = 0x02,
    #[doc = "spdif_outclock"]
    OUTCLK = 0x03,
}
impl Sai4Mclk3Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai4Mclk3Sel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai4Mclk3Sel {
    #[inline(always)]
    fn from(val: u8) -> Sai4Mclk3Sel {
        Sai4Mclk3Sel::from_bits(val)
    }
}
impl From<Sai4Mclk3Sel> for u8 {
    #[inline(always)]
    fn from(val: Sai4Mclk3Sel) -> u8 {
        Sai4Mclk3Sel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SyncEnable(pub u8);
impl SyncEnable {
    #[doc = "Channel is disabled"]
    pub const DISABLE: Self = Self(0x0);
    #[doc = "Channel is enabled"]
    pub const ENABLE: Self = Self(0x01);
}
impl SyncEnable {
    pub const fn from_bits(val: u8) -> SyncEnable {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for SyncEnable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("DISABLE"),
            0x01 => f.write_str("ENABLE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SyncEnable {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "DISABLE"),
            0x01 => defmt::write!(f, "ENABLE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for SyncEnable {
    #[inline(always)]
    fn from(val: u8) -> SyncEnable {
        SyncEnable::from_bits(val)
    }
}
impl From<SyncEnable> for u8 {
    #[inline(always)]
    fn from(val: SyncEnable) -> u8 {
        SyncEnable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TmrExtClkSel {
    #[doc = "CCM tmr_1588_clk_root is selected"]
    CCM_TIMER = 0x0,
    #[doc = "External pin is selected"]
    EXT_PIN = 0x01,
}
impl TmrExtClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TmrExtClkSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TmrExtClkSel {
    #[inline(always)]
    fn from(val: u8) -> TmrExtClkSel {
        TmrExtClkSel::from_bits(val)
    }
}
impl From<TmrExtClkSel> for u8 {
    #[inline(always)]
    fn from(val: TmrExtClkSel) -> u8 {
        TmrExtClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TmrTrig1Sel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl TmrTrig1Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TmrTrig1Sel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TmrTrig1Sel {
    #[inline(always)]
    fn from(val: u8) -> TmrTrig1Sel {
        TmrTrig1Sel::from_bits(val)
    }
}
impl From<TmrTrig1Sel> for u8 {
    #[inline(always)]
    fn from(val: TmrTrig1Sel) -> u8 {
        TmrTrig1Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TmrTrig2Sel {
    #[doc = "Input from IOMUX"]
    IOMUX = 0x0,
    #[doc = "Input from XBAR"]
    XBAR = 0x01,
}
impl TmrTrig2Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TmrTrig2Sel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TmrTrig2Sel {
    #[inline(always)]
    fn from(val: u8) -> TmrTrig2Sel {
        TmrTrig2Sel::from_bits(val)
    }
}
impl From<TmrTrig2Sel> for u8 {
    #[inline(always)]
    fn from(val: TmrTrig2Sel) -> u8 {
        TmrTrig2Sel::to_bits(val)
    }
}
