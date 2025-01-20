#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dbge {
    #[doc = "FLEXIO is disabled in Debug modes."]
    Disable = 0x0,
    #[doc = "FLEXIO is enabled in Debug modes."]
    Emable = 0x01,
}
impl Dbge {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dbge {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dbge {
    #[inline(always)]
    fn from(val: u8) -> Dbge {
        Dbge::from_bits(val)
    }
}
impl From<Dbge> for u8 {
    #[inline(always)]
    fn from(val: Dbge) -> u8 {
        Dbge::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dozen {
    #[doc = "FLEXIO enabled in Doze modes."]
    Enable = 0x0,
    #[doc = "FLEXIO disabled in Doze modes."]
    Disable = 0x01,
}
impl Dozen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dozen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dozen {
    #[inline(always)]
    fn from(val: u8) -> Dozen {
        Dozen::from_bits(val)
    }
}
impl From<Dozen> for u8 {
    #[inline(always)]
    fn from(val: Dozen) -> u8 {
        Dozen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fastacc {
    #[doc = "Configures for normal register accesses to FLEXIO"]
    Normal = 0x0,
    #[doc = "Configures for fast register accesses to FLEXIO"]
    Fast = 0x01,
}
impl Fastacc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fastacc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fastacc {
    #[inline(always)]
    fn from(val: u8) -> Fastacc {
        Fastacc::from_bits(val)
    }
}
impl From<Fastacc> for u8 {
    #[inline(always)]
    fn from(val: Fastacc) -> u8 {
        Fastacc::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Feature(pub u16);
impl Feature {
    #[doc = "Standard features implemented."]
    pub const STANDARD: Self = Self(0x0);
    #[doc = "Supports state, logic, and parallel modes."]
    pub const STATE_LOGIC_PARALLEL: Self = Self(0x01);
    #[doc = "Supports pin control registers."]
    pub const PINCTRL: Self = Self(0x02);
    #[doc = "Supports state, logic, and parallel modes, plus pin control registers."]
    pub const STATE_LOGIC_PARALLEL_PINCTRL: Self = Self(0x03);
}
impl Feature {
    pub const fn from_bits(val: u16) -> Feature {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Feature {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("STANDARD"),
            0x01 => f.write_str("STATE_LOGIC_PARALLEL"),
            0x02 => f.write_str("PINCTRL"),
            0x03 => f.write_str("STATE_LOGIC_PARALLEL_PINCTRL"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Feature {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "STANDARD"),
            0x01 => defmt::write!(f, "STATE_LOGIC_PARALLEL"),
            0x02 => defmt::write!(f, "PINCTRL"),
            0x03 => defmt::write!(f, "STATE_LOGIC_PARALLEL_PINCTRL"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Feature {
    #[inline(always)]
    fn from(val: u16) -> Feature {
        Feature::from_bits(val)
    }
}
impl From<Feature> for u16 {
    #[inline(always)]
    fn from(val: Feature) -> u16 {
        Feature::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Insrc {
    #[doc = "Pin"]
    Pin = 0x0,
    #[doc = "Shifter N+1 Output"]
    ShifterNplus1 = 0x01,
}
impl Insrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Insrc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Insrc {
    #[inline(always)]
    fn from(val: u8) -> Insrc {
        Insrc::from_bits(val)
    }
}
impl From<Insrc> for u8 {
    #[inline(always)]
    fn from(val: Insrc) -> u8 {
        Insrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Latst {
    #[doc = "Shift register stores the pre-shift register state."]
    Preshift = 0x0,
    #[doc = "Shift register stores the post-shift register state."]
    Postshift = 0x01,
}
impl Latst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Latst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Latst {
    #[inline(always)]
    fn from(val: u8) -> Latst {
        Latst::from_bits(val)
    }
}
impl From<Latst> for u8 {
    #[inline(always)]
    fn from(val: Latst) -> u8 {
        Latst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pinins {
    #[doc = "Timer pin input and output are selected by PINSEL."]
    Pinsel = 0x0,
    #[doc = "Timer pin input is selected by PINSEL+1. Timer pin output remains selected by PINSEL."]
    Pinselplus1 = 0x01,
}
impl Pinins {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pinins {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pinins {
    #[inline(always)]
    fn from(val: u8) -> Pinins {
        Pinins::from_bits(val)
    }
}
impl From<Pinins> for u8 {
    #[inline(always)]
    fn from(val: Pinins) -> u8 {
        Pinins::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ShiftctlPincfg {
    #[doc = "Shifter pin output disabled"]
    Disable = 0x0,
    #[doc = "Shifter pin open-drain or bidirectional output enable"]
    OpendBidirouten = 0x01,
    #[doc = "Shifter pin bidirectional output data"]
    BidirOutdata = 0x02,
    #[doc = "Shifter pin output"]
    Output = 0x03,
}
impl ShiftctlPincfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ShiftctlPincfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ShiftctlPincfg {
    #[inline(always)]
    fn from(val: u8) -> ShiftctlPincfg {
        ShiftctlPincfg::from_bits(val)
    }
}
impl From<ShiftctlPincfg> for u8 {
    #[inline(always)]
    fn from(val: ShiftctlPincfg) -> u8 {
        ShiftctlPincfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ShiftctlPinpol {
    #[doc = "Pin is active high"]
    ActiveHigh = 0x0,
    #[doc = "Pin is active low"]
    ActiveLow = 0x01,
}
impl ShiftctlPinpol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ShiftctlPinpol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ShiftctlPinpol {
    #[inline(always)]
    fn from(val: u8) -> ShiftctlPinpol {
        ShiftctlPinpol::from_bits(val)
    }
}
impl From<ShiftctlPinpol> for u8 {
    #[inline(always)]
    fn from(val: ShiftctlPinpol) -> u8 {
        ShiftctlPinpol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Smod {
    #[doc = "Disabled."]
    Disable = 0x0,
    #[doc = "Receive mode. Captures the current shifter content into the SHIFTBUF on expiration of the timer."]
    Receive = 0x01,
    #[doc = "Transmit mode. Load SHIFTBUF contents into the shifter on expiration of the timer."]
    Transmit = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Match Store mode. Shifter data is compared to SHIFTBUF content on expiration of the timer."]
    Matchstore = 0x04,
    #[doc = "Match Continuous mode. Shifter data is continuously compared to SHIFTBUF contents."]
    Matchcont = 0x05,
    #[doc = "State mode. SHIFTBUF contents are used for storing programmable state attributes."]
    State = 0x06,
    #[doc = "Logic mode. SHIFTBUF contents are used for implementing programmable logic lookup table."]
    Logic = 0x07,
}
impl Smod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Smod {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Smod {
    #[inline(always)]
    fn from(val: u8) -> Smod {
        Smod::from_bits(val)
    }
}
impl From<Smod> for u8 {
    #[inline(always)]
    fn from(val: Smod) -> u8 {
        Smod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ssize {
    #[doc = "Shift register is 32-bit."]
    Width32 = 0x0,
    #[doc = "Shift register is 24-bit."]
    Width24 = 0x01,
}
impl Ssize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ssize {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ssize {
    #[inline(always)]
    fn from(val: u8) -> Ssize {
        Ssize::from_bits(val)
    }
}
impl From<Ssize> for u8 {
    #[inline(always)]
    fn from(val: Ssize) -> u8 {
        Ssize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sstart {
    #[doc = "Start bit disabled for transmitter/receiver/match store. Transmitter loads data on enable."]
    Value00 = 0x0,
    #[doc = "Start bit disabled for transmitter/receiver/match store. Transmitter loads data on first shift."]
    Value01 = 0x01,
    #[doc = "Transmitter outputs start bit value 0 before loading data on first shift. If start bit is not 0, receiver/match store sets error flag."]
    Value10 = 0x02,
    #[doc = "Transmitter outputs start bit value 1 before loading data on first shift. If start bit is not 1, receiver/match store sets error flag."]
    Value11 = 0x03,
}
impl Sstart {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sstart {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sstart {
    #[inline(always)]
    fn from(val: u8) -> Sstart {
        Sstart::from_bits(val)
    }
}
impl From<Sstart> for u8 {
    #[inline(always)]
    fn from(val: Sstart) -> u8 {
        Sstart::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sstop {
    #[doc = "Stop bit disabled for transmitter/receiver/match store"]
    Value00 = 0x0,
    #[doc = "Stop bit disabled for transmitter/receiver/match store. When timer is in stop condition, receiver/match store stores receive data on the configured shift edge."]
    Value01 = 0x01,
    #[doc = "Transmitter outputs stop bit value 0 on store. If stop bit is not 0, receiver/match store sets error flag. When timer is in stop condition, receiver/match stores also store receive data on the configured shift edge."]
    Value10 = 0x02,
    #[doc = "Transmitter outputs stop bit value 1 on store. If stop bit is not 1, receiver/match store sets error flag. When timer is in stop condition, receiver/match store also stores receive data on the configured shift edge."]
    Value11 = 0x03,
}
impl Sstop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sstop {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sstop {
    #[inline(always)]
    fn from(val: u8) -> Sstop {
        Sstop::from_bits(val)
    }
}
impl From<Sstop> for u8 {
    #[inline(always)]
    fn from(val: Sstop) -> u8 {
        Sstop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TimctlPincfg {
    #[doc = "Timer pin output disabled"]
    Outdisable = 0x0,
    #[doc = "Timer pin open-drain or bidirectional output enable"]
    OpendBidirouten = 0x01,
    #[doc = "Timer pin bidirectional output data"]
    BidirOutdata = 0x02,
    #[doc = "Timer pin output"]
    Output = 0x03,
}
impl TimctlPincfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TimctlPincfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TimctlPincfg {
    #[inline(always)]
    fn from(val: u8) -> TimctlPincfg {
        TimctlPincfg::from_bits(val)
    }
}
impl From<TimctlPincfg> for u8 {
    #[inline(always)]
    fn from(val: TimctlPincfg) -> u8 {
        TimctlPincfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TimctlPinpol {
    #[doc = "Pin is active high"]
    ActiveHigh = 0x0,
    #[doc = "Pin is active low"]
    ActiveLow = 0x01,
}
impl TimctlPinpol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TimctlPinpol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TimctlPinpol {
    #[inline(always)]
    fn from(val: u8) -> TimctlPinpol {
        TimctlPinpol::from_bits(val)
    }
}
impl From<TimctlPinpol> for u8 {
    #[inline(always)]
    fn from(val: TimctlPinpol) -> u8 {
        TimctlPinpol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timdec {
    #[doc = "Decrement counter on FLEXIO clock. Shift clock equals timer output."]
    FlexioClkShiftclkTmrOut = 0x0,
    #[doc = "Decrement counter on trigger input (both edges). Shift clock equals timer output."]
    TrigEdgeShiftclkTmrOut = 0x01,
    #[doc = "Decrement counter on pin input (both edges). Shift clock equals pin input."]
    PinEdgeShiftclkTmrOut = 0x02,
    #[doc = "Decrement counter on trigger input (both edges). Shift clock equals trigger input."]
    TrigEdgeShiftclkTrigIn = 0x03,
    #[doc = "Decrement counter on FLEXIO clock divided by 16. Shift clock equals timer output."]
    FlexioClkDiv16ShiftclkTmrOut = 0x04,
    #[doc = "Decrement counter on FLEXIO clock divided by 256. Shift clock equals timer output."]
    FlexioClkDiv256ShiftclkTmrOut = 0x05,
    #[doc = "Decrement counter on pin input (rising edge). Shift clock equals pin input."]
    PinRiseShiftclkPinIn = 0x06,
    #[doc = "Decrement counter on trigger input (rising edge). Shift clock equals trigger input."]
    TrigRiseShiftclkTrigIn = 0x07,
}
impl Timdec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timdec {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timdec {
    #[inline(always)]
    fn from(val: u8) -> Timdec {
        Timdec::from_bits(val)
    }
}
impl From<Timdec> for u8 {
    #[inline(always)]
    fn from(val: Timdec) -> u8 {
        Timdec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timdis {
    #[doc = "Timer never disabled"]
    Never = 0x0,
    #[doc = "Timer disabled on Timer N-1 disable"]
    TmrNminus1 = 0x01,
    #[doc = "Timer disabled on timer compare (upper 8-bits match and decrement)"]
    TmrCmp = 0x02,
    #[doc = "Timer disabled on timer compare (upper 8-bits match and decrement) and trigger low"]
    TmrCmpTriglow = 0x03,
    #[doc = "Timer disabled on pin rising or falling edge"]
    PinEdge = 0x04,
    #[doc = "Timer disabled on pin rising or falling edge provided trigger is high"]
    PinEdgeTrighi = 0x05,
    #[doc = "Timer disabled on trigger falling edge"]
    TrigFalledge = 0x06,
    _RESERVED_7 = 0x07,
}
impl Timdis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timdis {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timdis {
    #[inline(always)]
    fn from(val: u8) -> Timdis {
        Timdis::from_bits(val)
    }
}
impl From<Timdis> for u8 {
    #[inline(always)]
    fn from(val: Timdis) -> u8 {
        Timdis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timena {
    #[doc = "Timer always enabled"]
    Enable = 0x0,
    #[doc = "Timer enabled on Timer N-1 enable"]
    TmrNminus1En = 0x01,
    #[doc = "Timer enabled on Trigger high"]
    TmrTrighiEn = 0x02,
    #[doc = "Timer enabled on Trigger high and Pin high"]
    TmrTrigPinHiEn = 0x03,
    #[doc = "Timer enabled on Pin rising edge"]
    TmrPinriseEn = 0x04,
    #[doc = "Timer enabled on Pin rising edge and Trigger high"]
    TmrPinriseTrighiEn = 0x05,
    #[doc = "Timer enabled on Trigger rising edge"]
    TmrTrigriseEn = 0x06,
    #[doc = "Timer enabled on Trigger rising or falling edge"]
    TmrTrigedgeEn = 0x07,
}
impl Timena {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timena {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timena {
    #[inline(always)]
    fn from(val: u8) -> Timena {
        Timena::from_bits(val)
    }
}
impl From<Timena> for u8 {
    #[inline(always)]
    fn from(val: Timena) -> u8 {
        Timena::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timod {
    #[doc = "Timer disabled."]
    Disable = 0x0,
    #[doc = "Dual 8-bit counters baud mode."]
    Dual8bitBaud = 0x01,
    #[doc = "Dual 8-bit counters PWM high mode."]
    Dual8bitPwmH = 0x02,
    #[doc = "Single 16-bit counter mode."]
    Single16bit = 0x03,
    #[doc = "Single 16-bit counter disable mode."]
    Single16bitDisable = 0x04,
    #[doc = "Dual 8-bit counters word mode."]
    Dual8bitWord = 0x05,
    #[doc = "Dual 8-bit counters PWM low mode."]
    Dual8bitPwmL = 0x06,
    #[doc = "Single 16-bit input capture mode."]
    Single16bitInCapture = 0x07,
}
impl Timod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timod {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timod {
    #[inline(always)]
    fn from(val: u8) -> Timod {
        Timod::from_bits(val)
    }
}
impl From<Timod> for u8 {
    #[inline(always)]
    fn from(val: Timod) -> u8 {
        Timod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timout {
    #[doc = "Timer output is logic one when enabled and is not affected by timer reset"]
    One = 0x0,
    #[doc = "Timer output is logic zero when enabled and is not affected by timer reset"]
    Zero = 0x01,
    #[doc = "Timer output is logic one when enabled and on timer reset"]
    OneTmrreset = 0x02,
    #[doc = "Timer output is logic zero when enabled and on timer reset"]
    ZeroTmrreset = 0x03,
}
impl Timout {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timout {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timout {
    #[inline(always)]
    fn from(val: u8) -> Timout {
        Timout::from_bits(val)
    }
}
impl From<Timout> for u8 {
    #[inline(always)]
    fn from(val: Timout) -> u8 {
        Timout::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timpol {
    #[doc = "Shift on posedge of shift clock"]
    Posedge = 0x0,
    #[doc = "Shift on negedge of shift clock"]
    Negedge = 0x01,
}
impl Timpol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timpol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timpol {
    #[inline(always)]
    fn from(val: u8) -> Timpol {
        Timpol::from_bits(val)
    }
}
impl From<Timpol> for u8 {
    #[inline(always)]
    fn from(val: Timpol) -> u8 {
        Timpol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timrst {
    #[doc = "Timer never reset"]
    Never = 0x0,
    #[doc = "Timer reset on timer output high."]
    TmrOutHi = 0x01,
    #[doc = "Timer reset on timer pin equal to timer output"]
    PinEqTmrOut = 0x02,
    #[doc = "Timer reset on timer trigger equal to timer output"]
    TrigEqTmrOut = 0x03,
    #[doc = "Timer reset on timer pin rising edge"]
    PinRiseEdge = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "Timer reset on trigger rising edge"]
    TrigRiseEdge = 0x06,
    #[doc = "Timer reset on trigger rising or falling edge"]
    TrigEdge = 0x07,
}
impl Timrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timrst {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timrst {
    #[inline(always)]
    fn from(val: u8) -> Timrst {
        Timrst::from_bits(val)
    }
}
impl From<Timrst> for u8 {
    #[inline(always)]
    fn from(val: Timrst) -> u8 {
        Timrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trgpol {
    #[doc = "Trigger active high"]
    ActiveHigh = 0x0,
    #[doc = "Trigger active low"]
    ActiveLow = 0x01,
}
impl Trgpol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trgpol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trgpol {
    #[inline(always)]
    fn from(val: u8) -> Trgpol {
        Trgpol::from_bits(val)
    }
}
impl From<Trgpol> for u8 {
    #[inline(always)]
    fn from(val: Trgpol) -> u8 {
        Trgpol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trgsrc {
    #[doc = "External trigger selected"]
    ExtTrig = 0x0,
    #[doc = "Internal trigger selected"]
    InternalTrig = 0x01,
}
impl Trgsrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trgsrc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trgsrc {
    #[inline(always)]
    fn from(val: u8) -> Trgsrc {
        Trgsrc::from_bits(val)
    }
}
impl From<Trgsrc> for u8 {
    #[inline(always)]
    fn from(val: Trgsrc) -> u8 {
        Trgsrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tstop {
    #[doc = "Stop bit disabled"]
    StopDisable = 0x0,
    #[doc = "Stop bit is enabled on timer compare"]
    EnableTmrcmp = 0x01,
    #[doc = "Stop bit is enabled on timer disable"]
    EnableTmrdisable = 0x02,
    #[doc = "Stop bit is enabled on timer compare and timer disable"]
    EnableTmrCmpDis = 0x03,
}
impl Tstop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tstop {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tstop {
    #[inline(always)]
    fn from(val: u8) -> Tstop {
        Tstop::from_bits(val)
    }
}
impl From<Tstop> for u8 {
    #[inline(always)]
    fn from(val: Tstop) -> u8 {
        Tstop::to_bits(val)
    }
}
