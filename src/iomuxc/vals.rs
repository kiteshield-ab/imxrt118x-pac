#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Can1IppIndCanrxSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_16 for Mode: ALT9"]
    SELECT_GPIO_AD_16_ALT9 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_01 for Mode: ALT1"]
    SELECT_GPIO_AON_01_ALT1 = 0x01,
    #[doc = "Selecting Pad: GPIO_AON_07 for Mode: ALT6"]
    SELECT_GPIO_AON_07_ALT6 = 0x02,
    #[doc = "Selecting Pad: GPIO_AON_17 for Mode: ALT6"]
    SELECT_GPIO_AON_17_ALT6 = 0x03,
}
impl Can1IppIndCanrxSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Can1IppIndCanrxSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Can1IppIndCanrxSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Can1IppIndCanrxSelectInputDaisy {
        Can1IppIndCanrxSelectInputDaisy::from_bits(val)
    }
}
impl From<Can1IppIndCanrxSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Can1IppIndCanrxSelectInputDaisy) -> u8 {
        Can1IppIndCanrxSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Can2IppIndCanrxSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_01 for Mode: ALT1"]
    SELECT_GPIO_AD_01_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_31 for Mode: ALT2"]
    SELECT_GPIO_AD_31_ALT2 = 0x01,
}
impl Can2IppIndCanrxSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Can2IppIndCanrxSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Can2IppIndCanrxSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Can2IppIndCanrxSelectInputDaisy {
        Can2IppIndCanrxSelectInputDaisy::from_bits(val)
    }
}
impl From<Can2IppIndCanrxSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Can2IppIndCanrxSelectInputDaisy) -> u8 {
        Can2IppIndCanrxSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Can3IppIndCanrxSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_07 for Mode: ALT1"]
    SELECT_GPIO_AD_07_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_11 for Mode: ALT2"]
    SELECT_GPIO_B2_11_ALT2 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_13 for Mode: ALT6"]
    SELECT_GPIO_B2_13_ALT6 = 0x02,
    #[doc = "Selecting Pad: GPIO_AON_03 for Mode: ALT1"]
    SELECT_GPIO_AON_03_ALT1 = 0x03,
    #[doc = "Selecting Pad: GPIO_AON_19 for Mode: ALT6"]
    SELECT_GPIO_AON_19_ALT6 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Can3IppIndCanrxSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Can3IppIndCanrxSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Can3IppIndCanrxSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Can3IppIndCanrxSelectInputDaisy {
        Can3IppIndCanrxSelectInputDaisy::from_bits(val)
    }
}
impl From<Can3IppIndCanrxSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Can3IppIndCanrxSelectInputDaisy) -> u8 {
        Can3IppIndCanrxSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dse {
    #[doc = "normal driver"]
    DSE_0_NORMAL_DRIVER = 0x0,
    #[doc = "high driver"]
    DSE_1_HIGH_DRIVER = 0x01,
}
impl Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dse {
    #[inline(always)]
    fn from(val: u8) -> Dse {
        Dse::from_bits(val)
    }
}
impl From<Dse> for u8 {
    #[inline(always)]
    fn from(val: Dse) -> u8 {
        Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EcatEcatRxClk0SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_02 for Mode: ALT10"]
    SELECT_GPIO_EMC_B1_02_ALT10 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_00 for Mode: ALT12"]
    SELECT_GPIO_EMC_B2_00_ALT12 = 0x01,
}
impl EcatEcatRxClk0SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EcatEcatRxClk0SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EcatEcatRxClk0SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> EcatEcatRxClk0SelectInputDaisy {
        EcatEcatRxClk0SelectInputDaisy::from_bits(val)
    }
}
impl From<EcatEcatRxClk0SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: EcatEcatRxClk0SelectInputDaisy) -> u8 {
        EcatEcatRxClk0SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EcatEcatRxClk1SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_38 for Mode: ALT6"]
    SELECT_GPIO_EMC_B1_38_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_13 for Mode: ALT12"]
    SELECT_GPIO_B2_13_ALT12 = 0x01,
}
impl EcatEcatRxClk1SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EcatEcatRxClk1SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EcatEcatRxClk1SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> EcatEcatRxClk1SelectInputDaisy {
        EcatEcatRxClk1SelectInputDaisy::from_bits(val)
    }
}
impl From<EcatEcatRxClk1SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: EcatEcatRxClk1SelectInputDaisy) -> u8 {
        EcatEcatRxClk1SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EcatEcatRxData00SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_09 for Mode: ALT10"]
    SELECT_GPIO_EMC_B1_09_ALT10 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_09 for Mode: ALT12"]
    SELECT_GPIO_EMC_B2_09_ALT12 = 0x01,
}
impl EcatEcatRxData00SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EcatEcatRxData00SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EcatEcatRxData00SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> EcatEcatRxData00SelectInputDaisy {
        EcatEcatRxData00SelectInputDaisy::from_bits(val)
    }
}
impl From<EcatEcatRxData00SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: EcatEcatRxData00SelectInputDaisy) -> u8 {
        EcatEcatRxData00SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EcatEcatRxData01SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_30 for Mode: ALT6"]
    SELECT_GPIO_EMC_B1_30_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_17 for Mode: ALT12"]
    SELECT_GPIO_EMC_B2_17_ALT12 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_10 for Mode: ALT12"]
    SELECT_GPIO_B2_10_ALT12 = 0x02,
    _RESERVED_3 = 0x03,
}
impl EcatEcatRxData01SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EcatEcatRxData01SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EcatEcatRxData01SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> EcatEcatRxData01SelectInputDaisy {
        EcatEcatRxData01SelectInputDaisy::from_bits(val)
    }
}
impl From<EcatEcatRxData01SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: EcatEcatRxData01SelectInputDaisy) -> u8 {
        EcatEcatRxData01SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EcatEcatRxData10SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_10 for Mode: ALT10"]
    SELECT_GPIO_EMC_B1_10_ALT10 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_10 for Mode: ALT12"]
    SELECT_GPIO_EMC_B2_10_ALT12 = 0x01,
}
impl EcatEcatRxData10SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EcatEcatRxData10SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EcatEcatRxData10SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> EcatEcatRxData10SelectInputDaisy {
        EcatEcatRxData10SelectInputDaisy::from_bits(val)
    }
}
impl From<EcatEcatRxData10SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: EcatEcatRxData10SelectInputDaisy) -> u8 {
        EcatEcatRxData10SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EcatEcatRxData11SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_31 for Mode: ALT6"]
    SELECT_GPIO_EMC_B1_31_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_18 for Mode: ALT12"]
    SELECT_GPIO_EMC_B2_18_ALT12 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_11 for Mode: ALT12"]
    SELECT_GPIO_B2_11_ALT12 = 0x02,
    _RESERVED_3 = 0x03,
}
impl EcatEcatRxData11SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EcatEcatRxData11SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EcatEcatRxData11SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> EcatEcatRxData11SelectInputDaisy {
        EcatEcatRxData11SelectInputDaisy::from_bits(val)
    }
}
impl From<EcatEcatRxData11SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: EcatEcatRxData11SelectInputDaisy) -> u8 {
        EcatEcatRxData11SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EcatEcatRxData20SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_04 for Mode: ALT10"]
    SELECT_GPIO_EMC_B1_04_ALT10 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_01 for Mode: ALT12"]
    SELECT_GPIO_EMC_B2_01_ALT12 = 0x01,
}
impl EcatEcatRxData20SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EcatEcatRxData20SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EcatEcatRxData20SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> EcatEcatRxData20SelectInputDaisy {
        EcatEcatRxData20SelectInputDaisy::from_bits(val)
    }
}
impl From<EcatEcatRxData20SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: EcatEcatRxData20SelectInputDaisy) -> u8 {
        EcatEcatRxData20SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EcatEcatRxData21SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_34 for Mode: ALT6"]
    SELECT_GPIO_EMC_B1_34_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_02 for Mode: ALT12"]
    SELECT_GPIO_B2_02_ALT12 = 0x01,
}
impl EcatEcatRxData21SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EcatEcatRxData21SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EcatEcatRxData21SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> EcatEcatRxData21SelectInputDaisy {
        EcatEcatRxData21SelectInputDaisy::from_bits(val)
    }
}
impl From<EcatEcatRxData21SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: EcatEcatRxData21SelectInputDaisy) -> u8 {
        EcatEcatRxData21SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EcatEcatRxData30SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_03 for Mode: ALT10"]
    SELECT_GPIO_EMC_B1_03_ALT10 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_02 for Mode: ALT12"]
    SELECT_GPIO_EMC_B2_02_ALT12 = 0x01,
}
impl EcatEcatRxData30SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EcatEcatRxData30SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EcatEcatRxData30SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> EcatEcatRxData30SelectInputDaisy {
        EcatEcatRxData30SelectInputDaisy::from_bits(val)
    }
}
impl From<EcatEcatRxData30SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: EcatEcatRxData30SelectInputDaisy) -> u8 {
        EcatEcatRxData30SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EcatEcatRxData31SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_35 for Mode: ALT6"]
    SELECT_GPIO_EMC_B1_35_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_03 for Mode: ALT12"]
    SELECT_GPIO_B2_03_ALT12 = 0x01,
}
impl EcatEcatRxData31SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EcatEcatRxData31SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EcatEcatRxData31SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> EcatEcatRxData31SelectInputDaisy {
        EcatEcatRxData31SelectInputDaisy::from_bits(val)
    }
}
impl From<EcatEcatRxData31SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: EcatEcatRxData31SelectInputDaisy) -> u8 {
        EcatEcatRxData31SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EcatEcatRxDv0SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_11 for Mode: ALT10"]
    SELECT_GPIO_EMC_B1_11_ALT10 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_11 for Mode: ALT12"]
    SELECT_GPIO_EMC_B2_11_ALT12 = 0x01,
}
impl EcatEcatRxDv0SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EcatEcatRxDv0SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EcatEcatRxDv0SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> EcatEcatRxDv0SelectInputDaisy {
        EcatEcatRxDv0SelectInputDaisy::from_bits(val)
    }
}
impl From<EcatEcatRxDv0SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: EcatEcatRxDv0SelectInputDaisy) -> u8 {
        EcatEcatRxDv0SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EcatEcatRxDv1SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_32 for Mode: ALT6"]
    SELECT_GPIO_EMC_B1_32_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_19 for Mode: ALT12"]
    SELECT_GPIO_EMC_B2_19_ALT12 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_12 for Mode: ALT12"]
    SELECT_GPIO_B2_12_ALT12 = 0x02,
    _RESERVED_3 = 0x03,
}
impl EcatEcatRxDv1SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EcatEcatRxDv1SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EcatEcatRxDv1SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> EcatEcatRxDv1SelectInputDaisy {
        EcatEcatRxDv1SelectInputDaisy::from_bits(val)
    }
}
impl From<EcatEcatRxDv1SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: EcatEcatRxDv1SelectInputDaisy) -> u8 {
        EcatEcatRxDv1SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EcatEcatRxEr0SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_12 for Mode: ALT10"]
    SELECT_GPIO_EMC_B1_12_ALT10 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_12 for Mode: ALT12"]
    SELECT_GPIO_EMC_B2_12_ALT12 = 0x01,
}
impl EcatEcatRxEr0SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EcatEcatRxEr0SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EcatEcatRxEr0SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> EcatEcatRxEr0SelectInputDaisy {
        EcatEcatRxEr0SelectInputDaisy::from_bits(val)
    }
}
impl From<EcatEcatRxEr0SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: EcatEcatRxEr0SelectInputDaisy) -> u8 {
        EcatEcatRxEr0SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EcatEcatRxEr1SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_33 for Mode: ALT6"]
    SELECT_GPIO_EMC_B1_33_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_20 for Mode: ALT12"]
    SELECT_GPIO_EMC_B2_20_ALT12 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_01 for Mode: ALT12"]
    SELECT_GPIO_B2_01_ALT12 = 0x02,
    _RESERVED_3 = 0x03,
}
impl EcatEcatRxEr1SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EcatEcatRxEr1SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EcatEcatRxEr1SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> EcatEcatRxEr1SelectInputDaisy {
        EcatEcatRxEr1SelectInputDaisy::from_bits(val)
    }
}
impl From<EcatEcatRxEr1SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: EcatEcatRxEr1SelectInputDaisy) -> u8 {
        EcatEcatRxEr1SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EcatEcatTxClk0SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_08 for Mode: ALT10"]
    SELECT_GPIO_EMC_B1_08_ALT10 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_08 for Mode: ALT12"]
    SELECT_GPIO_EMC_B2_08_ALT12 = 0x01,
}
impl EcatEcatTxClk0SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EcatEcatTxClk0SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EcatEcatTxClk0SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> EcatEcatTxClk0SelectInputDaisy {
        EcatEcatTxClk0SelectInputDaisy::from_bits(val)
    }
}
impl From<EcatEcatTxClk0SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: EcatEcatTxClk0SelectInputDaisy) -> u8 {
        EcatEcatTxClk0SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EcatEcatTxClk1SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_29 for Mode: ALT6"]
    SELECT_GPIO_EMC_B1_29_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_16 for Mode: ALT12"]
    SELECT_GPIO_EMC_B2_16_ALT12 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_09 for Mode: ALT12"]
    SELECT_GPIO_B2_09_ALT12 = 0x02,
    _RESERVED_3 = 0x03,
}
impl EcatEcatTxClk1SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EcatEcatTxClk1SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EcatEcatTxClk1SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> EcatEcatTxClk1SelectInputDaisy {
        EcatEcatTxClk1SelectInputDaisy::from_bits(val)
    }
}
impl From<EcatEcatTxClk1SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: EcatEcatTxClk1SelectInputDaisy) -> u8 {
        EcatEcatTxClk1SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EcatMdioDataInSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_31 for Mode: ALT12"]
    SELECT_GPIO_AD_31_ALT12 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_10 for Mode: ALT12"]
    SELECT_GPIO_SD_B2_10_ALT12 = 0x01,
}
impl EcatMdioDataInSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EcatMdioDataInSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EcatMdioDataInSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> EcatMdioDataInSelectInputDaisy {
        EcatMdioDataInSelectInputDaisy::from_bits(val)
    }
}
impl From<EcatMdioDataInSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: EcatMdioDataInSelectInputDaisy) -> u8 {
        EcatMdioDataInSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EcatPromDataInSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_19 for Mode: ALT12"]
    SELECT_GPIO_AD_19_ALT12 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_06 for Mode: ALT12"]
    SELECT_GPIO_AON_06_ALT12 = 0x01,
}
impl EcatPromDataInSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EcatPromDataInSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EcatPromDataInSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> EcatPromDataInSelectInputDaisy {
        EcatPromDataInSelectInputDaisy::from_bits(val)
    }
}
impl From<EcatPromDataInSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: EcatPromDataInSelectInputDaisy) -> u8 {
        EcatPromDataInSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm1IppIndPwmaSelectInput0Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_24 for Mode: ALT1"]
    SELECT_GPIO_EMC_B1_24_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_36 for Mode: ALT1"]
    SELECT_GPIO_EMC_B1_36_ALT1 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_00 for Mode: ALT4"]
    SELECT_GPIO_AD_00_ALT4 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Flexpwm1IppIndPwmaSelectInput0Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm1IppIndPwmaSelectInput0Daisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm1IppIndPwmaSelectInput0Daisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm1IppIndPwmaSelectInput0Daisy {
        Flexpwm1IppIndPwmaSelectInput0Daisy::from_bits(val)
    }
}
impl From<Flexpwm1IppIndPwmaSelectInput0Daisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm1IppIndPwmaSelectInput0Daisy) -> u8 {
        Flexpwm1IppIndPwmaSelectInput0Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm1IppIndPwmaSelectInput1Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_26 for Mode: ALT1"]
    SELECT_GPIO_EMC_B1_26_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_02 for Mode: ALT4"]
    SELECT_GPIO_AD_02_ALT4 = 0x01,
}
impl Flexpwm1IppIndPwmaSelectInput1Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm1IppIndPwmaSelectInput1Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm1IppIndPwmaSelectInput1Daisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm1IppIndPwmaSelectInput1Daisy {
        Flexpwm1IppIndPwmaSelectInput1Daisy::from_bits(val)
    }
}
impl From<Flexpwm1IppIndPwmaSelectInput1Daisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm1IppIndPwmaSelectInput1Daisy) -> u8 {
        Flexpwm1IppIndPwmaSelectInput1Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm1IppIndPwmaSelectInput2Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_29 for Mode: ALT1"]
    SELECT_GPIO_EMC_B1_29_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_05 for Mode: ALT4"]
    SELECT_GPIO_AD_05_ALT4 = 0x01,
}
impl Flexpwm1IppIndPwmaSelectInput2Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm1IppIndPwmaSelectInput2Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm1IppIndPwmaSelectInput2Daisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm1IppIndPwmaSelectInput2Daisy {
        Flexpwm1IppIndPwmaSelectInput2Daisy::from_bits(val)
    }
}
impl From<Flexpwm1IppIndPwmaSelectInput2Daisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm1IppIndPwmaSelectInput2Daisy) -> u8 {
        Flexpwm1IppIndPwmaSelectInput2Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm1IppIndPwmbSelectInput0Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_25 for Mode: ALT1"]
    SELECT_GPIO_EMC_B1_25_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_37 for Mode: ALT1"]
    SELECT_GPIO_EMC_B1_37_ALT1 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_01 for Mode: ALT4"]
    SELECT_GPIO_AD_01_ALT4 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Flexpwm1IppIndPwmbSelectInput0Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm1IppIndPwmbSelectInput0Daisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm1IppIndPwmbSelectInput0Daisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm1IppIndPwmbSelectInput0Daisy {
        Flexpwm1IppIndPwmbSelectInput0Daisy::from_bits(val)
    }
}
impl From<Flexpwm1IppIndPwmbSelectInput0Daisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm1IppIndPwmbSelectInput0Daisy) -> u8 {
        Flexpwm1IppIndPwmbSelectInput0Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm1IppIndPwmbSelectInput1Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_27 for Mode: ALT1"]
    SELECT_GPIO_EMC_B1_27_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_03 for Mode: ALT4"]
    SELECT_GPIO_AD_03_ALT4 = 0x01,
}
impl Flexpwm1IppIndPwmbSelectInput1Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm1IppIndPwmbSelectInput1Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm1IppIndPwmbSelectInput1Daisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm1IppIndPwmbSelectInput1Daisy {
        Flexpwm1IppIndPwmbSelectInput1Daisy::from_bits(val)
    }
}
impl From<Flexpwm1IppIndPwmbSelectInput1Daisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm1IppIndPwmbSelectInput1Daisy) -> u8 {
        Flexpwm1IppIndPwmbSelectInput1Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm1IppIndPwmbSelectInput2Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_28 for Mode: ALT1"]
    SELECT_GPIO_EMC_B1_28_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_04 for Mode: ALT4"]
    SELECT_GPIO_AD_04_ALT4 = 0x01,
}
impl Flexpwm1IppIndPwmbSelectInput2Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm1IppIndPwmbSelectInput2Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm1IppIndPwmbSelectInput2Daisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm1IppIndPwmbSelectInput2Daisy {
        Flexpwm1IppIndPwmbSelectInput2Daisy::from_bits(val)
    }
}
impl From<Flexpwm1IppIndPwmbSelectInput2Daisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm1IppIndPwmbSelectInput2Daisy) -> u8 {
        Flexpwm1IppIndPwmbSelectInput2Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm2IppIndPwmaSelectInput0Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_18 for Mode: ALT1"]
    SELECT_GPIO_EMC_B1_18_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_24 for Mode: ALT4"]
    SELECT_GPIO_AD_24_ALT4 = 0x01,
}
impl Flexpwm2IppIndPwmaSelectInput0Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm2IppIndPwmaSelectInput0Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm2IppIndPwmaSelectInput0Daisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm2IppIndPwmaSelectInput0Daisy {
        Flexpwm2IppIndPwmaSelectInput0Daisy::from_bits(val)
    }
}
impl From<Flexpwm2IppIndPwmaSelectInput0Daisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm2IppIndPwmaSelectInput0Daisy) -> u8 {
        Flexpwm2IppIndPwmaSelectInput0Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm2IppIndPwmaSelectInput1Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_20 for Mode: ALT1"]
    SELECT_GPIO_EMC_B1_20_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_26 for Mode: ALT4"]
    SELECT_GPIO_AD_26_ALT4 = 0x01,
}
impl Flexpwm2IppIndPwmaSelectInput1Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm2IppIndPwmaSelectInput1Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm2IppIndPwmaSelectInput1Daisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm2IppIndPwmaSelectInput1Daisy {
        Flexpwm2IppIndPwmaSelectInput1Daisy::from_bits(val)
    }
}
impl From<Flexpwm2IppIndPwmaSelectInput1Daisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm2IppIndPwmaSelectInput1Daisy) -> u8 {
        Flexpwm2IppIndPwmaSelectInput1Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm2IppIndPwmaSelectInput2Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_23 for Mode: ALT1"]
    SELECT_GPIO_EMC_B1_23_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_29 for Mode: ALT4"]
    SELECT_GPIO_AD_29_ALT4 = 0x01,
}
impl Flexpwm2IppIndPwmaSelectInput2Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm2IppIndPwmaSelectInput2Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm2IppIndPwmaSelectInput2Daisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm2IppIndPwmaSelectInput2Daisy {
        Flexpwm2IppIndPwmaSelectInput2Daisy::from_bits(val)
    }
}
impl From<Flexpwm2IppIndPwmaSelectInput2Daisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm2IppIndPwmaSelectInput2Daisy) -> u8 {
        Flexpwm2IppIndPwmaSelectInput2Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm2IppIndPwmbSelectInput0Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_19 for Mode: ALT1"]
    SELECT_GPIO_EMC_B1_19_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_25 for Mode: ALT4"]
    SELECT_GPIO_AD_25_ALT4 = 0x01,
}
impl Flexpwm2IppIndPwmbSelectInput0Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm2IppIndPwmbSelectInput0Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm2IppIndPwmbSelectInput0Daisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm2IppIndPwmbSelectInput0Daisy {
        Flexpwm2IppIndPwmbSelectInput0Daisy::from_bits(val)
    }
}
impl From<Flexpwm2IppIndPwmbSelectInput0Daisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm2IppIndPwmbSelectInput0Daisy) -> u8 {
        Flexpwm2IppIndPwmbSelectInput0Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm2IppIndPwmbSelectInput1Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_21 for Mode: ALT1"]
    SELECT_GPIO_EMC_B1_21_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_27 for Mode: ALT4"]
    SELECT_GPIO_AD_27_ALT4 = 0x01,
}
impl Flexpwm2IppIndPwmbSelectInput1Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm2IppIndPwmbSelectInput1Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm2IppIndPwmbSelectInput1Daisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm2IppIndPwmbSelectInput1Daisy {
        Flexpwm2IppIndPwmbSelectInput1Daisy::from_bits(val)
    }
}
impl From<Flexpwm2IppIndPwmbSelectInput1Daisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm2IppIndPwmbSelectInput1Daisy) -> u8 {
        Flexpwm2IppIndPwmbSelectInput1Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm2IppIndPwmbSelectInput2Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_22 for Mode: ALT1"]
    SELECT_GPIO_EMC_B1_22_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_28 for Mode: ALT4"]
    SELECT_GPIO_AD_28_ALT4 = 0x01,
}
impl Flexpwm2IppIndPwmbSelectInput2Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm2IppIndPwmbSelectInput2Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm2IppIndPwmbSelectInput2Daisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm2IppIndPwmbSelectInput2Daisy {
        Flexpwm2IppIndPwmbSelectInput2Daisy::from_bits(val)
    }
}
impl From<Flexpwm2IppIndPwmbSelectInput2Daisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm2IppIndPwmbSelectInput2Daisy) -> u8 {
        Flexpwm2IppIndPwmbSelectInput2Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm3IppIndPwmaSelectInput0Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_30 for Mode: ALT1"]
    SELECT_GPIO_EMC_B1_30_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_00 for Mode: ALT10"]
    SELECT_GPIO_EMC_B2_00_ALT10 = 0x01,
}
impl Flexpwm3IppIndPwmaSelectInput0Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm3IppIndPwmaSelectInput0Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm3IppIndPwmaSelectInput0Daisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm3IppIndPwmaSelectInput0Daisy {
        Flexpwm3IppIndPwmaSelectInput0Daisy::from_bits(val)
    }
}
impl From<Flexpwm3IppIndPwmaSelectInput0Daisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm3IppIndPwmaSelectInput0Daisy) -> u8 {
        Flexpwm3IppIndPwmaSelectInput0Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm3IppIndPwmaSelectInput1Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_32 for Mode: ALT1"]
    SELECT_GPIO_EMC_B1_32_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_02 for Mode: ALT10"]
    SELECT_GPIO_EMC_B2_02_ALT10 = 0x01,
}
impl Flexpwm3IppIndPwmaSelectInput1Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm3IppIndPwmaSelectInput1Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm3IppIndPwmaSelectInput1Daisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm3IppIndPwmaSelectInput1Daisy {
        Flexpwm3IppIndPwmaSelectInput1Daisy::from_bits(val)
    }
}
impl From<Flexpwm3IppIndPwmaSelectInput1Daisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm3IppIndPwmaSelectInput1Daisy) -> u8 {
        Flexpwm3IppIndPwmaSelectInput1Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm3IppIndPwmaSelectInput2Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_35 for Mode: ALT1"]
    SELECT_GPIO_EMC_B1_35_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_05 for Mode: ALT10"]
    SELECT_GPIO_EMC_B2_05_ALT10 = 0x01,
}
impl Flexpwm3IppIndPwmaSelectInput2Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm3IppIndPwmaSelectInput2Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm3IppIndPwmaSelectInput2Daisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm3IppIndPwmaSelectInput2Daisy {
        Flexpwm3IppIndPwmaSelectInput2Daisy::from_bits(val)
    }
}
impl From<Flexpwm3IppIndPwmaSelectInput2Daisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm3IppIndPwmaSelectInput2Daisy) -> u8 {
        Flexpwm3IppIndPwmaSelectInput2Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm3IppIndPwmaSelectInput3Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_11 for Mode: ALT1"]
    SELECT_GPIO_EMC_B1_11_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_07 for Mode: ALT10"]
    SELECT_GPIO_EMC_B2_07_ALT10 = 0x01,
}
impl Flexpwm3IppIndPwmaSelectInput3Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm3IppIndPwmaSelectInput3Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm3IppIndPwmaSelectInput3Daisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm3IppIndPwmaSelectInput3Daisy {
        Flexpwm3IppIndPwmaSelectInput3Daisy::from_bits(val)
    }
}
impl From<Flexpwm3IppIndPwmaSelectInput3Daisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm3IppIndPwmaSelectInput3Daisy) -> u8 {
        Flexpwm3IppIndPwmaSelectInput3Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm3IppIndPwmbSelectInput0Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_31 for Mode: ALT1"]
    SELECT_GPIO_EMC_B1_31_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_01 for Mode: ALT10"]
    SELECT_GPIO_EMC_B2_01_ALT10 = 0x01,
}
impl Flexpwm3IppIndPwmbSelectInput0Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm3IppIndPwmbSelectInput0Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm3IppIndPwmbSelectInput0Daisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm3IppIndPwmbSelectInput0Daisy {
        Flexpwm3IppIndPwmbSelectInput0Daisy::from_bits(val)
    }
}
impl From<Flexpwm3IppIndPwmbSelectInput0Daisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm3IppIndPwmbSelectInput0Daisy) -> u8 {
        Flexpwm3IppIndPwmbSelectInput0Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm3IppIndPwmbSelectInput1Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_33 for Mode: ALT1"]
    SELECT_GPIO_EMC_B1_33_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_03 for Mode: ALT10"]
    SELECT_GPIO_EMC_B2_03_ALT10 = 0x01,
}
impl Flexpwm3IppIndPwmbSelectInput1Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm3IppIndPwmbSelectInput1Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm3IppIndPwmbSelectInput1Daisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm3IppIndPwmbSelectInput1Daisy {
        Flexpwm3IppIndPwmbSelectInput1Daisy::from_bits(val)
    }
}
impl From<Flexpwm3IppIndPwmbSelectInput1Daisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm3IppIndPwmbSelectInput1Daisy) -> u8 {
        Flexpwm3IppIndPwmbSelectInput1Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm3IppIndPwmbSelectInput2Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_34 for Mode: ALT1"]
    SELECT_GPIO_EMC_B1_34_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_04 for Mode: ALT10"]
    SELECT_GPIO_EMC_B2_04_ALT10 = 0x01,
}
impl Flexpwm3IppIndPwmbSelectInput2Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm3IppIndPwmbSelectInput2Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm3IppIndPwmbSelectInput2Daisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm3IppIndPwmbSelectInput2Daisy {
        Flexpwm3IppIndPwmbSelectInput2Daisy::from_bits(val)
    }
}
impl From<Flexpwm3IppIndPwmbSelectInput2Daisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm3IppIndPwmbSelectInput2Daisy) -> u8 {
        Flexpwm3IppIndPwmbSelectInput2Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm3IppIndPwmbSelectInput3Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_10 for Mode: ALT1"]
    SELECT_GPIO_EMC_B1_10_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_06 for Mode: ALT10"]
    SELECT_GPIO_EMC_B2_06_ALT10 = 0x01,
}
impl Flexpwm3IppIndPwmbSelectInput3Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm3IppIndPwmbSelectInput3Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm3IppIndPwmbSelectInput3Daisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm3IppIndPwmbSelectInput3Daisy {
        Flexpwm3IppIndPwmbSelectInput3Daisy::from_bits(val)
    }
}
impl From<Flexpwm3IppIndPwmbSelectInput3Daisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm3IppIndPwmbSelectInput3Daisy) -> u8 {
        Flexpwm3IppIndPwmbSelectInput3Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi1Bus2bitIppIndDqsFaSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B2_12_DUMMY for Mode: ALT0"]
    SELECT_GPIO_SD_B2_12_DUMMY_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_07 for Mode: ALT7"]
    SELECT_GPIO_B2_07_ALT7 = 0x01,
}
impl Flexspi1Bus2bitIppIndDqsFaSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi1Bus2bitIppIndDqsFaSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi1Bus2bitIppIndDqsFaSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexspi1Bus2bitIppIndDqsFaSelectInputDaisy {
        Flexspi1Bus2bitIppIndDqsFaSelectInputDaisy::from_bits(val)
    }
}
impl From<Flexspi1Bus2bitIppIndDqsFaSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexspi1Bus2bitIppIndDqsFaSelectInputDaisy) -> u8 {
        Flexspi1Bus2bitIppIndDqsFaSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi1Bus2bitIppIndDqsFbSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B2_05 for Mode: ALT1"]
    SELECT_GPIO_SD_B2_05_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_12_DUMMY for Mode: ALT1"]
    SELECT_GPIO_SD_B2_12_DUMMY_ALT1 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_03 for Mode: ALT7"]
    SELECT_GPIO_B1_03_ALT7 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Flexspi1Bus2bitIppIndDqsFbSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi1Bus2bitIppIndDqsFbSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi1Bus2bitIppIndDqsFbSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexspi1Bus2bitIppIndDqsFbSelectInputDaisy {
        Flexspi1Bus2bitIppIndDqsFbSelectInputDaisy::from_bits(val)
    }
}
impl From<Flexspi1Bus2bitIppIndDqsFbSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexspi1Bus2bitIppIndDqsFbSelectInputDaisy) -> u8 {
        Flexspi1Bus2bitIppIndDqsFbSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi1Bus2bitIppIndIoFbBit0SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B2_08 for Mode: ALT1"]
    SELECT_GPIO_SD_B2_08_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_13 for Mode: ALT7"]
    SELECT_GPIO_B1_13_ALT7 = 0x01,
}
impl Flexspi1Bus2bitIppIndIoFbBit0SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi1Bus2bitIppIndIoFbBit0SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi1Bus2bitIppIndIoFbBit0SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexspi1Bus2bitIppIndIoFbBit0SelectInputDaisy {
        Flexspi1Bus2bitIppIndIoFbBit0SelectInputDaisy::from_bits(val)
    }
}
impl From<Flexspi1Bus2bitIppIndIoFbBit0SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexspi1Bus2bitIppIndIoFbBit0SelectInputDaisy) -> u8 {
        Flexspi1Bus2bitIppIndIoFbBit0SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi1Bus2bitIppIndIoFbBit1SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B2_09 for Mode: ALT1"]
    SELECT_GPIO_SD_B2_09_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_12 for Mode: ALT7"]
    SELECT_GPIO_B1_12_ALT7 = 0x01,
}
impl Flexspi1Bus2bitIppIndIoFbBit1SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi1Bus2bitIppIndIoFbBit1SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi1Bus2bitIppIndIoFbBit1SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexspi1Bus2bitIppIndIoFbBit1SelectInputDaisy {
        Flexspi1Bus2bitIppIndIoFbBit1SelectInputDaisy::from_bits(val)
    }
}
impl From<Flexspi1Bus2bitIppIndIoFbBit1SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexspi1Bus2bitIppIndIoFbBit1SelectInputDaisy) -> u8 {
        Flexspi1Bus2bitIppIndIoFbBit1SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi1Bus2bitIppIndIoFbBit2SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B2_10 for Mode: ALT1"]
    SELECT_GPIO_SD_B2_10_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_11 for Mode: ALT7"]
    SELECT_GPIO_B1_11_ALT7 = 0x01,
}
impl Flexspi1Bus2bitIppIndIoFbBit2SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi1Bus2bitIppIndIoFbBit2SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi1Bus2bitIppIndIoFbBit2SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexspi1Bus2bitIppIndIoFbBit2SelectInputDaisy {
        Flexspi1Bus2bitIppIndIoFbBit2SelectInputDaisy::from_bits(val)
    }
}
impl From<Flexspi1Bus2bitIppIndIoFbBit2SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexspi1Bus2bitIppIndIoFbBit2SelectInputDaisy) -> u8 {
        Flexspi1Bus2bitIppIndIoFbBit2SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi1Bus2bitIppIndIoFbBit3SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B2_11 for Mode: ALT1"]
    SELECT_GPIO_SD_B2_11_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_10 for Mode: ALT7"]
    SELECT_GPIO_B1_10_ALT7 = 0x01,
}
impl Flexspi1Bus2bitIppIndIoFbBit3SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi1Bus2bitIppIndIoFbBit3SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi1Bus2bitIppIndIoFbBit3SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexspi1Bus2bitIppIndIoFbBit3SelectInputDaisy {
        Flexspi1Bus2bitIppIndIoFbBit3SelectInputDaisy::from_bits(val)
    }
}
impl From<Flexspi1Bus2bitIppIndIoFbBit3SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexspi1Bus2bitIppIndIoFbBit3SelectInputDaisy) -> u8 {
        Flexspi1Bus2bitIppIndIoFbBit3SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi1Bus2bitIppIndIoFbBit4SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B2_00 for Mode: ALT1"]
    SELECT_GPIO_SD_B2_00_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_09 for Mode: ALT7"]
    SELECT_GPIO_B1_09_ALT7 = 0x01,
}
impl Flexspi1Bus2bitIppIndIoFbBit4SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi1Bus2bitIppIndIoFbBit4SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi1Bus2bitIppIndIoFbBit4SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexspi1Bus2bitIppIndIoFbBit4SelectInputDaisy {
        Flexspi1Bus2bitIppIndIoFbBit4SelectInputDaisy::from_bits(val)
    }
}
impl From<Flexspi1Bus2bitIppIndIoFbBit4SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexspi1Bus2bitIppIndIoFbBit4SelectInputDaisy) -> u8 {
        Flexspi1Bus2bitIppIndIoFbBit4SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi1Bus2bitIppIndIoFbBit5SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B2_01 for Mode: ALT1"]
    SELECT_GPIO_SD_B2_01_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_08 for Mode: ALT7"]
    SELECT_GPIO_B1_08_ALT7 = 0x01,
}
impl Flexspi1Bus2bitIppIndIoFbBit5SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi1Bus2bitIppIndIoFbBit5SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi1Bus2bitIppIndIoFbBit5SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexspi1Bus2bitIppIndIoFbBit5SelectInputDaisy {
        Flexspi1Bus2bitIppIndIoFbBit5SelectInputDaisy::from_bits(val)
    }
}
impl From<Flexspi1Bus2bitIppIndIoFbBit5SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexspi1Bus2bitIppIndIoFbBit5SelectInputDaisy) -> u8 {
        Flexspi1Bus2bitIppIndIoFbBit5SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi1Bus2bitIppIndIoFbBit6SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B2_02 for Mode: ALT1"]
    SELECT_GPIO_SD_B2_02_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_07 for Mode: ALT7"]
    SELECT_GPIO_B1_07_ALT7 = 0x01,
}
impl Flexspi1Bus2bitIppIndIoFbBit6SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi1Bus2bitIppIndIoFbBit6SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi1Bus2bitIppIndIoFbBit6SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexspi1Bus2bitIppIndIoFbBit6SelectInputDaisy {
        Flexspi1Bus2bitIppIndIoFbBit6SelectInputDaisy::from_bits(val)
    }
}
impl From<Flexspi1Bus2bitIppIndIoFbBit6SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexspi1Bus2bitIppIndIoFbBit6SelectInputDaisy) -> u8 {
        Flexspi1Bus2bitIppIndIoFbBit6SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi1Bus2bitIppIndIoFbBit7SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B2_03 for Mode: ALT1"]
    SELECT_GPIO_SD_B2_03_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_06 for Mode: ALT7"]
    SELECT_GPIO_B1_06_ALT7 = 0x01,
}
impl Flexspi1Bus2bitIppIndIoFbBit7SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi1Bus2bitIppIndIoFbBit7SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi1Bus2bitIppIndIoFbBit7SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexspi1Bus2bitIppIndIoFbBit7SelectInputDaisy {
        Flexspi1Bus2bitIppIndIoFbBit7SelectInputDaisy::from_bits(val)
    }
}
impl From<Flexspi1Bus2bitIppIndIoFbBit7SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexspi1Bus2bitIppIndIoFbBit7SelectInputDaisy) -> u8 {
        Flexspi1Bus2bitIppIndIoFbBit7SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi1Bus2bitIppIndSckFbSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B2_07 for Mode: ALT1"]
    SELECT_GPIO_SD_B2_07_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_05 for Mode: ALT7"]
    SELECT_GPIO_B1_05_ALT7 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_02 for Mode: ALT6"]
    SELECT_GPIO_B2_02_ALT6 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Flexspi1Bus2bitIppIndSckFbSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi1Bus2bitIppIndSckFbSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi1Bus2bitIppIndSckFbSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexspi1Bus2bitIppIndSckFbSelectInputDaisy {
        Flexspi1Bus2bitIppIndSckFbSelectInputDaisy::from_bits(val)
    }
}
impl From<Flexspi1Bus2bitIppIndSckFbSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexspi1Bus2bitIppIndSckFbSelectInputDaisy) -> u8 {
        Flexspi1Bus2bitIppIndSckFbSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi2Bus2bitIppIndDqsFaSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_40 for Mode: ALT3"]
    SELECT_GPIO_EMC_B1_40_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_21 for Mode: ALT8"]
    SELECT_GPIO_AON_21_ALT8 = 0x01,
    #[doc = "Selecting Pad: GPIO_AON_28_DUMMY for Mode: ALT0"]
    SELECT_GPIO_AON_28_DUMMY_ALT0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Flexspi2Bus2bitIppIndDqsFaSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi2Bus2bitIppIndDqsFaSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi2Bus2bitIppIndDqsFaSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexspi2Bus2bitIppIndDqsFaSelectInputDaisy {
        Flexspi2Bus2bitIppIndDqsFaSelectInputDaisy::from_bits(val)
    }
}
impl From<Flexspi2Bus2bitIppIndDqsFaSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexspi2Bus2bitIppIndDqsFaSelectInputDaisy) -> u8 {
        Flexspi2Bus2bitIppIndDqsFaSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi2Bus2bitIppIndDqsFbSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_21 for Mode: ALT10"]
    SELECT_GPIO_EMC_B1_21_ALT10 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_29 for Mode: ALT3"]
    SELECT_GPIO_EMC_B1_29_ALT3 = 0x01,
    #[doc = "Selecting Pad: GPIO_AON_20 for Mode: ALT0"]
    SELECT_GPIO_AON_20_ALT0 = 0x02,
    #[doc = "Selecting Pad: GPIO_AON_28_DUMMY for Mode: ALT1"]
    SELECT_GPIO_AON_28_DUMMY_ALT1 = 0x03,
}
impl Flexspi2Bus2bitIppIndDqsFbSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi2Bus2bitIppIndDqsFbSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi2Bus2bitIppIndDqsFbSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexspi2Bus2bitIppIndDqsFbSelectInputDaisy {
        Flexspi2Bus2bitIppIndDqsFbSelectInputDaisy::from_bits(val)
    }
}
impl From<Flexspi2Bus2bitIppIndDqsFbSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexspi2Bus2bitIppIndDqsFbSelectInputDaisy) -> u8 {
        Flexspi2Bus2bitIppIndDqsFbSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi2Bus2bitIppIndIoFaBit0SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_35 for Mode: ALT3"]
    SELECT_GPIO_EMC_B1_35_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_24 for Mode: ALT0"]
    SELECT_GPIO_AON_24_ALT0 = 0x01,
}
impl Flexspi2Bus2bitIppIndIoFaBit0SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi2Bus2bitIppIndIoFaBit0SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi2Bus2bitIppIndIoFaBit0SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexspi2Bus2bitIppIndIoFaBit0SelectInputDaisy {
        Flexspi2Bus2bitIppIndIoFaBit0SelectInputDaisy::from_bits(val)
    }
}
impl From<Flexspi2Bus2bitIppIndIoFaBit0SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexspi2Bus2bitIppIndIoFaBit0SelectInputDaisy) -> u8 {
        Flexspi2Bus2bitIppIndIoFaBit0SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi2Bus2bitIppIndIoFaBit1SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_36 for Mode: ALT3"]
    SELECT_GPIO_EMC_B1_36_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_25 for Mode: ALT0"]
    SELECT_GPIO_AON_25_ALT0 = 0x01,
}
impl Flexspi2Bus2bitIppIndIoFaBit1SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi2Bus2bitIppIndIoFaBit1SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi2Bus2bitIppIndIoFaBit1SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexspi2Bus2bitIppIndIoFaBit1SelectInputDaisy {
        Flexspi2Bus2bitIppIndIoFaBit1SelectInputDaisy::from_bits(val)
    }
}
impl From<Flexspi2Bus2bitIppIndIoFaBit1SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexspi2Bus2bitIppIndIoFaBit1SelectInputDaisy) -> u8 {
        Flexspi2Bus2bitIppIndIoFaBit1SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi2Bus2bitIppIndIoFaBit2SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_37 for Mode: ALT3"]
    SELECT_GPIO_EMC_B1_37_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_26 for Mode: ALT0"]
    SELECT_GPIO_AON_26_ALT0 = 0x01,
}
impl Flexspi2Bus2bitIppIndIoFaBit2SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi2Bus2bitIppIndIoFaBit2SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi2Bus2bitIppIndIoFaBit2SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexspi2Bus2bitIppIndIoFaBit2SelectInputDaisy {
        Flexspi2Bus2bitIppIndIoFaBit2SelectInputDaisy::from_bits(val)
    }
}
impl From<Flexspi2Bus2bitIppIndIoFaBit2SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexspi2Bus2bitIppIndIoFaBit2SelectInputDaisy) -> u8 {
        Flexspi2Bus2bitIppIndIoFaBit2SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi2Bus2bitIppIndIoFaBit3SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_38 for Mode: ALT3"]
    SELECT_GPIO_EMC_B1_38_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_27 for Mode: ALT0"]
    SELECT_GPIO_AON_27_ALT0 = 0x01,
}
impl Flexspi2Bus2bitIppIndIoFaBit3SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi2Bus2bitIppIndIoFaBit3SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi2Bus2bitIppIndIoFaBit3SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexspi2Bus2bitIppIndIoFaBit3SelectInputDaisy {
        Flexspi2Bus2bitIppIndIoFaBit3SelectInputDaisy::from_bits(val)
    }
}
impl From<Flexspi2Bus2bitIppIndIoFaBit3SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexspi2Bus2bitIppIndIoFaBit3SelectInputDaisy) -> u8 {
        Flexspi2Bus2bitIppIndIoFaBit3SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi2Bus2bitIppIndIoFbBit0SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_25 for Mode: ALT10"]
    SELECT_GPIO_EMC_B1_25_ALT10 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_33 for Mode: ALT3"]
    SELECT_GPIO_EMC_B1_33_ALT3 = 0x01,
    #[doc = "Selecting Pad: GPIO_AON_18 for Mode: ALT0"]
    SELECT_GPIO_AON_18_ALT0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Flexspi2Bus2bitIppIndIoFbBit0SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi2Bus2bitIppIndIoFbBit0SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi2Bus2bitIppIndIoFbBit0SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexspi2Bus2bitIppIndIoFbBit0SelectInputDaisy {
        Flexspi2Bus2bitIppIndIoFbBit0SelectInputDaisy::from_bits(val)
    }
}
impl From<Flexspi2Bus2bitIppIndIoFbBit0SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexspi2Bus2bitIppIndIoFbBit0SelectInputDaisy) -> u8 {
        Flexspi2Bus2bitIppIndIoFbBit0SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi2Bus2bitIppIndIoFbBit1SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_24 for Mode: ALT10"]
    SELECT_GPIO_EMC_B1_24_ALT10 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_32 for Mode: ALT3"]
    SELECT_GPIO_EMC_B1_32_ALT3 = 0x01,
    #[doc = "Selecting Pad: GPIO_AON_17 for Mode: ALT0"]
    SELECT_GPIO_AON_17_ALT0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Flexspi2Bus2bitIppIndIoFbBit1SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi2Bus2bitIppIndIoFbBit1SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi2Bus2bitIppIndIoFbBit1SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexspi2Bus2bitIppIndIoFbBit1SelectInputDaisy {
        Flexspi2Bus2bitIppIndIoFbBit1SelectInputDaisy::from_bits(val)
    }
}
impl From<Flexspi2Bus2bitIppIndIoFbBit1SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexspi2Bus2bitIppIndIoFbBit1SelectInputDaisy) -> u8 {
        Flexspi2Bus2bitIppIndIoFbBit1SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi2Bus2bitIppIndIoFbBit2SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_23 for Mode: ALT10"]
    SELECT_GPIO_EMC_B1_23_ALT10 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_31 for Mode: ALT3"]
    SELECT_GPIO_EMC_B1_31_ALT3 = 0x01,
    #[doc = "Selecting Pad: GPIO_AON_16 for Mode: ALT0"]
    SELECT_GPIO_AON_16_ALT0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Flexspi2Bus2bitIppIndIoFbBit2SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi2Bus2bitIppIndIoFbBit2SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi2Bus2bitIppIndIoFbBit2SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexspi2Bus2bitIppIndIoFbBit2SelectInputDaisy {
        Flexspi2Bus2bitIppIndIoFbBit2SelectInputDaisy::from_bits(val)
    }
}
impl From<Flexspi2Bus2bitIppIndIoFbBit2SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexspi2Bus2bitIppIndIoFbBit2SelectInputDaisy) -> u8 {
        Flexspi2Bus2bitIppIndIoFbBit2SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi2Bus2bitIppIndIoFbBit3SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_22 for Mode: ALT10"]
    SELECT_GPIO_EMC_B1_22_ALT10 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_30 for Mode: ALT3"]
    SELECT_GPIO_EMC_B1_30_ALT3 = 0x01,
    #[doc = "Selecting Pad: GPIO_AON_15 for Mode: ALT0"]
    SELECT_GPIO_AON_15_ALT0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Flexspi2Bus2bitIppIndIoFbBit3SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi2Bus2bitIppIndIoFbBit3SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi2Bus2bitIppIndIoFbBit3SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexspi2Bus2bitIppIndIoFbBit3SelectInputDaisy {
        Flexspi2Bus2bitIppIndIoFbBit3SelectInputDaisy::from_bits(val)
    }
}
impl From<Flexspi2Bus2bitIppIndIoFbBit3SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexspi2Bus2bitIppIndIoFbBit3SelectInputDaisy) -> u8 {
        Flexspi2Bus2bitIppIndIoFbBit3SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi2Bus2bitIppIndSckFaSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_41 for Mode: ALT3"]
    SELECT_GPIO_EMC_B1_41_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_23 for Mode: ALT0"]
    SELECT_GPIO_AON_23_ALT0 = 0x01,
}
impl Flexspi2Bus2bitIppIndSckFaSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi2Bus2bitIppIndSckFaSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi2Bus2bitIppIndSckFaSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexspi2Bus2bitIppIndSckFaSelectInputDaisy {
        Flexspi2Bus2bitIppIndSckFaSelectInputDaisy::from_bits(val)
    }
}
impl From<Flexspi2Bus2bitIppIndSckFaSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexspi2Bus2bitIppIndSckFaSelectInputDaisy) -> u8 {
        Flexspi2Bus2bitIppIndSckFaSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi2Bus2bitIppIndSckFbSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_34 for Mode: ALT3"]
    SELECT_GPIO_EMC_B1_34_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_19 for Mode: ALT0"]
    SELECT_GPIO_AON_19_ALT0 = 0x01,
}
impl Flexspi2Bus2bitIppIndSckFbSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi2Bus2bitIppIndSckFbSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi2Bus2bitIppIndSckFbSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexspi2Bus2bitIppIndSckFbSelectInputDaisy {
        Flexspi2Bus2bitIppIndSckFbSelectInputDaisy::from_bits(val)
    }
}
impl From<Flexspi2Bus2bitIppIndSckFbSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexspi2Bus2bitIppIndSckFbSelectInputDaisy) -> u8 {
        Flexspi2Bus2bitIppIndSckFbSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c2PinSclInSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_18 for Mode: ALT6"]
    SELECT_GPIO_AD_18_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_34 for Mode: ALT0"]
    SELECT_GPIO_AD_34_ALT0 = 0x01,
}
impl I3c2PinSclInSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c2PinSclInSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c2PinSclInSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> I3c2PinSclInSelectInputDaisy {
        I3c2PinSclInSelectInputDaisy::from_bits(val)
    }
}
impl From<I3c2PinSclInSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: I3c2PinSclInSelectInputDaisy) -> u8 {
        I3c2PinSclInSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c2PinSdaInSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_19 for Mode: ALT6"]
    SELECT_GPIO_AD_19_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_35 for Mode: ALT0"]
    SELECT_GPIO_AD_35_ALT0 = 0x01,
}
impl I3c2PinSdaInSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c2PinSdaInSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c2PinSdaInSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> I3c2PinSdaInSelectInputDaisy {
        I3c2PinSdaInSelectInputDaisy::from_bits(val)
    }
}
impl From<I3c2PinSdaInSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: I3c2PinSdaInSelectInputDaisy) -> u8 {
        I3c2PinSdaInSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IbeOff {
    #[doc = "Disabled"]
    IBE_OFF_0_DISABLED = 0x0,
    #[doc = "Enabled"]
    IBE_OFF_1_ENABLED = 0x01,
}
impl IbeOff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IbeOff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IbeOff {
    #[inline(always)]
    fn from(val: u8) -> IbeOff {
        IbeOff::from_bits(val)
    }
}
impl From<IbeOff> for u8 {
    #[inline(always)]
    fn from(val: IbeOff) -> u8 {
        IbeOff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KppIppIndColSelectInput0Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_15 for Mode: ALT6"]
    SELECT_GPIO_EMC_B1_15_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_27 for Mode: ALT6"]
    SELECT_GPIO_AD_27_ALT6 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_B2_03 for Mode: ALT4"]
    SELECT_GPIO_SD_B2_03_ALT4 = 0x02,
    _RESERVED_3 = 0x03,
}
impl KppIppIndColSelectInput0Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KppIppIndColSelectInput0Daisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KppIppIndColSelectInput0Daisy {
    #[inline(always)]
    fn from(val: u8) -> KppIppIndColSelectInput0Daisy {
        KppIppIndColSelectInput0Daisy::from_bits(val)
    }
}
impl From<KppIppIndColSelectInput0Daisy> for u8 {
    #[inline(always)]
    fn from(val: KppIppIndColSelectInput0Daisy) -> u8 {
        KppIppIndColSelectInput0Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KppIppIndColSelectInput1Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_13 for Mode: ALT6"]
    SELECT_GPIO_EMC_B1_13_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_33 for Mode: ALT6"]
    SELECT_GPIO_AD_33_ALT6 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_B2_01 for Mode: ALT4"]
    SELECT_GPIO_SD_B2_01_ALT4 = 0x02,
    _RESERVED_3 = 0x03,
}
impl KppIppIndColSelectInput1Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KppIppIndColSelectInput1Daisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KppIppIndColSelectInput1Daisy {
    #[inline(always)]
    fn from(val: u8) -> KppIppIndColSelectInput1Daisy {
        KppIppIndColSelectInput1Daisy::from_bits(val)
    }
}
impl From<KppIppIndColSelectInput1Daisy> for u8 {
    #[inline(always)]
    fn from(val: KppIppIndColSelectInput1Daisy) -> u8 {
        KppIppIndColSelectInput1Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KppIppIndColSelectInput2Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_03 for Mode: ALT6"]
    SELECT_GPIO_EMC_B1_03_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_31 for Mode: ALT6"]
    SELECT_GPIO_AD_31_ALT6 = 0x01,
}
impl KppIppIndColSelectInput2Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KppIppIndColSelectInput2Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KppIppIndColSelectInput2Daisy {
    #[inline(always)]
    fn from(val: u8) -> KppIppIndColSelectInput2Daisy {
        KppIppIndColSelectInput2Daisy::from_bits(val)
    }
}
impl From<KppIppIndColSelectInput2Daisy> for u8 {
    #[inline(always)]
    fn from(val: KppIppIndColSelectInput2Daisy) -> u8 {
        KppIppIndColSelectInput2Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KppIppIndColSelectInput3Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_01 for Mode: ALT6"]
    SELECT_GPIO_EMC_B1_01_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_29 for Mode: ALT6"]
    SELECT_GPIO_AD_29_ALT6 = 0x01,
}
impl KppIppIndColSelectInput3Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KppIppIndColSelectInput3Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KppIppIndColSelectInput3Daisy {
    #[inline(always)]
    fn from(val: u8) -> KppIppIndColSelectInput3Daisy {
        KppIppIndColSelectInput3Daisy::from_bits(val)
    }
}
impl From<KppIppIndColSelectInput3Daisy> for u8 {
    #[inline(always)]
    fn from(val: KppIppIndColSelectInput3Daisy) -> u8 {
        KppIppIndColSelectInput3Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KppIppIndColSelectInput4Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_12 for Mode: ALT6"]
    SELECT_GPIO_EMC_B1_12_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_19 for Mode: ALT3"]
    SELECT_GPIO_AD_19_ALT3 = 0x01,
}
impl KppIppIndColSelectInput4Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KppIppIndColSelectInput4Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KppIppIndColSelectInput4Daisy {
    #[inline(always)]
    fn from(val: u8) -> KppIppIndColSelectInput4Daisy {
        KppIppIndColSelectInput4Daisy::from_bits(val)
    }
}
impl From<KppIppIndColSelectInput4Daisy> for u8 {
    #[inline(always)]
    fn from(val: KppIppIndColSelectInput4Daisy) -> u8 {
        KppIppIndColSelectInput4Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KppIppIndColSelectInput5Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_10 for Mode: ALT6"]
    SELECT_GPIO_EMC_B1_10_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_17 for Mode: ALT3"]
    SELECT_GPIO_AD_17_ALT3 = 0x01,
}
impl KppIppIndColSelectInput5Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KppIppIndColSelectInput5Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KppIppIndColSelectInput5Daisy {
    #[inline(always)]
    fn from(val: u8) -> KppIppIndColSelectInput5Daisy {
        KppIppIndColSelectInput5Daisy::from_bits(val)
    }
}
impl From<KppIppIndColSelectInput5Daisy> for u8 {
    #[inline(always)]
    fn from(val: KppIppIndColSelectInput5Daisy) -> u8 {
        KppIppIndColSelectInput5Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KppIppIndColSelectInput6Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_08 for Mode: ALT6"]
    SELECT_GPIO_EMC_B1_08_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_15 for Mode: ALT3"]
    SELECT_GPIO_AD_15_ALT3 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_B1_03 for Mode: ALT8"]
    SELECT_GPIO_SD_B1_03_ALT8 = 0x02,
    _RESERVED_3 = 0x03,
}
impl KppIppIndColSelectInput6Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KppIppIndColSelectInput6Daisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KppIppIndColSelectInput6Daisy {
    #[inline(always)]
    fn from(val: u8) -> KppIppIndColSelectInput6Daisy {
        KppIppIndColSelectInput6Daisy::from_bits(val)
    }
}
impl From<KppIppIndColSelectInput6Daisy> for u8 {
    #[inline(always)]
    fn from(val: KppIppIndColSelectInput6Daisy) -> u8 {
        KppIppIndColSelectInput6Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KppIppIndColSelectInput7Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_06 for Mode: ALT6"]
    SELECT_GPIO_EMC_B1_06_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_13 for Mode: ALT3"]
    SELECT_GPIO_AD_13_ALT3 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_B1_01 for Mode: ALT8"]
    SELECT_GPIO_SD_B1_01_ALT8 = 0x02,
    _RESERVED_3 = 0x03,
}
impl KppIppIndColSelectInput7Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KppIppIndColSelectInput7Daisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KppIppIndColSelectInput7Daisy {
    #[inline(always)]
    fn from(val: u8) -> KppIppIndColSelectInput7Daisy {
        KppIppIndColSelectInput7Daisy::from_bits(val)
    }
}
impl From<KppIppIndColSelectInput7Daisy> for u8 {
    #[inline(always)]
    fn from(val: KppIppIndColSelectInput7Daisy) -> u8 {
        KppIppIndColSelectInput7Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KppIppIndRowSelectInput0Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_14 for Mode: ALT6"]
    SELECT_GPIO_EMC_B1_14_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_26 for Mode: ALT6"]
    SELECT_GPIO_AD_26_ALT6 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_B2_02 for Mode: ALT4"]
    SELECT_GPIO_SD_B2_02_ALT4 = 0x02,
    _RESERVED_3 = 0x03,
}
impl KppIppIndRowSelectInput0Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KppIppIndRowSelectInput0Daisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KppIppIndRowSelectInput0Daisy {
    #[inline(always)]
    fn from(val: u8) -> KppIppIndRowSelectInput0Daisy {
        KppIppIndRowSelectInput0Daisy::from_bits(val)
    }
}
impl From<KppIppIndRowSelectInput0Daisy> for u8 {
    #[inline(always)]
    fn from(val: KppIppIndRowSelectInput0Daisy) -> u8 {
        KppIppIndRowSelectInput0Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KppIppIndRowSelectInput1Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_04 for Mode: ALT6"]
    SELECT_GPIO_EMC_B1_04_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_32 for Mode: ALT6"]
    SELECT_GPIO_AD_32_ALT6 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_B2_00 for Mode: ALT4"]
    SELECT_GPIO_SD_B2_00_ALT4 = 0x02,
    _RESERVED_3 = 0x03,
}
impl KppIppIndRowSelectInput1Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KppIppIndRowSelectInput1Daisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KppIppIndRowSelectInput1Daisy {
    #[inline(always)]
    fn from(val: u8) -> KppIppIndRowSelectInput1Daisy {
        KppIppIndRowSelectInput1Daisy::from_bits(val)
    }
}
impl From<KppIppIndRowSelectInput1Daisy> for u8 {
    #[inline(always)]
    fn from(val: KppIppIndRowSelectInput1Daisy) -> u8 {
        KppIppIndRowSelectInput1Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KppIppIndRowSelectInput2Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_02 for Mode: ALT6"]
    SELECT_GPIO_EMC_B1_02_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_30 for Mode: ALT6"]
    SELECT_GPIO_AD_30_ALT6 = 0x01,
}
impl KppIppIndRowSelectInput2Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KppIppIndRowSelectInput2Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KppIppIndRowSelectInput2Daisy {
    #[inline(always)]
    fn from(val: u8) -> KppIppIndRowSelectInput2Daisy {
        KppIppIndRowSelectInput2Daisy::from_bits(val)
    }
}
impl From<KppIppIndRowSelectInput2Daisy> for u8 {
    #[inline(always)]
    fn from(val: KppIppIndRowSelectInput2Daisy) -> u8 {
        KppIppIndRowSelectInput2Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KppIppIndRowSelectInput3Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_00 for Mode: ALT6"]
    SELECT_GPIO_EMC_B1_00_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_28 for Mode: ALT6"]
    SELECT_GPIO_AD_28_ALT6 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_B2_04 for Mode: ALT4"]
    SELECT_GPIO_SD_B2_04_ALT4 = 0x02,
    _RESERVED_3 = 0x03,
}
impl KppIppIndRowSelectInput3Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KppIppIndRowSelectInput3Daisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KppIppIndRowSelectInput3Daisy {
    #[inline(always)]
    fn from(val: u8) -> KppIppIndRowSelectInput3Daisy {
        KppIppIndRowSelectInput3Daisy::from_bits(val)
    }
}
impl From<KppIppIndRowSelectInput3Daisy> for u8 {
    #[inline(always)]
    fn from(val: KppIppIndRowSelectInput3Daisy) -> u8 {
        KppIppIndRowSelectInput3Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KppIppIndRowSelectInput4Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_11 for Mode: ALT6"]
    SELECT_GPIO_EMC_B1_11_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_18 for Mode: ALT3"]
    SELECT_GPIO_AD_18_ALT3 = 0x01,
}
impl KppIppIndRowSelectInput4Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KppIppIndRowSelectInput4Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KppIppIndRowSelectInput4Daisy {
    #[inline(always)]
    fn from(val: u8) -> KppIppIndRowSelectInput4Daisy {
        KppIppIndRowSelectInput4Daisy::from_bits(val)
    }
}
impl From<KppIppIndRowSelectInput4Daisy> for u8 {
    #[inline(always)]
    fn from(val: KppIppIndRowSelectInput4Daisy) -> u8 {
        KppIppIndRowSelectInput4Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KppIppIndRowSelectInput5Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_09 for Mode: ALT6"]
    SELECT_GPIO_EMC_B1_09_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_16 for Mode: ALT3"]
    SELECT_GPIO_AD_16_ALT3 = 0x01,
}
impl KppIppIndRowSelectInput5Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KppIppIndRowSelectInput5Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KppIppIndRowSelectInput5Daisy {
    #[inline(always)]
    fn from(val: u8) -> KppIppIndRowSelectInput5Daisy {
        KppIppIndRowSelectInput5Daisy::from_bits(val)
    }
}
impl From<KppIppIndRowSelectInput5Daisy> for u8 {
    #[inline(always)]
    fn from(val: KppIppIndRowSelectInput5Daisy) -> u8 {
        KppIppIndRowSelectInput5Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KppIppIndRowSelectInput6Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_07 for Mode: ALT6"]
    SELECT_GPIO_EMC_B1_07_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_14 for Mode: ALT3"]
    SELECT_GPIO_AD_14_ALT3 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_B1_02 for Mode: ALT8"]
    SELECT_GPIO_SD_B1_02_ALT8 = 0x02,
    _RESERVED_3 = 0x03,
}
impl KppIppIndRowSelectInput6Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KppIppIndRowSelectInput6Daisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KppIppIndRowSelectInput6Daisy {
    #[inline(always)]
    fn from(val: u8) -> KppIppIndRowSelectInput6Daisy {
        KppIppIndRowSelectInput6Daisy::from_bits(val)
    }
}
impl From<KppIppIndRowSelectInput6Daisy> for u8 {
    #[inline(always)]
    fn from(val: KppIppIndRowSelectInput6Daisy) -> u8 {
        KppIppIndRowSelectInput6Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KppIppIndRowSelectInput7Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_05 for Mode: ALT6"]
    SELECT_GPIO_EMC_B1_05_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_12 for Mode: ALT3"]
    SELECT_GPIO_AD_12_ALT3 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_B1_00 for Mode: ALT8"]
    SELECT_GPIO_SD_B1_00_ALT8 = 0x02,
    _RESERVED_3 = 0x03,
}
impl KppIppIndRowSelectInput7Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KppIppIndRowSelectInput7Daisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KppIppIndRowSelectInput7Daisy {
    #[inline(always)]
    fn from(val: u8) -> KppIppIndRowSelectInput7Daisy {
        KppIppIndRowSelectInput7Daisy::from_bits(val)
    }
}
impl From<KppIppIndRowSelectInput7Daisy> for u8 {
    #[inline(always)]
    fn from(val: KppIppIndRowSelectInput7Daisy) -> u8 {
        KppIppIndRowSelectInput7Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c3IppIndLpi2cSclSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_00 for Mode: ALT9"]
    SELECT_GPIO_EMC_B2_00_ALT9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_19 for Mode: ALT8"]
    SELECT_GPIO_EMC_B2_19_ALT8 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_18 for Mode: ALT9"]
    SELECT_GPIO_AD_18_ALT9 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpi2c3IppIndLpi2cSclSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c3IppIndLpi2cSclSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c3IppIndLpi2cSclSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c3IppIndLpi2cSclSelectInputDaisy {
        Lpi2c3IppIndLpi2cSclSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpi2c3IppIndLpi2cSclSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c3IppIndLpi2cSclSelectInputDaisy) -> u8 {
        Lpi2c3IppIndLpi2cSclSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c3IppIndLpi2cSdaSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_01 for Mode: ALT9"]
    SELECT_GPIO_EMC_B2_01_ALT9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_20 for Mode: ALT8"]
    SELECT_GPIO_EMC_B2_20_ALT8 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_19 for Mode: ALT9"]
    SELECT_GPIO_AD_19_ALT9 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpi2c3IppIndLpi2cSdaSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c3IppIndLpi2cSdaSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c3IppIndLpi2cSdaSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c3IppIndLpi2cSdaSelectInputDaisy {
        Lpi2c3IppIndLpi2cSdaSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpi2c3IppIndLpi2cSdaSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c3IppIndLpi2cSdaSelectInputDaisy) -> u8 {
        Lpi2c3IppIndLpi2cSdaSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c4IppIndLpi2cSclSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_24 for Mode: ALT1"]
    SELECT_GPIO_AD_24_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_10 for Mode: ALT6"]
    SELECT_GPIO_B2_10_ALT6 = 0x01,
}
impl Lpi2c4IppIndLpi2cSclSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c4IppIndLpi2cSclSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c4IppIndLpi2cSclSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c4IppIndLpi2cSclSelectInputDaisy {
        Lpi2c4IppIndLpi2cSclSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpi2c4IppIndLpi2cSclSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c4IppIndLpi2cSclSelectInputDaisy) -> u8 {
        Lpi2c4IppIndLpi2cSclSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c4IppIndLpi2cSdaSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_25 for Mode: ALT1"]
    SELECT_GPIO_AD_25_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_11 for Mode: ALT6"]
    SELECT_GPIO_B2_11_ALT6 = 0x01,
}
impl Lpi2c4IppIndLpi2cSdaSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c4IppIndLpi2cSdaSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c4IppIndLpi2cSdaSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c4IppIndLpi2cSdaSelectInputDaisy {
        Lpi2c4IppIndLpi2cSdaSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpi2c4IppIndLpi2cSdaSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c4IppIndLpi2cSdaSelectInputDaisy) -> u8 {
        Lpi2c4IppIndLpi2cSdaSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c5IppIndLpi2cSclSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_08 for Mode: ALT1"]
    SELECT_GPIO_AD_08_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_32 for Mode: ALT0"]
    SELECT_GPIO_AD_32_ALT0 = 0x01,
}
impl Lpi2c5IppIndLpi2cSclSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c5IppIndLpi2cSclSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c5IppIndLpi2cSclSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c5IppIndLpi2cSclSelectInputDaisy {
        Lpi2c5IppIndLpi2cSclSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpi2c5IppIndLpi2cSclSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c5IppIndLpi2cSclSelectInputDaisy) -> u8 {
        Lpi2c5IppIndLpi2cSclSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c5IppIndLpi2cSdaSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_09 for Mode: ALT1"]
    SELECT_GPIO_AD_09_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_33 for Mode: ALT0"]
    SELECT_GPIO_AD_33_ALT0 = 0x01,
}
impl Lpi2c5IppIndLpi2cSdaSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c5IppIndLpi2cSdaSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c5IppIndLpi2cSdaSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c5IppIndLpi2cSdaSelectInputDaisy {
        Lpi2c5IppIndLpi2cSdaSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpi2c5IppIndLpi2cSdaSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c5IppIndLpi2cSdaSelectInputDaisy) -> u8 {
        Lpi2c5IppIndLpi2cSdaSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c6IppIndLpi2cSclSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_B1_02 for Mode: ALT2"]
    SELECT_GPIO_B1_02_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_08 for Mode: ALT6"]
    SELECT_GPIO_B2_08_ALT6 = 0x01,
}
impl Lpi2c6IppIndLpi2cSclSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c6IppIndLpi2cSclSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c6IppIndLpi2cSclSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c6IppIndLpi2cSclSelectInputDaisy {
        Lpi2c6IppIndLpi2cSclSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpi2c6IppIndLpi2cSclSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c6IppIndLpi2cSclSelectInputDaisy) -> u8 {
        Lpi2c6IppIndLpi2cSclSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c6IppIndLpi2cSdaSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_B1_03 for Mode: ALT2"]
    SELECT_GPIO_B1_03_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_09 for Mode: ALT6"]
    SELECT_GPIO_B2_09_ALT6 = 0x01,
}
impl Lpi2c6IppIndLpi2cSdaSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c6IppIndLpi2cSdaSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c6IppIndLpi2cSdaSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c6IppIndLpi2cSdaSelectInputDaisy {
        Lpi2c6IppIndLpi2cSdaSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpi2c6IppIndLpi2cSdaSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c6IppIndLpi2cSdaSelectInputDaisy) -> u8 {
        Lpi2c6IppIndLpi2cSdaSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi3IppIndLpspiPcsSelectInput0Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_05 for Mode: ALT8"]
    SELECT_GPIO_EMC_B2_05_ALT8 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_17 for Mode: ALT7"]
    SELECT_GPIO_AD_17_ALT7 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_B1_00 for Mode: ALT6"]
    SELECT_GPIO_SD_B1_00_ALT6 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpspi3IppIndLpspiPcsSelectInput0Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi3IppIndLpspiPcsSelectInput0Daisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi3IppIndLpspiPcsSelectInput0Daisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi3IppIndLpspiPcsSelectInput0Daisy {
        Lpspi3IppIndLpspiPcsSelectInput0Daisy::from_bits(val)
    }
}
impl From<Lpspi3IppIndLpspiPcsSelectInput0Daisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi3IppIndLpspiPcsSelectInput0Daisy) -> u8 {
        Lpspi3IppIndLpspiPcsSelectInput0Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi3IppIndLpspiPcsSelectInput1Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_10 for Mode: ALT8"]
    SELECT_GPIO_EMC_B2_10_ALT8 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_15 for Mode: ALT7"]
    SELECT_GPIO_AD_15_ALT7 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_B1_04 for Mode: ALT6"]
    SELECT_GPIO_SD_B1_04_ALT6 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpspi3IppIndLpspiPcsSelectInput1Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi3IppIndLpspiPcsSelectInput1Daisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi3IppIndLpspiPcsSelectInput1Daisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi3IppIndLpspiPcsSelectInput1Daisy {
        Lpspi3IppIndLpspiPcsSelectInput1Daisy::from_bits(val)
    }
}
impl From<Lpspi3IppIndLpspiPcsSelectInput1Daisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi3IppIndLpspiPcsSelectInput1Daisy) -> u8 {
        Lpspi3IppIndLpspiPcsSelectInput1Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi3IppIndLpspiPcsSelectInput2Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_09 for Mode: ALT8"]
    SELECT_GPIO_EMC_B2_09_ALT8 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B1_05 for Mode: ALT6"]
    SELECT_GPIO_SD_B1_05_ALT6 = 0x01,
}
impl Lpspi3IppIndLpspiPcsSelectInput2Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi3IppIndLpspiPcsSelectInput2Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi3IppIndLpspiPcsSelectInput2Daisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi3IppIndLpspiPcsSelectInput2Daisy {
        Lpspi3IppIndLpspiPcsSelectInput2Daisy::from_bits(val)
    }
}
impl From<Lpspi3IppIndLpspiPcsSelectInput2Daisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi3IppIndLpspiPcsSelectInput2Daisy) -> u8 {
        Lpspi3IppIndLpspiPcsSelectInput2Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi3IppIndLpspiPcsSelectInput3Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_08 for Mode: ALT8"]
    SELECT_GPIO_EMC_B2_08_ALT8 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_00 for Mode: ALT6"]
    SELECT_GPIO_SD_B2_00_ALT6 = 0x01,
}
impl Lpspi3IppIndLpspiPcsSelectInput3Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi3IppIndLpspiPcsSelectInput3Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi3IppIndLpspiPcsSelectInput3Daisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi3IppIndLpspiPcsSelectInput3Daisy {
        Lpspi3IppIndLpspiPcsSelectInput3Daisy::from_bits(val)
    }
}
impl From<Lpspi3IppIndLpspiPcsSelectInput3Daisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi3IppIndLpspiPcsSelectInput3Daisy) -> u8 {
        Lpspi3IppIndLpspiPcsSelectInput3Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi3IppIndLpspiSckSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_04 for Mode: ALT8"]
    SELECT_GPIO_EMC_B2_04_ALT8 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_16 for Mode: ALT7"]
    SELECT_GPIO_AD_16_ALT7 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_B1_01 for Mode: ALT6"]
    SELECT_GPIO_SD_B1_01_ALT6 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpspi3IppIndLpspiSckSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi3IppIndLpspiSckSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi3IppIndLpspiSckSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi3IppIndLpspiSckSelectInputDaisy {
        Lpspi3IppIndLpspiSckSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpspi3IppIndLpspiSckSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi3IppIndLpspiSckSelectInputDaisy) -> u8 {
        Lpspi3IppIndLpspiSckSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi3IppIndLpspiSdiSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_07 for Mode: ALT8"]
    SELECT_GPIO_EMC_B2_07_ALT8 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_19 for Mode: ALT7"]
    SELECT_GPIO_AD_19_ALT7 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_B1_03 for Mode: ALT6"]
    SELECT_GPIO_SD_B1_03_ALT6 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpspi3IppIndLpspiSdiSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi3IppIndLpspiSdiSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi3IppIndLpspiSdiSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi3IppIndLpspiSdiSelectInputDaisy {
        Lpspi3IppIndLpspiSdiSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpspi3IppIndLpspiSdiSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi3IppIndLpspiSdiSelectInputDaisy) -> u8 {
        Lpspi3IppIndLpspiSdiSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi3IppIndLpspiSdoSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_06 for Mode: ALT8"]
    SELECT_GPIO_EMC_B2_06_ALT8 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_18 for Mode: ALT7"]
    SELECT_GPIO_AD_18_ALT7 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_B1_02 for Mode: ALT6"]
    SELECT_GPIO_SD_B1_02_ALT6 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpspi3IppIndLpspiSdoSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi3IppIndLpspiSdoSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi3IppIndLpspiSdoSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi3IppIndLpspiSdoSelectInputDaisy {
        Lpspi3IppIndLpspiSdoSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpspi3IppIndLpspiSdoSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi3IppIndLpspiSdoSelectInputDaisy) -> u8 {
        Lpspi3IppIndLpspiSdoSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi4IppIndLpspiPcsSelectInput0Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_25 for Mode: ALT3"]
    SELECT_GPIO_EMC_B1_25_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_09 for Mode: ALT4"]
    SELECT_GPIO_SD_B2_09_ALT4 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_13 for Mode: ALT9"]
    SELECT_GPIO_B2_13_ALT9 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpspi4IppIndLpspiPcsSelectInput0Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi4IppIndLpspiPcsSelectInput0Daisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi4IppIndLpspiPcsSelectInput0Daisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi4IppIndLpspiPcsSelectInput0Daisy {
        Lpspi4IppIndLpspiPcsSelectInput0Daisy::from_bits(val)
    }
}
impl From<Lpspi4IppIndLpspiPcsSelectInput0Daisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi4IppIndLpspiPcsSelectInput0Daisy) -> u8 {
        Lpspi4IppIndLpspiPcsSelectInput0Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi4IppIndLpspiSckSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_22 for Mode: ALT3"]
    SELECT_GPIO_EMC_B1_22_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_08 for Mode: ALT4"]
    SELECT_GPIO_SD_B2_08_ALT4 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_10 for Mode: ALT9"]
    SELECT_GPIO_B2_10_ALT9 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpspi4IppIndLpspiSckSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi4IppIndLpspiSckSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi4IppIndLpspiSckSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi4IppIndLpspiSckSelectInputDaisy {
        Lpspi4IppIndLpspiSckSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpspi4IppIndLpspiSckSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi4IppIndLpspiSckSelectInputDaisy) -> u8 {
        Lpspi4IppIndLpspiSckSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi4IppIndLpspiSdiSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_23 for Mode: ALT3"]
    SELECT_GPIO_EMC_B1_23_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_11 for Mode: ALT4"]
    SELECT_GPIO_SD_B2_11_ALT4 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_11 for Mode: ALT9"]
    SELECT_GPIO_B2_11_ALT9 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpspi4IppIndLpspiSdiSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi4IppIndLpspiSdiSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi4IppIndLpspiSdiSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi4IppIndLpspiSdiSelectInputDaisy {
        Lpspi4IppIndLpspiSdiSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpspi4IppIndLpspiSdiSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi4IppIndLpspiSdiSelectInputDaisy) -> u8 {
        Lpspi4IppIndLpspiSdiSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi4IppIndLpspiSdoSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_24 for Mode: ALT3"]
    SELECT_GPIO_EMC_B1_24_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_10 for Mode: ALT4"]
    SELECT_GPIO_SD_B2_10_ALT4 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_12 for Mode: ALT9"]
    SELECT_GPIO_B2_12_ALT9 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpspi4IppIndLpspiSdoSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi4IppIndLpspiSdoSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi4IppIndLpspiSdoSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi4IppIndLpspiSdoSelectInputDaisy {
        Lpspi4IppIndLpspiSdoSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpspi4IppIndLpspiSdoSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi4IppIndLpspiSdoSelectInputDaisy) -> u8 {
        Lpspi4IppIndLpspiSdoSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi5IppIndLpspiPcsSelectInput0Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_34 for Mode: ALT10"]
    SELECT_GPIO_EMC_B1_34_ALT10 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_01 for Mode: ALT8"]
    SELECT_GPIO_EMC_B2_01_ALT8 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_29 for Mode: ALT0"]
    SELECT_GPIO_AD_29_ALT0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpspi5IppIndLpspiPcsSelectInput0Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi5IppIndLpspiPcsSelectInput0Daisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi5IppIndLpspiPcsSelectInput0Daisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi5IppIndLpspiPcsSelectInput0Daisy {
        Lpspi5IppIndLpspiPcsSelectInput0Daisy::from_bits(val)
    }
}
impl From<Lpspi5IppIndLpspiPcsSelectInput0Daisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi5IppIndLpspiPcsSelectInput0Daisy) -> u8 {
        Lpspi5IppIndLpspiPcsSelectInput0Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi5IppIndLpspiPcsSelectInput1Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_35 for Mode: ALT10"]
    SELECT_GPIO_EMC_B1_35_ALT10 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_13 for Mode: ALT4"]
    SELECT_GPIO_EMC_B2_13_ALT4 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_27 for Mode: ALT2"]
    SELECT_GPIO_AD_27_ALT2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpspi5IppIndLpspiPcsSelectInput1Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi5IppIndLpspiPcsSelectInput1Daisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi5IppIndLpspiPcsSelectInput1Daisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi5IppIndLpspiPcsSelectInput1Daisy {
        Lpspi5IppIndLpspiPcsSelectInput1Daisy::from_bits(val)
    }
}
impl From<Lpspi5IppIndLpspiPcsSelectInput1Daisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi5IppIndLpspiPcsSelectInput1Daisy) -> u8 {
        Lpspi5IppIndLpspiPcsSelectInput1Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi5IppIndLpspiPcsSelectInput2Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_12 for Mode: ALT4"]
    SELECT_GPIO_EMC_B2_12_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_26 for Mode: ALT2"]
    SELECT_GPIO_AD_26_ALT2 = 0x01,
}
impl Lpspi5IppIndLpspiPcsSelectInput2Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi5IppIndLpspiPcsSelectInput2Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi5IppIndLpspiPcsSelectInput2Daisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi5IppIndLpspiPcsSelectInput2Daisy {
        Lpspi5IppIndLpspiPcsSelectInput2Daisy::from_bits(val)
    }
}
impl From<Lpspi5IppIndLpspiPcsSelectInput2Daisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi5IppIndLpspiPcsSelectInput2Daisy) -> u8 {
        Lpspi5IppIndLpspiPcsSelectInput2Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi5IppIndLpspiPcsSelectInput3Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_11 for Mode: ALT4"]
    SELECT_GPIO_EMC_B2_11_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_25 for Mode: ALT2"]
    SELECT_GPIO_AD_25_ALT2 = 0x01,
}
impl Lpspi5IppIndLpspiPcsSelectInput3Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi5IppIndLpspiPcsSelectInput3Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi5IppIndLpspiPcsSelectInput3Daisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi5IppIndLpspiPcsSelectInput3Daisy {
        Lpspi5IppIndLpspiPcsSelectInput3Daisy::from_bits(val)
    }
}
impl From<Lpspi5IppIndLpspiPcsSelectInput3Daisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi5IppIndLpspiPcsSelectInput3Daisy) -> u8 {
        Lpspi5IppIndLpspiPcsSelectInput3Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi5IppIndLpspiSckSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_31 for Mode: ALT9"]
    SELECT_GPIO_EMC_B1_31_ALT9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_00 for Mode: ALT8"]
    SELECT_GPIO_EMC_B2_00_ALT8 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_28 for Mode: ALT0"]
    SELECT_GPIO_AD_28_ALT0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpspi5IppIndLpspiSckSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi5IppIndLpspiSckSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi5IppIndLpspiSckSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi5IppIndLpspiSckSelectInputDaisy {
        Lpspi5IppIndLpspiSckSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpspi5IppIndLpspiSckSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi5IppIndLpspiSckSelectInputDaisy) -> u8 {
        Lpspi5IppIndLpspiSckSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi5IppIndLpspiSdiSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_33 for Mode: ALT9"]
    SELECT_GPIO_EMC_B1_33_ALT9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_03 for Mode: ALT8"]
    SELECT_GPIO_EMC_B2_03_ALT8 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_31 for Mode: ALT0"]
    SELECT_GPIO_AD_31_ALT0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpspi5IppIndLpspiSdiSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi5IppIndLpspiSdiSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi5IppIndLpspiSdiSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi5IppIndLpspiSdiSelectInputDaisy {
        Lpspi5IppIndLpspiSdiSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpspi5IppIndLpspiSdiSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi5IppIndLpspiSdiSelectInputDaisy) -> u8 {
        Lpspi5IppIndLpspiSdiSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi5IppIndLpspiSdoSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_32 for Mode: ALT9"]
    SELECT_GPIO_EMC_B1_32_ALT9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_02 for Mode: ALT8"]
    SELECT_GPIO_EMC_B2_02_ALT8 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_30 for Mode: ALT0"]
    SELECT_GPIO_AD_30_ALT0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpspi5IppIndLpspiSdoSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi5IppIndLpspiSdoSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi5IppIndLpspiSdoSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi5IppIndLpspiSdoSelectInputDaisy {
        Lpspi5IppIndLpspiSdoSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpspi5IppIndLpspiSdoSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi5IppIndLpspiSdoSelectInputDaisy) -> u8 {
        Lpspi5IppIndLpspiSdoSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi6IppIndLpspiPcsSelectInput0Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_21 for Mode: ALT3"]
    SELECT_GPIO_EMC_B1_21_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_29 for Mode: ALT10"]
    SELECT_GPIO_EMC_B1_29_ALT10 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_12 for Mode: ALT9"]
    SELECT_GPIO_B1_12_ALT9 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpspi6IppIndLpspiPcsSelectInput0Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi6IppIndLpspiPcsSelectInput0Daisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi6IppIndLpspiPcsSelectInput0Daisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi6IppIndLpspiPcsSelectInput0Daisy {
        Lpspi6IppIndLpspiPcsSelectInput0Daisy::from_bits(val)
    }
}
impl From<Lpspi6IppIndLpspiPcsSelectInput0Daisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi6IppIndLpspiPcsSelectInput0Daisy) -> u8 {
        Lpspi6IppIndLpspiPcsSelectInput0Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi6IppIndLpspiPcsSelectInput1Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_17 for Mode: ALT10"]
    SELECT_GPIO_EMC_B1_17_ALT10 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_30 for Mode: ALT10"]
    SELECT_GPIO_EMC_B1_30_ALT10 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_09 for Mode: ALT9"]
    SELECT_GPIO_B1_09_ALT9 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpspi6IppIndLpspiPcsSelectInput1Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi6IppIndLpspiPcsSelectInput1Daisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi6IppIndLpspiPcsSelectInput1Daisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi6IppIndLpspiPcsSelectInput1Daisy {
        Lpspi6IppIndLpspiPcsSelectInput1Daisy::from_bits(val)
    }
}
impl From<Lpspi6IppIndLpspiPcsSelectInput1Daisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi6IppIndLpspiPcsSelectInput1Daisy) -> u8 {
        Lpspi6IppIndLpspiPcsSelectInput1Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi6IppIndLpspiPcsSelectInput2Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_16 for Mode: ALT10"]
    SELECT_GPIO_EMC_B1_16_ALT10 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_31 for Mode: ALT10"]
    SELECT_GPIO_EMC_B1_31_ALT10 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_10 for Mode: ALT9"]
    SELECT_GPIO_B1_10_ALT9 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpspi6IppIndLpspiPcsSelectInput2Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi6IppIndLpspiPcsSelectInput2Daisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi6IppIndLpspiPcsSelectInput2Daisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi6IppIndLpspiPcsSelectInput2Daisy {
        Lpspi6IppIndLpspiPcsSelectInput2Daisy::from_bits(val)
    }
}
impl From<Lpspi6IppIndLpspiPcsSelectInput2Daisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi6IppIndLpspiPcsSelectInput2Daisy) -> u8 {
        Lpspi6IppIndLpspiPcsSelectInput2Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi6IppIndLpspiPcsSelectInput3Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_32 for Mode: ALT10"]
    SELECT_GPIO_EMC_B1_32_ALT10 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_11 for Mode: ALT9"]
    SELECT_GPIO_B1_11_ALT9 = 0x01,
}
impl Lpspi6IppIndLpspiPcsSelectInput3Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi6IppIndLpspiPcsSelectInput3Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi6IppIndLpspiPcsSelectInput3Daisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi6IppIndLpspiPcsSelectInput3Daisy {
        Lpspi6IppIndLpspiPcsSelectInput3Daisy::from_bits(val)
    }
}
impl From<Lpspi6IppIndLpspiPcsSelectInput3Daisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi6IppIndLpspiPcsSelectInput3Daisy) -> u8 {
        Lpspi6IppIndLpspiPcsSelectInput3Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi6IppIndLpspiSckSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_18 for Mode: ALT3"]
    SELECT_GPIO_EMC_B1_18_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_26 for Mode: ALT10"]
    SELECT_GPIO_EMC_B1_26_ALT10 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_13 for Mode: ALT9"]
    SELECT_GPIO_B1_13_ALT9 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpspi6IppIndLpspiSckSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi6IppIndLpspiSckSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi6IppIndLpspiSckSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi6IppIndLpspiSckSelectInputDaisy {
        Lpspi6IppIndLpspiSckSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpspi6IppIndLpspiSckSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi6IppIndLpspiSckSelectInputDaisy) -> u8 {
        Lpspi6IppIndLpspiSckSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi6IppIndLpspiSdiSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_19 for Mode: ALT3"]
    SELECT_GPIO_EMC_B1_19_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_27 for Mode: ALT10"]
    SELECT_GPIO_EMC_B1_27_ALT10 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_07 for Mode: ALT9"]
    SELECT_GPIO_B1_07_ALT9 = 0x02,
    #[doc = "Selecting Pad: GPIO_B2_00 for Mode: ALT9"]
    SELECT_GPIO_B2_00_ALT9 = 0x03,
}
impl Lpspi6IppIndLpspiSdiSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi6IppIndLpspiSdiSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi6IppIndLpspiSdiSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi6IppIndLpspiSdiSelectInputDaisy {
        Lpspi6IppIndLpspiSdiSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpspi6IppIndLpspiSdiSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi6IppIndLpspiSdiSelectInputDaisy) -> u8 {
        Lpspi6IppIndLpspiSdiSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi6IppIndLpspiSdoSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_20 for Mode: ALT3"]
    SELECT_GPIO_EMC_B1_20_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_28 for Mode: ALT10"]
    SELECT_GPIO_EMC_B1_28_ALT10 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_08 for Mode: ALT9"]
    SELECT_GPIO_B1_08_ALT9 = 0x02,
    #[doc = "Selecting Pad: GPIO_B2_01 for Mode: ALT9"]
    SELECT_GPIO_B2_01_ALT9 = 0x03,
}
impl Lpspi6IppIndLpspiSdoSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi6IppIndLpspiSdoSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi6IppIndLpspiSdoSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi6IppIndLpspiSdoSelectInputDaisy {
        Lpspi6IppIndLpspiSdoSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpspi6IppIndLpspiSdoSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi6IppIndLpspiSdoSelectInputDaisy) -> u8 {
        Lpspi6IppIndLpspiSdoSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart10IppIndLpuartRxdSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_16 for Mode: ALT1"]
    SELECT_GPIO_AD_16_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_33 for Mode: ALT8"]
    SELECT_GPIO_AD_33_ALT8 = 0x01,
}
impl Lpuart10IppIndLpuartRxdSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart10IppIndLpuartRxdSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart10IppIndLpuartRxdSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart10IppIndLpuartRxdSelectInputDaisy {
        Lpuart10IppIndLpuartRxdSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart10IppIndLpuartRxdSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart10IppIndLpuartRxdSelectInputDaisy) -> u8 {
        Lpuart10IppIndLpuartRxdSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart10IppIndLpuartTxdSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_15 for Mode: ALT1"]
    SELECT_GPIO_AD_15_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_32 for Mode: ALT8"]
    SELECT_GPIO_AD_32_ALT8 = 0x01,
}
impl Lpuart10IppIndLpuartTxdSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart10IppIndLpuartTxdSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart10IppIndLpuartTxdSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart10IppIndLpuartTxdSelectInputDaisy {
        Lpuart10IppIndLpuartTxdSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart10IppIndLpuartTxdSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart10IppIndLpuartTxdSelectInputDaisy) -> u8 {
        Lpuart10IppIndLpuartTxdSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart11IppIndLpuartRxdSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_14 for Mode: ALT2"]
    SELECT_GPIO_EMC_B2_14_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_03 for Mode: ALT9"]
    SELECT_GPIO_B1_03_ALT9 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_07 for Mode: ALT9"]
    SELECT_GPIO_B2_07_ALT9 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpuart11IppIndLpuartRxdSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart11IppIndLpuartRxdSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart11IppIndLpuartRxdSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart11IppIndLpuartRxdSelectInputDaisy {
        Lpuart11IppIndLpuartRxdSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart11IppIndLpuartRxdSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart11IppIndLpuartRxdSelectInputDaisy) -> u8 {
        Lpuart11IppIndLpuartRxdSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart11IppIndLpuartTxdSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_13 for Mode: ALT2"]
    SELECT_GPIO_EMC_B2_13_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_02 for Mode: ALT9"]
    SELECT_GPIO_B1_02_ALT9 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_06 for Mode: ALT9"]
    SELECT_GPIO_B2_06_ALT9 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpuart11IppIndLpuartTxdSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart11IppIndLpuartTxdSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart11IppIndLpuartTxdSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart11IppIndLpuartTxdSelectInputDaisy {
        Lpuart11IppIndLpuartTxdSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart11IppIndLpuartTxdSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart11IppIndLpuartTxdSelectInputDaisy) -> u8 {
        Lpuart11IppIndLpuartTxdSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart3IppIndLpuartCtsNSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_00 for Mode: ALT3"]
    SELECT_GPIO_EMC_B1_00_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_15 for Mode: ALT6"]
    SELECT_GPIO_AD_15_ALT6 = 0x01,
}
impl Lpuart3IppIndLpuartCtsNSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart3IppIndLpuartCtsNSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart3IppIndLpuartCtsNSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart3IppIndLpuartCtsNSelectInputDaisy {
        Lpuart3IppIndLpuartCtsNSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart3IppIndLpuartCtsNSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart3IppIndLpuartCtsNSelectInputDaisy) -> u8 {
        Lpuart3IppIndLpuartCtsNSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart3IppIndLpuartRxdSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_02 for Mode: ALT3"]
    SELECT_GPIO_EMC_B1_02_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_14 for Mode: ALT6"]
    SELECT_GPIO_AD_14_ALT6 = 0x01,
}
impl Lpuart3IppIndLpuartRxdSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart3IppIndLpuartRxdSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart3IppIndLpuartRxdSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart3IppIndLpuartRxdSelectInputDaisy {
        Lpuart3IppIndLpuartRxdSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart3IppIndLpuartRxdSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart3IppIndLpuartRxdSelectInputDaisy) -> u8 {
        Lpuart3IppIndLpuartRxdSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart3IppIndLpuartTxdSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_03 for Mode: ALT3"]
    SELECT_GPIO_EMC_B1_03_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_13 for Mode: ALT6"]
    SELECT_GPIO_AD_13_ALT6 = 0x01,
}
impl Lpuart3IppIndLpuartTxdSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart3IppIndLpuartTxdSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart3IppIndLpuartTxdSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart3IppIndLpuartTxdSelectInputDaisy {
        Lpuart3IppIndLpuartTxdSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart3IppIndLpuartTxdSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart3IppIndLpuartTxdSelectInputDaisy) -> u8 {
        Lpuart3IppIndLpuartTxdSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart4IppIndLpuartCtsNSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_14 for Mode: ALT10"]
    SELECT_GPIO_EMC_B1_14_ALT10 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_21 for Mode: ALT9"]
    SELECT_GPIO_EMC_B1_21_ALT9 = 0x01,
}
impl Lpuart4IppIndLpuartCtsNSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart4IppIndLpuartCtsNSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart4IppIndLpuartCtsNSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart4IppIndLpuartCtsNSelectInputDaisy {
        Lpuart4IppIndLpuartCtsNSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart4IppIndLpuartCtsNSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart4IppIndLpuartCtsNSelectInputDaisy) -> u8 {
        Lpuart4IppIndLpuartCtsNSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart5IppIndLpuartCtsNSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_37 for Mode: ALT2"]
    SELECT_GPIO_EMC_B1_37_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_19 for Mode: ALT2"]
    SELECT_GPIO_EMC_B2_19_ALT2 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_B2_06 for Mode: ALT6"]
    SELECT_GPIO_SD_B2_06_ALT6 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpuart5IppIndLpuartCtsNSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart5IppIndLpuartCtsNSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart5IppIndLpuartCtsNSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart5IppIndLpuartCtsNSelectInputDaisy {
        Lpuart5IppIndLpuartCtsNSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart5IppIndLpuartCtsNSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart5IppIndLpuartCtsNSelectInputDaisy) -> u8 {
        Lpuart5IppIndLpuartCtsNSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart5IppIndLpuartDcdNSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_15 for Mode: ALT4"]
    SELECT_GPIO_EMC_B2_15_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_10 for Mode: ALT6"]
    SELECT_GPIO_SD_B2_10_ALT6 = 0x01,
}
impl Lpuart5IppIndLpuartDcdNSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart5IppIndLpuartDcdNSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart5IppIndLpuartDcdNSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart5IppIndLpuartDcdNSelectInputDaisy {
        Lpuart5IppIndLpuartDcdNSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart5IppIndLpuartDcdNSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart5IppIndLpuartDcdNSelectInputDaisy) -> u8 {
        Lpuart5IppIndLpuartDcdNSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart5IppIndLpuartDsrNSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_14 for Mode: ALT4"]
    SELECT_GPIO_EMC_B2_14_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_11 for Mode: ALT6"]
    SELECT_GPIO_SD_B2_11_ALT6 = 0x01,
}
impl Lpuart5IppIndLpuartDsrNSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart5IppIndLpuartDsrNSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart5IppIndLpuartDsrNSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart5IppIndLpuartDsrNSelectInputDaisy {
        Lpuart5IppIndLpuartDsrNSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart5IppIndLpuartDsrNSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart5IppIndLpuartDsrNSelectInputDaisy) -> u8 {
        Lpuart5IppIndLpuartDsrNSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart5IppIndLpuartRiNSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_18 for Mode: ALT2"]
    SELECT_GPIO_AD_18_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_04 for Mode: ALT6"]
    SELECT_GPIO_SD_B2_04_ALT6 = 0x01,
}
impl Lpuart5IppIndLpuartRiNSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart5IppIndLpuartRiNSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart5IppIndLpuartRiNSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart5IppIndLpuartRiNSelectInputDaisy {
        Lpuart5IppIndLpuartRiNSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart5IppIndLpuartRiNSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart5IppIndLpuartRiNSelectInputDaisy) -> u8 {
        Lpuart5IppIndLpuartRiNSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart5IppIndLpuartRxdSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_15 for Mode: ALT2"]
    SELECT_GPIO_EMC_B1_15_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_36 for Mode: ALT2"]
    SELECT_GPIO_EMC_B1_36_ALT2 = 0x01,
    #[doc = "Selecting Pad: GPIO_EMC_B2_18 for Mode: ALT2"]
    SELECT_GPIO_EMC_B2_18_ALT2 = 0x02,
    #[doc = "Selecting Pad: GPIO_AD_27 for Mode: ALT1"]
    SELECT_GPIO_AD_27_ALT1 = 0x03,
    #[doc = "Selecting Pad: GPIO_SD_B2_09 for Mode: ALT6"]
    SELECT_GPIO_SD_B2_09_ALT6 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Lpuart5IppIndLpuartRxdSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart5IppIndLpuartRxdSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart5IppIndLpuartRxdSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart5IppIndLpuartRxdSelectInputDaisy {
        Lpuart5IppIndLpuartRxdSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart5IppIndLpuartRxdSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart5IppIndLpuartRxdSelectInputDaisy) -> u8 {
        Lpuart5IppIndLpuartRxdSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart5IppIndLpuartTxdSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_14 for Mode: ALT2"]
    SELECT_GPIO_EMC_B1_14_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_35 for Mode: ALT2"]
    SELECT_GPIO_EMC_B1_35_ALT2 = 0x01,
    #[doc = "Selecting Pad: GPIO_EMC_B2_17 for Mode: ALT2"]
    SELECT_GPIO_EMC_B2_17_ALT2 = 0x02,
    #[doc = "Selecting Pad: GPIO_AD_26 for Mode: ALT1"]
    SELECT_GPIO_AD_26_ALT1 = 0x03,
    #[doc = "Selecting Pad: GPIO_SD_B2_08 for Mode: ALT6"]
    SELECT_GPIO_SD_B2_08_ALT6 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Lpuart5IppIndLpuartTxdSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart5IppIndLpuartTxdSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart5IppIndLpuartTxdSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart5IppIndLpuartTxdSelectInputDaisy {
        Lpuart5IppIndLpuartTxdSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart5IppIndLpuartTxdSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart5IppIndLpuartTxdSelectInputDaisy) -> u8 {
        Lpuart5IppIndLpuartTxdSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart6IppIndLpuartCtsNSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_33 for Mode: ALT2"]
    SELECT_GPIO_EMC_B1_33_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_26 for Mode: ALT0"]
    SELECT_GPIO_AD_26_ALT0 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_12 for Mode: ALT4"]
    SELECT_GPIO_B2_12_ALT4 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpuart6IppIndLpuartCtsNSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart6IppIndLpuartCtsNSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart6IppIndLpuartCtsNSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart6IppIndLpuartCtsNSelectInputDaisy {
        Lpuart6IppIndLpuartCtsNSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart6IppIndLpuartCtsNSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart6IppIndLpuartCtsNSelectInputDaisy) -> u8 {
        Lpuart6IppIndLpuartCtsNSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart6IppIndLpuartDcdNSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_29 for Mode: ALT9"]
    SELECT_GPIO_EMC_B1_29_ALT9 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_07 for Mode: ALT3"]
    SELECT_GPIO_B2_07_ALT3 = 0x01,
}
impl Lpuart6IppIndLpuartDcdNSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart6IppIndLpuartDcdNSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart6IppIndLpuartDcdNSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart6IppIndLpuartDcdNSelectInputDaisy {
        Lpuart6IppIndLpuartDcdNSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart6IppIndLpuartDcdNSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart6IppIndLpuartDcdNSelectInputDaisy) -> u8 {
        Lpuart6IppIndLpuartDcdNSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart6IppIndLpuartDsrNSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_30 for Mode: ALT9"]
    SELECT_GPIO_EMC_B1_30_ALT9 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_06 for Mode: ALT3"]
    SELECT_GPIO_B2_06_ALT3 = 0x01,
}
impl Lpuart6IppIndLpuartDsrNSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart6IppIndLpuartDsrNSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart6IppIndLpuartDsrNSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart6IppIndLpuartDsrNSelectInputDaisy {
        Lpuart6IppIndLpuartDsrNSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart6IppIndLpuartDsrNSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart6IppIndLpuartDsrNSelectInputDaisy) -> u8 {
        Lpuart6IppIndLpuartDsrNSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart6IppIndLpuartRiNSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_27 for Mode: ALT9"]
    SELECT_GPIO_EMC_B1_27_ALT9 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_08 for Mode: ALT4"]
    SELECT_GPIO_B2_08_ALT4 = 0x01,
}
impl Lpuart6IppIndLpuartRiNSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart6IppIndLpuartRiNSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart6IppIndLpuartRiNSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart6IppIndLpuartRiNSelectInputDaisy {
        Lpuart6IppIndLpuartRiNSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart6IppIndLpuartRiNSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart6IppIndLpuartRiNSelectInputDaisy) -> u8 {
        Lpuart6IppIndLpuartRiNSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart6IppIndLpuartRxdSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_32 for Mode: ALT2"]
    SELECT_GPIO_EMC_B1_32_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_25 for Mode: ALT0"]
    SELECT_GPIO_AD_25_ALT0 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_11 for Mode: ALT4"]
    SELECT_GPIO_B2_11_ALT4 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpuart6IppIndLpuartRxdSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart6IppIndLpuartRxdSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart6IppIndLpuartRxdSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart6IppIndLpuartRxdSelectInputDaisy {
        Lpuart6IppIndLpuartRxdSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart6IppIndLpuartRxdSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart6IppIndLpuartRxdSelectInputDaisy) -> u8 {
        Lpuart6IppIndLpuartRxdSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart6IppIndLpuartTxdSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_31 for Mode: ALT2"]
    SELECT_GPIO_EMC_B1_31_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_24 for Mode: ALT0"]
    SELECT_GPIO_AD_24_ALT0 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_10 for Mode: ALT4"]
    SELECT_GPIO_B2_10_ALT4 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpuart6IppIndLpuartTxdSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart6IppIndLpuartTxdSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart6IppIndLpuartTxdSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart6IppIndLpuartTxdSelectInputDaisy {
        Lpuart6IppIndLpuartTxdSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart6IppIndLpuartTxdSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart6IppIndLpuartTxdSelectInputDaisy) -> u8 {
        Lpuart6IppIndLpuartTxdSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart8IppIndLpuartCtsNSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B2_02 for Mode: ALT9"]
    SELECT_GPIO_SD_B2_02_ALT9 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_10 for Mode: ALT3"]
    SELECT_GPIO_B2_10_ALT3 = 0x01,
}
impl Lpuart8IppIndLpuartCtsNSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart8IppIndLpuartCtsNSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart8IppIndLpuartCtsNSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart8IppIndLpuartCtsNSelectInputDaisy {
        Lpuart8IppIndLpuartCtsNSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart8IppIndLpuartCtsNSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart8IppIndLpuartCtsNSelectInputDaisy) -> u8 {
        Lpuart8IppIndLpuartCtsNSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart8IppIndLpuartRxdSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_31 for Mode: ALT4"]
    SELECT_GPIO_AD_31_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_01 for Mode: ALT9"]
    SELECT_GPIO_SD_B2_01_ALT9 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_13 for Mode: ALT2"]
    SELECT_GPIO_B2_13_ALT2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpuart8IppIndLpuartRxdSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart8IppIndLpuartRxdSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart8IppIndLpuartRxdSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart8IppIndLpuartRxdSelectInputDaisy {
        Lpuart8IppIndLpuartRxdSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart8IppIndLpuartRxdSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart8IppIndLpuartRxdSelectInputDaisy) -> u8 {
        Lpuart8IppIndLpuartRxdSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart8IppIndLpuartTxdSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_30 for Mode: ALT4"]
    SELECT_GPIO_AD_30_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_00 for Mode: ALT9"]
    SELECT_GPIO_SD_B2_00_ALT9 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_12 for Mode: ALT2"]
    SELECT_GPIO_B2_12_ALT2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpuart8IppIndLpuartTxdSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart8IppIndLpuartTxdSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart8IppIndLpuartTxdSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart8IppIndLpuartTxdSelectInputDaisy {
        Lpuart8IppIndLpuartTxdSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart8IppIndLpuartTxdSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart8IppIndLpuartTxdSelectInputDaisy) -> u8 {
        Lpuart8IppIndLpuartTxdSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart9IppIndLpuartRxdSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_17 for Mode: ALT2"]
    SELECT_GPIO_EMC_B1_17_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_04 for Mode: ALT2"]
    SELECT_GPIO_B1_04_ALT2 = 0x01,
}
impl Lpuart9IppIndLpuartRxdSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart9IppIndLpuartRxdSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart9IppIndLpuartRxdSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart9IppIndLpuartRxdSelectInputDaisy {
        Lpuart9IppIndLpuartRxdSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart9IppIndLpuartRxdSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart9IppIndLpuartRxdSelectInputDaisy) -> u8 {
        Lpuart9IppIndLpuartRxdSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart9IppIndLpuartTxdSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_16 for Mode: ALT2"]
    SELECT_GPIO_EMC_B1_16_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_06 for Mode: ALT2"]
    SELECT_GPIO_B1_06_ALT2 = 0x01,
}
impl Lpuart9IppIndLpuartTxdSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart9IppIndLpuartTxdSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart9IppIndLpuartTxdSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart9IppIndLpuartTxdSelectInputDaisy {
        Lpuart9IppIndLpuartTxdSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart9IppIndLpuartTxdSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart9IppIndLpuartTxdSelectInputDaisy) -> u8 {
        Lpuart9IppIndLpuartTxdSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MicIppIndMicPdmBitstreamSelectInput0Daisy {
    #[doc = "Selecting Pad: GPIO_AD_01 for Mode: ALT2"]
    SELECT_GPIO_AD_01_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_28 for Mode: ALT12"]
    SELECT_GPIO_AD_28_ALT12 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_B2_00 for Mode: ALT12"]
    SELECT_GPIO_SD_B2_00_ALT12 = 0x02,
    #[doc = "Selecting Pad: GPIO_B2_10 for Mode: ALT0"]
    SELECT_GPIO_B2_10_ALT0 = 0x03,
}
impl MicIppIndMicPdmBitstreamSelectInput0Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MicIppIndMicPdmBitstreamSelectInput0Daisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MicIppIndMicPdmBitstreamSelectInput0Daisy {
    #[inline(always)]
    fn from(val: u8) -> MicIppIndMicPdmBitstreamSelectInput0Daisy {
        MicIppIndMicPdmBitstreamSelectInput0Daisy::from_bits(val)
    }
}
impl From<MicIppIndMicPdmBitstreamSelectInput0Daisy> for u8 {
    #[inline(always)]
    fn from(val: MicIppIndMicPdmBitstreamSelectInput0Daisy) -> u8 {
        MicIppIndMicPdmBitstreamSelectInput0Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MicIppIndMicPdmBitstreamSelectInput1Daisy {
    #[doc = "Selecting Pad: GPIO_AD_02 for Mode: ALT2"]
    SELECT_GPIO_AD_02_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_29 for Mode: ALT12"]
    SELECT_GPIO_AD_29_ALT12 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_B2_01 for Mode: ALT12"]
    SELECT_GPIO_SD_B2_01_ALT12 = 0x02,
    #[doc = "Selecting Pad: GPIO_B2_11 for Mode: ALT0"]
    SELECT_GPIO_B2_11_ALT0 = 0x03,
}
impl MicIppIndMicPdmBitstreamSelectInput1Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MicIppIndMicPdmBitstreamSelectInput1Daisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MicIppIndMicPdmBitstreamSelectInput1Daisy {
    #[inline(always)]
    fn from(val: u8) -> MicIppIndMicPdmBitstreamSelectInput1Daisy {
        MicIppIndMicPdmBitstreamSelectInput1Daisy::from_bits(val)
    }
}
impl From<MicIppIndMicPdmBitstreamSelectInput1Daisy> for u8 {
    #[inline(always)]
    fn from(val: MicIppIndMicPdmBitstreamSelectInput1Daisy) -> u8 {
        MicIppIndMicPdmBitstreamSelectInput1Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MicIppIndMicPdmBitstreamSelectInput2Daisy {
    #[doc = "Selecting Pad: GPIO_AD_03 for Mode: ALT2"]
    SELECT_GPIO_AD_03_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_26 for Mode: ALT12"]
    SELECT_GPIO_AD_26_ALT12 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_B2_02 for Mode: ALT12"]
    SELECT_GPIO_SD_B2_02_ALT12 = 0x02,
    #[doc = "Selecting Pad: GPIO_B2_12 for Mode: ALT0"]
    SELECT_GPIO_B2_12_ALT0 = 0x03,
}
impl MicIppIndMicPdmBitstreamSelectInput2Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MicIppIndMicPdmBitstreamSelectInput2Daisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MicIppIndMicPdmBitstreamSelectInput2Daisy {
    #[inline(always)]
    fn from(val: u8) -> MicIppIndMicPdmBitstreamSelectInput2Daisy {
        MicIppIndMicPdmBitstreamSelectInput2Daisy::from_bits(val)
    }
}
impl From<MicIppIndMicPdmBitstreamSelectInput2Daisy> for u8 {
    #[inline(always)]
    fn from(val: MicIppIndMicPdmBitstreamSelectInput2Daisy) -> u8 {
        MicIppIndMicPdmBitstreamSelectInput2Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MicIppIndMicPdmBitstreamSelectInput3Daisy {
    #[doc = "Selecting Pad: GPIO_AD_04 for Mode: ALT2"]
    SELECT_GPIO_AD_04_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_32 for Mode: ALT12"]
    SELECT_GPIO_AD_32_ALT12 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_B2_03 for Mode: ALT12"]
    SELECT_GPIO_SD_B2_03_ALT12 = 0x02,
    #[doc = "Selecting Pad: GPIO_B2_13 for Mode: ALT0"]
    SELECT_GPIO_B2_13_ALT0 = 0x03,
}
impl MicIppIndMicPdmBitstreamSelectInput3Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MicIppIndMicPdmBitstreamSelectInput3Daisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MicIppIndMicPdmBitstreamSelectInput3Daisy {
    #[inline(always)]
    fn from(val: u8) -> MicIppIndMicPdmBitstreamSelectInput3Daisy {
        MicIppIndMicPdmBitstreamSelectInput3Daisy::from_bits(val)
    }
}
impl From<MicIppIndMicPdmBitstreamSelectInput3Daisy> for u8 {
    #[inline(always)]
    fn from(val: MicIppIndMicPdmBitstreamSelectInput3Daisy) -> u8 {
        MicIppIndMicPdmBitstreamSelectInput3Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcClkgenIppTmrClkSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_20 for Mode: ALT7"]
    SELECT_GPIO_AD_20_ALT7 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_00 for Mode: ALT8"]
    SELECT_GPIO_SD_B2_00_ALT8 = 0x01,
}
impl NetcClkgenIppTmrClkSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcClkgenIppTmrClkSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcClkgenIppTmrClkSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> NetcClkgenIppTmrClkSelectInputDaisy {
        NetcClkgenIppTmrClkSelectInputDaisy::from_bits(val)
    }
}
impl From<NetcClkgenIppTmrClkSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: NetcClkgenIppTmrClkSelectInputDaisy) -> u8 {
        NetcClkgenIppTmrClkSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcEmdioInSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_19 for Mode: ALT10"]
    SELECT_GPIO_EMC_B1_19_ALT10 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_41 for Mode: ALT1"]
    SELECT_GPIO_EMC_B1_41_ALT1 = 0x01,
    #[doc = "Selecting Pad: GPIO_EMC_B2_01 for Mode: ALT3"]
    SELECT_GPIO_EMC_B2_01_ALT3 = 0x02,
    #[doc = "Selecting Pad: GPIO_EMC_B2_20 for Mode: ALT4"]
    SELECT_GPIO_EMC_B2_20_ALT4 = 0x03,
    #[doc = "Selecting Pad: GPIO_AD_31 for Mode: ALT7"]
    SELECT_GPIO_AD_31_ALT7 = 0x04,
    #[doc = "Selecting Pad: GPIO_SD_B2_10 for Mode: ALT10"]
    SELECT_GPIO_SD_B2_10_ALT10 = 0x05,
    #[doc = "Selecting Pad: GPIO_B1_12 for Mode: ALT1"]
    SELECT_GPIO_B1_12_ALT1 = 0x06,
    #[doc = "Selecting Pad: GPIO_B2_02 for Mode: ALT3"]
    SELECT_GPIO_B2_02_ALT3 = 0x07,
}
impl NetcEmdioInSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcEmdioInSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcEmdioInSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> NetcEmdioInSelectInputDaisy {
        NetcEmdioInSelectInputDaisy::from_bits(val)
    }
}
impl From<NetcEmdioInSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: NetcEmdioInSelectInputDaisy) -> u8 {
        NetcEmdioInSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcEth2ColSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_19 for Mode: ALT4"]
    SELECT_GPIO_EMC_B1_19_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_41 for Mode: ALT4"]
    SELECT_GPIO_EMC_B1_41_ALT4 = 0x01,
}
impl NetcEth2ColSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcEth2ColSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcEth2ColSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> NetcEth2ColSelectInputDaisy {
        NetcEth2ColSelectInputDaisy::from_bits(val)
    }
}
impl From<NetcEth2ColSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: NetcEth2ColSelectInputDaisy) -> u8 {
        NetcEth2ColSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcEth2CrsSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_18 for Mode: ALT4"]
    SELECT_GPIO_EMC_B1_18_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_40 for Mode: ALT4"]
    SELECT_GPIO_EMC_B1_40_ALT4 = 0x01,
}
impl NetcEth2CrsSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcEth2CrsSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcEth2CrsSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> NetcEth2CrsSelectInputDaisy {
        NetcEth2CrsSelectInputDaisy::from_bits(val)
    }
}
impl From<NetcEth2CrsSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: NetcEth2CrsSelectInputDaisy) -> u8 {
        NetcEth2CrsSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcEth2SlvMdcInSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_16 for Mode: ALT9"]
    SELECT_GPIO_EMC_B1_16_ALT9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_40 for Mode: ALT2"]
    SELECT_GPIO_EMC_B1_40_ALT2 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_01 for Mode: ALT10"]
    SELECT_GPIO_B2_01_ALT10 = 0x02,
    _RESERVED_3 = 0x03,
}
impl NetcEth2SlvMdcInSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcEth2SlvMdcInSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcEth2SlvMdcInSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> NetcEth2SlvMdcInSelectInputDaisy {
        NetcEth2SlvMdcInSelectInputDaisy::from_bits(val)
    }
}
impl From<NetcEth2SlvMdcInSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: NetcEth2SlvMdcInSelectInputDaisy) -> u8 {
        NetcEth2SlvMdcInSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcEth2SlvMdioInSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_17 for Mode: ALT9"]
    SELECT_GPIO_EMC_B1_17_ALT9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_41 for Mode: ALT2"]
    SELECT_GPIO_EMC_B1_41_ALT2 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_00 for Mode: ALT10"]
    SELECT_GPIO_B2_00_ALT10 = 0x02,
    _RESERVED_3 = 0x03,
}
impl NetcEth2SlvMdioInSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcEth2SlvMdioInSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcEth2SlvMdioInSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> NetcEth2SlvMdioInSelectInputDaisy {
        NetcEth2SlvMdioInSelectInputDaisy::from_bits(val)
    }
}
impl From<NetcEth2SlvMdioInSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: NetcEth2SlvMdioInSelectInputDaisy) -> u8 {
        NetcEth2SlvMdioInSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcEth3ColSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_15 for Mode: ALT4"]
    SELECT_GPIO_EMC_B1_15_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_04 for Mode: ALT9"]
    SELECT_GPIO_EMC_B2_04_ALT9 = 0x01,
}
impl NetcEth3ColSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcEth3ColSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcEth3ColSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> NetcEth3ColSelectInputDaisy {
        NetcEth3ColSelectInputDaisy::from_bits(val)
    }
}
impl From<NetcEth3ColSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: NetcEth3ColSelectInputDaisy) -> u8 {
        NetcEth3ColSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcEth3CrsSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_14 for Mode: ALT4"]
    SELECT_GPIO_EMC_B1_14_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_03 for Mode: ALT9"]
    SELECT_GPIO_EMC_B2_03_ALT9 = 0x01,
}
impl NetcEth3CrsSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcEth3CrsSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcEth3CrsSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> NetcEth3CrsSelectInputDaisy {
        NetcEth3CrsSelectInputDaisy::from_bits(val)
    }
}
impl From<NetcEth3CrsSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: NetcEth3CrsSelectInputDaisy) -> u8 {
        NetcEth3CrsSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcEth3SlvMdcInSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_16 for Mode: ALT4"]
    SELECT_GPIO_EMC_B1_16_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_24 for Mode: ALT9"]
    SELECT_GPIO_EMC_B1_24_ALT9 = 0x01,
    #[doc = "Selecting Pad: GPIO_EMC_B2_19 for Mode: ALT9"]
    SELECT_GPIO_EMC_B2_19_ALT9 = 0x02,
    _RESERVED_3 = 0x03,
}
impl NetcEth3SlvMdcInSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcEth3SlvMdcInSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcEth3SlvMdcInSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> NetcEth3SlvMdcInSelectInputDaisy {
        NetcEth3SlvMdcInSelectInputDaisy::from_bits(val)
    }
}
impl From<NetcEth3SlvMdcInSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: NetcEth3SlvMdcInSelectInputDaisy) -> u8 {
        NetcEth3SlvMdcInSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcEth3SlvMdioInSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_17 for Mode: ALT4"]
    SELECT_GPIO_EMC_B1_17_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_25 for Mode: ALT9"]
    SELECT_GPIO_EMC_B1_25_ALT9 = 0x01,
    #[doc = "Selecting Pad: GPIO_EMC_B2_20 for Mode: ALT9"]
    SELECT_GPIO_EMC_B2_20_ALT9 = 0x02,
    _RESERVED_3 = 0x03,
}
impl NetcEth3SlvMdioInSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcEth3SlvMdioInSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcEth3SlvMdioInSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> NetcEth3SlvMdioInSelectInputDaisy {
        NetcEth3SlvMdioInSelectInputDaisy::from_bits(val)
    }
}
impl From<NetcEth3SlvMdioInSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: NetcEth3SlvMdioInSelectInputDaisy) -> u8 {
        NetcEth3SlvMdioInSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcEth4ColSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_15 for Mode: ALT9"]
    SELECT_GPIO_EMC_B1_15_ALT9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_06 for Mode: ALT4"]
    SELECT_GPIO_EMC_B2_06_ALT4 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_01 for Mode: ALT8"]
    SELECT_GPIO_B2_01_ALT8 = 0x02,
    _RESERVED_3 = 0x03,
}
impl NetcEth4ColSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcEth4ColSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcEth4ColSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> NetcEth4ColSelectInputDaisy {
        NetcEth4ColSelectInputDaisy::from_bits(val)
    }
}
impl From<NetcEth4ColSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: NetcEth4ColSelectInputDaisy) -> u8 {
        NetcEth4ColSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcEth4CrsSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_14 for Mode: ALT9"]
    SELECT_GPIO_EMC_B1_14_ALT9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_05 for Mode: ALT4"]
    SELECT_GPIO_EMC_B2_05_ALT4 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_00 for Mode: ALT8"]
    SELECT_GPIO_B2_00_ALT8 = 0x02,
    _RESERVED_3 = 0x03,
}
impl NetcEth4CrsSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcEth4CrsSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcEth4CrsSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> NetcEth4CrsSelectInputDaisy {
        NetcEth4CrsSelectInputDaisy::from_bits(val)
    }
}
impl From<NetcEth4CrsSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: NetcEth4CrsSelectInputDaisy) -> u8 {
        NetcEth4CrsSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcEth4SlvMdcInSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_16 for Mode: ALT6"]
    SELECT_GPIO_EMC_B1_16_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_05 for Mode: ALT1"]
    SELECT_GPIO_EMC_B2_05_ALT1 = 0x01,
}
impl NetcEth4SlvMdcInSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcEth4SlvMdcInSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcEth4SlvMdcInSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> NetcEth4SlvMdcInSelectInputDaisy {
        NetcEth4SlvMdcInSelectInputDaisy::from_bits(val)
    }
}
impl From<NetcEth4SlvMdcInSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: NetcEth4SlvMdcInSelectInputDaisy) -> u8 {
        NetcEth4SlvMdcInSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcEth4SlvMdioInSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_17 for Mode: ALT6"]
    SELECT_GPIO_EMC_B1_17_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_06 for Mode: ALT1"]
    SELECT_GPIO_EMC_B2_06_ALT1 = 0x01,
}
impl NetcEth4SlvMdioInSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcEth4SlvMdioInSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcEth4SlvMdioInSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> NetcEth4SlvMdioInSelectInputDaisy {
        NetcEth4SlvMdioInSelectInputDaisy::from_bits(val)
    }
}
impl From<NetcEth4SlvMdioInSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: NetcEth4SlvMdioInSelectInputDaisy) -> u8 {
        NetcEth4SlvMdioInSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcPinmuxIppIndEth0RxClkSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_00 for Mode: ALT4"]
    SELECT_GPIO_EMC_B2_00_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_15 for Mode: ALT3"]
    SELECT_GPIO_EMC_B2_15_ALT3 = 0x01,
}
impl NetcPinmuxIppIndEth0RxClkSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcPinmuxIppIndEth0RxClkSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcPinmuxIppIndEth0RxClkSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> NetcPinmuxIppIndEth0RxClkSelectInputDaisy {
        NetcPinmuxIppIndEth0RxClkSelectInputDaisy::from_bits(val)
    }
}
impl From<NetcPinmuxIppIndEth0RxClkSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: NetcPinmuxIppIndEth0RxClkSelectInputDaisy) -> u8 {
        NetcPinmuxIppIndEth0RxClkSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcPinmuxIppIndEth0RxDvSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_40 for Mode: ALT9"]
    SELECT_GPIO_EMC_B1_40_ALT9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_11 for Mode: ALT3"]
    SELECT_GPIO_EMC_B2_11_ALT3 = 0x01,
}
impl NetcPinmuxIppIndEth0RxDvSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcPinmuxIppIndEth0RxDvSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcPinmuxIppIndEth0RxDvSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> NetcPinmuxIppIndEth0RxDvSelectInputDaisy {
        NetcPinmuxIppIndEth0RxDvSelectInputDaisy::from_bits(val)
    }
}
impl From<NetcPinmuxIppIndEth0RxDvSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: NetcPinmuxIppIndEth0RxDvSelectInputDaisy) -> u8 {
        NetcPinmuxIppIndEth0RxDvSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcPinmuxIppIndEth0RxErSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_41 for Mode: ALT9"]
    SELECT_GPIO_EMC_B1_41_ALT9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_12 for Mode: ALT3"]
    SELECT_GPIO_EMC_B2_12_ALT3 = 0x01,
}
impl NetcPinmuxIppIndEth0RxErSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcPinmuxIppIndEth0RxErSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcPinmuxIppIndEth0RxErSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> NetcPinmuxIppIndEth0RxErSelectInputDaisy {
        NetcPinmuxIppIndEth0RxErSelectInputDaisy::from_bits(val)
    }
}
impl From<NetcPinmuxIppIndEth0RxErSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: NetcPinmuxIppIndEth0RxErSelectInputDaisy) -> u8 {
        NetcPinmuxIppIndEth0RxErSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcPinmuxIppIndEth0RxdSelectInput0Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_38 for Mode: ALT9"]
    SELECT_GPIO_EMC_B1_38_ALT9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_09 for Mode: ALT3"]
    SELECT_GPIO_EMC_B2_09_ALT3 = 0x01,
}
impl NetcPinmuxIppIndEth0RxdSelectInput0Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcPinmuxIppIndEth0RxdSelectInput0Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcPinmuxIppIndEth0RxdSelectInput0Daisy {
    #[inline(always)]
    fn from(val: u8) -> NetcPinmuxIppIndEth0RxdSelectInput0Daisy {
        NetcPinmuxIppIndEth0RxdSelectInput0Daisy::from_bits(val)
    }
}
impl From<NetcPinmuxIppIndEth0RxdSelectInput0Daisy> for u8 {
    #[inline(always)]
    fn from(val: NetcPinmuxIppIndEth0RxdSelectInput0Daisy) -> u8 {
        NetcPinmuxIppIndEth0RxdSelectInput0Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcPinmuxIppIndEth0RxdSelectInput1Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_39 for Mode: ALT9"]
    SELECT_GPIO_EMC_B1_39_ALT9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_10 for Mode: ALT3"]
    SELECT_GPIO_EMC_B2_10_ALT3 = 0x01,
}
impl NetcPinmuxIppIndEth0RxdSelectInput1Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcPinmuxIppIndEth0RxdSelectInput1Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcPinmuxIppIndEth0RxdSelectInput1Daisy {
    #[inline(always)]
    fn from(val: u8) -> NetcPinmuxIppIndEth0RxdSelectInput1Daisy {
        NetcPinmuxIppIndEth0RxdSelectInput1Daisy::from_bits(val)
    }
}
impl From<NetcPinmuxIppIndEth0RxdSelectInput1Daisy> for u8 {
    #[inline(always)]
    fn from(val: NetcPinmuxIppIndEth0RxdSelectInput1Daisy) -> u8 {
        NetcPinmuxIppIndEth0RxdSelectInput1Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcPinmuxIppIndEth0RxdSelectInput2Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_01 for Mode: ALT4"]
    SELECT_GPIO_EMC_B2_01_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_16 for Mode: ALT3"]
    SELECT_GPIO_EMC_B2_16_ALT3 = 0x01,
}
impl NetcPinmuxIppIndEth0RxdSelectInput2Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcPinmuxIppIndEth0RxdSelectInput2Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcPinmuxIppIndEth0RxdSelectInput2Daisy {
    #[inline(always)]
    fn from(val: u8) -> NetcPinmuxIppIndEth0RxdSelectInput2Daisy {
        NetcPinmuxIppIndEth0RxdSelectInput2Daisy::from_bits(val)
    }
}
impl From<NetcPinmuxIppIndEth0RxdSelectInput2Daisy> for u8 {
    #[inline(always)]
    fn from(val: NetcPinmuxIppIndEth0RxdSelectInput2Daisy) -> u8 {
        NetcPinmuxIppIndEth0RxdSelectInput2Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcPinmuxIppIndEth0RxdSelectInput3Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_02 for Mode: ALT4"]
    SELECT_GPIO_EMC_B2_02_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_17 for Mode: ALT3"]
    SELECT_GPIO_EMC_B2_17_ALT3 = 0x01,
}
impl NetcPinmuxIppIndEth0RxdSelectInput3Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcPinmuxIppIndEth0RxdSelectInput3Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcPinmuxIppIndEth0RxdSelectInput3Daisy {
    #[inline(always)]
    fn from(val: u8) -> NetcPinmuxIppIndEth0RxdSelectInput3Daisy {
        NetcPinmuxIppIndEth0RxdSelectInput3Daisy::from_bits(val)
    }
}
impl From<NetcPinmuxIppIndEth0RxdSelectInput3Daisy> for u8 {
    #[inline(always)]
    fn from(val: NetcPinmuxIppIndEth0RxdSelectInput3Daisy) -> u8 {
        NetcPinmuxIppIndEth0RxdSelectInput3Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcPinmuxIppIndEth0TxClkSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_37 for Mode: ALT9"]
    SELECT_GPIO_EMC_B1_37_ALT9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_08 for Mode: ALT3"]
    SELECT_GPIO_EMC_B2_08_ALT3 = 0x01,
}
impl NetcPinmuxIppIndEth0TxClkSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcPinmuxIppIndEth0TxClkSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcPinmuxIppIndEth0TxClkSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> NetcPinmuxIppIndEth0TxClkSelectInputDaisy {
        NetcPinmuxIppIndEth0TxClkSelectInputDaisy::from_bits(val)
    }
}
impl From<NetcPinmuxIppIndEth0TxClkSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: NetcPinmuxIppIndEth0TxClkSelectInputDaisy) -> u8 {
        NetcPinmuxIppIndEth0TxClkSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcPinmuxIppIndEth2RxClkSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_21 for Mode: ALT4"]
    SELECT_GPIO_EMC_B1_21_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_33 for Mode: ALT10"]
    SELECT_GPIO_EMC_B1_33_ALT10 = 0x01,
    #[doc = "Selecting Pad: GPIO_EMC_B1_38 for Mode: ALT4"]
    SELECT_GPIO_EMC_B1_38_ALT4 = 0x02,
    #[doc = "Selecting Pad: GPIO_B2_13 for Mode: ALT11"]
    SELECT_GPIO_B2_13_ALT11 = 0x03,
}
impl NetcPinmuxIppIndEth2RxClkSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcPinmuxIppIndEth2RxClkSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcPinmuxIppIndEth2RxClkSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> NetcPinmuxIppIndEth2RxClkSelectInputDaisy {
        NetcPinmuxIppIndEth2RxClkSelectInputDaisy::from_bits(val)
    }
}
impl From<NetcPinmuxIppIndEth2RxClkSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: NetcPinmuxIppIndEth2RxClkSelectInputDaisy) -> u8 {
        NetcPinmuxIppIndEth2RxClkSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcPinmuxIppIndEth2RxDvSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_13 for Mode: ALT3"]
    SELECT_GPIO_EMC_B1_13_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_32 for Mode: ALT4"]
    SELECT_GPIO_EMC_B1_32_ALT4 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_12 for Mode: ALT11"]
    SELECT_GPIO_B2_12_ALT11 = 0x02,
    _RESERVED_3 = 0x03,
}
impl NetcPinmuxIppIndEth2RxDvSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcPinmuxIppIndEth2RxDvSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcPinmuxIppIndEth2RxDvSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> NetcPinmuxIppIndEth2RxDvSelectInputDaisy {
        NetcPinmuxIppIndEth2RxDvSelectInputDaisy::from_bits(val)
    }
}
impl From<NetcPinmuxIppIndEth2RxDvSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: NetcPinmuxIppIndEth2RxDvSelectInputDaisy) -> u8 {
        NetcPinmuxIppIndEth2RxDvSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcPinmuxIppIndEth2RxErSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_33 for Mode: ALT4"]
    SELECT_GPIO_EMC_B1_33_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_01 for Mode: ALT11"]
    SELECT_GPIO_B2_01_ALT11 = 0x01,
}
impl NetcPinmuxIppIndEth2RxErSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcPinmuxIppIndEth2RxErSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcPinmuxIppIndEth2RxErSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> NetcPinmuxIppIndEth2RxErSelectInputDaisy {
        NetcPinmuxIppIndEth2RxErSelectInputDaisy::from_bits(val)
    }
}
impl From<NetcPinmuxIppIndEth2RxErSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: NetcPinmuxIppIndEth2RxErSelectInputDaisy) -> u8 {
        NetcPinmuxIppIndEth2RxErSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcPinmuxIppIndEth2RxdSelectInput0Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_16 for Mode: ALT3"]
    SELECT_GPIO_EMC_B1_16_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_30 for Mode: ALT4"]
    SELECT_GPIO_EMC_B1_30_ALT4 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_10 for Mode: ALT11"]
    SELECT_GPIO_B2_10_ALT11 = 0x02,
    _RESERVED_3 = 0x03,
}
impl NetcPinmuxIppIndEth2RxdSelectInput0Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcPinmuxIppIndEth2RxdSelectInput0Daisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcPinmuxIppIndEth2RxdSelectInput0Daisy {
    #[inline(always)]
    fn from(val: u8) -> NetcPinmuxIppIndEth2RxdSelectInput0Daisy {
        NetcPinmuxIppIndEth2RxdSelectInput0Daisy::from_bits(val)
    }
}
impl From<NetcPinmuxIppIndEth2RxdSelectInput0Daisy> for u8 {
    #[inline(always)]
    fn from(val: NetcPinmuxIppIndEth2RxdSelectInput0Daisy) -> u8 {
        NetcPinmuxIppIndEth2RxdSelectInput0Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcPinmuxIppIndEth2RxdSelectInput1Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_17 for Mode: ALT3"]
    SELECT_GPIO_EMC_B1_17_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_31 for Mode: ALT4"]
    SELECT_GPIO_EMC_B1_31_ALT4 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_11 for Mode: ALT11"]
    SELECT_GPIO_B2_11_ALT11 = 0x02,
    _RESERVED_3 = 0x03,
}
impl NetcPinmuxIppIndEth2RxdSelectInput1Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcPinmuxIppIndEth2RxdSelectInput1Daisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcPinmuxIppIndEth2RxdSelectInput1Daisy {
    #[inline(always)]
    fn from(val: u8) -> NetcPinmuxIppIndEth2RxdSelectInput1Daisy {
        NetcPinmuxIppIndEth2RxdSelectInput1Daisy::from_bits(val)
    }
}
impl From<NetcPinmuxIppIndEth2RxdSelectInput1Daisy> for u8 {
    #[inline(always)]
    fn from(val: NetcPinmuxIppIndEth2RxdSelectInput1Daisy) -> u8 {
        NetcPinmuxIppIndEth2RxdSelectInput1Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcPinmuxIppIndEth2RxdSelectInput2Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_22 for Mode: ALT4"]
    SELECT_GPIO_EMC_B1_22_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_34 for Mode: ALT4"]
    SELECT_GPIO_EMC_B1_34_ALT4 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_02 for Mode: ALT11"]
    SELECT_GPIO_B2_02_ALT11 = 0x02,
    _RESERVED_3 = 0x03,
}
impl NetcPinmuxIppIndEth2RxdSelectInput2Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcPinmuxIppIndEth2RxdSelectInput2Daisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcPinmuxIppIndEth2RxdSelectInput2Daisy {
    #[inline(always)]
    fn from(val: u8) -> NetcPinmuxIppIndEth2RxdSelectInput2Daisy {
        NetcPinmuxIppIndEth2RxdSelectInput2Daisy::from_bits(val)
    }
}
impl From<NetcPinmuxIppIndEth2RxdSelectInput2Daisy> for u8 {
    #[inline(always)]
    fn from(val: NetcPinmuxIppIndEth2RxdSelectInput2Daisy) -> u8 {
        NetcPinmuxIppIndEth2RxdSelectInput2Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcPinmuxIppIndEth2RxdSelectInput3Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_23 for Mode: ALT4"]
    SELECT_GPIO_EMC_B1_23_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_35 for Mode: ALT4"]
    SELECT_GPIO_EMC_B1_35_ALT4 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_03 for Mode: ALT11"]
    SELECT_GPIO_B2_03_ALT11 = 0x02,
    _RESERVED_3 = 0x03,
}
impl NetcPinmuxIppIndEth2RxdSelectInput3Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcPinmuxIppIndEth2RxdSelectInput3Daisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcPinmuxIppIndEth2RxdSelectInput3Daisy {
    #[inline(always)]
    fn from(val: u8) -> NetcPinmuxIppIndEth2RxdSelectInput3Daisy {
        NetcPinmuxIppIndEth2RxdSelectInput3Daisy::from_bits(val)
    }
}
impl From<NetcPinmuxIppIndEth2RxdSelectInput3Daisy> for u8 {
    #[inline(always)]
    fn from(val: NetcPinmuxIppIndEth2RxdSelectInput3Daisy) -> u8 {
        NetcPinmuxIppIndEth2RxdSelectInput3Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcPinmuxIppIndEth2TxClkSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_15 for Mode: ALT3"]
    SELECT_GPIO_EMC_B1_15_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_29 for Mode: ALT4"]
    SELECT_GPIO_EMC_B1_29_ALT4 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_09 for Mode: ALT11"]
    SELECT_GPIO_B2_09_ALT11 = 0x02,
    _RESERVED_3 = 0x03,
}
impl NetcPinmuxIppIndEth2TxClkSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcPinmuxIppIndEth2TxClkSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcPinmuxIppIndEth2TxClkSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> NetcPinmuxIppIndEth2TxClkSelectInputDaisy {
        NetcPinmuxIppIndEth2TxClkSelectInputDaisy::from_bits(val)
    }
}
impl From<NetcPinmuxIppIndEth2TxClkSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: NetcPinmuxIppIndEth2TxClkSelectInputDaisy) -> u8 {
        NetcPinmuxIppIndEth2TxClkSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcPinmuxIppIndEth3RxClkSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_02 for Mode: ALT4"]
    SELECT_GPIO_EMC_B1_02_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_15 for Mode: ALT9"]
    SELECT_GPIO_EMC_B2_15_ALT9 = 0x01,
}
impl NetcPinmuxIppIndEth3RxClkSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcPinmuxIppIndEth3RxClkSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcPinmuxIppIndEth3RxClkSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> NetcPinmuxIppIndEth3RxClkSelectInputDaisy {
        NetcPinmuxIppIndEth3RxClkSelectInputDaisy::from_bits(val)
    }
}
impl From<NetcPinmuxIppIndEth3RxClkSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: NetcPinmuxIppIndEth3RxClkSelectInputDaisy) -> u8 {
        NetcPinmuxIppIndEth3RxClkSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcPinmuxIppIndEth3RxDvSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_11 for Mode: ALT4"]
    SELECT_GPIO_EMC_B1_11_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_11 for Mode: ALT9"]
    SELECT_GPIO_EMC_B2_11_ALT9 = 0x01,
}
impl NetcPinmuxIppIndEth3RxDvSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcPinmuxIppIndEth3RxDvSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcPinmuxIppIndEth3RxDvSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> NetcPinmuxIppIndEth3RxDvSelectInputDaisy {
        NetcPinmuxIppIndEth3RxDvSelectInputDaisy::from_bits(val)
    }
}
impl From<NetcPinmuxIppIndEth3RxDvSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: NetcPinmuxIppIndEth3RxDvSelectInputDaisy) -> u8 {
        NetcPinmuxIppIndEth3RxDvSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcPinmuxIppIndEth3RxErSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_12 for Mode: ALT4"]
    SELECT_GPIO_EMC_B1_12_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_12 for Mode: ALT9"]
    SELECT_GPIO_EMC_B2_12_ALT9 = 0x01,
}
impl NetcPinmuxIppIndEth3RxErSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcPinmuxIppIndEth3RxErSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcPinmuxIppIndEth3RxErSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> NetcPinmuxIppIndEth3RxErSelectInputDaisy {
        NetcPinmuxIppIndEth3RxErSelectInputDaisy::from_bits(val)
    }
}
impl From<NetcPinmuxIppIndEth3RxErSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: NetcPinmuxIppIndEth3RxErSelectInputDaisy) -> u8 {
        NetcPinmuxIppIndEth3RxErSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcPinmuxIppIndEth3RxdSelectInput0Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_09 for Mode: ALT4"]
    SELECT_GPIO_EMC_B1_09_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_09 for Mode: ALT9"]
    SELECT_GPIO_EMC_B2_09_ALT9 = 0x01,
}
impl NetcPinmuxIppIndEth3RxdSelectInput0Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcPinmuxIppIndEth3RxdSelectInput0Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcPinmuxIppIndEth3RxdSelectInput0Daisy {
    #[inline(always)]
    fn from(val: u8) -> NetcPinmuxIppIndEth3RxdSelectInput0Daisy {
        NetcPinmuxIppIndEth3RxdSelectInput0Daisy::from_bits(val)
    }
}
impl From<NetcPinmuxIppIndEth3RxdSelectInput0Daisy> for u8 {
    #[inline(always)]
    fn from(val: NetcPinmuxIppIndEth3RxdSelectInput0Daisy) -> u8 {
        NetcPinmuxIppIndEth3RxdSelectInput0Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcPinmuxIppIndEth3RxdSelectInput1Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_10 for Mode: ALT4"]
    SELECT_GPIO_EMC_B1_10_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_10 for Mode: ALT9"]
    SELECT_GPIO_EMC_B2_10_ALT9 = 0x01,
}
impl NetcPinmuxIppIndEth3RxdSelectInput1Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcPinmuxIppIndEth3RxdSelectInput1Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcPinmuxIppIndEth3RxdSelectInput1Daisy {
    #[inline(always)]
    fn from(val: u8) -> NetcPinmuxIppIndEth3RxdSelectInput1Daisy {
        NetcPinmuxIppIndEth3RxdSelectInput1Daisy::from_bits(val)
    }
}
impl From<NetcPinmuxIppIndEth3RxdSelectInput1Daisy> for u8 {
    #[inline(always)]
    fn from(val: NetcPinmuxIppIndEth3RxdSelectInput1Daisy) -> u8 {
        NetcPinmuxIppIndEth3RxdSelectInput1Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcPinmuxIppIndEth3RxdSelectInput2Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_04 for Mode: ALT4"]
    SELECT_GPIO_EMC_B1_04_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_16 for Mode: ALT9"]
    SELECT_GPIO_EMC_B2_16_ALT9 = 0x01,
}
impl NetcPinmuxIppIndEth3RxdSelectInput2Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcPinmuxIppIndEth3RxdSelectInput2Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcPinmuxIppIndEth3RxdSelectInput2Daisy {
    #[inline(always)]
    fn from(val: u8) -> NetcPinmuxIppIndEth3RxdSelectInput2Daisy {
        NetcPinmuxIppIndEth3RxdSelectInput2Daisy::from_bits(val)
    }
}
impl From<NetcPinmuxIppIndEth3RxdSelectInput2Daisy> for u8 {
    #[inline(always)]
    fn from(val: NetcPinmuxIppIndEth3RxdSelectInput2Daisy) -> u8 {
        NetcPinmuxIppIndEth3RxdSelectInput2Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcPinmuxIppIndEth3RxdSelectInput3Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_03 for Mode: ALT4"]
    SELECT_GPIO_EMC_B1_03_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_17 for Mode: ALT9"]
    SELECT_GPIO_EMC_B2_17_ALT9 = 0x01,
}
impl NetcPinmuxIppIndEth3RxdSelectInput3Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcPinmuxIppIndEth3RxdSelectInput3Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcPinmuxIppIndEth3RxdSelectInput3Daisy {
    #[inline(always)]
    fn from(val: u8) -> NetcPinmuxIppIndEth3RxdSelectInput3Daisy {
        NetcPinmuxIppIndEth3RxdSelectInput3Daisy::from_bits(val)
    }
}
impl From<NetcPinmuxIppIndEth3RxdSelectInput3Daisy> for u8 {
    #[inline(always)]
    fn from(val: NetcPinmuxIppIndEth3RxdSelectInput3Daisy) -> u8 {
        NetcPinmuxIppIndEth3RxdSelectInput3Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcPinmuxIppIndEth3TxClkSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_08 for Mode: ALT4"]
    SELECT_GPIO_EMC_B1_08_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_08 for Mode: ALT9"]
    SELECT_GPIO_EMC_B2_08_ALT9 = 0x01,
}
impl NetcPinmuxIppIndEth3TxClkSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcPinmuxIppIndEth3TxClkSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcPinmuxIppIndEth3TxClkSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> NetcPinmuxIppIndEth3TxClkSelectInputDaisy {
        NetcPinmuxIppIndEth3TxClkSelectInputDaisy::from_bits(val)
    }
}
impl From<NetcPinmuxIppIndEth3TxClkSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: NetcPinmuxIppIndEth3TxClkSelectInputDaisy) -> u8 {
        NetcPinmuxIppIndEth3TxClkSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcPinmuxIppIndEth4RxClkSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_02 for Mode: ALT9"]
    SELECT_GPIO_EMC_B1_02_ALT9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_08 for Mode: ALT1"]
    SELECT_GPIO_EMC_B2_08_ALT1 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_11 for Mode: ALT8"]
    SELECT_GPIO_B1_11_ALT8 = 0x02,
    _RESERVED_3 = 0x03,
}
impl NetcPinmuxIppIndEth4RxClkSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcPinmuxIppIndEth4RxClkSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcPinmuxIppIndEth4RxClkSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> NetcPinmuxIppIndEth4RxClkSelectInputDaisy {
        NetcPinmuxIppIndEth4RxClkSelectInputDaisy::from_bits(val)
    }
}
impl From<NetcPinmuxIppIndEth4RxClkSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: NetcPinmuxIppIndEth4RxClkSelectInputDaisy) -> u8 {
        NetcPinmuxIppIndEth4RxClkSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcPinmuxIppIndEth4RxDvSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_11 for Mode: ALT9"]
    SELECT_GPIO_EMC_B1_11_ALT9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_19 for Mode: ALT1"]
    SELECT_GPIO_EMC_B2_19_ALT1 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_06 for Mode: ALT8"]
    SELECT_GPIO_B1_06_ALT8 = 0x02,
    _RESERVED_3 = 0x03,
}
impl NetcPinmuxIppIndEth4RxDvSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcPinmuxIppIndEth4RxDvSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcPinmuxIppIndEth4RxDvSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> NetcPinmuxIppIndEth4RxDvSelectInputDaisy {
        NetcPinmuxIppIndEth4RxDvSelectInputDaisy::from_bits(val)
    }
}
impl From<NetcPinmuxIppIndEth4RxDvSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: NetcPinmuxIppIndEth4RxDvSelectInputDaisy) -> u8 {
        NetcPinmuxIppIndEth4RxDvSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcPinmuxIppIndEth4RxErSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_12 for Mode: ALT9"]
    SELECT_GPIO_EMC_B1_12_ALT9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_20 for Mode: ALT1"]
    SELECT_GPIO_EMC_B2_20_ALT1 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_12 for Mode: ALT8"]
    SELECT_GPIO_B1_12_ALT8 = 0x02,
    _RESERVED_3 = 0x03,
}
impl NetcPinmuxIppIndEth4RxErSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcPinmuxIppIndEth4RxErSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcPinmuxIppIndEth4RxErSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> NetcPinmuxIppIndEth4RxErSelectInputDaisy {
        NetcPinmuxIppIndEth4RxErSelectInputDaisy::from_bits(val)
    }
}
impl From<NetcPinmuxIppIndEth4RxErSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: NetcPinmuxIppIndEth4RxErSelectInputDaisy) -> u8 {
        NetcPinmuxIppIndEth4RxErSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcPinmuxIppIndEth4RxdSelectInput0Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_09 for Mode: ALT9"]
    SELECT_GPIO_EMC_B1_09_ALT9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_17 for Mode: ALT1"]
    SELECT_GPIO_EMC_B2_17_ALT1 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_04 for Mode: ALT8"]
    SELECT_GPIO_B1_04_ALT8 = 0x02,
    _RESERVED_3 = 0x03,
}
impl NetcPinmuxIppIndEth4RxdSelectInput0Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcPinmuxIppIndEth4RxdSelectInput0Daisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcPinmuxIppIndEth4RxdSelectInput0Daisy {
    #[inline(always)]
    fn from(val: u8) -> NetcPinmuxIppIndEth4RxdSelectInput0Daisy {
        NetcPinmuxIppIndEth4RxdSelectInput0Daisy::from_bits(val)
    }
}
impl From<NetcPinmuxIppIndEth4RxdSelectInput0Daisy> for u8 {
    #[inline(always)]
    fn from(val: NetcPinmuxIppIndEth4RxdSelectInput0Daisy) -> u8 {
        NetcPinmuxIppIndEth4RxdSelectInput0Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcPinmuxIppIndEth4RxdSelectInput1Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_10 for Mode: ALT9"]
    SELECT_GPIO_EMC_B1_10_ALT9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_18 for Mode: ALT1"]
    SELECT_GPIO_EMC_B2_18_ALT1 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_05 for Mode: ALT8"]
    SELECT_GPIO_B1_05_ALT8 = 0x02,
    _RESERVED_3 = 0x03,
}
impl NetcPinmuxIppIndEth4RxdSelectInput1Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcPinmuxIppIndEth4RxdSelectInput1Daisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcPinmuxIppIndEth4RxdSelectInput1Daisy {
    #[inline(always)]
    fn from(val: u8) -> NetcPinmuxIppIndEth4RxdSelectInput1Daisy {
        NetcPinmuxIppIndEth4RxdSelectInput1Daisy::from_bits(val)
    }
}
impl From<NetcPinmuxIppIndEth4RxdSelectInput1Daisy> for u8 {
    #[inline(always)]
    fn from(val: NetcPinmuxIppIndEth4RxdSelectInput1Daisy) -> u8 {
        NetcPinmuxIppIndEth4RxdSelectInput1Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcPinmuxIppIndEth4RxdSelectInput2Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_04 for Mode: ALT9"]
    SELECT_GPIO_EMC_B1_04_ALT9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_10 for Mode: ALT1"]
    SELECT_GPIO_EMC_B2_10_ALT1 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_09 for Mode: ALT8"]
    SELECT_GPIO_B1_09_ALT8 = 0x02,
    _RESERVED_3 = 0x03,
}
impl NetcPinmuxIppIndEth4RxdSelectInput2Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcPinmuxIppIndEth4RxdSelectInput2Daisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcPinmuxIppIndEth4RxdSelectInput2Daisy {
    #[inline(always)]
    fn from(val: u8) -> NetcPinmuxIppIndEth4RxdSelectInput2Daisy {
        NetcPinmuxIppIndEth4RxdSelectInput2Daisy::from_bits(val)
    }
}
impl From<NetcPinmuxIppIndEth4RxdSelectInput2Daisy> for u8 {
    #[inline(always)]
    fn from(val: NetcPinmuxIppIndEth4RxdSelectInput2Daisy) -> u8 {
        NetcPinmuxIppIndEth4RxdSelectInput2Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcPinmuxIppIndEth4RxdSelectInput3Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_03 for Mode: ALT9"]
    SELECT_GPIO_EMC_B1_03_ALT9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_09 for Mode: ALT1"]
    SELECT_GPIO_EMC_B2_09_ALT1 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_10 for Mode: ALT8"]
    SELECT_GPIO_B1_10_ALT8 = 0x02,
    _RESERVED_3 = 0x03,
}
impl NetcPinmuxIppIndEth4RxdSelectInput3Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcPinmuxIppIndEth4RxdSelectInput3Daisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcPinmuxIppIndEth4RxdSelectInput3Daisy {
    #[inline(always)]
    fn from(val: u8) -> NetcPinmuxIppIndEth4RxdSelectInput3Daisy {
        NetcPinmuxIppIndEth4RxdSelectInput3Daisy::from_bits(val)
    }
}
impl From<NetcPinmuxIppIndEth4RxdSelectInput3Daisy> for u8 {
    #[inline(always)]
    fn from(val: NetcPinmuxIppIndEth4RxdSelectInput3Daisy) -> u8 {
        NetcPinmuxIppIndEth4RxdSelectInput3Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcPinmuxIppIndEth4TxClkSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_08 for Mode: ALT9"]
    SELECT_GPIO_EMC_B1_08_ALT9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_16 for Mode: ALT1"]
    SELECT_GPIO_EMC_B2_16_ALT1 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_03 for Mode: ALT8"]
    SELECT_GPIO_B1_03_ALT8 = 0x02,
    _RESERVED_3 = 0x03,
}
impl NetcPinmuxIppIndEth4TxClkSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcPinmuxIppIndEth4TxClkSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcPinmuxIppIndEth4TxClkSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> NetcPinmuxIppIndEth4TxClkSelectInputDaisy {
        NetcPinmuxIppIndEth4TxClkSelectInputDaisy::from_bits(val)
    }
}
impl From<NetcPinmuxIppIndEth4TxClkSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: NetcPinmuxIppIndEth4TxClkSelectInputDaisy) -> u8 {
        NetcPinmuxIppIndEth4TxClkSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcTmrTrig1SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_20 for Mode: ALT6"]
    SELECT_GPIO_AD_20_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_24 for Mode: ALT7"]
    SELECT_GPIO_AD_24_ALT7 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_32 for Mode: ALT7"]
    SELECT_GPIO_AD_32_ALT7 = 0x02,
    #[doc = "Selecting Pad: GPIO_SD_B2_11 for Mode: ALT8"]
    SELECT_GPIO_SD_B2_11_ALT8 = 0x03,
}
impl NetcTmrTrig1SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcTmrTrig1SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcTmrTrig1SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> NetcTmrTrig1SelectInputDaisy {
        NetcTmrTrig1SelectInputDaisy::from_bits(val)
    }
}
impl From<NetcTmrTrig1SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: NetcTmrTrig1SelectInputDaisy) -> u8 {
        NetcTmrTrig1SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetcTmrTrig2SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_21 for Mode: ALT6"]
    SELECT_GPIO_AD_21_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_25 for Mode: ALT7"]
    SELECT_GPIO_AD_25_ALT7 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_33 for Mode: ALT7"]
    SELECT_GPIO_AD_33_ALT7 = 0x02,
    #[doc = "Selecting Pad: GPIO_SD_B2_10 for Mode: ALT8"]
    SELECT_GPIO_SD_B2_10_ALT8 = 0x03,
}
impl NetcTmrTrig2SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NetcTmrTrig2SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NetcTmrTrig2SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> NetcTmrTrig2SelectInputDaisy {
        NetcTmrTrig2SelectInputDaisy::from_bits(val)
    }
}
impl From<NetcTmrTrig2SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: NetcTmrTrig2SelectInputDaisy) -> u8 {
        NetcTmrTrig2SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ode {
    #[doc = "Disabled"]
    ODE_0_DISABLED = 0x0,
    #[doc = "Enabled"]
    ODE_1_ENABLED = 0x01,
}
impl Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ode {
    #[inline(always)]
    fn from(val: u8) -> Ode {
        Ode::from_bits(val)
    }
}
impl From<Ode> for u8 {
    #[inline(always)]
    fn from(val: Ode) -> u8 {
        Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdrv {
    #[doc = "high driver"]
    PDRV_0_HIGH_DRIVER = 0x0,
    #[doc = "normal driver"]
    PDRV_1_NORMAL_DRIVER = 0x01,
}
impl Pdrv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdrv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdrv {
    #[inline(always)]
    fn from(val: u8) -> Pdrv {
        Pdrv::from_bits(val)
    }
}
impl From<Pdrv> for u8 {
    #[inline(always)]
    fn from(val: Pdrv) -> u8 {
        Pdrv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pue {
    #[doc = "Pull Disable, Highz"]
    PUE_0_PULL_DISABLE__HIGHZ = 0x0,
    #[doc = "Pull Enable"]
    PUE_1_PULL_ENABLE = 0x01,
}
impl Pue {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pue {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pue {
    #[inline(always)]
    fn from(val: u8) -> Pue {
        Pue::from_bits(val)
    }
}
impl From<Pue> for u8 {
    #[inline(always)]
    fn from(val: Pue) -> u8 {
        Pue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pull {
    #[doc = "Forbidden"]
    PULL_0_FORBIDDEN = 0x0,
    #[doc = "PU"]
    PULL_1_PU = 0x01,
    #[doc = "PD"]
    PULL_2_PD = 0x02,
    #[doc = "No Pull"]
    PULL_3_NO_PULL = 0x03,
}
impl Pull {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pull {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pull {
    #[inline(always)]
    fn from(val: u8) -> Pull {
        Pull::from_bits(val)
    }
}
impl From<Pull> for u8 {
    #[inline(always)]
    fn from(val: Pull) -> u8 {
        Pull::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pus {
    #[doc = "Weak pull down"]
    PUS_0_WEAK_PULL_DOWN = 0x0,
    #[doc = "Weak pull up"]
    PUS_1_WEAK_PULL_UP = 0x01,
}
impl Pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pus {
    #[inline(always)]
    fn from(val: u8) -> Pus {
        Pus::from_bits(val)
    }
}
impl From<Pus> for u8 {
    #[inline(always)]
    fn from(val: Pus) -> u8 {
        Pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer1Tmr0InputSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_18 for Mode: ALT2"]
    SELECT_GPIO_EMC_B1_18_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_09 for Mode: ALT10"]
    SELECT_GPIO_EMC_B2_09_ALT10 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_00 for Mode: ALT3"]
    SELECT_GPIO_B1_00_ALT3 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Qtimer1Tmr0InputSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer1Tmr0InputSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer1Tmr0InputSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Qtimer1Tmr0InputSelectInputDaisy {
        Qtimer1Tmr0InputSelectInputDaisy::from_bits(val)
    }
}
impl From<Qtimer1Tmr0InputSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Qtimer1Tmr0InputSelectInputDaisy) -> u8 {
        Qtimer1Tmr0InputSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer1Tmr1InputSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_13 for Mode: ALT10"]
    SELECT_GPIO_EMC_B1_13_ALT10 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_10 for Mode: ALT10"]
    SELECT_GPIO_EMC_B2_10_ALT10 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_01 for Mode: ALT3"]
    SELECT_GPIO_B1_01_ALT3 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Qtimer1Tmr1InputSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer1Tmr1InputSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer1Tmr1InputSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Qtimer1Tmr1InputSelectInputDaisy {
        Qtimer1Tmr1InputSelectInputDaisy::from_bits(val)
    }
}
impl From<Qtimer1Tmr1InputSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Qtimer1Tmr1InputSelectInputDaisy) -> u8 {
        Qtimer1Tmr1InputSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer1Tmr2InputSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_11 for Mode: ALT10"]
    SELECT_GPIO_EMC_B2_11_ALT10 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_02 for Mode: ALT3"]
    SELECT_GPIO_B1_02_ALT3 = 0x01,
}
impl Qtimer1Tmr2InputSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer1Tmr2InputSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer1Tmr2InputSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Qtimer1Tmr2InputSelectInputDaisy {
        Qtimer1Tmr2InputSelectInputDaisy::from_bits(val)
    }
}
impl From<Qtimer1Tmr2InputSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Qtimer1Tmr2InputSelectInputDaisy) -> u8 {
        Qtimer1Tmr2InputSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer2Tmr0InputSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_19 for Mode: ALT2"]
    SELECT_GPIO_EMC_B1_19_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_13 for Mode: ALT10"]
    SELECT_GPIO_EMC_B2_13_ALT10 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_03 for Mode: ALT3"]
    SELECT_GPIO_B1_03_ALT3 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Qtimer2Tmr0InputSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer2Tmr0InputSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer2Tmr0InputSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Qtimer2Tmr0InputSelectInputDaisy {
        Qtimer2Tmr0InputSelectInputDaisy::from_bits(val)
    }
}
impl From<Qtimer2Tmr0InputSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Qtimer2Tmr0InputSelectInputDaisy) -> u8 {
        Qtimer2Tmr0InputSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer2Tmr1InputSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_39 for Mode: ALT6"]
    SELECT_GPIO_EMC_B1_39_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_14 for Mode: ALT10"]
    SELECT_GPIO_EMC_B2_14_ALT10 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_04 for Mode: ALT3"]
    SELECT_GPIO_B1_04_ALT3 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Qtimer2Tmr1InputSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer2Tmr1InputSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer2Tmr1InputSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Qtimer2Tmr1InputSelectInputDaisy {
        Qtimer2Tmr1InputSelectInputDaisy::from_bits(val)
    }
}
impl From<Qtimer2Tmr1InputSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Qtimer2Tmr1InputSelectInputDaisy) -> u8 {
        Qtimer2Tmr1InputSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer2Tmr2InputSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_15 for Mode: ALT10"]
    SELECT_GPIO_EMC_B2_15_ALT10 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_05 for Mode: ALT3"]
    SELECT_GPIO_B1_05_ALT3 = 0x01,
}
impl Qtimer2Tmr2InputSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer2Tmr2InputSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer2Tmr2InputSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Qtimer2Tmr2InputSelectInputDaisy {
        Qtimer2Tmr2InputSelectInputDaisy::from_bits(val)
    }
}
impl From<Qtimer2Tmr2InputSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Qtimer2Tmr2InputSelectInputDaisy) -> u8 {
        Qtimer2Tmr2InputSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer3Tmr0InputSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_20 for Mode: ALT2"]
    SELECT_GPIO_EMC_B1_20_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_17 for Mode: ALT10"]
    SELECT_GPIO_EMC_B2_17_ALT10 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_06 for Mode: ALT3"]
    SELECT_GPIO_B1_06_ALT3 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Qtimer3Tmr0InputSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer3Tmr0InputSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer3Tmr0InputSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Qtimer3Tmr0InputSelectInputDaisy {
        Qtimer3Tmr0InputSelectInputDaisy::from_bits(val)
    }
}
impl From<Qtimer3Tmr0InputSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Qtimer3Tmr0InputSelectInputDaisy) -> u8 {
        Qtimer3Tmr0InputSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer3Tmr1InputSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_40 for Mode: ALT6"]
    SELECT_GPIO_EMC_B1_40_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_18 for Mode: ALT10"]
    SELECT_GPIO_EMC_B2_18_ALT10 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_07 for Mode: ALT3"]
    SELECT_GPIO_B1_07_ALT3 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Qtimer3Tmr1InputSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer3Tmr1InputSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer3Tmr1InputSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Qtimer3Tmr1InputSelectInputDaisy {
        Qtimer3Tmr1InputSelectInputDaisy::from_bits(val)
    }
}
impl From<Qtimer3Tmr1InputSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Qtimer3Tmr1InputSelectInputDaisy) -> u8 {
        Qtimer3Tmr1InputSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer3Tmr2InputSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_19 for Mode: ALT10"]
    SELECT_GPIO_EMC_B2_19_ALT10 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_08 for Mode: ALT3"]
    SELECT_GPIO_B1_08_ALT3 = 0x01,
}
impl Qtimer3Tmr2InputSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer3Tmr2InputSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer3Tmr2InputSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Qtimer3Tmr2InputSelectInputDaisy {
        Qtimer3Tmr2InputSelectInputDaisy::from_bits(val)
    }
}
impl From<Qtimer3Tmr2InputSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Qtimer3Tmr2InputSelectInputDaisy) -> u8 {
        Qtimer3Tmr2InputSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer4Tmr0InputSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_21 for Mode: ALT2"]
    SELECT_GPIO_EMC_B1_21_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_00 for Mode: ALT9"]
    SELECT_GPIO_AD_00_ALT9 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_09 for Mode: ALT3"]
    SELECT_GPIO_B1_09_ALT3 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Qtimer4Tmr0InputSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer4Tmr0InputSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer4Tmr0InputSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Qtimer4Tmr0InputSelectInputDaisy {
        Qtimer4Tmr0InputSelectInputDaisy::from_bits(val)
    }
}
impl From<Qtimer4Tmr0InputSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Qtimer4Tmr0InputSelectInputDaisy) -> u8 {
        Qtimer4Tmr0InputSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer4Tmr1InputSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_41 for Mode: ALT6"]
    SELECT_GPIO_EMC_B1_41_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_01 for Mode: ALT9"]
    SELECT_GPIO_AD_01_ALT9 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_10 for Mode: ALT3"]
    SELECT_GPIO_B1_10_ALT3 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Qtimer4Tmr1InputSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer4Tmr1InputSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer4Tmr1InputSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Qtimer4Tmr1InputSelectInputDaisy {
        Qtimer4Tmr1InputSelectInputDaisy::from_bits(val)
    }
}
impl From<Qtimer4Tmr1InputSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Qtimer4Tmr1InputSelectInputDaisy) -> u8 {
        Qtimer4Tmr1InputSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer4Tmr2InputSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_02 for Mode: ALT9"]
    SELECT_GPIO_AD_02_ALT9 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_11 for Mode: ALT3"]
    SELECT_GPIO_B1_11_ALT3 = 0x01,
}
impl Qtimer4Tmr2InputSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer4Tmr2InputSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer4Tmr2InputSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Qtimer4Tmr2InputSelectInputDaisy {
        Qtimer4Tmr2InputSelectInputDaisy::from_bits(val)
    }
}
impl From<Qtimer4Tmr2InputSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Qtimer4Tmr2InputSelectInputDaisy) -> u8 {
        Qtimer4Tmr2InputSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer5Tmr0InputSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_22 for Mode: ALT2"]
    SELECT_GPIO_EMC_B1_22_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_04 for Mode: ALT9"]
    SELECT_GPIO_AD_04_ALT9 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_07 for Mode: ALT0"]
    SELECT_GPIO_B2_07_ALT0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Qtimer5Tmr0InputSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer5Tmr0InputSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer5Tmr0InputSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Qtimer5Tmr0InputSelectInputDaisy {
        Qtimer5Tmr0InputSelectInputDaisy::from_bits(val)
    }
}
impl From<Qtimer5Tmr0InputSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Qtimer5Tmr0InputSelectInputDaisy) -> u8 {
        Qtimer5Tmr0InputSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer5Tmr1InputSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_00 for Mode: ALT2"]
    SELECT_GPIO_EMC_B2_00_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_05 for Mode: ALT9"]
    SELECT_GPIO_AD_05_ALT9 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_08 for Mode: ALT0"]
    SELECT_GPIO_B2_08_ALT0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Qtimer5Tmr1InputSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer5Tmr1InputSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer5Tmr1InputSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Qtimer5Tmr1InputSelectInputDaisy {
        Qtimer5Tmr1InputSelectInputDaisy::from_bits(val)
    }
}
impl From<Qtimer5Tmr1InputSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Qtimer5Tmr1InputSelectInputDaisy) -> u8 {
        Qtimer5Tmr1InputSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer5Tmr2InputSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_06 for Mode: ALT9"]
    SELECT_GPIO_AD_06_ALT9 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_09 for Mode: ALT0"]
    SELECT_GPIO_B2_09_ALT0 = 0x01,
}
impl Qtimer5Tmr2InputSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer5Tmr2InputSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer5Tmr2InputSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Qtimer5Tmr2InputSelectInputDaisy {
        Qtimer5Tmr2InputSelectInputDaisy::from_bits(val)
    }
}
impl From<Qtimer5Tmr2InputSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Qtimer5Tmr2InputSelectInputDaisy) -> u8 {
        Qtimer5Tmr2InputSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer6Tmr0InputSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_23 for Mode: ALT2"]
    SELECT_GPIO_EMC_B1_23_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_08 for Mode: ALT9"]
    SELECT_GPIO_AD_08_ALT9 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_B2_01 for Mode: ALT3"]
    SELECT_GPIO_SD_B2_01_ALT3 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Qtimer6Tmr0InputSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer6Tmr0InputSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer6Tmr0InputSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Qtimer6Tmr0InputSelectInputDaisy {
        Qtimer6Tmr0InputSelectInputDaisy::from_bits(val)
    }
}
impl From<Qtimer6Tmr0InputSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Qtimer6Tmr0InputSelectInputDaisy) -> u8 {
        Qtimer6Tmr0InputSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer6Tmr1InputSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_01 for Mode: ALT2"]
    SELECT_GPIO_EMC_B2_01_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_09 for Mode: ALT9"]
    SELECT_GPIO_AD_09_ALT9 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_B2_02 for Mode: ALT3"]
    SELECT_GPIO_SD_B2_02_ALT3 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Qtimer6Tmr1InputSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer6Tmr1InputSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer6Tmr1InputSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Qtimer6Tmr1InputSelectInputDaisy {
        Qtimer6Tmr1InputSelectInputDaisy::from_bits(val)
    }
}
impl From<Qtimer6Tmr1InputSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Qtimer6Tmr1InputSelectInputDaisy) -> u8 {
        Qtimer6Tmr1InputSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer6Tmr2InputSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_10 for Mode: ALT9"]
    SELECT_GPIO_AD_10_ALT9 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_03 for Mode: ALT3"]
    SELECT_GPIO_SD_B2_03_ALT3 = 0x01,
}
impl Qtimer6Tmr2InputSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer6Tmr2InputSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer6Tmr2InputSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Qtimer6Tmr2InputSelectInputDaisy {
        Qtimer6Tmr2InputSelectInputDaisy::from_bits(val)
    }
}
impl From<Qtimer6Tmr2InputSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Qtimer6Tmr2InputSelectInputDaisy) -> u8 {
        Qtimer6Tmr2InputSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer7Tmr0InputSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_24 for Mode: ALT2"]
    SELECT_GPIO_EMC_B1_24_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_04 for Mode: ALT3"]
    SELECT_GPIO_SD_B2_04_ALT3 = 0x01,
}
impl Qtimer7Tmr0InputSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer7Tmr0InputSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer7Tmr0InputSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Qtimer7Tmr0InputSelectInputDaisy {
        Qtimer7Tmr0InputSelectInputDaisy::from_bits(val)
    }
}
impl From<Qtimer7Tmr0InputSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Qtimer7Tmr0InputSelectInputDaisy) -> u8 {
        Qtimer7Tmr0InputSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer7Tmr1InputSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_02 for Mode: ALT2"]
    SELECT_GPIO_EMC_B2_02_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_05 for Mode: ALT3"]
    SELECT_GPIO_SD_B2_05_ALT3 = 0x01,
}
impl Qtimer7Tmr1InputSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer7Tmr1InputSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer7Tmr1InputSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Qtimer7Tmr1InputSelectInputDaisy {
        Qtimer7Tmr1InputSelectInputDaisy::from_bits(val)
    }
}
impl From<Qtimer7Tmr1InputSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Qtimer7Tmr1InputSelectInputDaisy) -> u8 {
        Qtimer7Tmr1InputSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer8Tmr0InputSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_25 for Mode: ALT2"]
    SELECT_GPIO_EMC_B1_25_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_08 for Mode: ALT3"]
    SELECT_GPIO_SD_B2_08_ALT3 = 0x01,
}
impl Qtimer8Tmr0InputSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer8Tmr0InputSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer8Tmr0InputSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Qtimer8Tmr0InputSelectInputDaisy {
        Qtimer8Tmr0InputSelectInputDaisy::from_bits(val)
    }
}
impl From<Qtimer8Tmr0InputSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Qtimer8Tmr0InputSelectInputDaisy) -> u8 {
        Qtimer8Tmr0InputSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer8Tmr1InputSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_03 for Mode: ALT2"]
    SELECT_GPIO_EMC_B2_03_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_09 for Mode: ALT3"]
    SELECT_GPIO_SD_B2_09_ALT3 = 0x01,
}
impl Qtimer8Tmr1InputSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer8Tmr1InputSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer8Tmr1InputSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Qtimer8Tmr1InputSelectInputDaisy {
        Qtimer8Tmr1InputSelectInputDaisy::from_bits(val)
    }
}
impl From<Qtimer8Tmr1InputSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Qtimer8Tmr1InputSelectInputDaisy) -> u8 {
        Qtimer8Tmr1InputSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai4IpgClkSaiMclkSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_17 for Mode: ALT0"]
    SELECT_GPIO_AD_17_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_05 for Mode: ALT12"]
    SELECT_GPIO_B1_05_ALT12 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_00 for Mode: ALT4"]
    SELECT_GPIO_B2_00_ALT4 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sai4IpgClkSaiMclkSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai4IpgClkSaiMclkSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai4IpgClkSaiMclkSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Sai4IpgClkSaiMclkSelectInputDaisy {
        Sai4IpgClkSaiMclkSelectInputDaisy::from_bits(val)
    }
}
impl From<Sai4IpgClkSaiMclkSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Sai4IpgClkSaiMclkSelectInputDaisy) -> u8 {
        Sai4IpgClkSaiMclkSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai4IppIndSaiRxbclkSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_19 for Mode: ALT0"]
    SELECT_GPIO_AD_19_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_06 for Mode: ALT12"]
    SELECT_GPIO_B1_06_ALT12 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_05 for Mode: ALT4"]
    SELECT_GPIO_B2_05_ALT4 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sai4IppIndSaiRxbclkSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai4IppIndSaiRxbclkSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai4IppIndSaiRxbclkSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Sai4IppIndSaiRxbclkSelectInputDaisy {
        Sai4IppIndSaiRxbclkSelectInputDaisy::from_bits(val)
    }
}
impl From<Sai4IppIndSaiRxbclkSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Sai4IppIndSaiRxbclkSelectInputDaisy) -> u8 {
        Sai4IppIndSaiRxbclkSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai4IppIndSaiRxdataSelectInput0Daisy {
    #[doc = "Selecting Pad: GPIO_AD_20 for Mode: ALT0"]
    SELECT_GPIO_AD_20_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_01 for Mode: ALT12"]
    SELECT_GPIO_B1_01_ALT12 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_06 for Mode: ALT4"]
    SELECT_GPIO_B2_06_ALT4 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sai4IppIndSaiRxdataSelectInput0Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai4IppIndSaiRxdataSelectInput0Daisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai4IppIndSaiRxdataSelectInput0Daisy {
    #[inline(always)]
    fn from(val: u8) -> Sai4IppIndSaiRxdataSelectInput0Daisy {
        Sai4IppIndSaiRxdataSelectInput0Daisy::from_bits(val)
    }
}
impl From<Sai4IppIndSaiRxdataSelectInput0Daisy> for u8 {
    #[inline(always)]
    fn from(val: Sai4IppIndSaiRxdataSelectInput0Daisy) -> u8 {
        Sai4IppIndSaiRxdataSelectInput0Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai4IppIndSaiRxdataSelectInput1Daisy {
    #[doc = "Selecting Pad: GPIO_B1_02 for Mode: ALT12"]
    SELECT_GPIO_B1_02_ALT12 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_07 for Mode: ALT6"]
    SELECT_GPIO_B2_07_ALT6 = 0x01,
}
impl Sai4IppIndSaiRxdataSelectInput1Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai4IppIndSaiRxdataSelectInput1Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai4IppIndSaiRxdataSelectInput1Daisy {
    #[inline(always)]
    fn from(val: u8) -> Sai4IppIndSaiRxdataSelectInput1Daisy {
        Sai4IppIndSaiRxdataSelectInput1Daisy::from_bits(val)
    }
}
impl From<Sai4IppIndSaiRxdataSelectInput1Daisy> for u8 {
    #[inline(always)]
    fn from(val: Sai4IppIndSaiRxdataSelectInput1Daisy) -> u8 {
        Sai4IppIndSaiRxdataSelectInput1Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai4IppIndSaiRxsyncSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_18 for Mode: ALT0"]
    SELECT_GPIO_AD_18_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_07 for Mode: ALT12"]
    SELECT_GPIO_B1_07_ALT12 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_04 for Mode: ALT4"]
    SELECT_GPIO_B2_04_ALT4 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sai4IppIndSaiRxsyncSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai4IppIndSaiRxsyncSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai4IppIndSaiRxsyncSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Sai4IppIndSaiRxsyncSelectInputDaisy {
        Sai4IppIndSaiRxsyncSelectInputDaisy::from_bits(val)
    }
}
impl From<Sai4IppIndSaiRxsyncSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Sai4IppIndSaiRxsyncSelectInputDaisy) -> u8 {
        Sai4IppIndSaiRxsyncSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai4IppIndSaiTxbclkSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_22 for Mode: ALT0"]
    SELECT_GPIO_AD_22_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_08 for Mode: ALT12"]
    SELECT_GPIO_B1_08_ALT12 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_01 for Mode: ALT4"]
    SELECT_GPIO_B2_01_ALT4 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sai4IppIndSaiTxbclkSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai4IppIndSaiTxbclkSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai4IppIndSaiTxbclkSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Sai4IppIndSaiTxbclkSelectInputDaisy {
        Sai4IppIndSaiTxbclkSelectInputDaisy::from_bits(val)
    }
}
impl From<Sai4IppIndSaiTxbclkSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Sai4IppIndSaiTxbclkSelectInputDaisy) -> u8 {
        Sai4IppIndSaiTxbclkSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai4IppIndSaiTxsyncSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_23 for Mode: ALT0"]
    SELECT_GPIO_AD_23_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_09 for Mode: ALT12"]
    SELECT_GPIO_B1_09_ALT12 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_02 for Mode: ALT4"]
    SELECT_GPIO_B2_02_ALT4 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sai4IppIndSaiTxsyncSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai4IppIndSaiTxsyncSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai4IppIndSaiTxsyncSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Sai4IppIndSaiTxsyncSelectInputDaisy {
        Sai4IppIndSaiTxsyncSelectInputDaisy::from_bits(val)
    }
}
impl From<Sai4IppIndSaiTxsyncSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Sai4IppIndSaiTxsyncSelectInputDaisy) -> u8 {
        Sai4IppIndSaiTxsyncSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sinc1IppIndEmbitSelectInput0Daisy {
    #[doc = "Selecting Pad: GPIO_AD_04 for Mode: ALT6"]
    SELECT_GPIO_AD_04_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_21 for Mode: ALT3"]
    SELECT_GPIO_AD_21_ALT3 = 0x01,
}
impl Sinc1IppIndEmbitSelectInput0Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sinc1IppIndEmbitSelectInput0Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sinc1IppIndEmbitSelectInput0Daisy {
    #[inline(always)]
    fn from(val: u8) -> Sinc1IppIndEmbitSelectInput0Daisy {
        Sinc1IppIndEmbitSelectInput0Daisy::from_bits(val)
    }
}
impl From<Sinc1IppIndEmbitSelectInput0Daisy> for u8 {
    #[inline(always)]
    fn from(val: Sinc1IppIndEmbitSelectInput0Daisy) -> u8 {
        Sinc1IppIndEmbitSelectInput0Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sinc1IppIndEmbitSelectInput1Daisy {
    #[doc = "Selecting Pad: GPIO_AD_06 for Mode: ALT6"]
    SELECT_GPIO_AD_06_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_23 for Mode: ALT3"]
    SELECT_GPIO_AD_23_ALT3 = 0x01,
}
impl Sinc1IppIndEmbitSelectInput1Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sinc1IppIndEmbitSelectInput1Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sinc1IppIndEmbitSelectInput1Daisy {
    #[inline(always)]
    fn from(val: u8) -> Sinc1IppIndEmbitSelectInput1Daisy {
        Sinc1IppIndEmbitSelectInput1Daisy::from_bits(val)
    }
}
impl From<Sinc1IppIndEmbitSelectInput1Daisy> for u8 {
    #[inline(always)]
    fn from(val: Sinc1IppIndEmbitSelectInput1Daisy) -> u8 {
        Sinc1IppIndEmbitSelectInput1Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sinc1IppIndEmbitSelectInput2Daisy {
    #[doc = "Selecting Pad: GPIO_AD_08 for Mode: ALT6"]
    SELECT_GPIO_AD_08_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B1_01 for Mode: ALT1"]
    SELECT_GPIO_SD_B1_01_ALT1 = 0x01,
}
impl Sinc1IppIndEmbitSelectInput2Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sinc1IppIndEmbitSelectInput2Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sinc1IppIndEmbitSelectInput2Daisy {
    #[inline(always)]
    fn from(val: u8) -> Sinc1IppIndEmbitSelectInput2Daisy {
        Sinc1IppIndEmbitSelectInput2Daisy::from_bits(val)
    }
}
impl From<Sinc1IppIndEmbitSelectInput2Daisy> for u8 {
    #[inline(always)]
    fn from(val: Sinc1IppIndEmbitSelectInput2Daisy) -> u8 {
        Sinc1IppIndEmbitSelectInput2Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sinc1IppIndEmbitSelectInput3Daisy {
    #[doc = "Selecting Pad: GPIO_AD_10 for Mode: ALT6"]
    SELECT_GPIO_AD_10_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B1_03 for Mode: ALT1"]
    SELECT_GPIO_SD_B1_03_ALT1 = 0x01,
}
impl Sinc1IppIndEmbitSelectInput3Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sinc1IppIndEmbitSelectInput3Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sinc1IppIndEmbitSelectInput3Daisy {
    #[inline(always)]
    fn from(val: u8) -> Sinc1IppIndEmbitSelectInput3Daisy {
        Sinc1IppIndEmbitSelectInput3Daisy::from_bits(val)
    }
}
impl From<Sinc1IppIndEmbitSelectInput3Daisy> for u8 {
    #[inline(always)]
    fn from(val: Sinc1IppIndEmbitSelectInput3Daisy) -> u8 {
        Sinc1IppIndEmbitSelectInput3Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sinc1IppIndEmclkSelectInput0Daisy {
    #[doc = "Selecting Pad: GPIO_AD_03 for Mode: ALT6"]
    SELECT_GPIO_AD_03_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_20 for Mode: ALT3"]
    SELECT_GPIO_AD_20_ALT3 = 0x01,
}
impl Sinc1IppIndEmclkSelectInput0Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sinc1IppIndEmclkSelectInput0Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sinc1IppIndEmclkSelectInput0Daisy {
    #[inline(always)]
    fn from(val: u8) -> Sinc1IppIndEmclkSelectInput0Daisy {
        Sinc1IppIndEmclkSelectInput0Daisy::from_bits(val)
    }
}
impl From<Sinc1IppIndEmclkSelectInput0Daisy> for u8 {
    #[inline(always)]
    fn from(val: Sinc1IppIndEmclkSelectInput0Daisy) -> u8 {
        Sinc1IppIndEmclkSelectInput0Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sinc1IppIndEmclkSelectInput1Daisy {
    #[doc = "Selecting Pad: GPIO_AD_05 for Mode: ALT6"]
    SELECT_GPIO_AD_05_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_22 for Mode: ALT3"]
    SELECT_GPIO_AD_22_ALT3 = 0x01,
}
impl Sinc1IppIndEmclkSelectInput1Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sinc1IppIndEmclkSelectInput1Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sinc1IppIndEmclkSelectInput1Daisy {
    #[inline(always)]
    fn from(val: u8) -> Sinc1IppIndEmclkSelectInput1Daisy {
        Sinc1IppIndEmclkSelectInput1Daisy::from_bits(val)
    }
}
impl From<Sinc1IppIndEmclkSelectInput1Daisy> for u8 {
    #[inline(always)]
    fn from(val: Sinc1IppIndEmclkSelectInput1Daisy) -> u8 {
        Sinc1IppIndEmclkSelectInput1Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sinc1IppIndEmclkSelectInput2Daisy {
    #[doc = "Selecting Pad: GPIO_AD_07 for Mode: ALT6"]
    SELECT_GPIO_AD_07_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B1_00 for Mode: ALT1"]
    SELECT_GPIO_SD_B1_00_ALT1 = 0x01,
}
impl Sinc1IppIndEmclkSelectInput2Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sinc1IppIndEmclkSelectInput2Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sinc1IppIndEmclkSelectInput2Daisy {
    #[inline(always)]
    fn from(val: u8) -> Sinc1IppIndEmclkSelectInput2Daisy {
        Sinc1IppIndEmclkSelectInput2Daisy::from_bits(val)
    }
}
impl From<Sinc1IppIndEmclkSelectInput2Daisy> for u8 {
    #[inline(always)]
    fn from(val: Sinc1IppIndEmclkSelectInput2Daisy) -> u8 {
        Sinc1IppIndEmclkSelectInput2Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sinc1IppIndEmclkSelectInput3Daisy {
    #[doc = "Selecting Pad: GPIO_AD_09 for Mode: ALT6"]
    SELECT_GPIO_AD_09_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B1_02 for Mode: ALT1"]
    SELECT_GPIO_SD_B1_02_ALT1 = 0x01,
}
impl Sinc1IppIndEmclkSelectInput3Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sinc1IppIndEmclkSelectInput3Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sinc1IppIndEmclkSelectInput3Daisy {
    #[inline(always)]
    fn from(val: u8) -> Sinc1IppIndEmclkSelectInput3Daisy {
        Sinc1IppIndEmclkSelectInput3Daisy::from_bits(val)
    }
}
impl From<Sinc1IppIndEmclkSelectInput3Daisy> for u8 {
    #[inline(always)]
    fn from(val: Sinc1IppIndEmclkSelectInput3Daisy) -> u8 {
        Sinc1IppIndEmclkSelectInput3Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sinc2IppIndEmbitSelectInput2Daisy {
    #[doc = "Selecting Pad: GPIO_AD_31 for Mode: ALT3"]
    SELECT_GPIO_AD_31_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B1_05 for Mode: ALT2"]
    SELECT_GPIO_SD_B1_05_ALT2 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_09 for Mode: ALT1"]
    SELECT_GPIO_B2_09_ALT1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sinc2IppIndEmbitSelectInput2Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sinc2IppIndEmbitSelectInput2Daisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sinc2IppIndEmbitSelectInput2Daisy {
    #[inline(always)]
    fn from(val: u8) -> Sinc2IppIndEmbitSelectInput2Daisy {
        Sinc2IppIndEmbitSelectInput2Daisy::from_bits(val)
    }
}
impl From<Sinc2IppIndEmbitSelectInput2Daisy> for u8 {
    #[inline(always)]
    fn from(val: Sinc2IppIndEmbitSelectInput2Daisy) -> u8 {
        Sinc2IppIndEmbitSelectInput2Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sinc2IppIndEmbitSelectInput3Daisy {
    #[doc = "Selecting Pad: GPIO_AD_33 for Mode: ALT3"]
    SELECT_GPIO_AD_33_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_11 for Mode: ALT1"]
    SELECT_GPIO_B2_11_ALT1 = 0x01,
}
impl Sinc2IppIndEmbitSelectInput3Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sinc2IppIndEmbitSelectInput3Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sinc2IppIndEmbitSelectInput3Daisy {
    #[inline(always)]
    fn from(val: u8) -> Sinc2IppIndEmbitSelectInput3Daisy {
        Sinc2IppIndEmbitSelectInput3Daisy::from_bits(val)
    }
}
impl From<Sinc2IppIndEmbitSelectInput3Daisy> for u8 {
    #[inline(always)]
    fn from(val: Sinc2IppIndEmbitSelectInput3Daisy) -> u8 {
        Sinc2IppIndEmbitSelectInput3Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sinc2IppIndEmclkSelectInput0Daisy {
    #[doc = "Selecting Pad: GPIO_AD_26 for Mode: ALT3"]
    SELECT_GPIO_AD_26_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_13 for Mode: ALT1"]
    SELECT_GPIO_B2_13_ALT1 = 0x01,
}
impl Sinc2IppIndEmclkSelectInput0Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sinc2IppIndEmclkSelectInput0Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sinc2IppIndEmclkSelectInput0Daisy {
    #[inline(always)]
    fn from(val: u8) -> Sinc2IppIndEmclkSelectInput0Daisy {
        Sinc2IppIndEmclkSelectInput0Daisy::from_bits(val)
    }
}
impl From<Sinc2IppIndEmclkSelectInput0Daisy> for u8 {
    #[inline(always)]
    fn from(val: Sinc2IppIndEmclkSelectInput0Daisy) -> u8 {
        Sinc2IppIndEmclkSelectInput0Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sinc2IppIndEmclkSelectInput2Daisy {
    #[doc = "Selecting Pad: GPIO_AD_30 for Mode: ALT3"]
    SELECT_GPIO_AD_30_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B1_04 for Mode: ALT2"]
    SELECT_GPIO_SD_B1_04_ALT2 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_08 for Mode: ALT1"]
    SELECT_GPIO_B2_08_ALT1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sinc2IppIndEmclkSelectInput2Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sinc2IppIndEmclkSelectInput2Daisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sinc2IppIndEmclkSelectInput2Daisy {
    #[inline(always)]
    fn from(val: u8) -> Sinc2IppIndEmclkSelectInput2Daisy {
        Sinc2IppIndEmclkSelectInput2Daisy::from_bits(val)
    }
}
impl From<Sinc2IppIndEmclkSelectInput2Daisy> for u8 {
    #[inline(always)]
    fn from(val: Sinc2IppIndEmclkSelectInput2Daisy) -> u8 {
        Sinc2IppIndEmclkSelectInput2Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sinc2IppIndEmclkSelectInput3Daisy {
    #[doc = "Selecting Pad: GPIO_AD_32 for Mode: ALT3"]
    SELECT_GPIO_AD_32_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_10 for Mode: ALT1"]
    SELECT_GPIO_B2_10_ALT1 = 0x01,
}
impl Sinc2IppIndEmclkSelectInput3Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sinc2IppIndEmclkSelectInput3Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sinc2IppIndEmclkSelectInput3Daisy {
    #[inline(always)]
    fn from(val: u8) -> Sinc2IppIndEmclkSelectInput3Daisy {
        Sinc2IppIndEmclkSelectInput3Daisy::from_bits(val)
    }
}
impl From<Sinc2IppIndEmclkSelectInput3Daisy> for u8 {
    #[inline(always)]
    fn from(val: Sinc2IppIndEmclkSelectInput3Daisy) -> u8 {
        Sinc2IppIndEmclkSelectInput3Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpdifSpdifIn1SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_12 for Mode: ALT2"]
    SELECT_GPIO_EMC_B2_12_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_15 for Mode: ALT0"]
    SELECT_GPIO_AD_15_ALT0 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_08 for Mode: ALT9"]
    SELECT_GPIO_B2_08_ALT9 = 0x02,
    _RESERVED_3 = 0x03,
}
impl SpdifSpdifIn1SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SpdifSpdifIn1SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SpdifSpdifIn1SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> SpdifSpdifIn1SelectInputDaisy {
        SpdifSpdifIn1SelectInputDaisy::from_bits(val)
    }
}
impl From<SpdifSpdifIn1SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: SpdifSpdifIn1SelectInputDaisy) -> u8 {
        SpdifSpdifIn1SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sre {
    #[doc = "Fast Slew Rate"]
    SRE_0_FAST_SLEW_RATE = 0x0,
    #[doc = "Slow Slew Rate"]
    SRE_1_SLOW_SLEW_RATE = 0x01,
}
impl Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sre {
    #[inline(always)]
    fn from(val: u8) -> Sre {
        Sre::from_bits(val)
    }
}
impl From<Sre> for u8 {
    #[inline(always)]
    fn from(val: Sre) -> u8 {
        Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAd00MuxMode {
    _RESERVED_0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: CAN2_TX of instance: can2"]
    ALT1_CAN2_TX = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: MIC_CLK of instance: mic"]
    ALT2_MIC_CLK = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: GPT2_CAPTURE1 of instance: gpt2"]
    ALT3_GPT2_CAPTURE1 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM1_PWMA00 of instance: flexpwm1"]
    ALT4_FLEXPWM1_PWMA0 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO00 of instance: gpio4"]
    ALT5_GPIO4_IO0 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SINC1_MOD_CLK0 of instance: sinc1"]
    ALT6_SINC1_MOD_CLK0 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO00 of instance: flexio2"]
    ALT8_FLEXIO2_FLEXIO0 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: QTIMER4_TIMER0 of instance: qtimer4"]
    ALT9_QTIMER4_TIMER0 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAd00MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAd00MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAd00MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAd00MuxMode {
        SwMuxCtlPadGpioAd00MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAd00MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAd00MuxMode) -> u8 {
        SwMuxCtlPadGpioAd00MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAd01MuxMode {
    _RESERVED_0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: CAN2_RX of instance: can2"]
    ALT1_CAN2_RX = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: MIC_BITSTREAM00 of instance: mic"]
    ALT2_MIC_BITSTREAM0 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: GPT2_CAPTURE2 of instance: gpt2"]
    ALT3_GPT2_CAPTURE2 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM1_PWMB00 of instance: flexpwm1"]
    ALT4_FLEXPWM1_PWMB0 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO01 of instance: gpio4"]
    ALT5_GPIO4_IO1 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SINC1_MOD_CLK1 of instance: sinc1"]
    ALT6_SINC1_MOD_CLK1 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO01 of instance: flexio2"]
    ALT8_FLEXIO2_FLEXIO1 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: QTIMER4_TIMER1 of instance: qtimer4"]
    ALT9_QTIMER4_TIMER1 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAd01MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAd01MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAd01MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAd01MuxMode {
        SwMuxCtlPadGpioAd01MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAd01MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAd01MuxMode) -> u8 {
        SwMuxCtlPadGpioAd01MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAd02MuxMode {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: MIC_BITSTREAM01 of instance: mic"]
    ALT2_MIC_BITSTREAM1 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: GPT2_COMPARE1 of instance: gpt2"]
    ALT3_GPT2_COMPARE1 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM1_PWMA01 of instance: flexpwm1"]
    ALT4_FLEXPWM1_PWMA1 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO02 of instance: gpio4"]
    ALT5_GPIO4_IO2 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SINC1_MOD_CLK2 of instance: sinc1"]
    ALT6_SINC1_MOD_CLK2 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO02 of instance: flexio2"]
    ALT8_FLEXIO2_FLEXIO2 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: QTIMER4_TIMER2 of instance: qtimer4"]
    ALT9_QTIMER4_TIMER2 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAd02MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAd02MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAd02MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAd02MuxMode {
        SwMuxCtlPadGpioAd02MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAd02MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAd02MuxMode) -> u8 {
        SwMuxCtlPadGpioAd02MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAd03MuxMode {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: MIC_BITSTREAM02 of instance: mic"]
    ALT2_MIC_BITSTREAM2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: GPT2_COMPARE2 of instance: gpt2"]
    ALT3_GPT2_COMPARE2 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM1_PWMB01 of instance: flexpwm1"]
    ALT4_FLEXPWM1_PWMB1 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO03 of instance: gpio4"]
    ALT5_GPIO4_IO3 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SINC1_EMCLK00 of instance: sinc1"]
    ALT6_SINC1_EMCLK0 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO03 of instance: flexio2"]
    ALT8_FLEXIO2_FLEXIO3 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: QTIMER4_TIMER3 of instance: qtimer4"]
    ALT9_QTIMER4_TIMER3 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAd03MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAd03MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAd03MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAd03MuxMode {
        SwMuxCtlPadGpioAd03MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAd03MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAd03MuxMode) -> u8 {
        SwMuxCtlPadGpioAd03MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAd04MuxMode {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: MIC_BITSTREAM03 of instance: mic"]
    ALT2_MIC_BITSTREAM3 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: GPT2_COMPARE3 of instance: gpt2"]
    ALT3_GPT2_COMPARE3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM1_PWMB02 of instance: flexpwm1"]
    ALT4_FLEXPWM1_PWMB2 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO04 of instance: gpio4"]
    ALT5_GPIO4_IO4 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SINC1_EMBIT00 of instance: sinc1"]
    ALT6_SINC1_EMBIT0 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO04 of instance: flexio2"]
    ALT8_FLEXIO2_FLEXIO4 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: QTIMER5_TIMER0 of instance: qtimer5"]
    ALT9_QTIMER5_TIMER0 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAd04MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAd04MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAd04MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAd04MuxMode {
        SwMuxCtlPadGpioAd04MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAd04MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAd04MuxMode) -> u8 {
        SwMuxCtlPadGpioAd04MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAd05MuxMode {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: GPT2_CLK of instance: gpt2"]
    ALT3_GPT2_CLK = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM1_PWMA02 of instance: flexpwm1"]
    ALT4_FLEXPWM1_PWMA2 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO05 of instance: gpio4"]
    ALT5_GPIO4_IO5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SINC1_EMCLK01 of instance: sinc1"]
    ALT6_SINC1_EMCLK1 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: CCM_ENET_REF_CLK_25M of instance: ccm"]
    ALT7_CCM_ENET_REF_CLK_25M = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO05 of instance: flexio2"]
    ALT8_FLEXIO2_FLEXIO5 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: QTIMER5_TIMER1 of instance: qtimer5"]
    ALT9_QTIMER5_TIMER1 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAd05MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAd05MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAd05MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAd05MuxMode {
        SwMuxCtlPadGpioAd05MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAd05MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAd05MuxMode) -> u8 {
        SwMuxCtlPadGpioAd05MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAd06MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: USB_OTG2_OC of instance: usb"]
    ALT0_USB_OTG2_OC = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: CAN3_TX of instance: can3"]
    ALT1_CAN3_TX = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM1_PWMX00 of instance: flexpwm1"]
    ALT4_FLEXPWM1_PWMX0 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO06 of instance: gpio4"]
    ALT5_GPIO4_IO6 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SINC1_EMBIT01 of instance: sinc1"]
    ALT6_SINC1_EMBIT1 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO06 of instance: flexio2"]
    ALT8_FLEXIO2_FLEXIO6 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: QTIMER5_TIMER2 of instance: qtimer5"]
    ALT9_QTIMER5_TIMER2 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAd06MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAd06MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAd06MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAd06MuxMode {
        SwMuxCtlPadGpioAd06MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAd06MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAd06MuxMode) -> u8 {
        SwMuxCtlPadGpioAd06MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAd07MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: USB_OTG2_PWR of instance: usb"]
    ALT0_USB_OTG2_PWR = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: CAN3_RX of instance: can3"]
    ALT1_CAN3_RX = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM1_PWMX01 of instance: flexpwm1"]
    ALT4_FLEXPWM1_PWMX1 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO07 of instance: gpio4"]
    ALT5_GPIO4_IO7 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SINC1_EMCLK02 of instance: sinc1"]
    ALT6_SINC1_EMCLK2 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO07 of instance: flexio2"]
    ALT8_FLEXIO2_FLEXIO7 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: QTIMER5_TIMER3 of instance: qtimer5"]
    ALT9_QTIMER5_TIMER3 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAd07MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAd07MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAd07MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAd07MuxMode {
        SwMuxCtlPadGpioAd07MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAd07MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAd07MuxMode) -> u8 {
        SwMuxCtlPadGpioAd07MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAd08MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: USBPHY2_OTG_ID of instance: usbphy2"]
    ALT0_USBPHY2_OTG_ID = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPI2C5_SCL of instance: lpi2c5"]
    ALT1_LPI2C5_SCL = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM1_PWMX02 of instance: flexpwm1"]
    ALT4_FLEXPWM1_PWMX2 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO08 of instance: gpio4"]
    ALT5_GPIO4_IO8 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SINC1_EMBIT02 of instance: sinc1"]
    ALT6_SINC1_EMBIT2 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO08 of instance: flexio2"]
    ALT8_FLEXIO2_FLEXIO8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: QTIMER6_TIMER0 of instance: qtimer6"]
    ALT9_QTIMER6_TIMER0 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAd08MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAd08MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAd08MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAd08MuxMode {
        SwMuxCtlPadGpioAd08MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAd08MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAd08MuxMode) -> u8 {
        SwMuxCtlPadGpioAd08MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAd09MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: USBPHY1_OTG_ID of instance: usbphy1"]
    ALT0_USBPHY1_OTG_ID = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPI2C5_SDA of instance: lpi2c5"]
    ALT1_LPI2C5_SDA = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM1_PWMX03 of instance: flexpwm1"]
    ALT4_FLEXPWM1_PWMX3 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO09 of instance: gpio4"]
    ALT5_GPIO4_IO9 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SINC1_EMCLK03 of instance: sinc1"]
    ALT6_SINC1_EMCLK3 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO09 of instance: flexio2"]
    ALT8_FLEXIO2_FLEXIO9 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: QTIMER6_TIMER1 of instance: qtimer6"]
    ALT9_QTIMER6_TIMER1 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAd09MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAd09MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAd09MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAd09MuxMode {
        SwMuxCtlPadGpioAd09MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAd09MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAd09MuxMode) -> u8 {
        SwMuxCtlPadGpioAd09MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAd10MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: USB_OTG1_PWR of instance: usb"]
    ALT0_USB_OTG1_PWR = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM2_PWMX00 of instance: flexpwm2"]
    ALT4_FLEXPWM2_PWMX0 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO10 of instance: gpio4"]
    ALT5_GPIO4_IO10 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SINC1_EMBIT03 of instance: sinc1"]
    ALT6_SINC1_EMBIT3 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO10 of instance: flexio2"]
    ALT8_FLEXIO2_FLEXIO10 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: QTIMER6_TIMER2 of instance: qtimer6"]
    ALT9_QTIMER6_TIMER2 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAd10MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAd10MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAd10MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAd10MuxMode {
        SwMuxCtlPadGpioAd10MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAd10MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAd10MuxMode) -> u8 {
        SwMuxCtlPadGpioAd10MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAd11MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: USB_OTG1_OC of instance: usb"]
    ALT0_USB_OTG1_OC = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM2_PWMX01 of instance: flexpwm2"]
    ALT4_FLEXPWM2_PWMX1 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO11 of instance: gpio4"]
    ALT5_GPIO4_IO11 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SINC_FILTER_GLUE1_BREAK of instance: sinc_filter_glue1"]
    ALT6_SINC_FILTER_GLUE1_BREAK = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO11 of instance: flexio2"]
    ALT8_FLEXIO2_FLEXIO11 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: QTIMER6_TIMER3 of instance: qtimer6"]
    ALT9_QTIMER6_TIMER3 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAd11MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAd11MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAd11MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAd11MuxMode {
        SwMuxCtlPadGpioAd11MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAd11MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAd11MuxMode) -> u8 {
        SwMuxCtlPadGpioAd11MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAd12MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SPDIF_LOCK of instance: spdif"]
    ALT0_SPDIF_LOCK = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPI2C5_SCLS of instance: lpi2c5"]
    ALT1_LPI2C5_SCLS = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: GPT1_CAPTURE1 of instance: gpt1"]
    ALT2_GPT1_CAPTURE1 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: KPP_ROW07 of instance: kpp"]
    ALT3_KPP_ROW7 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM2_PWMX02 of instance: flexpwm2"]
    ALT4_FLEXPWM2_PWMX2 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO12 of instance: gpio4"]
    ALT5_GPIO4_IO12 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_XBAR_INOUT18 of instance: xbar1"]
    ALT6_XBAR1_XBAR_INOUT18 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: EWM_EWM_OUT_B of instance: ewm"]
    ALT7_EWM_EWM_OUT_B = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO12 of instance: flexio2"]
    ALT8_FLEXIO2_FLEXIO12 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAd12MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAd12MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAd12MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAd12MuxMode {
        SwMuxCtlPadGpioAd12MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAd12MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAd12MuxMode) -> u8 {
        SwMuxCtlPadGpioAd12MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAd13MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SPDIF_SR_CLK of instance: spdif"]
    ALT0_SPDIF_SR_CLK = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPI2C5_SDAS of instance: lpi2c5"]
    ALT1_LPI2C5_SDAS = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: GPT1_CAPTURE2 of instance: gpt1"]
    ALT2_GPT1_CAPTURE2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: KPP_COL07 of instance: kpp"]
    ALT3_KPP_COL7 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM2_PWMX03 of instance: flexpwm2"]
    ALT4_FLEXPWM2_PWMX3 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO13 of instance: gpio4"]
    ALT5_GPIO4_IO13 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPUART3_TX of instance: lpuart3"]
    ALT6_LPUART3_TX = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: USDHC2_CD_B of instance: usdhc2"]
    ALT7_USDHC2_CD_B = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO13 of instance: flexio2"]
    ALT8_FLEXIO2_FLEXIO13 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAd13MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAd13MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAd13MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAd13MuxMode {
        SwMuxCtlPadGpioAd13MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAd13MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAd13MuxMode) -> u8 {
        SwMuxCtlPadGpioAd13MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAd14MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SPDIF_EXT_CLK of instance: spdif"]
    ALT0_SPDIF_EXT_CLK = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPI2C5_HREQ of instance: lpi2c5"]
    ALT1_LPI2C5_HREQ = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: GPT1_COMPARE1 of instance: gpt1"]
    ALT2_GPT1_COMPARE1 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: KPP_ROW06 of instance: kpp"]
    ALT3_KPP_ROW6 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM3_PWMX00 of instance: flexpwm3"]
    ALT4_FLEXPWM3_PWMX0 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO14 of instance: gpio4"]
    ALT5_GPIO4_IO14 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPUART3_RX of instance: lpuart3"]
    ALT6_LPUART3_RX = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: USDHC2_WP of instance: usdhc2"]
    ALT7_USDHC2_WP = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO14 of instance: flexio2"]
    ALT8_FLEXIO2_FLEXIO14 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAd14MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAd14MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAd14MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAd14MuxMode {
        SwMuxCtlPadGpioAd14MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAd14MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAd14MuxMode) -> u8 {
        SwMuxCtlPadGpioAd14MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAd15MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SPDIF_IN of instance: spdif"]
    ALT0_SPDIF_IN = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPUART10_TX of instance: lpuart10"]
    ALT1_LPUART10_TX = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: GPT1_COMPARE2 of instance: gpt1"]
    ALT2_GPT1_COMPARE2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: KPP_COL06 of instance: kpp"]
    ALT3_KPP_COL6 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM3_PWMX01 of instance: flexpwm3"]
    ALT4_FLEXPWM3_PWMX1 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO15 of instance: gpio4"]
    ALT5_GPIO4_IO15 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPUART3_CTS_B of instance: lpuart3"]
    ALT6_LPUART3_CTS_B = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: LPSPI3_PCS1 of instance: lpspi3"]
    ALT7_LPSPI3_PCS1 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO15 of instance: flexio2"]
    ALT8_FLEXIO2_FLEXIO15 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: CAN1_TX of instance: can1"]
    ALT9_CAN1_TX = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_CLK_ECAT_CLK25 of instance: ecat"]
    ALT12_ECAT_CLK_ECAT_CLK25 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAd15MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAd15MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAd15MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAd15MuxMode {
        SwMuxCtlPadGpioAd15MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAd15MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAd15MuxMode) -> u8 {
        SwMuxCtlPadGpioAd15MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAd16MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SPDIF_OUT of instance: spdif"]
    ALT0_SPDIF_OUT = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPUART10_RX of instance: lpuart10"]
    ALT1_LPUART10_RX = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: GPT1_COMPARE3 of instance: gpt1"]
    ALT2_GPT1_COMPARE3 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: KPP_ROW05 of instance: kpp"]
    ALT3_KPP_ROW5 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM3_PWMX02 of instance: flexpwm3"]
    ALT4_FLEXPWM3_PWMX2 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO16 of instance: gpio4"]
    ALT5_GPIO4_IO16 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPUART3_RTS_B of instance: lpuart3"]
    ALT6_LPUART3_RTS_B = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: LPSPI3_SCK of instance: lpspi3"]
    ALT7_LPSPI3_SCK = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO16 of instance: flexio2"]
    ALT8_FLEXIO2_FLEXIO16 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: CAN1_RX of instance: can1"]
    ALT9_CAN1_RX = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_LINK_0 of instance: ecat"]
    ALT12_ECAT_LINK0 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAd16MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAd16MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAd16MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAd16MuxMode {
        SwMuxCtlPadGpioAd16MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAd16MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAd16MuxMode) -> u8 {
        SwMuxCtlPadGpioAd16MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAd17MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SAI4_MCLK of instance: sai4"]
    ALT0_SAI4_MCLK = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: ACMP1_CMPO of instance: acmp1"]
    ALT1_ACMP1_CMPO = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: GPT1_CLK of instance: gpt1"]
    ALT2_GPT1_CLK = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: KPP_COL05 of instance: kpp"]
    ALT3_KPP_COL5 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM3_PWMX03 of instance: flexpwm3"]
    ALT4_FLEXPWM3_PWMX3 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO17 of instance: gpio4"]
    ALT5_GPIO4_IO17 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: I3C2_PUR of instance: i3c2"]
    ALT6_I3C2_PUR = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: LPSPI3_PCS0 of instance: lpspi3"]
    ALT7_LPSPI3_PCS0 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO17 of instance: flexio2"]
    ALT8_FLEXIO2_FLEXIO17 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPI2C3_HREQ of instance: lpi2c3"]
    ALT9_LPI2C3_HREQ = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_LINK_1 of instance: ecat"]
    ALT12_ECAT_LINK1 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAd17MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAd17MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAd17MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAd17MuxMode {
        SwMuxCtlPadGpioAd17MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAd17MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAd17MuxMode) -> u8 {
        SwMuxCtlPadGpioAd17MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAd18MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SAI4_RX_SYNC of instance: sai4"]
    ALT0_SAI4_RX_SYNC = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: ACMP2_CMPO of instance: acmp2"]
    ALT1_ACMP2_CMPO = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART5_RI_B of instance: lpuart5"]
    ALT2_LPUART5_RI_B = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: KPP_ROW04 of instance: kpp"]
    ALT3_KPP_ROW4 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM4_PWMX00 of instance: flexpwm4"]
    ALT4_FLEXPWM4_PWMX0 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO18 of instance: gpio4"]
    ALT5_GPIO4_IO18 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: I3C2_SCL of instance: i3c2"]
    ALT6_I3C2_SCL = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: LPSPI3_SDO of instance: lpspi3"]
    ALT7_LPSPI3_SDO = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO18 of instance: flexio2"]
    ALT8_FLEXIO2_FLEXIO18 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPI2C3_SCL of instance: lpi2c3"]
    ALT9_LPI2C3_SCL = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_PROM_CLK of instance: ecat"]
    ALT12_ECAT_SCL = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAd18MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAd18MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAd18MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAd18MuxMode {
        SwMuxCtlPadGpioAd18MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAd18MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAd18MuxMode) -> u8 {
        SwMuxCtlPadGpioAd18MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAd19MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SAI4_RX_BCLK of instance: sai4"]
    ALT0_SAI4_RX_BCLK = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: ACMP3_CMPO of instance: acmp3"]
    ALT1_ACMP3_CMPO = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XBAR1_XBAR_INOUT19 of instance: xbar1"]
    ALT2_XBAR1_XBAR_INOUT19 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: KPP_COL04 of instance: kpp"]
    ALT3_KPP_COL4 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM4_PWMX01 of instance: flexpwm4"]
    ALT4_FLEXPWM4_PWMX1 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO19 of instance: gpio4"]
    ALT5_GPIO4_IO19 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: I3C2_SDA of instance: i3c2"]
    ALT6_I3C2_SDA = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: LPSPI3_SDI of instance: lpspi3"]
    ALT7_LPSPI3_SDI = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO19 of instance: flexio2"]
    ALT8_FLEXIO2_FLEXIO19 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPI2C3_SDA of instance: lpi2c3"]
    ALT9_LPI2C3_SDA = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_PROM_DATA of instance: ecat"]
    ALT12_ECAT_SDA = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAd19MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAd19MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAd19MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAd19MuxMode {
        SwMuxCtlPadGpioAd19MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAd19MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAd19MuxMode) -> u8 {
        SwMuxCtlPadGpioAd19MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAd20MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SAI4_RX_DATA00 of instance: sai4"]
    ALT0_SAI4_RX_DATA0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: ACMP4_CMPO of instance: acmp4"]
    ALT1_ACMP4_CMPO = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPIT2_TRIGGER00 of instance: lpit2"]
    ALT2_LPIT2_TRIGGER0 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SINC1_EMCLK00 of instance: sinc1"]
    ALT3_SINC1_EMCLK0 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM4_PWMX02 of instance: flexpwm4"]
    ALT4_FLEXPWM4_PWMX2 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO20 of instance: gpio4"]
    ALT5_GPIO4_IO20 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: NETC_TMR_TRIG1 of instance: netc"]
    ALT6_NETC_TMR_1588_TRIG1 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: NETC_1588_CLK of instance: netc_clkgen"]
    ALT7_NETC_CLKGEN_TMR_1588_CLK = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO20 of instance: flexio2"]
    ALT8_FLEXIO2_FLEXIO20 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAd20MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAd20MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAd20MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAd20MuxMode {
        SwMuxCtlPadGpioAd20MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAd20MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAd20MuxMode) -> u8 {
        SwMuxCtlPadGpioAd20MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAd21MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SAI4_TX_DATA00 of instance: sai4"]
    ALT0_SAI4_TX_DATA0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPIT2_TRIGGER01 of instance: lpit2"]
    ALT2_LPIT2_TRIGGER1 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SINC1_EMBIT00 of instance: sinc1"]
    ALT3_SINC1_EMBIT0 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM4_PWMX03 of instance: flexpwm4"]
    ALT4_FLEXPWM4_PWMX3 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO21 of instance: gpio4"]
    ALT5_GPIO4_IO21 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: NETC_TMR_TRIG2 of instance: netc"]
    ALT6_NETC_TMR_1588_TRIG2 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: NETC_TMR_GCLK of instance: netc"]
    ALT7_NETC_TMR_1588_GCLK = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO21 of instance: flexio2"]
    ALT8_FLEXIO2_FLEXIO21 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_LED_RUN of instance: ecat"]
    ALT12_ECAT_LED_RUN = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAd21MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAd21MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAd21MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAd21MuxMode {
        SwMuxCtlPadGpioAd21MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAd21MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAd21MuxMode) -> u8 {
        SwMuxCtlPadGpioAd21MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAd22MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SAI4_TX_BCLK of instance: sai4"]
    ALT0_SAI4_TX_BCLK = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPIT2_TRIGGER02 of instance: lpit2"]
    ALT2_LPIT2_TRIGGER2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SINC1_EMCLK01 of instance: sinc1"]
    ALT3_SINC1_EMCLK1 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO22 of instance: gpio4"]
    ALT5_GPIO4_IO22 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: NETC_TMR_ALARM1 of instance: netc"]
    ALT7_NETC_TMR_1588_ALARM1 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO22 of instance: flexio2"]
    ALT8_FLEXIO2_FLEXIO22 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_LED_ERR of instance: ecat"]
    ALT12_ECAT_LED_ERR = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAd22MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAd22MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAd22MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAd22MuxMode {
        SwMuxCtlPadGpioAd22MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAd22MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAd22MuxMode) -> u8 {
        SwMuxCtlPadGpioAd22MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAd23MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SAI4_TX_SYNC of instance: sai4"]
    ALT0_SAI4_TX_SYNC = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPIT2_TRIGGER03 of instance: lpit2"]
    ALT2_LPIT2_TRIGGER3 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SINC1_EMBIT01 of instance: sinc1"]
    ALT3_SINC1_EMBIT1 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO23 of instance: gpio4"]
    ALT5_GPIO4_IO23 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: NETC_TMR_ALARM2 of instance: netc"]
    ALT7_NETC_TMR_1588_ALARM2 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO23 of instance: flexio2"]
    ALT8_FLEXIO2_FLEXIO23 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_LED_STATE_RUN of instance: ecat"]
    ALT12_ECAT_LED_STATE_RUN = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAd23MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAd23MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAd23MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAd23MuxMode {
        SwMuxCtlPadGpioAd23MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAd23MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAd23MuxMode) -> u8 {
        SwMuxCtlPadGpioAd23MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAd24MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: LPUART6_TX of instance: lpuart6"]
    ALT0_LPUART6_TX = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPI2C4_SCL of instance: lpi2c4"]
    ALT1_LPI2C4_SCL = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SINC2_MOD_CLK1 of instance: sinc2"]
    ALT3_SINC2_MOD_CLK1 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM2_PWMA00 of instance: flexpwm2"]
    ALT4_FLEXPWM2_PWMA0 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO24 of instance: gpio4"]
    ALT5_GPIO4_IO24 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: NETC_TMR_TRIG1 of instance: netc"]
    ALT7_NETC_TMR_1588_TRIG1 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO24 of instance: flexio2"]
    ALT8_FLEXIO2_FLEXIO24 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_LINK_ACT00 of instance: ecat"]
    ALT12_ECAT_LINK_ACT0 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAd24MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAd24MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAd24MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAd24MuxMode {
        SwMuxCtlPadGpioAd24MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAd24MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAd24MuxMode) -> u8 {
        SwMuxCtlPadGpioAd24MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAd25MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: LPUART6_RX of instance: lpuart6"]
    ALT0_LPUART6_RX = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPI2C4_SDA of instance: lpi2c4"]
    ALT1_LPI2C4_SDA = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPSPI5_PCS3 of instance: lpspi5"]
    ALT2_LPSPI5_PCS3 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SINC2_MOD_CLK2 of instance: sinc2"]
    ALT3_SINC2_MOD_CLK2 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM2_PWMB00 of instance: flexpwm2"]
    ALT4_FLEXPWM2_PWMB0 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO25 of instance: gpio4"]
    ALT5_GPIO4_IO25 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: NETC_TMR_TRIG2 of instance: netc"]
    ALT7_NETC_TMR_1588_TRIG2 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO25 of instance: flexio2"]
    ALT8_FLEXIO2_FLEXIO25 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_LINK_ACT01 of instance: ecat"]
    ALT12_ECAT_LINK_ACT1 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAd25MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAd25MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAd25MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAd25MuxMode {
        SwMuxCtlPadGpioAd25MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAd25MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAd25MuxMode) -> u8 {
        SwMuxCtlPadGpioAd25MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAd26MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: LPUART6_CTS_B of instance: lpuart6"]
    ALT0_LPUART6_CTS_B = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPUART5_TX of instance: lpuart5"]
    ALT1_LPUART5_TX = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPSPI5_PCS2 of instance: lpspi5"]
    ALT2_LPSPI5_PCS2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SINC2_EMCLK00 of instance: sinc2"]
    ALT3_SINC2_EMCLK0 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM2_PWMA01 of instance: flexpwm2"]
    ALT4_FLEXPWM2_PWMA1 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO26 of instance: gpio4"]
    ALT5_GPIO4_IO26 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_ROW00 of instance: kpp"]
    ALT6_KPP_ROW0 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: NETC_TMR_PP1 of instance: netc"]
    ALT7_NETC_TMR_1588_PP1 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO26 of instance: flexio2"]
    ALT8_FLEXIO2_FLEXIO26 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: USDHC2_CD_B of instance: usdhc2"]
    ALT9_USDHC2_CD_B = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: MIC_BITSTREAM02 of instance: mic"]
    ALT12_MIC_BITSTREAM2 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAd26MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAd26MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAd26MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAd26MuxMode {
        SwMuxCtlPadGpioAd26MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAd26MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAd26MuxMode) -> u8 {
        SwMuxCtlPadGpioAd26MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAd27MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: LPUART6_RTS_B of instance: lpuart6"]
    ALT0_LPUART6_RTS_B = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPUART5_RX of instance: lpuart5"]
    ALT1_LPUART5_RX = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPSPI5_PCS1 of instance: lpspi5"]
    ALT2_LPSPI5_PCS1 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SINC2_EMBIT00 of instance: sinc2"]
    ALT3_SINC2_EMBIT0 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM2_PWMB01 of instance: flexpwm2"]
    ALT4_FLEXPWM2_PWMB1 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO27 of instance: gpio4"]
    ALT5_GPIO4_IO27 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_COL00 of instance: kpp"]
    ALT6_KPP_COL0 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: NETC_TMR_PP2 of instance: netc"]
    ALT7_NETC_TMR_1588_PP2 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO27 of instance: flexio2"]
    ALT8_FLEXIO2_FLEXIO27 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: USDHC2_WP of instance: usdhc2"]
    ALT9_USDHC2_WP = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: MIC_CLK of instance: mic"]
    ALT12_MIC_CLK = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAd27MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAd27MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAd27MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAd27MuxMode {
        SwMuxCtlPadGpioAd27MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAd27MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAd27MuxMode) -> u8 {
        SwMuxCtlPadGpioAd27MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAd28MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: LPSPI5_SCK of instance: lpspi5"]
    ALT0_LPSPI5_SCK = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: I3C1_PUR of instance: i3c1"]
    ALT2_I3C1_PUR = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SINC2_EMCLK01 of instance: sinc2"]
    ALT3_SINC2_EMCLK1 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM2_PWMB02 of instance: flexpwm2"]
    ALT4_FLEXPWM2_PWMB2 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO28 of instance: gpio4"]
    ALT5_GPIO4_IO28 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_ROW03 of instance: kpp"]
    ALT6_KPP_ROW3 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: NETC_TMR_PP3 of instance: netc"]
    ALT7_NETC_TMR_1588_PP3 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO28 of instance: flexio2"]
    ALT8_FLEXIO2_FLEXIO28 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: USDHC2_RESET_B of instance: usdhc2"]
    ALT9_USDHC2_RESET_B = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: MIC_BITSTREAM00 of instance: mic"]
    ALT12_MIC_BITSTREAM0 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAd28MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAd28MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAd28MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAd28MuxMode {
        SwMuxCtlPadGpioAd28MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAd28MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAd28MuxMode) -> u8 {
        SwMuxCtlPadGpioAd28MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAd29MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: LPSPI5_PCS0 of instance: lpspi5"]
    ALT0_LPSPI5_PCS0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: USDHC2_CD_B of instance: usdhc2"]
    ALT2_USDHC2_CD_B = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SINC2_EMBIT01 of instance: sinc2"]
    ALT3_SINC2_EMBIT1 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM2_PWMA02 of instance: flexpwm2"]
    ALT4_FLEXPWM2_PWMA2 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO29 of instance: gpio4"]
    ALT5_GPIO4_IO29 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_COL03 of instance: kpp"]
    ALT6_KPP_COL3 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: EWM_EWM_OUT_B of instance: ewm"]
    ALT7_EWM_EWM_OUT_B = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO29 of instance: flexio2"]
    ALT8_FLEXIO2_FLEXIO29 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: USDHC2_VSELECT of instance: usdhc2"]
    ALT9_USDHC2_VSELECT = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: MIC_BITSTREAM01 of instance: mic"]
    ALT12_MIC_BITSTREAM1 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAd29MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAd29MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAd29MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAd29MuxMode {
        SwMuxCtlPadGpioAd29MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAd29MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAd29MuxMode) -> u8 {
        SwMuxCtlPadGpioAd29MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAd30MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: LPSPI5_SDO of instance: lpspi5"]
    ALT0_LPSPI5_SDO = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: USB_OTG2_OC of instance: usb"]
    ALT1_USB_OTG2_OC = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: CAN2_TX of instance: can2"]
    ALT2_CAN2_TX = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SINC2_EMCLK02 of instance: sinc2"]
    ALT3_SINC2_EMCLK2 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPUART8_TX of instance: lpuart8"]
    ALT4_LPUART8_TX = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO30 of instance: gpio4"]
    ALT5_GPIO4_IO30 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_ROW02 of instance: kpp"]
    ALT6_KPP_ROW2 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: NETC_EMDC of instance: netc"]
    ALT7_NETC_EMDC = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO30 of instance: flexio2"]
    ALT8_FLEXIO2_FLEXIO30 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: XBAR1_XBAR_INOUT24 of instance: xbar1"]
    ALT9_XBAR1_XBAR_INOUT24 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_MCLK of instance: ecat"]
    ALT12_ECAT_MDC = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAd30MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAd30MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAd30MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAd30MuxMode {
        SwMuxCtlPadGpioAd30MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAd30MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAd30MuxMode) -> u8 {
        SwMuxCtlPadGpioAd30MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAd31MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: LPSPI5_SDI of instance: lpspi5"]
    ALT0_LPSPI5_SDI = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: USB_OTG2_PWR of instance: usb"]
    ALT1_USB_OTG2_PWR = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: CAN2_RX of instance: can2"]
    ALT2_CAN2_RX = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SINC2_EMBIT02 of instance: sinc2"]
    ALT3_SINC2_EMBIT2 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPUART8_RX of instance: lpuart8"]
    ALT4_LPUART8_RX = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO31 of instance: gpio4"]
    ALT5_GPIO4_IO31 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_COL02 of instance: kpp"]
    ALT6_KPP_COL2 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: NETC_EMDIO of instance: netc"]
    ALT7_NETC_EMDIO = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO31 of instance: flexio2"]
    ALT8_FLEXIO2_FLEXIO31 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: XBAR1_XBAR_INOUT25 of instance: xbar1"]
    ALT9_XBAR1_XBAR_INOUT25 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_MDIO of instance: ecat"]
    ALT12_ECAT_MDIO = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAd31MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAd31MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAd31MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAd31MuxMode {
        SwMuxCtlPadGpioAd31MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAd31MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAd31MuxMode) -> u8 {
        SwMuxCtlPadGpioAd31MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAd32MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: LPI2C5_SCL of instance: lpi2c5"]
    ALT0_LPI2C5_SCL = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: USBPHY2_OTG_ID of instance: usbphy2"]
    ALT1_USBPHY2_OTG_ID = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: GPC_PMIC_RDY of instance: gpc"]
    ALT2_GPC_PMIC_RDY = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SINC2_EMCLK03 of instance: sinc2"]
    ALT3_SINC2_EMCLK3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: USDHC1_CD_B of instance: usdhc1"]
    ALT4_USDHC1_CD_B = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO00 of instance: gpio5"]
    ALT5_GPIO5_IO0 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_ROW01 of instance: kpp"]
    ALT6_KPP_ROW1 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: NETC_TMR_TRIG1 of instance: netc"]
    ALT7_NETC_TMR_1588_TRIG1 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPUART10_TX of instance: lpuart10"]
    ALT8_LPUART10_TX = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: MIC_BITSTREAM03 of instance: mic"]
    ALT12_MIC_BITSTREAM3 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAd32MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAd32MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAd32MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAd32MuxMode {
        SwMuxCtlPadGpioAd32MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAd32MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAd32MuxMode) -> u8 {
        SwMuxCtlPadGpioAd32MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAd33MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: LPI2C5_SDA of instance: lpi2c5"]
    ALT0_LPI2C5_SDA = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: USBPHY1_OTG_ID of instance: usbphy1"]
    ALT1_USBPHY1_OTG_ID = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XBAR1_XBAR_INOUT17 of instance: xbar1"]
    ALT2_XBAR1_XBAR_INOUT17 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SINC2_EMBIT03 of instance: sinc2"]
    ALT3_SINC2_EMBIT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: USDHC1_WP of instance: usdhc1"]
    ALT4_USDHC1_WP = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO01 of instance: gpio5"]
    ALT5_GPIO5_IO1 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_COL01 of instance: kpp"]
    ALT6_KPP_COL1 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: NETC_TMR_TRIG2 of instance: netc"]
    ALT7_NETC_TMR_1588_TRIG2 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPUART10_RX of instance: lpuart10"]
    ALT8_LPUART10_RX = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAd33MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAd33MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAd33MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAd33MuxMode {
        SwMuxCtlPadGpioAd33MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAd33MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAd33MuxMode) -> u8 {
        SwMuxCtlPadGpioAd33MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAd34MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: I3C2_SCL of instance: i3c2"]
    ALT0_I3C2_SCL = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: USB_OTG1_PWR of instance: usb"]
    ALT1_USB_OTG1_PWR = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XBAR1_XBAR_INOUT18 of instance: xbar1"]
    ALT2_XBAR1_XBAR_INOUT18 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SINC_FILTER_GLUE2_BREAK of instance: sinc_filter_glue2"]
    ALT3_SINC_FILTER_GLUE2_BREAK = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: USDHC1_VSELECT of instance: usdhc1"]
    ALT4_USDHC1_VSELECT = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO02 of instance: gpio5"]
    ALT5_GPIO5_IO2 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: NETC_TMR_ALARM1 of instance: netc"]
    ALT7_NETC_TMR_1588_ALARM1 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPUART10_CTS_B of instance: lpuart10"]
    ALT8_LPUART10_CTS_B = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAd34MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAd34MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAd34MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAd34MuxMode {
        SwMuxCtlPadGpioAd34MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAd34MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAd34MuxMode) -> u8 {
        SwMuxCtlPadGpioAd34MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioAd35MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: I3C2_SDA of instance: i3c2"]
    ALT0_I3C2_SDA = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: USB_OTG1_OC of instance: usb"]
    ALT1_USB_OTG1_OC = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XBAR1_XBAR_INOUT19 of instance: xbar1"]
    ALT2_XBAR1_XBAR_INOUT19 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SINC2_MOD_CLK0 of instance: sinc2"]
    ALT3_SINC2_MOD_CLK0 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: USDHC1_RESET_B of instance: usdhc1"]
    ALT4_USDHC1_RESET_B = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO03 of instance: gpio5"]
    ALT5_GPIO5_IO3 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: NETC_TMR_ALARM2 of instance: netc"]
    ALT7_NETC_TMR_1588_ALARM2 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPUART10_RTS_B of instance: lpuart10"]
    ALT8_LPUART10_RTS_B = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioAd35MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioAd35MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioAd35MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioAd35MuxMode {
        SwMuxCtlPadGpioAd35MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioAd35MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioAd35MuxMode) -> u8 {
        SwMuxCtlPadGpioAd35MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioB100MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: NETC_PINMUX_ETH1_TXD00 of instance: netc_pinmux"]
    ALT0_NETC_PINMUX_ETH1_TXD0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: ADC2_CONV_D00 of instance: adc2"]
    ALT1_ADC2_CONV_D0 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SEMC_CSX01 of instance: semc"]
    ALT2_SEMC_CSX1 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER1_TIMER0 of instance: qtimer1"]
    ALT3_QTIMER1_TIMER0 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: XBAR1_XBAR_INOUT26 of instance: xbar1"]
    ALT4_XBAR1_XBAR_INOUT26 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO00 of instance: gpio6"]
    ALT5_GPIO6_IO0 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: TPM5_CH00 of instance: tpm5"]
    ALT6_TPM5_CH0 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_PINMUX_ETH4_TXD00 of instance: netc_pinmux"]
    ALT8_NETC_PINMUX_ETH4_TXD0 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioB100MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioB100MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioB100MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioB100MuxMode {
        SwMuxCtlPadGpioB100MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioB100MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioB100MuxMode) -> u8 {
        SwMuxCtlPadGpioB100MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioB101MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: NETC_PINMUX_ETH1_TXD01 of instance: netc_pinmux"]
    ALT0_NETC_PINMUX_ETH1_TXD1 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: ADC2_CONV_D01 of instance: adc2"]
    ALT1_ADC2_CONV_D1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SEMC_CSX02 of instance: semc"]
    ALT2_SEMC_CSX2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER1_TIMER1 of instance: qtimer1"]
    ALT3_QTIMER1_TIMER1 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: XBAR1_XBAR_INOUT27 of instance: xbar1"]
    ALT4_XBAR1_XBAR_INOUT27 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO01 of instance: gpio6"]
    ALT5_GPIO6_IO1 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: TPM5_CH01 of instance: tpm5"]
    ALT6_TPM5_CH1 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_PINMUX_ETH4_TXD01 of instance: netc_pinmux"]
    ALT8_NETC_PINMUX_ETH4_TXD1 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: SAI4_RX_DATA00 of instance: sai4"]
    ALT12_SAI4_RX_DATA0 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioB101MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioB101MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioB101MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioB101MuxMode {
        SwMuxCtlPadGpioB101MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioB101MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioB101MuxMode) -> u8 {
        SwMuxCtlPadGpioB101MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioB102MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: NETC_PINMUX_ETH1_TX_EN of instance: netc_pinmux"]
    ALT0_NETC_PINMUX_ETH1_TX_EN = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: ADC2_CONV_D02 of instance: adc2"]
    ALT1_ADC2_CONV_D2 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPI2C6_SCL of instance: lpi2c6"]
    ALT2_LPI2C6_SCL = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER1_TIMER2 of instance: qtimer1"]
    ALT3_QTIMER1_TIMER2 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: XBAR1_XBAR_INOUT28 of instance: xbar1"]
    ALT4_XBAR1_XBAR_INOUT28 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO02 of instance: gpio6"]
    ALT5_GPIO6_IO2 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: TPM5_CH02 of instance: tpm5"]
    ALT6_TPM5_CH2 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_B_SS1_B of instance: flexspi1_bus2bit"]
    ALT7_FLEXSPI1_BUS2BIT_B_SS1_B = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_PINMUX_ETH4_TX_EN of instance: netc_pinmux"]
    ALT8_NETC_PINMUX_ETH4_TX_EN = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPUART11_TX of instance: lpuart11"]
    ALT9_LPUART11_TX = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: SAI4_RX_DATA01 of instance: sai4"]
    ALT12_SAI4_RX_DATA1 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioB102MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioB102MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioB102MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioB102MuxMode {
        SwMuxCtlPadGpioB102MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioB102MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioB102MuxMode) -> u8 {
        SwMuxCtlPadGpioB102MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioB103MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: NETC_PINMUX_ETH1_TX_CLK of instance: netc_pinmux"]
    ALT0_NETC_PINMUX_ETH1_TX_CLK = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: ADC2_CONV_D03 of instance: adc2"]
    ALT1_ADC2_CONV_D3 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPI2C6_SDA of instance: lpi2c6"]
    ALT2_LPI2C6_SDA = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER2_TIMER0 of instance: qtimer2"]
    ALT3_QTIMER2_TIMER0 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: XBAR1_XBAR_INOUT29 of instance: xbar1"]
    ALT4_XBAR1_XBAR_INOUT29 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO03 of instance: gpio6"]
    ALT5_GPIO6_IO3 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: TPM5_CH03 of instance: tpm5"]
    ALT6_TPM5_CH3 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_B_DQS of instance: flexspi1_bus2bit"]
    ALT7_FLEXSPI1_BUS2BIT_B_DQS = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_PINMUX_ETH4_TX_CLK of instance: netc_pinmux"]
    ALT8_NETC_PINMUX_ETH4_TX_CLK = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPUART11_RX of instance: lpuart11"]
    ALT9_LPUART11_RX = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: SAI4_RX_DATA02 of instance: sai4"]
    ALT12_SAI4_RX_DATA2 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioB103MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioB103MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioB103MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioB103MuxMode {
        SwMuxCtlPadGpioB103MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioB103MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioB103MuxMode) -> u8 {
        SwMuxCtlPadGpioB103MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioB104MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: NETC_PINMUX_ETH1_RXD00 of instance: netc_pinmux"]
    ALT0_NETC_PINMUX_ETH1_RXD0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: ADC2_CONV_D04 of instance: adc2"]
    ALT1_ADC2_CONV_D4 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART9_RX of instance: lpuart9"]
    ALT2_LPUART9_RX = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER2_TIMER1 of instance: qtimer2"]
    ALT3_QTIMER2_TIMER1 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: XBAR1_XBAR_INOUT30 of instance: xbar1"]
    ALT4_XBAR1_XBAR_INOUT30 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO04 of instance: gpio6"]
    ALT5_GPIO6_IO4 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: TPM5_EXTCLK of instance: tpm5"]
    ALT6_TPM5_EXTCLK = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_B_SS0_B of instance: flexspi1_bus2bit"]
    ALT7_FLEXSPI1_BUS2BIT_B_SS0_B = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_PINMUX_ETH4_RXD00 of instance: netc_pinmux"]
    ALT8_NETC_PINMUX_ETH4_RXD0 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: SAI4_RX_DATA03 of instance: sai4"]
    ALT12_SAI4_RX_DATA3 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioB104MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioB104MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioB104MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioB104MuxMode {
        SwMuxCtlPadGpioB104MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioB104MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioB104MuxMode) -> u8 {
        SwMuxCtlPadGpioB104MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioB105MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: NETC_PINMUX_ETH1_RXD01 of instance: netc_pinmux"]
    ALT0_NETC_PINMUX_ETH1_RXD1 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: ADC2_CONV_D05 of instance: adc2"]
    ALT1_ADC2_CONV_D5 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART9_CTS_B of instance: lpuart9"]
    ALT2_LPUART9_CTS_B = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER2_TIMER2 of instance: qtimer2"]
    ALT3_QTIMER2_TIMER2 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: XBAR1_XBAR_INOUT31 of instance: xbar1"]
    ALT4_XBAR1_XBAR_INOUT31 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO05 of instance: gpio6"]
    ALT5_GPIO6_IO5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: TPM6_EXTCLK of instance: tpm6"]
    ALT6_TPM6_EXTCLK = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_B_SCLK of instance: flexspi1_bus2bit"]
    ALT7_FLEXSPI1_BUS2BIT_B_SCLK = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_PINMUX_ETH4_RXD01 of instance: netc_pinmux"]
    ALT8_NETC_PINMUX_ETH4_RXD1 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: SAI4_MCLK of instance: sai4"]
    ALT12_SAI4_MCLK = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioB105MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioB105MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioB105MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioB105MuxMode {
        SwMuxCtlPadGpioB105MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioB105MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioB105MuxMode) -> u8 {
        SwMuxCtlPadGpioB105MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioB106MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: NETC_PINMUX_ETH1_RX_DV of instance: netc_pinmux"]
    ALT0_NETC_PINMUX_ETH1_RX_DV = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: ADC2_CONV_D06 of instance: adc2"]
    ALT1_ADC2_CONV_D6 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART9_TX of instance: lpuart9"]
    ALT2_LPUART9_TX = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER3_TIMER0 of instance: qtimer3"]
    ALT3_QTIMER3_TIMER0 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: XBAR1_XBAR_INOUT32 of instance: xbar1"]
    ALT4_XBAR1_XBAR_INOUT32 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO06 of instance: gpio6"]
    ALT5_GPIO6_IO6 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: TPM6_CH00 of instance: tpm6"]
    ALT6_TPM6_CH0 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_B_DATA07 of instance: flexspi1_bus2bit"]
    ALT7_FLEXSPI1_BUS2BIT_B_DATA7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_PINMUX_ETH4_RX_DV of instance: netc_pinmux"]
    ALT8_NETC_PINMUX_ETH4_RX_DV = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: SAI4_RX_BCLK of instance: sai4"]
    ALT12_SAI4_RX_BCLK = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioB106MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioB106MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioB106MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioB106MuxMode {
        SwMuxCtlPadGpioB106MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioB106MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioB106MuxMode) -> u8 {
        SwMuxCtlPadGpioB106MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioB107MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: NETC_PINMUX_ETH1_TXD02 of instance: netc_pinmux"]
    ALT0_NETC_PINMUX_ETH1_TXD2 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: ADC2_CONV_D07 of instance: adc2"]
    ALT1_ADC2_CONV_D7 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART9_RTS_B of instance: lpuart9"]
    ALT2_LPUART9_RTS_B = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER3_TIMER1 of instance: qtimer3"]
    ALT3_QTIMER3_TIMER1 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: XBAR1_XBAR_INOUT33 of instance: xbar1"]
    ALT4_XBAR1_XBAR_INOUT33 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO07 of instance: gpio6"]
    ALT5_GPIO6_IO7 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: TPM6_CH01 of instance: tpm6"]
    ALT6_TPM6_CH1 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_B_DATA06 of instance: flexspi1_bus2bit"]
    ALT7_FLEXSPI1_BUS2BIT_B_DATA6 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_PINMUX_ETH4_TXD02 of instance: netc_pinmux"]
    ALT8_NETC_PINMUX_ETH4_TXD2 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPSPI6_SDI of instance: lpspi6"]
    ALT9_LPSPI6_SDI = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: SAI4_RX_SYNC of instance: sai4"]
    ALT12_SAI4_RX_SYNC = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioB107MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioB107MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioB107MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioB107MuxMode {
        SwMuxCtlPadGpioB107MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioB107MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioB107MuxMode) -> u8 {
        SwMuxCtlPadGpioB107MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioB108MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: NETC_PINMUX_ETH1_TXD03 of instance: netc_pinmux"]
    ALT0_NETC_PINMUX_ETH1_TXD3 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: ADC2_CONV_RDY_CLK of instance: adc2"]
    ALT1_ADC2_CONV_RDY_CLK = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: USDHC1_CD_B of instance: usdhc1"]
    ALT2_USDHC1_CD_B = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER3_TIMER2 of instance: qtimer3"]
    ALT3_QTIMER3_TIMER2 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: XBAR1_XBAR_INOUT36 of instance: xbar1"]
    ALT4_XBAR1_XBAR_INOUT36 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO08 of instance: gpio6"]
    ALT5_GPIO6_IO8 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: TPM6_CH02 of instance: tpm6"]
    ALT6_TPM6_CH2 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_B_DATA05 of instance: flexspi1_bus2bit"]
    ALT7_FLEXSPI1_BUS2BIT_B_DATA5 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_PINMUX_ETH4_TXD03 of instance: netc_pinmux"]
    ALT8_NETC_PINMUX_ETH4_TXD3 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPSPI6_SDO of instance: lpspi6"]
    ALT9_LPSPI6_SDO = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: SAI4_TX_BCLK of instance: sai4"]
    ALT12_SAI4_TX_BCLK = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioB108MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioB108MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioB108MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioB108MuxMode {
        SwMuxCtlPadGpioB108MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioB108MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioB108MuxMode) -> u8 {
        SwMuxCtlPadGpioB108MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioB109MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: NETC_PINMUX_ETH1_RXD02 of instance: netc_pinmux"]
    ALT0_NETC_PINMUX_ETH1_RXD2 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: USDHC1_WP of instance: usdhc1"]
    ALT2_USDHC1_WP = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER4_TIMER0 of instance: qtimer4"]
    ALT3_QTIMER4_TIMER0 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: XBAR1_XBAR_INOUT37 of instance: xbar1"]
    ALT4_XBAR1_XBAR_INOUT37 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO09 of instance: gpio6"]
    ALT5_GPIO6_IO9 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: TPM6_CH03 of instance: tpm6"]
    ALT6_TPM6_CH3 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_B_DATA04 of instance: flexspi1_bus2bit"]
    ALT7_FLEXSPI1_BUS2BIT_B_DATA4 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_PINMUX_ETH4_RXD02 of instance: netc_pinmux"]
    ALT8_NETC_PINMUX_ETH4_RXD2 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPSPI6_PCS1 of instance: lpspi6"]
    ALT9_LPSPI6_PCS1 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: SAI4_TX_SYNC of instance: sai4"]
    ALT12_SAI4_TX_SYNC = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioB109MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioB109MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioB109MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioB109MuxMode {
        SwMuxCtlPadGpioB109MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioB109MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioB109MuxMode) -> u8 {
        SwMuxCtlPadGpioB109MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioB110MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: NETC_PINMUX_ETH1_RXD03 of instance: netc_pinmux"]
    ALT0_NETC_PINMUX_ETH1_RXD3 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: USDHC1_RESET_B of instance: usdhc1"]
    ALT2_USDHC1_RESET_B = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER4_TIMER1 of instance: qtimer4"]
    ALT3_QTIMER4_TIMER1 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: XBAR1_XBAR_INOUT34 of instance: xbar1"]
    ALT4_XBAR1_XBAR_INOUT34 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO10 of instance: gpio6"]
    ALT5_GPIO6_IO10 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_B_DATA03 of instance: flexspi1_bus2bit"]
    ALT7_FLEXSPI1_BUS2BIT_B_DATA3 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_PINMUX_ETH4_RXD03 of instance: netc_pinmux"]
    ALT8_NETC_PINMUX_ETH4_RXD3 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPSPI6_PCS2 of instance: lpspi6"]
    ALT9_LPSPI6_PCS2 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: SAI4_TX_DATA00 of instance: sai4"]
    ALT12_SAI4_TX_DATA0 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioB110MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioB110MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioB110MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioB110MuxMode {
        SwMuxCtlPadGpioB110MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioB110MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioB110MuxMode) -> u8 {
        SwMuxCtlPadGpioB110MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioB111MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: NETC_PINMUX_ETH1_RX_CLK of instance: netc_pinmux"]
    ALT0_NETC_PINMUX_ETH1_RX_CLK = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER4_TIMER2 of instance: qtimer4"]
    ALT3_QTIMER4_TIMER2 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: XBAR1_XBAR_INOUT35 of instance: xbar1"]
    ALT4_XBAR1_XBAR_INOUT35 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO11 of instance: gpio6"]
    ALT5_GPIO6_IO11 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_B_DATA02 of instance: flexspi1_bus2bit"]
    ALT7_FLEXSPI1_BUS2BIT_B_DATA2 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_PINMUX_ETH4_RX_CLK of instance: netc_pinmux"]
    ALT8_NETC_PINMUX_ETH4_RX_CLK = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPSPI6_PCS3 of instance: lpspi6"]
    ALT9_LPSPI6_PCS3 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: SAI4_TX_DATA01 of instance: sai4"]
    ALT12_SAI4_TX_DATA1 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioB111MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioB111MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioB111MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioB111MuxMode {
        SwMuxCtlPadGpioB111MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioB111MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioB111MuxMode) -> u8 {
        SwMuxCtlPadGpioB111MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioB112MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: NETC_PINMUX_ETH1_RX_ER of instance: netc_pinmux"]
    ALT0_NETC_PINMUX_ETH1_RX_ER = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: NETC_EMDIO of instance: netc"]
    ALT1_NETC_EMDIO = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO12 of instance: gpio6"]
    ALT5_GPIO6_IO12 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_B_DATA01 of instance: flexspi1_bus2bit"]
    ALT7_FLEXSPI1_BUS2BIT_B_DATA1 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_PINMUX_ETH4_RX_ER of instance: netc_pinmux"]
    ALT8_NETC_PINMUX_ETH4_RX_ER = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPSPI6_PCS0 of instance: lpspi6"]
    ALT9_LPSPI6_PCS0 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: SAI4_TX_DATA02 of instance: sai4"]
    ALT12_SAI4_TX_DATA2 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioB112MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioB112MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioB112MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioB112MuxMode {
        SwMuxCtlPadGpioB112MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioB112MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioB112MuxMode) -> u8 {
        SwMuxCtlPadGpioB112MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioB113MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: NETC_PINMUX_ETH1_TX_ER of instance: netc_pinmux"]
    ALT0_NETC_PINMUX_ETH1_TX_ER = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: NETC_EMDC of instance: netc"]
    ALT1_NETC_EMDC = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: USDHC1_VSELECT of instance: usdhc1"]
    ALT2_USDHC1_VSELECT = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: CCM_ENET_REF_CLK_25M of instance: ccm"]
    ALT3_CCM_ENET_REF_CLK_25M = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO13 of instance: gpio6"]
    ALT5_GPIO6_IO13 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_B_DATA00 of instance: flexspi1_bus2bit"]
    ALT7_FLEXSPI1_BUS2BIT_B_DATA0 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_PINMUX_ETH4_TX_ER of instance: netc_pinmux"]
    ALT8_NETC_PINMUX_ETH4_TX_ER = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPSPI6_SCK of instance: lpspi6"]
    ALT9_LPSPI6_SCK = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: SAI4_TX_DATA03 of instance: sai4"]
    ALT12_SAI4_TX_DATA3 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioB113MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioB113MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioB113MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioB113MuxMode {
        SwMuxCtlPadGpioB113MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioB113MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioB113MuxMode) -> u8 {
        SwMuxCtlPadGpioB113MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioB200MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: NETC_ETH1_CRS of instance: netc"]
    ALT0_NETC_ETH1_CRS = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: SEMC_CSX03 of instance: semc"]
    ALT1_SEMC_CSX3 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPIT3_TRIGGER00 of instance: lpit3"]
    ALT2_LPIT3_TRIGGER0 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: SAI4_MCLK of instance: sai4"]
    ALT4_SAI4_MCLK = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO14 of instance: gpio6"]
    ALT5_GPIO6_IO14 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_ETH4_CRS of instance: netc"]
    ALT8_NETC_ETH4_CRS = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPSPI6_SDI of instance: lpspi6"]
    ALT9_LPSPI6_SDI = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: NETC_ETH2_SLV_MDIO of instance: netc"]
    ALT10_NETC_ETH2_SLV_MDIO = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_CLK_ECAT_CLK25 of instance: ecat"]
    ALT12_ECAT_CLK_ECAT_CLK25 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioB200MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioB200MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioB200MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioB200MuxMode {
        SwMuxCtlPadGpioB200MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioB200MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioB200MuxMode) -> u8 {
        SwMuxCtlPadGpioB200MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioB201MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: NETC_ETH1_COL of instance: netc"]
    ALT0_NETC_ETH1_COL = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPIT3_TRIGGER01 of instance: lpit3"]
    ALT2_LPIT3_TRIGGER1 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: SAI4_TX_BCLK of instance: sai4"]
    ALT4_SAI4_TX_BCLK = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO15 of instance: gpio6"]
    ALT5_GPIO6_IO15 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: FLEXSPI1_BUS2BIT_A_SS1_B of instance: flexspi1_bus2bit"]
    ALT6_FLEXSPI1_BUS2BIT_A_SS1_B = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_ETH4_COL of instance: netc"]
    ALT8_NETC_ETH4_COL = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPSPI6_SDO of instance: lpspi6"]
    ALT9_LPSPI6_SDO = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: NETC_ETH2_SLV_MDC of instance: netc"]
    ALT10_NETC_ETH2_SLV_MDC = 0x0a,
    #[doc = "Select mux mode: ALT11 mux port: NETC_PINMUX_ETH2_RX_ER of instance: netc_pinmux"]
    ALT11_NETC_PINMUX_ETH2_RX_ER = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_RX_ER_1 of instance: ecat"]
    ALT12_ECAT_PT1_RX_ER = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioB201MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioB201MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioB201MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioB201MuxMode {
        SwMuxCtlPadGpioB201MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioB201MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioB201MuxMode) -> u8 {
        SwMuxCtlPadGpioB201MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioB202MuxMode {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPIT3_TRIGGER02 of instance: lpit3"]
    ALT2_LPIT3_TRIGGER2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_EMDIO of instance: netc"]
    ALT3_NETC_EMDIO = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: SAI4_TX_SYNC of instance: sai4"]
    ALT4_SAI4_TX_SYNC = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO16 of instance: gpio6"]
    ALT5_GPIO6_IO16 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: FLEXSPI1_BUS2BIT_B_SCLK of instance: flexspi1_bus2bit"]
    ALT6_FLEXSPI1_BUS2BIT_B_SCLK = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: CCM_ENET_REF_CLK_25M of instance: ccm"]
    ALT9_CCM_ENET_REF_CLK_25M = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: EWM_EWM_OUT_B of instance: ewm"]
    ALT10_EWM_EWM_OUT_B = 0x0a,
    #[doc = "Select mux mode: ALT11 mux port: NETC_PINMUX_ETH2_RXD02 of instance: netc_pinmux"]
    ALT11_NETC_PINMUX_ETH2_RXD2 = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_RX_DATA2_1 of instance: ecat"]
    ALT12_ECAT_PT1_RXD2 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioB202MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioB202MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioB202MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioB202MuxMode {
        SwMuxCtlPadGpioB202MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioB202MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioB202MuxMode) -> u8 {
        SwMuxCtlPadGpioB202MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioB203MuxMode {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPIT3_TRIGGER03 of instance: lpit3"]
    ALT2_LPIT3_TRIGGER3 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_EMDC of instance: netc"]
    ALT3_NETC_EMDC = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: SAI4_TX_DATA00 of instance: sai4"]
    ALT4_SAI4_TX_DATA0 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO17 of instance: gpio6"]
    ALT5_GPIO6_IO17 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_A_DATA04 of instance: flexspi1_bus2bit"]
    ALT7_FLEXSPI1_BUS2BIT_A_DATA4 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: XSPI_SLV_DATA04 of instance: xspi_slv"]
    ALT10_XSPI_SLV_DATA4 = 0x0a,
    #[doc = "Select mux mode: ALT11 mux port: NETC_PINMUX_ETH2_RXD03 of instance: netc_pinmux"]
    ALT11_NETC_PINMUX_ETH2_RXD3 = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_RX_DATA3_1 of instance: ecat"]
    ALT12_ECAT_PT1_RXD3 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioB203MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioB203MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioB203MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioB203MuxMode {
        SwMuxCtlPadGpioB203MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioB203MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioB203MuxMode) -> u8 {
        SwMuxCtlPadGpioB203MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioB204MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SINC1_MOD_CLK0 of instance: sinc1"]
    ALT0_SINC1_MOD_CLK0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: SINC2_MOD_CLK0 of instance: sinc2"]
    ALT1_SINC2_MOD_CLK0 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SINC3_MOD_CLK0 of instance: sinc3"]
    ALT2_SINC3_MOD_CLK0 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: SAI4_RX_SYNC of instance: sai4"]
    ALT4_SAI4_RX_SYNC = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO18 of instance: gpio6"]
    ALT5_GPIO6_IO18 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_A_DATA05 of instance: flexspi1_bus2bit"]
    ALT7_FLEXSPI1_BUS2BIT_A_DATA5 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: TPM3_EXTCLK of instance: tpm3"]
    ALT8_TPM3_EXTCLK = 0x08,
    _RESERVED_9 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: XSPI_SLV_DATA05 of instance: xspi_slv"]
    ALT10_XSPI_SLV_DATA5 = 0x0a,
    #[doc = "Select mux mode: ALT11 mux port: NETC_PINMUX_ETH2_TXD02 of instance: netc_pinmux"]
    ALT11_NETC_PINMUX_ETH2_TXD2 = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_TX_DATA2_1 of instance: ecat"]
    ALT12_ECAT_PT1_TXD2 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioB204MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioB204MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioB204MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioB204MuxMode {
        SwMuxCtlPadGpioB204MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioB204MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioB204MuxMode) -> u8 {
        SwMuxCtlPadGpioB204MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioB205MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SINC1_MOD_CLK1 of instance: sinc1"]
    ALT0_SINC1_MOD_CLK1 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: SINC2_MOD_CLK1 of instance: sinc2"]
    ALT1_SINC2_MOD_CLK1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SINC3_MOD_CLK1 of instance: sinc3"]
    ALT2_SINC3_MOD_CLK1 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: SAI4_RX_BCLK of instance: sai4"]
    ALT4_SAI4_RX_BCLK = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO19 of instance: gpio6"]
    ALT5_GPIO6_IO19 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: MIC_CLK of instance: mic"]
    ALT6_MIC_CLK = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_A_DATA06 of instance: flexspi1_bus2bit"]
    ALT7_FLEXSPI1_BUS2BIT_A_DATA6 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: TPM3_CH00 of instance: tpm3"]
    ALT8_TPM3_CH0 = 0x08,
    _RESERVED_9 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: XSPI_SLV_DATA06 of instance: xspi_slv"]
    ALT10_XSPI_SLV_DATA6 = 0x0a,
    #[doc = "Select mux mode: ALT11 mux port: NETC_PINMUX_ETH2_TXD03 of instance: netc_pinmux"]
    ALT11_NETC_PINMUX_ETH2_TXD3 = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_TX_DATA3_1 of instance: ecat"]
    ALT12_ECAT_PT1_TXD3 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioB205MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioB205MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioB205MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioB205MuxMode {
        SwMuxCtlPadGpioB205MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioB205MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioB205MuxMode) -> u8 {
        SwMuxCtlPadGpioB205MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioB206MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SINC1_MOD_CLK2 of instance: sinc1"]
    ALT0_SINC1_MOD_CLK2 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: SINC2_MOD_CLK2 of instance: sinc2"]
    ALT1_SINC2_MOD_CLK2 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SINC3_MOD_CLK2 of instance: sinc3"]
    ALT2_SINC3_MOD_CLK2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART6_DSR_B of instance: lpuart6"]
    ALT3_LPUART6_DSR_B = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: SAI4_RX_DATA00 of instance: sai4"]
    ALT4_SAI4_RX_DATA0 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO20 of instance: gpio6"]
    ALT5_GPIO6_IO20 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_A_DATA07 of instance: flexspi1_bus2bit"]
    ALT7_FLEXSPI1_BUS2BIT_A_DATA7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: TPM3_CH01 of instance: tpm3"]
    ALT8_TPM3_CH1 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPUART11_TX of instance: lpuart11"]
    ALT9_LPUART11_TX = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: XSPI_SLV_DATA07 of instance: xspi_slv"]
    ALT10_XSPI_SLV_DATA7 = 0x0a,
    #[doc = "Select mux mode: ALT11 mux port: NETC_PINMUX_ETH2_TXD00 of instance: netc_pinmux"]
    ALT11_NETC_PINMUX_ETH2_TXD0 = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_TX_DATA0_1 of instance: ecat"]
    ALT12_ECAT_PT1_TXD0 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioB206MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioB206MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioB206MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioB206MuxMode {
        SwMuxCtlPadGpioB206MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioB206MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioB206MuxMode) -> u8 {
        SwMuxCtlPadGpioB206MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioB207MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: QTIMER5_TIMER0 of instance: qtimer5"]
    ALT0_QTIMER5_TIMER0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART6_DCD_B of instance: lpuart6"]
    ALT3_LPUART6_DCD_B = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: SAI4_TX_DATA01 of instance: sai4"]
    ALT4_SAI4_TX_DATA1 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO21 of instance: gpio6"]
    ALT5_GPIO6_IO21 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SAI4_RX_DATA01 of instance: sai4"]
    ALT6_SAI4_RX_DATA1 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_A_DQS of instance: flexspi1_bus2bit"]
    ALT7_FLEXSPI1_BUS2BIT_A_DQS = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: TPM3_CH02 of instance: tpm3"]
    ALT8_TPM3_CH2 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPUART11_RX of instance: lpuart11"]
    ALT9_LPUART11_RX = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: XSPI_SLV_DQS of instance: xspi_slv"]
    ALT10_XSPI_SLV_DQS = 0x0a,
    #[doc = "Select mux mode: ALT11 mux port: NETC_PINMUX_ETH2_TXD01 of instance: netc_pinmux"]
    ALT11_NETC_PINMUX_ETH2_TXD1 = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_TX_DATA1_1 of instance: ecat"]
    ALT12_ECAT_PT1_TXD1 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioB207MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioB207MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioB207MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioB207MuxMode {
        SwMuxCtlPadGpioB207MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioB207MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioB207MuxMode) -> u8 {
        SwMuxCtlPadGpioB207MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioB208MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: QTIMER5_TIMER1 of instance: qtimer5"]
    ALT0_QTIMER5_TIMER1 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: SINC2_EMCLK02 of instance: sinc2"]
    ALT1_SINC2_EMCLK2 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPUART6_RI_B of instance: lpuart6"]
    ALT4_LPUART6_RI_B = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO22 of instance: gpio6"]
    ALT5_GPIO6_IO22 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPI2C6_SCL of instance: lpi2c6"]
    ALT6_LPI2C6_SCL = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_A_SCLK of instance: flexspi1_bus2bit"]
    ALT7_FLEXSPI1_BUS2BIT_A_SCLK = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: TPM3_CH03 of instance: tpm3"]
    ALT8_TPM3_CH3 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: SPDIF_IN of instance: spdif"]
    ALT9_SPDIF_IN = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: XSPI_SLV_CLK of instance: xspi_slv"]
    ALT10_XSPI_SLV_CLK = 0x0a,
    #[doc = "Select mux mode: ALT11 mux port: NETC_PINMUX_ETH2_TX_EN of instance: netc_pinmux"]
    ALT11_NETC_PINMUX_ETH2_TX_EN = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_TX_EN_1 of instance: ecat"]
    ALT12_ECAT_PT1_TX_EN = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioB208MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioB208MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioB208MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioB208MuxMode {
        SwMuxCtlPadGpioB208MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioB208MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioB208MuxMode) -> u8 {
        SwMuxCtlPadGpioB208MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioB209MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: QTIMER5_TIMER2 of instance: qtimer5"]
    ALT0_QTIMER5_TIMER2 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: SINC2_EMBIT02 of instance: sinc2"]
    ALT1_SINC2_EMBIT2 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPUART6_DTR_B of instance: lpuart6"]
    ALT4_LPUART6_DTR_B = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO23 of instance: gpio6"]
    ALT5_GPIO6_IO23 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPI2C6_SDA of instance: lpi2c6"]
    ALT6_LPI2C6_SDA = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_A_SS0_B of instance: flexspi1_bus2bit"]
    ALT7_FLEXSPI1_BUS2BIT_A_SS0_B = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: TPM4_EXTCLK of instance: tpm4"]
    ALT8_TPM4_EXTCLK = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: SPDIF_OUT of instance: spdif"]
    ALT9_SPDIF_OUT = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: XSPI_SLV_CS of instance: xspi_slv"]
    ALT10_XSPI_SLV_CS = 0x0a,
    #[doc = "Select mux mode: ALT11 mux port: NETC_PINMUX_ETH2_TX_CLK of instance: netc_pinmux"]
    ALT11_NETC_PINMUX_ETH2_TX_CLK = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_TX_CLK_1 of instance: ecat"]
    ALT12_ECAT_PT1_TX_CLK = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioB209MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioB209MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioB209MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioB209MuxMode {
        SwMuxCtlPadGpioB209MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioB209MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioB209MuxMode) -> u8 {
        SwMuxCtlPadGpioB209MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioB210MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: MIC_BITSTREAM00 of instance: mic"]
    ALT0_MIC_BITSTREAM0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: SINC2_EMCLK03 of instance: sinc2"]
    ALT1_SINC2_EMCLK3 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: CAN3_TX of instance: can3"]
    ALT2_CAN3_TX = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART8_CTS_B of instance: lpuart8"]
    ALT3_LPUART8_CTS_B = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPUART6_TX of instance: lpuart6"]
    ALT4_LPUART6_TX = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO24 of instance: gpio6"]
    ALT5_GPIO6_IO24 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPI2C4_SCL of instance: lpi2c4"]
    ALT6_LPI2C4_SCL = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_A_DATA00 of instance: flexspi1_bus2bit"]
    ALT7_FLEXSPI1_BUS2BIT_A_DATA0 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: TPM4_CH00 of instance: tpm4"]
    ALT8_TPM4_CH0 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPSPI4_SCK of instance: lpspi4"]
    ALT9_LPSPI4_SCK = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: XSPI_SLV_DATA00 of instance: xspi_slv"]
    ALT10_XSPI_SLV_DATA0 = 0x0a,
    #[doc = "Select mux mode: ALT11 mux port: NETC_PINMUX_ETH2_RXD00 of instance: netc_pinmux"]
    ALT11_NETC_PINMUX_ETH2_RXD0 = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_RX_DATA0_1 of instance: ecat"]
    ALT12_ECAT_PT1_RXD0 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioB210MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioB210MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioB210MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioB210MuxMode {
        SwMuxCtlPadGpioB210MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioB210MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioB210MuxMode) -> u8 {
        SwMuxCtlPadGpioB210MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioB211MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: MIC_BITSTREAM01 of instance: mic"]
    ALT0_MIC_BITSTREAM1 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: SINC2_EMBIT03 of instance: sinc2"]
    ALT1_SINC2_EMBIT3 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: CAN3_RX of instance: can3"]
    ALT2_CAN3_RX = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART8_RTS_B of instance: lpuart8"]
    ALT3_LPUART8_RTS_B = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPUART6_RX of instance: lpuart6"]
    ALT4_LPUART6_RX = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO25 of instance: gpio6"]
    ALT5_GPIO6_IO25 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPI2C4_SDA of instance: lpi2c4"]
    ALT6_LPI2C4_SDA = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_A_DATA01 of instance: flexspi1_bus2bit"]
    ALT7_FLEXSPI1_BUS2BIT_A_DATA1 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: TPM4_CH01 of instance: tpm4"]
    ALT8_TPM4_CH1 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPSPI4_SDI of instance: lpspi4"]
    ALT9_LPSPI4_SDI = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: XSPI_SLV_DATA01 of instance: xspi_slv"]
    ALT10_XSPI_SLV_DATA1 = 0x0a,
    #[doc = "Select mux mode: ALT11 mux port: NETC_PINMUX_ETH2_RXD01 of instance: netc_pinmux"]
    ALT11_NETC_PINMUX_ETH2_RXD1 = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_RX_DATA1_1 of instance: ecat"]
    ALT12_ECAT_PT1_RXD1 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioB211MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioB211MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioB211MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioB211MuxMode {
        SwMuxCtlPadGpioB211MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioB211MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioB211MuxMode) -> u8 {
        SwMuxCtlPadGpioB211MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioB212MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: MIC_BITSTREAM02 of instance: mic"]
    ALT0_MIC_BITSTREAM2 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: SINC_FILTER_GLUE2_BREAK of instance: sinc_filter_glue2"]
    ALT1_SINC_FILTER_GLUE2_BREAK = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART8_TX of instance: lpuart8"]
    ALT2_LPUART8_TX = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPUART6_CTS_B of instance: lpuart6"]
    ALT4_LPUART6_CTS_B = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO26 of instance: gpio6"]
    ALT5_GPIO6_IO26 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: CAN3_TX of instance: can3"]
    ALT6_CAN3_TX = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_A_DATA02 of instance: flexspi1_bus2bit"]
    ALT7_FLEXSPI1_BUS2BIT_A_DATA2 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: TPM4_CH02 of instance: tpm4"]
    ALT8_TPM4_CH2 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPSPI4_SDO of instance: lpspi4"]
    ALT9_LPSPI4_SDO = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: XSPI_SLV_DATA02 of instance: xspi_slv"]
    ALT10_XSPI_SLV_DATA2 = 0x0a,
    #[doc = "Select mux mode: ALT11 mux port: NETC_PINMUX_ETH2_RX_DV of instance: netc_pinmux"]
    ALT11_NETC_PINMUX_ETH2_RX_DV = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_RX_DV_1 of instance: ecat"]
    ALT12_ECAT_PT1_RX_DV = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioB212MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioB212MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioB212MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioB212MuxMode {
        SwMuxCtlPadGpioB212MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioB212MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioB212MuxMode) -> u8 {
        SwMuxCtlPadGpioB212MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioB213MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: MIC_BITSTREAM03 of instance: mic"]
    ALT0_MIC_BITSTREAM3 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: SINC2_EMCLK00 of instance: sinc2"]
    ALT1_SINC2_EMCLK0 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART8_RX of instance: lpuart8"]
    ALT2_LPUART8_RX = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPUART6_RTS_B of instance: lpuart6"]
    ALT4_LPUART6_RTS_B = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO27 of instance: gpio6"]
    ALT5_GPIO6_IO27 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: CAN3_RX of instance: can3"]
    ALT6_CAN3_RX = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_A_DATA03 of instance: flexspi1_bus2bit"]
    ALT7_FLEXSPI1_BUS2BIT_A_DATA3 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: TPM4_CH03 of instance: tpm4"]
    ALT8_TPM4_CH3 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPSPI4_PCS0 of instance: lpspi4"]
    ALT9_LPSPI4_PCS0 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: XSPI_SLV_DATA03 of instance: xspi_slv"]
    ALT10_XSPI_SLV_DATA3 = 0x0a,
    #[doc = "Select mux mode: ALT11 mux port: NETC_PINMUX_ETH2_RX_CLK of instance: netc_pinmux"]
    ALT11_NETC_PINMUX_ETH2_RX_CLK = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_RX_CLK_1 of instance: ecat"]
    ALT12_ECAT_PT1_RX_CLK = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioB213MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioB213MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioB213MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioB213MuxMode {
        SwMuxCtlPadGpioB213MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioB213MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioB213MuxMode) -> u8 {
        SwMuxCtlPadGpioB213MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB100MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA00 of instance: semc"]
    ALT0_SEMC_DATA0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: XBAR1_XBAR_INOUT04 of instance: xbar1"]
    ALT1_XBAR1_XBAR_INOUT4 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SINC3_MOD_CLK0 of instance: sinc3"]
    ALT2_SINC3_MOD_CLK0 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART3_CTS_B of instance: lpuart3"]
    ALT3_LPUART3_CTS_B = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH3_TXD03 of instance: netc_pinmux"]
    ALT4_NETC_PINMUX_ETH3_TXD3 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO00 of instance: gpio2"]
    ALT5_GPIO2_IO0 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_ROW03 of instance: kpp"]
    ALT6_KPP_ROW3 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO00 of instance: flexio1"]
    ALT8_FLEXIO1_FLEXIO0 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH4_TXD03 of instance: netc_pinmux"]
    ALT9_NETC_PINMUX_ETH4_TXD3 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: ECAT_TX_DATA3_0 of instance: ecat"]
    ALT10_ECAT_PT0_TXD3 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_DATA00 of instance: ahb_sramc"]
    ALT12_AHB_SRAMC_DATA0 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB100MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB100MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB100MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB100MuxMode {
        SwMuxCtlPadGpioEmcB100MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB100MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB100MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB100MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB101MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA01 of instance: semc"]
    ALT0_SEMC_DATA1 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: XBAR1_XBAR_INOUT05 of instance: xbar1"]
    ALT1_XBAR1_XBAR_INOUT5 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SINC3_MOD_CLK1 of instance: sinc3"]
    ALT2_SINC3_MOD_CLK1 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART3_RTS_B of instance: lpuart3"]
    ALT3_LPUART3_RTS_B = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH3_TXD02 of instance: netc_pinmux"]
    ALT4_NETC_PINMUX_ETH3_TXD2 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO01 of instance: gpio2"]
    ALT5_GPIO2_IO1 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_COL03 of instance: kpp"]
    ALT6_KPP_COL3 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO01 of instance: flexio1"]
    ALT8_FLEXIO1_FLEXIO1 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH4_TXD02 of instance: netc_pinmux"]
    ALT9_NETC_PINMUX_ETH4_TXD2 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: ECAT_TX_DATA2_0 of instance: ecat"]
    ALT10_ECAT_PT0_TXD2 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_DATA01 of instance: ahb_sramc"]
    ALT12_AHB_SRAMC_DATA1 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB101MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB101MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB101MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB101MuxMode {
        SwMuxCtlPadGpioEmcB101MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB101MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB101MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB101MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB102MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA02 of instance: semc"]
    ALT0_SEMC_DATA2 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: XBAR1_XBAR_INOUT06 of instance: xbar1"]
    ALT1_XBAR1_XBAR_INOUT6 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SINC3_MOD_CLK2 of instance: sinc3"]
    ALT2_SINC3_MOD_CLK2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART3_RX of instance: lpuart3"]
    ALT3_LPUART3_RX = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH3_RX_CLK of instance: netc_pinmux"]
    ALT4_NETC_PINMUX_ETH3_RX_CLK = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO02 of instance: gpio2"]
    ALT5_GPIO2_IO2 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_ROW02 of instance: kpp"]
    ALT6_KPP_ROW2 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO02 of instance: flexio1"]
    ALT8_FLEXIO1_FLEXIO2 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH4_RX_CLK of instance: netc_pinmux"]
    ALT9_NETC_PINMUX_ETH4_RX_CLK = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: ECAT_RX_CLK_0 of instance: ecat"]
    ALT10_ECAT_PT0_RX_CLK = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_DATA02 of instance: ahb_sramc"]
    ALT12_AHB_SRAMC_DATA2 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB102MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB102MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB102MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB102MuxMode {
        SwMuxCtlPadGpioEmcB102MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB102MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB102MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB102MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB103MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA03 of instance: semc"]
    ALT0_SEMC_DATA3 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: XBAR1_XBAR_INOUT07 of instance: xbar1"]
    ALT1_XBAR1_XBAR_INOUT7 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SINC3_EMCLK00 of instance: sinc3"]
    ALT2_SINC3_EMCLK0 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART3_TX of instance: lpuart3"]
    ALT3_LPUART3_TX = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH3_RXD03 of instance: netc_pinmux"]
    ALT4_NETC_PINMUX_ETH3_RXD3 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO03 of instance: gpio2"]
    ALT5_GPIO2_IO3 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_COL02 of instance: kpp"]
    ALT6_KPP_COL2 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO03 of instance: flexio1"]
    ALT8_FLEXIO1_FLEXIO3 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH4_RXD03 of instance: netc_pinmux"]
    ALT9_NETC_PINMUX_ETH4_RXD3 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: ECAT_RX_DATA3_0 of instance: ecat"]
    ALT10_ECAT_PT0_RXD3 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_DATA03 of instance: ahb_sramc"]
    ALT12_AHB_SRAMC_DATA3 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB103MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB103MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB103MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB103MuxMode {
        SwMuxCtlPadGpioEmcB103MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB103MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB103MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB103MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB104MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA04 of instance: semc"]
    ALT0_SEMC_DATA4 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: XBAR1_XBAR_INOUT08 of instance: xbar1"]
    ALT1_XBAR1_XBAR_INOUT8 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SINC3_EMBIT00 of instance: sinc3"]
    ALT2_SINC3_EMBIT0 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART3_DSR_B of instance: lpuart3"]
    ALT3_LPUART3_DSR_B = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH3_RXD02 of instance: netc_pinmux"]
    ALT4_NETC_PINMUX_ETH3_RXD2 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO04 of instance: gpio2"]
    ALT5_GPIO2_IO4 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_ROW01 of instance: kpp"]
    ALT6_KPP_ROW1 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO04 of instance: flexio1"]
    ALT8_FLEXIO1_FLEXIO4 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH4_RXD02 of instance: netc_pinmux"]
    ALT9_NETC_PINMUX_ETH4_RXD2 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: ECAT_RX_DATA2_0 of instance: ecat"]
    ALT10_ECAT_PT0_RXD2 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_DATA04 of instance: ahb_sramc"]
    ALT12_AHB_SRAMC_DATA4 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB104MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB104MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB104MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB104MuxMode {
        SwMuxCtlPadGpioEmcB104MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB104MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB104MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB104MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB105MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA05 of instance: semc"]
    ALT0_SEMC_DATA5 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: XBAR1_XBAR_INOUT09 of instance: xbar1"]
    ALT1_XBAR1_XBAR_INOUT9 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SINC3_EMCLK01 of instance: sinc3"]
    ALT2_SINC3_EMCLK1 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART3_DCD_B of instance: lpuart3"]
    ALT3_LPUART3_DCD_B = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH3_TXD00 of instance: netc_pinmux"]
    ALT4_NETC_PINMUX_ETH3_TXD0 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO05 of instance: gpio2"]
    ALT5_GPIO2_IO5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_ROW07 of instance: kpp"]
    ALT6_KPP_ROW7 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO05 of instance: flexio1"]
    ALT8_FLEXIO1_FLEXIO5 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH4_TXD00 of instance: netc_pinmux"]
    ALT9_NETC_PINMUX_ETH4_TXD0 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: ECAT_TX_DATA0_0 of instance: ecat"]
    ALT10_ECAT_PT0_TXD0 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_DATA05 of instance: ahb_sramc"]
    ALT12_AHB_SRAMC_DATA5 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB105MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB105MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB105MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB105MuxMode {
        SwMuxCtlPadGpioEmcB105MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB105MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB105MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB105MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB106MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA06 of instance: semc"]
    ALT0_SEMC_DATA6 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM4_PWMB03 of instance: flexpwm4"]
    ALT1_FLEXPWM4_PWMB3 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SINC3_EMBIT01 of instance: sinc3"]
    ALT2_SINC3_EMBIT1 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART3_RI_B of instance: lpuart3"]
    ALT3_LPUART3_RI_B = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH3_TXD01 of instance: netc_pinmux"]
    ALT4_NETC_PINMUX_ETH3_TXD1 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO06 of instance: gpio2"]
    ALT5_GPIO2_IO6 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_COL07 of instance: kpp"]
    ALT6_KPP_COL7 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO06 of instance: flexio1"]
    ALT8_FLEXIO1_FLEXIO6 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH4_TXD01 of instance: netc_pinmux"]
    ALT9_NETC_PINMUX_ETH4_TXD1 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: ECAT_TX_DATA1_0 of instance: ecat"]
    ALT10_ECAT_PT0_TXD1 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_DATA06 of instance: ahb_sramc"]
    ALT12_AHB_SRAMC_DATA6 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB106MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB106MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB106MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB106MuxMode {
        SwMuxCtlPadGpioEmcB106MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB106MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB106MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB106MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB107MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA07 of instance: semc"]
    ALT0_SEMC_DATA7 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM4_PWMA03 of instance: flexpwm4"]
    ALT1_FLEXPWM4_PWMA3 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SINC3_EMCLK02 of instance: sinc3"]
    ALT2_SINC3_EMCLK2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART3_DTR_B of instance: lpuart3"]
    ALT3_LPUART3_DTR_B = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH3_TX_EN of instance: netc_pinmux"]
    ALT4_NETC_PINMUX_ETH3_TX_EN = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO07 of instance: gpio2"]
    ALT5_GPIO2_IO7 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_ROW06 of instance: kpp"]
    ALT6_KPP_ROW6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO07 of instance: flexio1"]
    ALT8_FLEXIO1_FLEXIO7 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH4_TX_EN of instance: netc_pinmux"]
    ALT9_NETC_PINMUX_ETH4_TX_EN = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: ECAT_TX_EN_0 of instance: ecat"]
    ALT10_ECAT_PT0_TX_EN = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_DATA07 of instance: ahb_sramc"]
    ALT12_AHB_SRAMC_DATA7 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB107MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB107MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB107MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB107MuxMode {
        SwMuxCtlPadGpioEmcB107MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB107MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB107MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB107MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB108MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DM00 of instance: semc"]
    ALT0_SEMC_DM0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM2_PWMB03 of instance: flexpwm2"]
    ALT1_FLEXPWM2_PWMB3 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SINC3_EMBIT02 of instance: sinc3"]
    ALT2_SINC3_EMBIT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART4_DSR_B of instance: lpuart4"]
    ALT3_LPUART4_DSR_B = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH3_TX_CLK of instance: netc_pinmux"]
    ALT4_NETC_PINMUX_ETH3_TX_CLK = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO08 of instance: gpio2"]
    ALT5_GPIO2_IO8 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_COL06 of instance: kpp"]
    ALT6_KPP_COL6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO08 of instance: flexio1"]
    ALT8_FLEXIO1_FLEXIO8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH4_TX_CLK of instance: netc_pinmux"]
    ALT9_NETC_PINMUX_ETH4_TX_CLK = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: ECAT_TX_CLK_0 of instance: ecat"]
    ALT10_ECAT_PT0_TX_CLK = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_LBB of instance: ahb_sramc"]
    ALT12_AHB_SRAMC_LBB = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB108MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB108MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB108MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB108MuxMode {
        SwMuxCtlPadGpioEmcB108MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB108MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB108MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB108MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB109MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_ADDR00 of instance: semc"]
    ALT0_SEMC_ADDR0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM2_PWMA03 of instance: flexpwm2"]
    ALT1_FLEXPWM2_PWMA3 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SINC3_EMCLK03 of instance: sinc3"]
    ALT2_SINC3_EMCLK3 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART4_DCD_B of instance: lpuart4"]
    ALT3_LPUART4_DCD_B = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH3_RXD00 of instance: netc_pinmux"]
    ALT4_NETC_PINMUX_ETH3_RXD0 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO09 of instance: gpio2"]
    ALT5_GPIO2_IO9 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_ROW05 of instance: kpp"]
    ALT6_KPP_ROW5 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO09 of instance: flexio1"]
    ALT8_FLEXIO1_FLEXIO9 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH4_RXD00 of instance: netc_pinmux"]
    ALT9_NETC_PINMUX_ETH4_RXD0 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: ECAT_RX_DATA0_0 of instance: ecat"]
    ALT10_ECAT_PT0_RXD0 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_ADDR00 of instance: ahb_sramc"]
    ALT12_AHB_SRAMC_ADDR0 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB109MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB109MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB109MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB109MuxMode {
        SwMuxCtlPadGpioEmcB109MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB109MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB109MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB109MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB110MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_ADDR01 of instance: semc"]
    ALT0_SEMC_ADDR1 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM3_PWMB03 of instance: flexpwm3"]
    ALT1_FLEXPWM3_PWMB3 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SINC3_EMBIT03 of instance: sinc3"]
    ALT2_SINC3_EMBIT3 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART4_RI_B of instance: lpuart4"]
    ALT3_LPUART4_RI_B = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH3_RXD01 of instance: netc_pinmux"]
    ALT4_NETC_PINMUX_ETH3_RXD1 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO10 of instance: gpio2"]
    ALT5_GPIO2_IO10 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_COL05 of instance: kpp"]
    ALT6_KPP_COL5 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO10 of instance: flexio1"]
    ALT8_FLEXIO1_FLEXIO10 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH4_RXD01 of instance: netc_pinmux"]
    ALT9_NETC_PINMUX_ETH4_RXD1 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: ECAT_RX_DATA1_0 of instance: ecat"]
    ALT10_ECAT_PT0_RXD1 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_ADDR01 of instance: ahb_sramc"]
    ALT12_AHB_SRAMC_ADDR1 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB110MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB110MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB110MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB110MuxMode {
        SwMuxCtlPadGpioEmcB110MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB110MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB110MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB110MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB111MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_ADDR02 of instance: semc"]
    ALT0_SEMC_ADDR2 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM3_PWMA03 of instance: flexpwm3"]
    ALT1_FLEXPWM3_PWMA3 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SINC_FILTER_GLUE3_BREAK of instance: sinc_filter_glue3"]
    ALT2_SINC_FILTER_GLUE3_BREAK = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART4_DTR_B of instance: lpuart4"]
    ALT3_LPUART4_DTR_B = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH3_RX_DV of instance: netc_pinmux"]
    ALT4_NETC_PINMUX_ETH3_RX_DV = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO11 of instance: gpio2"]
    ALT5_GPIO2_IO11 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_ROW04 of instance: kpp"]
    ALT6_KPP_ROW4 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO11 of instance: flexio1"]
    ALT8_FLEXIO1_FLEXIO11 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH4_RX_DV of instance: netc_pinmux"]
    ALT9_NETC_PINMUX_ETH4_RX_DV = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: ECAT_RX_DV_0 of instance: ecat"]
    ALT10_ECAT_PT0_RX_DV = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_ADDR02 of instance: ahb_sramc"]
    ALT12_AHB_SRAMC_ADDR2 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB111MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB111MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB111MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB111MuxMode {
        SwMuxCtlPadGpioEmcB111MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB111MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB111MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB111MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB112MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_ADDR03 of instance: semc"]
    ALT0_SEMC_ADDR3 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM4_PWMA00 of instance: flexpwm4"]
    ALT1_FLEXPWM4_PWMA0 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART4_TX of instance: lpuart4"]
    ALT2_LPUART4_TX = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH3_RX_ER of instance: netc_pinmux"]
    ALT4_NETC_PINMUX_ETH3_RX_ER = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO12 of instance: gpio2"]
    ALT5_GPIO2_IO12 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_COL04 of instance: kpp"]
    ALT6_KPP_COL4 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO12 of instance: flexio1"]
    ALT8_FLEXIO1_FLEXIO12 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH4_RX_ER of instance: netc_pinmux"]
    ALT9_NETC_PINMUX_ETH4_RX_ER = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: ECAT_PT0_RX_ER of instance: ecat"]
    ALT10_ECAT_PT0_RX_ER = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_ADDR03 of instance: ahb_sramc"]
    ALT12_AHB_SRAMC_ADDR3 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB112MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB112MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB112MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB112MuxMode {
        SwMuxCtlPadGpioEmcB112MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB112MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB112MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB112MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB113MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_ADDR04 of instance: semc"]
    ALT0_SEMC_ADDR4 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM4_PWMB00 of instance: flexpwm4"]
    ALT1_FLEXPWM4_PWMB0 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART4_RX of instance: lpuart4"]
    ALT2_LPUART4_RX = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_PINMUX_ETH2_RX_DV of instance: netc_pinmux"]
    ALT3_NETC_PINMUX_ETH2_RX_DV = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH3_TX_ER of instance: netc_pinmux"]
    ALT4_NETC_PINMUX_ETH3_TX_ER = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO13 of instance: gpio2"]
    ALT5_GPIO2_IO13 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_COL01 of instance: kpp"]
    ALT6_KPP_COL1 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO13 of instance: flexio1"]
    ALT8_FLEXIO1_FLEXIO13 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH4_TX_ER of instance: netc_pinmux"]
    ALT9_NETC_PINMUX_ETH4_TX_ER = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: QTIMER1_TIMER1 of instance: qtimer1"]
    ALT10_QTIMER1_TIMER1 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_ADDR04 of instance: ahb_sramc"]
    ALT12_AHB_SRAMC_ADDR4 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB113MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB113MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB113MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB113MuxMode {
        SwMuxCtlPadGpioEmcB113MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB113MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB113MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB113MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB114MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_ADDR05 of instance: semc"]
    ALT0_SEMC_ADDR5 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM4_PWMA01 of instance: flexpwm4"]
    ALT1_FLEXPWM4_PWMA1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART5_TX of instance: lpuart5"]
    ALT2_LPUART5_TX = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_PINMUX_ETH2_TX_EN of instance: netc_pinmux"]
    ALT3_NETC_PINMUX_ETH2_TX_EN = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_ETH3_CRS of instance: netc"]
    ALT4_NETC_ETH3_CRS = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO14 of instance: gpio2"]
    ALT5_GPIO2_IO14 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_ROW00 of instance: kpp"]
    ALT6_KPP_ROW0 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO14 of instance: flexio1"]
    ALT8_FLEXIO1_FLEXIO14 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_ETH4_CRS of instance: netc"]
    ALT9_NETC_ETH4_CRS = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: LPUART4_CTS_B of instance: lpuart4"]
    ALT10_LPUART4_CTS_B = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_ADDR05 of instance: ahb_sramc"]
    ALT12_AHB_SRAMC_ADDR5 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB114MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB114MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB114MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB114MuxMode {
        SwMuxCtlPadGpioEmcB114MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB114MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB114MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB114MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB115MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_ADDR06 of instance: semc"]
    ALT0_SEMC_ADDR6 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM4_PWMB01 of instance: flexpwm4"]
    ALT1_FLEXPWM4_PWMB1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART5_RX of instance: lpuart5"]
    ALT2_LPUART5_RX = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_PINMUX_ETH2_TX_CLK of instance: netc_pinmux"]
    ALT3_NETC_PINMUX_ETH2_TX_CLK = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_ETH3_COL of instance: netc"]
    ALT4_NETC_ETH3_COL = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO15 of instance: gpio2"]
    ALT5_GPIO2_IO15 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_COL00 of instance: kpp"]
    ALT6_KPP_COL0 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO15 of instance: flexio1"]
    ALT8_FLEXIO1_FLEXIO15 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_ETH4_COL of instance: netc"]
    ALT9_NETC_ETH4_COL = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: LPUART4_RTS_B of instance: lpuart4"]
    ALT10_LPUART4_RTS_B = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_ADDR06 of instance: ahb_sramc"]
    ALT12_AHB_SRAMC_ADDR6 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB115MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB115MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB115MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB115MuxMode {
        SwMuxCtlPadGpioEmcB115MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB115MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB115MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB115MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB116MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_ADDR07 of instance: semc"]
    ALT0_SEMC_ADDR7 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM4_PWMB02 of instance: flexpwm4"]
    ALT1_FLEXPWM4_PWMB2 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART9_TX of instance: lpuart9"]
    ALT2_LPUART9_TX = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_PINMUX_ETH2_RXD00 of instance: netc_pinmux"]
    ALT3_NETC_PINMUX_ETH2_RXD0 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_ETH3_SLV_MDC of instance: netc"]
    ALT4_NETC_ETH3_SLV_MDC = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO16 of instance: gpio2"]
    ALT5_GPIO2_IO16 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: NETC_ETH4_SLV_MDC of instance: netc"]
    ALT6_NETC_ETH4_SLV_MDC = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO16 of instance: flexio1"]
    ALT8_FLEXIO1_FLEXIO16 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_ETH2_SLV_MDC of instance: netc"]
    ALT9_NETC_ETH2_SLV_MDC = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: LPSPI6_PCS2 of instance: lpspi6"]
    ALT10_LPSPI6_PCS2 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_ADDR07 of instance: ahb_sramc"]
    ALT12_AHB_SRAMC_ADDR7 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB116MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB116MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB116MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB116MuxMode {
        SwMuxCtlPadGpioEmcB116MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB116MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB116MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB116MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB117MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_ADDR08 of instance: semc"]
    ALT0_SEMC_ADDR8 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM4_PWMA02 of instance: flexpwm4"]
    ALT1_FLEXPWM4_PWMA2 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART9_RX of instance: lpuart9"]
    ALT2_LPUART9_RX = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_PINMUX_ETH2_RXD01 of instance: netc_pinmux"]
    ALT3_NETC_PINMUX_ETH2_RXD1 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_ETH3_SLV_MDIO of instance: netc"]
    ALT4_NETC_ETH3_SLV_MDIO = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO17 of instance: gpio2"]
    ALT5_GPIO2_IO17 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: NETC_ETH4_SLV_MDIO of instance: netc"]
    ALT6_NETC_ETH4_SLV_MDIO = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO17 of instance: flexio1"]
    ALT8_FLEXIO1_FLEXIO17 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_ETH2_SLV_MDIO of instance: netc"]
    ALT9_NETC_ETH2_SLV_MDIO = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: LPSPI6_PCS1 of instance: lpspi6"]
    ALT10_LPSPI6_PCS1 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_ADDR08 of instance: ahb_sramc"]
    ALT12_AHB_SRAMC_ADDR8 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB117MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB117MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB117MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB117MuxMode {
        SwMuxCtlPadGpioEmcB117MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB117MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB117MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB117MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB118MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_ADDR09 of instance: semc"]
    ALT0_SEMC_ADDR9 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM2_PWMA00 of instance: flexpwm2"]
    ALT1_FLEXPWM2_PWMA0 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: QTIMER1_TIMER0 of instance: qtimer1"]
    ALT2_QTIMER1_TIMER0 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPSPI6_SCK of instance: lpspi6"]
    ALT3_LPSPI6_SCK = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_ETH2_CRS of instance: netc"]
    ALT4_NETC_ETH2_CRS = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO18 of instance: gpio2"]
    ALT5_GPIO2_IO18 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO18 of instance: flexio1"]
    ALT8_FLEXIO1_FLEXIO18 = 0x08,
    _RESERVED_9 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: NETC_EMDC of instance: netc"]
    ALT10_NETC_EMDC = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_ADDR09 of instance: ahb_sramc"]
    ALT12_AHB_SRAMC_ADDR9 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB118MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB118MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB118MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB118MuxMode {
        SwMuxCtlPadGpioEmcB118MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB118MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB118MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB118MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB119MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_ADDR11 of instance: semc"]
    ALT0_SEMC_ADDR11 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM2_PWMB00 of instance: flexpwm2"]
    ALT1_FLEXPWM2_PWMB0 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: QTIMER2_TIMER0 of instance: qtimer2"]
    ALT2_QTIMER2_TIMER0 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPSPI6_SDI of instance: lpspi6"]
    ALT3_LPSPI6_SDI = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_ETH2_COL of instance: netc"]
    ALT4_NETC_ETH2_COL = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO19 of instance: gpio2"]
    ALT5_GPIO2_IO19 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO19 of instance: flexio1"]
    ALT8_FLEXIO1_FLEXIO19 = 0x08,
    _RESERVED_9 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: NETC_EMDIO of instance: netc"]
    ALT10_NETC_EMDIO = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_ADDR11 of instance: ahb_sramc"]
    ALT12_AHB_SRAMC_ADDR11 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB119MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB119MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB119MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB119MuxMode {
        SwMuxCtlPadGpioEmcB119MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB119MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB119MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB119MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB120MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_ADDR12 of instance: semc"]
    ALT0_SEMC_ADDR12 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM2_PWMA01 of instance: flexpwm2"]
    ALT1_FLEXPWM2_PWMA1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: QTIMER3_TIMER0 of instance: qtimer3"]
    ALT2_QTIMER3_TIMER0 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPSPI6_SDO of instance: lpspi6"]
    ALT3_LPSPI6_SDO = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH2_TX_ER of instance: netc_pinmux"]
    ALT4_NETC_PINMUX_ETH2_TX_ER = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO20 of instance: gpio2"]
    ALT5_GPIO2_IO20 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO20 of instance: flexio1"]
    ALT8_FLEXIO1_FLEXIO20 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_ADDR12 of instance: ahb_sramc"]
    ALT12_AHB_SRAMC_ADDR12 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB120MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB120MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB120MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB120MuxMode {
        SwMuxCtlPadGpioEmcB120MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB120MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB120MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB120MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB121MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_BA0 of instance: semc"]
    ALT0_SEMC_BA0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM2_PWMB01 of instance: flexpwm2"]
    ALT1_FLEXPWM2_PWMB1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: QTIMER4_TIMER0 of instance: qtimer4"]
    ALT2_QTIMER4_TIMER0 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPSPI6_PCS0 of instance: lpspi6"]
    ALT3_LPSPI6_PCS0 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH2_RX_CLK of instance: netc_pinmux"]
    ALT4_NETC_PINMUX_ETH2_RX_CLK = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO21 of instance: gpio2"]
    ALT5_GPIO2_IO21 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO21 of instance: flexio1"]
    ALT8_FLEXIO1_FLEXIO21 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPUART4_CTS_B of instance: lpuart4"]
    ALT9_LPUART4_CTS_B = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: FLEXSPI2_BUS2BIT_B_DQS of instance: flexspi2_bus2bit"]
    ALT10_FLEXSPI2_BUS2BIT_B_DQS = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_ADDR13 of instance: ahb_sramc"]
    ALT12_AHB_SRAMC_ADDR13 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB121MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB121MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB121MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB121MuxMode {
        SwMuxCtlPadGpioEmcB121MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB121MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB121MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB121MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB122MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_BA1 of instance: semc"]
    ALT0_SEMC_BA1 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM2_PWMB02 of instance: flexpwm2"]
    ALT1_FLEXPWM2_PWMB2 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: QTIMER5_TIMER0 of instance: qtimer5"]
    ALT2_QTIMER5_TIMER0 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPSPI4_SCK of instance: lpspi4"]
    ALT3_LPSPI4_SCK = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH2_RXD02 of instance: netc_pinmux"]
    ALT4_NETC_PINMUX_ETH2_RXD2 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO22 of instance: gpio2"]
    ALT5_GPIO2_IO22 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO22 of instance: flexio1"]
    ALT8_FLEXIO1_FLEXIO22 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPUART4_RTS_B of instance: lpuart4"]
    ALT9_LPUART4_RTS_B = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: FLEXSPI2_BUS2BIT_B_DATA03 of instance: flexspi2_bus2bit"]
    ALT10_FLEXSPI2_BUS2BIT_B_DATA3 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_ADDR14 of instance: ahb_sramc"]
    ALT12_AHB_SRAMC_ADDR14 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB122MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB122MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB122MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB122MuxMode {
        SwMuxCtlPadGpioEmcB122MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB122MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB122MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB122MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB123MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_ADDR10 of instance: semc"]
    ALT0_SEMC_ADDR10 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM2_PWMA02 of instance: flexpwm2"]
    ALT1_FLEXPWM2_PWMA2 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: QTIMER6_TIMER0 of instance: qtimer6"]
    ALT2_QTIMER6_TIMER0 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPSPI4_SDI of instance: lpspi4"]
    ALT3_LPSPI4_SDI = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH2_RXD03 of instance: netc_pinmux"]
    ALT4_NETC_PINMUX_ETH2_RXD3 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO23 of instance: gpio2"]
    ALT5_GPIO2_IO23 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO23 of instance: flexio1"]
    ALT8_FLEXIO1_FLEXIO23 = 0x08,
    _RESERVED_9 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: FLEXSPI2_BUS2BIT_B_DATA02 of instance: flexspi2_bus2bit"]
    ALT10_FLEXSPI2_BUS2BIT_B_DATA2 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_ADDR10 of instance: ahb_sramc"]
    ALT12_AHB_SRAMC_ADDR10 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB123MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB123MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB123MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB123MuxMode {
        SwMuxCtlPadGpioEmcB123MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB123MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB123MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB123MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB124MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_CAS of instance: semc"]
    ALT0_SEMC_CAS = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMA00 of instance: flexpwm1"]
    ALT1_FLEXPWM1_PWMA0 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: QTIMER7_TIMER0 of instance: qtimer7"]
    ALT2_QTIMER7_TIMER0 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPSPI4_SDO of instance: lpspi4"]
    ALT3_LPSPI4_SDO = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH2_TXD03 of instance: netc_pinmux"]
    ALT4_NETC_PINMUX_ETH2_TXD3 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO24 of instance: gpio2"]
    ALT5_GPIO2_IO24 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO24 of instance: flexio1"]
    ALT8_FLEXIO1_FLEXIO24 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_ETH3_SLV_MDC of instance: netc"]
    ALT9_NETC_ETH3_SLV_MDC = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: FLEXSPI2_BUS2BIT_B_DATA01 of instance: flexspi2_bus2bit"]
    ALT10_FLEXSPI2_BUS2BIT_B_DATA1 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_ADDR15 of instance: ahb_sramc"]
    ALT12_AHB_SRAMC_ADDR15 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB124MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB124MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB124MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB124MuxMode {
        SwMuxCtlPadGpioEmcB124MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB124MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB124MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB124MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB125MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_RAS of instance: semc"]
    ALT0_SEMC_RAS = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMB00 of instance: flexpwm1"]
    ALT1_FLEXPWM1_PWMB0 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: QTIMER8_TIMER0 of instance: qtimer8"]
    ALT2_QTIMER8_TIMER0 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPSPI4_PCS0 of instance: lpspi4"]
    ALT3_LPSPI4_PCS0 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH2_TXD02 of instance: netc_pinmux"]
    ALT4_NETC_PINMUX_ETH2_TXD2 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO25 of instance: gpio2"]
    ALT5_GPIO2_IO25 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO25 of instance: flexio1"]
    ALT8_FLEXIO1_FLEXIO25 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_ETH3_SLV_MDIO of instance: netc"]
    ALT9_NETC_ETH3_SLV_MDIO = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: FLEXSPI2_BUS2BIT_B_DATA00 of instance: flexspi2_bus2bit"]
    ALT10_FLEXSPI2_BUS2BIT_B_DATA0 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_ADDR16 of instance: ahb_sramc"]
    ALT12_AHB_SRAMC_ADDR16 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB125MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB125MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB125MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB125MuxMode {
        SwMuxCtlPadGpioEmcB125MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB125MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB125MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB125MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB126MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_CLK of instance: semc"]
    ALT0_SEMC_CLK = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMA01 of instance: flexpwm1"]
    ALT1_FLEXPWM1_PWMA1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XBAR1_XBAR_INOUT10 of instance: xbar1"]
    ALT2_XBAR1_XBAR_INOUT10 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: FLEXSPI2_BUS2BIT_A_SS1_B of instance: flexspi2_bus2bit"]
    ALT3_FLEXSPI2_BUS2BIT_A_SS1_B = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH2_TXD01 of instance: netc_pinmux"]
    ALT4_NETC_PINMUX_ETH2_TXD1 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO26 of instance: gpio2"]
    ALT5_GPIO2_IO26 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: ECAT_TX_DATA1_1 of instance: ecat"]
    ALT6_ECAT_PT1_TXD1 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: LPSPI6_SCK of instance: lpspi6"]
    ALT10_LPSPI6_SCK = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_WE of instance: ahb_sramc"]
    ALT12_AHB_SRAMC_WE = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB126MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB126MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB126MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB126MuxMode {
        SwMuxCtlPadGpioEmcB126MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB126MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB126MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB126MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB127MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_CKE of instance: semc"]
    ALT0_SEMC_CKE = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMB01 of instance: flexpwm1"]
    ALT1_FLEXPWM1_PWMB1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XBAR1_XBAR_INOUT11 of instance: xbar1"]
    ALT2_XBAR1_XBAR_INOUT11 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: FLEXSPI2_BUS2BIT_B_SS1_B of instance: flexspi2_bus2bit"]
    ALT3_FLEXSPI2_BUS2BIT_B_SS1_B = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH2_TXD00 of instance: netc_pinmux"]
    ALT4_NETC_PINMUX_ETH2_TXD0 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO27 of instance: gpio2"]
    ALT5_GPIO2_IO27 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: ECAT_TX_DATA0_1 of instance: ecat"]
    ALT6_ECAT_PT1_TXD0 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPUART6_RI_B of instance: lpuart6"]
    ALT9_LPUART6_RI_B = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: LPSPI6_SDI of instance: lpspi6"]
    ALT10_LPSPI6_SDI = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_OEB of instance: ahb_sramc"]
    ALT12_AHB_SRAMC_OEB = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB127MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB127MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB127MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB127MuxMode {
        SwMuxCtlPadGpioEmcB127MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB127MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB127MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB127MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB128MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_WE of instance: semc"]
    ALT0_SEMC_WE = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMB02 of instance: flexpwm1"]
    ALT1_FLEXPWM1_PWMB2 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XBAR1_XBAR_INOUT12 of instance: xbar1"]
    ALT2_XBAR1_XBAR_INOUT12 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: FLEXSPI2_BUS2BIT_B_SS0_B of instance: flexspi2_bus2bit"]
    ALT3_FLEXSPI2_BUS2BIT_B_SS0_B = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH2_TX_EN of instance: netc_pinmux"]
    ALT4_NETC_PINMUX_ETH2_TX_EN = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO28 of instance: gpio2"]
    ALT5_GPIO2_IO28 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: ECAT_TX_EN_1 of instance: ecat"]
    ALT6_ECAT_PT1_TX_EN = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPUART6_DTR_B of instance: lpuart6"]
    ALT9_LPUART6_DTR_B = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: LPSPI6_SDO of instance: lpspi6"]
    ALT10_LPSPI6_SDO = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_ADV of instance: ahb_sramc"]
    ALT12_AHB_SRAMC_ADV = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB128MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB128MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB128MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB128MuxMode {
        SwMuxCtlPadGpioEmcB128MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB128MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB128MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB128MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB129MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_CS0 of instance: semc"]
    ALT0_SEMC_CS0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMA02 of instance: flexpwm1"]
    ALT1_FLEXPWM1_PWMA2 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XBAR1_XBAR_INOUT13 of instance: xbar1"]
    ALT2_XBAR1_XBAR_INOUT13 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: FLEXSPI2_BUS2BIT_B_DQS of instance: flexspi2_bus2bit"]
    ALT3_FLEXSPI2_BUS2BIT_B_DQS = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH2_TX_CLK of instance: netc_pinmux"]
    ALT4_NETC_PINMUX_ETH2_TX_CLK = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO29 of instance: gpio2"]
    ALT5_GPIO2_IO29 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: ECAT_TX_CLK_1 of instance: ecat"]
    ALT6_ECAT_PT1_TX_CLK = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPUART6_DCD_B of instance: lpuart6"]
    ALT9_LPUART6_DCD_B = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: LPSPI6_PCS0 of instance: lpspi6"]
    ALT10_LPSPI6_PCS0 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_CS0 of instance: ahb_sramc"]
    ALT12_AHB_SRAMC_CS0 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB129MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB129MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB129MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB129MuxMode {
        SwMuxCtlPadGpioEmcB129MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB129MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB129MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB129MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB130MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA08 of instance: semc"]
    ALT0_SEMC_DATA8 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM3_PWMA00 of instance: flexpwm3"]
    ALT1_FLEXPWM3_PWMA0 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XBAR1_XBAR_INOUT14 of instance: xbar1"]
    ALT2_XBAR1_XBAR_INOUT14 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: FLEXSPI2_BUS2BIT_B_DATA03 of instance: flexspi2_bus2bit"]
    ALT3_FLEXSPI2_BUS2BIT_B_DATA3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH2_RXD00 of instance: netc_pinmux"]
    ALT4_NETC_PINMUX_ETH2_RXD0 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO30 of instance: gpio2"]
    ALT5_GPIO2_IO30 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: ECAT_RX_DATA0_1 of instance: ecat"]
    ALT6_ECAT_PT1_RXD0 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPUART6_DSR_B of instance: lpuart6"]
    ALT9_LPUART6_DSR_B = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: LPSPI6_PCS1 of instance: lpspi6"]
    ALT10_LPSPI6_PCS1 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_DATA08 of instance: ahb_sramc"]
    ALT12_AHB_SRAMC_DATA8 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB130MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB130MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB130MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB130MuxMode {
        SwMuxCtlPadGpioEmcB130MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB130MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB130MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB130MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB131MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA09 of instance: semc"]
    ALT0_SEMC_DATA9 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM3_PWMB00 of instance: flexpwm3"]
    ALT1_FLEXPWM3_PWMB0 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART6_TX of instance: lpuart6"]
    ALT2_LPUART6_TX = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: FLEXSPI2_BUS2BIT_B_DATA02 of instance: flexspi2_bus2bit"]
    ALT3_FLEXSPI2_BUS2BIT_B_DATA2 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH2_RXD01 of instance: netc_pinmux"]
    ALT4_NETC_PINMUX_ETH2_RXD1 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO31 of instance: gpio2"]
    ALT5_GPIO2_IO31 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: ECAT_RX_DATA1_1 of instance: ecat"]
    ALT6_ECAT_PT1_RXD1 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPSPI5_SCK of instance: lpspi5"]
    ALT9_LPSPI5_SCK = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: LPSPI6_PCS2 of instance: lpspi6"]
    ALT10_LPSPI6_PCS2 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_DATA09 of instance: ahb_sramc"]
    ALT12_AHB_SRAMC_DATA9 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB131MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB131MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB131MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB131MuxMode {
        SwMuxCtlPadGpioEmcB131MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB131MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB131MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB131MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB132MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA10 of instance: semc"]
    ALT0_SEMC_DATA10 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM3_PWMA01 of instance: flexpwm3"]
    ALT1_FLEXPWM3_PWMA1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART6_RX of instance: lpuart6"]
    ALT2_LPUART6_RX = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: FLEXSPI2_BUS2BIT_B_DATA01 of instance: flexspi2_bus2bit"]
    ALT3_FLEXSPI2_BUS2BIT_B_DATA1 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH2_RX_DV of instance: netc_pinmux"]
    ALT4_NETC_PINMUX_ETH2_RX_DV = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO00 of instance: gpio3"]
    ALT5_GPIO3_IO0 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: ECAT_RX_DV_1 of instance: ecat"]
    ALT6_ECAT_PT1_RX_DV = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPSPI5_SDO of instance: lpspi5"]
    ALT9_LPSPI5_SDO = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: LPSPI6_PCS3 of instance: lpspi6"]
    ALT10_LPSPI6_PCS3 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_DATA10 of instance: ahb_sramc"]
    ALT12_AHB_SRAMC_DATA10 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB132MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB132MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB132MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB132MuxMode {
        SwMuxCtlPadGpioEmcB132MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB132MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB132MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB132MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB133MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA11 of instance: semc"]
    ALT0_SEMC_DATA11 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM3_PWMB01 of instance: flexpwm3"]
    ALT1_FLEXPWM3_PWMB1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART6_CTS_B of instance: lpuart6"]
    ALT2_LPUART6_CTS_B = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: FLEXSPI2_BUS2BIT_B_DATA00 of instance: flexspi2_bus2bit"]
    ALT3_FLEXSPI2_BUS2BIT_B_DATA0 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH2_RX_ER of instance: netc_pinmux"]
    ALT4_NETC_PINMUX_ETH2_RX_ER = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO01 of instance: gpio3"]
    ALT5_GPIO3_IO1 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: ECAT_RX_ER_1 of instance: ecat"]
    ALT6_ECAT_PT1_RX_ER = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPSPI5_SDI of instance: lpspi5"]
    ALT9_LPSPI5_SDI = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: NETC_PINMUX_ETH2_RX_CLK of instance: netc_pinmux"]
    ALT10_NETC_PINMUX_ETH2_RX_CLK = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_DATA11 of instance: ahb_sramc"]
    ALT12_AHB_SRAMC_DATA11 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB133MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB133MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB133MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB133MuxMode {
        SwMuxCtlPadGpioEmcB133MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB133MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB133MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB133MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB134MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA12 of instance: semc"]
    ALT0_SEMC_DATA12 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM3_PWMB02 of instance: flexpwm3"]
    ALT1_FLEXPWM3_PWMB2 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART6_RTS_B of instance: lpuart6"]
    ALT2_LPUART6_RTS_B = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: FLEXSPI2_BUS2BIT_B_SCLK of instance: flexspi2_bus2bit"]
    ALT3_FLEXSPI2_BUS2BIT_B_SCLK = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH2_RXD02 of instance: netc_pinmux"]
    ALT4_NETC_PINMUX_ETH2_RXD2 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO02 of instance: gpio3"]
    ALT5_GPIO3_IO2 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: ECAT_RX_DATA2_1 of instance: ecat"]
    ALT6_ECAT_PT1_RXD2 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH0_TXD00 of instance: netc_pinmux"]
    ALT9_NETC_PINMUX_ETH0_TXD0 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: LPSPI5_PCS0 of instance: lpspi5"]
    ALT10_LPSPI5_PCS0 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_DATA12 of instance: ahb_sramc"]
    ALT12_AHB_SRAMC_DATA12 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB134MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB134MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB134MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB134MuxMode {
        SwMuxCtlPadGpioEmcB134MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB134MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB134MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB134MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB135MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA13 of instance: semc"]
    ALT0_SEMC_DATA13 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM3_PWMA02 of instance: flexpwm3"]
    ALT1_FLEXPWM3_PWMA2 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART5_TX of instance: lpuart5"]
    ALT2_LPUART5_TX = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: FLEXSPI2_BUS2BIT_A_DATA00 of instance: flexspi2_bus2bit"]
    ALT3_FLEXSPI2_BUS2BIT_A_DATA0 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH2_RXD03 of instance: netc_pinmux"]
    ALT4_NETC_PINMUX_ETH2_RXD3 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO03 of instance: gpio3"]
    ALT5_GPIO3_IO3 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: ECAT_RX_DATA3_1 of instance: ecat"]
    ALT6_ECAT_PT1_RXD3 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH0_TXD01 of instance: netc_pinmux"]
    ALT9_NETC_PINMUX_ETH0_TXD1 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: LPSPI5_PCS1 of instance: lpspi5"]
    ALT10_LPSPI5_PCS1 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_DATA13 of instance: ahb_sramc"]
    ALT12_AHB_SRAMC_DATA13 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB135MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB135MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB135MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB135MuxMode {
        SwMuxCtlPadGpioEmcB135MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB135MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB135MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB135MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB136MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA14 of instance: semc"]
    ALT0_SEMC_DATA14 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMA00 of instance: flexpwm1"]
    ALT1_FLEXPWM1_PWMA0 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART5_RX of instance: lpuart5"]
    ALT2_LPUART5_RX = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: FLEXSPI2_BUS2BIT_A_DATA01 of instance: flexspi2_bus2bit"]
    ALT3_FLEXSPI2_BUS2BIT_A_DATA1 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH2_TXD03 of instance: netc_pinmux"]
    ALT4_NETC_PINMUX_ETH2_TXD3 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO04 of instance: gpio3"]
    ALT5_GPIO3_IO4 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: ECAT_TX_DATA3_1 of instance: ecat"]
    ALT6_ECAT_PT1_TXD3 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH0_TX_EN of instance: netc_pinmux"]
    ALT9_NETC_PINMUX_ETH0_TX_EN = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_DATA14 of instance: ahb_sramc"]
    ALT12_AHB_SRAMC_DATA14 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB136MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB136MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB136MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB136MuxMode {
        SwMuxCtlPadGpioEmcB136MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB136MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB136MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB136MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB137MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA15 of instance: semc"]
    ALT0_SEMC_DATA15 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMB00 of instance: flexpwm1"]
    ALT1_FLEXPWM1_PWMB0 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART5_CTS_B of instance: lpuart5"]
    ALT2_LPUART5_CTS_B = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: FLEXSPI2_BUS2BIT_A_DATA02 of instance: flexspi2_bus2bit"]
    ALT3_FLEXSPI2_BUS2BIT_A_DATA2 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH2_TXD02 of instance: netc_pinmux"]
    ALT4_NETC_PINMUX_ETH2_TXD2 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO05 of instance: gpio3"]
    ALT5_GPIO3_IO5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: ECAT_TX_DATA2_1 of instance: ecat"]
    ALT6_ECAT_PT1_TXD2 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH0_TX_CLK of instance: netc_pinmux"]
    ALT9_NETC_PINMUX_ETH0_TX_CLK = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_DATA15 of instance: ahb_sramc"]
    ALT12_AHB_SRAMC_DATA15 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB137MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB137MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB137MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB137MuxMode {
        SwMuxCtlPadGpioEmcB137MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB137MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB137MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB137MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB138MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DM01 of instance: semc"]
    ALT0_SEMC_DM1 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMB03 of instance: flexpwm1"]
    ALT1_FLEXPWM1_PWMB3 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART5_RTS_B of instance: lpuart5"]
    ALT2_LPUART5_RTS_B = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: FLEXSPI2_BUS2BIT_A_DATA03 of instance: flexspi2_bus2bit"]
    ALT3_FLEXSPI2_BUS2BIT_A_DATA3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH2_RX_CLK of instance: netc_pinmux"]
    ALT4_NETC_PINMUX_ETH2_RX_CLK = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO06 of instance: gpio3"]
    ALT5_GPIO3_IO6 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: ECAT_RX_CLK_1 of instance: ecat"]
    ALT6_ECAT_PT1_RX_CLK = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH0_RXD00 of instance: netc_pinmux"]
    ALT9_NETC_PINMUX_ETH0_RXD0 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_UBB of instance: ahb_sramc"]
    ALT12_AHB_SRAMC_UBB = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB138MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB138MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB138MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB138MuxMode {
        SwMuxCtlPadGpioEmcB138MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB138MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB138MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB138MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB139MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DQS of instance: semc"]
    ALT0_SEMC_DQS = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMA03 of instance: flexpwm1"]
    ALT1_FLEXPWM1_PWMA3 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XBAR1_XBAR_INOUT15 of instance: xbar1"]
    ALT2_XBAR1_XBAR_INOUT15 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: FLEXSPI2_BUS2BIT_A_SS0_B of instance: flexspi2_bus2bit"]
    ALT3_FLEXSPI2_BUS2BIT_A_SS0_B = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH2_TX_ER of instance: netc_pinmux"]
    ALT4_NETC_PINMUX_ETH2_TX_ER = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO07 of instance: gpio3"]
    ALT5_GPIO3_IO7 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: QTIMER2_TIMER1 of instance: qtimer2"]
    ALT6_QTIMER2_TIMER1 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH0_RXD01 of instance: netc_pinmux"]
    ALT9_NETC_PINMUX_ETH0_RXD1 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_CS1 of instance: ahb_sramc"]
    ALT12_AHB_SRAMC_CS1 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB139MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB139MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB139MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB139MuxMode {
        SwMuxCtlPadGpioEmcB139MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB139MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB139MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB139MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB140MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_RDY of instance: semc"]
    ALT0_SEMC_RDY = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: NETC_EMDC of instance: netc"]
    ALT1_NETC_EMDC = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: NETC_ETH2_SLV_MDC of instance: netc"]
    ALT2_NETC_ETH2_SLV_MDC = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: FLEXSPI2_BUS2BIT_A_DQS of instance: flexspi2_bus2bit"]
    ALT3_FLEXSPI2_BUS2BIT_A_DQS = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_ETH2_CRS of instance: netc"]
    ALT4_NETC_ETH2_CRS = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO08 of instance: gpio3"]
    ALT5_GPIO3_IO8 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: QTIMER3_TIMER1 of instance: qtimer3"]
    ALT6_QTIMER3_TIMER1 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH0_RX_DV of instance: netc_pinmux"]
    ALT9_NETC_PINMUX_ETH0_RX_DV = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_CS2 of instance: ahb_sramc"]
    ALT12_AHB_SRAMC_CS2 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB140MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB140MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB140MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB140MuxMode {
        SwMuxCtlPadGpioEmcB140MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB140MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB140MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB140MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB141MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_CSX00 of instance: semc"]
    ALT0_SEMC_CSX0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: NETC_EMDIO of instance: netc"]
    ALT1_NETC_EMDIO = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: NETC_ETH2_SLV_MDIO of instance: netc"]
    ALT2_NETC_ETH2_SLV_MDIO = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: FLEXSPI2_BUS2BIT_A_SCLK of instance: flexspi2_bus2bit"]
    ALT3_FLEXSPI2_BUS2BIT_A_SCLK = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_ETH2_COL of instance: netc"]
    ALT4_NETC_ETH2_COL = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO09 of instance: gpio3"]
    ALT5_GPIO3_IO9 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: QTIMER4_TIMER1 of instance: qtimer4"]
    ALT6_QTIMER4_TIMER1 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH0_RX_ER of instance: netc_pinmux"]
    ALT9_NETC_PINMUX_ETH0_RX_ER = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_CS3 of instance: ahb_sramc"]
    ALT12_AHB_SRAMC_CS3 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB141MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB141MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB141MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB141MuxMode {
        SwMuxCtlPadGpioEmcB141MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB141MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB141MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB141MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB200MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA16 of instance: semc"]
    ALT0_SEMC_DATA16 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: CCM_ENET_REF_CLK_25M of instance: ccm"]
    ALT1_CCM_ENET_REF_CLK_25M = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: QTIMER5_TIMER1 of instance: qtimer5"]
    ALT2_QTIMER5_TIMER1 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_EMDC of instance: netc"]
    ALT3_NETC_EMDC = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH0_RX_CLK of instance: netc_pinmux"]
    ALT4_NETC_PINMUX_ETH0_RX_CLK = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO10 of instance: gpio3"]
    ALT5_GPIO3_IO10 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_XBAR_INOUT20 of instance: xbar1"]
    ALT6_XBAR1_XBAR_INOUT20 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPSPI5_SCK of instance: lpspi5"]
    ALT8_LPSPI5_SCK = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPI2C3_SCL of instance: lpi2c3"]
    ALT9_LPI2C3_SCL = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: FLEXPWM3_PWMA00 of instance: flexpwm3"]
    ALT10_FLEXPWM3_PWMA0 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_RX_CLK_0 of instance: ecat"]
    ALT12_ECAT_PT0_RX_CLK = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB200MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB200MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB200MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB200MuxMode {
        SwMuxCtlPadGpioEmcB200MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB200MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB200MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB200MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB201MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA17 of instance: semc"]
    ALT0_SEMC_DATA17 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: USDHC2_CD_B of instance: usdhc2"]
    ALT1_USDHC2_CD_B = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: QTIMER6_TIMER1 of instance: qtimer6"]
    ALT2_QTIMER6_TIMER1 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_EMDIO of instance: netc"]
    ALT3_NETC_EMDIO = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH0_RXD02 of instance: netc_pinmux"]
    ALT4_NETC_PINMUX_ETH0_RXD2 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO11 of instance: gpio3"]
    ALT5_GPIO3_IO11 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_XBAR_INOUT21 of instance: xbar1"]
    ALT6_XBAR1_XBAR_INOUT21 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPSPI5_PCS0 of instance: lpspi5"]
    ALT8_LPSPI5_PCS0 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPI2C3_SDA of instance: lpi2c3"]
    ALT9_LPI2C3_SDA = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: FLEXPWM3_PWMB00 of instance: flexpwm3"]
    ALT10_FLEXPWM3_PWMB0 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_RX_DATA2_0 of instance: ecat"]
    ALT12_ECAT_PT0_RXD2 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB201MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB201MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB201MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB201MuxMode {
        SwMuxCtlPadGpioEmcB201MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB201MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB201MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB201MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB202MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA18 of instance: semc"]
    ALT0_SEMC_DATA18 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: USDHC2_WP of instance: usdhc2"]
    ALT1_USDHC2_WP = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: QTIMER7_TIMER1 of instance: qtimer7"]
    ALT2_QTIMER7_TIMER1 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH0_RXD03 of instance: netc_pinmux"]
    ALT4_NETC_PINMUX_ETH0_RXD3 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO12 of instance: gpio3"]
    ALT5_GPIO3_IO12 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_XBAR_INOUT22 of instance: xbar1"]
    ALT6_XBAR1_XBAR_INOUT22 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPSPI5_SDO of instance: lpspi5"]
    ALT8_LPSPI5_SDO = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: CCM_CLKO1 of instance: ccm"]
    ALT9_CCM_CLKO1 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: FLEXPWM3_PWMA01 of instance: flexpwm3"]
    ALT10_FLEXPWM3_PWMA1 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_RX_DATA3_0 of instance: ecat"]
    ALT12_ECAT_PT0_RXD3 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB202MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB202MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB202MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB202MuxMode {
        SwMuxCtlPadGpioEmcB202MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB202MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB202MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB202MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB203MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA19 of instance: semc"]
    ALT0_SEMC_DATA19 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: USDHC2_VSELECT of instance: usdhc2"]
    ALT1_USDHC2_VSELECT = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: QTIMER8_TIMER1 of instance: qtimer8"]
    ALT2_QTIMER8_TIMER1 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH0_TXD02 of instance: netc_pinmux"]
    ALT4_NETC_PINMUX_ETH0_TXD2 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO13 of instance: gpio3"]
    ALT5_GPIO3_IO13 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_XBAR_INOUT23 of instance: xbar1"]
    ALT6_XBAR1_XBAR_INOUT23 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPSPI5_SDI of instance: lpspi5"]
    ALT8_LPSPI5_SDI = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_ETH3_CRS of instance: netc"]
    ALT9_NETC_ETH3_CRS = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: FLEXPWM3_PWMB01 of instance: flexpwm3"]
    ALT10_FLEXPWM3_PWMB1 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_TX_DATA2_0 of instance: ecat"]
    ALT12_ECAT_PT0_TXD2 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB203MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB203MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB203MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB203MuxMode {
        SwMuxCtlPadGpioEmcB203MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB203MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB203MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB203MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB204MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA20 of instance: semc"]
    ALT0_SEMC_DATA20 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: USDHC2_RESET_B of instance: usdhc2"]
    ALT1_USDHC2_RESET_B = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SAI2_MCLK of instance: sai2"]
    ALT2_SAI2_MCLK = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH0_TXD03 of instance: netc_pinmux"]
    ALT4_NETC_PINMUX_ETH0_TXD3 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO14 of instance: gpio3"]
    ALT5_GPIO3_IO14 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_XBAR_INOUT24 of instance: xbar1"]
    ALT6_XBAR1_XBAR_INOUT24 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPSPI3_SCK of instance: lpspi3"]
    ALT8_LPSPI3_SCK = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_ETH3_COL of instance: netc"]
    ALT9_NETC_ETH3_COL = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: FLEXPWM3_PWMB02 of instance: flexpwm3"]
    ALT10_FLEXPWM3_PWMB2 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_TX_DATA3_0 of instance: ecat"]
    ALT12_ECAT_PT0_TXD3 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB204MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB204MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB204MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB204MuxMode {
        SwMuxCtlPadGpioEmcB204MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB204MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB204MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB204MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB205MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA21 of instance: semc"]
    ALT0_SEMC_DATA21 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: NETC_ETH4_SLV_MDC of instance: netc"]
    ALT1_NETC_ETH4_SLV_MDC = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SAI2_RX_SYNC of instance: sai2"]
    ALT2_SAI2_RX_SYNC = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_PINMUX_ETH0_TXD00 of instance: netc_pinmux"]
    ALT3_NETC_PINMUX_ETH0_TXD0 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_ETH4_CRS of instance: netc"]
    ALT4_NETC_ETH4_CRS = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO15 of instance: gpio3"]
    ALT5_GPIO3_IO15 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_XBAR_INOUT25 of instance: xbar1"]
    ALT6_XBAR1_XBAR_INOUT25 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPSPI3_PCS0 of instance: lpspi3"]
    ALT8_LPSPI3_PCS0 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH3_TXD00 of instance: netc_pinmux"]
    ALT9_NETC_PINMUX_ETH3_TXD0 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: FLEXPWM3_PWMA02 of instance: flexpwm3"]
    ALT10_FLEXPWM3_PWMA2 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_TX_DATA0_0 of instance: ecat"]
    ALT12_ECAT_PT0_TXD0 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB205MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB205MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB205MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB205MuxMode {
        SwMuxCtlPadGpioEmcB205MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB205MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB205MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB205MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB206MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA22 of instance: semc"]
    ALT0_SEMC_DATA22 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: NETC_ETH4_SLV_MDIO of instance: netc"]
    ALT1_NETC_ETH4_SLV_MDIO = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SAI2_RX_BCLK of instance: sai2"]
    ALT2_SAI2_RX_BCLK = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_PINMUX_ETH0_TXD01 of instance: netc_pinmux"]
    ALT3_NETC_PINMUX_ETH0_TXD1 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_ETH4_COL of instance: netc"]
    ALT4_NETC_ETH4_COL = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO16 of instance: gpio3"]
    ALT5_GPIO3_IO16 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_XBAR_INOUT26 of instance: xbar1"]
    ALT6_XBAR1_XBAR_INOUT26 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPSPI3_SDO of instance: lpspi3"]
    ALT8_LPSPI3_SDO = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH3_TXD01 of instance: netc_pinmux"]
    ALT9_NETC_PINMUX_ETH3_TXD1 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: FLEXPWM3_PWMB03 of instance: flexpwm3"]
    ALT10_FLEXPWM3_PWMB3 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_TX_DATA1_0 of instance: ecat"]
    ALT12_ECAT_PT0_TXD1 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB206MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB206MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB206MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB206MuxMode {
        SwMuxCtlPadGpioEmcB206MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB206MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB206MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB206MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB207MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA23 of instance: semc"]
    ALT0_SEMC_DATA23 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: NETC_PINMUX_ETH4_TX_ER of instance: netc_pinmux"]
    ALT1_NETC_PINMUX_ETH4_TX_ER = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SAI2_RX_DATA of instance: sai2"]
    ALT2_SAI2_RX_DATA = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_PINMUX_ETH0_TX_EN of instance: netc_pinmux"]
    ALT3_NETC_PINMUX_ETH0_TX_EN = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO17 of instance: gpio3"]
    ALT5_GPIO3_IO17 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_XBAR_INOUT27 of instance: xbar1"]
    ALT6_XBAR1_XBAR_INOUT27 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPSPI3_SDI of instance: lpspi3"]
    ALT8_LPSPI3_SDI = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH3_TX_EN of instance: netc_pinmux"]
    ALT9_NETC_PINMUX_ETH3_TX_EN = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: FLEXPWM3_PWMA03 of instance: flexpwm3"]
    ALT10_FLEXPWM3_PWMA3 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_TX_EN_0 of instance: ecat"]
    ALT12_ECAT_PT0_TX_EN = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB207MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB207MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB207MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB207MuxMode {
        SwMuxCtlPadGpioEmcB207MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB207MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB207MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB207MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB208MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DM02 of instance: semc"]
    ALT0_SEMC_DM2 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: NETC_PINMUX_ETH4_RX_CLK of instance: netc_pinmux"]
    ALT1_NETC_PINMUX_ETH4_RX_CLK = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SAI2_TX_DATA of instance: sai2"]
    ALT2_SAI2_TX_DATA = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_PINMUX_ETH0_TX_CLK of instance: netc_pinmux"]
    ALT3_NETC_PINMUX_ETH0_TX_CLK = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO18 of instance: gpio3"]
    ALT5_GPIO3_IO18 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_XBAR_INOUT28 of instance: xbar1"]
    ALT6_XBAR1_XBAR_INOUT28 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPSPI3_PCS3 of instance: lpspi3"]
    ALT8_LPSPI3_PCS3 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH3_TX_CLK of instance: netc_pinmux"]
    ALT9_NETC_PINMUX_ETH3_TX_CLK = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: CCM_CLKO2 of instance: ccm"]
    ALT10_CCM_CLKO2 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_TX_CLK_0 of instance: ecat"]
    ALT12_ECAT_PT0_TX_CLK = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB208MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB208MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB208MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB208MuxMode {
        SwMuxCtlPadGpioEmcB208MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB208MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB208MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB208MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB209MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA24 of instance: semc"]
    ALT0_SEMC_DATA24 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: NETC_PINMUX_ETH4_RXD03 of instance: netc_pinmux"]
    ALT1_NETC_PINMUX_ETH4_RXD3 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SAI2_TX_BCLK of instance: sai2"]
    ALT2_SAI2_TX_BCLK = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_PINMUX_ETH0_RXD00 of instance: netc_pinmux"]
    ALT3_NETC_PINMUX_ETH0_RXD0 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO19 of instance: gpio3"]
    ALT5_GPIO3_IO19 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_XBAR_INOUT29 of instance: xbar1"]
    ALT6_XBAR1_XBAR_INOUT29 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPSPI3_PCS2 of instance: lpspi3"]
    ALT8_LPSPI3_PCS2 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH3_RXD00 of instance: netc_pinmux"]
    ALT9_NETC_PINMUX_ETH3_RXD0 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: QTIMER1_TIMER0 of instance: qtimer1"]
    ALT10_QTIMER1_TIMER0 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_RX_DATA0_0 of instance: ecat"]
    ALT12_ECAT_PT0_RXD0 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB209MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB209MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB209MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB209MuxMode {
        SwMuxCtlPadGpioEmcB209MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB209MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB209MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB209MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB210MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA25 of instance: semc"]
    ALT0_SEMC_DATA25 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: NETC_PINMUX_ETH4_RXD02 of instance: netc_pinmux"]
    ALT1_NETC_PINMUX_ETH4_RXD2 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SAI2_TX_SYNC of instance: sai2"]
    ALT2_SAI2_TX_SYNC = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_PINMUX_ETH0_RXD01 of instance: netc_pinmux"]
    ALT3_NETC_PINMUX_ETH0_RXD1 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO20 of instance: gpio3"]
    ALT5_GPIO3_IO20 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_XBAR_INOUT30 of instance: xbar1"]
    ALT6_XBAR1_XBAR_INOUT30 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPSPI3_PCS1 of instance: lpspi3"]
    ALT8_LPSPI3_PCS1 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH3_RXD01 of instance: netc_pinmux"]
    ALT9_NETC_PINMUX_ETH3_RXD1 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: QTIMER1_TIMER1 of instance: qtimer1"]
    ALT10_QTIMER1_TIMER1 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_RX_DATA1_0 of instance: ecat"]
    ALT12_ECAT_PT0_RXD1 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB210MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB210MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB210MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB210MuxMode {
        SwMuxCtlPadGpioEmcB210MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB210MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB210MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB210MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB211MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA26 of instance: semc"]
    ALT0_SEMC_DATA26 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: NETC_PINMUX_ETH4_TXD03 of instance: netc_pinmux"]
    ALT1_NETC_PINMUX_ETH4_TXD3 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SPDIF_OUT of instance: spdif"]
    ALT2_SPDIF_OUT = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_PINMUX_ETH0_RX_DV of instance: netc_pinmux"]
    ALT3_NETC_PINMUX_ETH0_RX_DV = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPSPI5_PCS3 of instance: lpspi5"]
    ALT4_LPSPI5_PCS3 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO21 of instance: gpio3"]
    ALT5_GPIO3_IO21 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_XBAR_INOUT31 of instance: xbar1"]
    ALT6_XBAR1_XBAR_INOUT31 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: SAI3_RX_SYNC of instance: sai3"]
    ALT8_SAI3_RX_SYNC = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH3_RX_DV of instance: netc_pinmux"]
    ALT9_NETC_PINMUX_ETH3_RX_DV = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: QTIMER1_TIMER2 of instance: qtimer1"]
    ALT10_QTIMER1_TIMER2 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_RX_DV_0 of instance: ecat"]
    ALT12_ECAT_PT0_RX_DV = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB211MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB211MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB211MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB211MuxMode {
        SwMuxCtlPadGpioEmcB211MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB211MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB211MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB211MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB212MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA27 of instance: semc"]
    ALT0_SEMC_DATA27 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: NETC_PINMUX_ETH4_TXD02 of instance: netc_pinmux"]
    ALT1_NETC_PINMUX_ETH4_TXD2 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SPDIF_IN of instance: spdif"]
    ALT2_SPDIF_IN = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_PINMUX_ETH0_RX_ER of instance: netc_pinmux"]
    ALT3_NETC_PINMUX_ETH0_RX_ER = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPSPI5_PCS2 of instance: lpspi5"]
    ALT4_LPSPI5_PCS2 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO22 of instance: gpio3"]
    ALT5_GPIO3_IO22 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_XBAR_INOUT32 of instance: xbar1"]
    ALT6_XBAR1_XBAR_INOUT32 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: SAI3_RX_BCLK of instance: sai3"]
    ALT8_SAI3_RX_BCLK = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH3_RX_ER of instance: netc_pinmux"]
    ALT9_NETC_PINMUX_ETH3_RX_ER = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: QTIMER1_TIMER3 of instance: qtimer1"]
    ALT10_QTIMER1_TIMER3 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_PT0_RX_ER of instance: ecat"]
    ALT12_ECAT_PT0_RX_ER = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB212MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB212MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB212MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB212MuxMode {
        SwMuxCtlPadGpioEmcB212MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB212MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB212MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB212MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB213MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA28 of instance: semc"]
    ALT0_SEMC_DATA28 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: NETC_PINMUX_ETH4_TXD00 of instance: netc_pinmux"]
    ALT1_NETC_PINMUX_ETH4_TXD0 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART11_TX of instance: lpuart11"]
    ALT2_LPUART11_TX = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_PINMUX_ETH0_TXD03 of instance: netc_pinmux"]
    ALT3_NETC_PINMUX_ETH0_TXD3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPSPI5_PCS1 of instance: lpspi5"]
    ALT4_LPSPI5_PCS1 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO23 of instance: gpio3"]
    ALT5_GPIO3_IO23 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_XBAR_INOUT33 of instance: xbar1"]
    ALT6_XBAR1_XBAR_INOUT33 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: SAI3_RX_DATA of instance: sai3"]
    ALT8_SAI3_RX_DATA = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH3_TXD03 of instance: netc_pinmux"]
    ALT9_NETC_PINMUX_ETH3_TXD3 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: QTIMER2_TIMER0 of instance: qtimer2"]
    ALT10_QTIMER2_TIMER0 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_TX_DATA0_1 of instance: ecat"]
    ALT12_ECAT_PT1_TXD0 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB213MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB213MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB213MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB213MuxMode {
        SwMuxCtlPadGpioEmcB213MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB213MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB213MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB213MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB214MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA29 of instance: semc"]
    ALT0_SEMC_DATA29 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: NETC_PINMUX_ETH4_TXD01 of instance: netc_pinmux"]
    ALT1_NETC_PINMUX_ETH4_TXD1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART11_RX of instance: lpuart11"]
    ALT2_LPUART11_RX = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_PINMUX_ETH0_TXD02 of instance: netc_pinmux"]
    ALT3_NETC_PINMUX_ETH0_TXD2 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPUART5_DSR_B of instance: lpuart5"]
    ALT4_LPUART5_DSR_B = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO24 of instance: gpio3"]
    ALT5_GPIO3_IO24 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_XBAR_INOUT34 of instance: xbar1"]
    ALT6_XBAR1_XBAR_INOUT34 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: SAI3_TX_DATA of instance: sai3"]
    ALT8_SAI3_TX_DATA = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH3_TXD02 of instance: netc_pinmux"]
    ALT9_NETC_PINMUX_ETH3_TXD2 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: QTIMER2_TIMER1 of instance: qtimer2"]
    ALT10_QTIMER2_TIMER1 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_TX_DATA1_1 of instance: ecat"]
    ALT12_ECAT_PT1_TXD1 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB214MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB214MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB214MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB214MuxMode {
        SwMuxCtlPadGpioEmcB214MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB214MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB214MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB214MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB215MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA30 of instance: semc"]
    ALT0_SEMC_DATA30 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: NETC_PINMUX_ETH4_TX_EN of instance: netc_pinmux"]
    ALT1_NETC_PINMUX_ETH4_TX_EN = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART11_CTS_B of instance: lpuart11"]
    ALT2_LPUART11_CTS_B = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_PINMUX_ETH0_RX_CLK of instance: netc_pinmux"]
    ALT3_NETC_PINMUX_ETH0_RX_CLK = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPUART5_DCD_B of instance: lpuart5"]
    ALT4_LPUART5_DCD_B = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO25 of instance: gpio3"]
    ALT5_GPIO3_IO25 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_XBAR_INOUT35 of instance: xbar1"]
    ALT6_XBAR1_XBAR_INOUT35 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: SAI3_TX_BCLK of instance: sai3"]
    ALT8_SAI3_TX_BCLK = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH3_RX_CLK of instance: netc_pinmux"]
    ALT9_NETC_PINMUX_ETH3_RX_CLK = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: QTIMER2_TIMER2 of instance: qtimer2"]
    ALT10_QTIMER2_TIMER2 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_TX_EN_1 of instance: ecat"]
    ALT12_ECAT_PT1_TX_EN = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB215MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB215MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB215MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB215MuxMode {
        SwMuxCtlPadGpioEmcB215MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB215MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB215MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB215MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB216MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA31 of instance: semc"]
    ALT0_SEMC_DATA31 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: NETC_PINMUX_ETH4_TX_CLK of instance: netc_pinmux"]
    ALT1_NETC_PINMUX_ETH4_TX_CLK = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART11_RTS_B of instance: lpuart11"]
    ALT2_LPUART11_RTS_B = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_PINMUX_ETH0_RXD02 of instance: netc_pinmux"]
    ALT3_NETC_PINMUX_ETH0_RXD2 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPUART5_DTR_B of instance: lpuart5"]
    ALT4_LPUART5_DTR_B = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO26 of instance: gpio3"]
    ALT5_GPIO3_IO26 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_XBAR_INOUT14 of instance: xbar1"]
    ALT6_XBAR1_XBAR_INOUT14 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: SAI3_TX_SYNC of instance: sai3"]
    ALT8_SAI3_TX_SYNC = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH3_RXD02 of instance: netc_pinmux"]
    ALT9_NETC_PINMUX_ETH3_RXD2 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: QTIMER2_TIMER3 of instance: qtimer2"]
    ALT10_QTIMER2_TIMER3 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_TX_CLK_1 of instance: ecat"]
    ALT12_ECAT_PT1_TX_CLK = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB216MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB216MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB216MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB216MuxMode {
        SwMuxCtlPadGpioEmcB216MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB216MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB216MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB216MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB217MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DM03 of instance: semc"]
    ALT0_SEMC_DM3 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: NETC_PINMUX_ETH4_RXD00 of instance: netc_pinmux"]
    ALT1_NETC_PINMUX_ETH4_RXD0 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART5_TX of instance: lpuart5"]
    ALT2_LPUART5_TX = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_PINMUX_ETH0_RXD03 of instance: netc_pinmux"]
    ALT3_NETC_PINMUX_ETH0_RXD3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO27 of instance: gpio3"]
    ALT5_GPIO3_IO27 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_XBAR_INOUT15 of instance: xbar1"]
    ALT6_XBAR1_XBAR_INOUT15 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: SAI3_MCLK of instance: sai3"]
    ALT8_SAI3_MCLK = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH3_RXD03 of instance: netc_pinmux"]
    ALT9_NETC_PINMUX_ETH3_RXD3 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: QTIMER3_TIMER0 of instance: qtimer3"]
    ALT10_QTIMER3_TIMER0 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_RX_DATA0_1 of instance: ecat"]
    ALT12_ECAT_PT1_RXD0 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB217MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB217MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB217MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB217MuxMode {
        SwMuxCtlPadGpioEmcB217MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB217MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB217MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB217MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB218MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DQS4 of instance: semc"]
    ALT0_SEMC_DQS4 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: NETC_PINMUX_ETH4_RXD01 of instance: netc_pinmux"]
    ALT1_NETC_PINMUX_ETH4_RXD1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART5_RX of instance: lpuart5"]
    ALT2_LPUART5_RX = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_PINMUX_ETH0_TX_ER of instance: netc_pinmux"]
    ALT3_NETC_PINMUX_ETH0_TX_ER = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO28 of instance: gpio3"]
    ALT5_GPIO3_IO28 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_XBAR_INOUT16 of instance: xbar1"]
    ALT6_XBAR1_XBAR_INOUT16 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: EWM_EWM_OUT_B of instance: ewm"]
    ALT8_EWM_EWM_OUT_B = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH3_TX_ER of instance: netc_pinmux"]
    ALT9_NETC_PINMUX_ETH3_TX_ER = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: QTIMER3_TIMER1 of instance: qtimer3"]
    ALT10_QTIMER3_TIMER1 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_RX_DATA1_1 of instance: ecat"]
    ALT12_ECAT_PT1_RXD1 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB218MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB218MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB218MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB218MuxMode {
        SwMuxCtlPadGpioEmcB218MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB218MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB218MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB218MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB219MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_CLKX00 of instance: semc"]
    ALT0_SEMC_CLKX0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: NETC_PINMUX_ETH4_RX_DV of instance: netc_pinmux"]
    ALT1_NETC_PINMUX_ETH4_RX_DV = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART5_CTS_B of instance: lpuart5"]
    ALT2_LPUART5_CTS_B = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_ETH0_CRS of instance: netc"]
    ALT3_NETC_ETH0_CRS = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_EMDC of instance: netc"]
    ALT4_NETC_EMDC = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO29 of instance: gpio3"]
    ALT5_GPIO3_IO29 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_XBAR_INOUT36 of instance: xbar1"]
    ALT6_XBAR1_XBAR_INOUT36 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPI2C3_SCL of instance: lpi2c3"]
    ALT8_LPI2C3_SCL = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_ETH3_SLV_MDC of instance: netc"]
    ALT9_NETC_ETH3_SLV_MDC = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: QTIMER3_TIMER2 of instance: qtimer3"]
    ALT10_QTIMER3_TIMER2 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_RX_DV_1 of instance: ecat"]
    ALT12_ECAT_PT1_RX_DV = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB219MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB219MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB219MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB219MuxMode {
        SwMuxCtlPadGpioEmcB219MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB219MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB219MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB219MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioEmcB220MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SEMC_CLKX01 of instance: semc"]
    ALT0_SEMC_CLKX1 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: NETC_PINMUX_ETH4_RX_ER of instance: netc_pinmux"]
    ALT1_NETC_PINMUX_ETH4_RX_ER = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART5_RTS_B of instance: lpuart5"]
    ALT2_LPUART5_RTS_B = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_ETH0_COL of instance: netc"]
    ALT3_NETC_ETH0_COL = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_EMDIO of instance: netc"]
    ALT4_NETC_EMDIO = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO30 of instance: gpio3"]
    ALT5_GPIO3_IO30 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_XBAR_INOUT37 of instance: xbar1"]
    ALT6_XBAR1_XBAR_INOUT37 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPI2C3_SDA of instance: lpi2c3"]
    ALT8_LPI2C3_SDA = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_ETH3_SLV_MDIO of instance: netc"]
    ALT9_NETC_ETH3_SLV_MDIO = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: QTIMER3_TIMER3 of instance: qtimer3"]
    ALT10_QTIMER3_TIMER3 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_RX_ER_1 of instance: ecat"]
    ALT12_ECAT_PT1_RX_ER = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioEmcB220MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioEmcB220MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioEmcB220MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioEmcB220MuxMode {
        SwMuxCtlPadGpioEmcB220MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioEmcB220MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioEmcB220MuxMode) -> u8 {
        SwMuxCtlPadGpioEmcB220MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSdB100MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: USDHC1_CMD of instance: usdhc1"]
    ALT0_USDHC1_CMD = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: SINC1_EMCLK02 of instance: sinc1"]
    ALT1_SINC1_EMCLK2 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XBAR1_XBAR_INOUT20 of instance: xbar1"]
    ALT2_XBAR1_XBAR_INOUT20 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPTMR2_ALT1 of instance: lptmr2"]
    ALT3_LPTMR2_ALT1 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: XSPI_SLV_CS of instance: xspi_slv"]
    ALT4_XSPI_SLV_CS = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO04 of instance: gpio5"]
    ALT5_GPIO5_IO4 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPSPI3_PCS0 of instance: lpspi3"]
    ALT6_LPSPI3_PCS0 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: KPP_ROW07 of instance: kpp"]
    ALT8_KPP_ROW7 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: CCM_CLKO1 of instance: ccm"]
    ALT12_CCM_CLKO1 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioSdB100MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSdB100MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSdB100MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSdB100MuxMode {
        SwMuxCtlPadGpioSdB100MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSdB100MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSdB100MuxMode) -> u8 {
        SwMuxCtlPadGpioSdB100MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSdB101MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: USDHC1_CLK of instance: usdhc1"]
    ALT0_USDHC1_CLK = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: SINC1_EMBIT02 of instance: sinc1"]
    ALT1_SINC1_EMBIT2 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XBAR1_XBAR_INOUT21 of instance: xbar1"]
    ALT2_XBAR1_XBAR_INOUT21 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPTMR2_ALT2 of instance: lptmr2"]
    ALT3_LPTMR2_ALT2 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: XSPI_SLV_CLK of instance: xspi_slv"]
    ALT4_XSPI_SLV_CLK = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO05 of instance: gpio5"]
    ALT5_GPIO5_IO5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPSPI3_SCK of instance: lpspi3"]
    ALT6_LPSPI3_SCK = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: KPP_COL07 of instance: kpp"]
    ALT8_KPP_COL7 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: CCM_CLKO2 of instance: ccm"]
    ALT12_CCM_CLKO2 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioSdB101MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSdB101MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSdB101MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSdB101MuxMode {
        SwMuxCtlPadGpioSdB101MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSdB101MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSdB101MuxMode) -> u8 {
        SwMuxCtlPadGpioSdB101MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSdB102MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: USDHC1_DATA0 of instance: usdhc1"]
    ALT0_USDHC1_DATA0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: SINC1_EMCLK03 of instance: sinc1"]
    ALT1_SINC1_EMCLK3 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XBAR1_XBAR_INOUT22 of instance: xbar1"]
    ALT2_XBAR1_XBAR_INOUT22 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPTMR2_ALT3 of instance: lptmr2"]
    ALT3_LPTMR2_ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: XSPI_SLV_DATA04 of instance: xspi_slv"]
    ALT4_XSPI_SLV_DATA4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO06 of instance: gpio5"]
    ALT5_GPIO5_IO6 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPSPI3_SDO of instance: lpspi3"]
    ALT6_LPSPI3_SDO = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: KPP_ROW06 of instance: kpp"]
    ALT8_KPP_ROW6 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: FLEXSPI1_BUS2BIT_A_SS1_B of instance: flexspi1_bus2bit"]
    ALT9_FLEXSPI1_BUS2BIT_A_SS1_B = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_RESET_OUT of instance: ecat"]
    ALT12_ECAT_RESET_OUT = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioSdB102MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSdB102MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSdB102MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSdB102MuxMode {
        SwMuxCtlPadGpioSdB102MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSdB102MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSdB102MuxMode) -> u8 {
        SwMuxCtlPadGpioSdB102MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSdB103MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: USDHC1_DATA1 of instance: usdhc1"]
    ALT0_USDHC1_DATA1 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: SINC1_EMBIT03 of instance: sinc1"]
    ALT1_SINC1_EMBIT3 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XBAR1_XBAR_INOUT23 of instance: xbar1"]
    ALT2_XBAR1_XBAR_INOUT23 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPTMR3_ALT1 of instance: lptmr3"]
    ALT3_LPTMR3_ALT1 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: XSPI_SLV_DATA05 of instance: xspi_slv"]
    ALT4_XSPI_SLV_DATA5 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO07 of instance: gpio5"]
    ALT5_GPIO5_IO7 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPSPI3_SDI of instance: lpspi3"]
    ALT6_LPSPI3_SDI = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: KPP_COL06 of instance: kpp"]
    ALT8_KPP_COL6 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: FLEXSPI1_BUS2BIT_B_SS1_B of instance: flexspi1_bus2bit"]
    ALT9_FLEXSPI1_BUS2BIT_B_SS1_B = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioSdB103MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSdB103MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSdB103MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSdB103MuxMode {
        SwMuxCtlPadGpioSdB103MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSdB103MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSdB103MuxMode) -> u8 {
        SwMuxCtlPadGpioSdB103MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSdB104MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: USDHC1_DATA2 of instance: usdhc1"]
    ALT0_USDHC1_DATA2 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: SINC_FILTER_GLUE1_BREAK of instance: sinc_filter_glue1"]
    ALT1_SINC_FILTER_GLUE1_BREAK = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SINC2_EMCLK02 of instance: sinc2"]
    ALT2_SINC2_EMCLK2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPTMR3_ALT2 of instance: lptmr3"]
    ALT3_LPTMR3_ALT2 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: XSPI_SLV_DATA06 of instance: xspi_slv"]
    ALT4_XSPI_SLV_DATA6 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO08 of instance: gpio5"]
    ALT5_GPIO5_IO8 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPSPI3_PCS1 of instance: lpspi3"]
    ALT6_LPSPI3_PCS1 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXSPI1_BUS2BIT_B_SS0_B of instance: flexspi1_bus2bit"]
    ALT8_FLEXSPI1_BUS2BIT_B_SS0_B = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: FLEXSPI1_BUS2BIT_A_SS1_B of instance: flexspi1_bus2bit"]
    ALT9_FLEXSPI1_BUS2BIT_A_SS1_B = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioSdB104MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSdB104MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSdB104MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSdB104MuxMode {
        SwMuxCtlPadGpioSdB104MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSdB104MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSdB104MuxMode) -> u8 {
        SwMuxCtlPadGpioSdB104MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSdB105MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: USDHC1_DATA3 of instance: usdhc1"]
    ALT0_USDHC1_DATA3 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SINC2_EMBIT02 of instance: sinc2"]
    ALT2_SINC2_EMBIT2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPTMR3_ALT3 of instance: lptmr3"]
    ALT3_LPTMR3_ALT3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: XSPI_SLV_DATA07 of instance: xspi_slv"]
    ALT4_XSPI_SLV_DATA7 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO09 of instance: gpio5"]
    ALT5_GPIO5_IO9 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPSPI3_PCS2 of instance: lpspi3"]
    ALT6_LPSPI3_PCS2 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: FLEXSPI1_BUS2BIT_B_SS0_B of instance: flexspi1_bus2bit"]
    ALT9_FLEXSPI1_BUS2BIT_B_SS0_B = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioSdB105MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSdB105MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSdB105MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSdB105MuxMode {
        SwMuxCtlPadGpioSdB105MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSdB105MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSdB105MuxMode) -> u8 {
        SwMuxCtlPadGpioSdB105MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSdB200MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: USDHC2_DATA3 of instance: usdhc2"]
    ALT0_USDHC2_DATA3 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXSPI1_BUS2BIT_B_DATA04 of instance: flexspi1_bus2bit"]
    ALT1_FLEXSPI1_BUS2BIT_B_DATA4 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XSPI_SLV_DATA04 of instance: xspi_slv"]
    ALT2_XSPI_SLV_DATA4 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: XBAR1_XBAR_INOUT17 of instance: xbar1"]
    ALT3_XBAR1_XBAR_INOUT17 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: KPP_ROW01 of instance: kpp"]
    ALT4_KPP_ROW1 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO10 of instance: gpio5"]
    ALT5_GPIO5_IO10 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPSPI3_PCS3 of instance: lpspi3"]
    ALT6_LPSPI3_PCS3 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_1588_CLK of instance: netc_clkgen"]
    ALT8_NETC_CLKGEN_TMR_1588_CLK = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPUART8_TX of instance: lpuart8"]
    ALT9_LPUART8_TX = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: MIC_BITSTREAM00 of instance: mic"]
    ALT12_MIC_BITSTREAM0 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioSdB200MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSdB200MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSdB200MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSdB200MuxMode {
        SwMuxCtlPadGpioSdB200MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSdB200MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSdB200MuxMode) -> u8 {
        SwMuxCtlPadGpioSdB200MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSdB201MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: USDHC2_DATA2 of instance: usdhc2"]
    ALT0_USDHC2_DATA2 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXSPI1_BUS2BIT_B_DATA05 of instance: flexspi1_bus2bit"]
    ALT1_FLEXSPI1_BUS2BIT_B_DATA5 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XSPI_SLV_DATA05 of instance: xspi_slv"]
    ALT2_XSPI_SLV_DATA5 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER6_TIMER0 of instance: qtimer6"]
    ALT3_QTIMER6_TIMER0 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: KPP_COL01 of instance: kpp"]
    ALT4_KPP_COL1 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO11 of instance: gpio5"]
    ALT5_GPIO5_IO11 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_TMR_GCLK of instance: netc"]
    ALT8_NETC_TMR_1588_GCLK = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPUART8_RX of instance: lpuart8"]
    ALT9_LPUART8_RX = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: MIC_BITSTREAM01 of instance: mic"]
    ALT12_MIC_BITSTREAM1 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioSdB201MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSdB201MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSdB201MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSdB201MuxMode {
        SwMuxCtlPadGpioSdB201MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSdB201MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSdB201MuxMode) -> u8 {
        SwMuxCtlPadGpioSdB201MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSdB202MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: USDHC2_DATA1 of instance: usdhc2"]
    ALT0_USDHC2_DATA1 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXSPI1_BUS2BIT_B_DATA06 of instance: flexspi1_bus2bit"]
    ALT1_FLEXSPI1_BUS2BIT_B_DATA6 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XSPI_SLV_DATA06 of instance: xspi_slv"]
    ALT2_XSPI_SLV_DATA6 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER6_TIMER1 of instance: qtimer6"]
    ALT3_QTIMER6_TIMER1 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: KPP_ROW00 of instance: kpp"]
    ALT4_KPP_ROW0 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO12 of instance: gpio5"]
    ALT5_GPIO5_IO12 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_TMR_ALARM1 of instance: netc"]
    ALT8_NETC_TMR_1588_ALARM1 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPUART8_CTS_B of instance: lpuart8"]
    ALT9_LPUART8_CTS_B = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: MIC_BITSTREAM02 of instance: mic"]
    ALT12_MIC_BITSTREAM2 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioSdB202MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSdB202MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSdB202MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSdB202MuxMode {
        SwMuxCtlPadGpioSdB202MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSdB202MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSdB202MuxMode) -> u8 {
        SwMuxCtlPadGpioSdB202MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSdB203MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: USDHC2_DATA0 of instance: usdhc2"]
    ALT0_USDHC2_DATA0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXSPI1_BUS2BIT_B_DATA07 of instance: flexspi1_bus2bit"]
    ALT1_FLEXSPI1_BUS2BIT_B_DATA7 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XSPI_SLV_DATA07 of instance: xspi_slv"]
    ALT2_XSPI_SLV_DATA7 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER6_TIMER2 of instance: qtimer6"]
    ALT3_QTIMER6_TIMER2 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: KPP_COL00 of instance: kpp"]
    ALT4_KPP_COL0 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO13 of instance: gpio5"]
    ALT5_GPIO5_IO13 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_TMR_ALARM2 of instance: netc"]
    ALT8_NETC_TMR_1588_ALARM2 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPUART8_RTS_B of instance: lpuart8"]
    ALT9_LPUART8_RTS_B = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: MIC_BITSTREAM03 of instance: mic"]
    ALT12_MIC_BITSTREAM3 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioSdB203MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSdB203MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSdB203MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSdB203MuxMode {
        SwMuxCtlPadGpioSdB203MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSdB203MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSdB203MuxMode) -> u8 {
        SwMuxCtlPadGpioSdB203MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSdB204MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: USDHC2_CLK of instance: usdhc2"]
    ALT0_USDHC2_CLK = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXSPI1_BUS2BIT_B_SS1_B of instance: flexspi1_bus2bit"]
    ALT1_FLEXSPI1_BUS2BIT_B_SS1_B = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER7_TIMER0 of instance: qtimer7"]
    ALT3_QTIMER7_TIMER0 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: KPP_ROW03 of instance: kpp"]
    ALT4_KPP_ROW3 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO14 of instance: gpio5"]
    ALT5_GPIO5_IO14 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPUART5_RI_B of instance: lpuart5"]
    ALT6_LPUART5_RI_B = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_TMR_PP1 of instance: netc"]
    ALT8_NETC_TMR_1588_PP1 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: MIC_CLK of instance: mic"]
    ALT12_MIC_CLK = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioSdB204MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSdB204MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSdB204MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSdB204MuxMode {
        SwMuxCtlPadGpioSdB204MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSdB204MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSdB204MuxMode) -> u8 {
        SwMuxCtlPadGpioSdB204MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSdB205MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: USDHC2_CMD of instance: usdhc2"]
    ALT0_USDHC2_CMD = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXSPI1_BUS2BIT_B_DQS of instance: flexspi1_bus2bit"]
    ALT1_FLEXSPI1_BUS2BIT_B_DQS = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XSPI_SLV_DQS of instance: xspi_slv"]
    ALT2_XSPI_SLV_DQS = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER7_TIMER1 of instance: qtimer7"]
    ALT3_QTIMER7_TIMER1 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPSPI4_PCS3 of instance: lpspi4"]
    ALT4_LPSPI4_PCS3 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO15 of instance: gpio5"]
    ALT5_GPIO5_IO15 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPUART5_DTR_B of instance: lpuart5"]
    ALT6_LPUART5_DTR_B = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_TMR_PP2 of instance: netc"]
    ALT8_NETC_TMR_1588_PP2 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioSdB205MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSdB205MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSdB205MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSdB205MuxMode {
        SwMuxCtlPadGpioSdB205MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSdB205MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSdB205MuxMode) -> u8 {
        SwMuxCtlPadGpioSdB205MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSdB206MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: USDHC2_RESET_B of instance: usdhc2"]
    ALT0_USDHC2_RESET_B = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXSPI1_BUS2BIT_B_SS0_B of instance: flexspi1_bus2bit"]
    ALT1_FLEXSPI1_BUS2BIT_B_SS0_B = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XSPI_SLV_CS of instance: xspi_slv"]
    ALT2_XSPI_SLV_CS = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER7_TIMER2 of instance: qtimer7"]
    ALT3_QTIMER7_TIMER2 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPSPI4_PCS2 of instance: lpspi4"]
    ALT4_LPSPI4_PCS2 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO16 of instance: gpio5"]
    ALT5_GPIO5_IO16 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPUART5_CTS_B of instance: lpuart5"]
    ALT6_LPUART5_CTS_B = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_TMR_PP3 of instance: netc"]
    ALT8_NETC_TMR_1588_PP3 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioSdB206MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSdB206MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSdB206MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSdB206MuxMode {
        SwMuxCtlPadGpioSdB206MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSdB206MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSdB206MuxMode) -> u8 {
        SwMuxCtlPadGpioSdB206MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSdB207MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: USDHC2_STROBE of instance: usdhc2"]
    ALT0_USDHC2_STROBE = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXSPI1_BUS2BIT_B_SCLK of instance: flexspi1_bus2bit"]
    ALT1_FLEXSPI1_BUS2BIT_B_SCLK = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XSPI_SLV_CLK of instance: xspi_slv"]
    ALT2_XSPI_SLV_CLK = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER7_TIMER3 of instance: qtimer7"]
    ALT3_QTIMER7_TIMER3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPSPI4_PCS1 of instance: lpspi4"]
    ALT4_LPSPI4_PCS1 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO17 of instance: gpio5"]
    ALT5_GPIO5_IO17 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPUART5_RTS_B of instance: lpuart5"]
    ALT6_LPUART5_RTS_B = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_TMR_ALARM1 of instance: netc"]
    ALT8_NETC_TMR_1588_ALARM1 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioSdB207MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSdB207MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSdB207MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSdB207MuxMode {
        SwMuxCtlPadGpioSdB207MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSdB207MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSdB207MuxMode) -> u8 {
        SwMuxCtlPadGpioSdB207MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSdB208MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: USDHC2_DATA4 of instance: usdhc2"]
    ALT0_USDHC2_DATA4 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXSPI1_BUS2BIT_B_DATA00 of instance: flexspi1_bus2bit"]
    ALT1_FLEXSPI1_BUS2BIT_B_DATA0 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XSPI_SLV_DATA00 of instance: xspi_slv"]
    ALT2_XSPI_SLV_DATA0 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER8_TIMER0 of instance: qtimer8"]
    ALT3_QTIMER8_TIMER0 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPSPI4_SCK of instance: lpspi4"]
    ALT4_LPSPI4_SCK = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO18 of instance: gpio5"]
    ALT5_GPIO5_IO18 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPUART5_TX of instance: lpuart5"]
    ALT6_LPUART5_TX = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_TMR_ALARM2 of instance: netc"]
    ALT8_NETC_TMR_1588_ALARM2 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_TMR_PP2 of instance: netc"]
    ALT9_NETC_TMR_1588_PP2 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioSdB208MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSdB208MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSdB208MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSdB208MuxMode {
        SwMuxCtlPadGpioSdB208MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSdB208MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSdB208MuxMode) -> u8 {
        SwMuxCtlPadGpioSdB208MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSdB209MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: USDHC2_DATA5 of instance: usdhc2"]
    ALT0_USDHC2_DATA5 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXSPI1_BUS2BIT_B_DATA01 of instance: flexspi1_bus2bit"]
    ALT1_FLEXSPI1_BUS2BIT_B_DATA1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XSPI_SLV_DATA01 of instance: xspi_slv"]
    ALT2_XSPI_SLV_DATA1 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER8_TIMER1 of instance: qtimer8"]
    ALT3_QTIMER8_TIMER1 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPSPI4_PCS0 of instance: lpspi4"]
    ALT4_LPSPI4_PCS0 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO19 of instance: gpio5"]
    ALT5_GPIO5_IO19 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPUART5_RX of instance: lpuart5"]
    ALT6_LPUART5_RX = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_TMR_PP1 of instance: netc"]
    ALT9_NETC_TMR_1588_PP1 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioSdB209MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSdB209MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSdB209MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSdB209MuxMode {
        SwMuxCtlPadGpioSdB209MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSdB209MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSdB209MuxMode) -> u8 {
        SwMuxCtlPadGpioSdB209MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSdB210MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: USDHC2_DATA6 of instance: usdhc2"]
    ALT0_USDHC2_DATA6 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXSPI1_BUS2BIT_B_DATA02 of instance: flexspi1_bus2bit"]
    ALT1_FLEXSPI1_BUS2BIT_B_DATA2 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XSPI_SLV_DATA02 of instance: xspi_slv"]
    ALT2_XSPI_SLV_DATA2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER8_TIMER2 of instance: qtimer8"]
    ALT3_QTIMER8_TIMER2 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPSPI4_SDO of instance: lpspi4"]
    ALT4_LPSPI4_SDO = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO20 of instance: gpio5"]
    ALT5_GPIO5_IO20 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPUART5_DCD_B of instance: lpuart5"]
    ALT6_LPUART5_DCD_B = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_TMR_TRIG2 of instance: netc"]
    ALT8_NETC_TMR_1588_TRIG2 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_TMR_PP3 of instance: netc"]
    ALT9_NETC_TMR_1588_PP3 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: NETC_EMDIO of instance: netc"]
    ALT10_NETC_EMDIO = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_MDIO of instance: ecat"]
    ALT12_ECAT_MDIO = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioSdB210MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSdB210MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSdB210MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSdB210MuxMode {
        SwMuxCtlPadGpioSdB210MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSdB210MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSdB210MuxMode) -> u8 {
        SwMuxCtlPadGpioSdB210MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSdB211MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: USDHC2_DATA7 of instance: usdhc2"]
    ALT0_USDHC2_DATA7 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXSPI1_BUS2BIT_B_DATA03 of instance: flexspi1_bus2bit"]
    ALT1_FLEXSPI1_BUS2BIT_B_DATA3 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XSPI_SLV_DATA03 of instance: xspi_slv"]
    ALT2_XSPI_SLV_DATA3 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER8_TIMER3 of instance: qtimer8"]
    ALT3_QTIMER8_TIMER3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPSPI4_SDI of instance: lpspi4"]
    ALT4_LPSPI4_SDI = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO21 of instance: gpio5"]
    ALT5_GPIO5_IO21 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPUART5_DSR_B of instance: lpuart5"]
    ALT6_LPUART5_DSR_B = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: SFA_ATX_CLK_OUT of instance: sfa"]
    ALT7_SFA_ATX_CLK_OUT = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_TMR_TRIG1 of instance: netc"]
    ALT8_NETC_TMR_1588_TRIG1 = 0x08,
    _RESERVED_9 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: NETC_EMDC of instance: netc"]
    ALT10_NETC_EMDC = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_MCLK of instance: ecat"]
    ALT12_ECAT_MDC = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SwMuxCtlPadGpioSdB211MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSdB211MuxMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSdB211MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSdB211MuxMode {
        SwMuxCtlPadGpioSdB211MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSdB211MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSdB211MuxMode) -> u8 {
        SwMuxCtlPadGpioSdB211MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwMuxCtlPadGpioSdB212MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: FLEXSPI1_BUS2BIT_A_DQS of instance: flexspi1_bus2bit"]
    ALT0_FLEXSPI1_BUS2BIT_A_DQS = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXSPI1_BUS2BIT_B_DQS of instance: flexspi1_bus2bit"]
    ALT1_FLEXSPI1_BUS2BIT_B_DQS = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO22 of instance: gpio5"]
    ALT5_GPIO5_IO22 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SwMuxCtlPadGpioSdB212MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwMuxCtlPadGpioSdB212MuxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwMuxCtlPadGpioSdB212MuxMode {
    #[inline(always)]
    fn from(val: u8) -> SwMuxCtlPadGpioSdB212MuxMode {
        SwMuxCtlPadGpioSdB212MuxMode::from_bits(val)
    }
}
impl From<SwMuxCtlPadGpioSdB212MuxMode> for u8 {
    #[inline(always)]
    fn from(val: SwMuxCtlPadGpioSdB212MuxMode) -> u8 {
        SwMuxCtlPadGpioSdB212MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbIppIndOtg2OcSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_06 for Mode: ALT0"]
    SELECT_GPIO_AD_06_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_30 for Mode: ALT1"]
    SELECT_GPIO_AD_30_ALT1 = 0x01,
}
impl UsbIppIndOtg2OcSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UsbIppIndOtg2OcSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UsbIppIndOtg2OcSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> UsbIppIndOtg2OcSelectInputDaisy {
        UsbIppIndOtg2OcSelectInputDaisy::from_bits(val)
    }
}
impl From<UsbIppIndOtg2OcSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: UsbIppIndOtg2OcSelectInputDaisy) -> u8 {
        UsbIppIndOtg2OcSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbIppIndOtgOcSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_11 for Mode: ALT0"]
    SELECT_GPIO_AD_11_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_35 for Mode: ALT1"]
    SELECT_GPIO_AD_35_ALT1 = 0x01,
}
impl UsbIppIndOtgOcSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UsbIppIndOtgOcSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UsbIppIndOtgOcSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> UsbIppIndOtgOcSelectInputDaisy {
        UsbIppIndOtgOcSelectInputDaisy::from_bits(val)
    }
}
impl From<UsbIppIndOtgOcSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: UsbIppIndOtgOcSelectInputDaisy) -> u8 {
        UsbIppIndOtgOcSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usbphy1UsbIdSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_09 for Mode: ALT0"]
    SELECT_GPIO_AD_09_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_33 for Mode: ALT1"]
    SELECT_GPIO_AD_33_ALT1 = 0x01,
}
impl Usbphy1UsbIdSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usbphy1UsbIdSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usbphy1UsbIdSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Usbphy1UsbIdSelectInputDaisy {
        Usbphy1UsbIdSelectInputDaisy::from_bits(val)
    }
}
impl From<Usbphy1UsbIdSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Usbphy1UsbIdSelectInputDaisy) -> u8 {
        Usbphy1UsbIdSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usbphy2UsbIdSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_08 for Mode: ALT0"]
    SELECT_GPIO_AD_08_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_32 for Mode: ALT1"]
    SELECT_GPIO_AD_32_ALT1 = 0x01,
}
impl Usbphy2UsbIdSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usbphy2UsbIdSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usbphy2UsbIdSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Usbphy2UsbIdSelectInputDaisy {
        Usbphy2UsbIdSelectInputDaisy::from_bits(val)
    }
}
impl From<Usbphy2UsbIdSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Usbphy2UsbIdSelectInputDaisy) -> u8 {
        Usbphy2UsbIdSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usdhc1IppCardDetSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_32 for Mode: ALT4"]
    SELECT_GPIO_AD_32_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_08 for Mode: ALT2"]
    SELECT_GPIO_B1_08_ALT2 = 0x01,
}
impl Usdhc1IppCardDetSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usdhc1IppCardDetSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usdhc1IppCardDetSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Usdhc1IppCardDetSelectInputDaisy {
        Usdhc1IppCardDetSelectInputDaisy::from_bits(val)
    }
}
impl From<Usdhc1IppCardDetSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Usdhc1IppCardDetSelectInputDaisy) -> u8 {
        Usdhc1IppCardDetSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usdhc1IppWpOnSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_33 for Mode: ALT4"]
    SELECT_GPIO_AD_33_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_09 for Mode: ALT2"]
    SELECT_GPIO_B1_09_ALT2 = 0x01,
}
impl Usdhc1IppWpOnSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usdhc1IppWpOnSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usdhc1IppWpOnSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Usdhc1IppWpOnSelectInputDaisy {
        Usdhc1IppWpOnSelectInputDaisy::from_bits(val)
    }
}
impl From<Usdhc1IppWpOnSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Usdhc1IppWpOnSelectInputDaisy) -> u8 {
        Usdhc1IppWpOnSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usdhc2IppCardDetSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_01 for Mode: ALT1"]
    SELECT_GPIO_EMC_B2_01_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_13 for Mode: ALT7"]
    SELECT_GPIO_AD_13_ALT7 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_26 for Mode: ALT9"]
    SELECT_GPIO_AD_26_ALT9 = 0x02,
    #[doc = "Selecting Pad: GPIO_AD_29 for Mode: ALT2"]
    SELECT_GPIO_AD_29_ALT2 = 0x03,
}
impl Usdhc2IppCardDetSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usdhc2IppCardDetSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usdhc2IppCardDetSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Usdhc2IppCardDetSelectInputDaisy {
        Usdhc2IppCardDetSelectInputDaisy::from_bits(val)
    }
}
impl From<Usdhc2IppCardDetSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Usdhc2IppCardDetSelectInputDaisy) -> u8 {
        Usdhc2IppCardDetSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usdhc2IppWpOnSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_02 for Mode: ALT1"]
    SELECT_GPIO_EMC_B2_02_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_14 for Mode: ALT7"]
    SELECT_GPIO_AD_14_ALT7 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_27 for Mode: ALT9"]
    SELECT_GPIO_AD_27_ALT9 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Usdhc2IppWpOnSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usdhc2IppWpOnSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usdhc2IppWpOnSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Usdhc2IppWpOnSelectInputDaisy {
        Usdhc2IppWpOnSelectInputDaisy::from_bits(val)
    }
}
impl From<Usdhc2IppWpOnSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Usdhc2IppWpOnSelectInputDaisy) -> u8 {
        Usdhc2IppWpOnSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xbar1XbarInSelectInput14Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_30 for Mode: ALT2"]
    SELECT_GPIO_EMC_B1_30_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_16 for Mode: ALT6"]
    SELECT_GPIO_EMC_B2_16_ALT6 = 0x01,
}
impl Xbar1XbarInSelectInput14Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xbar1XbarInSelectInput14Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xbar1XbarInSelectInput14Daisy {
    #[inline(always)]
    fn from(val: u8) -> Xbar1XbarInSelectInput14Daisy {
        Xbar1XbarInSelectInput14Daisy::from_bits(val)
    }
}
impl From<Xbar1XbarInSelectInput14Daisy> for u8 {
    #[inline(always)]
    fn from(val: Xbar1XbarInSelectInput14Daisy) -> u8 {
        Xbar1XbarInSelectInput14Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xbar1XbarInSelectInput15Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B1_39 for Mode: ALT2"]
    SELECT_GPIO_EMC_B1_39_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_17 for Mode: ALT6"]
    SELECT_GPIO_EMC_B2_17_ALT6 = 0x01,
}
impl Xbar1XbarInSelectInput15Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xbar1XbarInSelectInput15Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xbar1XbarInSelectInput15Daisy {
    #[inline(always)]
    fn from(val: u8) -> Xbar1XbarInSelectInput15Daisy {
        Xbar1XbarInSelectInput15Daisy::from_bits(val)
    }
}
impl From<Xbar1XbarInSelectInput15Daisy> for u8 {
    #[inline(always)]
    fn from(val: Xbar1XbarInSelectInput15Daisy) -> u8 {
        Xbar1XbarInSelectInput15Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xbar1XbarInSelectInput17Daisy {
    #[doc = "Selecting Pad: GPIO_AD_33 for Mode: ALT2"]
    SELECT_GPIO_AD_33_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_00 for Mode: ALT3"]
    SELECT_GPIO_SD_B2_00_ALT3 = 0x01,
}
impl Xbar1XbarInSelectInput17Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xbar1XbarInSelectInput17Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xbar1XbarInSelectInput17Daisy {
    #[inline(always)]
    fn from(val: u8) -> Xbar1XbarInSelectInput17Daisy {
        Xbar1XbarInSelectInput17Daisy::from_bits(val)
    }
}
impl From<Xbar1XbarInSelectInput17Daisy> for u8 {
    #[inline(always)]
    fn from(val: Xbar1XbarInSelectInput17Daisy) -> u8 {
        Xbar1XbarInSelectInput17Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xbar1XbarInSelectInput18Daisy {
    #[doc = "Selecting Pad: GPIO_AD_12 for Mode: ALT6"]
    SELECT_GPIO_AD_12_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_34 for Mode: ALT2"]
    SELECT_GPIO_AD_34_ALT2 = 0x01,
}
impl Xbar1XbarInSelectInput18Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xbar1XbarInSelectInput18Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xbar1XbarInSelectInput18Daisy {
    #[inline(always)]
    fn from(val: u8) -> Xbar1XbarInSelectInput18Daisy {
        Xbar1XbarInSelectInput18Daisy::from_bits(val)
    }
}
impl From<Xbar1XbarInSelectInput18Daisy> for u8 {
    #[inline(always)]
    fn from(val: Xbar1XbarInSelectInput18Daisy) -> u8 {
        Xbar1XbarInSelectInput18Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xbar1XbarInSelectInput19Daisy {
    #[doc = "Selecting Pad: GPIO_AD_19 for Mode: ALT2"]
    SELECT_GPIO_AD_19_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_35 for Mode: ALT2"]
    SELECT_GPIO_AD_35_ALT2 = 0x01,
}
impl Xbar1XbarInSelectInput19Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xbar1XbarInSelectInput19Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xbar1XbarInSelectInput19Daisy {
    #[inline(always)]
    fn from(val: u8) -> Xbar1XbarInSelectInput19Daisy {
        Xbar1XbarInSelectInput19Daisy::from_bits(val)
    }
}
impl From<Xbar1XbarInSelectInput19Daisy> for u8 {
    #[inline(always)]
    fn from(val: Xbar1XbarInSelectInput19Daisy) -> u8 {
        Xbar1XbarInSelectInput19Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xbar1XbarInSelectInput20Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_00 for Mode: ALT6"]
    SELECT_GPIO_EMC_B2_00_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B1_00 for Mode: ALT2"]
    SELECT_GPIO_SD_B1_00_ALT2 = 0x01,
}
impl Xbar1XbarInSelectInput20Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xbar1XbarInSelectInput20Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xbar1XbarInSelectInput20Daisy {
    #[inline(always)]
    fn from(val: u8) -> Xbar1XbarInSelectInput20Daisy {
        Xbar1XbarInSelectInput20Daisy::from_bits(val)
    }
}
impl From<Xbar1XbarInSelectInput20Daisy> for u8 {
    #[inline(always)]
    fn from(val: Xbar1XbarInSelectInput20Daisy) -> u8 {
        Xbar1XbarInSelectInput20Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xbar1XbarInSelectInput21Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_01 for Mode: ALT6"]
    SELECT_GPIO_EMC_B2_01_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B1_01 for Mode: ALT2"]
    SELECT_GPIO_SD_B1_01_ALT2 = 0x01,
}
impl Xbar1XbarInSelectInput21Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xbar1XbarInSelectInput21Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xbar1XbarInSelectInput21Daisy {
    #[inline(always)]
    fn from(val: u8) -> Xbar1XbarInSelectInput21Daisy {
        Xbar1XbarInSelectInput21Daisy::from_bits(val)
    }
}
impl From<Xbar1XbarInSelectInput21Daisy> for u8 {
    #[inline(always)]
    fn from(val: Xbar1XbarInSelectInput21Daisy) -> u8 {
        Xbar1XbarInSelectInput21Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xbar1XbarInSelectInput22Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_02 for Mode: ALT6"]
    SELECT_GPIO_EMC_B2_02_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B1_02 for Mode: ALT2"]
    SELECT_GPIO_SD_B1_02_ALT2 = 0x01,
}
impl Xbar1XbarInSelectInput22Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xbar1XbarInSelectInput22Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xbar1XbarInSelectInput22Daisy {
    #[inline(always)]
    fn from(val: u8) -> Xbar1XbarInSelectInput22Daisy {
        Xbar1XbarInSelectInput22Daisy::from_bits(val)
    }
}
impl From<Xbar1XbarInSelectInput22Daisy> for u8 {
    #[inline(always)]
    fn from(val: Xbar1XbarInSelectInput22Daisy) -> u8 {
        Xbar1XbarInSelectInput22Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xbar1XbarInSelectInput23Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_03 for Mode: ALT6"]
    SELECT_GPIO_EMC_B2_03_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B1_03 for Mode: ALT2"]
    SELECT_GPIO_SD_B1_03_ALT2 = 0x01,
}
impl Xbar1XbarInSelectInput23Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xbar1XbarInSelectInput23Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xbar1XbarInSelectInput23Daisy {
    #[inline(always)]
    fn from(val: u8) -> Xbar1XbarInSelectInput23Daisy {
        Xbar1XbarInSelectInput23Daisy::from_bits(val)
    }
}
impl From<Xbar1XbarInSelectInput23Daisy> for u8 {
    #[inline(always)]
    fn from(val: Xbar1XbarInSelectInput23Daisy) -> u8 {
        Xbar1XbarInSelectInput23Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xbar1XbarInSelectInput24Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_04 for Mode: ALT6"]
    SELECT_GPIO_EMC_B2_04_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_30 for Mode: ALT9"]
    SELECT_GPIO_AD_30_ALT9 = 0x01,
}
impl Xbar1XbarInSelectInput24Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xbar1XbarInSelectInput24Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xbar1XbarInSelectInput24Daisy {
    #[inline(always)]
    fn from(val: u8) -> Xbar1XbarInSelectInput24Daisy {
        Xbar1XbarInSelectInput24Daisy::from_bits(val)
    }
}
impl From<Xbar1XbarInSelectInput24Daisy> for u8 {
    #[inline(always)]
    fn from(val: Xbar1XbarInSelectInput24Daisy) -> u8 {
        Xbar1XbarInSelectInput24Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xbar1XbarInSelectInput25Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_05 for Mode: ALT6"]
    SELECT_GPIO_EMC_B2_05_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_31 for Mode: ALT9"]
    SELECT_GPIO_AD_31_ALT9 = 0x01,
}
impl Xbar1XbarInSelectInput25Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xbar1XbarInSelectInput25Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xbar1XbarInSelectInput25Daisy {
    #[inline(always)]
    fn from(val: u8) -> Xbar1XbarInSelectInput25Daisy {
        Xbar1XbarInSelectInput25Daisy::from_bits(val)
    }
}
impl From<Xbar1XbarInSelectInput25Daisy> for u8 {
    #[inline(always)]
    fn from(val: Xbar1XbarInSelectInput25Daisy) -> u8 {
        Xbar1XbarInSelectInput25Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xbar1XbarInSelectInput26Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_06 for Mode: ALT6"]
    SELECT_GPIO_EMC_B2_06_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_00 for Mode: ALT4"]
    SELECT_GPIO_B1_00_ALT4 = 0x01,
}
impl Xbar1XbarInSelectInput26Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xbar1XbarInSelectInput26Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xbar1XbarInSelectInput26Daisy {
    #[inline(always)]
    fn from(val: u8) -> Xbar1XbarInSelectInput26Daisy {
        Xbar1XbarInSelectInput26Daisy::from_bits(val)
    }
}
impl From<Xbar1XbarInSelectInput26Daisy> for u8 {
    #[inline(always)]
    fn from(val: Xbar1XbarInSelectInput26Daisy) -> u8 {
        Xbar1XbarInSelectInput26Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xbar1XbarInSelectInput27Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_07 for Mode: ALT6"]
    SELECT_GPIO_EMC_B2_07_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_01 for Mode: ALT4"]
    SELECT_GPIO_B1_01_ALT4 = 0x01,
}
impl Xbar1XbarInSelectInput27Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xbar1XbarInSelectInput27Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xbar1XbarInSelectInput27Daisy {
    #[inline(always)]
    fn from(val: u8) -> Xbar1XbarInSelectInput27Daisy {
        Xbar1XbarInSelectInput27Daisy::from_bits(val)
    }
}
impl From<Xbar1XbarInSelectInput27Daisy> for u8 {
    #[inline(always)]
    fn from(val: Xbar1XbarInSelectInput27Daisy) -> u8 {
        Xbar1XbarInSelectInput27Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xbar1XbarInSelectInput28Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_08 for Mode: ALT6"]
    SELECT_GPIO_EMC_B2_08_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_02 for Mode: ALT4"]
    SELECT_GPIO_B1_02_ALT4 = 0x01,
}
impl Xbar1XbarInSelectInput28Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xbar1XbarInSelectInput28Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xbar1XbarInSelectInput28Daisy {
    #[inline(always)]
    fn from(val: u8) -> Xbar1XbarInSelectInput28Daisy {
        Xbar1XbarInSelectInput28Daisy::from_bits(val)
    }
}
impl From<Xbar1XbarInSelectInput28Daisy> for u8 {
    #[inline(always)]
    fn from(val: Xbar1XbarInSelectInput28Daisy) -> u8 {
        Xbar1XbarInSelectInput28Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xbar1XbarInSelectInput29Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_09 for Mode: ALT6"]
    SELECT_GPIO_EMC_B2_09_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_03 for Mode: ALT4"]
    SELECT_GPIO_B1_03_ALT4 = 0x01,
}
impl Xbar1XbarInSelectInput29Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xbar1XbarInSelectInput29Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xbar1XbarInSelectInput29Daisy {
    #[inline(always)]
    fn from(val: u8) -> Xbar1XbarInSelectInput29Daisy {
        Xbar1XbarInSelectInput29Daisy::from_bits(val)
    }
}
impl From<Xbar1XbarInSelectInput29Daisy> for u8 {
    #[inline(always)]
    fn from(val: Xbar1XbarInSelectInput29Daisy) -> u8 {
        Xbar1XbarInSelectInput29Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xbar1XbarInSelectInput30Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_10 for Mode: ALT6"]
    SELECT_GPIO_EMC_B2_10_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_04 for Mode: ALT4"]
    SELECT_GPIO_B1_04_ALT4 = 0x01,
}
impl Xbar1XbarInSelectInput30Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xbar1XbarInSelectInput30Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xbar1XbarInSelectInput30Daisy {
    #[inline(always)]
    fn from(val: u8) -> Xbar1XbarInSelectInput30Daisy {
        Xbar1XbarInSelectInput30Daisy::from_bits(val)
    }
}
impl From<Xbar1XbarInSelectInput30Daisy> for u8 {
    #[inline(always)]
    fn from(val: Xbar1XbarInSelectInput30Daisy) -> u8 {
        Xbar1XbarInSelectInput30Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xbar1XbarInSelectInput31Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_11 for Mode: ALT6"]
    SELECT_GPIO_EMC_B2_11_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_05 for Mode: ALT4"]
    SELECT_GPIO_B1_05_ALT4 = 0x01,
}
impl Xbar1XbarInSelectInput31Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xbar1XbarInSelectInput31Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xbar1XbarInSelectInput31Daisy {
    #[inline(always)]
    fn from(val: u8) -> Xbar1XbarInSelectInput31Daisy {
        Xbar1XbarInSelectInput31Daisy::from_bits(val)
    }
}
impl From<Xbar1XbarInSelectInput31Daisy> for u8 {
    #[inline(always)]
    fn from(val: Xbar1XbarInSelectInput31Daisy) -> u8 {
        Xbar1XbarInSelectInput31Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xbar1XbarInSelectInput32Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_12 for Mode: ALT6"]
    SELECT_GPIO_EMC_B2_12_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_06 for Mode: ALT4"]
    SELECT_GPIO_B1_06_ALT4 = 0x01,
}
impl Xbar1XbarInSelectInput32Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xbar1XbarInSelectInput32Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xbar1XbarInSelectInput32Daisy {
    #[inline(always)]
    fn from(val: u8) -> Xbar1XbarInSelectInput32Daisy {
        Xbar1XbarInSelectInput32Daisy::from_bits(val)
    }
}
impl From<Xbar1XbarInSelectInput32Daisy> for u8 {
    #[inline(always)]
    fn from(val: Xbar1XbarInSelectInput32Daisy) -> u8 {
        Xbar1XbarInSelectInput32Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xbar1XbarInSelectInput33Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_13 for Mode: ALT6"]
    SELECT_GPIO_EMC_B2_13_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_07 for Mode: ALT4"]
    SELECT_GPIO_B1_07_ALT4 = 0x01,
}
impl Xbar1XbarInSelectInput33Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xbar1XbarInSelectInput33Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xbar1XbarInSelectInput33Daisy {
    #[inline(always)]
    fn from(val: u8) -> Xbar1XbarInSelectInput33Daisy {
        Xbar1XbarInSelectInput33Daisy::from_bits(val)
    }
}
impl From<Xbar1XbarInSelectInput33Daisy> for u8 {
    #[inline(always)]
    fn from(val: Xbar1XbarInSelectInput33Daisy) -> u8 {
        Xbar1XbarInSelectInput33Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xbar1XbarInSelectInput34Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_14 for Mode: ALT6"]
    SELECT_GPIO_EMC_B2_14_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_10 for Mode: ALT4"]
    SELECT_GPIO_B1_10_ALT4 = 0x01,
}
impl Xbar1XbarInSelectInput34Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xbar1XbarInSelectInput34Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xbar1XbarInSelectInput34Daisy {
    #[inline(always)]
    fn from(val: u8) -> Xbar1XbarInSelectInput34Daisy {
        Xbar1XbarInSelectInput34Daisy::from_bits(val)
    }
}
impl From<Xbar1XbarInSelectInput34Daisy> for u8 {
    #[inline(always)]
    fn from(val: Xbar1XbarInSelectInput34Daisy) -> u8 {
        Xbar1XbarInSelectInput34Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xbar1XbarInSelectInput35Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_15 for Mode: ALT6"]
    SELECT_GPIO_EMC_B2_15_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_11 for Mode: ALT4"]
    SELECT_GPIO_B1_11_ALT4 = 0x01,
}
impl Xbar1XbarInSelectInput35Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xbar1XbarInSelectInput35Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xbar1XbarInSelectInput35Daisy {
    #[inline(always)]
    fn from(val: u8) -> Xbar1XbarInSelectInput35Daisy {
        Xbar1XbarInSelectInput35Daisy::from_bits(val)
    }
}
impl From<Xbar1XbarInSelectInput35Daisy> for u8 {
    #[inline(always)]
    fn from(val: Xbar1XbarInSelectInput35Daisy) -> u8 {
        Xbar1XbarInSelectInput35Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xbar1XbarInSelectInput36Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_19 for Mode: ALT6"]
    SELECT_GPIO_EMC_B2_19_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_08 for Mode: ALT4"]
    SELECT_GPIO_B1_08_ALT4 = 0x01,
}
impl Xbar1XbarInSelectInput36Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xbar1XbarInSelectInput36Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xbar1XbarInSelectInput36Daisy {
    #[inline(always)]
    fn from(val: u8) -> Xbar1XbarInSelectInput36Daisy {
        Xbar1XbarInSelectInput36Daisy::from_bits(val)
    }
}
impl From<Xbar1XbarInSelectInput36Daisy> for u8 {
    #[inline(always)]
    fn from(val: Xbar1XbarInSelectInput36Daisy) -> u8 {
        Xbar1XbarInSelectInput36Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xbar1XbarInSelectInput37Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_B2_20 for Mode: ALT6"]
    SELECT_GPIO_EMC_B2_20_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_09 for Mode: ALT4"]
    SELECT_GPIO_B1_09_ALT4 = 0x01,
}
impl Xbar1XbarInSelectInput37Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xbar1XbarInSelectInput37Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xbar1XbarInSelectInput37Daisy {
    #[inline(always)]
    fn from(val: u8) -> Xbar1XbarInSelectInput37Daisy {
        Xbar1XbarInSelectInput37Daisy::from_bits(val)
    }
}
impl From<Xbar1XbarInSelectInput37Daisy> for u8 {
    #[inline(always)]
    fn from(val: Xbar1XbarInSelectInput37Daisy) -> u8 {
        Xbar1XbarInSelectInput37Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum XspiSlvIppIndCsSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_00 for Mode: ALT4"]
    SELECT_GPIO_SD_B1_00_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_06 for Mode: ALT2"]
    SELECT_GPIO_SD_B2_06_ALT2 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_09 for Mode: ALT10"]
    SELECT_GPIO_B2_09_ALT10 = 0x02,
    _RESERVED_3 = 0x03,
}
impl XspiSlvIppIndCsSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> XspiSlvIppIndCsSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for XspiSlvIppIndCsSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> XspiSlvIppIndCsSelectInputDaisy {
        XspiSlvIppIndCsSelectInputDaisy::from_bits(val)
    }
}
impl From<XspiSlvIppIndCsSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: XspiSlvIppIndCsSelectInputDaisy) -> u8 {
        XspiSlvIppIndCsSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum XspiSlvIppIndDqsSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B2_05 for Mode: ALT2"]
    SELECT_GPIO_SD_B2_05_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_07 for Mode: ALT10"]
    SELECT_GPIO_B2_07_ALT10 = 0x01,
}
impl XspiSlvIppIndDqsSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> XspiSlvIppIndDqsSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for XspiSlvIppIndDqsSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> XspiSlvIppIndDqsSelectInputDaisy {
        XspiSlvIppIndDqsSelectInputDaisy::from_bits(val)
    }
}
impl From<XspiSlvIppIndDqsSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: XspiSlvIppIndDqsSelectInputDaisy) -> u8 {
        XspiSlvIppIndDqsSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum XspiSlvIppIndIoSelectInput0Daisy {
    #[doc = "Selecting Pad: GPIO_SD_B2_08 for Mode: ALT2"]
    SELECT_GPIO_SD_B2_08_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_10 for Mode: ALT10"]
    SELECT_GPIO_B2_10_ALT10 = 0x01,
}
impl XspiSlvIppIndIoSelectInput0Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> XspiSlvIppIndIoSelectInput0Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for XspiSlvIppIndIoSelectInput0Daisy {
    #[inline(always)]
    fn from(val: u8) -> XspiSlvIppIndIoSelectInput0Daisy {
        XspiSlvIppIndIoSelectInput0Daisy::from_bits(val)
    }
}
impl From<XspiSlvIppIndIoSelectInput0Daisy> for u8 {
    #[inline(always)]
    fn from(val: XspiSlvIppIndIoSelectInput0Daisy) -> u8 {
        XspiSlvIppIndIoSelectInput0Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum XspiSlvIppIndIoSelectInput1Daisy {
    #[doc = "Selecting Pad: GPIO_SD_B2_09 for Mode: ALT2"]
    SELECT_GPIO_SD_B2_09_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_11 for Mode: ALT10"]
    SELECT_GPIO_B2_11_ALT10 = 0x01,
}
impl XspiSlvIppIndIoSelectInput1Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> XspiSlvIppIndIoSelectInput1Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for XspiSlvIppIndIoSelectInput1Daisy {
    #[inline(always)]
    fn from(val: u8) -> XspiSlvIppIndIoSelectInput1Daisy {
        XspiSlvIppIndIoSelectInput1Daisy::from_bits(val)
    }
}
impl From<XspiSlvIppIndIoSelectInput1Daisy> for u8 {
    #[inline(always)]
    fn from(val: XspiSlvIppIndIoSelectInput1Daisy) -> u8 {
        XspiSlvIppIndIoSelectInput1Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum XspiSlvIppIndIoSelectInput2Daisy {
    #[doc = "Selecting Pad: GPIO_SD_B2_10 for Mode: ALT2"]
    SELECT_GPIO_SD_B2_10_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_12 for Mode: ALT10"]
    SELECT_GPIO_B2_12_ALT10 = 0x01,
}
impl XspiSlvIppIndIoSelectInput2Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> XspiSlvIppIndIoSelectInput2Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for XspiSlvIppIndIoSelectInput2Daisy {
    #[inline(always)]
    fn from(val: u8) -> XspiSlvIppIndIoSelectInput2Daisy {
        XspiSlvIppIndIoSelectInput2Daisy::from_bits(val)
    }
}
impl From<XspiSlvIppIndIoSelectInput2Daisy> for u8 {
    #[inline(always)]
    fn from(val: XspiSlvIppIndIoSelectInput2Daisy) -> u8 {
        XspiSlvIppIndIoSelectInput2Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum XspiSlvIppIndIoSelectInput3Daisy {
    #[doc = "Selecting Pad: GPIO_SD_B2_11 for Mode: ALT2"]
    SELECT_GPIO_SD_B2_11_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_13 for Mode: ALT10"]
    SELECT_GPIO_B2_13_ALT10 = 0x01,
}
impl XspiSlvIppIndIoSelectInput3Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> XspiSlvIppIndIoSelectInput3Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for XspiSlvIppIndIoSelectInput3Daisy {
    #[inline(always)]
    fn from(val: u8) -> XspiSlvIppIndIoSelectInput3Daisy {
        XspiSlvIppIndIoSelectInput3Daisy::from_bits(val)
    }
}
impl From<XspiSlvIppIndIoSelectInput3Daisy> for u8 {
    #[inline(always)]
    fn from(val: XspiSlvIppIndIoSelectInput3Daisy) -> u8 {
        XspiSlvIppIndIoSelectInput3Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum XspiSlvIppIndIoSelectInput4Daisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_02 for Mode: ALT4"]
    SELECT_GPIO_SD_B1_02_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_00 for Mode: ALT2"]
    SELECT_GPIO_SD_B2_00_ALT2 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_03 for Mode: ALT10"]
    SELECT_GPIO_B2_03_ALT10 = 0x02,
    _RESERVED_3 = 0x03,
}
impl XspiSlvIppIndIoSelectInput4Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> XspiSlvIppIndIoSelectInput4Daisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for XspiSlvIppIndIoSelectInput4Daisy {
    #[inline(always)]
    fn from(val: u8) -> XspiSlvIppIndIoSelectInput4Daisy {
        XspiSlvIppIndIoSelectInput4Daisy::from_bits(val)
    }
}
impl From<XspiSlvIppIndIoSelectInput4Daisy> for u8 {
    #[inline(always)]
    fn from(val: XspiSlvIppIndIoSelectInput4Daisy) -> u8 {
        XspiSlvIppIndIoSelectInput4Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum XspiSlvIppIndIoSelectInput5Daisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_03 for Mode: ALT4"]
    SELECT_GPIO_SD_B1_03_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_01 for Mode: ALT2"]
    SELECT_GPIO_SD_B2_01_ALT2 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_04 for Mode: ALT10"]
    SELECT_GPIO_B2_04_ALT10 = 0x02,
    _RESERVED_3 = 0x03,
}
impl XspiSlvIppIndIoSelectInput5Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> XspiSlvIppIndIoSelectInput5Daisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for XspiSlvIppIndIoSelectInput5Daisy {
    #[inline(always)]
    fn from(val: u8) -> XspiSlvIppIndIoSelectInput5Daisy {
        XspiSlvIppIndIoSelectInput5Daisy::from_bits(val)
    }
}
impl From<XspiSlvIppIndIoSelectInput5Daisy> for u8 {
    #[inline(always)]
    fn from(val: XspiSlvIppIndIoSelectInput5Daisy) -> u8 {
        XspiSlvIppIndIoSelectInput5Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum XspiSlvIppIndIoSelectInput6Daisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_04 for Mode: ALT4"]
    SELECT_GPIO_SD_B1_04_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_02 for Mode: ALT2"]
    SELECT_GPIO_SD_B2_02_ALT2 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_05 for Mode: ALT10"]
    SELECT_GPIO_B2_05_ALT10 = 0x02,
    _RESERVED_3 = 0x03,
}
impl XspiSlvIppIndIoSelectInput6Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> XspiSlvIppIndIoSelectInput6Daisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for XspiSlvIppIndIoSelectInput6Daisy {
    #[inline(always)]
    fn from(val: u8) -> XspiSlvIppIndIoSelectInput6Daisy {
        XspiSlvIppIndIoSelectInput6Daisy::from_bits(val)
    }
}
impl From<XspiSlvIppIndIoSelectInput6Daisy> for u8 {
    #[inline(always)]
    fn from(val: XspiSlvIppIndIoSelectInput6Daisy) -> u8 {
        XspiSlvIppIndIoSelectInput6Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum XspiSlvIppIndIoSelectInput7Daisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_05 for Mode: ALT4"]
    SELECT_GPIO_SD_B1_05_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_03 for Mode: ALT2"]
    SELECT_GPIO_SD_B2_03_ALT2 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_06 for Mode: ALT10"]
    SELECT_GPIO_B2_06_ALT10 = 0x02,
    _RESERVED_3 = 0x03,
}
impl XspiSlvIppIndIoSelectInput7Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> XspiSlvIppIndIoSelectInput7Daisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for XspiSlvIppIndIoSelectInput7Daisy {
    #[inline(always)]
    fn from(val: u8) -> XspiSlvIppIndIoSelectInput7Daisy {
        XspiSlvIppIndIoSelectInput7Daisy::from_bits(val)
    }
}
impl From<XspiSlvIppIndIoSelectInput7Daisy> for u8 {
    #[inline(always)]
    fn from(val: XspiSlvIppIndIoSelectInput7Daisy) -> u8 {
        XspiSlvIppIndIoSelectInput7Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum XspiSlvIppIndSckSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_01 for Mode: ALT4"]
    SELECT_GPIO_SD_B1_01_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_07 for Mode: ALT2"]
    SELECT_GPIO_SD_B2_07_ALT2 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_08 for Mode: ALT10"]
    SELECT_GPIO_B2_08_ALT10 = 0x02,
    _RESERVED_3 = 0x03,
}
impl XspiSlvIppIndSckSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> XspiSlvIppIndSckSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for XspiSlvIppIndSckSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> XspiSlvIppIndSckSelectInputDaisy {
        XspiSlvIppIndSckSelectInputDaisy::from_bits(val)
    }
}
impl From<XspiSlvIppIndSckSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: XspiSlvIppIndSckSelectInputDaisy) -> u8 {
        XspiSlvIppIndSckSelectInputDaisy::to_bits(val)
    }
}
