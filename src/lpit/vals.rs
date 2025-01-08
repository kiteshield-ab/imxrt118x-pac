#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mode {
    #[doc = "32-bit periodic counter"]
    CTR_32BIT = 0x0,
    #[doc = "Dual 16-bit periodic counter"]
    CTR_DUAL_16BIT = 0x01,
    #[doc = "32-bit trigger accumulator"]
    TRIG_ACCUM_32BIT = 0x02,
    #[doc = "32-bit trigger input capture"]
    TRIG_INPUT_32BIT = 0x03,
}
impl Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mode {
    #[inline(always)]
    fn from(val: u8) -> Mode {
        Mode::from_bits(val)
    }
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(val: Mode) -> u8 {
        Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tif0 {
    #[doc = "Not timed out"]
    TIMEOUT_NO = 0x0,
    #[doc = "Timed out"]
    TIMEOUT_YES = 0x01,
}
impl Tif0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tif0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tif0 {
    #[inline(always)]
    fn from(val: u8) -> Tif0 {
        Tif0::from_bits(val)
    }
}
impl From<Tif0> for u8 {
    #[inline(always)]
    fn from(val: Tif0) -> u8 {
        Tif0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tif1 {
    #[doc = "Not timed out"]
    TIMEOUT_NO = 0x0,
    #[doc = "Timed out"]
    TIMEOUT_YES = 0x01,
}
impl Tif1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tif1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tif1 {
    #[inline(always)]
    fn from(val: u8) -> Tif1 {
        Tif1::from_bits(val)
    }
}
impl From<Tif1> for u8 {
    #[inline(always)]
    fn from(val: Tif1) -> u8 {
        Tif1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tif2 {
    #[doc = "Not timed out"]
    TIMEOUT_NO = 0x0,
    #[doc = "Timed out"]
    TIMEOUT_YES = 0x01,
}
impl Tif2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tif2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tif2 {
    #[inline(always)]
    fn from(val: u8) -> Tif2 {
        Tif2::from_bits(val)
    }
}
impl From<Tif2> for u8 {
    #[inline(always)]
    fn from(val: Tif2) -> u8 {
        Tif2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tif3 {
    #[doc = "Not timed out"]
    TIMEOUT_NO = 0x0,
    #[doc = "Timed out"]
    TIMEOUT_YES = 0x01,
}
impl Tif3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tif3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tif3 {
    #[inline(always)]
    fn from(val: u8) -> Tif3 {
        Tif3::from_bits(val)
    }
}
impl From<Tif3> for u8 {
    #[inline(always)]
    fn from(val: Tif3) -> u8 {
        Tif3::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TmrVal(pub u32);
impl TmrVal {
    #[doc = "Invalid load value in Compare mode"]
    pub const INVALID_COMPARE_MODE_VALUE_0: Self = Self(0x0);
    #[doc = "Invalid load value in Compare mode"]
    pub const INVALID_COMPARE_MODE_VALUE_1: Self = Self(0x01);
    #[doc = "In Compare mode: the value to be loaded; in Capture mode, the value of the timer"]
    pub const VALUE_2: Self = Self(0x02);
    #[doc = "In Compare mode: the value to be loaded; in Capture mode, the value of the timer"]
    pub const VALUE_3: Self = Self(0x03);
    #[doc = "In Compare mode: the value to be loaded; in Capture mode, the value of the timer"]
    pub const VALUE_4: Self = Self(0x04);
    #[doc = "In Compare mode: the value to be loaded; in Capture mode, the value of the timer"]
    pub const VALUE_5: Self = Self(0x05);
    #[doc = "In Compare mode: the value to be loaded; in Capture mode, the value of the timer"]
    pub const VALUE_6: Self = Self(0x06);
    #[doc = "In Compare mode: the value to be loaded; in Capture mode, the value of the timer"]
    pub const VALUE_7: Self = Self(0x07);
    #[doc = "In Compare mode: the value to be loaded; in Capture mode, the value of the timer"]
    pub const VALUE_8: Self = Self(0x08);
    #[doc = "In Compare mode: the value to be loaded; in Capture mode, the value of the timer"]
    pub const VALUE_9: Self = Self(0x09);
}
impl TmrVal {
    pub const fn from_bits(val: u32) -> TmrVal {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for TmrVal {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("INVALID_COMPARE_MODE_VALUE_0"),
            0x01 => f.write_str("INVALID_COMPARE_MODE_VALUE_1"),
            0x02 => f.write_str("VALUE_2"),
            0x03 => f.write_str("VALUE_3"),
            0x04 => f.write_str("VALUE_4"),
            0x05 => f.write_str("VALUE_5"),
            0x06 => f.write_str("VALUE_6"),
            0x07 => f.write_str("VALUE_7"),
            0x08 => f.write_str("VALUE_8"),
            0x09 => f.write_str("VALUE_9"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TmrVal {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "INVALID_COMPARE_MODE_VALUE_0"),
            0x01 => defmt::write!(f, "INVALID_COMPARE_MODE_VALUE_1"),
            0x02 => defmt::write!(f, "VALUE_2"),
            0x03 => defmt::write!(f, "VALUE_3"),
            0x04 => defmt::write!(f, "VALUE_4"),
            0x05 => defmt::write!(f, "VALUE_5"),
            0x06 => defmt::write!(f, "VALUE_6"),
            0x07 => defmt::write!(f, "VALUE_7"),
            0x08 => defmt::write!(f, "VALUE_8"),
            0x09 => defmt::write!(f, "VALUE_9"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for TmrVal {
    #[inline(always)]
    fn from(val: u32) -> TmrVal {
        TmrVal::from_bits(val)
    }
}
impl From<TmrVal> for u32 {
    #[inline(always)]
    fn from(val: TmrVal) -> u32 {
        TmrVal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TrgSel {
    #[doc = "Timer channel 0-3 trigger source"]
    TRIG_SOURCE_0 = 0x0,
    #[doc = "Timer channel 0-3 trigger source"]
    TRIG_SOURCE_1 = 0x01,
    #[doc = "Timer channel 0-3 trigger source"]
    TRIG_SOURCE_2 = 0x02,
    #[doc = "Timer channel 0-3 trigger source"]
    TRIG_SOURCE_3 = 0x03,
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
impl TrgSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TrgSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TrgSel {
    #[inline(always)]
    fn from(val: u8) -> TrgSel {
        TrgSel::from_bits(val)
    }
}
impl From<TrgSel> for u8 {
    #[inline(always)]
    fn from(val: TrgSel) -> u8 {
        TrgSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TrgSrc {
    #[doc = "External"]
    EXT_TRIG = 0x0,
    #[doc = "Internal"]
    INT_TRIG = 0x01,
}
impl TrgSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TrgSrc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TrgSrc {
    #[inline(always)]
    fn from(val: u8) -> TrgSrc {
        TrgSrc::from_bits(val)
    }
}
impl From<TrgSrc> for u8 {
    #[inline(always)]
    fn from(val: TrgSrc) -> u8 {
        TrgSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsot {
    #[doc = "Immediately"]
    IMMEDIATELY = 0x0,
    #[doc = "When a rising edge is detected"]
    RISING_EDGE = 0x01,
}
impl Tsot {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsot {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsot {
    #[inline(always)]
    fn from(val: u8) -> Tsot {
        Tsot::from_bits(val)
    }
}
impl From<Tsot> for u8 {
    #[inline(always)]
    fn from(val: Tsot) -> u8 {
        Tsot::to_bits(val)
    }
}
