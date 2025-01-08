#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Acph1tc {
    #[doc = "Phase 1 active time in one sampling period equals to T"]
    VALUE_T = 0x0,
    #[doc = "Phase 1 active time in one sampling period equals to 2 * T"]
    VALUE_2T = 0x01,
    #[doc = "Phase 1 active time in one sampling period equals to 4 * T"]
    VALUE_4T = 0x02,
    #[doc = "Phase 1 active time in one sampling period equals to 8 * T"]
    VALUE_8T = 0x03,
    #[doc = "Phase 1 active time in one sampling period equals to T"]
    VALUE4_T = 0x04,
    #[doc = "Phase 1 active time in one sampling period equals to T"]
    VALUE5_T = 0x05,
    #[doc = "Phase 1 active time in one sampling period equals to T"]
    VALUE6_T = 0x06,
    #[doc = "Phase 1 active time in one sampling period equals to 0"]
    VALUE_0 = 0x07,
}
impl Acph1tc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Acph1tc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Acph1tc {
    #[inline(always)]
    fn from(val: u8) -> Acph1tc {
        Acph1tc::from_bits(val)
    }
}
impl From<Acph1tc> for u8 {
    #[inline(always)]
    fn from(val: Acph1tc) -> u8 {
        Acph1tc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Acph2tc {
    #[doc = "Phase 2 active time in one sampling period equals to T"]
    VALUE_T = 0x0,
    #[doc = "Phase 2 active time in one sampling period equals to 2 * T"]
    VALUE_2T = 0x01,
    #[doc = "Phase 2 active time in one sampling period equals to 4 * T"]
    VALUE_4T = 0x02,
    #[doc = "Phase 2 active time in one sampling period equals to 8 * T"]
    VALUE_8T = 0x03,
    #[doc = "Phase 2 active time in one sampling period equals to 16 * T"]
    VALUE_16T = 0x04,
    #[doc = "Phase 2 active time in one sampling period equals to 32 * T"]
    VALUE_32T = 0x05,
    #[doc = "Phase 2 active time in one sampling period equals to 64 * T"]
    VALUE_64T = 0x06,
    #[doc = "Phase 2 active time in one sampling period equals to 16 * T"]
    VALUE7_16T = 0x07,
}
impl Acph2tc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Acph2tc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Acph2tc {
    #[inline(always)]
    fn from(val: u8) -> Acph2tc {
        Acph2tc::from_bits(val)
    }
}
impl From<Acph2tc> for u8 {
    #[inline(always)]
    fn from(val: Acph2tc) -> u8 {
        Acph2tc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Acsat {
    #[doc = "The sampling time equals to T"]
    VALUE_T = 0x0,
    #[doc = "The sampling time equals to 2 * T"]
    VALUE_2T = 0x01,
    #[doc = "The sampling time equals to 4 * T"]
    VALUE_4T = 0x02,
    #[doc = "The sampling time equals to 8 * T"]
    VALUE_8T = 0x03,
    #[doc = "The sampling time equals to 16 * T"]
    VALUE_16T = 0x04,
    #[doc = "The sampling time equals to 32 * T"]
    VALUE_32T = 0x05,
    #[doc = "The sampling time equals to 64 * T"]
    VALUE_64T = 0x06,
    #[doc = "The sampling time equals to 256 * T"]
    VALUE_256T = 0x07,
}
impl Acsat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Acsat {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Acsat {
    #[inline(always)]
    fn from(val: u8) -> Acsat {
        Acsat::from_bits(val)
    }
}
impl From<Acsat> for u8 {
    #[inline(always)]
    fn from(val: Acsat) -> u8 {
        Acsat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Chn0 {
    #[doc = "Disable"]
    CHN0_DISABLED = 0x0,
    #[doc = "Enable"]
    CHN0_ENABLED = 0x01,
}
impl Chn0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Chn0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Chn0 {
    #[inline(always)]
    fn from(val: u8) -> Chn0 {
        Chn0::from_bits(val)
    }
}
impl From<Chn0> for u8 {
    #[inline(always)]
    fn from(val: Chn0) -> u8 {
        Chn0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Chn1 {
    #[doc = "Disable"]
    CHN1_DISABLED = 0x0,
    #[doc = "Enable"]
    CHN1_ENABLED = 0x01,
}
impl Chn1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Chn1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Chn1 {
    #[inline(always)]
    fn from(val: u8) -> Chn1 {
        Chn1::from_bits(val)
    }
}
impl From<Chn1> for u8 {
    #[inline(always)]
    fn from(val: Chn1) -> u8 {
        Chn1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Chn2 {
    #[doc = "Disable"]
    CHN2_DISABLED = 0x0,
    #[doc = "Enable"]
    CHN2_ENABLED = 0x01,
}
impl Chn2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Chn2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Chn2 {
    #[inline(always)]
    fn from(val: u8) -> Chn2 {
        Chn2::from_bits(val)
    }
}
impl From<Chn2> for u8 {
    #[inline(always)]
    fn from(val: Chn2) -> u8 {
        Chn2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Chn3 {
    #[doc = "Disable"]
    CHN3_DISABLED = 0x0,
    #[doc = "Enable"]
    CHN3_ENABLED = 0x01,
}
impl Chn3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Chn3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Chn3 {
    #[inline(always)]
    fn from(val: u8) -> Chn3 {
        Chn3::from_bits(val)
    }
}
impl From<Chn3> for u8 {
    #[inline(always)]
    fn from(val: Chn3) -> u8 {
        Chn3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Chn4 {
    #[doc = "Disable"]
    CHN4_DISABLED = 0x0,
    #[doc = "Enable"]
    CHN4_ENABLED = 0x01,
}
impl Chn4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Chn4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Chn4 {
    #[inline(always)]
    fn from(val: u8) -> Chn4 {
        Chn4::from_bits(val)
    }
}
impl From<Chn4> for u8 {
    #[inline(always)]
    fn from(val: Chn4) -> u8 {
        Chn4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Chn5 {
    #[doc = "Disable"]
    CHN5_DISABLED = 0x0,
    #[doc = "Enable"]
    CHN5_ENABLED = 0x01,
}
impl Chn5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Chn5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Chn5 {
    #[inline(always)]
    fn from(val: u8) -> Chn5 {
        Chn5::from_bits(val)
    }
}
impl From<Chn5> for u8 {
    #[inline(always)]
    fn from(val: Chn5) -> u8 {
        Chn5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cos {
    #[doc = "COUT"]
    FILTERED = 0x0,
    #[doc = "COUTA"]
    UNFILTERED = 0x01,
}
impl Cos {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cos {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cos {
    #[inline(always)]
    fn from(val: u8) -> Cos {
        Cos::from_bits(val)
    }
}
impl From<Cos> for u8 {
    #[inline(always)]
    fn from(val: Cos) -> u8 {
        Cos::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmaen {
    #[doc = "Disable"]
    INT_DISABLED = 0x0,
    #[doc = "Enable"]
    INT_ENABLED = 0x01,
}
impl Dmaen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmaen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmaen {
    #[inline(always)]
    fn from(val: u8) -> Dmaen {
        Dmaen::from_bits(val)
    }
}
impl From<Dmaen> for u8 {
    #[inline(always)]
    fn from(val: Dmaen) -> u8 {
        Dmaen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmcs {
    #[doc = "Slow clock is selected for the timing generation."]
    SLOW_CLOCK = 0x0,
    #[doc = "Fast clock is selected for the timing generation."]
    FAST_CLOCK = 0x01,
}
impl Dmcs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmcs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmcs {
    #[inline(always)]
    fn from(val: u8) -> Dmcs {
        Dmcs::from_bits(val)
    }
}
impl From<Dmcs> for u8 {
    #[inline(always)]
    fn from(val: Dmcs) -> u8 {
        Dmcs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmode {
    #[doc = "Low-Speed and Low-Power mode"]
    LOW_SPEED_LOW_POWER = 0x0,
    #[doc = "High-Speed and High-Power mode"]
    HIGH_SPEED_HIGH_POWER = 0x01,
}
impl Dmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmode {
    #[inline(always)]
    fn from(val: u8) -> Dmode {
        Dmode::from_bits(val)
    }
}
impl From<Dmode> for u8 {
    #[inline(always)]
    fn from(val: Dmode) -> u8 {
        Dmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum En {
    #[doc = "Disable"]
    COMPARATOR_DISABLED = 0x0,
    #[doc = "Enable"]
    COMPARATOR_ENABLED = 0x01,
}
impl En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for En {
    #[inline(always)]
    fn from(val: u8) -> En {
        En::from_bits(val)
    }
}
impl From<En> for u8 {
    #[inline(always)]
    fn from(val: En) -> u8 {
        En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FilterCnt {
    #[doc = "Filter is disabled (if C0\\[SE\\] = 1, then COUT is a logic zero (this is not a legal state, and is not recommended); if C0\\[SE\\] = 0, COUT = COUTA)"]
    FILTER_DISABLED = 0x0,
    #[doc = "One consecutive sample (comparator output is simply sampled)"]
    SAMPLE_1 = 0x01,
    #[doc = "Two consecutive samples"]
    SAMPLE_2 = 0x02,
    #[doc = "Three consecutive samples"]
    SAMPLE_3 = 0x03,
    #[doc = "Four consecutive samples"]
    SAMPLE_4 = 0x04,
    #[doc = "Five consecutive samples"]
    SAMPLE_5 = 0x05,
    #[doc = "Six consecutive samples"]
    SAMPLE_6 = 0x06,
    #[doc = "Seven consecutive samples"]
    SAMPLE_7 = 0x07,
}
impl FilterCnt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FilterCnt {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FilterCnt {
    #[inline(always)]
    fn from(val: u8) -> FilterCnt {
        FilterCnt::from_bits(val)
    }
}
impl From<FilterCnt> for u8 {
    #[inline(always)]
    fn from(val: FilterCnt) -> u8 {
        FilterCnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fxmp {
    #[doc = "Fix plus port"]
    FIXED_PLUS_PORT = 0x0,
    #[doc = "Fix minus port"]
    FIXED_MINUS_PORT = 0x01,
}
impl Fxmp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fxmp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fxmp {
    #[inline(always)]
    fn from(val: u8) -> Fxmp {
        Fxmp::from_bits(val)
    }
}
impl From<Fxmp> for u8 {
    #[inline(always)]
    fn from(val: Fxmp) -> u8 {
        Fxmp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fxmxch {
    #[doc = "External reference input 0"]
    INPUT_0 = 0x0,
    #[doc = "External reference input 1"]
    INPUT_1 = 0x01,
    #[doc = "External reference input 2"]
    INPUT_2 = 0x02,
    #[doc = "External reference input 3"]
    INPUT_3 = 0x03,
    #[doc = "External reference input 4"]
    INPUT_4 = 0x04,
    #[doc = "External reference input 5"]
    INPUT_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "8-bit DAC"]
    DAC = 0x07,
}
impl Fxmxch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fxmxch {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fxmxch {
    #[inline(always)]
    fn from(val: u8) -> Fxmxch {
        Fxmxch::from_bits(val)
    }
}
impl From<Fxmxch> for u8 {
    #[inline(always)]
    fn from(val: Fxmxch) -> u8 {
        Fxmxch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ief {
    #[doc = "Disable"]
    INT_DISABLED = 0x0,
    #[doc = "Enable"]
    INT_ENABLED = 0x01,
}
impl Ief {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ief {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ief {
    #[inline(always)]
    fn from(val: u8) -> Ief {
        Ief::from_bits(val)
    }
}
impl From<Ief> for u8 {
    #[inline(always)]
    fn from(val: Ief) -> u8 {
        Ief::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ier {
    #[doc = "Disable"]
    INT_DISABLED = 0x0,
    #[doc = "Enable"]
    INT_ENABLED = 0x01,
}
impl Ier {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ier {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ier {
    #[inline(always)]
    fn from(val: u8) -> Ier {
        Ier::from_bits(val)
    }
}
impl From<Ier> for u8 {
    #[inline(always)]
    fn from(val: Ier) -> u8 {
        Ier::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Linken {
    #[doc = "Disable"]
    INT_DISABLED = 0x0,
    #[doc = "Enable"]
    INT_ENABLED = 0x01,
}
impl Linken {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Linken {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Linken {
    #[inline(always)]
    fn from(val: u8) -> Linken {
        Linken::from_bits(val)
    }
}
impl From<Linken> for u8 {
    #[inline(always)]
    fn from(val: Linken) -> u8 {
        Linken::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Msel {
    #[doc = "Internal negative input 0 for minus channel (internal minus input)"]
    NEGATIVE_INPUT_0 = 0x0,
    #[doc = "External input 1 for minus channel (reference input 0)"]
    INPUT_1 = 0x01,
    #[doc = "External input 2 for minus channel (reference input 1)"]
    INPUT_2 = 0x02,
    #[doc = "External input 3 for minus channel (reference input 2)"]
    INPUT_3 = 0x03,
    #[doc = "External input 4 for minus channel (reference input 3)"]
    INPUT_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Internal 8-bit DAC output"]
    DAC = 0x07,
}
impl Msel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Msel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Msel {
    #[inline(always)]
    fn from(val: u8) -> Msel {
        Msel::from_bits(val)
    }
}
impl From<Msel> for u8 {
    #[inline(always)]
    fn from(val: Msel) -> u8 {
        Msel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nsam {
    #[doc = "As soon as the active channel is scanned in one round-robin clock"]
    VALUE_0 = 0x0,
    #[doc = "After one round-robin clock cycle"]
    VALUE_1 = 0x01,
    #[doc = "After two round-robin clock cycles"]
    VALUE_2 = 0x02,
    #[doc = "After three round-robin clock cycles"]
    VALUE_3 = 0x03,
}
impl Nsam {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nsam {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nsam {
    #[inline(always)]
    fn from(val: u8) -> Nsam {
        Nsam::from_bits(val)
    }
}
impl From<Nsam> for u8 {
    #[inline(always)]
    fn from(val: Nsam) -> u8 {
        Nsam::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ope {
    #[doc = "Disable"]
    OUTPUT_NOT_AVAILABLE = 0x0,
    #[doc = "Enable"]
    OUTPUT_AVAILABLE = 0x01,
}
impl Ope {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ope {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ope {
    #[inline(always)]
    fn from(val: u8) -> Ope {
        Ope::from_bits(val)
    }
}
impl From<Ope> for u8 {
    #[inline(always)]
    fn from(val: Ope) -> u8 {
        Ope::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pchcten {
    #[doc = "in Discrete Mode and special timing needs to be configured"]
    DISABLED = 0x0,
    #[doc = "in Continuous Mode and no special timing is required"]
    CONTINUOUS_MODE = 0x01,
}
impl Pchcten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pchcten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pchcten {
    #[inline(always)]
    fn from(val: u8) -> Pchcten {
        Pchcten::from_bits(val)
    }
}
impl From<Pchcten> for u8 {
    #[inline(always)]
    fn from(val: Pchcten) -> u8 {
        Pchcten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pmode {
    #[doc = "Low-speed (LS)"]
    LS = 0x0,
    #[doc = "High-speed (HS)"]
    HS = 0x01,
}
impl Pmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pmode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pmode {
    #[inline(always)]
    fn from(val: u8) -> Pmode {
        Pmode::from_bits(val)
    }
}
impl From<Pmode> for u8 {
    #[inline(always)]
    fn from(val: Pmode) -> u8 {
        Pmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Psel {
    #[doc = "Internal positive input 0 for plus channel (internal plus input)"]
    POSITIVE_INPUT_0 = 0x0,
    #[doc = "External input 1 for plus Channel (reference input 0)"]
    INPUT_1 = 0x01,
    #[doc = "External input 2 for plus channel (reference input 1)"]
    INPUT_2 = 0x02,
    #[doc = "External input 3 for plus channel (reference input 2)"]
    INPUT_3 = 0x03,
    #[doc = "External input 4 for plus channel (reference input 3)"]
    INPUT_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Internal 8-bit DAC output"]
    DAC = 0x07,
}
impl Psel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Psel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Psel {
    #[inline(always)]
    fn from(val: u8) -> Psel {
        Psel::from_bits(val)
    }
}
impl From<Psel> for u8 {
    #[inline(always)]
    fn from(val: Psel) -> u8 {
        Psel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vrsel {
    #[doc = "Vin1"]
    VIN1 = 0x0,
    #[doc = "Vin2"]
    VIN2 = 0x01,
}
impl Vrsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vrsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vrsel {
    #[inline(always)]
    fn from(val: u8) -> Vrsel {
        Vrsel::from_bits(val)
    }
}
impl From<Vrsel> for u8 {
    #[inline(always)]
    fn from(val: Vrsel) -> u8 {
        Vrsel::to_bits(val)
    }
}
