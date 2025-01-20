#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c1PinSclInSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AON_16 for Mode: ALT9"]
    SelectGpioAon16Alt9 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_21 for Mode: ALT3"]
    SelectGpioAon21Alt3 = 0x01,
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
    SelectGpioAon15Alt9 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_20 for Mode: ALT3"]
    SelectGpioAon20Alt3 = 0x01,
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
    SelectGpioAon07Alt3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_09 for Mode: ALT6"]
    SelectGpioAon09Alt6 = 0x01,
    #[doc = "Selecting Pad: GPIO_AON_21 for Mode: ALT2"]
    SelectGpioAon21Alt2 = 0x02,
    #[doc = "Selecting Pad: GPIO_AON_25 for Mode: ALT1"]
    SelectGpioAon25Alt1 = 0x03,
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
    SelectGpioAon06Alt3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_08 for Mode: ALT6"]
    SelectGpioAon08Alt6 = 0x01,
    #[doc = "Selecting Pad: GPIO_AON_20 for Mode: ALT2"]
    SelectGpioAon20Alt2 = 0x02,
    #[doc = "Selecting Pad: GPIO_AON_24 for Mode: ALT1"]
    SelectGpioAon24Alt1 = 0x03,
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
    SelectGpioAon16Alt4 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_18 for Mode: ALT3"]
    SelectGpioAon18Alt3 = 0x01,
    #[doc = "Selecting Pad: GPIO_AON_23 for Mode: ALT1"]
    SelectGpioAon23Alt1 = 0x02,
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
    SelectGpioAon15Alt4 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_17 for Mode: ALT3"]
    SelectGpioAon17Alt3 = 0x01,
    #[doc = "Selecting Pad: GPIO_AON_22 for Mode: ALT1"]
    SelectGpioAon22Alt1 = 0x02,
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
    SelectGpioAon05Alt0 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_13 for Mode: ALT8"]
    SelectGpioAon13Alt8 = 0x01,
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
    SelectGpioAon03Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_08 for Mode: ALT8"]
    SelectGpioAon08Alt8 = 0x01,
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
    SelectGpioAon04Alt0 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_12 for Mode: ALT8"]
    SelectGpioAon12Alt8 = 0x01,
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
    SelectGpioAon07Alt0 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_15 for Mode: ALT8"]
    SelectGpioAon15Alt8 = 0x01,
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
    SelectGpioAon06Alt0 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_14 for Mode: ALT8"]
    SelectGpioAon14Alt8 = 0x01,
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
    SelectGpioAon10Alt1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_16 for Mode: ALT1"]
    SelectGpioAon16Alt1 = 0x01,
    #[doc = "Selecting Pad: GPIO_AON_25 for Mode: ALT6"]
    SelectGpioAon25Alt6 = 0x02,
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
    SelectGpioAon15Alt1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_21 for Mode: ALT1"]
    SelectGpioAon21Alt1 = 0x01,
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
    SelectGpioAon02Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_27 for Mode: ALT1"]
    SelectGpioAon27Alt1 = 0x01,
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
    SelectGpioAon19Alt1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_22 for Mode: ALT6"]
    SelectGpioAon22Alt6 = 0x01,
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
    SelectGpioAon03Alt3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_17 for Mode: ALT1"]
    SelectGpioAon17Alt1 = 0x01,
    #[doc = "Selecting Pad: GPIO_AON_24 for Mode: ALT6"]
    SelectGpioAon24Alt6 = 0x02,
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
    SelectGpioAon02Alt3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_18 for Mode: ALT1"]
    SelectGpioAon18Alt1 = 0x01,
    #[doc = "Selecting Pad: GPIO_AON_23 for Mode: ALT6"]
    SelectGpioAon23Alt6 = 0x02,
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
    SelectGpioAon00Alt4 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_13 for Mode: ALT6"]
    SelectGpioAon13Alt6 = 0x01,
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
    SelectGpioAon01Alt4 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_14 for Mode: ALT6"]
    SelectGpioAon14Alt6 = 0x01,
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
    SelectGpioAon02Alt4 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_15 for Mode: ALT6"]
    SelectGpioAon15Alt6 = 0x01,
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
    SelectGpioAon13Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_22 for Mode: ALT3"]
    SelectGpioAon22Alt3 = 0x01,
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
    SelectGpioAon16Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_20 for Mode: ALT9"]
    SelectGpioAon20Alt9 = 0x01,
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
    SelectGpioAon15Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_19 for Mode: ALT9"]
    SelectGpioAon19Alt9 = 0x01,
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
    SelectGpioAon11Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_19 for Mode: ALT4"]
    SelectGpioAon19Alt4 = 0x01,
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
    SelectGpioAon14Alt3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_18 for Mode: ALT4"]
    SelectGpioAon18Alt4 = 0x01,
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
    SelectGpioAon13Alt3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_17 for Mode: ALT4"]
    SelectGpioAon17Alt4 = 0x01,
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
    SelectGpioAon03Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_25 for Mode: ALT2"]
    SelectGpioAon25Alt2 = 0x01,
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
    SelectGpioAon01Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_27 for Mode: ALT2"]
    SelectGpioAon27Alt2 = 0x01,
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
    SelectGpioAon00Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_26 for Mode: ALT2"]
    SelectGpioAon26Alt2 = 0x01,
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
    SelectGpioAon04Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_16 for Mode: ALT8"]
    SelectGpioAon16Alt8 = 0x01,
    #[doc = "Selecting Pad: GPIO_AON_24 for Mode: ALT3"]
    SelectGpioAon24Alt3 = 0x02,
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
    SelectGpioAon18Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_23 for Mode: ALT2"]
    SelectGpioAon23Alt2 = 0x01,
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
    SelectGpioAon17Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_22 for Mode: ALT2"]
    SelectGpioAon22Alt2 = 0x01,
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
    SelectGpioAon07Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_24 for Mode: ALT4"]
    SelectGpioAon24Alt4 = 0x01,
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
    SelectGpioAon09Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_26 for Mode: ALT4"]
    SelectGpioAon26Alt4 = 0x01,
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
    SelectGpioAon08Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_25 for Mode: ALT4"]
    SelectGpioAon25Alt4 = 0x01,
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
    SelectGpioAon04Alt3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_21 for Mode: ALT9"]
    SelectGpioAon21Alt9 = 0x01,
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
    SelectGpioAon10Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_27 for Mode: ALT4"]
    SelectGpioAon27Alt4 = 0x01,
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
    SelectGpioAon06Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_23 for Mode: ALT4"]
    SelectGpioAon23Alt4 = 0x01,
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
    SelectGpioAon05Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_22 for Mode: ALT4"]
    SelectGpioAon22Alt4 = 0x01,
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
    Alt0SrcBootMode0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: CAN1_TX of instance: can1"]
    Alt1Can1Tx = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPTMR1_ALT1 of instance: lptmr1"]
    Alt4Lptmr1Alt1 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO00 of instance: gpio1"]
    Alt5Gpio1Io0 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPUART2_TX of instance: lpuart2"]
    Alt6Lpuart2Tx = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: TPM1_EXTCLK of instance: tpm1"]
    Alt8Tpm1Extclk = 0x08,
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
    Alt0SrcBootMode1 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: CAN1_RX of instance: can1"]
    Alt1Can1Rx = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPTMR1_ALT2 of instance: lptmr1"]
    Alt4Lptmr1Alt2 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO01 of instance: gpio1"]
    Alt5Gpio1Io1 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPUART2_RX of instance: lpuart2"]
    Alt6Lpuart2Rx = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: TPM1_CH00 of instance: tpm1"]
    Alt8Tpm1Ch0 = 0x08,
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
    Alt0SrcBootMode2 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: CAN3_TX of instance: can3"]
    Alt1Can3Tx = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPSPI2_PCS3 of instance: lpspi2"]
    Alt2Lpspi2Pcs3 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPSPI2_SDO of instance: lpspi2"]
    Alt3Lpspi2Sdo = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPTMR1_ALT3 of instance: lptmr1"]
    Alt4Lptmr1Alt3 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO02 of instance: gpio1"]
    Alt5Gpio1Io2 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPUART2_RTS_B of instance: lpuart2"]
    Alt6Lpuart2RtsB = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: TPM1_CH01 of instance: tpm1"]
    Alt8Tpm1Ch1 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_CLK_ECAT_CLK25 of instance: ecat"]
    Alt12EcatClkEcatClk25 = 0x0c,
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
    Alt1Can3Rx = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPSPI1_PCS1 of instance: lpspi1"]
    Alt2Lpspi1Pcs1 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPSPI2_SDI of instance: lpspi2"]
    Alt3Lpspi2Sdi = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPSPI1_PCS3 of instance: lpspi1"]
    Alt4Lpspi1Pcs3 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO03 of instance: gpio1"]
    Alt5Gpio1Io3 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPUART2_CTS_B of instance: lpuart2"]
    Alt6Lpuart2CtsB = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: TPM1_CH02 of instance: tpm1"]
    Alt8Tpm1Ch2 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_LED_STATE_RUN of instance: ecat"]
    Alt12EcatLedStateRun = 0x0c,
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
    Alt0Lpspi1Sck = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SAI1_TX_DATA00 of instance: sai1"]
    Alt2Sai1TxData0 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SAI1_RX_DATA01 of instance: sai1"]
    Alt3Sai1RxData1 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO04 of instance: gpio1"]
    Alt5Gpio1Io4 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPUART7_CTS_B of instance: lpuart7"]
    Alt6Lpuart7CtsB = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: TPM1_CH03 of instance: tpm1"]
    Alt8Tpm1Ch3 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_LED_RUN of instance: ecat"]
    Alt12EcatLedRun = 0x0c,
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
    Alt0Lpspi1Pcs0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SAI1_TX_SYNC of instance: sai1"]
    Alt2Sai1TxSync = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO05 of instance: gpio1"]
    Alt5Gpio1Io5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPUART7_RTS_B of instance: lpuart7"]
    Alt6Lpuart7RtsB = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: NMI_GLUE_NMI of instance: nmi_glue"]
    Alt7NmiGlueNmi = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_LED_ERR of instance: ecat"]
    Alt12EcatLedErr = 0x0c,
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
    Alt0Lpspi1Sdo = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: I3C1_PUR of instance: i3c1"]
    Alt1I3c1Pur = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SAI1_TX_BCLK of instance: sai1"]
    Alt2Sai1TxBclk = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPI2C1_SDA of instance: lpi2c1"]
    Alt3Lpi2c1Sda = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO06 of instance: gpio1"]
    Alt5Gpio1Io6 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: CAN1_TX of instance: can1"]
    Alt6Can1Tx = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_SDA of instance: ecat"]
    Alt12EcatSda = 0x0c,
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
    Alt0Lpspi1Sdi = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SAI1_MCLK of instance: sai1"]
    Alt2Sai1Mclk = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPI2C1_SCL of instance: lpi2c1"]
    Alt3Lpi2c1Scl = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO07 of instance: gpio1"]
    Alt5Gpio1Io7 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: CAN1_RX of instance: can1"]
    Alt6Can1Rx = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_SCL of instance: ecat"]
    Alt12EcatScl = 0x0c,
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
    Alt0Lpuart1Tx = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: S400_TX of instance: s400"]
    Alt1S400Tx = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SAI1_RX_DATA00 of instance: sai1"]
    Alt2Sai1RxData0 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SAI1_TX_DATA01 of instance: sai1"]
    Alt3Sai1TxData1 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO08 of instance: gpio1"]
    Alt5Gpio1Io8 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPI2C1_SDA of instance: lpi2c1"]
    Alt6Lpi2c1Sda = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPSPI1_PCS1 of instance: lpspi1"]
    Alt8Lpspi1Pcs1 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_LINK_ACT00 of instance: ecat"]
    Alt12EcatLinkAct0 = 0x0c,
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
    Alt0Lpuart1Rx = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: S400_RX of instance: s400"]
    Alt1S400Rx = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SAI1_RX_BCLK of instance: sai1"]
    Alt2Sai1RxBclk = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPIT1_TRIGGER00 of instance: lpit1"]
    Alt3Lpit1Trigger0 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO09 of instance: gpio1"]
    Alt5Gpio1Io9 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPI2C1_SCL of instance: lpi2c1"]
    Alt6Lpi2c1Scl = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPSPI1_PCS2 of instance: lpspi1"]
    Alt8Lpspi1Pcs2 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_LINK_ACT01 of instance: ecat"]
    Alt12EcatLinkAct1 = 0x0c,
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
    Alt0JtagMuxTrstb = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPSPI2_PCS0 of instance: lpspi2"]
    Alt1Lpspi2Pcs0 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SAI1_RX_SYNC of instance: sai1"]
    Alt2Sai1RxSync = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPIT1_TRIGGER01 of instance: lpit1"]
    Alt3Lpit1Trigger1 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: TPM2_EXTCLK of instance: tpm2"]
    Alt4Tpm2Extclk = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO10 of instance: gpio1"]
    Alt5Gpio1Io10 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPI2C1_SCLS of instance: lpi2c1"]
    Alt6Lpi2c1Scls = 0x06,
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
    Alt0JtagMuxTdo = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART1_CTS_B of instance: lpuart1"]
    Alt2Lpuart1CtsB = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPIT1_TRIGGER02 of instance: lpit1"]
    Alt3Lpit1Trigger2 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: TPM2_CH00 of instance: tpm2"]
    Alt4Tpm2Ch0 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO11 of instance: gpio1"]
    Alt5Gpio1Io11 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPI2C1_SDAS of instance: lpi2c1"]
    Alt6Lpi2c1Sdas = 0x06,
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
    Alt0JtagMuxTdi = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART1_RTS_B of instance: lpuart1"]
    Alt2Lpuart1RtsB = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPIT1_TRIGGER03 of instance: lpit1"]
    Alt3Lpit1Trigger3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: TPM2_CH01 of instance: tpm2"]
    Alt4Tpm2Ch1 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO12 of instance: gpio1"]
    Alt5Gpio1Io12 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPI2C1_HREQ of instance: lpi2c1"]
    Alt6Lpi2c1Hreq = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPSPI1_SCK of instance: lpspi1"]
    Alt8Lpspi1Sck = 0x08,
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
    Alt0JtagMuxTck = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART12_CTS_B of instance: lpuart12"]
    Alt2Lpuart12CtsB = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART1_DSR_B of instance: lpuart1"]
    Alt3Lpuart1DsrB = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: TPM2_CH02 of instance: tpm2"]
    Alt4Tpm2Ch2 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO13 of instance: gpio1"]
    Alt5Gpio1Io13 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPTMR1_ALT1 of instance: lptmr1"]
    Alt6Lptmr1Alt1 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPSPI1_PCS0 of instance: lpspi1"]
    Alt8Lpspi1Pcs0 = 0x08,
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
    Alt0JtagMuxTms = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART12_RTS_B of instance: lpuart12"]
    Alt2Lpuart12RtsB = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART1_DCD_B of instance: lpuart1"]
    Alt3Lpuart1DcdB = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: TPM2_CH03 of instance: tpm2"]
    Alt4Tpm2Ch3 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO14 of instance: gpio1"]
    Alt5Gpio1Io14 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPTMR1_ALT2 of instance: lptmr1"]
    Alt6Lptmr1Alt2 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPSPI1_SDO of instance: lpspi1"]
    Alt8Lpspi1Sdo = 0x08,
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
    Alt0Flexspi2Bus2bitBData3 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPSPI2_PCS1 of instance: lpspi2"]
    Alt1Lpspi2Pcs1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART12_TX of instance: lpuart12"]
    Alt2Lpuart12Tx = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART1_RI_B of instance: lpuart1"]
    Alt3Lpuart1RiB = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPI2C2_SDA of instance: lpi2c2"]
    Alt4Lpi2c2Sda = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO15 of instance: gpio1"]
    Alt5Gpio1Io15 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPTMR1_ALT3 of instance: lptmr1"]
    Alt6Lptmr1Alt3 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPSPI1_SDI of instance: lpspi1"]
    Alt8Lpspi1Sdi = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: I3C1_SDA of instance: i3c1"]
    Alt9I3c1Sda = 0x09,
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
    Alt0Flexspi2Bus2bitBData2 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPSPI2_PCS0 of instance: lpspi2"]
    Alt1Lpspi2Pcs0 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART12_RX of instance: lpuart12"]
    Alt2Lpuart12Rx = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART1_DTR_B of instance: lpuart1"]
    Alt3Lpuart1DtrB = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPI2C2_SCL of instance: lpi2c2"]
    Alt4Lpi2c2Scl = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO16 of instance: gpio1"]
    Alt5Gpio1Io16 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: CAN1_TX of instance: can1"]
    Alt6Can1Tx = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPUART7_CTS_B of instance: lpuart7"]
    Alt8Lpuart7CtsB = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: I3C1_SCL of instance: i3c1"]
    Alt9I3c1Scl = 0x09,
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
    Alt0Flexspi2Bus2bitBData1 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPSPI2_SDI of instance: lpspi2"]
    Alt1Lpspi2Sdi = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART7_TX of instance: lpuart7"]
    Alt2Lpuart7Tx = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPI2C2_SDA of instance: lpi2c2"]
    Alt3Lpi2c2Sda = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPUART1_DSR_B of instance: lpuart1"]
    Alt4Lpuart1DsrB = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO17 of instance: gpio1"]
    Alt5Gpio1Io17 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: CAN1_RX of instance: can1"]
    Alt6Can1Rx = 0x06,
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
    Alt0Flexspi2Bus2bitBData0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPSPI2_SDO of instance: lpspi2"]
    Alt1Lpspi2Sdo = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART7_RX of instance: lpuart7"]
    Alt2Lpuart7Rx = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPI2C2_SCL of instance: lpi2c2"]
    Alt3Lpi2c2Scl = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPUART1_DCD_B of instance: lpuart1"]
    Alt4Lpuart1DcdB = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO18 of instance: gpio1"]
    Alt5Gpio1Io18 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: CAN3_TX of instance: can3"]
    Alt6Can3Tx = 0x06,
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
    Alt0Flexspi2Bus2bitBSclk = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPSPI2_SCK of instance: lpspi2"]
    Alt1Lpspi2Sck = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: FLEXSPI2_BUS2BIT_A_SS1_B of instance: flexspi2_bus2bit"]
    Alt3Flexspi2Bus2bitASs1B = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPUART1_CTS_B of instance: lpuart1"]
    Alt4Lpuart1CtsB = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO19 of instance: gpio1"]
    Alt5Gpio1Io19 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: CAN3_RX of instance: can3"]
    Alt6Can3Rx = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPUART7_RTS_B of instance: lpuart7"]
    Alt8Lpuart7RtsB = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPUART12_TX of instance: lpuart12"]
    Alt9Lpuart12Tx = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ADC1_CONV_D00 of instance: adc1"]
    Alt12Adc1ConvD0 = 0x0c,
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
    Alt0Flexspi2Bus2bitBDqs = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXSPI2_BUS2BIT_A_SS1_B of instance: flexspi2_bus2bit"]
    Alt1Flexspi2Bus2bitASs1B = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPI2C1_SDA of instance: lpi2c1"]
    Alt2Lpi2c1Sda = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: I3C1_SDA of instance: i3c1"]
    Alt3I3c1Sda = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPUART1_RTS_B of instance: lpuart1"]
    Alt4Lpuart1RtsB = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO20 of instance: gpio1"]
    Alt5Gpio1Io20 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPUART12_RX of instance: lpuart12"]
    Alt9Lpuart12Rx = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ADC1_CONV_D01 of instance: adc1"]
    Alt12Adc1ConvD1 = 0x0c,
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
    Alt0Flexspi2Bus2bitBSs0B = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPSPI2_PCS1 of instance: lpspi2"]
    Alt1Lpspi2Pcs1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPI2C1_SCL of instance: lpi2c1"]
    Alt2Lpi2c1Scl = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: I3C1_SCL of instance: i3c1"]
    Alt3I3c1Scl = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: SAI1_TX_DATA00 of instance: sai1"]
    Alt4Sai1TxData0 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO21 of instance: gpio1"]
    Alt5Gpio1Io21 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXSPI2_BUS2BIT_A_DQS of instance: flexspi2_bus2bit"]
    Alt8Flexspi2Bus2bitADqs = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: SAI1_RX_DATA01 of instance: sai1"]
    Alt9Sai1RxData1 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ADC1_CONV_D02 of instance: adc1"]
    Alt12Adc1ConvD2 = 0x0c,
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
    Alt0Flexspi2Bus2bitASs0B = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPI2C2_SDA of instance: lpi2c2"]
    Alt1Lpi2c2Sda = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART7_TX of instance: lpuart7"]
    Alt2Lpuart7Tx = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART12_CTS_B of instance: lpuart12"]
    Alt3Lpuart12CtsB = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: SAI1_TX_SYNC of instance: sai1"]
    Alt4Sai1TxSync = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO22 of instance: gpio1"]
    Alt5Gpio1Io22 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPSPI2_SCK of instance: lpspi2"]
    Alt6Lpspi2Sck = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: CCMSRCGPC_CCMOBS1 of instance: ccmsrcgpc"]
    Alt10CcmsrcgpcmixCcmobs1 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ADC1_CONV_D03 of instance: adc1"]
    Alt12Adc1ConvD3 = 0x0c,
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
    Alt0Flexspi2Bus2bitASclk = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPI2C2_SCL of instance: lpi2c2"]
    Alt1Lpi2c2Scl = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART7_RX of instance: lpuart7"]
    Alt2Lpuart7Rx = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART12_RTS_B of instance: lpuart12"]
    Alt3Lpuart12RtsB = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: SAI1_TX_BCLK of instance: sai1"]
    Alt4Sai1TxBclk = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO23 of instance: gpio1"]
    Alt5Gpio1Io23 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPSPI2_SDO of instance: lpspi2"]
    Alt6Lpspi2Sdo = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: CCMSRCGPC_CCMOBS2 of instance: ccmsrcgpc"]
    Alt10CcmsrcgpcmixCcmobs2 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ADC1_CONV_D04 of instance: adc1"]
    Alt12Adc1ConvD4 = 0x0c,
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
    Alt0Flexspi2Bus2bitAData0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPI2C1_SDA of instance: lpi2c1"]
    Alt1Lpi2c1Sda = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART2_RTS_B of instance: lpuart2"]
    Alt2Lpuart2RtsB = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART7_CTS_B of instance: lpuart7"]
    Alt3Lpuart7CtsB = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: SAI1_MCLK of instance: sai1"]
    Alt4Sai1Mclk = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO24 of instance: gpio1"]
    Alt5Gpio1Io24 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPSPI2_SDI of instance: lpspi2"]
    Alt6Lpspi2Sdi = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ADC1_CONV_D05 of instance: adc1"]
    Alt12Adc1ConvD5 = 0x0c,
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
    Alt0Flexspi2Bus2bitAData1 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPI2C1_SCL of instance: lpi2c1"]
    Alt1Lpi2c1Scl = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART2_CTS_B of instance: lpuart2"]
    Alt2Lpuart2CtsB = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART7_RTS_B of instance: lpuart7"]
    Alt3Lpuart7RtsB = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: SAI1_RX_DATA00 of instance: sai1"]
    Alt4Sai1RxData0 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO25 of instance: gpio1"]
    Alt5Gpio1Io25 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPSPI2_PCS0 of instance: lpspi2"]
    Alt6Lpspi2Pcs0 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: SAI1_TX_DATA01 of instance: sai1"]
    Alt7Sai1TxData1 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ADC1_CONV_D06 of instance: adc1"]
    Alt12Adc1ConvD6 = 0x0c,
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
    Alt0Flexspi2Bus2bitAData2 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPSPI2_PCS2 of instance: lpspi2"]
    Alt1Lpspi2Pcs2 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART2_TX of instance: lpuart2"]
    Alt2Lpuart2Tx = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: SAI1_RX_BCLK of instance: sai1"]
    Alt4Sai1RxBclk = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO26 of instance: gpio1"]
    Alt5Gpio1Io26 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ADC1_CONV_D07 of instance: adc1"]
    Alt12Adc1ConvD7 = 0x0c,
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
    Alt0Flexspi2Bus2bitAData3 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPSPI2_PCS3 of instance: lpspi2"]
    Alt1Lpspi2Pcs3 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART2_RX of instance: lpuart2"]
    Alt2Lpuart2Rx = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: SAI1_RX_SYNC of instance: sai1"]
    Alt4Sai1RxSync = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO27 of instance: gpio1"]
    Alt5Gpio1Io27 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: EWM_EWM_OUT_B of instance: ewm"]
    Alt7EwmEwmOutB = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ADC1_CONV_RDY_CLK of instance: adc1"]
    Alt12Adc1ConvRdyClk = 0x0c,
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
    Alt0Flexspi2Bus2bitADqs = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXSPI2_BUS2BIT_B_DQS of instance: flexspi2_bus2bit"]
    Alt1Flexspi2Bus2bitBDqs = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO28 of instance: gpio1"]
    Alt5Gpio1Io28 = 0x05,
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
