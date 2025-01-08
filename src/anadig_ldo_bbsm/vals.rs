#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegDisable {
    #[doc = "Enable"]
    ENABLE = 0x0,
    #[doc = "Disable"]
    DISABLE = 0x01,
}
impl RegDisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegDisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegDisable {
    #[inline(always)]
    fn from(val: u8) -> RegDisable {
        RegDisable::from_bits(val)
    }
}
impl From<RegDisable> for u8 {
    #[inline(always)]
    fn from(val: RegDisable) -> u8 {
        RegDisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegLpEn {
    #[doc = "Enable"]
    ENABLE = 0x0,
    #[doc = "Disable"]
    DISABLE = 0x01,
}
impl RegLpEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegLpEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegLpEn {
    #[inline(always)]
    fn from(val: u8) -> RegLpEn {
        RegLpEn::from_bits(val)
    }
}
impl From<RegLpEn> for u8 {
    #[inline(always)]
    fn from(val: RegLpEn) -> u8 {
        RegLpEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TrackModeEn {
    #[doc = "Normal use"]
    NORMAL = 0x0,
    #[doc = "Switch preparation"]
    SWITCH = 0x01,
}
impl TrackModeEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TrackModeEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TrackModeEn {
    #[inline(always)]
    fn from(val: u8) -> TrackModeEn {
        TrackModeEn::from_bits(val)
    }
}
impl From<TrackModeEn> for u8 {
    #[inline(always)]
    fn from(val: TrackModeEn) -> u8 {
        TrackModeEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VoltageSelect {
    #[doc = "Stable Voltage (range)"]
    BITVAL0 = 0x0,
    #[doc = "Stable Voltage (range)"]
    BITVAL1 = 0x01,
    #[doc = "Stable Voltage (range)"]
    BITVAL2 = 0x02,
    #[doc = "Stable Voltage (range)"]
    BITVAL3 = 0x03,
    #[doc = "Stable Voltage (range)"]
    BITVAL4 = 0x04,
    #[doc = "Stable Voltage (range)"]
    BITVAL5 = 0x05,
    #[doc = "Stable Voltage (range)"]
    BITVAL6 = 0x06,
    #[doc = "Stable Voltage (range)"]
    BITVAL7 = 0x07,
    #[doc = "Stable Voltage (range)"]
    BITVAL8 = 0x08,
    #[doc = "Stable Voltage (range)"]
    BITVAL9 = 0x09,
    #[doc = "Stable Voltage (range)"]
    BITVAL10 = 0x0a,
    #[doc = "Stable Voltage (range)"]
    BITVAL11 = 0x0b,
    #[doc = "Stable Voltage (range)"]
    BITVAL12 = 0x0c,
    #[doc = "Stable Voltage (range)"]
    BITVAL13 = 0x0d,
    #[doc = "Stable Voltage (range)"]
    BITVAL14 = 0x0e,
    #[doc = "Stable Voltage (range)"]
    BITVAL15 = 0x0f,
    #[doc = "Stable Voltage (range)"]
    BITVAL16 = 0x10,
    #[doc = "Stable Voltage (range)"]
    BITVAL17 = 0x11,
    #[doc = "Stable Voltage (range)"]
    BITVAL18 = 0x12,
    #[doc = "Stable Voltage (range)"]
    BITVAL19 = 0x13,
    #[doc = "Stable Voltage (range)"]
    BITVAL20 = 0x14,
    #[doc = "Stable Voltage (range)"]
    BITVAL21 = 0x15,
    #[doc = "Stable Voltage (range)"]
    BITVAL22 = 0x16,
    #[doc = "Stable Voltage (range)"]
    BITVAL23 = 0x17,
    #[doc = "Stable Voltage (range)"]
    BITVAL24 = 0x18,
    #[doc = "Stable Voltage (range)"]
    BITVAL25 = 0x19,
    #[doc = "Stable Voltage (range)"]
    BITVAL26 = 0x1a,
    #[doc = "Stable Voltage (range)"]
    BITVAL27 = 0x1b,
    #[doc = "Stable Voltage (range)"]
    BITVAL28 = 0x1c,
    #[doc = "Stable Voltage (range)"]
    BITVAL29 = 0x1d,
    #[doc = "Stable Voltage (range)"]
    BITVAL30 = 0x1e,
    #[doc = "Stable Voltage (range)"]
    BITVAL31 = 0x1f,
}
impl VoltageSelect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VoltageSelect {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VoltageSelect {
    #[inline(always)]
    fn from(val: u8) -> VoltageSelect {
        VoltageSelect::from_bits(val)
    }
}
impl From<VoltageSelect> for u8 {
    #[inline(always)]
    fn from(val: VoltageSelect) -> u8 {
        VoltageSelect::to_bits(val)
    }
}
