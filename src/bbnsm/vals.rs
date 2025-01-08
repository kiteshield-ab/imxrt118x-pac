#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BtnTimeout {
    #[doc = "5 seconds."]
    SEC_5 = 0x0,
    #[doc = "10 seconds."]
    SEC_10 = 0x01,
    #[doc = "15 seconds."]
    SEC_15 = 0x02,
    #[doc = "Timeout disabled. Long button presses will not request a power down."]
    DISABLE = 0x03,
}
impl BtnTimeout {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BtnTimeout {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BtnTimeout {
    #[inline(always)]
    fn from(val: u8) -> BtnTimeout {
        BtnTimeout::from_bits(val)
    }
}
impl From<BtnTimeout> for u8 {
    #[inline(always)]
    fn from(val: BtnTimeout) -> u8 {
        BtnTimeout::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CalVal {
    #[doc = "+0 counts per each 32768 ticks of the counter clock."]
    ADD_0_PER_32768_TICKS = 0x0,
    #[doc = "+1 counts per each 32768 ticks of the counter clock."]
    ADD_1_PER_32768_TICKS = 0x01,
    #[doc = "+2 counts per each 32768 ticks of the counter clock."]
    ADD_2_PER_32768_TICKS = 0x02,
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
    #[doc = "+15 counts per each 32768 ticks of the counter clock."]
    ADD_15_PER_32768_TICKS = 0x0f,
    #[doc = "-16 counts per each 32768 ticks of the counter clock."]
    SUB_16_PER_32768_TICKS = 0x10,
    #[doc = "-15 counts per each 32768 ticks of the counter clock."]
    SUB_15_PER_32768_TICKS = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "-2 counts per each 32768 ticks of the counter clock."]
    SUB_2_PER_32768_TICKS = 0x1e,
    #[doc = "-1 counts per each 32768 ticks of the counter clock."]
    SUB_1_PER_32768_TICKS = 0x1f,
}
impl CalVal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CalVal {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CalVal {
    #[inline(always)]
    fn from(val: u8) -> CalVal {
        CalVal::from_bits(val)
    }
}
impl From<CalVal> for u8 {
    #[inline(always)]
    fn from(val: CalVal) -> u8 {
        CalVal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Debounce {
    #[doc = "50 milliseconds."]
    MSEC_50 = 0x0,
    #[doc = "100 milliseconds."]
    MSEC_100 = 0x01,
    #[doc = "500 milliseconds."]
    MSEC_500 = 0x02,
    #[doc = "0 milliseconds."]
    MSEC_0 = 0x03,
}
impl Debounce {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Debounce {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Debounce {
    #[inline(always)]
    fn from(val: u8) -> Debounce {
        Debounce::from_bits(val)
    }
}
impl From<Debounce> for u8 {
    #[inline(always)]
    fn from(val: Debounce) -> u8 {
        Debounce::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EmgOff {
    #[doc = "An emergency power off has not been requested."]
    NO_EVENT = 0x0,
    #[doc = "An emergency power off has been requested."]
    EVENT_ASSERTED = 0x01,
}
impl EmgOff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EmgOff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EmgOff {
    #[inline(always)]
    fn from(val: u8) -> EmgOff {
        EmgOff::from_bits(val)
    }
}
impl From<EmgOff> for u8 {
    #[inline(always)]
    fn from(val: EmgOff) -> u8 {
        EmgOff::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct GprSz(pub u8);
impl GprSz {
    #[doc = "This version of BBNSM does not implement a general-purpose register array."]
    pub const NO_GPR: Self = Self(0x0);
}
impl GprSz {
    pub const fn from_bits(val: u8) -> GprSz {
        Self(val & 0x3f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for GprSz {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NO_GPR"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GprSz {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NO_GPR"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for GprSz {
    #[inline(always)]
    fn from(val: u8) -> GprSz {
        GprSz::from_bits(val)
    }
}
impl From<GprSz> for u8 {
    #[inline(always)]
    fn from(val: GprSz) -> u8 {
        GprSz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PadCtrl0 {
    #[doc = "Deasserts bit n in bbnsm_pad_ctrl\\[n\\]"]
    ZERO = 0x0,
    #[doc = "Assert bit n in bbnsm_pad_ctrl\\[n\\]"]
    ONE = 0x01,
}
impl PadCtrl0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PadCtrl0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PadCtrl0 {
    #[inline(always)]
    fn from(val: u8) -> PadCtrl0 {
        PadCtrl0::from_bits(val)
    }
}
impl From<PadCtrl0> for u8 {
    #[inline(always)]
    fn from(val: PadCtrl0) -> u8 {
        PadCtrl0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PadCtrl1 {
    #[doc = "Deasserts bit n in bbnsm_pad_ctrl\\[n\\]"]
    ZERO = 0x0,
    #[doc = "Assert bit n in bbnsm_pad_ctrl\\[n\\]"]
    ONE = 0x01,
}
impl PadCtrl1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PadCtrl1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PadCtrl1 {
    #[inline(always)]
    fn from(val: u8) -> PadCtrl1 {
        PadCtrl1::from_bits(val)
    }
}
impl From<PadCtrl1> for u8 {
    #[inline(always)]
    fn from(val: PadCtrl1) -> u8 {
        PadCtrl1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PadCtrl10 {
    #[doc = "Deasserts bit n in bbnsm_pad_ctrl\\[n\\]"]
    ZERO = 0x0,
    #[doc = "Assert bit n in bbnsm_pad_ctrl\\[n\\]"]
    ONE = 0x01,
}
impl PadCtrl10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PadCtrl10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PadCtrl10 {
    #[inline(always)]
    fn from(val: u8) -> PadCtrl10 {
        PadCtrl10::from_bits(val)
    }
}
impl From<PadCtrl10> for u8 {
    #[inline(always)]
    fn from(val: PadCtrl10) -> u8 {
        PadCtrl10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PadCtrl11 {
    #[doc = "Deasserts bit n in bbnsm_pad_ctrl\\[n\\]"]
    ZERO = 0x0,
    #[doc = "Assert bit n in bbnsm_pad_ctrl\\[n\\]"]
    ONE = 0x01,
}
impl PadCtrl11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PadCtrl11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PadCtrl11 {
    #[inline(always)]
    fn from(val: u8) -> PadCtrl11 {
        PadCtrl11::from_bits(val)
    }
}
impl From<PadCtrl11> for u8 {
    #[inline(always)]
    fn from(val: PadCtrl11) -> u8 {
        PadCtrl11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PadCtrl12 {
    #[doc = "Deasserts bit n in bbnsm_pad_ctrl\\[n\\]"]
    ZERO = 0x0,
    #[doc = "Assert bit n in bbnsm_pad_ctrl\\[n\\]"]
    ONE = 0x01,
}
impl PadCtrl12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PadCtrl12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PadCtrl12 {
    #[inline(always)]
    fn from(val: u8) -> PadCtrl12 {
        PadCtrl12::from_bits(val)
    }
}
impl From<PadCtrl12> for u8 {
    #[inline(always)]
    fn from(val: PadCtrl12) -> u8 {
        PadCtrl12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PadCtrl13 {
    #[doc = "Deasserts bit n in bbnsm_pad_ctrl\\[n\\]"]
    ZERO = 0x0,
    #[doc = "Assert bit n in bbnsm_pad_ctrl\\[n\\]"]
    ONE = 0x01,
}
impl PadCtrl13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PadCtrl13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PadCtrl13 {
    #[inline(always)]
    fn from(val: u8) -> PadCtrl13 {
        PadCtrl13::from_bits(val)
    }
}
impl From<PadCtrl13> for u8 {
    #[inline(always)]
    fn from(val: PadCtrl13) -> u8 {
        PadCtrl13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PadCtrl14 {
    #[doc = "Deasserts bit n in bbnsm_pad_ctrl\\[n\\]"]
    ZERO = 0x0,
    #[doc = "Assert bit n in bbnsm_pad_ctrl\\[n\\]"]
    ONE = 0x01,
}
impl PadCtrl14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PadCtrl14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PadCtrl14 {
    #[inline(always)]
    fn from(val: u8) -> PadCtrl14 {
        PadCtrl14::from_bits(val)
    }
}
impl From<PadCtrl14> for u8 {
    #[inline(always)]
    fn from(val: PadCtrl14) -> u8 {
        PadCtrl14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PadCtrl15 {
    #[doc = "Deasserts bit n in bbnsm_pad_ctrl\\[n\\]"]
    ZERO = 0x0,
    #[doc = "Assert bit n in bbnsm_pad_ctrl\\[n\\]"]
    ONE = 0x01,
}
impl PadCtrl15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PadCtrl15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PadCtrl15 {
    #[inline(always)]
    fn from(val: u8) -> PadCtrl15 {
        PadCtrl15::from_bits(val)
    }
}
impl From<PadCtrl15> for u8 {
    #[inline(always)]
    fn from(val: PadCtrl15) -> u8 {
        PadCtrl15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PadCtrl2 {
    #[doc = "Deasserts bit n in bbnsm_pad_ctrl\\[n\\]"]
    ZERO = 0x0,
    #[doc = "Assert bit n in bbnsm_pad_ctrl\\[n\\]"]
    ONE = 0x01,
}
impl PadCtrl2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PadCtrl2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PadCtrl2 {
    #[inline(always)]
    fn from(val: u8) -> PadCtrl2 {
        PadCtrl2::from_bits(val)
    }
}
impl From<PadCtrl2> for u8 {
    #[inline(always)]
    fn from(val: PadCtrl2) -> u8 {
        PadCtrl2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PadCtrl3 {
    #[doc = "Deasserts bit n in bbnsm_pad_ctrl\\[n\\]"]
    ZERO = 0x0,
    #[doc = "Assert bit n in bbnsm_pad_ctrl\\[n\\]"]
    ONE = 0x01,
}
impl PadCtrl3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PadCtrl3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PadCtrl3 {
    #[inline(always)]
    fn from(val: u8) -> PadCtrl3 {
        PadCtrl3::from_bits(val)
    }
}
impl From<PadCtrl3> for u8 {
    #[inline(always)]
    fn from(val: PadCtrl3) -> u8 {
        PadCtrl3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PadCtrl4 {
    #[doc = "Deasserts bit n in bbnsm_pad_ctrl\\[n\\]"]
    ZERO = 0x0,
    #[doc = "Assert bit n in bbnsm_pad_ctrl\\[n\\]"]
    ONE = 0x01,
}
impl PadCtrl4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PadCtrl4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PadCtrl4 {
    #[inline(always)]
    fn from(val: u8) -> PadCtrl4 {
        PadCtrl4::from_bits(val)
    }
}
impl From<PadCtrl4> for u8 {
    #[inline(always)]
    fn from(val: PadCtrl4) -> u8 {
        PadCtrl4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PadCtrl5 {
    #[doc = "Deasserts bit n in bbnsm_pad_ctrl\\[n\\]"]
    ZERO = 0x0,
    #[doc = "Assert bit n in bbnsm_pad_ctrl\\[n\\]"]
    ONE = 0x01,
}
impl PadCtrl5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PadCtrl5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PadCtrl5 {
    #[inline(always)]
    fn from(val: u8) -> PadCtrl5 {
        PadCtrl5::from_bits(val)
    }
}
impl From<PadCtrl5> for u8 {
    #[inline(always)]
    fn from(val: PadCtrl5) -> u8 {
        PadCtrl5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PadCtrl6 {
    #[doc = "Deasserts bit n in bbnsm_pad_ctrl\\[n\\]"]
    ZERO = 0x0,
    #[doc = "Assert bit n in bbnsm_pad_ctrl\\[n\\]"]
    ONE = 0x01,
}
impl PadCtrl6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PadCtrl6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PadCtrl6 {
    #[inline(always)]
    fn from(val: u8) -> PadCtrl6 {
        PadCtrl6::from_bits(val)
    }
}
impl From<PadCtrl6> for u8 {
    #[inline(always)]
    fn from(val: PadCtrl6) -> u8 {
        PadCtrl6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PadCtrl7 {
    #[doc = "Deasserts bit n in bbnsm_pad_ctrl\\[n\\]"]
    ZERO = 0x0,
    #[doc = "Assert bit n in bbnsm_pad_ctrl\\[n\\]"]
    ONE = 0x01,
}
impl PadCtrl7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PadCtrl7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PadCtrl7 {
    #[inline(always)]
    fn from(val: u8) -> PadCtrl7 {
        PadCtrl7::from_bits(val)
    }
}
impl From<PadCtrl7> for u8 {
    #[inline(always)]
    fn from(val: PadCtrl7) -> u8 {
        PadCtrl7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PadCtrl8 {
    #[doc = "Deasserts bit n in bbnsm_pad_ctrl\\[n\\]"]
    ZERO = 0x0,
    #[doc = "Assert bit n in bbnsm_pad_ctrl\\[n\\]"]
    ONE = 0x01,
}
impl PadCtrl8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PadCtrl8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PadCtrl8 {
    #[inline(always)]
    fn from(val: u8) -> PadCtrl8 {
        PadCtrl8::from_bits(val)
    }
}
impl From<PadCtrl8> for u8 {
    #[inline(always)]
    fn from(val: PadCtrl8) -> u8 {
        PadCtrl8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PadCtrl9 {
    #[doc = "Deasserts bit n in bbnsm_pad_ctrl\\[n\\]"]
    ZERO = 0x0,
    #[doc = "Assert bit n in bbnsm_pad_ctrl\\[n\\]"]
    ONE = 0x01,
}
impl PadCtrl9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PadCtrl9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PadCtrl9 {
    #[inline(always)]
    fn from(val: u8) -> PadCtrl9 {
        PadCtrl9::from_bits(val)
    }
}
impl From<PadCtrl9> for u8 {
    #[inline(always)]
    fn from(val: PadCtrl9) -> u8 {
        PadCtrl9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwrOff {
    #[doc = "The power off interrupt has not been requested."]
    NO_EVENT = 0x0,
    #[doc = "The power off interrupt has been requested."]
    EVENT_ASSERTED = 0x01,
}
impl PwrOff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwrOff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwrOff {
    #[inline(always)]
    fn from(val: u8) -> PwrOff {
        PwrOff::from_bits(val)
    }
}
impl From<PwrOff> for u8 {
    #[inline(always)]
    fn from(val: PwrOff) -> u8 {
        PwrOff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwrOn {
    #[doc = "The power on interrupt has not been requested."]
    NO_EVENT = 0x0,
    #[doc = "The power on interrupt has been requested."]
    EVENT_ASSERTED = 0x01,
}
impl PwrOn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwrOn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwrOn {
    #[inline(always)]
    fn from(val: u8) -> PwrOn {
        PwrOn::from_bits(val)
    }
}
impl From<PwrOn> for u8 {
    #[inline(always)]
    fn from(val: PwrOn) -> u8 {
        PwrOn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RtcEn {
    _RESERVED_0 = 0x0,
    #[doc = "Disable the real-time counter."]
    DISABLED = 0x01,
    #[doc = "Enable the real-time counter."]
    ENABLED = 0x02,
    _RESERVED_3 = 0x03,
}
impl RtcEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RtcEn {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RtcEn {
    #[inline(always)]
    fn from(val: u8) -> RtcEn {
        RtcEn::from_bits(val)
    }
}
impl From<RtcEn> for u8 {
    #[inline(always)]
    fn from(val: RtcEn) -> u8 {
        RtcEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RtcIntEn {
    _RESERVED_0 = 0x0,
    #[doc = "Do not issue an interrupt when RTC has rolled over. The interrupt is cleared when this value is written."]
    DISABLED = 0x01,
    #[doc = "Issue an interrupt when RTC has rolled over."]
    ENABLED = 0x02,
    _RESERVED_3 = 0x03,
}
impl RtcIntEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RtcIntEn {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RtcIntEn {
    #[inline(always)]
    fn from(val: u8) -> RtcIntEn {
        RtcIntEn::from_bits(val)
    }
}
impl From<RtcIntEn> for u8 {
    #[inline(always)]
    fn from(val: RtcIntEn) -> u8 {
        RtcIntEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RtcRoll {
    _RESERVED_0 = 0x0,
    #[doc = "The real-time counter has not rolled over."]
    NO_EVENT = 0x01,
    #[doc = "The real-time counter has rolled over."]
    EVENT_ASSERTED = 0x02,
    _RESERVED_3 = 0x03,
}
impl RtcRoll {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RtcRoll {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RtcRoll {
    #[inline(always)]
    fn from(val: u8) -> RtcRoll {
        RtcRoll::from_bits(val)
    }
}
impl From<RtcRoll> for u8 {
    #[inline(always)]
    fn from(val: RtcRoll) -> u8 {
        RtcRoll::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ta {
    _RESERVED_0 = 0x0,
    #[doc = "The real-time counter has not reached the alarm time."]
    NO_EVENT = 0x01,
    #[doc = "The real-time counter has reached the alarm time."]
    EVENT_ASSERTED = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ta {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ta {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ta {
    #[inline(always)]
    fn from(val: u8) -> Ta {
        Ta::from_bits(val)
    }
}
impl From<Ta> for u8 {
    #[inline(always)]
    fn from(val: Ta) -> u8 {
        Ta::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TaEn {
    _RESERVED_0 = 0x0,
    #[doc = "Disable the time alarm."]
    DISABLED = 0x01,
    #[doc = "Enable the time alarm. A time alarm event occurs if the value in the real-time counter register is equal to the value in the time alarm register."]
    ENABLED = 0x02,
    _RESERVED_3 = 0x03,
}
impl TaEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TaEn {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TaEn {
    #[inline(always)]
    fn from(val: u8) -> TaEn {
        TaEn::from_bits(val)
    }
}
impl From<TaEn> for u8 {
    #[inline(always)]
    fn from(val: TaEn) -> u8 {
        TaEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TaIntEn {
    _RESERVED_0 = 0x0,
    #[doc = "Do not issue an interrupt when RTC has reached alarm time. The interrupt is cleared when this value is written."]
    DISABLED = 0x01,
    #[doc = "Issue an interrupt when RTC has reached alarm time."]
    ENABLED = 0x02,
    _RESERVED_3 = 0x03,
}
impl TaIntEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TaIntEn {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TaIntEn {
    #[inline(always)]
    fn from(val: u8) -> TaIntEn {
        TaIntEn::from_bits(val)
    }
}
impl From<TaIntEn> for u8 {
    #[inline(always)]
    fn from(val: TaIntEn) -> u8 {
        TaIntEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TurnOnTime {
    #[doc = "500 milliseconds."]
    MSEC_50 = 0x0,
    #[doc = "50 milliseconds."]
    MSEC_100 = 0x01,
    #[doc = "100 milliseconds."]
    MSEC_500 = 0x02,
    #[doc = "0 milliseconds."]
    MSEC_0 = 0x03,
}
impl TurnOnTime {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TurnOnTime {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TurnOnTime {
    #[inline(always)]
    fn from(val: u8) -> TurnOnTime {
        TurnOnTime::from_bits(val)
    }
}
impl From<TurnOnTime> for u8 {
    #[inline(always)]
    fn from(val: TurnOnTime) -> u8 {
        TurnOnTime::to_bits(val)
    }
}
