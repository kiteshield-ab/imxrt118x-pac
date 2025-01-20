#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmod {
    #[doc = "TPM counter is disabled"]
    Disable = 0x0,
    #[doc = "TPM counter increments on every TPM counter clock"]
    Counter = 0x01,
    #[doc = "TPM counter increments on the rising edge of EXTCLK synchronized to the TPM counter clock"]
    Extclk = 0x02,
    #[doc = "TPM counter increments on the rising edge of the selected external input trigger"]
    Trig = 0x03,
}
impl Cmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmod {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmod {
    #[inline(always)]
    fn from(val: u8) -> Cmod {
        Cmod::from_bits(val)
    }
}
impl From<Cmod> for u8 {
    #[inline(always)]
    fn from(val: Cmod) -> u8 {
        Cmod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpwms {
    #[doc = "Up counting mode"]
    Up = 0x0,
    #[doc = "Up-down counting mode"]
    UpDown = 0x01,
}
impl Cpwms {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpwms {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpwms {
    #[inline(always)]
    fn from(val: u8) -> Cpwms {
        Cpwms::from_bits(val)
    }
}
impl From<Cpwms> for u8 {
    #[inline(always)]
    fn from(val: Cpwms) -> u8 {
        Cpwms::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dbgmode {
    #[doc = "TPM counter pauses"]
    NoCount = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "TPM counter continues"]
    Count = 0x03,
}
impl Dbgmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dbgmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dbgmode {
    #[inline(always)]
    fn from(val: u8) -> Dbgmode {
        Dbgmode::from_bits(val)
    }
}
impl From<Dbgmode> for u8 {
    #[inline(always)]
    fn from(val: Dbgmode) -> u8 {
        Dbgmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dozeen {
    #[doc = "TPM counter continues"]
    Count = 0x0,
    #[doc = "TPM counter pauses"]
    NoCount = 0x01,
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Feature(pub u16);
impl Feature {
    #[doc = "Standard feature set"]
    pub const STANDARD: Self = Self(0x01);
    #[doc = "Standard feature set with the filter and combine registers implemented"]
    pub const FILT_COMBINE: Self = Self(0x03);
    #[doc = "Standard feature set with the quadrature register implemented"]
    pub const QUAD: Self = Self(0x05);
    #[doc = "Standard feature set with the filter, combine, and quadrature registers implemented"]
    pub const FILT_COMBINE_QUAD: Self = Self(0x07);
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
            0x01 => f.write_str("STANDARD"),
            0x03 => f.write_str("FILT_COMBINE"),
            0x05 => f.write_str("QUAD"),
            0x07 => f.write_str("FILT_COMBINE_QUAD"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Feature {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "STANDARD"),
            0x03 => defmt::write!(f, "FILT_COMBINE"),
            0x05 => defmt::write!(f, "QUAD"),
            0x07 => defmt::write!(f, "FILT_COMBINE_QUAD"),
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
pub enum Noupdate {
    #[doc = "Internal double-buffered registers update as normal"]
    Update = 0x0,
    #[doc = "Internal double-buffered registers do not update"]
    Noupdate = 0x01,
}
impl Noupdate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Noupdate {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Noupdate {
    #[inline(always)]
    fn from(val: u8) -> Noupdate {
        Noupdate::from_bits(val)
    }
}
impl From<Noupdate> for u8 {
    #[inline(always)]
    fn from(val: Noupdate) -> u8 {
        Noupdate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pol0 {
    #[doc = "Active high"]
    High = 0x0,
    #[doc = "Active low"]
    Low = 0x01,
}
impl Pol0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pol0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pol0 {
    #[inline(always)]
    fn from(val: u8) -> Pol0 {
        Pol0::from_bits(val)
    }
}
impl From<Pol0> for u8 {
    #[inline(always)]
    fn from(val: Pol0) -> u8 {
        Pol0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pol1 {
    #[doc = "Active high"]
    High = 0x0,
    #[doc = "Active low"]
    Low = 0x01,
}
impl Pol1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pol1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pol1 {
    #[inline(always)]
    fn from(val: u8) -> Pol1 {
        Pol1::from_bits(val)
    }
}
impl From<Pol1> for u8 {
    #[inline(always)]
    fn from(val: Pol1) -> u8 {
        Pol1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pol2 {
    #[doc = "Active high"]
    High = 0x0,
    #[doc = "Active low"]
    Low = 0x01,
}
impl Pol2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pol2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pol2 {
    #[inline(always)]
    fn from(val: u8) -> Pol2 {
        Pol2::from_bits(val)
    }
}
impl From<Pol2> for u8 {
    #[inline(always)]
    fn from(val: Pol2) -> u8 {
        Pol2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pol3 {
    #[doc = "Active high"]
    High = 0x0,
    #[doc = "Active low"]
    Low = 0x01,
}
impl Pol3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pol3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pol3 {
    #[inline(always)]
    fn from(val: u8) -> Pol3 {
        Pol3::from_bits(val)
    }
}
impl From<Pol3> for u8 {
    #[inline(always)]
    fn from(val: Pol3) -> u8 {
        Pol3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ps {
    #[doc = "Divide by 1"]
    Div1 = 0x0,
    #[doc = "Divide by 2"]
    Div2 = 0x01,
    #[doc = "Divide by 4"]
    Div4 = 0x02,
    #[doc = "Divide by 8"]
    Div8 = 0x03,
    #[doc = "Divide by 16"]
    Div16 = 0x04,
    #[doc = "Divide by 32"]
    Div32 = 0x05,
    #[doc = "Divide by 64"]
    Div64 = 0x06,
    #[doc = "Divide by 128"]
    Div128 = 0x07,
}
impl Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ps {
        unsafe { core::mem::transmute(val & 0x07) }
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
pub enum Quadir {
    #[doc = "Decreasing (counter decrement)"]
    Down = 0x0,
    #[doc = "Increasing (counter increment)"]
    Up = 0x01,
}
impl Quadir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Quadir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Quadir {
    #[inline(always)]
    fn from(val: u8) -> Quadir {
        Quadir::from_bits(val)
    }
}
impl From<Quadir> for u8 {
    #[inline(always)]
    fn from(val: Quadir) -> u8 {
        Quadir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Quadmode {
    #[doc = "Phase encoding mode"]
    Phase = 0x0,
    #[doc = "Count and direction encoding mode"]
    CountDir = 0x01,
}
impl Quadmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Quadmode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Quadmode {
    #[inline(always)]
    fn from(val: u8) -> Quadmode {
        Quadmode::from_bits(val)
    }
}
impl From<Quadmode> for u8 {
    #[inline(always)]
    fn from(val: Quadmode) -> u8 {
        Quadmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tofdir {
    #[doc = "Bottom of counting"]
    Bottom = 0x0,
    #[doc = "Top of counting"]
    Top = 0x01,
}
impl Tofdir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tofdir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tofdir {
    #[inline(always)]
    fn from(val: u8) -> Tofdir {
        Tofdir::from_bits(val)
    }
}
impl From<Tofdir> for u8 {
    #[inline(always)]
    fn from(val: Tofdir) -> u8 {
        Tofdir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trgpol {
    #[doc = "Active high"]
    High = 0x0,
    #[doc = "Active low"]
    Low = 0x01,
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
pub enum Trgsel {
    _RESERVED_0 = 0x0,
    #[doc = "Channel 0 pin input capture"]
    Ch0 = 0x01,
    #[doc = "Channel 1 pin input capture"]
    Ch1 = 0x02,
    #[doc = "Channel 0 or channel 1 pin input capture"]
    Ch01 = 0x03,
}
impl Trgsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trgsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trgsel {
    #[inline(always)]
    fn from(val: u8) -> Trgsel {
        Trgsel::from_bits(val)
    }
}
impl From<Trgsel> for u8 {
    #[inline(always)]
    fn from(val: Trgsel) -> u8 {
        Trgsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trgsrc {
    #[doc = "External"]
    External = 0x0,
    #[doc = "Internal (channel pin input capture)"]
    Internal = 0x01,
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
pub enum Trig0 {
    #[doc = "No effect"]
    NoEffect = 0x0,
    #[doc = "Configures trigger input 0 to be used by channel 0"]
    UseTrig = 0x01,
}
impl Trig0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig0 {
    #[inline(always)]
    fn from(val: u8) -> Trig0 {
        Trig0::from_bits(val)
    }
}
impl From<Trig0> for u8 {
    #[inline(always)]
    fn from(val: Trig0) -> u8 {
        Trig0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig1 {
    #[doc = "No effect"]
    NoEffect = 0x0,
    #[doc = "Configures trigger input 1 to be used by channel 1"]
    UseTrig = 0x01,
}
impl Trig1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig1 {
    #[inline(always)]
    fn from(val: u8) -> Trig1 {
        Trig1::from_bits(val)
    }
}
impl From<Trig1> for u8 {
    #[inline(always)]
    fn from(val: Trig1) -> u8 {
        Trig1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig2 {
    #[doc = "No effect"]
    NoEffect = 0x0,
    #[doc = "Configures trigger input 0 to be used by channel 2"]
    UseTrig = 0x01,
}
impl Trig2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig2 {
    #[inline(always)]
    fn from(val: u8) -> Trig2 {
        Trig2::from_bits(val)
    }
}
impl From<Trig2> for u8 {
    #[inline(always)]
    fn from(val: Trig2) -> u8 {
        Trig2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig3 {
    #[doc = "No effect"]
    NoEffect = 0x0,
    #[doc = "Configures trigger input 1 to be used by channel 3"]
    UseTrig = 0x01,
}
impl Trig3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig3 {
    #[inline(always)]
    fn from(val: u8) -> Trig3 {
        Trig3::from_bits(val)
    }
}
impl From<Trig3> for u8 {
    #[inline(always)]
    fn from(val: Trig3) -> u8 {
        Trig3::to_bits(val)
    }
}
