#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c1PinSclInSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AON_16 for Mode: ALT9"]
    SELECT_GPIO_AON_16_ALT9 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_21 for Mode: ALT3"]
    SELECT_GPIO_AON_21_ALT3 = 0x01,
}
impl I3c1PinSclInSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c1PinSclInSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c1PinSclInSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> I3c1PinSclInSelectInputDaisy {
        I3c1PinSclInSelectInputDaisy::from_bits(val)
    }
}
impl From<I3c1PinSclInSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: I3c1PinSclInSelectInputDaisy) -> u8 {
        I3c1PinSclInSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c1PinSdaInSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AON_15 for Mode: ALT9"]
    SELECT_GPIO_AON_15_ALT9 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_20 for Mode: ALT3"]
    SELECT_GPIO_AON_20_ALT3 = 0x01,
}
impl I3c1PinSdaInSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c1PinSdaInSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c1PinSdaInSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> I3c1PinSdaInSelectInputDaisy {
        I3c1PinSdaInSelectInputDaisy::from_bits(val)
    }
}
impl From<I3c1PinSdaInSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: I3c1PinSdaInSelectInputDaisy) -> u8 {
        I3c1PinSdaInSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c1IppIndLpi2cSclSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AON_07 for Mode: ALT3"]
    SELECT_GPIO_AON_07_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_09 for Mode: ALT6"]
    SELECT_GPIO_AON_09_ALT6 = 0x01,
    #[doc = "Selecting Pad: GPIO_AON_21 for Mode: ALT2"]
    SELECT_GPIO_AON_21_ALT2 = 0x02,
    #[doc = "Selecting Pad: GPIO_AON_25 for Mode: ALT1"]
    SELECT_GPIO_AON_25_ALT1 = 0x03,
}
impl Lpi2c1IppIndLpi2cSclSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c1IppIndLpi2cSclSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c1IppIndLpi2cSclSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c1IppIndLpi2cSclSelectInputDaisy {
        Lpi2c1IppIndLpi2cSclSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpi2c1IppIndLpi2cSclSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c1IppIndLpi2cSclSelectInputDaisy) -> u8 {
        Lpi2c1IppIndLpi2cSclSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c1IppIndLpi2cSdaSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AON_06 for Mode: ALT3"]
    SELECT_GPIO_AON_06_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_08 for Mode: ALT6"]
    SELECT_GPIO_AON_08_ALT6 = 0x01,
    #[doc = "Selecting Pad: GPIO_AON_20 for Mode: ALT2"]
    SELECT_GPIO_AON_20_ALT2 = 0x02,
    #[doc = "Selecting Pad: GPIO_AON_24 for Mode: ALT1"]
    SELECT_GPIO_AON_24_ALT1 = 0x03,
}
impl Lpi2c1IppIndLpi2cSdaSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c1IppIndLpi2cSdaSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c1IppIndLpi2cSdaSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c1IppIndLpi2cSdaSelectInputDaisy {
        Lpi2c1IppIndLpi2cSdaSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpi2c1IppIndLpi2cSdaSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c1IppIndLpi2cSdaSelectInputDaisy) -> u8 {
        Lpi2c1IppIndLpi2cSdaSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c2IppIndLpi2cSclSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AON_16 for Mode: ALT4"]
    SELECT_GPIO_AON_16_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_18 for Mode: ALT3"]
    SELECT_GPIO_AON_18_ALT3 = 0x01,
    #[doc = "Selecting Pad: GPIO_AON_23 for Mode: ALT1"]
    SELECT_GPIO_AON_23_ALT1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpi2c2IppIndLpi2cSclSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c2IppIndLpi2cSclSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c2IppIndLpi2cSclSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c2IppIndLpi2cSclSelectInputDaisy {
        Lpi2c2IppIndLpi2cSclSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpi2c2IppIndLpi2cSclSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c2IppIndLpi2cSclSelectInputDaisy) -> u8 {
        Lpi2c2IppIndLpi2cSclSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c2IppIndLpi2cSdaSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AON_15 for Mode: ALT4"]
    SELECT_GPIO_AON_15_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_17 for Mode: ALT3"]
    SELECT_GPIO_AON_17_ALT3 = 0x01,
    #[doc = "Selecting Pad: GPIO_AON_22 for Mode: ALT1"]
    SELECT_GPIO_AON_22_ALT1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpi2c2IppIndLpi2cSdaSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c2IppIndLpi2cSdaSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c2IppIndLpi2cSdaSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c2IppIndLpi2cSdaSelectInputDaisy {
        Lpi2c2IppIndLpi2cSdaSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpi2c2IppIndLpi2cSdaSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c2IppIndLpi2cSdaSelectInputDaisy) -> u8 {
        Lpi2c2IppIndLpi2cSdaSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi1IppIndLpspiPcsSelectInput0Daisy {
    #[doc = "Selecting Pad: GPIO_AON_05 for Mode: ALT0"]
    SELECT_GPIO_AON_05_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_13 for Mode: ALT8"]
    SELECT_GPIO_AON_13_ALT8 = 0x01,
}
impl Lpspi1IppIndLpspiPcsSelectInput0Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi1IppIndLpspiPcsSelectInput0Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi1IppIndLpspiPcsSelectInput0Daisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi1IppIndLpspiPcsSelectInput0Daisy {
        Lpspi1IppIndLpspiPcsSelectInput0Daisy::from_bits(val)
    }
}
impl From<Lpspi1IppIndLpspiPcsSelectInput0Daisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi1IppIndLpspiPcsSelectInput0Daisy) -> u8 {
        Lpspi1IppIndLpspiPcsSelectInput0Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi1IppIndLpspiPcsSelectInput1Daisy {
    #[doc = "Selecting Pad: GPIO_AON_03 for Mode: ALT2"]
    SELECT_GPIO_AON_03_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_08 for Mode: ALT8"]
    SELECT_GPIO_AON_08_ALT8 = 0x01,
}
impl Lpspi1IppIndLpspiPcsSelectInput1Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi1IppIndLpspiPcsSelectInput1Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi1IppIndLpspiPcsSelectInput1Daisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi1IppIndLpspiPcsSelectInput1Daisy {
        Lpspi1IppIndLpspiPcsSelectInput1Daisy::from_bits(val)
    }
}
impl From<Lpspi1IppIndLpspiPcsSelectInput1Daisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi1IppIndLpspiPcsSelectInput1Daisy) -> u8 {
        Lpspi1IppIndLpspiPcsSelectInput1Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi1IppIndLpspiSckSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AON_04 for Mode: ALT0"]
    SELECT_GPIO_AON_04_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_12 for Mode: ALT8"]
    SELECT_GPIO_AON_12_ALT8 = 0x01,
}
impl Lpspi1IppIndLpspiSckSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi1IppIndLpspiSckSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi1IppIndLpspiSckSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi1IppIndLpspiSckSelectInputDaisy {
        Lpspi1IppIndLpspiSckSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpspi1IppIndLpspiSckSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi1IppIndLpspiSckSelectInputDaisy) -> u8 {
        Lpspi1IppIndLpspiSckSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi1IppIndLpspiSdiSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AON_07 for Mode: ALT0"]
    SELECT_GPIO_AON_07_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_15 for Mode: ALT8"]
    SELECT_GPIO_AON_15_ALT8 = 0x01,
}
impl Lpspi1IppIndLpspiSdiSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi1IppIndLpspiSdiSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi1IppIndLpspiSdiSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi1IppIndLpspiSdiSelectInputDaisy {
        Lpspi1IppIndLpspiSdiSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpspi1IppIndLpspiSdiSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi1IppIndLpspiSdiSelectInputDaisy) -> u8 {
        Lpspi1IppIndLpspiSdiSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi1IppIndLpspiSdoSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AON_06 for Mode: ALT0"]
    SELECT_GPIO_AON_06_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_14 for Mode: ALT8"]
    SELECT_GPIO_AON_14_ALT8 = 0x01,
}
impl Lpspi1IppIndLpspiSdoSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi1IppIndLpspiSdoSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi1IppIndLpspiSdoSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi1IppIndLpspiSdoSelectInputDaisy {
        Lpspi1IppIndLpspiSdoSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpspi1IppIndLpspiSdoSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi1IppIndLpspiSdoSelectInputDaisy) -> u8 {
        Lpspi1IppIndLpspiSdoSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi2IppIndLpspiPcsSelectInput0Daisy {
    #[doc = "Selecting Pad: GPIO_AON_10 for Mode: ALT1"]
    SELECT_GPIO_AON_10_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_16 for Mode: ALT1"]
    SELECT_GPIO_AON_16_ALT1 = 0x01,
    #[doc = "Selecting Pad: GPIO_AON_25 for Mode: ALT6"]
    SELECT_GPIO_AON_25_ALT6 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpspi2IppIndLpspiPcsSelectInput0Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi2IppIndLpspiPcsSelectInput0Daisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi2IppIndLpspiPcsSelectInput0Daisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi2IppIndLpspiPcsSelectInput0Daisy {
        Lpspi2IppIndLpspiPcsSelectInput0Daisy::from_bits(val)
    }
}
impl From<Lpspi2IppIndLpspiPcsSelectInput0Daisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi2IppIndLpspiPcsSelectInput0Daisy) -> u8 {
        Lpspi2IppIndLpspiPcsSelectInput0Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi2IppIndLpspiPcsSelectInput1Daisy {
    #[doc = "Selecting Pad: GPIO_AON_15 for Mode: ALT1"]
    SELECT_GPIO_AON_15_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_21 for Mode: ALT1"]
    SELECT_GPIO_AON_21_ALT1 = 0x01,
}
impl Lpspi2IppIndLpspiPcsSelectInput1Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi2IppIndLpspiPcsSelectInput1Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi2IppIndLpspiPcsSelectInput1Daisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi2IppIndLpspiPcsSelectInput1Daisy {
        Lpspi2IppIndLpspiPcsSelectInput1Daisy::from_bits(val)
    }
}
impl From<Lpspi2IppIndLpspiPcsSelectInput1Daisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi2IppIndLpspiPcsSelectInput1Daisy) -> u8 {
        Lpspi2IppIndLpspiPcsSelectInput1Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi2IppIndLpspiPcsSelectInput3Daisy {
    #[doc = "Selecting Pad: GPIO_AON_02 for Mode: ALT2"]
    SELECT_GPIO_AON_02_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_27 for Mode: ALT1"]
    SELECT_GPIO_AON_27_ALT1 = 0x01,
}
impl Lpspi2IppIndLpspiPcsSelectInput3Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi2IppIndLpspiPcsSelectInput3Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi2IppIndLpspiPcsSelectInput3Daisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi2IppIndLpspiPcsSelectInput3Daisy {
        Lpspi2IppIndLpspiPcsSelectInput3Daisy::from_bits(val)
    }
}
impl From<Lpspi2IppIndLpspiPcsSelectInput3Daisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi2IppIndLpspiPcsSelectInput3Daisy) -> u8 {
        Lpspi2IppIndLpspiPcsSelectInput3Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi2IppIndLpspiSckSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AON_19 for Mode: ALT1"]
    SELECT_GPIO_AON_19_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_22 for Mode: ALT6"]
    SELECT_GPIO_AON_22_ALT6 = 0x01,
}
impl Lpspi2IppIndLpspiSckSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi2IppIndLpspiSckSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi2IppIndLpspiSckSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi2IppIndLpspiSckSelectInputDaisy {
        Lpspi2IppIndLpspiSckSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpspi2IppIndLpspiSckSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi2IppIndLpspiSckSelectInputDaisy) -> u8 {
        Lpspi2IppIndLpspiSckSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi2IppIndLpspiSdiSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AON_03 for Mode: ALT3"]
    SELECT_GPIO_AON_03_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_17 for Mode: ALT1"]
    SELECT_GPIO_AON_17_ALT1 = 0x01,
    #[doc = "Selecting Pad: GPIO_AON_24 for Mode: ALT6"]
    SELECT_GPIO_AON_24_ALT6 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpspi2IppIndLpspiSdiSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi2IppIndLpspiSdiSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi2IppIndLpspiSdiSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi2IppIndLpspiSdiSelectInputDaisy {
        Lpspi2IppIndLpspiSdiSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpspi2IppIndLpspiSdiSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi2IppIndLpspiSdiSelectInputDaisy) -> u8 {
        Lpspi2IppIndLpspiSdiSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi2IppIndLpspiSdoSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AON_02 for Mode: ALT3"]
    SELECT_GPIO_AON_02_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_18 for Mode: ALT1"]
    SELECT_GPIO_AON_18_ALT1 = 0x01,
    #[doc = "Selecting Pad: GPIO_AON_23 for Mode: ALT6"]
    SELECT_GPIO_AON_23_ALT6 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpspi2IppIndLpspiSdoSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi2IppIndLpspiSdoSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi2IppIndLpspiSdoSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi2IppIndLpspiSdoSelectInputDaisy {
        Lpspi2IppIndLpspiSdoSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpspi2IppIndLpspiSdoSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi2IppIndLpspiSdoSelectInputDaisy) -> u8 {
        Lpspi2IppIndLpspiSdoSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lptmr1IppIndLptimerSelectInput1Daisy {
    #[doc = "Selecting Pad: GPIO_AON_00 for Mode: ALT4"]
    SELECT_GPIO_AON_00_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_13 for Mode: ALT6"]
    SELECT_GPIO_AON_13_ALT6 = 0x01,
}
impl Lptmr1IppIndLptimerSelectInput1Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lptmr1IppIndLptimerSelectInput1Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lptmr1IppIndLptimerSelectInput1Daisy {
    #[inline(always)]
    fn from(val: u8) -> Lptmr1IppIndLptimerSelectInput1Daisy {
        Lptmr1IppIndLptimerSelectInput1Daisy::from_bits(val)
    }
}
impl From<Lptmr1IppIndLptimerSelectInput1Daisy> for u8 {
    #[inline(always)]
    fn from(val: Lptmr1IppIndLptimerSelectInput1Daisy) -> u8 {
        Lptmr1IppIndLptimerSelectInput1Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lptmr1IppIndLptimerSelectInput2Daisy {
    #[doc = "Selecting Pad: GPIO_AON_01 for Mode: ALT4"]
    SELECT_GPIO_AON_01_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_14 for Mode: ALT6"]
    SELECT_GPIO_AON_14_ALT6 = 0x01,
}
impl Lptmr1IppIndLptimerSelectInput2Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lptmr1IppIndLptimerSelectInput2Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lptmr1IppIndLptimerSelectInput2Daisy {
    #[inline(always)]
    fn from(val: u8) -> Lptmr1IppIndLptimerSelectInput2Daisy {
        Lptmr1IppIndLptimerSelectInput2Daisy::from_bits(val)
    }
}
impl From<Lptmr1IppIndLptimerSelectInput2Daisy> for u8 {
    #[inline(always)]
    fn from(val: Lptmr1IppIndLptimerSelectInput2Daisy) -> u8 {
        Lptmr1IppIndLptimerSelectInput2Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lptmr1IppIndLptimerSelectInput3Daisy {
    #[doc = "Selecting Pad: GPIO_AON_02 for Mode: ALT4"]
    SELECT_GPIO_AON_02_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_15 for Mode: ALT6"]
    SELECT_GPIO_AON_15_ALT6 = 0x01,
}
impl Lptmr1IppIndLptimerSelectInput3Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lptmr1IppIndLptimerSelectInput3Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lptmr1IppIndLptimerSelectInput3Daisy {
    #[inline(always)]
    fn from(val: u8) -> Lptmr1IppIndLptimerSelectInput3Daisy {
        Lptmr1IppIndLptimerSelectInput3Daisy::from_bits(val)
    }
}
impl From<Lptmr1IppIndLptimerSelectInput3Daisy> for u8 {
    #[inline(always)]
    fn from(val: Lptmr1IppIndLptimerSelectInput3Daisy) -> u8 {
        Lptmr1IppIndLptimerSelectInput3Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart12IppIndLpuartCtsNSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AON_13 for Mode: ALT2"]
    SELECT_GPIO_AON_13_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_22 for Mode: ALT3"]
    SELECT_GPIO_AON_22_ALT3 = 0x01,
}
impl Lpuart12IppIndLpuartCtsNSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart12IppIndLpuartCtsNSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart12IppIndLpuartCtsNSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart12IppIndLpuartCtsNSelectInputDaisy {
        Lpuart12IppIndLpuartCtsNSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart12IppIndLpuartCtsNSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart12IppIndLpuartCtsNSelectInputDaisy) -> u8 {
        Lpuart12IppIndLpuartCtsNSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart12IppIndLpuartRxdSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AON_16 for Mode: ALT2"]
    SELECT_GPIO_AON_16_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_20 for Mode: ALT9"]
    SELECT_GPIO_AON_20_ALT9 = 0x01,
}
impl Lpuart12IppIndLpuartRxdSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart12IppIndLpuartRxdSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart12IppIndLpuartRxdSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart12IppIndLpuartRxdSelectInputDaisy {
        Lpuart12IppIndLpuartRxdSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart12IppIndLpuartRxdSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart12IppIndLpuartRxdSelectInputDaisy) -> u8 {
        Lpuart12IppIndLpuartRxdSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart12IppIndLpuartTxdSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AON_15 for Mode: ALT2"]
    SELECT_GPIO_AON_15_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_19 for Mode: ALT9"]
    SELECT_GPIO_AON_19_ALT9 = 0x01,
}
impl Lpuart12IppIndLpuartTxdSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart12IppIndLpuartTxdSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart12IppIndLpuartTxdSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart12IppIndLpuartTxdSelectInputDaisy {
        Lpuart12IppIndLpuartTxdSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart12IppIndLpuartTxdSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart12IppIndLpuartTxdSelectInputDaisy) -> u8 {
        Lpuart12IppIndLpuartTxdSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart1IppIndLpuartCtsNSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AON_11 for Mode: ALT2"]
    SELECT_GPIO_AON_11_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_19 for Mode: ALT4"]
    SELECT_GPIO_AON_19_ALT4 = 0x01,
}
impl Lpuart1IppIndLpuartCtsNSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart1IppIndLpuartCtsNSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart1IppIndLpuartCtsNSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart1IppIndLpuartCtsNSelectInputDaisy {
        Lpuart1IppIndLpuartCtsNSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart1IppIndLpuartCtsNSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart1IppIndLpuartCtsNSelectInputDaisy) -> u8 {
        Lpuart1IppIndLpuartCtsNSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart1IppIndLpuartDcdNSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AON_14 for Mode: ALT3"]
    SELECT_GPIO_AON_14_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_18 for Mode: ALT4"]
    SELECT_GPIO_AON_18_ALT4 = 0x01,
}
impl Lpuart1IppIndLpuartDcdNSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart1IppIndLpuartDcdNSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart1IppIndLpuartDcdNSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart1IppIndLpuartDcdNSelectInputDaisy {
        Lpuart1IppIndLpuartDcdNSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart1IppIndLpuartDcdNSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart1IppIndLpuartDcdNSelectInputDaisy) -> u8 {
        Lpuart1IppIndLpuartDcdNSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart1IppIndLpuartDsrNSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AON_13 for Mode: ALT3"]
    SELECT_GPIO_AON_13_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_17 for Mode: ALT4"]
    SELECT_GPIO_AON_17_ALT4 = 0x01,
}
impl Lpuart1IppIndLpuartDsrNSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart1IppIndLpuartDsrNSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart1IppIndLpuartDsrNSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart1IppIndLpuartDsrNSelectInputDaisy {
        Lpuart1IppIndLpuartDsrNSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart1IppIndLpuartDsrNSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart1IppIndLpuartDsrNSelectInputDaisy) -> u8 {
        Lpuart1IppIndLpuartDsrNSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart2IppIndLpuartCtsNSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AON_03 for Mode: ALT6"]
    SELECT_GPIO_AON_03_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_25 for Mode: ALT2"]
    SELECT_GPIO_AON_25_ALT2 = 0x01,
}
impl Lpuart2IppIndLpuartCtsNSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart2IppIndLpuartCtsNSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart2IppIndLpuartCtsNSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart2IppIndLpuartCtsNSelectInputDaisy {
        Lpuart2IppIndLpuartCtsNSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart2IppIndLpuartCtsNSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart2IppIndLpuartCtsNSelectInputDaisy) -> u8 {
        Lpuart2IppIndLpuartCtsNSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart2IppIndLpuartRxdSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AON_01 for Mode: ALT6"]
    SELECT_GPIO_AON_01_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_27 for Mode: ALT2"]
    SELECT_GPIO_AON_27_ALT2 = 0x01,
}
impl Lpuart2IppIndLpuartRxdSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart2IppIndLpuartRxdSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart2IppIndLpuartRxdSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart2IppIndLpuartRxdSelectInputDaisy {
        Lpuart2IppIndLpuartRxdSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart2IppIndLpuartRxdSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart2IppIndLpuartRxdSelectInputDaisy) -> u8 {
        Lpuart2IppIndLpuartRxdSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart2IppIndLpuartTxdSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AON_00 for Mode: ALT6"]
    SELECT_GPIO_AON_00_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_26 for Mode: ALT2"]
    SELECT_GPIO_AON_26_ALT2 = 0x01,
}
impl Lpuart2IppIndLpuartTxdSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart2IppIndLpuartTxdSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart2IppIndLpuartTxdSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart2IppIndLpuartTxdSelectInputDaisy {
        Lpuart2IppIndLpuartTxdSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart2IppIndLpuartTxdSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart2IppIndLpuartTxdSelectInputDaisy) -> u8 {
        Lpuart2IppIndLpuartTxdSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart7IppIndLpuartCtsNSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AON_04 for Mode: ALT6"]
    SELECT_GPIO_AON_04_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_16 for Mode: ALT8"]
    SELECT_GPIO_AON_16_ALT8 = 0x01,
    #[doc = "Selecting Pad: GPIO_AON_24 for Mode: ALT3"]
    SELECT_GPIO_AON_24_ALT3 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpuart7IppIndLpuartCtsNSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart7IppIndLpuartCtsNSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart7IppIndLpuartCtsNSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart7IppIndLpuartCtsNSelectInputDaisy {
        Lpuart7IppIndLpuartCtsNSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart7IppIndLpuartCtsNSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart7IppIndLpuartCtsNSelectInputDaisy) -> u8 {
        Lpuart7IppIndLpuartCtsNSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart7IppIndLpuartRxdSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AON_18 for Mode: ALT2"]
    SELECT_GPIO_AON_18_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_23 for Mode: ALT2"]
    SELECT_GPIO_AON_23_ALT2 = 0x01,
}
impl Lpuart7IppIndLpuartRxdSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart7IppIndLpuartRxdSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart7IppIndLpuartRxdSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart7IppIndLpuartRxdSelectInputDaisy {
        Lpuart7IppIndLpuartRxdSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart7IppIndLpuartRxdSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart7IppIndLpuartRxdSelectInputDaisy) -> u8 {
        Lpuart7IppIndLpuartRxdSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart7IppIndLpuartTxdSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AON_17 for Mode: ALT2"]
    SELECT_GPIO_AON_17_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_22 for Mode: ALT2"]
    SELECT_GPIO_AON_22_ALT2 = 0x01,
}
impl Lpuart7IppIndLpuartTxdSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart7IppIndLpuartTxdSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart7IppIndLpuartTxdSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart7IppIndLpuartTxdSelectInputDaisy {
        Lpuart7IppIndLpuartTxdSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart7IppIndLpuartTxdSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart7IppIndLpuartTxdSelectInputDaisy) -> u8 {
        Lpuart7IppIndLpuartTxdSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1IpgClkSaiMclkSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AON_07 for Mode: ALT2"]
    SELECT_GPIO_AON_07_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_24 for Mode: ALT4"]
    SELECT_GPIO_AON_24_ALT4 = 0x01,
}
impl Sai1IpgClkSaiMclkSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1IpgClkSaiMclkSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1IpgClkSaiMclkSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Sai1IpgClkSaiMclkSelectInputDaisy {
        Sai1IpgClkSaiMclkSelectInputDaisy::from_bits(val)
    }
}
impl From<Sai1IpgClkSaiMclkSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Sai1IpgClkSaiMclkSelectInputDaisy) -> u8 {
        Sai1IpgClkSaiMclkSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1IppIndSaiRxbclkSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AON_09 for Mode: ALT2"]
    SELECT_GPIO_AON_09_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_26 for Mode: ALT4"]
    SELECT_GPIO_AON_26_ALT4 = 0x01,
}
impl Sai1IppIndSaiRxbclkSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1IppIndSaiRxbclkSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1IppIndSaiRxbclkSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Sai1IppIndSaiRxbclkSelectInputDaisy {
        Sai1IppIndSaiRxbclkSelectInputDaisy::from_bits(val)
    }
}
impl From<Sai1IppIndSaiRxbclkSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Sai1IppIndSaiRxbclkSelectInputDaisy) -> u8 {
        Sai1IppIndSaiRxbclkSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1IppIndSaiRxdataSelectInput0Daisy {
    #[doc = "Selecting Pad: GPIO_AON_08 for Mode: ALT2"]
    SELECT_GPIO_AON_08_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_25 for Mode: ALT4"]
    SELECT_GPIO_AON_25_ALT4 = 0x01,
}
impl Sai1IppIndSaiRxdataSelectInput0Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1IppIndSaiRxdataSelectInput0Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1IppIndSaiRxdataSelectInput0Daisy {
    #[inline(always)]
    fn from(val: u8) -> Sai1IppIndSaiRxdataSelectInput0Daisy {
        Sai1IppIndSaiRxdataSelectInput0Daisy::from_bits(val)
    }
}
impl From<Sai1IppIndSaiRxdataSelectInput0Daisy> for u8 {
    #[inline(always)]
    fn from(val: Sai1IppIndSaiRxdataSelectInput0Daisy) -> u8 {
        Sai1IppIndSaiRxdataSelectInput0Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1IppIndSaiRxdataSelectInput1Daisy {
    #[doc = "Selecting Pad: GPIO_AON_04 for Mode: ALT3"]
    SELECT_GPIO_AON_04_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_21 for Mode: ALT9"]
    SELECT_GPIO_AON_21_ALT9 = 0x01,
}
impl Sai1IppIndSaiRxdataSelectInput1Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1IppIndSaiRxdataSelectInput1Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1IppIndSaiRxdataSelectInput1Daisy {
    #[inline(always)]
    fn from(val: u8) -> Sai1IppIndSaiRxdataSelectInput1Daisy {
        Sai1IppIndSaiRxdataSelectInput1Daisy::from_bits(val)
    }
}
impl From<Sai1IppIndSaiRxdataSelectInput1Daisy> for u8 {
    #[inline(always)]
    fn from(val: Sai1IppIndSaiRxdataSelectInput1Daisy) -> u8 {
        Sai1IppIndSaiRxdataSelectInput1Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1IppIndSaiRxsyncSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AON_10 for Mode: ALT2"]
    SELECT_GPIO_AON_10_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_27 for Mode: ALT4"]
    SELECT_GPIO_AON_27_ALT4 = 0x01,
}
impl Sai1IppIndSaiRxsyncSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1IppIndSaiRxsyncSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1IppIndSaiRxsyncSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Sai1IppIndSaiRxsyncSelectInputDaisy {
        Sai1IppIndSaiRxsyncSelectInputDaisy::from_bits(val)
    }
}
impl From<Sai1IppIndSaiRxsyncSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Sai1IppIndSaiRxsyncSelectInputDaisy) -> u8 {
        Sai1IppIndSaiRxsyncSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1IppIndSaiTxbclkSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AON_06 for Mode: ALT2"]
    SELECT_GPIO_AON_06_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_23 for Mode: ALT4"]
    SELECT_GPIO_AON_23_ALT4 = 0x01,
}
impl Sai1IppIndSaiTxbclkSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1IppIndSaiTxbclkSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1IppIndSaiTxbclkSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Sai1IppIndSaiTxbclkSelectInputDaisy {
        Sai1IppIndSaiTxbclkSelectInputDaisy::from_bits(val)
    }
}
impl From<Sai1IppIndSaiTxbclkSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Sai1IppIndSaiTxbclkSelectInputDaisy) -> u8 {
        Sai1IppIndSaiTxbclkSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1IppIndSaiTxsyncSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AON_05 for Mode: ALT2"]
    SELECT_GPIO_AON_05_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_22 for Mode: ALT4"]
    SELECT_GPIO_AON_22_ALT4 = 0x01,
}
impl Sai1IppIndSaiTxsyncSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1IppIndSaiTxsyncSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1IppIndSaiTxsyncSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Sai1IppIndSaiTxsyncSelectInputDaisy {
        Sai1IppIndSaiTxsyncSelectInputDaisy::from_bits(val)
    }
}
impl From<Sai1IppIndSaiTxsyncSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Sai1IppIndSaiTxsyncSelectInputDaisy) -> u8 {
        Sai1IppIndSaiTxsyncSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAon00MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SRC_BOOT_MODE00 of instance: src"]
    ALT0_SRC_BOOT_MODE0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: CAN1_TX of instance: can1"]
    ALT1_CAN1_TX = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPTMR1_ALT1 of instance: lptmr1"]
    ALT4_LPTMR1_ALT1 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO00 of instance: gpio1"]
    ALT5_GPIO1_IO0 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPUART2_TX of instance: lpuart2"]
    ALT6_LPUART2_TX = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: TPM1_EXTCLK of instance: tpm1"]
    ALT8_TPM1_EXTCLK = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAon00MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAon00MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAon00MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAon00MuxMode {
        SwMuxCtlPadGpioAon00MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAon00MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAon00MuxMode) -> u8 {
        SwMuxCtlPadGpioAon00MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAon01MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SRC_BOOT_MODE01 of instance: src"]
    ALT0_SRC_BOOT_MODE1 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: CAN1_RX of instance: can1"]
    ALT1_CAN1_RX = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPTMR1_ALT2 of instance: lptmr1"]
    ALT4_LPTMR1_ALT2 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO01 of instance: gpio1"]
    ALT5_GPIO1_IO1 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPUART2_RX of instance: lpuart2"]
    ALT6_LPUART2_RX = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: TPM1_CH00 of instance: tpm1"]
    ALT8_TPM1_CH0 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAon01MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAon01MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAon01MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAon01MuxMode {
        SwMuxCtlPadGpioAon01MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAon01MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAon01MuxMode) -> u8 {
        SwMuxCtlPadGpioAon01MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAon02MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SRC_BOOT_MODE02 of instance: src"]
    ALT0_SRC_BOOT_MODE2 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: CAN3_TX of instance: can3"]
    ALT1_CAN3_TX = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPSPI2_PCS3 of instance: lpspi2"]
    ALT2_LPSPI2_PCS3 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPSPI2_SDO of instance: lpspi2"]
    ALT3_LPSPI2_SDO = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPTMR1_ALT3 of instance: lptmr1"]
    ALT4_LPTMR1_ALT3 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO02 of instance: gpio1"]
    ALT5_GPIO1_IO2 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPUART2_RTS_B of instance: lpuart2"]
    ALT6_LPUART2_RTS_B = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: TPM1_CH01 of instance: tpm1"]
    ALT8_TPM1_CH1 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_CLK_ECAT_CLK25 of instance: ecat"]
    ALT12_ECAT_CLK_ECAT_CLK25 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAon02MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAon02MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAon02MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAon02MuxMode {
        SwMuxCtlPadGpioAon02MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAon02MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAon02MuxMode) -> u8 {
        SwMuxCtlPadGpioAon02MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAon03MuxMode {
    _RESERVED_0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: CAN3_RX of instance: can3"]
    ALT1_CAN3_RX = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPSPI1_PCS1 of instance: lpspi1"]
    ALT2_LPSPI1_PCS1 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPSPI2_SDI of instance: lpspi2"]
    ALT3_LPSPI2_SDI = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPSPI1_PCS3 of instance: lpspi1"]
    ALT4_LPSPI1_PCS3 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO03 of instance: gpio1"]
    ALT5_GPIO1_IO3 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPUART2_CTS_B of instance: lpuart2"]
    ALT6_LPUART2_CTS_B = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: TPM1_CH02 of instance: tpm1"]
    ALT8_TPM1_CH2 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_LED_STATE_RUN of instance: ecat"]
    ALT12_ECAT_LED_STATE_RUN = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAon03MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAon03MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAon03MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAon03MuxMode {
        SwMuxCtlPadGpioAon03MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAon03MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAon03MuxMode) -> u8 {
        SwMuxCtlPadGpioAon03MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAon04MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: LPSPI1_SCK of instance: lpspi1"]
    ALT0_LPSPI1_SCK = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SAI1_TX_DATA00 of instance: sai1"]
    ALT2_SAI1_TX_DATA0 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SAI1_RX_DATA01 of instance: sai1"]
    ALT3_SAI1_RX_DATA1 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO04 of instance: gpio1"]
    ALT5_GPIO1_IO4 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPUART7_CTS_B of instance: lpuart7"]
    ALT6_LPUART7_CTS_B = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: TPM1_CH03 of instance: tpm1"]
    ALT8_TPM1_CH3 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_LED_RUN of instance: ecat"]
    ALT12_ECAT_LED_RUN = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAon04MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAon04MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAon04MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAon04MuxMode {
        SwMuxCtlPadGpioAon04MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAon04MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAon04MuxMode) -> u8 {
        SwMuxCtlPadGpioAon04MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAon05MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: LPSPI1_PCS0 of instance: lpspi1"]
    ALT0_LPSPI1_PCS0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SAI1_TX_SYNC of instance: sai1"]
    ALT2_SAI1_TX_SYNC = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO05 of instance: gpio1"]
    ALT5_GPIO1_IO5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPUART7_RTS_B of instance: lpuart7"]
    ALT6_LPUART7_RTS_B = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: NMI_GLUE_NMI of instance: nmi_glue"]
    ALT7_NMI_GLUE_NMI = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_LED_ERR of instance: ecat"]
    ALT12_ECAT_LED_ERR = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAon05MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAon05MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAon05MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAon05MuxMode {
        SwMuxCtlPadGpioAon05MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAon05MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAon05MuxMode) -> u8 {
        SwMuxCtlPadGpioAon05MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAon06MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: LPSPI1_SDO of instance: lpspi1"]
    ALT0_LPSPI1_SDO = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: I3C1_PUR of instance: i3c1"]
    ALT1_I3C1_PUR = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SAI1_TX_BCLK of instance: sai1"]
    ALT2_SAI1_TX_BCLK = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPI2C1_SDA of instance: lpi2c1"]
    ALT3_LPI2C1_SDA = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO06 of instance: gpio1"]
    ALT5_GPIO1_IO6 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: CAN1_TX of instance: can1"]
    ALT6_CAN1_TX = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_SDA of instance: ecat"]
    ALT12_ECAT_SDA = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAon06MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAon06MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAon06MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAon06MuxMode {
        SwMuxCtlPadGpioAon06MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAon06MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAon06MuxMode) -> u8 {
        SwMuxCtlPadGpioAon06MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAon07MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: LPSPI1_SDI of instance: lpspi1"]
    ALT0_LPSPI1_SDI = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SAI1_MCLK of instance: sai1"]
    ALT2_SAI1_MCLK = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPI2C1_SCL of instance: lpi2c1"]
    ALT3_LPI2C1_SCL = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO07 of instance: gpio1"]
    ALT5_GPIO1_IO7 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: CAN1_RX of instance: can1"]
    ALT6_CAN1_RX = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_SCL of instance: ecat"]
    ALT12_ECAT_SCL = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAon07MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAon07MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAon07MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAon07MuxMode {
        SwMuxCtlPadGpioAon07MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAon07MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAon07MuxMode) -> u8 {
        SwMuxCtlPadGpioAon07MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAon08MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: LPUART1_TX of instance: lpuart1"]
    ALT0_LPUART1_TX = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: S400_TX of instance: s400"]
    ALT1_S400_TX = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SAI1_RX_DATA00 of instance: sai1"]
    ALT2_SAI1_RX_DATA0 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SAI1_TX_DATA01 of instance: sai1"]
    ALT3_SAI1_TX_DATA1 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO08 of instance: gpio1"]
    ALT5_GPIO1_IO8 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPI2C1_SDA of instance: lpi2c1"]
    ALT6_LPI2C1_SDA = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPSPI1_PCS1 of instance: lpspi1"]
    ALT8_LPSPI1_PCS1 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_LINK_ACT00 of instance: ecat"]
    ALT12_ECAT_LINK_ACT0 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAon08MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAon08MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAon08MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAon08MuxMode {
        SwMuxCtlPadGpioAon08MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAon08MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAon08MuxMode) -> u8 {
        SwMuxCtlPadGpioAon08MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAon09MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: LPUART1_RX of instance: lpuart1"]
    ALT0_LPUART1_RX = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: S400_RX of instance: s400"]
    ALT1_S400_RX = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SAI1_RX_BCLK of instance: sai1"]
    ALT2_SAI1_RX_BCLK = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPIT1_TRIGGER00 of instance: lpit1"]
    ALT3_LPIT1_TRIGGER0 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO09 of instance: gpio1"]
    ALT5_GPIO1_IO9 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPI2C1_SCL of instance: lpi2c1"]
    ALT6_LPI2C1_SCL = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPSPI1_PCS2 of instance: lpspi1"]
    ALT8_LPSPI1_PCS2 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_LINK_ACT01 of instance: ecat"]
    ALT12_ECAT_LINK_ACT1 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAon09MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAon09MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAon09MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAon09MuxMode {
        SwMuxCtlPadGpioAon09MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAon09MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAon09MuxMode) -> u8 {
        SwMuxCtlPadGpioAon09MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAon10MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: JTAG_MUX_TRSTB of instance: jtag_mux"]
    ALT0_JTAG_MUX_TRSTB = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPSPI2_PCS0 of instance: lpspi2"]
    ALT1_LPSPI2_PCS0 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SAI1_RX_SYNC of instance: sai1"]
    ALT2_SAI1_RX_SYNC = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPIT1_TRIGGER01 of instance: lpit1"]
    ALT3_LPIT1_TRIGGER1 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: TPM2_EXTCLK of instance: tpm2"]
    ALT4_TPM2_EXTCLK = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO10 of instance: gpio1"]
    ALT5_GPIO1_IO10 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPI2C1_SCLS of instance: lpi2c1"]
    ALT6_LPI2C1_SCLS = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioAon10MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAon10MuxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAon10MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAon10MuxMode {
        SwMuxCtlPadGpioAon10MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAon10MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAon10MuxMode) -> u8 {
        SwMuxCtlPadGpioAon10MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAon11MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: JTAG_MUX_TDO of instance: jtag_mux"]
    ALT0_JTAG_MUX_TDO = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART1_CTS_B of instance: lpuart1"]
    ALT2_LPUART1_CTS_B = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPIT1_TRIGGER02 of instance: lpit1"]
    ALT3_LPIT1_TRIGGER2 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: TPM2_CH00 of instance: tpm2"]
    ALT4_TPM2_CH0 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO11 of instance: gpio1"]
    ALT5_GPIO1_IO11 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPI2C1_SDAS of instance: lpi2c1"]
    ALT6_LPI2C1_SDAS = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioAon11MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAon11MuxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAon11MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAon11MuxMode {
        SwMuxCtlPadGpioAon11MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAon11MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAon11MuxMode) -> u8 {
        SwMuxCtlPadGpioAon11MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAon12MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: JTAG_MUX_TDI of instance: jtag_mux"]
    ALT0_JTAG_MUX_TDI = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART1_RTS_B of instance: lpuart1"]
    ALT2_LPUART1_RTS_B = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPIT1_TRIGGER03 of instance: lpit1"]
    ALT3_LPIT1_TRIGGER3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: TPM2_CH01 of instance: tpm2"]
    ALT4_TPM2_CH1 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO12 of instance: gpio1"]
    ALT5_GPIO1_IO12 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPI2C1_HREQ of instance: lpi2c1"]
    ALT6_LPI2C1_HREQ = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPSPI1_SCK of instance: lpspi1"]
    ALT8_LPSPI1_SCK = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAon12MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAon12MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAon12MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAon12MuxMode {
        SwMuxCtlPadGpioAon12MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAon12MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAon12MuxMode) -> u8 {
        SwMuxCtlPadGpioAon12MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAon13MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: JTAG_MUX_TCK of instance: jtag_mux"]
    ALT0_JTAG_MUX_TCK = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART12_CTS_B of instance: lpuart12"]
    ALT2_LPUART12_CTS_B = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART1_DSR_B of instance: lpuart1"]
    ALT3_LPUART1_DSR_B = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: TPM2_CH02 of instance: tpm2"]
    ALT4_TPM2_CH2 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO13 of instance: gpio1"]
    ALT5_GPIO1_IO13 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPTMR1_ALT1 of instance: lptmr1"]
    ALT6_LPTMR1_ALT1 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPSPI1_PCS0 of instance: lpspi1"]
    ALT8_LPSPI1_PCS0 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAon13MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAon13MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAon13MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAon13MuxMode {
        SwMuxCtlPadGpioAon13MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAon13MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAon13MuxMode) -> u8 {
        SwMuxCtlPadGpioAon13MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAon14MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: JTAG_MUX_TMS of instance: jtag_mux"]
    ALT0_JTAG_MUX_TMS = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART12_RTS_B of instance: lpuart12"]
    ALT2_LPUART12_RTS_B = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART1_DCD_B of instance: lpuart1"]
    ALT3_LPUART1_DCD_B = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: TPM2_CH03 of instance: tpm2"]
    ALT4_TPM2_CH3 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO14 of instance: gpio1"]
    ALT5_GPIO1_IO14 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPTMR1_ALT2 of instance: lptmr1"]
    ALT6_LPTMR1_ALT2 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPSPI1_SDO of instance: lpspi1"]
    ALT8_LPSPI1_SDO = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAon14MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAon14MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAon14MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAon14MuxMode {
        SwMuxCtlPadGpioAon14MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAon14MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAon14MuxMode) -> u8 {
        SwMuxCtlPadGpioAon14MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAon15MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI2_BUS2BIT_B_DATA03 of instance: flexspi2_bus2bit"]
    ALT0_FLEXSPI2_BUS2BIT_B_DATA3 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPSPI2_PCS1 of instance: lpspi2"]
    ALT1_LPSPI2_PCS1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART12_TX of instance: lpuart12"]
    ALT2_LPUART12_TX = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART1_RI_B of instance: lpuart1"]
    ALT3_LPUART1_RI_B = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPI2C2_SDA of instance: lpi2c2"]
    ALT4_LPI2C2_SDA = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO15 of instance: gpio1"]
    ALT5_GPIO1_IO15 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPTMR1_ALT3 of instance: lptmr1"]
    ALT6_LPTMR1_ALT3 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPSPI1_SDI of instance: lpspi1"]
    ALT8_LPSPI1_SDI = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: I3C1_SDA of instance: i3c1"]
    ALT9_I3C1_SDA = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAon15MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAon15MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAon15MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAon15MuxMode {
        SwMuxCtlPadGpioAon15MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAon15MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAon15MuxMode) -> u8 {
        SwMuxCtlPadGpioAon15MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAon16MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI2_BUS2BIT_B_DATA02 of instance: flexspi2_bus2bit"]
    ALT0_FLEXSPI2_BUS2BIT_B_DATA2 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPSPI2_PCS0 of instance: lpspi2"]
    ALT1_LPSPI2_PCS0 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART12_RX of instance: lpuart12"]
    ALT2_LPUART12_RX = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART1_DTR_B of instance: lpuart1"]
    ALT3_LPUART1_DTR_B = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPI2C2_SCL of instance: lpi2c2"]
    ALT4_LPI2C2_SCL = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO16 of instance: gpio1"]
    ALT5_GPIO1_IO16 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: CAN1_TX of instance: can1"]
    ALT6_CAN1_TX = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPUART7_CTS_B of instance: lpuart7"]
    ALT8_LPUART7_CTS_B = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: I3C1_SCL of instance: i3c1"]
    ALT9_I3C1_SCL = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAon16MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAon16MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAon16MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAon16MuxMode {
        SwMuxCtlPadGpioAon16MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAon16MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAon16MuxMode) -> u8 {
        SwMuxCtlPadGpioAon16MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAon17MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI2_BUS2BIT_B_DATA01 of instance: flexspi2_bus2bit"]
    ALT0_FLEXSPI2_BUS2BIT_B_DATA1 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPSPI2_SDI of instance: lpspi2"]
    ALT1_LPSPI2_SDI = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART7_TX of instance: lpuart7"]
    ALT2_LPUART7_TX = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPI2C2_SDA of instance: lpi2c2"]
    ALT3_LPI2C2_SDA = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPUART1_DSR_B of instance: lpuart1"]
    ALT4_LPUART1_DSR_B = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO17 of instance: gpio1"]
    ALT5_GPIO1_IO17 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: CAN1_RX of instance: can1"]
    ALT6_CAN1_RX = 0x06,
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
impl SwMuxCtlPadGpioAon17MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAon17MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAon17MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAon17MuxMode {
        SwMuxCtlPadGpioAon17MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAon17MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAon17MuxMode) -> u8 {
        SwMuxCtlPadGpioAon17MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAon18MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI2_BUS2BIT_B_DATA00 of instance: flexspi2_bus2bit"]
    ALT0_FLEXSPI2_BUS2BIT_B_DATA0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPSPI2_SDO of instance: lpspi2"]
    ALT1_LPSPI2_SDO = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART7_RX of instance: lpuart7"]
    ALT2_LPUART7_RX = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPI2C2_SCL of instance: lpi2c2"]
    ALT3_LPI2C2_SCL = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPUART1_DCD_B of instance: lpuart1"]
    ALT4_LPUART1_DCD_B = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO18 of instance: gpio1"]
    ALT5_GPIO1_IO18 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: CAN3_TX of instance: can3"]
    ALT6_CAN3_TX = 0x06,
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
impl SwMuxCtlPadGpioAon18MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAon18MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAon18MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAon18MuxMode {
        SwMuxCtlPadGpioAon18MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAon18MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAon18MuxMode) -> u8 {
        SwMuxCtlPadGpioAon18MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAon19MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI2_BUS2BIT_B_SCLK of instance: flexspi2_bus2bit"]
    ALT0_FLEXSPI2_BUS2BIT_B_SCLK = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPSPI2_SCK of instance: lpspi2"]
    ALT1_LPSPI2_SCK = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: FLEXSPI2_BUS2BIT_A_SS1_B of instance: flexspi2_bus2bit"]
    ALT3_FLEXSPI2_BUS2BIT_A_SS1_B = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPUART1_CTS_B of instance: lpuart1"]
    ALT4_LPUART1_CTS_B = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO19 of instance: gpio1"]
    ALT5_GPIO1_IO19 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: CAN3_RX of instance: can3"]
    ALT6_CAN3_RX = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPUART7_RTS_B of instance: lpuart7"]
    ALT8_LPUART7_RTS_B = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPUART12_TX of instance: lpuart12"]
    ALT9_LPUART12_TX = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ADC1_CONV_D00 of instance: adc1"]
    ALT12_ADC1_CONV_D0 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAon19MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAon19MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAon19MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAon19MuxMode {
        SwMuxCtlPadGpioAon19MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAon19MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAon19MuxMode) -> u8 {
        SwMuxCtlPadGpioAon19MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAon20MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI2_BUS2BIT_B_DQS of instance: flexspi2_bus2bit"]
    ALT0_FLEXSPI2_BUS2BIT_B_DQS = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXSPI2_BUS2BIT_A_SS1_B of instance: flexspi2_bus2bit"]
    ALT1_FLEXSPI2_BUS2BIT_A_SS1_B = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPI2C1_SDA of instance: lpi2c1"]
    ALT2_LPI2C1_SDA = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: I3C1_SDA of instance: i3c1"]
    ALT3_I3C1_SDA = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPUART1_RTS_B of instance: lpuart1"]
    ALT4_LPUART1_RTS_B = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO20 of instance: gpio1"]
    ALT5_GPIO1_IO20 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPUART12_RX of instance: lpuart12"]
    ALT9_LPUART12_RX = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ADC1_CONV_D01 of instance: adc1"]
    ALT12_ADC1_CONV_D1 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAon20MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAon20MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAon20MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAon20MuxMode {
        SwMuxCtlPadGpioAon20MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAon20MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAon20MuxMode) -> u8 {
        SwMuxCtlPadGpioAon20MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAon21MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI2_BUS2BIT_B_SS0_B of instance: flexspi2_bus2bit"]
    ALT0_FLEXSPI2_BUS2BIT_B_SS0_B = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPSPI2_PCS1 of instance: lpspi2"]
    ALT1_LPSPI2_PCS1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPI2C1_SCL of instance: lpi2c1"]
    ALT2_LPI2C1_SCL = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: I3C1_SCL of instance: i3c1"]
    ALT3_I3C1_SCL = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: SAI1_TX_DATA00 of instance: sai1"]
    ALT4_SAI1_TX_DATA0 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO21 of instance: gpio1"]
    ALT5_GPIO1_IO21 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXSPI2_BUS2BIT_A_DQS of instance: flexspi2_bus2bit"]
    ALT8_FLEXSPI2_BUS2BIT_A_DQS = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: SAI1_RX_DATA01 of instance: sai1"]
    ALT9_SAI1_RX_DATA1 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ADC1_CONV_D02 of instance: adc1"]
    ALT12_ADC1_CONV_D2 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAon21MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAon21MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAon21MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAon21MuxMode {
        SwMuxCtlPadGpioAon21MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAon21MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAon21MuxMode) -> u8 {
        SwMuxCtlPadGpioAon21MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAon22MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI2_BUS2BIT_A_SS0_B of instance: flexspi2_bus2bit"]
    ALT0_FLEXSPI2_BUS2BIT_A_SS0_B = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPI2C2_SDA of instance: lpi2c2"]
    ALT1_LPI2C2_SDA = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART7_TX of instance: lpuart7"]
    ALT2_LPUART7_TX = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART12_CTS_B of instance: lpuart12"]
    ALT3_LPUART12_CTS_B = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: SAI1_TX_SYNC of instance: sai1"]
    ALT4_SAI1_TX_SYNC = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO22 of instance: gpio1"]
    ALT5_GPIO1_IO22 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPSPI2_SCK of instance: lpspi2"]
    ALT6_LPSPI2_SCK = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: CCMSRCGPC_CCMOBS1 of instance: ccmsrcgpc"]
    ALT10_CCMSRCGPCMIX_CCMOBS1 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ADC1_CONV_D03 of instance: adc1"]
    ALT12_ADC1_CONV_D3 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAon22MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAon22MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAon22MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAon22MuxMode {
        SwMuxCtlPadGpioAon22MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAon22MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAon22MuxMode) -> u8 {
        SwMuxCtlPadGpioAon22MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAon23MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI2_BUS2BIT_A_SCLK of instance: flexspi2_bus2bit"]
    ALT0_FLEXSPI2_BUS2BIT_A_SCLK = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPI2C2_SCL of instance: lpi2c2"]
    ALT1_LPI2C2_SCL = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART7_RX of instance: lpuart7"]
    ALT2_LPUART7_RX = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART12_RTS_B of instance: lpuart12"]
    ALT3_LPUART12_RTS_B = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: SAI1_TX_BCLK of instance: sai1"]
    ALT4_SAI1_TX_BCLK = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO23 of instance: gpio1"]
    ALT5_GPIO1_IO23 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPSPI2_SDO of instance: lpspi2"]
    ALT6_LPSPI2_SDO = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: CCMSRCGPC_CCMOBS2 of instance: ccmsrcgpc"]
    ALT10_CCMSRCGPCMIX_CCMOBS2 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ADC1_CONV_D04 of instance: adc1"]
    ALT12_ADC1_CONV_D4 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAon23MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAon23MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAon23MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAon23MuxMode {
        SwMuxCtlPadGpioAon23MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAon23MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAon23MuxMode) -> u8 {
        SwMuxCtlPadGpioAon23MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAon24MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI2_BUS2BIT_A_DATA00 of instance: flexspi2_bus2bit"]
    ALT0_FLEXSPI2_BUS2BIT_A_DATA0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPI2C1_SDA of instance: lpi2c1"]
    ALT1_LPI2C1_SDA = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART2_RTS_B of instance: lpuart2"]
    ALT2_LPUART2_RTS_B = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART7_CTS_B of instance: lpuart7"]
    ALT3_LPUART7_CTS_B = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: SAI1_MCLK of instance: sai1"]
    ALT4_SAI1_MCLK = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO24 of instance: gpio1"]
    ALT5_GPIO1_IO24 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPSPI2_SDI of instance: lpspi2"]
    ALT6_LPSPI2_SDI = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ADC1_CONV_D05 of instance: adc1"]
    ALT12_ADC1_CONV_D5 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAon24MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAon24MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAon24MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAon24MuxMode {
        SwMuxCtlPadGpioAon24MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAon24MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAon24MuxMode) -> u8 {
        SwMuxCtlPadGpioAon24MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAon25MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI2_BUS2BIT_A_DATA01 of instance: flexspi2_bus2bit"]
    ALT0_FLEXSPI2_BUS2BIT_A_DATA1 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPI2C1_SCL of instance: lpi2c1"]
    ALT1_LPI2C1_SCL = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART2_CTS_B of instance: lpuart2"]
    ALT2_LPUART2_CTS_B = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART7_RTS_B of instance: lpuart7"]
    ALT3_LPUART7_RTS_B = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: SAI1_RX_DATA00 of instance: sai1"]
    ALT4_SAI1_RX_DATA0 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO25 of instance: gpio1"]
    ALT5_GPIO1_IO25 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPSPI2_PCS0 of instance: lpspi2"]
    ALT6_LPSPI2_PCS0 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: SAI1_TX_DATA01 of instance: sai1"]
    ALT7_SAI1_TX_DATA1 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ADC1_CONV_D06 of instance: adc1"]
    ALT12_ADC1_CONV_D6 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAon25MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAon25MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAon25MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAon25MuxMode {
        SwMuxCtlPadGpioAon25MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAon25MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAon25MuxMode) -> u8 {
        SwMuxCtlPadGpioAon25MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAon26MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI2_BUS2BIT_A_DATA02 of instance: flexspi2_bus2bit"]
    ALT0_FLEXSPI2_BUS2BIT_A_DATA2 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPSPI2_PCS2 of instance: lpspi2"]
    ALT1_LPSPI2_PCS2 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART2_TX of instance: lpuart2"]
    ALT2_LPUART2_TX = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: SAI1_RX_BCLK of instance: sai1"]
    ALT4_SAI1_RX_BCLK = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO26 of instance: gpio1"]
    ALT5_GPIO1_IO26 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ADC1_CONV_D07 of instance: adc1"]
    ALT12_ADC1_CONV_D7 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAon26MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAon26MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAon26MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAon26MuxMode {
        SwMuxCtlPadGpioAon26MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAon26MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAon26MuxMode) -> u8 {
        SwMuxCtlPadGpioAon26MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAon27MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI2_BUS2BIT_A_DATA03 of instance: flexspi2_bus2bit"]
    ALT0_FLEXSPI2_BUS2BIT_A_DATA3 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPSPI2_PCS3 of instance: lpspi2"]
    ALT1_LPSPI2_PCS3 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART2_RX of instance: lpuart2"]
    ALT2_LPUART2_RX = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: SAI1_RX_SYNC of instance: sai1"]
    ALT4_SAI1_RX_SYNC = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO27 of instance: gpio1"]
    ALT5_GPIO1_IO27 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: EWM_EWM_OUT_B of instance: ewm"]
    ALT7_EWM_EWM_OUT_B = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ADC1_CONV_RDY_CLK of instance: adc1"]
    ALT12_ADC1_CONV_RDY_CLK = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAon27MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAon27MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAon27MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAon27MuxMode {
        SwMuxCtlPadGpioAon27MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAon27MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAon27MuxMode) -> u8 {
        SwMuxCtlPadGpioAon27MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAon28MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI2_BUS2BIT_A_DQS of instance: flexspi2_bus2bit"]
    ALT0_FLEXSPI2_BUS2BIT_A_DQS = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXSPI2_BUS2BIT_B_DQS of instance: flexspi2_bus2bit"]
    ALT1_FLEXSPI2_BUS2BIT_B_DQS = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO28 of instance: gpio1"]
    ALT5_GPIO1_IO28 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioAon28MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAon28MuxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAon28MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAon28MuxMode {
        SwMuxCtlPadGpioAon28MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAon28MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAon28MuxMode) -> u8 {
        SwMuxCtlPadGpioAon28MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon00Dse {
    #[doc = "normal driver"]
    DSE_0_NORMAL_DRIVER = 0x0,
    #[doc = "high driver"]
    DSE_1_HIGH_DRIVER = 0x01,
}
impl SwPadCtlPadGpioAon00Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon00Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon00Dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon00Dse {
        SwPadCtlPadGpioAon00Dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon00Dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon00Dse) -> u8 {
        SwPadCtlPadGpioAon00Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon00Ode {
    #[doc = "Disabled"]
    ODE_0_DISABLED = 0x0,
    #[doc = "Enabled"]
    ODE_1_ENABLED = 0x01,
}
impl SwPadCtlPadGpioAon00Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon00Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon00Ode {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon00Ode {
        SwPadCtlPadGpioAon00Ode::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon00Ode> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon00Ode) -> u8 {
        SwPadCtlPadGpioAon00Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon00Pue {
    #[doc = "Pull Disable, Highz"]
    PUE_0_PULL_DISABLE__HIGHZ = 0x0,
    #[doc = "Pull Enable"]
    PUE_1_PULL_ENABLE = 0x01,
}
impl SwPadCtlPadGpioAon00Pue {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon00Pue {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon00Pue {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon00Pue {
        SwPadCtlPadGpioAon00Pue::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon00Pue> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon00Pue) -> u8 {
        SwPadCtlPadGpioAon00Pue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon00Pus {
    #[doc = "Weak pull down"]
    PUS_0_WEAK_PULL_DOWN = 0x0,
    #[doc = "Weak pull up"]
    PUS_1_WEAK_PULL_UP = 0x01,
}
impl SwPadCtlPadGpioAon00Pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon00Pus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon00Pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon00Pus {
        SwPadCtlPadGpioAon00Pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon00Pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon00Pus) -> u8 {
        SwPadCtlPadGpioAon00Pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon00Sre {
    #[doc = "Fast Slew Rate"]
    SRE_0_FAST_SLEW_RATE = 0x0,
    #[doc = "Slow Slew Rate"]
    SRE_1_SLOW_SLEW_RATE = 0x01,
}
impl SwPadCtlPadGpioAon00Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon00Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon00Sre {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon00Sre {
        SwPadCtlPadGpioAon00Sre::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon00Sre> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon00Sre) -> u8 {
        SwPadCtlPadGpioAon00Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon01Dse {
    #[doc = "normal driver"]
    DSE_0_NORMAL_DRIVER = 0x0,
    #[doc = "high driver"]
    DSE_1_HIGH_DRIVER = 0x01,
}
impl SwPadCtlPadGpioAon01Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon01Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon01Dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon01Dse {
        SwPadCtlPadGpioAon01Dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon01Dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon01Dse) -> u8 {
        SwPadCtlPadGpioAon01Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon01Ode {
    #[doc = "Disabled"]
    ODE_0_DISABLED = 0x0,
    #[doc = "Enabled"]
    ODE_1_ENABLED = 0x01,
}
impl SwPadCtlPadGpioAon01Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon01Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon01Ode {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon01Ode {
        SwPadCtlPadGpioAon01Ode::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon01Ode> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon01Ode) -> u8 {
        SwPadCtlPadGpioAon01Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon01Pue {
    #[doc = "Pull Disable, Highz"]
    PUE_0_PULL_DISABLE__HIGHZ = 0x0,
    #[doc = "Pull Enable"]
    PUE_1_PULL_ENABLE = 0x01,
}
impl SwPadCtlPadGpioAon01Pue {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon01Pue {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon01Pue {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon01Pue {
        SwPadCtlPadGpioAon01Pue::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon01Pue> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon01Pue) -> u8 {
        SwPadCtlPadGpioAon01Pue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon01Pus {
    #[doc = "Weak pull down"]
    PUS_0_WEAK_PULL_DOWN = 0x0,
    #[doc = "Weak pull up"]
    PUS_1_WEAK_PULL_UP = 0x01,
}
impl SwPadCtlPadGpioAon01Pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon01Pus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon01Pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon01Pus {
        SwPadCtlPadGpioAon01Pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon01Pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon01Pus) -> u8 {
        SwPadCtlPadGpioAon01Pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon01Sre {
    #[doc = "Fast Slew Rate"]
    SRE_0_FAST_SLEW_RATE = 0x0,
    #[doc = "Slow Slew Rate"]
    SRE_1_SLOW_SLEW_RATE = 0x01,
}
impl SwPadCtlPadGpioAon01Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon01Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon01Sre {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon01Sre {
        SwPadCtlPadGpioAon01Sre::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon01Sre> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon01Sre) -> u8 {
        SwPadCtlPadGpioAon01Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon02Dse {
    #[doc = "normal driver"]
    DSE_0_NORMAL_DRIVER = 0x0,
    #[doc = "high driver"]
    DSE_1_HIGH_DRIVER = 0x01,
}
impl SwPadCtlPadGpioAon02Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon02Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon02Dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon02Dse {
        SwPadCtlPadGpioAon02Dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon02Dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon02Dse) -> u8 {
        SwPadCtlPadGpioAon02Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon02Ode {
    #[doc = "Disabled"]
    ODE_0_DISABLED = 0x0,
    #[doc = "Enabled"]
    ODE_1_ENABLED = 0x01,
}
impl SwPadCtlPadGpioAon02Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon02Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon02Ode {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon02Ode {
        SwPadCtlPadGpioAon02Ode::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon02Ode> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon02Ode) -> u8 {
        SwPadCtlPadGpioAon02Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon02Pue {
    #[doc = "Pull Disable, Highz"]
    PUE_0_PULL_DISABLE__HIGHZ = 0x0,
    #[doc = "Pull Enable"]
    PUE_1_PULL_ENABLE = 0x01,
}
impl SwPadCtlPadGpioAon02Pue {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon02Pue {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon02Pue {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon02Pue {
        SwPadCtlPadGpioAon02Pue::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon02Pue> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon02Pue) -> u8 {
        SwPadCtlPadGpioAon02Pue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon02Pus {
    #[doc = "Weak pull down"]
    PUS_0_WEAK_PULL_DOWN = 0x0,
    #[doc = "Weak pull up"]
    PUS_1_WEAK_PULL_UP = 0x01,
}
impl SwPadCtlPadGpioAon02Pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon02Pus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon02Pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon02Pus {
        SwPadCtlPadGpioAon02Pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon02Pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon02Pus) -> u8 {
        SwPadCtlPadGpioAon02Pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon02Sre {
    #[doc = "Fast Slew Rate"]
    SRE_0_FAST_SLEW_RATE = 0x0,
    #[doc = "Slow Slew Rate"]
    SRE_1_SLOW_SLEW_RATE = 0x01,
}
impl SwPadCtlPadGpioAon02Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon02Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon02Sre {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon02Sre {
        SwPadCtlPadGpioAon02Sre::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon02Sre> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon02Sre) -> u8 {
        SwPadCtlPadGpioAon02Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon03Dse {
    #[doc = "normal driver"]
    DSE_0_NORMAL_DRIVER = 0x0,
    #[doc = "high driver"]
    DSE_1_HIGH_DRIVER = 0x01,
}
impl SwPadCtlPadGpioAon03Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon03Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon03Dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon03Dse {
        SwPadCtlPadGpioAon03Dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon03Dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon03Dse) -> u8 {
        SwPadCtlPadGpioAon03Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon03Ode {
    #[doc = "Disabled"]
    ODE_0_DISABLED = 0x0,
    #[doc = "Enabled"]
    ODE_1_ENABLED = 0x01,
}
impl SwPadCtlPadGpioAon03Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon03Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon03Ode {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon03Ode {
        SwPadCtlPadGpioAon03Ode::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon03Ode> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon03Ode) -> u8 {
        SwPadCtlPadGpioAon03Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon03Pue {
    #[doc = "Pull Disable, Highz"]
    PUE_0_PULL_DISABLE__HIGHZ = 0x0,
    #[doc = "Pull Enable"]
    PUE_1_PULL_ENABLE = 0x01,
}
impl SwPadCtlPadGpioAon03Pue {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon03Pue {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon03Pue {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon03Pue {
        SwPadCtlPadGpioAon03Pue::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon03Pue> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon03Pue) -> u8 {
        SwPadCtlPadGpioAon03Pue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon03Pus {
    #[doc = "Weak pull down"]
    PUS_0_WEAK_PULL_DOWN = 0x0,
    #[doc = "Weak pull up"]
    PUS_1_WEAK_PULL_UP = 0x01,
}
impl SwPadCtlPadGpioAon03Pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon03Pus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon03Pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon03Pus {
        SwPadCtlPadGpioAon03Pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon03Pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon03Pus) -> u8 {
        SwPadCtlPadGpioAon03Pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon03Sre {
    #[doc = "Fast Slew Rate"]
    SRE_0_FAST_SLEW_RATE = 0x0,
    #[doc = "Slow Slew Rate"]
    SRE_1_SLOW_SLEW_RATE = 0x01,
}
impl SwPadCtlPadGpioAon03Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon03Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon03Sre {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon03Sre {
        SwPadCtlPadGpioAon03Sre::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon03Sre> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon03Sre) -> u8 {
        SwPadCtlPadGpioAon03Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon04Dse {
    #[doc = "normal driver"]
    DSE_0_NORMAL_DRIVER = 0x0,
    #[doc = "high driver"]
    DSE_1_HIGH_DRIVER = 0x01,
}
impl SwPadCtlPadGpioAon04Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon04Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon04Dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon04Dse {
        SwPadCtlPadGpioAon04Dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon04Dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon04Dse) -> u8 {
        SwPadCtlPadGpioAon04Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon04Ode {
    #[doc = "Disabled"]
    ODE_0_DISABLED = 0x0,
    #[doc = "Enabled"]
    ODE_1_ENABLED = 0x01,
}
impl SwPadCtlPadGpioAon04Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon04Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon04Ode {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon04Ode {
        SwPadCtlPadGpioAon04Ode::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon04Ode> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon04Ode) -> u8 {
        SwPadCtlPadGpioAon04Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon04Pue {
    #[doc = "Pull Disable, Highz"]
    PUE_0_PULL_DISABLE__HIGHZ = 0x0,
    #[doc = "Pull Enable"]
    PUE_1_PULL_ENABLE = 0x01,
}
impl SwPadCtlPadGpioAon04Pue {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon04Pue {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon04Pue {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon04Pue {
        SwPadCtlPadGpioAon04Pue::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon04Pue> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon04Pue) -> u8 {
        SwPadCtlPadGpioAon04Pue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon04Pus {
    #[doc = "Weak pull down"]
    PUS_0_WEAK_PULL_DOWN = 0x0,
    #[doc = "Weak pull up"]
    PUS_1_WEAK_PULL_UP = 0x01,
}
impl SwPadCtlPadGpioAon04Pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon04Pus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon04Pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon04Pus {
        SwPadCtlPadGpioAon04Pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon04Pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon04Pus) -> u8 {
        SwPadCtlPadGpioAon04Pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon04Sre {
    #[doc = "Fast Slew Rate"]
    SRE_0_FAST_SLEW_RATE = 0x0,
    #[doc = "Slow Slew Rate"]
    SRE_1_SLOW_SLEW_RATE = 0x01,
}
impl SwPadCtlPadGpioAon04Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon04Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon04Sre {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon04Sre {
        SwPadCtlPadGpioAon04Sre::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon04Sre> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon04Sre) -> u8 {
        SwPadCtlPadGpioAon04Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon05Dse {
    #[doc = "normal driver"]
    DSE_0_NORMAL_DRIVER = 0x0,
    #[doc = "high driver"]
    DSE_1_HIGH_DRIVER = 0x01,
}
impl SwPadCtlPadGpioAon05Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon05Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon05Dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon05Dse {
        SwPadCtlPadGpioAon05Dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon05Dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon05Dse) -> u8 {
        SwPadCtlPadGpioAon05Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon05Ode {
    #[doc = "Disabled"]
    ODE_0_DISABLED = 0x0,
    #[doc = "Enabled"]
    ODE_1_ENABLED = 0x01,
}
impl SwPadCtlPadGpioAon05Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon05Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon05Ode {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon05Ode {
        SwPadCtlPadGpioAon05Ode::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon05Ode> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon05Ode) -> u8 {
        SwPadCtlPadGpioAon05Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon05Pue {
    #[doc = "Pull Disable, Highz"]
    PUE_0_PULL_DISABLE__HIGHZ = 0x0,
    #[doc = "Pull Enable"]
    PUE_1_PULL_ENABLE = 0x01,
}
impl SwPadCtlPadGpioAon05Pue {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon05Pue {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon05Pue {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon05Pue {
        SwPadCtlPadGpioAon05Pue::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon05Pue> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon05Pue) -> u8 {
        SwPadCtlPadGpioAon05Pue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon05Pus {
    #[doc = "Weak pull down"]
    PUS_0_WEAK_PULL_DOWN = 0x0,
    #[doc = "Weak pull up"]
    PUS_1_WEAK_PULL_UP = 0x01,
}
impl SwPadCtlPadGpioAon05Pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon05Pus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon05Pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon05Pus {
        SwPadCtlPadGpioAon05Pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon05Pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon05Pus) -> u8 {
        SwPadCtlPadGpioAon05Pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon05Sre {
    #[doc = "Fast Slew Rate"]
    SRE_0_FAST_SLEW_RATE = 0x0,
    #[doc = "Slow Slew Rate"]
    SRE_1_SLOW_SLEW_RATE = 0x01,
}
impl SwPadCtlPadGpioAon05Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon05Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon05Sre {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon05Sre {
        SwPadCtlPadGpioAon05Sre::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon05Sre> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon05Sre) -> u8 {
        SwPadCtlPadGpioAon05Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon06Dse {
    #[doc = "normal driver"]
    DSE_0_NORMAL_DRIVER = 0x0,
    #[doc = "high driver"]
    DSE_1_HIGH_DRIVER = 0x01,
}
impl SwPadCtlPadGpioAon06Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon06Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon06Dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon06Dse {
        SwPadCtlPadGpioAon06Dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon06Dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon06Dse) -> u8 {
        SwPadCtlPadGpioAon06Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon06Ode {
    #[doc = "Disabled"]
    ODE_0_DISABLED = 0x0,
    #[doc = "Enabled"]
    ODE_1_ENABLED = 0x01,
}
impl SwPadCtlPadGpioAon06Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon06Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon06Ode {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon06Ode {
        SwPadCtlPadGpioAon06Ode::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon06Ode> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon06Ode) -> u8 {
        SwPadCtlPadGpioAon06Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon06Pue {
    #[doc = "Pull Disable, Highz"]
    PUE_0_PULL_DISABLE__HIGHZ = 0x0,
    #[doc = "Pull Enable"]
    PUE_1_PULL_ENABLE = 0x01,
}
impl SwPadCtlPadGpioAon06Pue {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon06Pue {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon06Pue {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon06Pue {
        SwPadCtlPadGpioAon06Pue::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon06Pue> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon06Pue) -> u8 {
        SwPadCtlPadGpioAon06Pue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon06Pus {
    #[doc = "Weak pull down"]
    PUS_0_WEAK_PULL_DOWN = 0x0,
    #[doc = "Weak pull up"]
    PUS_1_WEAK_PULL_UP = 0x01,
}
impl SwPadCtlPadGpioAon06Pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon06Pus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon06Pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon06Pus {
        SwPadCtlPadGpioAon06Pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon06Pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon06Pus) -> u8 {
        SwPadCtlPadGpioAon06Pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon06Sre {
    #[doc = "Fast Slew Rate"]
    SRE_0_FAST_SLEW_RATE = 0x0,
    #[doc = "Slow Slew Rate"]
    SRE_1_SLOW_SLEW_RATE = 0x01,
}
impl SwPadCtlPadGpioAon06Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon06Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon06Sre {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon06Sre {
        SwPadCtlPadGpioAon06Sre::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon06Sre> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon06Sre) -> u8 {
        SwPadCtlPadGpioAon06Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon07Dse {
    #[doc = "normal driver"]
    DSE_0_NORMAL_DRIVER = 0x0,
    #[doc = "high driver"]
    DSE_1_HIGH_DRIVER = 0x01,
}
impl SwPadCtlPadGpioAon07Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon07Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon07Dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon07Dse {
        SwPadCtlPadGpioAon07Dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon07Dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon07Dse) -> u8 {
        SwPadCtlPadGpioAon07Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon07Ode {
    #[doc = "Disabled"]
    ODE_0_DISABLED = 0x0,
    #[doc = "Enabled"]
    ODE_1_ENABLED = 0x01,
}
impl SwPadCtlPadGpioAon07Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon07Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon07Ode {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon07Ode {
        SwPadCtlPadGpioAon07Ode::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon07Ode> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon07Ode) -> u8 {
        SwPadCtlPadGpioAon07Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon07Pue {
    #[doc = "Pull Disable, Highz"]
    PUE_0_PULL_DISABLE__HIGHZ = 0x0,
    #[doc = "Pull Enable"]
    PUE_1_PULL_ENABLE = 0x01,
}
impl SwPadCtlPadGpioAon07Pue {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon07Pue {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon07Pue {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon07Pue {
        SwPadCtlPadGpioAon07Pue::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon07Pue> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon07Pue) -> u8 {
        SwPadCtlPadGpioAon07Pue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon07Pus {
    #[doc = "Weak pull down"]
    PUS_0_WEAK_PULL_DOWN = 0x0,
    #[doc = "Weak pull up"]
    PUS_1_WEAK_PULL_UP = 0x01,
}
impl SwPadCtlPadGpioAon07Pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon07Pus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon07Pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon07Pus {
        SwPadCtlPadGpioAon07Pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon07Pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon07Pus) -> u8 {
        SwPadCtlPadGpioAon07Pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon07Sre {
    #[doc = "Fast Slew Rate"]
    SRE_0_FAST_SLEW_RATE = 0x0,
    #[doc = "Slow Slew Rate"]
    SRE_1_SLOW_SLEW_RATE = 0x01,
}
impl SwPadCtlPadGpioAon07Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon07Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon07Sre {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon07Sre {
        SwPadCtlPadGpioAon07Sre::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon07Sre> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon07Sre) -> u8 {
        SwPadCtlPadGpioAon07Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon08Dse {
    #[doc = "normal driver"]
    DSE_0_NORMAL_DRIVER = 0x0,
    #[doc = "high driver"]
    DSE_1_HIGH_DRIVER = 0x01,
}
impl SwPadCtlPadGpioAon08Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon08Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon08Dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon08Dse {
        SwPadCtlPadGpioAon08Dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon08Dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon08Dse) -> u8 {
        SwPadCtlPadGpioAon08Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon08Ode {
    #[doc = "Disabled"]
    ODE_0_DISABLED = 0x0,
    #[doc = "Enabled"]
    ODE_1_ENABLED = 0x01,
}
impl SwPadCtlPadGpioAon08Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon08Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon08Ode {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon08Ode {
        SwPadCtlPadGpioAon08Ode::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon08Ode> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon08Ode) -> u8 {
        SwPadCtlPadGpioAon08Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon08Pue {
    #[doc = "Pull Disable, Highz"]
    PUE_0_PULL_DISABLE__HIGHZ = 0x0,
    #[doc = "Pull Enable"]
    PUE_1_PULL_ENABLE = 0x01,
}
impl SwPadCtlPadGpioAon08Pue {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon08Pue {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon08Pue {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon08Pue {
        SwPadCtlPadGpioAon08Pue::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon08Pue> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon08Pue) -> u8 {
        SwPadCtlPadGpioAon08Pue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon08Pus {
    #[doc = "Weak pull down"]
    PUS_0_WEAK_PULL_DOWN = 0x0,
    #[doc = "Weak pull up"]
    PUS_1_WEAK_PULL_UP = 0x01,
}
impl SwPadCtlPadGpioAon08Pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon08Pus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon08Pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon08Pus {
        SwPadCtlPadGpioAon08Pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon08Pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon08Pus) -> u8 {
        SwPadCtlPadGpioAon08Pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon08Sre {
    #[doc = "Fast Slew Rate"]
    SRE_0_FAST_SLEW_RATE = 0x0,
    #[doc = "Slow Slew Rate"]
    SRE_1_SLOW_SLEW_RATE = 0x01,
}
impl SwPadCtlPadGpioAon08Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon08Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon08Sre {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon08Sre {
        SwPadCtlPadGpioAon08Sre::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon08Sre> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon08Sre) -> u8 {
        SwPadCtlPadGpioAon08Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon09Dse {
    #[doc = "normal driver"]
    DSE_0_NORMAL_DRIVER = 0x0,
    #[doc = "high driver"]
    DSE_1_HIGH_DRIVER = 0x01,
}
impl SwPadCtlPadGpioAon09Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon09Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon09Dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon09Dse {
        SwPadCtlPadGpioAon09Dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon09Dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon09Dse) -> u8 {
        SwPadCtlPadGpioAon09Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon09Ode {
    #[doc = "Disabled"]
    ODE_0_DISABLED = 0x0,
    #[doc = "Enabled"]
    ODE_1_ENABLED = 0x01,
}
impl SwPadCtlPadGpioAon09Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon09Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon09Ode {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon09Ode {
        SwPadCtlPadGpioAon09Ode::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon09Ode> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon09Ode) -> u8 {
        SwPadCtlPadGpioAon09Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon09Pue {
    #[doc = "Pull Disable, Highz"]
    PUE_0_PULL_DISABLE__HIGHZ = 0x0,
    #[doc = "Pull Enable"]
    PUE_1_PULL_ENABLE = 0x01,
}
impl SwPadCtlPadGpioAon09Pue {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon09Pue {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon09Pue {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon09Pue {
        SwPadCtlPadGpioAon09Pue::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon09Pue> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon09Pue) -> u8 {
        SwPadCtlPadGpioAon09Pue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon09Pus {
    #[doc = "Weak pull down"]
    PUS_0_WEAK_PULL_DOWN = 0x0,
    #[doc = "Weak pull up"]
    PUS_1_WEAK_PULL_UP = 0x01,
}
impl SwPadCtlPadGpioAon09Pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon09Pus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon09Pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon09Pus {
        SwPadCtlPadGpioAon09Pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon09Pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon09Pus) -> u8 {
        SwPadCtlPadGpioAon09Pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon09Sre {
    #[doc = "Fast Slew Rate"]
    SRE_0_FAST_SLEW_RATE = 0x0,
    #[doc = "Slow Slew Rate"]
    SRE_1_SLOW_SLEW_RATE = 0x01,
}
impl SwPadCtlPadGpioAon09Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon09Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon09Sre {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon09Sre {
        SwPadCtlPadGpioAon09Sre::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon09Sre> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon09Sre) -> u8 {
        SwPadCtlPadGpioAon09Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon10Dse {
    #[doc = "normal driver"]
    DSE_0_NORMAL_DRIVER = 0x0,
    #[doc = "high driver"]
    DSE_1_HIGH_DRIVER = 0x01,
}
impl SwPadCtlPadGpioAon10Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon10Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon10Dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon10Dse {
        SwPadCtlPadGpioAon10Dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon10Dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon10Dse) -> u8 {
        SwPadCtlPadGpioAon10Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon10Ode {
    #[doc = "Disabled"]
    ODE_0_DISABLED = 0x0,
    #[doc = "Enabled"]
    ODE_1_ENABLED = 0x01,
}
impl SwPadCtlPadGpioAon10Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon10Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon10Ode {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon10Ode {
        SwPadCtlPadGpioAon10Ode::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon10Ode> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon10Ode) -> u8 {
        SwPadCtlPadGpioAon10Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon10Pue {
    #[doc = "Pull Disable, Highz"]
    PUE_0_PULL_DISABLE__HIGHZ = 0x0,
    #[doc = "Pull Enable"]
    PUE_1_PULL_ENABLE = 0x01,
}
impl SwPadCtlPadGpioAon10Pue {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon10Pue {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon10Pue {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon10Pue {
        SwPadCtlPadGpioAon10Pue::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon10Pue> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon10Pue) -> u8 {
        SwPadCtlPadGpioAon10Pue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon10Pus {
    #[doc = "Weak pull down"]
    PUS_0_WEAK_PULL_DOWN = 0x0,
    #[doc = "Weak pull up"]
    PUS_1_WEAK_PULL_UP = 0x01,
}
impl SwPadCtlPadGpioAon10Pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon10Pus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon10Pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon10Pus {
        SwPadCtlPadGpioAon10Pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon10Pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon10Pus) -> u8 {
        SwPadCtlPadGpioAon10Pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon10Sre {
    #[doc = "Fast Slew Rate"]
    SRE_0_FAST_SLEW_RATE = 0x0,
    #[doc = "Slow Slew Rate"]
    SRE_1_SLOW_SLEW_RATE = 0x01,
}
impl SwPadCtlPadGpioAon10Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon10Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon10Sre {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon10Sre {
        SwPadCtlPadGpioAon10Sre::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon10Sre> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon10Sre) -> u8 {
        SwPadCtlPadGpioAon10Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon11Dse {
    #[doc = "normal driver"]
    DSE_0_NORMAL_DRIVER = 0x0,
    #[doc = "high driver"]
    DSE_1_HIGH_DRIVER = 0x01,
}
impl SwPadCtlPadGpioAon11Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon11Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon11Dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon11Dse {
        SwPadCtlPadGpioAon11Dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon11Dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon11Dse) -> u8 {
        SwPadCtlPadGpioAon11Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon11Ode {
    #[doc = "Disabled"]
    ODE_0_DISABLED = 0x0,
    #[doc = "Enabled"]
    ODE_1_ENABLED = 0x01,
}
impl SwPadCtlPadGpioAon11Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon11Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon11Ode {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon11Ode {
        SwPadCtlPadGpioAon11Ode::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon11Ode> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon11Ode) -> u8 {
        SwPadCtlPadGpioAon11Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon11Pue {
    #[doc = "Pull Disable, Highz"]
    PUE_0_PULL_DISABLE__HIGHZ = 0x0,
    #[doc = "Pull Enable"]
    PUE_1_PULL_ENABLE = 0x01,
}
impl SwPadCtlPadGpioAon11Pue {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon11Pue {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon11Pue {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon11Pue {
        SwPadCtlPadGpioAon11Pue::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon11Pue> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon11Pue) -> u8 {
        SwPadCtlPadGpioAon11Pue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon11Pus {
    #[doc = "Weak pull down"]
    PUS_0_WEAK_PULL_DOWN = 0x0,
    #[doc = "Weak pull up"]
    PUS_1_WEAK_PULL_UP = 0x01,
}
impl SwPadCtlPadGpioAon11Pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon11Pus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon11Pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon11Pus {
        SwPadCtlPadGpioAon11Pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon11Pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon11Pus) -> u8 {
        SwPadCtlPadGpioAon11Pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon11Sre {
    #[doc = "Fast Slew Rate"]
    SRE_0_FAST_SLEW_RATE = 0x0,
    #[doc = "Slow Slew Rate"]
    SRE_1_SLOW_SLEW_RATE = 0x01,
}
impl SwPadCtlPadGpioAon11Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon11Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon11Sre {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon11Sre {
        SwPadCtlPadGpioAon11Sre::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon11Sre> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon11Sre) -> u8 {
        SwPadCtlPadGpioAon11Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon12Dse {
    #[doc = "normal driver"]
    DSE_0_NORMAL_DRIVER = 0x0,
    #[doc = "high driver"]
    DSE_1_HIGH_DRIVER = 0x01,
}
impl SwPadCtlPadGpioAon12Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon12Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon12Dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon12Dse {
        SwPadCtlPadGpioAon12Dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon12Dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon12Dse) -> u8 {
        SwPadCtlPadGpioAon12Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon12Ode {
    #[doc = "Disabled"]
    ODE_0_DISABLED = 0x0,
    #[doc = "Enabled"]
    ODE_1_ENABLED = 0x01,
}
impl SwPadCtlPadGpioAon12Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon12Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon12Ode {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon12Ode {
        SwPadCtlPadGpioAon12Ode::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon12Ode> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon12Ode) -> u8 {
        SwPadCtlPadGpioAon12Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon12Pue {
    #[doc = "Pull Disable, Highz"]
    PUE_0_PULL_DISABLE__HIGHZ = 0x0,
    #[doc = "Pull Enable"]
    PUE_1_PULL_ENABLE = 0x01,
}
impl SwPadCtlPadGpioAon12Pue {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon12Pue {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon12Pue {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon12Pue {
        SwPadCtlPadGpioAon12Pue::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon12Pue> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon12Pue) -> u8 {
        SwPadCtlPadGpioAon12Pue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon12Pus {
    #[doc = "Weak pull down"]
    PUS_0_WEAK_PULL_DOWN = 0x0,
    #[doc = "Weak pull up"]
    PUS_1_WEAK_PULL_UP = 0x01,
}
impl SwPadCtlPadGpioAon12Pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon12Pus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon12Pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon12Pus {
        SwPadCtlPadGpioAon12Pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon12Pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon12Pus) -> u8 {
        SwPadCtlPadGpioAon12Pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon12Sre {
    #[doc = "Fast Slew Rate"]
    SRE_0_FAST_SLEW_RATE = 0x0,
    #[doc = "Slow Slew Rate"]
    SRE_1_SLOW_SLEW_RATE = 0x01,
}
impl SwPadCtlPadGpioAon12Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon12Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon12Sre {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon12Sre {
        SwPadCtlPadGpioAon12Sre::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon12Sre> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon12Sre) -> u8 {
        SwPadCtlPadGpioAon12Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon13Dse {
    #[doc = "normal driver"]
    DSE_0_NORMAL_DRIVER = 0x0,
    #[doc = "high driver"]
    DSE_1_HIGH_DRIVER = 0x01,
}
impl SwPadCtlPadGpioAon13Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon13Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon13Dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon13Dse {
        SwPadCtlPadGpioAon13Dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon13Dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon13Dse) -> u8 {
        SwPadCtlPadGpioAon13Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon13Ode {
    #[doc = "Disabled"]
    ODE_0_DISABLED = 0x0,
    #[doc = "Enabled"]
    ODE_1_ENABLED = 0x01,
}
impl SwPadCtlPadGpioAon13Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon13Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon13Ode {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon13Ode {
        SwPadCtlPadGpioAon13Ode::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon13Ode> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon13Ode) -> u8 {
        SwPadCtlPadGpioAon13Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon13Pue {
    #[doc = "Pull Disable, Highz"]
    PUE_0_PULL_DISABLE__HIGHZ = 0x0,
    #[doc = "Pull Enable"]
    PUE_1_PULL_ENABLE = 0x01,
}
impl SwPadCtlPadGpioAon13Pue {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon13Pue {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon13Pue {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon13Pue {
        SwPadCtlPadGpioAon13Pue::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon13Pue> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon13Pue) -> u8 {
        SwPadCtlPadGpioAon13Pue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon13Pus {
    #[doc = "Weak pull down"]
    PUS_0_WEAK_PULL_DOWN = 0x0,
    #[doc = "Weak pull up"]
    PUS_1_WEAK_PULL_UP = 0x01,
}
impl SwPadCtlPadGpioAon13Pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon13Pus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon13Pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon13Pus {
        SwPadCtlPadGpioAon13Pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon13Pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon13Pus) -> u8 {
        SwPadCtlPadGpioAon13Pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon13Sre {
    #[doc = "Fast Slew Rate"]
    SRE_0_FAST_SLEW_RATE = 0x0,
    #[doc = "Slow Slew Rate"]
    SRE_1_SLOW_SLEW_RATE = 0x01,
}
impl SwPadCtlPadGpioAon13Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon13Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon13Sre {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon13Sre {
        SwPadCtlPadGpioAon13Sre::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon13Sre> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon13Sre) -> u8 {
        SwPadCtlPadGpioAon13Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon14Dse {
    #[doc = "normal driver"]
    DSE_0_NORMAL_DRIVER = 0x0,
    #[doc = "high driver"]
    DSE_1_HIGH_DRIVER = 0x01,
}
impl SwPadCtlPadGpioAon14Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon14Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon14Dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon14Dse {
        SwPadCtlPadGpioAon14Dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon14Dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon14Dse) -> u8 {
        SwPadCtlPadGpioAon14Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon14Ode {
    #[doc = "Disabled"]
    ODE_0_DISABLED = 0x0,
    #[doc = "Enabled"]
    ODE_1_ENABLED = 0x01,
}
impl SwPadCtlPadGpioAon14Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon14Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon14Ode {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon14Ode {
        SwPadCtlPadGpioAon14Ode::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon14Ode> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon14Ode) -> u8 {
        SwPadCtlPadGpioAon14Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon14Pue {
    #[doc = "Pull Disable, Highz"]
    PUE_0_PULL_DISABLE__HIGHZ = 0x0,
    #[doc = "Pull Enable"]
    PUE_1_PULL_ENABLE = 0x01,
}
impl SwPadCtlPadGpioAon14Pue {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon14Pue {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon14Pue {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon14Pue {
        SwPadCtlPadGpioAon14Pue::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon14Pue> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon14Pue) -> u8 {
        SwPadCtlPadGpioAon14Pue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon14Pus {
    #[doc = "Weak pull down"]
    PUS_0_WEAK_PULL_DOWN = 0x0,
    #[doc = "Weak pull up"]
    PUS_1_WEAK_PULL_UP = 0x01,
}
impl SwPadCtlPadGpioAon14Pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon14Pus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon14Pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon14Pus {
        SwPadCtlPadGpioAon14Pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon14Pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon14Pus) -> u8 {
        SwPadCtlPadGpioAon14Pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon14Sre {
    #[doc = "Fast Slew Rate"]
    SRE_0_FAST_SLEW_RATE = 0x0,
    #[doc = "Slow Slew Rate"]
    SRE_1_SLOW_SLEW_RATE = 0x01,
}
impl SwPadCtlPadGpioAon14Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon14Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon14Sre {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon14Sre {
        SwPadCtlPadGpioAon14Sre::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon14Sre> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon14Sre) -> u8 {
        SwPadCtlPadGpioAon14Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon15Dse {
    #[doc = "normal driver"]
    DSE_0_NORMAL_DRIVER = 0x0,
    #[doc = "high driver"]
    DSE_1_HIGH_DRIVER = 0x01,
}
impl SwPadCtlPadGpioAon15Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon15Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon15Dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon15Dse {
        SwPadCtlPadGpioAon15Dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon15Dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon15Dse) -> u8 {
        SwPadCtlPadGpioAon15Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon15Ode {
    #[doc = "Disabled"]
    ODE_0_DISABLED = 0x0,
    #[doc = "Enabled"]
    ODE_1_ENABLED = 0x01,
}
impl SwPadCtlPadGpioAon15Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon15Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon15Ode {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon15Ode {
        SwPadCtlPadGpioAon15Ode::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon15Ode> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon15Ode) -> u8 {
        SwPadCtlPadGpioAon15Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon15Pue {
    #[doc = "Pull Disable, Highz"]
    PUE_0_PULL_DISABLE__HIGHZ = 0x0,
    #[doc = "Pull Enable"]
    PUE_1_PULL_ENABLE = 0x01,
}
impl SwPadCtlPadGpioAon15Pue {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon15Pue {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon15Pue {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon15Pue {
        SwPadCtlPadGpioAon15Pue::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon15Pue> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon15Pue) -> u8 {
        SwPadCtlPadGpioAon15Pue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon15Pus {
    #[doc = "Weak pull down"]
    PUS_0_WEAK_PULL_DOWN = 0x0,
    #[doc = "Weak pull up"]
    PUS_1_WEAK_PULL_UP = 0x01,
}
impl SwPadCtlPadGpioAon15Pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon15Pus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon15Pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon15Pus {
        SwPadCtlPadGpioAon15Pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon15Pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon15Pus) -> u8 {
        SwPadCtlPadGpioAon15Pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon15Sre {
    #[doc = "Fast Slew Rate"]
    SRE_0_FAST_SLEW_RATE = 0x0,
    #[doc = "Slow Slew Rate"]
    SRE_1_SLOW_SLEW_RATE = 0x01,
}
impl SwPadCtlPadGpioAon15Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon15Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon15Sre {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon15Sre {
        SwPadCtlPadGpioAon15Sre::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon15Sre> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon15Sre) -> u8 {
        SwPadCtlPadGpioAon15Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon16Dse {
    #[doc = "normal driver"]
    DSE_0_NORMAL_DRIVER = 0x0,
    #[doc = "high driver"]
    DSE_1_HIGH_DRIVER = 0x01,
}
impl SwPadCtlPadGpioAon16Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon16Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon16Dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon16Dse {
        SwPadCtlPadGpioAon16Dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon16Dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon16Dse) -> u8 {
        SwPadCtlPadGpioAon16Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon16Ode {
    #[doc = "Disabled"]
    ODE_0_DISABLED = 0x0,
    #[doc = "Enabled"]
    ODE_1_ENABLED = 0x01,
}
impl SwPadCtlPadGpioAon16Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon16Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon16Ode {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon16Ode {
        SwPadCtlPadGpioAon16Ode::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon16Ode> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon16Ode) -> u8 {
        SwPadCtlPadGpioAon16Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon16Pue {
    #[doc = "Pull Disable, Highz"]
    PUE_0_PULL_DISABLE__HIGHZ = 0x0,
    #[doc = "Pull Enable"]
    PUE_1_PULL_ENABLE = 0x01,
}
impl SwPadCtlPadGpioAon16Pue {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon16Pue {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon16Pue {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon16Pue {
        SwPadCtlPadGpioAon16Pue::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon16Pue> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon16Pue) -> u8 {
        SwPadCtlPadGpioAon16Pue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon16Pus {
    #[doc = "Weak pull down"]
    PUS_0_WEAK_PULL_DOWN = 0x0,
    #[doc = "Weak pull up"]
    PUS_1_WEAK_PULL_UP = 0x01,
}
impl SwPadCtlPadGpioAon16Pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon16Pus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon16Pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon16Pus {
        SwPadCtlPadGpioAon16Pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon16Pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon16Pus) -> u8 {
        SwPadCtlPadGpioAon16Pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon16Sre {
    #[doc = "Fast Slew Rate"]
    SRE_0_FAST_SLEW_RATE = 0x0,
    #[doc = "Slow Slew Rate"]
    SRE_1_SLOW_SLEW_RATE = 0x01,
}
impl SwPadCtlPadGpioAon16Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon16Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon16Sre {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon16Sre {
        SwPadCtlPadGpioAon16Sre::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon16Sre> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon16Sre) -> u8 {
        SwPadCtlPadGpioAon16Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon17Dse {
    #[doc = "normal driver"]
    DSE_0_NORMAL_DRIVER = 0x0,
    #[doc = "high driver"]
    DSE_1_HIGH_DRIVER = 0x01,
}
impl SwPadCtlPadGpioAon17Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon17Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon17Dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon17Dse {
        SwPadCtlPadGpioAon17Dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon17Dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon17Dse) -> u8 {
        SwPadCtlPadGpioAon17Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon17Ode {
    #[doc = "Disabled"]
    ODE_0_DISABLED = 0x0,
    #[doc = "Enabled"]
    ODE_1_ENABLED = 0x01,
}
impl SwPadCtlPadGpioAon17Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon17Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon17Ode {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon17Ode {
        SwPadCtlPadGpioAon17Ode::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon17Ode> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon17Ode) -> u8 {
        SwPadCtlPadGpioAon17Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon17Pue {
    #[doc = "Pull Disable, Highz"]
    PUE_0_PULL_DISABLE__HIGHZ = 0x0,
    #[doc = "Pull Enable"]
    PUE_1_PULL_ENABLE = 0x01,
}
impl SwPadCtlPadGpioAon17Pue {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon17Pue {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon17Pue {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon17Pue {
        SwPadCtlPadGpioAon17Pue::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon17Pue> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon17Pue) -> u8 {
        SwPadCtlPadGpioAon17Pue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon17Pus {
    #[doc = "Weak pull down"]
    PUS_0_WEAK_PULL_DOWN = 0x0,
    #[doc = "Weak pull up"]
    PUS_1_WEAK_PULL_UP = 0x01,
}
impl SwPadCtlPadGpioAon17Pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon17Pus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon17Pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon17Pus {
        SwPadCtlPadGpioAon17Pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon17Pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon17Pus) -> u8 {
        SwPadCtlPadGpioAon17Pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon17Sre {
    #[doc = "Fast Slew Rate"]
    SRE_0_FAST_SLEW_RATE = 0x0,
    #[doc = "Slow Slew Rate"]
    SRE_1_SLOW_SLEW_RATE = 0x01,
}
impl SwPadCtlPadGpioAon17Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon17Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon17Sre {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon17Sre {
        SwPadCtlPadGpioAon17Sre::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon17Sre> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon17Sre) -> u8 {
        SwPadCtlPadGpioAon17Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon18Dse {
    #[doc = "normal driver"]
    DSE_0_NORMAL_DRIVER = 0x0,
    #[doc = "high driver"]
    DSE_1_HIGH_DRIVER = 0x01,
}
impl SwPadCtlPadGpioAon18Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon18Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon18Dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon18Dse {
        SwPadCtlPadGpioAon18Dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon18Dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon18Dse) -> u8 {
        SwPadCtlPadGpioAon18Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon18Ode {
    #[doc = "Disabled"]
    ODE_0_DISABLED = 0x0,
    #[doc = "Enabled"]
    ODE_1_ENABLED = 0x01,
}
impl SwPadCtlPadGpioAon18Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon18Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon18Ode {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon18Ode {
        SwPadCtlPadGpioAon18Ode::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon18Ode> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon18Ode) -> u8 {
        SwPadCtlPadGpioAon18Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon18Pue {
    #[doc = "Pull Disable, Highz"]
    PUE_0_PULL_DISABLE__HIGHZ = 0x0,
    #[doc = "Pull Enable"]
    PUE_1_PULL_ENABLE = 0x01,
}
impl SwPadCtlPadGpioAon18Pue {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon18Pue {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon18Pue {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon18Pue {
        SwPadCtlPadGpioAon18Pue::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon18Pue> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon18Pue) -> u8 {
        SwPadCtlPadGpioAon18Pue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon18Pus {
    #[doc = "Weak pull down"]
    PUS_0_WEAK_PULL_DOWN = 0x0,
    #[doc = "Weak pull up"]
    PUS_1_WEAK_PULL_UP = 0x01,
}
impl SwPadCtlPadGpioAon18Pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon18Pus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon18Pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon18Pus {
        SwPadCtlPadGpioAon18Pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon18Pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon18Pus) -> u8 {
        SwPadCtlPadGpioAon18Pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon18Sre {
    #[doc = "Fast Slew Rate"]
    SRE_0_FAST_SLEW_RATE = 0x0,
    #[doc = "Slow Slew Rate"]
    SRE_1_SLOW_SLEW_RATE = 0x01,
}
impl SwPadCtlPadGpioAon18Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon18Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon18Sre {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon18Sre {
        SwPadCtlPadGpioAon18Sre::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon18Sre> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon18Sre) -> u8 {
        SwPadCtlPadGpioAon18Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon19Dse {
    #[doc = "normal driver"]
    DSE_0_NORMAL_DRIVER = 0x0,
    #[doc = "high driver"]
    DSE_1_HIGH_DRIVER = 0x01,
}
impl SwPadCtlPadGpioAon19Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon19Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon19Dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon19Dse {
        SwPadCtlPadGpioAon19Dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon19Dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon19Dse) -> u8 {
        SwPadCtlPadGpioAon19Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon19Ode {
    #[doc = "Disabled"]
    ODE_0_DISABLED = 0x0,
    #[doc = "Enabled"]
    ODE_1_ENABLED = 0x01,
}
impl SwPadCtlPadGpioAon19Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon19Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon19Ode {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon19Ode {
        SwPadCtlPadGpioAon19Ode::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon19Ode> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon19Ode) -> u8 {
        SwPadCtlPadGpioAon19Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon19Pue {
    #[doc = "Pull Disable, Highz"]
    PUE_0_PULL_DISABLE__HIGHZ = 0x0,
    #[doc = "Pull Enable"]
    PUE_1_PULL_ENABLE = 0x01,
}
impl SwPadCtlPadGpioAon19Pue {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon19Pue {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon19Pue {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon19Pue {
        SwPadCtlPadGpioAon19Pue::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon19Pue> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon19Pue) -> u8 {
        SwPadCtlPadGpioAon19Pue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon19Pus {
    #[doc = "Weak pull down"]
    PUS_0_WEAK_PULL_DOWN = 0x0,
    #[doc = "Weak pull up"]
    PUS_1_WEAK_PULL_UP = 0x01,
}
impl SwPadCtlPadGpioAon19Pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon19Pus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon19Pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon19Pus {
        SwPadCtlPadGpioAon19Pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon19Pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon19Pus) -> u8 {
        SwPadCtlPadGpioAon19Pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon19Sre {
    #[doc = "Fast Slew Rate"]
    SRE_0_FAST_SLEW_RATE = 0x0,
    #[doc = "Slow Slew Rate"]
    SRE_1_SLOW_SLEW_RATE = 0x01,
}
impl SwPadCtlPadGpioAon19Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon19Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon19Sre {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon19Sre {
        SwPadCtlPadGpioAon19Sre::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon19Sre> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon19Sre) -> u8 {
        SwPadCtlPadGpioAon19Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon20Dse {
    #[doc = "normal driver"]
    DSE_0_NORMAL_DRIVER = 0x0,
    #[doc = "high driver"]
    DSE_1_HIGH_DRIVER = 0x01,
}
impl SwPadCtlPadGpioAon20Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon20Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon20Dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon20Dse {
        SwPadCtlPadGpioAon20Dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon20Dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon20Dse) -> u8 {
        SwPadCtlPadGpioAon20Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon20Ode {
    #[doc = "Disabled"]
    ODE_0_DISABLED = 0x0,
    #[doc = "Enabled"]
    ODE_1_ENABLED = 0x01,
}
impl SwPadCtlPadGpioAon20Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon20Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon20Ode {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon20Ode {
        SwPadCtlPadGpioAon20Ode::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon20Ode> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon20Ode) -> u8 {
        SwPadCtlPadGpioAon20Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon20Pue {
    #[doc = "Pull Disable, Highz"]
    PUE_0_PULL_DISABLE__HIGHZ = 0x0,
    #[doc = "Pull Enable"]
    PUE_1_PULL_ENABLE = 0x01,
}
impl SwPadCtlPadGpioAon20Pue {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon20Pue {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon20Pue {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon20Pue {
        SwPadCtlPadGpioAon20Pue::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon20Pue> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon20Pue) -> u8 {
        SwPadCtlPadGpioAon20Pue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon20Pus {
    #[doc = "Weak pull down"]
    PUS_0_WEAK_PULL_DOWN = 0x0,
    #[doc = "Weak pull up"]
    PUS_1_WEAK_PULL_UP = 0x01,
}
impl SwPadCtlPadGpioAon20Pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon20Pus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon20Pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon20Pus {
        SwPadCtlPadGpioAon20Pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon20Pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon20Pus) -> u8 {
        SwPadCtlPadGpioAon20Pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon20Sre {
    #[doc = "Fast Slew Rate"]
    SRE_0_FAST_SLEW_RATE = 0x0,
    #[doc = "Slow Slew Rate"]
    SRE_1_SLOW_SLEW_RATE = 0x01,
}
impl SwPadCtlPadGpioAon20Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon20Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon20Sre {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon20Sre {
        SwPadCtlPadGpioAon20Sre::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon20Sre> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon20Sre) -> u8 {
        SwPadCtlPadGpioAon20Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon21Dse {
    #[doc = "normal driver"]
    DSE_0_NORMAL_DRIVER = 0x0,
    #[doc = "high driver"]
    DSE_1_HIGH_DRIVER = 0x01,
}
impl SwPadCtlPadGpioAon21Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon21Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon21Dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon21Dse {
        SwPadCtlPadGpioAon21Dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon21Dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon21Dse) -> u8 {
        SwPadCtlPadGpioAon21Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon21Ode {
    #[doc = "Disabled"]
    ODE_0_DISABLED = 0x0,
    #[doc = "Enabled"]
    ODE_1_ENABLED = 0x01,
}
impl SwPadCtlPadGpioAon21Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon21Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon21Ode {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon21Ode {
        SwPadCtlPadGpioAon21Ode::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon21Ode> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon21Ode) -> u8 {
        SwPadCtlPadGpioAon21Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon21Pue {
    #[doc = "Pull Disable, Highz"]
    PUE_0_PULL_DISABLE__HIGHZ = 0x0,
    #[doc = "Pull Enable"]
    PUE_1_PULL_ENABLE = 0x01,
}
impl SwPadCtlPadGpioAon21Pue {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon21Pue {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon21Pue {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon21Pue {
        SwPadCtlPadGpioAon21Pue::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon21Pue> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon21Pue) -> u8 {
        SwPadCtlPadGpioAon21Pue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon21Pus {
    #[doc = "Weak pull down"]
    PUS_0_WEAK_PULL_DOWN = 0x0,
    #[doc = "Weak pull up"]
    PUS_1_WEAK_PULL_UP = 0x01,
}
impl SwPadCtlPadGpioAon21Pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon21Pus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon21Pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon21Pus {
        SwPadCtlPadGpioAon21Pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon21Pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon21Pus) -> u8 {
        SwPadCtlPadGpioAon21Pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon21Sre {
    #[doc = "Fast Slew Rate"]
    SRE_0_FAST_SLEW_RATE = 0x0,
    #[doc = "Slow Slew Rate"]
    SRE_1_SLOW_SLEW_RATE = 0x01,
}
impl SwPadCtlPadGpioAon21Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon21Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon21Sre {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon21Sre {
        SwPadCtlPadGpioAon21Sre::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon21Sre> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon21Sre) -> u8 {
        SwPadCtlPadGpioAon21Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon22Dse {
    #[doc = "normal driver"]
    DSE_0_NORMAL_DRIVER = 0x0,
    #[doc = "high driver"]
    DSE_1_HIGH_DRIVER = 0x01,
}
impl SwPadCtlPadGpioAon22Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon22Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon22Dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon22Dse {
        SwPadCtlPadGpioAon22Dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon22Dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon22Dse) -> u8 {
        SwPadCtlPadGpioAon22Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon22Ode {
    #[doc = "Disabled"]
    ODE_0_DISABLED = 0x0,
    #[doc = "Enabled"]
    ODE_1_ENABLED = 0x01,
}
impl SwPadCtlPadGpioAon22Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon22Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon22Ode {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon22Ode {
        SwPadCtlPadGpioAon22Ode::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon22Ode> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon22Ode) -> u8 {
        SwPadCtlPadGpioAon22Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon22Pue {
    #[doc = "Pull Disable, Highz"]
    PUE_0_PULL_DISABLE__HIGHZ = 0x0,
    #[doc = "Pull Enable"]
    PUE_1_PULL_ENABLE = 0x01,
}
impl SwPadCtlPadGpioAon22Pue {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon22Pue {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon22Pue {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon22Pue {
        SwPadCtlPadGpioAon22Pue::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon22Pue> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon22Pue) -> u8 {
        SwPadCtlPadGpioAon22Pue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon22Pus {
    #[doc = "Weak pull down"]
    PUS_0_WEAK_PULL_DOWN = 0x0,
    #[doc = "Weak pull up"]
    PUS_1_WEAK_PULL_UP = 0x01,
}
impl SwPadCtlPadGpioAon22Pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon22Pus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon22Pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon22Pus {
        SwPadCtlPadGpioAon22Pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon22Pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon22Pus) -> u8 {
        SwPadCtlPadGpioAon22Pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon22Sre {
    #[doc = "Fast Slew Rate"]
    SRE_0_FAST_SLEW_RATE = 0x0,
    #[doc = "Slow Slew Rate"]
    SRE_1_SLOW_SLEW_RATE = 0x01,
}
impl SwPadCtlPadGpioAon22Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon22Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon22Sre {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon22Sre {
        SwPadCtlPadGpioAon22Sre::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon22Sre> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon22Sre) -> u8 {
        SwPadCtlPadGpioAon22Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon23Dse {
    #[doc = "normal driver"]
    DSE_0_NORMAL_DRIVER = 0x0,
    #[doc = "high driver"]
    DSE_1_HIGH_DRIVER = 0x01,
}
impl SwPadCtlPadGpioAon23Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon23Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon23Dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon23Dse {
        SwPadCtlPadGpioAon23Dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon23Dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon23Dse) -> u8 {
        SwPadCtlPadGpioAon23Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon23Ode {
    #[doc = "Disabled"]
    ODE_0_DISABLED = 0x0,
    #[doc = "Enabled"]
    ODE_1_ENABLED = 0x01,
}
impl SwPadCtlPadGpioAon23Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon23Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon23Ode {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon23Ode {
        SwPadCtlPadGpioAon23Ode::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon23Ode> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon23Ode) -> u8 {
        SwPadCtlPadGpioAon23Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon23Pue {
    #[doc = "Pull Disable, Highz"]
    PUE_0_PULL_DISABLE__HIGHZ = 0x0,
    #[doc = "Pull Enable"]
    PUE_1_PULL_ENABLE = 0x01,
}
impl SwPadCtlPadGpioAon23Pue {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon23Pue {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon23Pue {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon23Pue {
        SwPadCtlPadGpioAon23Pue::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon23Pue> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon23Pue) -> u8 {
        SwPadCtlPadGpioAon23Pue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon23Pus {
    #[doc = "Weak pull down"]
    PUS_0_WEAK_PULL_DOWN = 0x0,
    #[doc = "Weak pull up"]
    PUS_1_WEAK_PULL_UP = 0x01,
}
impl SwPadCtlPadGpioAon23Pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon23Pus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon23Pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon23Pus {
        SwPadCtlPadGpioAon23Pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon23Pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon23Pus) -> u8 {
        SwPadCtlPadGpioAon23Pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon23Sre {
    #[doc = "Fast Slew Rate"]
    SRE_0_FAST_SLEW_RATE = 0x0,
    #[doc = "Slow Slew Rate"]
    SRE_1_SLOW_SLEW_RATE = 0x01,
}
impl SwPadCtlPadGpioAon23Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon23Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon23Sre {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon23Sre {
        SwPadCtlPadGpioAon23Sre::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon23Sre> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon23Sre) -> u8 {
        SwPadCtlPadGpioAon23Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon24Dse {
    #[doc = "normal driver"]
    DSE_0_NORMAL_DRIVER = 0x0,
    #[doc = "high driver"]
    DSE_1_HIGH_DRIVER = 0x01,
}
impl SwPadCtlPadGpioAon24Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon24Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon24Dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon24Dse {
        SwPadCtlPadGpioAon24Dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon24Dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon24Dse) -> u8 {
        SwPadCtlPadGpioAon24Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon24Ode {
    #[doc = "Disabled"]
    ODE_0_DISABLED = 0x0,
    #[doc = "Enabled"]
    ODE_1_ENABLED = 0x01,
}
impl SwPadCtlPadGpioAon24Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon24Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon24Ode {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon24Ode {
        SwPadCtlPadGpioAon24Ode::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon24Ode> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon24Ode) -> u8 {
        SwPadCtlPadGpioAon24Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon24Pue {
    #[doc = "Pull Disable, Highz"]
    PUE_0_PULL_DISABLE__HIGHZ = 0x0,
    #[doc = "Pull Enable"]
    PUE_1_PULL_ENABLE = 0x01,
}
impl SwPadCtlPadGpioAon24Pue {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon24Pue {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon24Pue {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon24Pue {
        SwPadCtlPadGpioAon24Pue::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon24Pue> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon24Pue) -> u8 {
        SwPadCtlPadGpioAon24Pue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon24Pus {
    #[doc = "Weak pull down"]
    PUS_0_WEAK_PULL_DOWN = 0x0,
    #[doc = "Weak pull up"]
    PUS_1_WEAK_PULL_UP = 0x01,
}
impl SwPadCtlPadGpioAon24Pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon24Pus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon24Pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon24Pus {
        SwPadCtlPadGpioAon24Pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon24Pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon24Pus) -> u8 {
        SwPadCtlPadGpioAon24Pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon24Sre {
    #[doc = "Fast Slew Rate"]
    SRE_0_FAST_SLEW_RATE = 0x0,
    #[doc = "Slow Slew Rate"]
    SRE_1_SLOW_SLEW_RATE = 0x01,
}
impl SwPadCtlPadGpioAon24Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon24Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon24Sre {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon24Sre {
        SwPadCtlPadGpioAon24Sre::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon24Sre> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon24Sre) -> u8 {
        SwPadCtlPadGpioAon24Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon25Dse {
    #[doc = "normal driver"]
    DSE_0_NORMAL_DRIVER = 0x0,
    #[doc = "high driver"]
    DSE_1_HIGH_DRIVER = 0x01,
}
impl SwPadCtlPadGpioAon25Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon25Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon25Dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon25Dse {
        SwPadCtlPadGpioAon25Dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon25Dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon25Dse) -> u8 {
        SwPadCtlPadGpioAon25Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon25Ode {
    #[doc = "Disabled"]
    ODE_0_DISABLED = 0x0,
    #[doc = "Enabled"]
    ODE_1_ENABLED = 0x01,
}
impl SwPadCtlPadGpioAon25Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon25Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon25Ode {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon25Ode {
        SwPadCtlPadGpioAon25Ode::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon25Ode> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon25Ode) -> u8 {
        SwPadCtlPadGpioAon25Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon25Pue {
    #[doc = "Pull Disable, Highz"]
    PUE_0_PULL_DISABLE__HIGHZ = 0x0,
    #[doc = "Pull Enable"]
    PUE_1_PULL_ENABLE = 0x01,
}
impl SwPadCtlPadGpioAon25Pue {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon25Pue {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon25Pue {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon25Pue {
        SwPadCtlPadGpioAon25Pue::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon25Pue> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon25Pue) -> u8 {
        SwPadCtlPadGpioAon25Pue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon25Pus {
    #[doc = "Weak pull down"]
    PUS_0_WEAK_PULL_DOWN = 0x0,
    #[doc = "Weak pull up"]
    PUS_1_WEAK_PULL_UP = 0x01,
}
impl SwPadCtlPadGpioAon25Pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon25Pus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon25Pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon25Pus {
        SwPadCtlPadGpioAon25Pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon25Pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon25Pus) -> u8 {
        SwPadCtlPadGpioAon25Pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon25Sre {
    #[doc = "Fast Slew Rate"]
    SRE_0_FAST_SLEW_RATE = 0x0,
    #[doc = "Slow Slew Rate"]
    SRE_1_SLOW_SLEW_RATE = 0x01,
}
impl SwPadCtlPadGpioAon25Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon25Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon25Sre {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon25Sre {
        SwPadCtlPadGpioAon25Sre::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon25Sre> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon25Sre) -> u8 {
        SwPadCtlPadGpioAon25Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon26Dse {
    #[doc = "normal driver"]
    DSE_0_NORMAL_DRIVER = 0x0,
    #[doc = "high driver"]
    DSE_1_HIGH_DRIVER = 0x01,
}
impl SwPadCtlPadGpioAon26Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon26Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon26Dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon26Dse {
        SwPadCtlPadGpioAon26Dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon26Dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon26Dse) -> u8 {
        SwPadCtlPadGpioAon26Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon26Ode {
    #[doc = "Disabled"]
    ODE_0_DISABLED = 0x0,
    #[doc = "Enabled"]
    ODE_1_ENABLED = 0x01,
}
impl SwPadCtlPadGpioAon26Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon26Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon26Ode {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon26Ode {
        SwPadCtlPadGpioAon26Ode::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon26Ode> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon26Ode) -> u8 {
        SwPadCtlPadGpioAon26Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon26Pue {
    #[doc = "Pull Disable, Highz"]
    PUE_0_PULL_DISABLE__HIGHZ = 0x0,
    #[doc = "Pull Enable"]
    PUE_1_PULL_ENABLE = 0x01,
}
impl SwPadCtlPadGpioAon26Pue {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon26Pue {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon26Pue {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon26Pue {
        SwPadCtlPadGpioAon26Pue::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon26Pue> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon26Pue) -> u8 {
        SwPadCtlPadGpioAon26Pue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon26Pus {
    #[doc = "Weak pull down"]
    PUS_0_WEAK_PULL_DOWN = 0x0,
    #[doc = "Weak pull up"]
    PUS_1_WEAK_PULL_UP = 0x01,
}
impl SwPadCtlPadGpioAon26Pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon26Pus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon26Pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon26Pus {
        SwPadCtlPadGpioAon26Pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon26Pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon26Pus) -> u8 {
        SwPadCtlPadGpioAon26Pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon26Sre {
    #[doc = "Fast Slew Rate"]
    SRE_0_FAST_SLEW_RATE = 0x0,
    #[doc = "Slow Slew Rate"]
    SRE_1_SLOW_SLEW_RATE = 0x01,
}
impl SwPadCtlPadGpioAon26Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon26Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon26Sre {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon26Sre {
        SwPadCtlPadGpioAon26Sre::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon26Sre> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon26Sre) -> u8 {
        SwPadCtlPadGpioAon26Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon27Dse {
    #[doc = "normal driver"]
    DSE_0_NORMAL_DRIVER = 0x0,
    #[doc = "high driver"]
    DSE_1_HIGH_DRIVER = 0x01,
}
impl SwPadCtlPadGpioAon27Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon27Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon27Dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon27Dse {
        SwPadCtlPadGpioAon27Dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon27Dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon27Dse) -> u8 {
        SwPadCtlPadGpioAon27Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon27Ode {
    #[doc = "Disabled"]
    ODE_0_DISABLED = 0x0,
    #[doc = "Enabled"]
    ODE_1_ENABLED = 0x01,
}
impl SwPadCtlPadGpioAon27Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon27Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon27Ode {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon27Ode {
        SwPadCtlPadGpioAon27Ode::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon27Ode> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon27Ode) -> u8 {
        SwPadCtlPadGpioAon27Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon27Pue {
    #[doc = "Pull Disable, Highz"]
    PUE_0_PULL_DISABLE__HIGHZ = 0x0,
    #[doc = "Pull Enable"]
    PUE_1_PULL_ENABLE = 0x01,
}
impl SwPadCtlPadGpioAon27Pue {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon27Pue {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon27Pue {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon27Pue {
        SwPadCtlPadGpioAon27Pue::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon27Pue> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon27Pue) -> u8 {
        SwPadCtlPadGpioAon27Pue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon27Pus {
    #[doc = "Weak pull down"]
    PUS_0_WEAK_PULL_DOWN = 0x0,
    #[doc = "Weak pull up"]
    PUS_1_WEAK_PULL_UP = 0x01,
}
impl SwPadCtlPadGpioAon27Pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon27Pus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon27Pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon27Pus {
        SwPadCtlPadGpioAon27Pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon27Pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon27Pus) -> u8 {
        SwPadCtlPadGpioAon27Pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon27Sre {
    #[doc = "Fast Slew Rate"]
    SRE_0_FAST_SLEW_RATE = 0x0,
    #[doc = "Slow Slew Rate"]
    SRE_1_SLOW_SLEW_RATE = 0x01,
}
impl SwPadCtlPadGpioAon27Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon27Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon27Sre {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon27Sre {
        SwPadCtlPadGpioAon27Sre::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon27Sre> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon27Sre) -> u8 {
        SwPadCtlPadGpioAon27Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon28Dse {
    #[doc = "normal driver"]
    DSE_0_NORMAL_DRIVER = 0x0,
    #[doc = "high driver"]
    DSE_1_HIGH_DRIVER = 0x01,
}
impl SwPadCtlPadGpioAon28Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon28Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon28Dse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon28Dse {
        SwPadCtlPadGpioAon28Dse::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon28Dse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon28Dse) -> u8 {
        SwPadCtlPadGpioAon28Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon28Ode {
    #[doc = "Disabled"]
    ODE_0_DISABLED = 0x0,
    #[doc = "Enabled"]
    ODE_1_ENABLED = 0x01,
}
impl SwPadCtlPadGpioAon28Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon28Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon28Ode {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon28Ode {
        SwPadCtlPadGpioAon28Ode::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon28Ode> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon28Ode) -> u8 {
        SwPadCtlPadGpioAon28Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon28Pue {
    #[doc = "Pull Disable, Highz"]
    PUE_0_PULL_DISABLE__HIGHZ = 0x0,
    #[doc = "Pull Enable"]
    PUE_1_PULL_ENABLE = 0x01,
}
impl SwPadCtlPadGpioAon28Pue {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon28Pue {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon28Pue {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon28Pue {
        SwPadCtlPadGpioAon28Pue::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon28Pue> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon28Pue) -> u8 {
        SwPadCtlPadGpioAon28Pue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon28Pus {
    #[doc = "Weak pull down"]
    PUS_0_WEAK_PULL_DOWN = 0x0,
    #[doc = "Weak pull up"]
    PUS_1_WEAK_PULL_UP = 0x01,
}
impl SwPadCtlPadGpioAon28Pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon28Pus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon28Pus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon28Pus {
        SwPadCtlPadGpioAon28Pus::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon28Pus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon28Pus) -> u8 {
        SwPadCtlPadGpioAon28Pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadGpioAon28Sre {
    #[doc = "Fast Slew Rate"]
    SRE_0_FAST_SLEW_RATE = 0x0,
    #[doc = "Slow Slew Rate"]
    SRE_1_SLOW_SLEW_RATE = 0x01,
}
impl SwPadCtlPadGpioAon28Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadGpioAon28Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadGpioAon28Sre {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadGpioAon28Sre {
        SwPadCtlPadGpioAon28Sre::from_bits(val)
    }
}
impl From<SwPadCtlPadGpioAon28Sre> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadGpioAon28Sre) -> u8 {
        SwPadCtlPadGpioAon28Sre::to_bits(val)
    }
}
