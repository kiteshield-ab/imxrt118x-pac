#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Finish {
    #[doc = "No read"]
    NO_READING = 0x0,
    #[doc = "New read"]
    NEW_READING = 0x01,
}
impl Finish {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Finish {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Finish {
    #[inline(always)]
    fn from(val: u8) -> Finish {
        Finish::from_bits(val)
    }
}
impl From<Finish> for u8 {
    #[inline(always)]
    fn from(val: Finish) -> u8 {
        Finish::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Freq(pub u16);
impl Freq {
    #[doc = "Single Reading Mode. A new reading is available every time you change START from 0 to 1."]
    pub const SINGLE_MODE: Self = Self(0x0);
    #[doc = "Continuous Reading Mode. TMPSNS takes the next temperature reading when it completes the programmed number of cycles after the current reading."]
    pub const CONTINUOUS_MODE_1: Self = Self(0x01);
    #[doc = "Continuous Reading Mode. TMPSNS takes the next temperature reading when it completes the programmed number of cycles after the current reading."]
    pub const CONTINUOUS_MODE_2: Self = Self(0x02);
    #[doc = "Continuous Reading Mode. TMPSNS takes the next temperature reading when it completes the programmed number of cycles after the current reading."]
    pub const CONTINUOUS_MODE_3: Self = Self(0x03);
    #[doc = "Continuous Reading Mode. TMPSNS takes the next temperature reading when it completes the programmed number of cycles after the current reading."]
    pub const CONTINUOUS_MODE_4: Self = Self(0x04);
    #[doc = "Continuous Reading Mode. TMPSNS takes the next temperature reading when it completes the programmed number of cycles after the current reading."]
    pub const CONTINUOUS_MODE_5: Self = Self(0x05);
    #[doc = "Continuous Reading Mode. TMPSNS takes the next temperature reading when it completes the programmed number of cycles after the current reading."]
    pub const CONTINUOUS_MODE_6: Self = Self(0x06);
    #[doc = "Continuous Reading Mode. TMPSNS takes the next temperature reading when it completes the programmed number of cycles after the current reading."]
    pub const CONTINUOUS_MODE_7: Self = Self(0x07);
    #[doc = "Continuous Reading Mode. TMPSNS takes the next temperature reading when it completes the programmed number of cycles after the current reading."]
    pub const CONTINUOUS_MODE_8: Self = Self(0x08);
    #[doc = "Continuous Reading Mode. TMPSNS takes the next temperature reading when it completes the programmed number of cycles after the current reading."]
    pub const CONTINUOUS_MODE_9: Self = Self(0x09);
}
impl Freq {
    pub const fn from_bits(val: u16) -> Freq {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Freq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("SINGLE_MODE"),
            0x01 => f.write_str("CONTINUOUS_MODE_1"),
            0x02 => f.write_str("CONTINUOUS_MODE_2"),
            0x03 => f.write_str("CONTINUOUS_MODE_3"),
            0x04 => f.write_str("CONTINUOUS_MODE_4"),
            0x05 => f.write_str("CONTINUOUS_MODE_5"),
            0x06 => f.write_str("CONTINUOUS_MODE_6"),
            0x07 => f.write_str("CONTINUOUS_MODE_7"),
            0x08 => f.write_str("CONTINUOUS_MODE_8"),
            0x09 => f.write_str("CONTINUOUS_MODE_9"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Freq {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "SINGLE_MODE"),
            0x01 => defmt::write!(f, "CONTINUOUS_MODE_1"),
            0x02 => defmt::write!(f, "CONTINUOUS_MODE_2"),
            0x03 => defmt::write!(f, "CONTINUOUS_MODE_3"),
            0x04 => defmt::write!(f, "CONTINUOUS_MODE_4"),
            0x05 => defmt::write!(f, "CONTINUOUS_MODE_5"),
            0x06 => defmt::write!(f, "CONTINUOUS_MODE_6"),
            0x07 => defmt::write!(f, "CONTINUOUS_MODE_7"),
            0x08 => defmt::write!(f, "CONTINUOUS_MODE_8"),
            0x09 => defmt::write!(f, "CONTINUOUS_MODE_9"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Freq {
    #[inline(always)]
    fn from(val: u16) -> Freq {
        Freq::from_bits(val)
    }
}
impl From<Freq> for u16 {
    #[inline(always)]
    fn from(val: Freq) -> u16 {
        Freq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwd {
    #[doc = "Active"]
    ACTIVE = 0x0,
    #[doc = "Inactive"]
    INACTIVE = 0x01,
}
impl Pwd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwd {
    #[inline(always)]
    fn from(val: u8) -> Pwd {
        Pwd::from_bits(val)
    }
}
impl From<Pwd> for u8 {
    #[inline(always)]
    fn from(val: Pwd) -> u8 {
        Pwd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwdFull {
    #[doc = "Active"]
    ACTIVE = 0x0,
    #[doc = "Inactive"]
    INACTIVE = 0x01,
}
impl PwdFull {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwdFull {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwdFull {
    #[inline(always)]
    fn from(val: u8) -> PwdFull {
        PwdFull::from_bits(val)
    }
}
impl From<PwdFull> for u8 {
    #[inline(always)]
    fn from(val: PwdFull) -> u8 {
        PwdFull::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Start {
    #[doc = "No read"]
    NO_READING_TAKEN = 0x0,
    #[doc = "New read"]
    NEW_READING = 0x01,
}
impl Start {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Start {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Start {
    #[inline(always)]
    fn from(val: u8) -> Start {
        Start::from_bits(val)
    }
}
impl From<Start> for u8 {
    #[inline(always)]
    fn from(val: Start) -> u8 {
        Start::to_bits(val)
    }
}
