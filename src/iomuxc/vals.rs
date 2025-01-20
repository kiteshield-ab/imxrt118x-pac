#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Can1IppIndCanrxSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_16 for Mode: ALT9"]
    SelectGpioAd16Alt9 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_01 for Mode: ALT1"]
    SelectGpioAon01Alt1 = 0x01,
    #[doc = "Selecting Pad: GPIO_AON_07 for Mode: ALT6"]
    SelectGpioAon07Alt6 = 0x02,
    #[doc = "Selecting Pad: GPIO_AON_17 for Mode: ALT6"]
    SelectGpioAon17Alt6 = 0x03,
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
    SelectGpioAd01Alt1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_31 for Mode: ALT2"]
    SelectGpioAd31Alt2 = 0x01,
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
    SelectGpioAd07Alt1 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_11 for Mode: ALT2"]
    SelectGpioB211Alt2 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_13 for Mode: ALT6"]
    SelectGpioB213Alt6 = 0x02,
    #[doc = "Selecting Pad: GPIO_AON_03 for Mode: ALT1"]
    SelectGpioAon03Alt1 = 0x03,
    #[doc = "Selecting Pad: GPIO_AON_19 for Mode: ALT6"]
    SelectGpioAon19Alt6 = 0x04,
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
    Dse0NormalDriver = 0x0,
    #[doc = "high driver"]
    Dse1HighDriver = 0x01,
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
    SelectGpioEmcB102Alt10 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_00 for Mode: ALT12"]
    SelectGpioEmcB200Alt12 = 0x01,
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
    SelectGpioEmcB138Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_13 for Mode: ALT12"]
    SelectGpioB213Alt12 = 0x01,
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
    SelectGpioEmcB109Alt10 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_09 for Mode: ALT12"]
    SelectGpioEmcB209Alt12 = 0x01,
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
    SelectGpioEmcB130Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_17 for Mode: ALT12"]
    SelectGpioEmcB217Alt12 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_10 for Mode: ALT12"]
    SelectGpioB210Alt12 = 0x02,
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
    SelectGpioEmcB110Alt10 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_10 for Mode: ALT12"]
    SelectGpioEmcB210Alt12 = 0x01,
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
    SelectGpioEmcB131Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_18 for Mode: ALT12"]
    SelectGpioEmcB218Alt12 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_11 for Mode: ALT12"]
    SelectGpioB211Alt12 = 0x02,
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
    SelectGpioEmcB104Alt10 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_01 for Mode: ALT12"]
    SelectGpioEmcB201Alt12 = 0x01,
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
    SelectGpioEmcB134Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_02 for Mode: ALT12"]
    SelectGpioB202Alt12 = 0x01,
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
    SelectGpioEmcB103Alt10 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_02 for Mode: ALT12"]
    SelectGpioEmcB202Alt12 = 0x01,
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
    SelectGpioEmcB135Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_03 for Mode: ALT12"]
    SelectGpioB203Alt12 = 0x01,
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
    SelectGpioEmcB111Alt10 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_11 for Mode: ALT12"]
    SelectGpioEmcB211Alt12 = 0x01,
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
    SelectGpioEmcB132Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_19 for Mode: ALT12"]
    SelectGpioEmcB219Alt12 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_12 for Mode: ALT12"]
    SelectGpioB212Alt12 = 0x02,
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
    SelectGpioEmcB112Alt10 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_12 for Mode: ALT12"]
    SelectGpioEmcB212Alt12 = 0x01,
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
    SelectGpioEmcB133Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_20 for Mode: ALT12"]
    SelectGpioEmcB220Alt12 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_01 for Mode: ALT12"]
    SelectGpioB201Alt12 = 0x02,
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
    SelectGpioEmcB108Alt10 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_08 for Mode: ALT12"]
    SelectGpioEmcB208Alt12 = 0x01,
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
    SelectGpioEmcB129Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_16 for Mode: ALT12"]
    SelectGpioEmcB216Alt12 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_09 for Mode: ALT12"]
    SelectGpioB209Alt12 = 0x02,
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
    SelectGpioAd31Alt12 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_10 for Mode: ALT12"]
    SelectGpioSdB210Alt12 = 0x01,
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
    SelectGpioAd19Alt12 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_06 for Mode: ALT12"]
    SelectGpioAon06Alt12 = 0x01,
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
    SelectGpioEmcB124Alt1 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_36 for Mode: ALT1"]
    SelectGpioEmcB136Alt1 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_00 for Mode: ALT4"]
    SelectGpioAd00Alt4 = 0x02,
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
    SelectGpioEmcB126Alt1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_02 for Mode: ALT4"]
    SelectGpioAd02Alt4 = 0x01,
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
    SelectGpioEmcB129Alt1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_05 for Mode: ALT4"]
    SelectGpioAd05Alt4 = 0x01,
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
    SelectGpioEmcB125Alt1 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_37 for Mode: ALT1"]
    SelectGpioEmcB137Alt1 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_01 for Mode: ALT4"]
    SelectGpioAd01Alt4 = 0x02,
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
    SelectGpioEmcB127Alt1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_03 for Mode: ALT4"]
    SelectGpioAd03Alt4 = 0x01,
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
    SelectGpioEmcB128Alt1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_04 for Mode: ALT4"]
    SelectGpioAd04Alt4 = 0x01,
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
    SelectGpioEmcB118Alt1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_24 for Mode: ALT4"]
    SelectGpioAd24Alt4 = 0x01,
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
    SelectGpioEmcB120Alt1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_26 for Mode: ALT4"]
    SelectGpioAd26Alt4 = 0x01,
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
    SelectGpioEmcB123Alt1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_29 for Mode: ALT4"]
    SelectGpioAd29Alt4 = 0x01,
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
    SelectGpioEmcB119Alt1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_25 for Mode: ALT4"]
    SelectGpioAd25Alt4 = 0x01,
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
    SelectGpioEmcB121Alt1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_27 for Mode: ALT4"]
    SelectGpioAd27Alt4 = 0x01,
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
    SelectGpioEmcB122Alt1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_28 for Mode: ALT4"]
    SelectGpioAd28Alt4 = 0x01,
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
    SelectGpioEmcB130Alt1 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_00 for Mode: ALT10"]
    SelectGpioEmcB200Alt10 = 0x01,
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
    SelectGpioEmcB132Alt1 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_02 for Mode: ALT10"]
    SelectGpioEmcB202Alt10 = 0x01,
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
    SelectGpioEmcB135Alt1 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_05 for Mode: ALT10"]
    SelectGpioEmcB205Alt10 = 0x01,
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
    SelectGpioEmcB111Alt1 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_07 for Mode: ALT10"]
    SelectGpioEmcB207Alt10 = 0x01,
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
    SelectGpioEmcB131Alt1 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_01 for Mode: ALT10"]
    SelectGpioEmcB201Alt10 = 0x01,
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
    SelectGpioEmcB133Alt1 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_03 for Mode: ALT10"]
    SelectGpioEmcB203Alt10 = 0x01,
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
    SelectGpioEmcB134Alt1 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_04 for Mode: ALT10"]
    SelectGpioEmcB204Alt10 = 0x01,
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
    SelectGpioEmcB110Alt1 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_06 for Mode: ALT10"]
    SelectGpioEmcB206Alt10 = 0x01,
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
    SelectGpioSdB212DummyAlt0 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_07 for Mode: ALT7"]
    SelectGpioB207Alt7 = 0x01,
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
    SelectGpioSdB205Alt1 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_12_DUMMY for Mode: ALT1"]
    SelectGpioSdB212DummyAlt1 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_03 for Mode: ALT7"]
    SelectGpioB103Alt7 = 0x02,
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
    SelectGpioSdB208Alt1 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_13 for Mode: ALT7"]
    SelectGpioB113Alt7 = 0x01,
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
    SelectGpioSdB209Alt1 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_12 for Mode: ALT7"]
    SelectGpioB112Alt7 = 0x01,
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
    SelectGpioSdB210Alt1 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_11 for Mode: ALT7"]
    SelectGpioB111Alt7 = 0x01,
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
    SelectGpioSdB211Alt1 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_10 for Mode: ALT7"]
    SelectGpioB110Alt7 = 0x01,
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
    SelectGpioSdB200Alt1 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_09 for Mode: ALT7"]
    SelectGpioB109Alt7 = 0x01,
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
    SelectGpioSdB201Alt1 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_08 for Mode: ALT7"]
    SelectGpioB108Alt7 = 0x01,
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
    SelectGpioSdB202Alt1 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_07 for Mode: ALT7"]
    SelectGpioB107Alt7 = 0x01,
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
    SelectGpioSdB203Alt1 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_06 for Mode: ALT7"]
    SelectGpioB106Alt7 = 0x01,
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
    SelectGpioSdB207Alt1 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_05 for Mode: ALT7"]
    SelectGpioB105Alt7 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_02 for Mode: ALT6"]
    SelectGpioB202Alt6 = 0x02,
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
    SelectGpioEmcB140Alt3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_21 for Mode: ALT8"]
    SelectGpioAon21Alt8 = 0x01,
    #[doc = "Selecting Pad: GPIO_AON_28_DUMMY for Mode: ALT0"]
    SelectGpioAon28DummyAlt0 = 0x02,
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
    SelectGpioEmcB121Alt10 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_29 for Mode: ALT3"]
    SelectGpioEmcB129Alt3 = 0x01,
    #[doc = "Selecting Pad: GPIO_AON_20 for Mode: ALT0"]
    SelectGpioAon20Alt0 = 0x02,
    #[doc = "Selecting Pad: GPIO_AON_28_DUMMY for Mode: ALT1"]
    SelectGpioAon28DummyAlt1 = 0x03,
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
    SelectGpioEmcB135Alt3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_24 for Mode: ALT0"]
    SelectGpioAon24Alt0 = 0x01,
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
    SelectGpioEmcB136Alt3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_25 for Mode: ALT0"]
    SelectGpioAon25Alt0 = 0x01,
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
    SelectGpioEmcB137Alt3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_26 for Mode: ALT0"]
    SelectGpioAon26Alt0 = 0x01,
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
    SelectGpioEmcB138Alt3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_27 for Mode: ALT0"]
    SelectGpioAon27Alt0 = 0x01,
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
    SelectGpioEmcB125Alt10 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_33 for Mode: ALT3"]
    SelectGpioEmcB133Alt3 = 0x01,
    #[doc = "Selecting Pad: GPIO_AON_18 for Mode: ALT0"]
    SelectGpioAon18Alt0 = 0x02,
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
    SelectGpioEmcB124Alt10 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_32 for Mode: ALT3"]
    SelectGpioEmcB132Alt3 = 0x01,
    #[doc = "Selecting Pad: GPIO_AON_17 for Mode: ALT0"]
    SelectGpioAon17Alt0 = 0x02,
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
    SelectGpioEmcB123Alt10 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_31 for Mode: ALT3"]
    SelectGpioEmcB131Alt3 = 0x01,
    #[doc = "Selecting Pad: GPIO_AON_16 for Mode: ALT0"]
    SelectGpioAon16Alt0 = 0x02,
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
    SelectGpioEmcB122Alt10 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_30 for Mode: ALT3"]
    SelectGpioEmcB130Alt3 = 0x01,
    #[doc = "Selecting Pad: GPIO_AON_15 for Mode: ALT0"]
    SelectGpioAon15Alt0 = 0x02,
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
    SelectGpioEmcB141Alt3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_23 for Mode: ALT0"]
    SelectGpioAon23Alt0 = 0x01,
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
    SelectGpioEmcB134Alt3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AON_19 for Mode: ALT0"]
    SelectGpioAon19Alt0 = 0x01,
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
    SelectGpioAd18Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_34 for Mode: ALT0"]
    SelectGpioAd34Alt0 = 0x01,
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
    SelectGpioAd19Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_35 for Mode: ALT0"]
    SelectGpioAd35Alt0 = 0x01,
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
    IbeOff0Disabled = 0x0,
    #[doc = "Enabled"]
    IbeOff1Enabled = 0x01,
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
    SelectGpioEmcB115Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_27 for Mode: ALT6"]
    SelectGpioAd27Alt6 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_B2_03 for Mode: ALT4"]
    SelectGpioSdB203Alt4 = 0x02,
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
    SelectGpioEmcB113Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_33 for Mode: ALT6"]
    SelectGpioAd33Alt6 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_B2_01 for Mode: ALT4"]
    SelectGpioSdB201Alt4 = 0x02,
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
    SelectGpioEmcB103Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_31 for Mode: ALT6"]
    SelectGpioAd31Alt6 = 0x01,
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
    SelectGpioEmcB101Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_29 for Mode: ALT6"]
    SelectGpioAd29Alt6 = 0x01,
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
    SelectGpioEmcB112Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_19 for Mode: ALT3"]
    SelectGpioAd19Alt3 = 0x01,
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
    SelectGpioEmcB110Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_17 for Mode: ALT3"]
    SelectGpioAd17Alt3 = 0x01,
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
    SelectGpioEmcB108Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_15 for Mode: ALT3"]
    SelectGpioAd15Alt3 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_B1_03 for Mode: ALT8"]
    SelectGpioSdB103Alt8 = 0x02,
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
    SelectGpioEmcB106Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_13 for Mode: ALT3"]
    SelectGpioAd13Alt3 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_B1_01 for Mode: ALT8"]
    SelectGpioSdB101Alt8 = 0x02,
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
    SelectGpioEmcB114Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_26 for Mode: ALT6"]
    SelectGpioAd26Alt6 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_B2_02 for Mode: ALT4"]
    SelectGpioSdB202Alt4 = 0x02,
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
    SelectGpioEmcB104Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_32 for Mode: ALT6"]
    SelectGpioAd32Alt6 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_B2_00 for Mode: ALT4"]
    SelectGpioSdB200Alt4 = 0x02,
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
    SelectGpioEmcB102Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_30 for Mode: ALT6"]
    SelectGpioAd30Alt6 = 0x01,
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
    SelectGpioEmcB100Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_28 for Mode: ALT6"]
    SelectGpioAd28Alt6 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_B2_04 for Mode: ALT4"]
    SelectGpioSdB204Alt4 = 0x02,
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
    SelectGpioEmcB111Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_18 for Mode: ALT3"]
    SelectGpioAd18Alt3 = 0x01,
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
    SelectGpioEmcB109Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_16 for Mode: ALT3"]
    SelectGpioAd16Alt3 = 0x01,
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
    SelectGpioEmcB107Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_14 for Mode: ALT3"]
    SelectGpioAd14Alt3 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_B1_02 for Mode: ALT8"]
    SelectGpioSdB102Alt8 = 0x02,
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
    SelectGpioEmcB105Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_12 for Mode: ALT3"]
    SelectGpioAd12Alt3 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_B1_00 for Mode: ALT8"]
    SelectGpioSdB100Alt8 = 0x02,
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
    SelectGpioEmcB200Alt9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_19 for Mode: ALT8"]
    SelectGpioEmcB219Alt8 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_18 for Mode: ALT9"]
    SelectGpioAd18Alt9 = 0x02,
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
    SelectGpioEmcB201Alt9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_20 for Mode: ALT8"]
    SelectGpioEmcB220Alt8 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_19 for Mode: ALT9"]
    SelectGpioAd19Alt9 = 0x02,
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
    SelectGpioAd24Alt1 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_10 for Mode: ALT6"]
    SelectGpioB210Alt6 = 0x01,
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
    SelectGpioAd25Alt1 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_11 for Mode: ALT6"]
    SelectGpioB211Alt6 = 0x01,
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
    SelectGpioAd08Alt1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_32 for Mode: ALT0"]
    SelectGpioAd32Alt0 = 0x01,
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
    SelectGpioAd09Alt1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_33 for Mode: ALT0"]
    SelectGpioAd33Alt0 = 0x01,
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
    SelectGpioB102Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_08 for Mode: ALT6"]
    SelectGpioB208Alt6 = 0x01,
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
    SelectGpioB103Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_09 for Mode: ALT6"]
    SelectGpioB209Alt6 = 0x01,
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
    SelectGpioEmcB205Alt8 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_17 for Mode: ALT7"]
    SelectGpioAd17Alt7 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_B1_00 for Mode: ALT6"]
    SelectGpioSdB100Alt6 = 0x02,
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
    SelectGpioEmcB210Alt8 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_15 for Mode: ALT7"]
    SelectGpioAd15Alt7 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_B1_04 for Mode: ALT6"]
    SelectGpioSdB104Alt6 = 0x02,
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
    SelectGpioEmcB209Alt8 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B1_05 for Mode: ALT6"]
    SelectGpioSdB105Alt6 = 0x01,
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
    SelectGpioEmcB208Alt8 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_00 for Mode: ALT6"]
    SelectGpioSdB200Alt6 = 0x01,
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
    SelectGpioEmcB204Alt8 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_16 for Mode: ALT7"]
    SelectGpioAd16Alt7 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_B1_01 for Mode: ALT6"]
    SelectGpioSdB101Alt6 = 0x02,
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
    SelectGpioEmcB207Alt8 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_19 for Mode: ALT7"]
    SelectGpioAd19Alt7 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_B1_03 for Mode: ALT6"]
    SelectGpioSdB103Alt6 = 0x02,
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
    SelectGpioEmcB206Alt8 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_18 for Mode: ALT7"]
    SelectGpioAd18Alt7 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_B1_02 for Mode: ALT6"]
    SelectGpioSdB102Alt6 = 0x02,
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
    SelectGpioEmcB125Alt3 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_09 for Mode: ALT4"]
    SelectGpioSdB209Alt4 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_13 for Mode: ALT9"]
    SelectGpioB213Alt9 = 0x02,
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
    SelectGpioEmcB122Alt3 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_08 for Mode: ALT4"]
    SelectGpioSdB208Alt4 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_10 for Mode: ALT9"]
    SelectGpioB210Alt9 = 0x02,
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
    SelectGpioEmcB123Alt3 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_11 for Mode: ALT4"]
    SelectGpioSdB211Alt4 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_11 for Mode: ALT9"]
    SelectGpioB211Alt9 = 0x02,
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
    SelectGpioEmcB124Alt3 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_10 for Mode: ALT4"]
    SelectGpioSdB210Alt4 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_12 for Mode: ALT9"]
    SelectGpioB212Alt9 = 0x02,
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
    SelectGpioEmcB134Alt10 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_01 for Mode: ALT8"]
    SelectGpioEmcB201Alt8 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_29 for Mode: ALT0"]
    SelectGpioAd29Alt0 = 0x02,
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
    SelectGpioEmcB135Alt10 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_13 for Mode: ALT4"]
    SelectGpioEmcB213Alt4 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_27 for Mode: ALT2"]
    SelectGpioAd27Alt2 = 0x02,
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
    SelectGpioEmcB212Alt4 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_26 for Mode: ALT2"]
    SelectGpioAd26Alt2 = 0x01,
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
    SelectGpioEmcB211Alt4 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_25 for Mode: ALT2"]
    SelectGpioAd25Alt2 = 0x01,
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
    SelectGpioEmcB131Alt9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_00 for Mode: ALT8"]
    SelectGpioEmcB200Alt8 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_28 for Mode: ALT0"]
    SelectGpioAd28Alt0 = 0x02,
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
    SelectGpioEmcB133Alt9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_03 for Mode: ALT8"]
    SelectGpioEmcB203Alt8 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_31 for Mode: ALT0"]
    SelectGpioAd31Alt0 = 0x02,
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
    SelectGpioEmcB132Alt9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_02 for Mode: ALT8"]
    SelectGpioEmcB202Alt8 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_30 for Mode: ALT0"]
    SelectGpioAd30Alt0 = 0x02,
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
    SelectGpioEmcB121Alt3 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_29 for Mode: ALT10"]
    SelectGpioEmcB129Alt10 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_12 for Mode: ALT9"]
    SelectGpioB112Alt9 = 0x02,
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
    SelectGpioEmcB117Alt10 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_30 for Mode: ALT10"]
    SelectGpioEmcB130Alt10 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_09 for Mode: ALT9"]
    SelectGpioB109Alt9 = 0x02,
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
    SelectGpioEmcB116Alt10 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_31 for Mode: ALT10"]
    SelectGpioEmcB131Alt10 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_10 for Mode: ALT9"]
    SelectGpioB110Alt9 = 0x02,
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
    SelectGpioEmcB132Alt10 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_11 for Mode: ALT9"]
    SelectGpioB111Alt9 = 0x01,
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
    SelectGpioEmcB118Alt3 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_26 for Mode: ALT10"]
    SelectGpioEmcB126Alt10 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_13 for Mode: ALT9"]
    SelectGpioB113Alt9 = 0x02,
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
    SelectGpioEmcB119Alt3 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_27 for Mode: ALT10"]
    SelectGpioEmcB127Alt10 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_07 for Mode: ALT9"]
    SelectGpioB107Alt9 = 0x02,
    #[doc = "Selecting Pad: GPIO_B2_00 for Mode: ALT9"]
    SelectGpioB200Alt9 = 0x03,
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
    SelectGpioEmcB120Alt3 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_28 for Mode: ALT10"]
    SelectGpioEmcB128Alt10 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_08 for Mode: ALT9"]
    SelectGpioB108Alt9 = 0x02,
    #[doc = "Selecting Pad: GPIO_B2_01 for Mode: ALT9"]
    SelectGpioB201Alt9 = 0x03,
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
    SelectGpioAd16Alt1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_33 for Mode: ALT8"]
    SelectGpioAd33Alt8 = 0x01,
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
    SelectGpioAd15Alt1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_32 for Mode: ALT8"]
    SelectGpioAd32Alt8 = 0x01,
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
    SelectGpioEmcB214Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_03 for Mode: ALT9"]
    SelectGpioB103Alt9 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_07 for Mode: ALT9"]
    SelectGpioB207Alt9 = 0x02,
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
    SelectGpioEmcB213Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_02 for Mode: ALT9"]
    SelectGpioB102Alt9 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_06 for Mode: ALT9"]
    SelectGpioB206Alt9 = 0x02,
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
    SelectGpioEmcB100Alt3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_15 for Mode: ALT6"]
    SelectGpioAd15Alt6 = 0x01,
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
    SelectGpioEmcB102Alt3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_14 for Mode: ALT6"]
    SelectGpioAd14Alt6 = 0x01,
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
    SelectGpioEmcB103Alt3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_13 for Mode: ALT6"]
    SelectGpioAd13Alt6 = 0x01,
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
    SelectGpioEmcB114Alt10 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_21 for Mode: ALT9"]
    SelectGpioEmcB121Alt9 = 0x01,
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
    SelectGpioEmcB137Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_19 for Mode: ALT2"]
    SelectGpioEmcB219Alt2 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_B2_06 for Mode: ALT6"]
    SelectGpioSdB206Alt6 = 0x02,
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
    SelectGpioEmcB215Alt4 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_10 for Mode: ALT6"]
    SelectGpioSdB210Alt6 = 0x01,
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
    SelectGpioEmcB214Alt4 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_11 for Mode: ALT6"]
    SelectGpioSdB211Alt6 = 0x01,
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
    SelectGpioAd18Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_04 for Mode: ALT6"]
    SelectGpioSdB204Alt6 = 0x01,
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
    SelectGpioEmcB115Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_36 for Mode: ALT2"]
    SelectGpioEmcB136Alt2 = 0x01,
    #[doc = "Selecting Pad: GPIO_EMC_B2_18 for Mode: ALT2"]
    SelectGpioEmcB218Alt2 = 0x02,
    #[doc = "Selecting Pad: GPIO_AD_27 for Mode: ALT1"]
    SelectGpioAd27Alt1 = 0x03,
    #[doc = "Selecting Pad: GPIO_SD_B2_09 for Mode: ALT6"]
    SelectGpioSdB209Alt6 = 0x04,
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
    SelectGpioEmcB114Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_35 for Mode: ALT2"]
    SelectGpioEmcB135Alt2 = 0x01,
    #[doc = "Selecting Pad: GPIO_EMC_B2_17 for Mode: ALT2"]
    SelectGpioEmcB217Alt2 = 0x02,
    #[doc = "Selecting Pad: GPIO_AD_26 for Mode: ALT1"]
    SelectGpioAd26Alt1 = 0x03,
    #[doc = "Selecting Pad: GPIO_SD_B2_08 for Mode: ALT6"]
    SelectGpioSdB208Alt6 = 0x04,
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
    SelectGpioEmcB133Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_26 for Mode: ALT0"]
    SelectGpioAd26Alt0 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_12 for Mode: ALT4"]
    SelectGpioB212Alt4 = 0x02,
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
    SelectGpioEmcB129Alt9 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_07 for Mode: ALT3"]
    SelectGpioB207Alt3 = 0x01,
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
    SelectGpioEmcB130Alt9 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_06 for Mode: ALT3"]
    SelectGpioB206Alt3 = 0x01,
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
    SelectGpioEmcB127Alt9 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_08 for Mode: ALT4"]
    SelectGpioB208Alt4 = 0x01,
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
    SelectGpioEmcB132Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_25 for Mode: ALT0"]
    SelectGpioAd25Alt0 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_11 for Mode: ALT4"]
    SelectGpioB211Alt4 = 0x02,
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
    SelectGpioEmcB131Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_24 for Mode: ALT0"]
    SelectGpioAd24Alt0 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_10 for Mode: ALT4"]
    SelectGpioB210Alt4 = 0x02,
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
    SelectGpioSdB202Alt9 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_10 for Mode: ALT3"]
    SelectGpioB210Alt3 = 0x01,
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
    SelectGpioAd31Alt4 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_01 for Mode: ALT9"]
    SelectGpioSdB201Alt9 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_13 for Mode: ALT2"]
    SelectGpioB213Alt2 = 0x02,
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
    SelectGpioAd30Alt4 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_00 for Mode: ALT9"]
    SelectGpioSdB200Alt9 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_12 for Mode: ALT2"]
    SelectGpioB212Alt2 = 0x02,
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
    SelectGpioEmcB117Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_04 for Mode: ALT2"]
    SelectGpioB104Alt2 = 0x01,
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
    SelectGpioEmcB116Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_06 for Mode: ALT2"]
    SelectGpioB106Alt2 = 0x01,
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
    SelectGpioAd01Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_28 for Mode: ALT12"]
    SelectGpioAd28Alt12 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_B2_00 for Mode: ALT12"]
    SelectGpioSdB200Alt12 = 0x02,
    #[doc = "Selecting Pad: GPIO_B2_10 for Mode: ALT0"]
    SelectGpioB210Alt0 = 0x03,
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
    SelectGpioAd02Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_29 for Mode: ALT12"]
    SelectGpioAd29Alt12 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_B2_01 for Mode: ALT12"]
    SelectGpioSdB201Alt12 = 0x02,
    #[doc = "Selecting Pad: GPIO_B2_11 for Mode: ALT0"]
    SelectGpioB211Alt0 = 0x03,
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
    SelectGpioAd03Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_26 for Mode: ALT12"]
    SelectGpioAd26Alt12 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_B2_02 for Mode: ALT12"]
    SelectGpioSdB202Alt12 = 0x02,
    #[doc = "Selecting Pad: GPIO_B2_12 for Mode: ALT0"]
    SelectGpioB212Alt0 = 0x03,
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
    SelectGpioAd04Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_32 for Mode: ALT12"]
    SelectGpioAd32Alt12 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_B2_03 for Mode: ALT12"]
    SelectGpioSdB203Alt12 = 0x02,
    #[doc = "Selecting Pad: GPIO_B2_13 for Mode: ALT0"]
    SelectGpioB213Alt0 = 0x03,
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
    SelectGpioAd20Alt7 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_00 for Mode: ALT8"]
    SelectGpioSdB200Alt8 = 0x01,
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
    SelectGpioEmcB119Alt10 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_41 for Mode: ALT1"]
    SelectGpioEmcB141Alt1 = 0x01,
    #[doc = "Selecting Pad: GPIO_EMC_B2_01 for Mode: ALT3"]
    SelectGpioEmcB201Alt3 = 0x02,
    #[doc = "Selecting Pad: GPIO_EMC_B2_20 for Mode: ALT4"]
    SelectGpioEmcB220Alt4 = 0x03,
    #[doc = "Selecting Pad: GPIO_AD_31 for Mode: ALT7"]
    SelectGpioAd31Alt7 = 0x04,
    #[doc = "Selecting Pad: GPIO_SD_B2_10 for Mode: ALT10"]
    SelectGpioSdB210Alt10 = 0x05,
    #[doc = "Selecting Pad: GPIO_B1_12 for Mode: ALT1"]
    SelectGpioB112Alt1 = 0x06,
    #[doc = "Selecting Pad: GPIO_B2_02 for Mode: ALT3"]
    SelectGpioB202Alt3 = 0x07,
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
    SelectGpioEmcB119Alt4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_41 for Mode: ALT4"]
    SelectGpioEmcB141Alt4 = 0x01,
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
    SelectGpioEmcB118Alt4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_40 for Mode: ALT4"]
    SelectGpioEmcB140Alt4 = 0x01,
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
    SelectGpioEmcB116Alt9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_40 for Mode: ALT2"]
    SelectGpioEmcB140Alt2 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_01 for Mode: ALT10"]
    SelectGpioB201Alt10 = 0x02,
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
    SelectGpioEmcB117Alt9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_41 for Mode: ALT2"]
    SelectGpioEmcB141Alt2 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_00 for Mode: ALT10"]
    SelectGpioB200Alt10 = 0x02,
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
    SelectGpioEmcB115Alt4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_04 for Mode: ALT9"]
    SelectGpioEmcB204Alt9 = 0x01,
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
    SelectGpioEmcB114Alt4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_03 for Mode: ALT9"]
    SelectGpioEmcB203Alt9 = 0x01,
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
    SelectGpioEmcB116Alt4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_24 for Mode: ALT9"]
    SelectGpioEmcB124Alt9 = 0x01,
    #[doc = "Selecting Pad: GPIO_EMC_B2_19 for Mode: ALT9"]
    SelectGpioEmcB219Alt9 = 0x02,
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
    SelectGpioEmcB117Alt4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_25 for Mode: ALT9"]
    SelectGpioEmcB125Alt9 = 0x01,
    #[doc = "Selecting Pad: GPIO_EMC_B2_20 for Mode: ALT9"]
    SelectGpioEmcB220Alt9 = 0x02,
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
    SelectGpioEmcB115Alt9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_06 for Mode: ALT4"]
    SelectGpioEmcB206Alt4 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_01 for Mode: ALT8"]
    SelectGpioB201Alt8 = 0x02,
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
    SelectGpioEmcB114Alt9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_05 for Mode: ALT4"]
    SelectGpioEmcB205Alt4 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_00 for Mode: ALT8"]
    SelectGpioB200Alt8 = 0x02,
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
    SelectGpioEmcB116Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_05 for Mode: ALT1"]
    SelectGpioEmcB205Alt1 = 0x01,
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
    SelectGpioEmcB117Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_06 for Mode: ALT1"]
    SelectGpioEmcB206Alt1 = 0x01,
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
    SelectGpioEmcB200Alt4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_15 for Mode: ALT3"]
    SelectGpioEmcB215Alt3 = 0x01,
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
    SelectGpioEmcB140Alt9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_11 for Mode: ALT3"]
    SelectGpioEmcB211Alt3 = 0x01,
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
    SelectGpioEmcB141Alt9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_12 for Mode: ALT3"]
    SelectGpioEmcB212Alt3 = 0x01,
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
    SelectGpioEmcB138Alt9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_09 for Mode: ALT3"]
    SelectGpioEmcB209Alt3 = 0x01,
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
    SelectGpioEmcB139Alt9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_10 for Mode: ALT3"]
    SelectGpioEmcB210Alt3 = 0x01,
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
    SelectGpioEmcB201Alt4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_16 for Mode: ALT3"]
    SelectGpioEmcB216Alt3 = 0x01,
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
    SelectGpioEmcB202Alt4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_17 for Mode: ALT3"]
    SelectGpioEmcB217Alt3 = 0x01,
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
    SelectGpioEmcB137Alt9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_08 for Mode: ALT3"]
    SelectGpioEmcB208Alt3 = 0x01,
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
    SelectGpioEmcB121Alt4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_33 for Mode: ALT10"]
    SelectGpioEmcB133Alt10 = 0x01,
    #[doc = "Selecting Pad: GPIO_EMC_B1_38 for Mode: ALT4"]
    SelectGpioEmcB138Alt4 = 0x02,
    #[doc = "Selecting Pad: GPIO_B2_13 for Mode: ALT11"]
    SelectGpioB213Alt11 = 0x03,
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
    SelectGpioEmcB113Alt3 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_32 for Mode: ALT4"]
    SelectGpioEmcB132Alt4 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_12 for Mode: ALT11"]
    SelectGpioB212Alt11 = 0x02,
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
    SelectGpioEmcB133Alt4 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_01 for Mode: ALT11"]
    SelectGpioB201Alt11 = 0x01,
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
    SelectGpioEmcB116Alt3 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_30 for Mode: ALT4"]
    SelectGpioEmcB130Alt4 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_10 for Mode: ALT11"]
    SelectGpioB210Alt11 = 0x02,
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
    SelectGpioEmcB117Alt3 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_31 for Mode: ALT4"]
    SelectGpioEmcB131Alt4 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_11 for Mode: ALT11"]
    SelectGpioB211Alt11 = 0x02,
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
    SelectGpioEmcB122Alt4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_34 for Mode: ALT4"]
    SelectGpioEmcB134Alt4 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_02 for Mode: ALT11"]
    SelectGpioB202Alt11 = 0x02,
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
    SelectGpioEmcB123Alt4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_35 for Mode: ALT4"]
    SelectGpioEmcB135Alt4 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_03 for Mode: ALT11"]
    SelectGpioB203Alt11 = 0x02,
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
    SelectGpioEmcB115Alt3 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B1_29 for Mode: ALT4"]
    SelectGpioEmcB129Alt4 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_09 for Mode: ALT11"]
    SelectGpioB209Alt11 = 0x02,
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
    SelectGpioEmcB102Alt4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_15 for Mode: ALT9"]
    SelectGpioEmcB215Alt9 = 0x01,
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
    SelectGpioEmcB111Alt4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_11 for Mode: ALT9"]
    SelectGpioEmcB211Alt9 = 0x01,
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
    SelectGpioEmcB112Alt4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_12 for Mode: ALT9"]
    SelectGpioEmcB212Alt9 = 0x01,
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
    SelectGpioEmcB109Alt4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_09 for Mode: ALT9"]
    SelectGpioEmcB209Alt9 = 0x01,
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
    SelectGpioEmcB110Alt4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_10 for Mode: ALT9"]
    SelectGpioEmcB210Alt9 = 0x01,
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
    SelectGpioEmcB104Alt4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_16 for Mode: ALT9"]
    SelectGpioEmcB216Alt9 = 0x01,
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
    SelectGpioEmcB103Alt4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_17 for Mode: ALT9"]
    SelectGpioEmcB217Alt9 = 0x01,
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
    SelectGpioEmcB108Alt4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_08 for Mode: ALT9"]
    SelectGpioEmcB208Alt9 = 0x01,
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
    SelectGpioEmcB102Alt9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_08 for Mode: ALT1"]
    SelectGpioEmcB208Alt1 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_11 for Mode: ALT8"]
    SelectGpioB111Alt8 = 0x02,
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
    SelectGpioEmcB111Alt9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_19 for Mode: ALT1"]
    SelectGpioEmcB219Alt1 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_06 for Mode: ALT8"]
    SelectGpioB106Alt8 = 0x02,
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
    SelectGpioEmcB112Alt9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_20 for Mode: ALT1"]
    SelectGpioEmcB220Alt1 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_12 for Mode: ALT8"]
    SelectGpioB112Alt8 = 0x02,
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
    SelectGpioEmcB109Alt9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_17 for Mode: ALT1"]
    SelectGpioEmcB217Alt1 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_04 for Mode: ALT8"]
    SelectGpioB104Alt8 = 0x02,
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
    SelectGpioEmcB110Alt9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_18 for Mode: ALT1"]
    SelectGpioEmcB218Alt1 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_05 for Mode: ALT8"]
    SelectGpioB105Alt8 = 0x02,
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
    SelectGpioEmcB104Alt9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_10 for Mode: ALT1"]
    SelectGpioEmcB210Alt1 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_09 for Mode: ALT8"]
    SelectGpioB109Alt8 = 0x02,
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
    SelectGpioEmcB103Alt9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_09 for Mode: ALT1"]
    SelectGpioEmcB209Alt1 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_10 for Mode: ALT8"]
    SelectGpioB110Alt8 = 0x02,
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
    SelectGpioEmcB108Alt9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_16 for Mode: ALT1"]
    SelectGpioEmcB216Alt1 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_03 for Mode: ALT8"]
    SelectGpioB103Alt8 = 0x02,
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
    SelectGpioAd20Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_24 for Mode: ALT7"]
    SelectGpioAd24Alt7 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_32 for Mode: ALT7"]
    SelectGpioAd32Alt7 = 0x02,
    #[doc = "Selecting Pad: GPIO_SD_B2_11 for Mode: ALT8"]
    SelectGpioSdB211Alt8 = 0x03,
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
    SelectGpioAd21Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_25 for Mode: ALT7"]
    SelectGpioAd25Alt7 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_33 for Mode: ALT7"]
    SelectGpioAd33Alt7 = 0x02,
    #[doc = "Selecting Pad: GPIO_SD_B2_10 for Mode: ALT8"]
    SelectGpioSdB210Alt8 = 0x03,
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
    Ode0Disabled = 0x0,
    #[doc = "Enabled"]
    Ode1Enabled = 0x01,
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
    Pdrv0HighDriver = 0x0,
    #[doc = "normal driver"]
    Pdrv1NormalDriver = 0x01,
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
    Pue0PullDisable_highz = 0x0,
    #[doc = "Pull Enable"]
    Pue1PullEnable = 0x01,
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
    Pull0Forbidden = 0x0,
    #[doc = "PU"]
    Pull1Pu = 0x01,
    #[doc = "PD"]
    Pull2Pd = 0x02,
    #[doc = "No Pull"]
    Pull3NoPull = 0x03,
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
    Pus0WeakPullDown = 0x0,
    #[doc = "Weak pull up"]
    Pus1WeakPullUp = 0x01,
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
    SelectGpioEmcB118Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_09 for Mode: ALT10"]
    SelectGpioEmcB209Alt10 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_00 for Mode: ALT3"]
    SelectGpioB100Alt3 = 0x02,
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
    SelectGpioEmcB113Alt10 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_10 for Mode: ALT10"]
    SelectGpioEmcB210Alt10 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_01 for Mode: ALT3"]
    SelectGpioB101Alt3 = 0x02,
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
    SelectGpioEmcB211Alt10 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_02 for Mode: ALT3"]
    SelectGpioB102Alt3 = 0x01,
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
    SelectGpioEmcB119Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_13 for Mode: ALT10"]
    SelectGpioEmcB213Alt10 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_03 for Mode: ALT3"]
    SelectGpioB103Alt3 = 0x02,
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
    SelectGpioEmcB139Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_14 for Mode: ALT10"]
    SelectGpioEmcB214Alt10 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_04 for Mode: ALT3"]
    SelectGpioB104Alt3 = 0x02,
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
    SelectGpioEmcB215Alt10 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_05 for Mode: ALT3"]
    SelectGpioB105Alt3 = 0x01,
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
    SelectGpioEmcB120Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_17 for Mode: ALT10"]
    SelectGpioEmcB217Alt10 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_06 for Mode: ALT3"]
    SelectGpioB106Alt3 = 0x02,
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
    SelectGpioEmcB140Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_18 for Mode: ALT10"]
    SelectGpioEmcB218Alt10 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_07 for Mode: ALT3"]
    SelectGpioB107Alt3 = 0x02,
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
    SelectGpioEmcB219Alt10 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_08 for Mode: ALT3"]
    SelectGpioB108Alt3 = 0x01,
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
    SelectGpioEmcB121Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_00 for Mode: ALT9"]
    SelectGpioAd00Alt9 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_09 for Mode: ALT3"]
    SelectGpioB109Alt3 = 0x02,
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
    SelectGpioEmcB141Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_01 for Mode: ALT9"]
    SelectGpioAd01Alt9 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_10 for Mode: ALT3"]
    SelectGpioB110Alt3 = 0x02,
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
    SelectGpioAd02Alt9 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_11 for Mode: ALT3"]
    SelectGpioB111Alt3 = 0x01,
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
    SelectGpioEmcB122Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_04 for Mode: ALT9"]
    SelectGpioAd04Alt9 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_07 for Mode: ALT0"]
    SelectGpioB207Alt0 = 0x02,
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
    SelectGpioEmcB200Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_05 for Mode: ALT9"]
    SelectGpioAd05Alt9 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_08 for Mode: ALT0"]
    SelectGpioB208Alt0 = 0x02,
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
    SelectGpioAd06Alt9 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_09 for Mode: ALT0"]
    SelectGpioB209Alt0 = 0x01,
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
    SelectGpioEmcB123Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_08 for Mode: ALT9"]
    SelectGpioAd08Alt9 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_B2_01 for Mode: ALT3"]
    SelectGpioSdB201Alt3 = 0x02,
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
    SelectGpioEmcB201Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_09 for Mode: ALT9"]
    SelectGpioAd09Alt9 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_B2_02 for Mode: ALT3"]
    SelectGpioSdB202Alt3 = 0x02,
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
    SelectGpioAd10Alt9 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_03 for Mode: ALT3"]
    SelectGpioSdB203Alt3 = 0x01,
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
    SelectGpioEmcB124Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_04 for Mode: ALT3"]
    SelectGpioSdB204Alt3 = 0x01,
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
    SelectGpioEmcB202Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_05 for Mode: ALT3"]
    SelectGpioSdB205Alt3 = 0x01,
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
    SelectGpioEmcB125Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_08 for Mode: ALT3"]
    SelectGpioSdB208Alt3 = 0x01,
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
    SelectGpioEmcB203Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_09 for Mode: ALT3"]
    SelectGpioSdB209Alt3 = 0x01,
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
    SelectGpioAd17Alt0 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_05 for Mode: ALT12"]
    SelectGpioB105Alt12 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_00 for Mode: ALT4"]
    SelectGpioB200Alt4 = 0x02,
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
    SelectGpioAd19Alt0 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_06 for Mode: ALT12"]
    SelectGpioB106Alt12 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_05 for Mode: ALT4"]
    SelectGpioB205Alt4 = 0x02,
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
    SelectGpioAd20Alt0 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_01 for Mode: ALT12"]
    SelectGpioB101Alt12 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_06 for Mode: ALT4"]
    SelectGpioB206Alt4 = 0x02,
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
    SelectGpioB102Alt12 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_07 for Mode: ALT6"]
    SelectGpioB207Alt6 = 0x01,
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
    SelectGpioAd18Alt0 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_07 for Mode: ALT12"]
    SelectGpioB107Alt12 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_04 for Mode: ALT4"]
    SelectGpioB204Alt4 = 0x02,
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
    SelectGpioAd22Alt0 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_08 for Mode: ALT12"]
    SelectGpioB108Alt12 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_01 for Mode: ALT4"]
    SelectGpioB201Alt4 = 0x02,
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
    SelectGpioAd23Alt0 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_09 for Mode: ALT12"]
    SelectGpioB109Alt12 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_02 for Mode: ALT4"]
    SelectGpioB202Alt4 = 0x02,
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
    SelectGpioAd04Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_21 for Mode: ALT3"]
    SelectGpioAd21Alt3 = 0x01,
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
    SelectGpioAd06Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_23 for Mode: ALT3"]
    SelectGpioAd23Alt3 = 0x01,
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
    SelectGpioAd08Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B1_01 for Mode: ALT1"]
    SelectGpioSdB101Alt1 = 0x01,
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
    SelectGpioAd10Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B1_03 for Mode: ALT1"]
    SelectGpioSdB103Alt1 = 0x01,
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
    SelectGpioAd03Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_20 for Mode: ALT3"]
    SelectGpioAd20Alt3 = 0x01,
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
    SelectGpioAd05Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_22 for Mode: ALT3"]
    SelectGpioAd22Alt3 = 0x01,
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
    SelectGpioAd07Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B1_00 for Mode: ALT1"]
    SelectGpioSdB100Alt1 = 0x01,
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
    SelectGpioAd09Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B1_02 for Mode: ALT1"]
    SelectGpioSdB102Alt1 = 0x01,
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
    SelectGpioAd31Alt3 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B1_05 for Mode: ALT2"]
    SelectGpioSdB105Alt2 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_09 for Mode: ALT1"]
    SelectGpioB209Alt1 = 0x02,
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
    SelectGpioAd33Alt3 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_11 for Mode: ALT1"]
    SelectGpioB211Alt1 = 0x01,
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
    SelectGpioAd26Alt3 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_13 for Mode: ALT1"]
    SelectGpioB213Alt1 = 0x01,
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
    SelectGpioAd30Alt3 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B1_04 for Mode: ALT2"]
    SelectGpioSdB104Alt2 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_08 for Mode: ALT1"]
    SelectGpioB208Alt1 = 0x02,
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
    SelectGpioAd32Alt3 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_10 for Mode: ALT1"]
    SelectGpioB210Alt1 = 0x01,
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
    SelectGpioEmcB212Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_15 for Mode: ALT0"]
    SelectGpioAd15Alt0 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_08 for Mode: ALT9"]
    SelectGpioB208Alt9 = 0x02,
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
    Sre0FastSlewRate = 0x0,
    #[doc = "Slow Slew Rate"]
    Sre1SlowSlewRate = 0x01,
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
    Alt1Can2Tx = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: MIC_CLK of instance: mic"]
    Alt2MicClk = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: GPT2_CAPTURE1 of instance: gpt2"]
    Alt3Gpt2Capture1 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM1_PWMA00 of instance: flexpwm1"]
    Alt4Flexpwm1Pwma0 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO00 of instance: gpio4"]
    Alt5Gpio4Io0 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SINC1_MOD_CLK0 of instance: sinc1"]
    Alt6Sinc1ModClk0 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO00 of instance: flexio2"]
    Alt8Flexio2Flexio0 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: QTIMER4_TIMER0 of instance: qtimer4"]
    Alt9Qtimer4Timer0 = 0x09,
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
    Alt1Can2Rx = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: MIC_BITSTREAM00 of instance: mic"]
    Alt2MicBitstream0 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: GPT2_CAPTURE2 of instance: gpt2"]
    Alt3Gpt2Capture2 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM1_PWMB00 of instance: flexpwm1"]
    Alt4Flexpwm1Pwmb0 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO01 of instance: gpio4"]
    Alt5Gpio4Io1 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SINC1_MOD_CLK1 of instance: sinc1"]
    Alt6Sinc1ModClk1 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO01 of instance: flexio2"]
    Alt8Flexio2Flexio1 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: QTIMER4_TIMER1 of instance: qtimer4"]
    Alt9Qtimer4Timer1 = 0x09,
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
    Alt2MicBitstream1 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: GPT2_COMPARE1 of instance: gpt2"]
    Alt3Gpt2Compare1 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM1_PWMA01 of instance: flexpwm1"]
    Alt4Flexpwm1Pwma1 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO02 of instance: gpio4"]
    Alt5Gpio4Io2 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SINC1_MOD_CLK2 of instance: sinc1"]
    Alt6Sinc1ModClk2 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO02 of instance: flexio2"]
    Alt8Flexio2Flexio2 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: QTIMER4_TIMER2 of instance: qtimer4"]
    Alt9Qtimer4Timer2 = 0x09,
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
    Alt2MicBitstream2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: GPT2_COMPARE2 of instance: gpt2"]
    Alt3Gpt2Compare2 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM1_PWMB01 of instance: flexpwm1"]
    Alt4Flexpwm1Pwmb1 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO03 of instance: gpio4"]
    Alt5Gpio4Io3 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SINC1_EMCLK00 of instance: sinc1"]
    Alt6Sinc1Emclk0 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO03 of instance: flexio2"]
    Alt8Flexio2Flexio3 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: QTIMER4_TIMER3 of instance: qtimer4"]
    Alt9Qtimer4Timer3 = 0x09,
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
    Alt2MicBitstream3 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: GPT2_COMPARE3 of instance: gpt2"]
    Alt3Gpt2Compare3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM1_PWMB02 of instance: flexpwm1"]
    Alt4Flexpwm1Pwmb2 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO04 of instance: gpio4"]
    Alt5Gpio4Io4 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SINC1_EMBIT00 of instance: sinc1"]
    Alt6Sinc1Embit0 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO04 of instance: flexio2"]
    Alt8Flexio2Flexio4 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: QTIMER5_TIMER0 of instance: qtimer5"]
    Alt9Qtimer5Timer0 = 0x09,
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
    Alt3Gpt2Clk = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM1_PWMA02 of instance: flexpwm1"]
    Alt4Flexpwm1Pwma2 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO05 of instance: gpio4"]
    Alt5Gpio4Io5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SINC1_EMCLK01 of instance: sinc1"]
    Alt6Sinc1Emclk1 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: CCM_ENET_REF_CLK_25M of instance: ccm"]
    Alt7CcmEnetRefClk25m = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO05 of instance: flexio2"]
    Alt8Flexio2Flexio5 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: QTIMER5_TIMER1 of instance: qtimer5"]
    Alt9Qtimer5Timer1 = 0x09,
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
    Alt0UsbOtg2Oc = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: CAN3_TX of instance: can3"]
    Alt1Can3Tx = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM1_PWMX00 of instance: flexpwm1"]
    Alt4Flexpwm1Pwmx0 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO06 of instance: gpio4"]
    Alt5Gpio4Io6 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SINC1_EMBIT01 of instance: sinc1"]
    Alt6Sinc1Embit1 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO06 of instance: flexio2"]
    Alt8Flexio2Flexio6 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: QTIMER5_TIMER2 of instance: qtimer5"]
    Alt9Qtimer5Timer2 = 0x09,
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
    Alt0UsbOtg2Pwr = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: CAN3_RX of instance: can3"]
    Alt1Can3Rx = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM1_PWMX01 of instance: flexpwm1"]
    Alt4Flexpwm1Pwmx1 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO07 of instance: gpio4"]
    Alt5Gpio4Io7 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SINC1_EMCLK02 of instance: sinc1"]
    Alt6Sinc1Emclk2 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO07 of instance: flexio2"]
    Alt8Flexio2Flexio7 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: QTIMER5_TIMER3 of instance: qtimer5"]
    Alt9Qtimer5Timer3 = 0x09,
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
    Alt0Usbphy2OtgId = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPI2C5_SCL of instance: lpi2c5"]
    Alt1Lpi2c5Scl = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM1_PWMX02 of instance: flexpwm1"]
    Alt4Flexpwm1Pwmx2 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO08 of instance: gpio4"]
    Alt5Gpio4Io8 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SINC1_EMBIT02 of instance: sinc1"]
    Alt6Sinc1Embit2 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO08 of instance: flexio2"]
    Alt8Flexio2Flexio8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: QTIMER6_TIMER0 of instance: qtimer6"]
    Alt9Qtimer6Timer0 = 0x09,
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
    Alt0Usbphy1OtgId = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPI2C5_SDA of instance: lpi2c5"]
    Alt1Lpi2c5Sda = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM1_PWMX03 of instance: flexpwm1"]
    Alt4Flexpwm1Pwmx3 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO09 of instance: gpio4"]
    Alt5Gpio4Io9 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SINC1_EMCLK03 of instance: sinc1"]
    Alt6Sinc1Emclk3 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO09 of instance: flexio2"]
    Alt8Flexio2Flexio9 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: QTIMER6_TIMER1 of instance: qtimer6"]
    Alt9Qtimer6Timer1 = 0x09,
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
    Alt0UsbOtg1Pwr = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM2_PWMX00 of instance: flexpwm2"]
    Alt4Flexpwm2Pwmx0 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO10 of instance: gpio4"]
    Alt5Gpio4Io10 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SINC1_EMBIT03 of instance: sinc1"]
    Alt6Sinc1Embit3 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO10 of instance: flexio2"]
    Alt8Flexio2Flexio10 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: QTIMER6_TIMER2 of instance: qtimer6"]
    Alt9Qtimer6Timer2 = 0x09,
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
    Alt0UsbOtg1Oc = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM2_PWMX01 of instance: flexpwm2"]
    Alt4Flexpwm2Pwmx1 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO11 of instance: gpio4"]
    Alt5Gpio4Io11 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SINC_FILTER_GLUE1_BREAK of instance: sinc_filter_glue1"]
    Alt6SincFilterGlue1Break = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO11 of instance: flexio2"]
    Alt8Flexio2Flexio11 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: QTIMER6_TIMER3 of instance: qtimer6"]
    Alt9Qtimer6Timer3 = 0x09,
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
    Alt0SpdifLock = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPI2C5_SCLS of instance: lpi2c5"]
    Alt1Lpi2c5Scls = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: GPT1_CAPTURE1 of instance: gpt1"]
    Alt2Gpt1Capture1 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: KPP_ROW07 of instance: kpp"]
    Alt3KppRow7 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM2_PWMX02 of instance: flexpwm2"]
    Alt4Flexpwm2Pwmx2 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO12 of instance: gpio4"]
    Alt5Gpio4Io12 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_XBAR_INOUT18 of instance: xbar1"]
    Alt6Xbar1XbarInout18 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: EWM_EWM_OUT_B of instance: ewm"]
    Alt7EwmEwmOutB = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO12 of instance: flexio2"]
    Alt8Flexio2Flexio12 = 0x08,
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
    Alt0SpdifSrClk = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPI2C5_SDAS of instance: lpi2c5"]
    Alt1Lpi2c5Sdas = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: GPT1_CAPTURE2 of instance: gpt1"]
    Alt2Gpt1Capture2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: KPP_COL07 of instance: kpp"]
    Alt3KppCol7 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM2_PWMX03 of instance: flexpwm2"]
    Alt4Flexpwm2Pwmx3 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO13 of instance: gpio4"]
    Alt5Gpio4Io13 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPUART3_TX of instance: lpuart3"]
    Alt6Lpuart3Tx = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: USDHC2_CD_B of instance: usdhc2"]
    Alt7Usdhc2CdB = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO13 of instance: flexio2"]
    Alt8Flexio2Flexio13 = 0x08,
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
    Alt0SpdifExtClk = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPI2C5_HREQ of instance: lpi2c5"]
    Alt1Lpi2c5Hreq = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: GPT1_COMPARE1 of instance: gpt1"]
    Alt2Gpt1Compare1 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: KPP_ROW06 of instance: kpp"]
    Alt3KppRow6 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM3_PWMX00 of instance: flexpwm3"]
    Alt4Flexpwm3Pwmx0 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO14 of instance: gpio4"]
    Alt5Gpio4Io14 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPUART3_RX of instance: lpuart3"]
    Alt6Lpuart3Rx = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: USDHC2_WP of instance: usdhc2"]
    Alt7Usdhc2Wp = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO14 of instance: flexio2"]
    Alt8Flexio2Flexio14 = 0x08,
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
    Alt0SpdifIn = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPUART10_TX of instance: lpuart10"]
    Alt1Lpuart10Tx = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: GPT1_COMPARE2 of instance: gpt1"]
    Alt2Gpt1Compare2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: KPP_COL06 of instance: kpp"]
    Alt3KppCol6 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM3_PWMX01 of instance: flexpwm3"]
    Alt4Flexpwm3Pwmx1 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO15 of instance: gpio4"]
    Alt5Gpio4Io15 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPUART3_CTS_B of instance: lpuart3"]
    Alt6Lpuart3CtsB = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: LPSPI3_PCS1 of instance: lpspi3"]
    Alt7Lpspi3Pcs1 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO15 of instance: flexio2"]
    Alt8Flexio2Flexio15 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: CAN1_TX of instance: can1"]
    Alt9Can1Tx = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_CLK_ECAT_CLK25 of instance: ecat"]
    Alt12EcatClkEcatClk25 = 0x0c,
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
    Alt0SpdifOut = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPUART10_RX of instance: lpuart10"]
    Alt1Lpuart10Rx = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: GPT1_COMPARE3 of instance: gpt1"]
    Alt2Gpt1Compare3 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: KPP_ROW05 of instance: kpp"]
    Alt3KppRow5 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM3_PWMX02 of instance: flexpwm3"]
    Alt4Flexpwm3Pwmx2 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO16 of instance: gpio4"]
    Alt5Gpio4Io16 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPUART3_RTS_B of instance: lpuart3"]
    Alt6Lpuart3RtsB = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: LPSPI3_SCK of instance: lpspi3"]
    Alt7Lpspi3Sck = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO16 of instance: flexio2"]
    Alt8Flexio2Flexio16 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: CAN1_RX of instance: can1"]
    Alt9Can1Rx = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_LINK_0 of instance: ecat"]
    Alt12EcatLink0 = 0x0c,
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
    Alt0Sai4Mclk = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: ACMP1_CMPO of instance: acmp1"]
    Alt1Acmp1Cmpo = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: GPT1_CLK of instance: gpt1"]
    Alt2Gpt1Clk = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: KPP_COL05 of instance: kpp"]
    Alt3KppCol5 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM3_PWMX03 of instance: flexpwm3"]
    Alt4Flexpwm3Pwmx3 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO17 of instance: gpio4"]
    Alt5Gpio4Io17 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: I3C2_PUR of instance: i3c2"]
    Alt6I3c2Pur = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: LPSPI3_PCS0 of instance: lpspi3"]
    Alt7Lpspi3Pcs0 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO17 of instance: flexio2"]
    Alt8Flexio2Flexio17 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPI2C3_HREQ of instance: lpi2c3"]
    Alt9Lpi2c3Hreq = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_LINK_1 of instance: ecat"]
    Alt12EcatLink1 = 0x0c,
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
    Alt0Sai4RxSync = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: ACMP2_CMPO of instance: acmp2"]
    Alt1Acmp2Cmpo = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART5_RI_B of instance: lpuart5"]
    Alt2Lpuart5RiB = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: KPP_ROW04 of instance: kpp"]
    Alt3KppRow4 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM4_PWMX00 of instance: flexpwm4"]
    Alt4Flexpwm4Pwmx0 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO18 of instance: gpio4"]
    Alt5Gpio4Io18 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: I3C2_SCL of instance: i3c2"]
    Alt6I3c2Scl = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: LPSPI3_SDO of instance: lpspi3"]
    Alt7Lpspi3Sdo = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO18 of instance: flexio2"]
    Alt8Flexio2Flexio18 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPI2C3_SCL of instance: lpi2c3"]
    Alt9Lpi2c3Scl = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_PROM_CLK of instance: ecat"]
    Alt12EcatScl = 0x0c,
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
    Alt0Sai4RxBclk = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: ACMP3_CMPO of instance: acmp3"]
    Alt1Acmp3Cmpo = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XBAR1_XBAR_INOUT19 of instance: xbar1"]
    Alt2Xbar1XbarInout19 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: KPP_COL04 of instance: kpp"]
    Alt3KppCol4 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM4_PWMX01 of instance: flexpwm4"]
    Alt4Flexpwm4Pwmx1 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO19 of instance: gpio4"]
    Alt5Gpio4Io19 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: I3C2_SDA of instance: i3c2"]
    Alt6I3c2Sda = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: LPSPI3_SDI of instance: lpspi3"]
    Alt7Lpspi3Sdi = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO19 of instance: flexio2"]
    Alt8Flexio2Flexio19 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPI2C3_SDA of instance: lpi2c3"]
    Alt9Lpi2c3Sda = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_PROM_DATA of instance: ecat"]
    Alt12EcatSda = 0x0c,
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
    Alt0Sai4RxData0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: ACMP4_CMPO of instance: acmp4"]
    Alt1Acmp4Cmpo = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPIT2_TRIGGER00 of instance: lpit2"]
    Alt2Lpit2Trigger0 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SINC1_EMCLK00 of instance: sinc1"]
    Alt3Sinc1Emclk0 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM4_PWMX02 of instance: flexpwm4"]
    Alt4Flexpwm4Pwmx2 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO20 of instance: gpio4"]
    Alt5Gpio4Io20 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: NETC_TMR_TRIG1 of instance: netc"]
    Alt6NetcTmr1588Trig1 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: NETC_1588_CLK of instance: netc_clkgen"]
    Alt7NetcClkgenTmr1588Clk = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO20 of instance: flexio2"]
    Alt8Flexio2Flexio20 = 0x08,
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
    Alt0Sai4TxData0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPIT2_TRIGGER01 of instance: lpit2"]
    Alt2Lpit2Trigger1 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SINC1_EMBIT00 of instance: sinc1"]
    Alt3Sinc1Embit0 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM4_PWMX03 of instance: flexpwm4"]
    Alt4Flexpwm4Pwmx3 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO21 of instance: gpio4"]
    Alt5Gpio4Io21 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: NETC_TMR_TRIG2 of instance: netc"]
    Alt6NetcTmr1588Trig2 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: NETC_TMR_GCLK of instance: netc"]
    Alt7NetcTmr1588Gclk = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO21 of instance: flexio2"]
    Alt8Flexio2Flexio21 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_LED_RUN of instance: ecat"]
    Alt12EcatLedRun = 0x0c,
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
    Alt0Sai4TxBclk = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPIT2_TRIGGER02 of instance: lpit2"]
    Alt2Lpit2Trigger2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SINC1_EMCLK01 of instance: sinc1"]
    Alt3Sinc1Emclk1 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO22 of instance: gpio4"]
    Alt5Gpio4Io22 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: NETC_TMR_ALARM1 of instance: netc"]
    Alt7NetcTmr1588Alarm1 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO22 of instance: flexio2"]
    Alt8Flexio2Flexio22 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_LED_ERR of instance: ecat"]
    Alt12EcatLedErr = 0x0c,
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
    Alt0Sai4TxSync = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPIT2_TRIGGER03 of instance: lpit2"]
    Alt2Lpit2Trigger3 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SINC1_EMBIT01 of instance: sinc1"]
    Alt3Sinc1Embit1 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO23 of instance: gpio4"]
    Alt5Gpio4Io23 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: NETC_TMR_ALARM2 of instance: netc"]
    Alt7NetcTmr1588Alarm2 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO23 of instance: flexio2"]
    Alt8Flexio2Flexio23 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_LED_STATE_RUN of instance: ecat"]
    Alt12EcatLedStateRun = 0x0c,
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
    Alt0Lpuart6Tx = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPI2C4_SCL of instance: lpi2c4"]
    Alt1Lpi2c4Scl = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SINC2_MOD_CLK1 of instance: sinc2"]
    Alt3Sinc2ModClk1 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM2_PWMA00 of instance: flexpwm2"]
    Alt4Flexpwm2Pwma0 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO24 of instance: gpio4"]
    Alt5Gpio4Io24 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: NETC_TMR_TRIG1 of instance: netc"]
    Alt7NetcTmr1588Trig1 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO24 of instance: flexio2"]
    Alt8Flexio2Flexio24 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_LINK_ACT00 of instance: ecat"]
    Alt12EcatLinkAct0 = 0x0c,
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
    Alt0Lpuart6Rx = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPI2C4_SDA of instance: lpi2c4"]
    Alt1Lpi2c4Sda = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPSPI5_PCS3 of instance: lpspi5"]
    Alt2Lpspi5Pcs3 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SINC2_MOD_CLK2 of instance: sinc2"]
    Alt3Sinc2ModClk2 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM2_PWMB00 of instance: flexpwm2"]
    Alt4Flexpwm2Pwmb0 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO25 of instance: gpio4"]
    Alt5Gpio4Io25 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: NETC_TMR_TRIG2 of instance: netc"]
    Alt7NetcTmr1588Trig2 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO25 of instance: flexio2"]
    Alt8Flexio2Flexio25 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_LINK_ACT01 of instance: ecat"]
    Alt12EcatLinkAct1 = 0x0c,
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
    Alt0Lpuart6CtsB = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPUART5_TX of instance: lpuart5"]
    Alt1Lpuart5Tx = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPSPI5_PCS2 of instance: lpspi5"]
    Alt2Lpspi5Pcs2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SINC2_EMCLK00 of instance: sinc2"]
    Alt3Sinc2Emclk0 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM2_PWMA01 of instance: flexpwm2"]
    Alt4Flexpwm2Pwma1 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO26 of instance: gpio4"]
    Alt5Gpio4Io26 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_ROW00 of instance: kpp"]
    Alt6KppRow0 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: NETC_TMR_PP1 of instance: netc"]
    Alt7NetcTmr1588Pp1 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO26 of instance: flexio2"]
    Alt8Flexio2Flexio26 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: USDHC2_CD_B of instance: usdhc2"]
    Alt9Usdhc2CdB = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: MIC_BITSTREAM02 of instance: mic"]
    Alt12MicBitstream2 = 0x0c,
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
    Alt0Lpuart6RtsB = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: LPUART5_RX of instance: lpuart5"]
    Alt1Lpuart5Rx = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPSPI5_PCS1 of instance: lpspi5"]
    Alt2Lpspi5Pcs1 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SINC2_EMBIT00 of instance: sinc2"]
    Alt3Sinc2Embit0 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM2_PWMB01 of instance: flexpwm2"]
    Alt4Flexpwm2Pwmb1 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO27 of instance: gpio4"]
    Alt5Gpio4Io27 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_COL00 of instance: kpp"]
    Alt6KppCol0 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: NETC_TMR_PP2 of instance: netc"]
    Alt7NetcTmr1588Pp2 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO27 of instance: flexio2"]
    Alt8Flexio2Flexio27 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: USDHC2_WP of instance: usdhc2"]
    Alt9Usdhc2Wp = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: MIC_CLK of instance: mic"]
    Alt12MicClk = 0x0c,
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
    Alt0Lpspi5Sck = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: I3C1_PUR of instance: i3c1"]
    Alt2I3c1Pur = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SINC2_EMCLK01 of instance: sinc2"]
    Alt3Sinc2Emclk1 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM2_PWMB02 of instance: flexpwm2"]
    Alt4Flexpwm2Pwmb2 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO28 of instance: gpio4"]
    Alt5Gpio4Io28 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_ROW03 of instance: kpp"]
    Alt6KppRow3 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: NETC_TMR_PP3 of instance: netc"]
    Alt7NetcTmr1588Pp3 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO28 of instance: flexio2"]
    Alt8Flexio2Flexio28 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: USDHC2_RESET_B of instance: usdhc2"]
    Alt9Usdhc2ResetB = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: MIC_BITSTREAM00 of instance: mic"]
    Alt12MicBitstream0 = 0x0c,
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
    Alt0Lpspi5Pcs0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: USDHC2_CD_B of instance: usdhc2"]
    Alt2Usdhc2CdB = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SINC2_EMBIT01 of instance: sinc2"]
    Alt3Sinc2Embit1 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM2_PWMA02 of instance: flexpwm2"]
    Alt4Flexpwm2Pwma2 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO29 of instance: gpio4"]
    Alt5Gpio4Io29 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_COL03 of instance: kpp"]
    Alt6KppCol3 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: EWM_EWM_OUT_B of instance: ewm"]
    Alt7EwmEwmOutB = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO29 of instance: flexio2"]
    Alt8Flexio2Flexio29 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: USDHC2_VSELECT of instance: usdhc2"]
    Alt9Usdhc2Vselect = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: MIC_BITSTREAM01 of instance: mic"]
    Alt12MicBitstream1 = 0x0c,
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
    Alt0Lpspi5Sdo = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: USB_OTG2_OC of instance: usb"]
    Alt1UsbOtg2Oc = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: CAN2_TX of instance: can2"]
    Alt2Can2Tx = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SINC2_EMCLK02 of instance: sinc2"]
    Alt3Sinc2Emclk2 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPUART8_TX of instance: lpuart8"]
    Alt4Lpuart8Tx = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO30 of instance: gpio4"]
    Alt5Gpio4Io30 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_ROW02 of instance: kpp"]
    Alt6KppRow2 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: NETC_EMDC of instance: netc"]
    Alt7NetcEmdc = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO30 of instance: flexio2"]
    Alt8Flexio2Flexio30 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: XBAR1_XBAR_INOUT24 of instance: xbar1"]
    Alt9Xbar1XbarInout24 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_MCLK of instance: ecat"]
    Alt12EcatMdc = 0x0c,
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
    Alt0Lpspi5Sdi = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: USB_OTG2_PWR of instance: usb"]
    Alt1UsbOtg2Pwr = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: CAN2_RX of instance: can2"]
    Alt2Can2Rx = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SINC2_EMBIT02 of instance: sinc2"]
    Alt3Sinc2Embit2 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPUART8_RX of instance: lpuart8"]
    Alt4Lpuart8Rx = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO4_IO31 of instance: gpio4"]
    Alt5Gpio4Io31 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_COL02 of instance: kpp"]
    Alt6KppCol2 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: NETC_EMDIO of instance: netc"]
    Alt7NetcEmdio = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO2_FLEXIO31 of instance: flexio2"]
    Alt8Flexio2Flexio31 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: XBAR1_XBAR_INOUT25 of instance: xbar1"]
    Alt9Xbar1XbarInout25 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_MDIO of instance: ecat"]
    Alt12EcatMdio = 0x0c,
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
    Alt0Lpi2c5Scl = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: USBPHY2_OTG_ID of instance: usbphy2"]
    Alt1Usbphy2OtgId = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: GPC_PMIC_RDY of instance: gpc"]
    Alt2GpcPmicRdy = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SINC2_EMCLK03 of instance: sinc2"]
    Alt3Sinc2Emclk3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: USDHC1_CD_B of instance: usdhc1"]
    Alt4Usdhc1CdB = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO00 of instance: gpio5"]
    Alt5Gpio5Io0 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_ROW01 of instance: kpp"]
    Alt6KppRow1 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: NETC_TMR_TRIG1 of instance: netc"]
    Alt7NetcTmr1588Trig1 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPUART10_TX of instance: lpuart10"]
    Alt8Lpuart10Tx = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: MIC_BITSTREAM03 of instance: mic"]
    Alt12MicBitstream3 = 0x0c,
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
    Alt0Lpi2c5Sda = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: USBPHY1_OTG_ID of instance: usbphy1"]
    Alt1Usbphy1OtgId = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XBAR1_XBAR_INOUT17 of instance: xbar1"]
    Alt2Xbar1XbarInout17 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SINC2_EMBIT03 of instance: sinc2"]
    Alt3Sinc2Embit3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: USDHC1_WP of instance: usdhc1"]
    Alt4Usdhc1Wp = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO01 of instance: gpio5"]
    Alt5Gpio5Io1 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_COL01 of instance: kpp"]
    Alt6KppCol1 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: NETC_TMR_TRIG2 of instance: netc"]
    Alt7NetcTmr1588Trig2 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPUART10_RX of instance: lpuart10"]
    Alt8Lpuart10Rx = 0x08,
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
    Alt0I3c2Scl = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: USB_OTG1_PWR of instance: usb"]
    Alt1UsbOtg1Pwr = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XBAR1_XBAR_INOUT18 of instance: xbar1"]
    Alt2Xbar1XbarInout18 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SINC_FILTER_GLUE2_BREAK of instance: sinc_filter_glue2"]
    Alt3SincFilterGlue2Break = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: USDHC1_VSELECT of instance: usdhc1"]
    Alt4Usdhc1Vselect = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO02 of instance: gpio5"]
    Alt5Gpio5Io2 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: NETC_TMR_ALARM1 of instance: netc"]
    Alt7NetcTmr1588Alarm1 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPUART10_CTS_B of instance: lpuart10"]
    Alt8Lpuart10CtsB = 0x08,
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
    Alt0I3c2Sda = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: USB_OTG1_OC of instance: usb"]
    Alt1UsbOtg1Oc = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XBAR1_XBAR_INOUT19 of instance: xbar1"]
    Alt2Xbar1XbarInout19 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: SINC2_MOD_CLK0 of instance: sinc2"]
    Alt3Sinc2ModClk0 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: USDHC1_RESET_B of instance: usdhc1"]
    Alt4Usdhc1ResetB = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO03 of instance: gpio5"]
    Alt5Gpio5Io3 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: NETC_TMR_ALARM2 of instance: netc"]
    Alt7NetcTmr1588Alarm2 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPUART10_RTS_B of instance: lpuart10"]
    Alt8Lpuart10RtsB = 0x08,
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
    Alt0NetcPinmuxEth1Txd0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: ADC2_CONV_D00 of instance: adc2"]
    Alt1Adc2ConvD0 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SEMC_CSX01 of instance: semc"]
    Alt2SemcCsx1 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER1_TIMER0 of instance: qtimer1"]
    Alt3Qtimer1Timer0 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: XBAR1_XBAR_INOUT26 of instance: xbar1"]
    Alt4Xbar1XbarInout26 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO00 of instance: gpio6"]
    Alt5Gpio6Io0 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: TPM5_CH00 of instance: tpm5"]
    Alt6Tpm5Ch0 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_PINMUX_ETH4_TXD00 of instance: netc_pinmux"]
    Alt8NetcPinmuxEth4Txd0 = 0x08,
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
    Alt0NetcPinmuxEth1Txd1 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: ADC2_CONV_D01 of instance: adc2"]
    Alt1Adc2ConvD1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SEMC_CSX02 of instance: semc"]
    Alt2SemcCsx2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER1_TIMER1 of instance: qtimer1"]
    Alt3Qtimer1Timer1 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: XBAR1_XBAR_INOUT27 of instance: xbar1"]
    Alt4Xbar1XbarInout27 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO01 of instance: gpio6"]
    Alt5Gpio6Io1 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: TPM5_CH01 of instance: tpm5"]
    Alt6Tpm5Ch1 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_PINMUX_ETH4_TXD01 of instance: netc_pinmux"]
    Alt8NetcPinmuxEth4Txd1 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: SAI4_RX_DATA00 of instance: sai4"]
    Alt12Sai4RxData0 = 0x0c,
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
    Alt0NetcPinmuxEth1TxEn = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: ADC2_CONV_D02 of instance: adc2"]
    Alt1Adc2ConvD2 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPI2C6_SCL of instance: lpi2c6"]
    Alt2Lpi2c6Scl = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER1_TIMER2 of instance: qtimer1"]
    Alt3Qtimer1Timer2 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: XBAR1_XBAR_INOUT28 of instance: xbar1"]
    Alt4Xbar1XbarInout28 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO02 of instance: gpio6"]
    Alt5Gpio6Io2 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: TPM5_CH02 of instance: tpm5"]
    Alt6Tpm5Ch2 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_B_SS1_B of instance: flexspi1_bus2bit"]
    Alt7Flexspi1Bus2bitBSs1B = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_PINMUX_ETH4_TX_EN of instance: netc_pinmux"]
    Alt8NetcPinmuxEth4TxEn = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPUART11_TX of instance: lpuart11"]
    Alt9Lpuart11Tx = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: SAI4_RX_DATA01 of instance: sai4"]
    Alt12Sai4RxData1 = 0x0c,
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
    Alt0NetcPinmuxEth1TxClk = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: ADC2_CONV_D03 of instance: adc2"]
    Alt1Adc2ConvD3 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPI2C6_SDA of instance: lpi2c6"]
    Alt2Lpi2c6Sda = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER2_TIMER0 of instance: qtimer2"]
    Alt3Qtimer2Timer0 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: XBAR1_XBAR_INOUT29 of instance: xbar1"]
    Alt4Xbar1XbarInout29 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO03 of instance: gpio6"]
    Alt5Gpio6Io3 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: TPM5_CH03 of instance: tpm5"]
    Alt6Tpm5Ch3 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_B_DQS of instance: flexspi1_bus2bit"]
    Alt7Flexspi1Bus2bitBDqs = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_PINMUX_ETH4_TX_CLK of instance: netc_pinmux"]
    Alt8NetcPinmuxEth4TxClk = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPUART11_RX of instance: lpuart11"]
    Alt9Lpuart11Rx = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: SAI4_RX_DATA02 of instance: sai4"]
    Alt12Sai4RxData2 = 0x0c,
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
    Alt0NetcPinmuxEth1Rxd0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: ADC2_CONV_D04 of instance: adc2"]
    Alt1Adc2ConvD4 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART9_RX of instance: lpuart9"]
    Alt2Lpuart9Rx = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER2_TIMER1 of instance: qtimer2"]
    Alt3Qtimer2Timer1 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: XBAR1_XBAR_INOUT30 of instance: xbar1"]
    Alt4Xbar1XbarInout30 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO04 of instance: gpio6"]
    Alt5Gpio6Io4 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: TPM5_EXTCLK of instance: tpm5"]
    Alt6Tpm5Extclk = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_B_SS0_B of instance: flexspi1_bus2bit"]
    Alt7Flexspi1Bus2bitBSs0B = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_PINMUX_ETH4_RXD00 of instance: netc_pinmux"]
    Alt8NetcPinmuxEth4Rxd0 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: SAI4_RX_DATA03 of instance: sai4"]
    Alt12Sai4RxData3 = 0x0c,
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
    Alt0NetcPinmuxEth1Rxd1 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: ADC2_CONV_D05 of instance: adc2"]
    Alt1Adc2ConvD5 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART9_CTS_B of instance: lpuart9"]
    Alt2Lpuart9CtsB = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER2_TIMER2 of instance: qtimer2"]
    Alt3Qtimer2Timer2 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: XBAR1_XBAR_INOUT31 of instance: xbar1"]
    Alt4Xbar1XbarInout31 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO05 of instance: gpio6"]
    Alt5Gpio6Io5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: TPM6_EXTCLK of instance: tpm6"]
    Alt6Tpm6Extclk = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_B_SCLK of instance: flexspi1_bus2bit"]
    Alt7Flexspi1Bus2bitBSclk = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_PINMUX_ETH4_RXD01 of instance: netc_pinmux"]
    Alt8NetcPinmuxEth4Rxd1 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: SAI4_MCLK of instance: sai4"]
    Alt12Sai4Mclk = 0x0c,
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
    Alt0NetcPinmuxEth1RxDv = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: ADC2_CONV_D06 of instance: adc2"]
    Alt1Adc2ConvD6 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART9_TX of instance: lpuart9"]
    Alt2Lpuart9Tx = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER3_TIMER0 of instance: qtimer3"]
    Alt3Qtimer3Timer0 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: XBAR1_XBAR_INOUT32 of instance: xbar1"]
    Alt4Xbar1XbarInout32 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO06 of instance: gpio6"]
    Alt5Gpio6Io6 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: TPM6_CH00 of instance: tpm6"]
    Alt6Tpm6Ch0 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_B_DATA07 of instance: flexspi1_bus2bit"]
    Alt7Flexspi1Bus2bitBData7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_PINMUX_ETH4_RX_DV of instance: netc_pinmux"]
    Alt8NetcPinmuxEth4RxDv = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: SAI4_RX_BCLK of instance: sai4"]
    Alt12Sai4RxBclk = 0x0c,
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
    Alt0NetcPinmuxEth1Txd2 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: ADC2_CONV_D07 of instance: adc2"]
    Alt1Adc2ConvD7 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART9_RTS_B of instance: lpuart9"]
    Alt2Lpuart9RtsB = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER3_TIMER1 of instance: qtimer3"]
    Alt3Qtimer3Timer1 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: XBAR1_XBAR_INOUT33 of instance: xbar1"]
    Alt4Xbar1XbarInout33 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO07 of instance: gpio6"]
    Alt5Gpio6Io7 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: TPM6_CH01 of instance: tpm6"]
    Alt6Tpm6Ch1 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_B_DATA06 of instance: flexspi1_bus2bit"]
    Alt7Flexspi1Bus2bitBData6 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_PINMUX_ETH4_TXD02 of instance: netc_pinmux"]
    Alt8NetcPinmuxEth4Txd2 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPSPI6_SDI of instance: lpspi6"]
    Alt9Lpspi6Sdi = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: SAI4_RX_SYNC of instance: sai4"]
    Alt12Sai4RxSync = 0x0c,
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
    Alt0NetcPinmuxEth1Txd3 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: ADC2_CONV_RDY_CLK of instance: adc2"]
    Alt1Adc2ConvRdyClk = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: USDHC1_CD_B of instance: usdhc1"]
    Alt2Usdhc1CdB = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER3_TIMER2 of instance: qtimer3"]
    Alt3Qtimer3Timer2 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: XBAR1_XBAR_INOUT36 of instance: xbar1"]
    Alt4Xbar1XbarInout36 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO08 of instance: gpio6"]
    Alt5Gpio6Io8 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: TPM6_CH02 of instance: tpm6"]
    Alt6Tpm6Ch2 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_B_DATA05 of instance: flexspi1_bus2bit"]
    Alt7Flexspi1Bus2bitBData5 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_PINMUX_ETH4_TXD03 of instance: netc_pinmux"]
    Alt8NetcPinmuxEth4Txd3 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPSPI6_SDO of instance: lpspi6"]
    Alt9Lpspi6Sdo = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: SAI4_TX_BCLK of instance: sai4"]
    Alt12Sai4TxBclk = 0x0c,
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
    Alt0NetcPinmuxEth1Rxd2 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: USDHC1_WP of instance: usdhc1"]
    Alt2Usdhc1Wp = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER4_TIMER0 of instance: qtimer4"]
    Alt3Qtimer4Timer0 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: XBAR1_XBAR_INOUT37 of instance: xbar1"]
    Alt4Xbar1XbarInout37 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO09 of instance: gpio6"]
    Alt5Gpio6Io9 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: TPM6_CH03 of instance: tpm6"]
    Alt6Tpm6Ch3 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_B_DATA04 of instance: flexspi1_bus2bit"]
    Alt7Flexspi1Bus2bitBData4 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_PINMUX_ETH4_RXD02 of instance: netc_pinmux"]
    Alt8NetcPinmuxEth4Rxd2 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPSPI6_PCS1 of instance: lpspi6"]
    Alt9Lpspi6Pcs1 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: SAI4_TX_SYNC of instance: sai4"]
    Alt12Sai4TxSync = 0x0c,
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
    Alt0NetcPinmuxEth1Rxd3 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: USDHC1_RESET_B of instance: usdhc1"]
    Alt2Usdhc1ResetB = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER4_TIMER1 of instance: qtimer4"]
    Alt3Qtimer4Timer1 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: XBAR1_XBAR_INOUT34 of instance: xbar1"]
    Alt4Xbar1XbarInout34 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO10 of instance: gpio6"]
    Alt5Gpio6Io10 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_B_DATA03 of instance: flexspi1_bus2bit"]
    Alt7Flexspi1Bus2bitBData3 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_PINMUX_ETH4_RXD03 of instance: netc_pinmux"]
    Alt8NetcPinmuxEth4Rxd3 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPSPI6_PCS2 of instance: lpspi6"]
    Alt9Lpspi6Pcs2 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: SAI4_TX_DATA00 of instance: sai4"]
    Alt12Sai4TxData0 = 0x0c,
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
    Alt0NetcPinmuxEth1RxClk = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER4_TIMER2 of instance: qtimer4"]
    Alt3Qtimer4Timer2 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: XBAR1_XBAR_INOUT35 of instance: xbar1"]
    Alt4Xbar1XbarInout35 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO11 of instance: gpio6"]
    Alt5Gpio6Io11 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_B_DATA02 of instance: flexspi1_bus2bit"]
    Alt7Flexspi1Bus2bitBData2 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_PINMUX_ETH4_RX_CLK of instance: netc_pinmux"]
    Alt8NetcPinmuxEth4RxClk = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPSPI6_PCS3 of instance: lpspi6"]
    Alt9Lpspi6Pcs3 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: SAI4_TX_DATA01 of instance: sai4"]
    Alt12Sai4TxData1 = 0x0c,
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
    Alt0NetcPinmuxEth1RxEr = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: NETC_EMDIO of instance: netc"]
    Alt1NetcEmdio = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO12 of instance: gpio6"]
    Alt5Gpio6Io12 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_B_DATA01 of instance: flexspi1_bus2bit"]
    Alt7Flexspi1Bus2bitBData1 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_PINMUX_ETH4_RX_ER of instance: netc_pinmux"]
    Alt8NetcPinmuxEth4RxEr = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPSPI6_PCS0 of instance: lpspi6"]
    Alt9Lpspi6Pcs0 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: SAI4_TX_DATA02 of instance: sai4"]
    Alt12Sai4TxData2 = 0x0c,
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
    Alt0NetcPinmuxEth1TxEr = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: NETC_EMDC of instance: netc"]
    Alt1NetcEmdc = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: USDHC1_VSELECT of instance: usdhc1"]
    Alt2Usdhc1Vselect = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: CCM_ENET_REF_CLK_25M of instance: ccm"]
    Alt3CcmEnetRefClk25m = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO13 of instance: gpio6"]
    Alt5Gpio6Io13 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_B_DATA00 of instance: flexspi1_bus2bit"]
    Alt7Flexspi1Bus2bitBData0 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_PINMUX_ETH4_TX_ER of instance: netc_pinmux"]
    Alt8NetcPinmuxEth4TxEr = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPSPI6_SCK of instance: lpspi6"]
    Alt9Lpspi6Sck = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: SAI4_TX_DATA03 of instance: sai4"]
    Alt12Sai4TxData3 = 0x0c,
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
    Alt0NetcEth1Crs = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: SEMC_CSX03 of instance: semc"]
    Alt1SemcCsx3 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPIT3_TRIGGER00 of instance: lpit3"]
    Alt2Lpit3Trigger0 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: SAI4_MCLK of instance: sai4"]
    Alt4Sai4Mclk = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO14 of instance: gpio6"]
    Alt5Gpio6Io14 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_ETH4_CRS of instance: netc"]
    Alt8NetcEth4Crs = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPSPI6_SDI of instance: lpspi6"]
    Alt9Lpspi6Sdi = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: NETC_ETH2_SLV_MDIO of instance: netc"]
    Alt10NetcEth2SlvMdio = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_CLK_ECAT_CLK25 of instance: ecat"]
    Alt12EcatClkEcatClk25 = 0x0c,
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
    Alt0NetcEth1Col = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPIT3_TRIGGER01 of instance: lpit3"]
    Alt2Lpit3Trigger1 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: SAI4_TX_BCLK of instance: sai4"]
    Alt4Sai4TxBclk = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO15 of instance: gpio6"]
    Alt5Gpio6Io15 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: FLEXSPI1_BUS2BIT_A_SS1_B of instance: flexspi1_bus2bit"]
    Alt6Flexspi1Bus2bitASs1B = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_ETH4_COL of instance: netc"]
    Alt8NetcEth4Col = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPSPI6_SDO of instance: lpspi6"]
    Alt9Lpspi6Sdo = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: NETC_ETH2_SLV_MDC of instance: netc"]
    Alt10NetcEth2SlvMdc = 0x0a,
    #[doc = "Select mux mode: ALT11 mux port: NETC_PINMUX_ETH2_RX_ER of instance: netc_pinmux"]
    Alt11NetcPinmuxEth2RxEr = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_RX_ER_1 of instance: ecat"]
    Alt12EcatPt1RxEr = 0x0c,
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
    Alt2Lpit3Trigger2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_EMDIO of instance: netc"]
    Alt3NetcEmdio = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: SAI4_TX_SYNC of instance: sai4"]
    Alt4Sai4TxSync = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO16 of instance: gpio6"]
    Alt5Gpio6Io16 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: FLEXSPI1_BUS2BIT_B_SCLK of instance: flexspi1_bus2bit"]
    Alt6Flexspi1Bus2bitBSclk = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: CCM_ENET_REF_CLK_25M of instance: ccm"]
    Alt9CcmEnetRefClk25m = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: EWM_EWM_OUT_B of instance: ewm"]
    Alt10EwmEwmOutB = 0x0a,
    #[doc = "Select mux mode: ALT11 mux port: NETC_PINMUX_ETH2_RXD02 of instance: netc_pinmux"]
    Alt11NetcPinmuxEth2Rxd2 = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_RX_DATA2_1 of instance: ecat"]
    Alt12EcatPt1Rxd2 = 0x0c,
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
    Alt2Lpit3Trigger3 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_EMDC of instance: netc"]
    Alt3NetcEmdc = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: SAI4_TX_DATA00 of instance: sai4"]
    Alt4Sai4TxData0 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO17 of instance: gpio6"]
    Alt5Gpio6Io17 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_A_DATA04 of instance: flexspi1_bus2bit"]
    Alt7Flexspi1Bus2bitAData4 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: XSPI_SLV_DATA04 of instance: xspi_slv"]
    Alt10XspiSlvData4 = 0x0a,
    #[doc = "Select mux mode: ALT11 mux port: NETC_PINMUX_ETH2_RXD03 of instance: netc_pinmux"]
    Alt11NetcPinmuxEth2Rxd3 = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_RX_DATA3_1 of instance: ecat"]
    Alt12EcatPt1Rxd3 = 0x0c,
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
    Alt0Sinc1ModClk0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: SINC2_MOD_CLK0 of instance: sinc2"]
    Alt1Sinc2ModClk0 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SINC3_MOD_CLK0 of instance: sinc3"]
    Alt2Sinc3ModClk0 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: SAI4_RX_SYNC of instance: sai4"]
    Alt4Sai4RxSync = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO18 of instance: gpio6"]
    Alt5Gpio6Io18 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_A_DATA05 of instance: flexspi1_bus2bit"]
    Alt7Flexspi1Bus2bitAData5 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: TPM3_EXTCLK of instance: tpm3"]
    Alt8Tpm3Extclk = 0x08,
    _RESERVED_9 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: XSPI_SLV_DATA05 of instance: xspi_slv"]
    Alt10XspiSlvData5 = 0x0a,
    #[doc = "Select mux mode: ALT11 mux port: NETC_PINMUX_ETH2_TXD02 of instance: netc_pinmux"]
    Alt11NetcPinmuxEth2Txd2 = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_TX_DATA2_1 of instance: ecat"]
    Alt12EcatPt1Txd2 = 0x0c,
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
    Alt0Sinc1ModClk1 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: SINC2_MOD_CLK1 of instance: sinc2"]
    Alt1Sinc2ModClk1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SINC3_MOD_CLK1 of instance: sinc3"]
    Alt2Sinc3ModClk1 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: SAI4_RX_BCLK of instance: sai4"]
    Alt4Sai4RxBclk = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO19 of instance: gpio6"]
    Alt5Gpio6Io19 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: MIC_CLK of instance: mic"]
    Alt6MicClk = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_A_DATA06 of instance: flexspi1_bus2bit"]
    Alt7Flexspi1Bus2bitAData6 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: TPM3_CH00 of instance: tpm3"]
    Alt8Tpm3Ch0 = 0x08,
    _RESERVED_9 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: XSPI_SLV_DATA06 of instance: xspi_slv"]
    Alt10XspiSlvData6 = 0x0a,
    #[doc = "Select mux mode: ALT11 mux port: NETC_PINMUX_ETH2_TXD03 of instance: netc_pinmux"]
    Alt11NetcPinmuxEth2Txd3 = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_TX_DATA3_1 of instance: ecat"]
    Alt12EcatPt1Txd3 = 0x0c,
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
    Alt0Sinc1ModClk2 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: SINC2_MOD_CLK2 of instance: sinc2"]
    Alt1Sinc2ModClk2 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SINC3_MOD_CLK2 of instance: sinc3"]
    Alt2Sinc3ModClk2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART6_DSR_B of instance: lpuart6"]
    Alt3Lpuart6DsrB = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: SAI4_RX_DATA00 of instance: sai4"]
    Alt4Sai4RxData0 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO20 of instance: gpio6"]
    Alt5Gpio6Io20 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_A_DATA07 of instance: flexspi1_bus2bit"]
    Alt7Flexspi1Bus2bitAData7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: TPM3_CH01 of instance: tpm3"]
    Alt8Tpm3Ch1 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPUART11_TX of instance: lpuart11"]
    Alt9Lpuart11Tx = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: XSPI_SLV_DATA07 of instance: xspi_slv"]
    Alt10XspiSlvData7 = 0x0a,
    #[doc = "Select mux mode: ALT11 mux port: NETC_PINMUX_ETH2_TXD00 of instance: netc_pinmux"]
    Alt11NetcPinmuxEth2Txd0 = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_TX_DATA0_1 of instance: ecat"]
    Alt12EcatPt1Txd0 = 0x0c,
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
    Alt0Qtimer5Timer0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART6_DCD_B of instance: lpuart6"]
    Alt3Lpuart6DcdB = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: SAI4_TX_DATA01 of instance: sai4"]
    Alt4Sai4TxData1 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO21 of instance: gpio6"]
    Alt5Gpio6Io21 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: SAI4_RX_DATA01 of instance: sai4"]
    Alt6Sai4RxData1 = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_A_DQS of instance: flexspi1_bus2bit"]
    Alt7Flexspi1Bus2bitADqs = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: TPM3_CH02 of instance: tpm3"]
    Alt8Tpm3Ch2 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPUART11_RX of instance: lpuart11"]
    Alt9Lpuart11Rx = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: XSPI_SLV_DQS of instance: xspi_slv"]
    Alt10XspiSlvDqs = 0x0a,
    #[doc = "Select mux mode: ALT11 mux port: NETC_PINMUX_ETH2_TXD01 of instance: netc_pinmux"]
    Alt11NetcPinmuxEth2Txd1 = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_TX_DATA1_1 of instance: ecat"]
    Alt12EcatPt1Txd1 = 0x0c,
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
    Alt0Qtimer5Timer1 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: SINC2_EMCLK02 of instance: sinc2"]
    Alt1Sinc2Emclk2 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPUART6_RI_B of instance: lpuart6"]
    Alt4Lpuart6RiB = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO22 of instance: gpio6"]
    Alt5Gpio6Io22 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPI2C6_SCL of instance: lpi2c6"]
    Alt6Lpi2c6Scl = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_A_SCLK of instance: flexspi1_bus2bit"]
    Alt7Flexspi1Bus2bitASclk = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: TPM3_CH03 of instance: tpm3"]
    Alt8Tpm3Ch3 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: SPDIF_IN of instance: spdif"]
    Alt9SpdifIn = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: XSPI_SLV_CLK of instance: xspi_slv"]
    Alt10XspiSlvClk = 0x0a,
    #[doc = "Select mux mode: ALT11 mux port: NETC_PINMUX_ETH2_TX_EN of instance: netc_pinmux"]
    Alt11NetcPinmuxEth2TxEn = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_TX_EN_1 of instance: ecat"]
    Alt12EcatPt1TxEn = 0x0c,
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
    Alt0Qtimer5Timer2 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: SINC2_EMBIT02 of instance: sinc2"]
    Alt1Sinc2Embit2 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPUART6_DTR_B of instance: lpuart6"]
    Alt4Lpuart6DtrB = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO23 of instance: gpio6"]
    Alt5Gpio6Io23 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPI2C6_SDA of instance: lpi2c6"]
    Alt6Lpi2c6Sda = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_A_SS0_B of instance: flexspi1_bus2bit"]
    Alt7Flexspi1Bus2bitASs0B = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: TPM4_EXTCLK of instance: tpm4"]
    Alt8Tpm4Extclk = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: SPDIF_OUT of instance: spdif"]
    Alt9SpdifOut = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: XSPI_SLV_CS of instance: xspi_slv"]
    Alt10XspiSlvCs = 0x0a,
    #[doc = "Select mux mode: ALT11 mux port: NETC_PINMUX_ETH2_TX_CLK of instance: netc_pinmux"]
    Alt11NetcPinmuxEth2TxClk = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_TX_CLK_1 of instance: ecat"]
    Alt12EcatPt1TxClk = 0x0c,
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
    Alt0MicBitstream0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: SINC2_EMCLK03 of instance: sinc2"]
    Alt1Sinc2Emclk3 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: CAN3_TX of instance: can3"]
    Alt2Can3Tx = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART8_CTS_B of instance: lpuart8"]
    Alt3Lpuart8CtsB = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPUART6_TX of instance: lpuart6"]
    Alt4Lpuart6Tx = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO24 of instance: gpio6"]
    Alt5Gpio6Io24 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPI2C4_SCL of instance: lpi2c4"]
    Alt6Lpi2c4Scl = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_A_DATA00 of instance: flexspi1_bus2bit"]
    Alt7Flexspi1Bus2bitAData0 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: TPM4_CH00 of instance: tpm4"]
    Alt8Tpm4Ch0 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPSPI4_SCK of instance: lpspi4"]
    Alt9Lpspi4Sck = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: XSPI_SLV_DATA00 of instance: xspi_slv"]
    Alt10XspiSlvData0 = 0x0a,
    #[doc = "Select mux mode: ALT11 mux port: NETC_PINMUX_ETH2_RXD00 of instance: netc_pinmux"]
    Alt11NetcPinmuxEth2Rxd0 = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_RX_DATA0_1 of instance: ecat"]
    Alt12EcatPt1Rxd0 = 0x0c,
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
    Alt0MicBitstream1 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: SINC2_EMBIT03 of instance: sinc2"]
    Alt1Sinc2Embit3 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: CAN3_RX of instance: can3"]
    Alt2Can3Rx = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART8_RTS_B of instance: lpuart8"]
    Alt3Lpuart8RtsB = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPUART6_RX of instance: lpuart6"]
    Alt4Lpuart6Rx = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO25 of instance: gpio6"]
    Alt5Gpio6Io25 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPI2C4_SDA of instance: lpi2c4"]
    Alt6Lpi2c4Sda = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_A_DATA01 of instance: flexspi1_bus2bit"]
    Alt7Flexspi1Bus2bitAData1 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: TPM4_CH01 of instance: tpm4"]
    Alt8Tpm4Ch1 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPSPI4_SDI of instance: lpspi4"]
    Alt9Lpspi4Sdi = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: XSPI_SLV_DATA01 of instance: xspi_slv"]
    Alt10XspiSlvData1 = 0x0a,
    #[doc = "Select mux mode: ALT11 mux port: NETC_PINMUX_ETH2_RXD01 of instance: netc_pinmux"]
    Alt11NetcPinmuxEth2Rxd1 = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_RX_DATA1_1 of instance: ecat"]
    Alt12EcatPt1Rxd1 = 0x0c,
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
    Alt0MicBitstream2 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: SINC_FILTER_GLUE2_BREAK of instance: sinc_filter_glue2"]
    Alt1SincFilterGlue2Break = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART8_TX of instance: lpuart8"]
    Alt2Lpuart8Tx = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPUART6_CTS_B of instance: lpuart6"]
    Alt4Lpuart6CtsB = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO26 of instance: gpio6"]
    Alt5Gpio6Io26 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: CAN3_TX of instance: can3"]
    Alt6Can3Tx = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_A_DATA02 of instance: flexspi1_bus2bit"]
    Alt7Flexspi1Bus2bitAData2 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: TPM4_CH02 of instance: tpm4"]
    Alt8Tpm4Ch2 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPSPI4_SDO of instance: lpspi4"]
    Alt9Lpspi4Sdo = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: XSPI_SLV_DATA02 of instance: xspi_slv"]
    Alt10XspiSlvData2 = 0x0a,
    #[doc = "Select mux mode: ALT11 mux port: NETC_PINMUX_ETH2_RX_DV of instance: netc_pinmux"]
    Alt11NetcPinmuxEth2RxDv = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_RX_DV_1 of instance: ecat"]
    Alt12EcatPt1RxDv = 0x0c,
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
    Alt0MicBitstream3 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: SINC2_EMCLK00 of instance: sinc2"]
    Alt1Sinc2Emclk0 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART8_RX of instance: lpuart8"]
    Alt2Lpuart8Rx = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPUART6_RTS_B of instance: lpuart6"]
    Alt4Lpuart6RtsB = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO6_IO27 of instance: gpio6"]
    Alt5Gpio6Io27 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: CAN3_RX of instance: can3"]
    Alt6Can3Rx = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: FLEXSPI1_BUS2BIT_A_DATA03 of instance: flexspi1_bus2bit"]
    Alt7Flexspi1Bus2bitAData3 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: TPM4_CH03 of instance: tpm4"]
    Alt8Tpm4Ch3 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPSPI4_PCS0 of instance: lpspi4"]
    Alt9Lpspi4Pcs0 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: XSPI_SLV_DATA03 of instance: xspi_slv"]
    Alt10XspiSlvData3 = 0x0a,
    #[doc = "Select mux mode: ALT11 mux port: NETC_PINMUX_ETH2_RX_CLK of instance: netc_pinmux"]
    Alt11NetcPinmuxEth2RxClk = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_RX_CLK_1 of instance: ecat"]
    Alt12EcatPt1RxClk = 0x0c,
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
    Alt0SemcData0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: XBAR1_XBAR_INOUT04 of instance: xbar1"]
    Alt1Xbar1XbarInout4 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SINC3_MOD_CLK0 of instance: sinc3"]
    Alt2Sinc3ModClk0 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART3_CTS_B of instance: lpuart3"]
    Alt3Lpuart3CtsB = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH3_TXD03 of instance: netc_pinmux"]
    Alt4NetcPinmuxEth3Txd3 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO00 of instance: gpio2"]
    Alt5Gpio2Io0 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_ROW03 of instance: kpp"]
    Alt6KppRow3 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO00 of instance: flexio1"]
    Alt8Flexio1Flexio0 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH4_TXD03 of instance: netc_pinmux"]
    Alt9NetcPinmuxEth4Txd3 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: ECAT_TX_DATA3_0 of instance: ecat"]
    Alt10EcatPt0Txd3 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_DATA00 of instance: ahb_sramc"]
    Alt12AhbSramcData0 = 0x0c,
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
    Alt0SemcData1 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: XBAR1_XBAR_INOUT05 of instance: xbar1"]
    Alt1Xbar1XbarInout5 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SINC3_MOD_CLK1 of instance: sinc3"]
    Alt2Sinc3ModClk1 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART3_RTS_B of instance: lpuart3"]
    Alt3Lpuart3RtsB = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH3_TXD02 of instance: netc_pinmux"]
    Alt4NetcPinmuxEth3Txd2 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO01 of instance: gpio2"]
    Alt5Gpio2Io1 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_COL03 of instance: kpp"]
    Alt6KppCol3 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO01 of instance: flexio1"]
    Alt8Flexio1Flexio1 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH4_TXD02 of instance: netc_pinmux"]
    Alt9NetcPinmuxEth4Txd2 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: ECAT_TX_DATA2_0 of instance: ecat"]
    Alt10EcatPt0Txd2 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_DATA01 of instance: ahb_sramc"]
    Alt12AhbSramcData1 = 0x0c,
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
    Alt0SemcData2 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: XBAR1_XBAR_INOUT06 of instance: xbar1"]
    Alt1Xbar1XbarInout6 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SINC3_MOD_CLK2 of instance: sinc3"]
    Alt2Sinc3ModClk2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART3_RX of instance: lpuart3"]
    Alt3Lpuart3Rx = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH3_RX_CLK of instance: netc_pinmux"]
    Alt4NetcPinmuxEth3RxClk = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO02 of instance: gpio2"]
    Alt5Gpio2Io2 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_ROW02 of instance: kpp"]
    Alt6KppRow2 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO02 of instance: flexio1"]
    Alt8Flexio1Flexio2 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH4_RX_CLK of instance: netc_pinmux"]
    Alt9NetcPinmuxEth4RxClk = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: ECAT_RX_CLK_0 of instance: ecat"]
    Alt10EcatPt0RxClk = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_DATA02 of instance: ahb_sramc"]
    Alt12AhbSramcData2 = 0x0c,
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
    Alt0SemcData3 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: XBAR1_XBAR_INOUT07 of instance: xbar1"]
    Alt1Xbar1XbarInout7 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SINC3_EMCLK00 of instance: sinc3"]
    Alt2Sinc3Emclk0 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART3_TX of instance: lpuart3"]
    Alt3Lpuart3Tx = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH3_RXD03 of instance: netc_pinmux"]
    Alt4NetcPinmuxEth3Rxd3 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO03 of instance: gpio2"]
    Alt5Gpio2Io3 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_COL02 of instance: kpp"]
    Alt6KppCol2 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO03 of instance: flexio1"]
    Alt8Flexio1Flexio3 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH4_RXD03 of instance: netc_pinmux"]
    Alt9NetcPinmuxEth4Rxd3 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: ECAT_RX_DATA3_0 of instance: ecat"]
    Alt10EcatPt0Rxd3 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_DATA03 of instance: ahb_sramc"]
    Alt12AhbSramcData3 = 0x0c,
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
    Alt0SemcData4 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: XBAR1_XBAR_INOUT08 of instance: xbar1"]
    Alt1Xbar1XbarInout8 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SINC3_EMBIT00 of instance: sinc3"]
    Alt2Sinc3Embit0 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART3_DSR_B of instance: lpuart3"]
    Alt3Lpuart3DsrB = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH3_RXD02 of instance: netc_pinmux"]
    Alt4NetcPinmuxEth3Rxd2 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO04 of instance: gpio2"]
    Alt5Gpio2Io4 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_ROW01 of instance: kpp"]
    Alt6KppRow1 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO04 of instance: flexio1"]
    Alt8Flexio1Flexio4 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH4_RXD02 of instance: netc_pinmux"]
    Alt9NetcPinmuxEth4Rxd2 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: ECAT_RX_DATA2_0 of instance: ecat"]
    Alt10EcatPt0Rxd2 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_DATA04 of instance: ahb_sramc"]
    Alt12AhbSramcData4 = 0x0c,
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
    Alt0SemcData5 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: XBAR1_XBAR_INOUT09 of instance: xbar1"]
    Alt1Xbar1XbarInout9 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SINC3_EMCLK01 of instance: sinc3"]
    Alt2Sinc3Emclk1 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART3_DCD_B of instance: lpuart3"]
    Alt3Lpuart3DcdB = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH3_TXD00 of instance: netc_pinmux"]
    Alt4NetcPinmuxEth3Txd0 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO05 of instance: gpio2"]
    Alt5Gpio2Io5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_ROW07 of instance: kpp"]
    Alt6KppRow7 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO05 of instance: flexio1"]
    Alt8Flexio1Flexio5 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH4_TXD00 of instance: netc_pinmux"]
    Alt9NetcPinmuxEth4Txd0 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: ECAT_TX_DATA0_0 of instance: ecat"]
    Alt10EcatPt0Txd0 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_DATA05 of instance: ahb_sramc"]
    Alt12AhbSramcData5 = 0x0c,
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
    Alt0SemcData6 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM4_PWMB03 of instance: flexpwm4"]
    Alt1Flexpwm4Pwmb3 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SINC3_EMBIT01 of instance: sinc3"]
    Alt2Sinc3Embit1 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART3_RI_B of instance: lpuart3"]
    Alt3Lpuart3RiB = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH3_TXD01 of instance: netc_pinmux"]
    Alt4NetcPinmuxEth3Txd1 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO06 of instance: gpio2"]
    Alt5Gpio2Io6 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_COL07 of instance: kpp"]
    Alt6KppCol7 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO06 of instance: flexio1"]
    Alt8Flexio1Flexio6 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH4_TXD01 of instance: netc_pinmux"]
    Alt9NetcPinmuxEth4Txd1 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: ECAT_TX_DATA1_0 of instance: ecat"]
    Alt10EcatPt0Txd1 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_DATA06 of instance: ahb_sramc"]
    Alt12AhbSramcData6 = 0x0c,
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
    Alt0SemcData7 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM4_PWMA03 of instance: flexpwm4"]
    Alt1Flexpwm4Pwma3 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SINC3_EMCLK02 of instance: sinc3"]
    Alt2Sinc3Emclk2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART3_DTR_B of instance: lpuart3"]
    Alt3Lpuart3DtrB = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH3_TX_EN of instance: netc_pinmux"]
    Alt4NetcPinmuxEth3TxEn = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO07 of instance: gpio2"]
    Alt5Gpio2Io7 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_ROW06 of instance: kpp"]
    Alt6KppRow6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO07 of instance: flexio1"]
    Alt8Flexio1Flexio7 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH4_TX_EN of instance: netc_pinmux"]
    Alt9NetcPinmuxEth4TxEn = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: ECAT_TX_EN_0 of instance: ecat"]
    Alt10EcatPt0TxEn = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_DATA07 of instance: ahb_sramc"]
    Alt12AhbSramcData7 = 0x0c,
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
    Alt0SemcDm0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM2_PWMB03 of instance: flexpwm2"]
    Alt1Flexpwm2Pwmb3 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SINC3_EMBIT02 of instance: sinc3"]
    Alt2Sinc3Embit2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART4_DSR_B of instance: lpuart4"]
    Alt3Lpuart4DsrB = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH3_TX_CLK of instance: netc_pinmux"]
    Alt4NetcPinmuxEth3TxClk = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO08 of instance: gpio2"]
    Alt5Gpio2Io8 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_COL06 of instance: kpp"]
    Alt6KppCol6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO08 of instance: flexio1"]
    Alt8Flexio1Flexio8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH4_TX_CLK of instance: netc_pinmux"]
    Alt9NetcPinmuxEth4TxClk = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: ECAT_TX_CLK_0 of instance: ecat"]
    Alt10EcatPt0TxClk = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_LBB of instance: ahb_sramc"]
    Alt12AhbSramcLbb = 0x0c,
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
    Alt0SemcAddr0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM2_PWMA03 of instance: flexpwm2"]
    Alt1Flexpwm2Pwma3 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SINC3_EMCLK03 of instance: sinc3"]
    Alt2Sinc3Emclk3 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART4_DCD_B of instance: lpuart4"]
    Alt3Lpuart4DcdB = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH3_RXD00 of instance: netc_pinmux"]
    Alt4NetcPinmuxEth3Rxd0 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO09 of instance: gpio2"]
    Alt5Gpio2Io9 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_ROW05 of instance: kpp"]
    Alt6KppRow5 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO09 of instance: flexio1"]
    Alt8Flexio1Flexio9 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH4_RXD00 of instance: netc_pinmux"]
    Alt9NetcPinmuxEth4Rxd0 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: ECAT_RX_DATA0_0 of instance: ecat"]
    Alt10EcatPt0Rxd0 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_ADDR00 of instance: ahb_sramc"]
    Alt12AhbSramcAddr0 = 0x0c,
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
    Alt0SemcAddr1 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM3_PWMB03 of instance: flexpwm3"]
    Alt1Flexpwm3Pwmb3 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SINC3_EMBIT03 of instance: sinc3"]
    Alt2Sinc3Embit3 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART4_RI_B of instance: lpuart4"]
    Alt3Lpuart4RiB = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH3_RXD01 of instance: netc_pinmux"]
    Alt4NetcPinmuxEth3Rxd1 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO10 of instance: gpio2"]
    Alt5Gpio2Io10 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_COL05 of instance: kpp"]
    Alt6KppCol5 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO10 of instance: flexio1"]
    Alt8Flexio1Flexio10 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH4_RXD01 of instance: netc_pinmux"]
    Alt9NetcPinmuxEth4Rxd1 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: ECAT_RX_DATA1_0 of instance: ecat"]
    Alt10EcatPt0Rxd1 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_ADDR01 of instance: ahb_sramc"]
    Alt12AhbSramcAddr1 = 0x0c,
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
    Alt0SemcAddr2 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM3_PWMA03 of instance: flexpwm3"]
    Alt1Flexpwm3Pwma3 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SINC_FILTER_GLUE3_BREAK of instance: sinc_filter_glue3"]
    Alt2SincFilterGlue3Break = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPUART4_DTR_B of instance: lpuart4"]
    Alt3Lpuart4DtrB = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH3_RX_DV of instance: netc_pinmux"]
    Alt4NetcPinmuxEth3RxDv = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO11 of instance: gpio2"]
    Alt5Gpio2Io11 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_ROW04 of instance: kpp"]
    Alt6KppRow4 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO11 of instance: flexio1"]
    Alt8Flexio1Flexio11 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH4_RX_DV of instance: netc_pinmux"]
    Alt9NetcPinmuxEth4RxDv = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: ECAT_RX_DV_0 of instance: ecat"]
    Alt10EcatPt0RxDv = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_ADDR02 of instance: ahb_sramc"]
    Alt12AhbSramcAddr2 = 0x0c,
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
    Alt0SemcAddr3 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM4_PWMA00 of instance: flexpwm4"]
    Alt1Flexpwm4Pwma0 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART4_TX of instance: lpuart4"]
    Alt2Lpuart4Tx = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH3_RX_ER of instance: netc_pinmux"]
    Alt4NetcPinmuxEth3RxEr = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO12 of instance: gpio2"]
    Alt5Gpio2Io12 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_COL04 of instance: kpp"]
    Alt6KppCol4 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO12 of instance: flexio1"]
    Alt8Flexio1Flexio12 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH4_RX_ER of instance: netc_pinmux"]
    Alt9NetcPinmuxEth4RxEr = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: ECAT_PT0_RX_ER of instance: ecat"]
    Alt10EcatPt0RxEr = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_ADDR03 of instance: ahb_sramc"]
    Alt12AhbSramcAddr3 = 0x0c,
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
    Alt0SemcAddr4 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM4_PWMB00 of instance: flexpwm4"]
    Alt1Flexpwm4Pwmb0 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART4_RX of instance: lpuart4"]
    Alt2Lpuart4Rx = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_PINMUX_ETH2_RX_DV of instance: netc_pinmux"]
    Alt3NetcPinmuxEth2RxDv = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH3_TX_ER of instance: netc_pinmux"]
    Alt4NetcPinmuxEth3TxEr = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO13 of instance: gpio2"]
    Alt5Gpio2Io13 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_COL01 of instance: kpp"]
    Alt6KppCol1 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO13 of instance: flexio1"]
    Alt8Flexio1Flexio13 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH4_TX_ER of instance: netc_pinmux"]
    Alt9NetcPinmuxEth4TxEr = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: QTIMER1_TIMER1 of instance: qtimer1"]
    Alt10Qtimer1Timer1 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_ADDR04 of instance: ahb_sramc"]
    Alt12AhbSramcAddr4 = 0x0c,
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
    Alt0SemcAddr5 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM4_PWMA01 of instance: flexpwm4"]
    Alt1Flexpwm4Pwma1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART5_TX of instance: lpuart5"]
    Alt2Lpuart5Tx = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_PINMUX_ETH2_TX_EN of instance: netc_pinmux"]
    Alt3NetcPinmuxEth2TxEn = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_ETH3_CRS of instance: netc"]
    Alt4NetcEth3Crs = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO14 of instance: gpio2"]
    Alt5Gpio2Io14 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_ROW00 of instance: kpp"]
    Alt6KppRow0 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO14 of instance: flexio1"]
    Alt8Flexio1Flexio14 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_ETH4_CRS of instance: netc"]
    Alt9NetcEth4Crs = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: LPUART4_CTS_B of instance: lpuart4"]
    Alt10Lpuart4CtsB = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_ADDR05 of instance: ahb_sramc"]
    Alt12AhbSramcAddr5 = 0x0c,
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
    Alt0SemcAddr6 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM4_PWMB01 of instance: flexpwm4"]
    Alt1Flexpwm4Pwmb1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART5_RX of instance: lpuart5"]
    Alt2Lpuart5Rx = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_PINMUX_ETH2_TX_CLK of instance: netc_pinmux"]
    Alt3NetcPinmuxEth2TxClk = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_ETH3_COL of instance: netc"]
    Alt4NetcEth3Col = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO15 of instance: gpio2"]
    Alt5Gpio2Io15 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: KPP_COL00 of instance: kpp"]
    Alt6KppCol0 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO15 of instance: flexio1"]
    Alt8Flexio1Flexio15 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_ETH4_COL of instance: netc"]
    Alt9NetcEth4Col = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: LPUART4_RTS_B of instance: lpuart4"]
    Alt10Lpuart4RtsB = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_ADDR06 of instance: ahb_sramc"]
    Alt12AhbSramcAddr6 = 0x0c,
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
    Alt0SemcAddr7 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM4_PWMB02 of instance: flexpwm4"]
    Alt1Flexpwm4Pwmb2 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART9_TX of instance: lpuart9"]
    Alt2Lpuart9Tx = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_PINMUX_ETH2_RXD00 of instance: netc_pinmux"]
    Alt3NetcPinmuxEth2Rxd0 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_ETH3_SLV_MDC of instance: netc"]
    Alt4NetcEth3SlvMdc = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO16 of instance: gpio2"]
    Alt5Gpio2Io16 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: NETC_ETH4_SLV_MDC of instance: netc"]
    Alt6NetcEth4SlvMdc = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO16 of instance: flexio1"]
    Alt8Flexio1Flexio16 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_ETH2_SLV_MDC of instance: netc"]
    Alt9NetcEth2SlvMdc = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: LPSPI6_PCS2 of instance: lpspi6"]
    Alt10Lpspi6Pcs2 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_ADDR07 of instance: ahb_sramc"]
    Alt12AhbSramcAddr7 = 0x0c,
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
    Alt0SemcAddr8 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM4_PWMA02 of instance: flexpwm4"]
    Alt1Flexpwm4Pwma2 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART9_RX of instance: lpuart9"]
    Alt2Lpuart9Rx = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_PINMUX_ETH2_RXD01 of instance: netc_pinmux"]
    Alt3NetcPinmuxEth2Rxd1 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_ETH3_SLV_MDIO of instance: netc"]
    Alt4NetcEth3SlvMdio = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO17 of instance: gpio2"]
    Alt5Gpio2Io17 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: NETC_ETH4_SLV_MDIO of instance: netc"]
    Alt6NetcEth4SlvMdio = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO17 of instance: flexio1"]
    Alt8Flexio1Flexio17 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_ETH2_SLV_MDIO of instance: netc"]
    Alt9NetcEth2SlvMdio = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: LPSPI6_PCS1 of instance: lpspi6"]
    Alt10Lpspi6Pcs1 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_ADDR08 of instance: ahb_sramc"]
    Alt12AhbSramcAddr8 = 0x0c,
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
    Alt0SemcAddr9 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM2_PWMA00 of instance: flexpwm2"]
    Alt1Flexpwm2Pwma0 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: QTIMER1_TIMER0 of instance: qtimer1"]
    Alt2Qtimer1Timer0 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPSPI6_SCK of instance: lpspi6"]
    Alt3Lpspi6Sck = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_ETH2_CRS of instance: netc"]
    Alt4NetcEth2Crs = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO18 of instance: gpio2"]
    Alt5Gpio2Io18 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO18 of instance: flexio1"]
    Alt8Flexio1Flexio18 = 0x08,
    _RESERVED_9 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: NETC_EMDC of instance: netc"]
    Alt10NetcEmdc = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_ADDR09 of instance: ahb_sramc"]
    Alt12AhbSramcAddr9 = 0x0c,
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
    Alt0SemcAddr11 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM2_PWMB00 of instance: flexpwm2"]
    Alt1Flexpwm2Pwmb0 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: QTIMER2_TIMER0 of instance: qtimer2"]
    Alt2Qtimer2Timer0 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPSPI6_SDI of instance: lpspi6"]
    Alt3Lpspi6Sdi = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_ETH2_COL of instance: netc"]
    Alt4NetcEth2Col = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO19 of instance: gpio2"]
    Alt5Gpio2Io19 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO19 of instance: flexio1"]
    Alt8Flexio1Flexio19 = 0x08,
    _RESERVED_9 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: NETC_EMDIO of instance: netc"]
    Alt10NetcEmdio = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_ADDR11 of instance: ahb_sramc"]
    Alt12AhbSramcAddr11 = 0x0c,
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
    Alt0SemcAddr12 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM2_PWMA01 of instance: flexpwm2"]
    Alt1Flexpwm2Pwma1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: QTIMER3_TIMER0 of instance: qtimer3"]
    Alt2Qtimer3Timer0 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPSPI6_SDO of instance: lpspi6"]
    Alt3Lpspi6Sdo = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH2_TX_ER of instance: netc_pinmux"]
    Alt4NetcPinmuxEth2TxEr = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO20 of instance: gpio2"]
    Alt5Gpio2Io20 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO20 of instance: flexio1"]
    Alt8Flexio1Flexio20 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_ADDR12 of instance: ahb_sramc"]
    Alt12AhbSramcAddr12 = 0x0c,
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
    Alt0SemcBa0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM2_PWMB01 of instance: flexpwm2"]
    Alt1Flexpwm2Pwmb1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: QTIMER4_TIMER0 of instance: qtimer4"]
    Alt2Qtimer4Timer0 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPSPI6_PCS0 of instance: lpspi6"]
    Alt3Lpspi6Pcs0 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH2_RX_CLK of instance: netc_pinmux"]
    Alt4NetcPinmuxEth2RxClk = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO21 of instance: gpio2"]
    Alt5Gpio2Io21 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO21 of instance: flexio1"]
    Alt8Flexio1Flexio21 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPUART4_CTS_B of instance: lpuart4"]
    Alt9Lpuart4CtsB = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: FLEXSPI2_BUS2BIT_B_DQS of instance: flexspi2_bus2bit"]
    Alt10Flexspi2Bus2bitBDqs = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_ADDR13 of instance: ahb_sramc"]
    Alt12AhbSramcAddr13 = 0x0c,
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
    Alt0SemcBa1 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM2_PWMB02 of instance: flexpwm2"]
    Alt1Flexpwm2Pwmb2 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: QTIMER5_TIMER0 of instance: qtimer5"]
    Alt2Qtimer5Timer0 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPSPI4_SCK of instance: lpspi4"]
    Alt3Lpspi4Sck = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH2_RXD02 of instance: netc_pinmux"]
    Alt4NetcPinmuxEth2Rxd2 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO22 of instance: gpio2"]
    Alt5Gpio2Io22 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO22 of instance: flexio1"]
    Alt8Flexio1Flexio22 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPUART4_RTS_B of instance: lpuart4"]
    Alt9Lpuart4RtsB = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: FLEXSPI2_BUS2BIT_B_DATA03 of instance: flexspi2_bus2bit"]
    Alt10Flexspi2Bus2bitBData3 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_ADDR14 of instance: ahb_sramc"]
    Alt12AhbSramcAddr14 = 0x0c,
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
    Alt0SemcAddr10 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM2_PWMA02 of instance: flexpwm2"]
    Alt1Flexpwm2Pwma2 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: QTIMER6_TIMER0 of instance: qtimer6"]
    Alt2Qtimer6Timer0 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPSPI4_SDI of instance: lpspi4"]
    Alt3Lpspi4Sdi = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH2_RXD03 of instance: netc_pinmux"]
    Alt4NetcPinmuxEth2Rxd3 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO23 of instance: gpio2"]
    Alt5Gpio2Io23 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO23 of instance: flexio1"]
    Alt8Flexio1Flexio23 = 0x08,
    _RESERVED_9 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: FLEXSPI2_BUS2BIT_B_DATA02 of instance: flexspi2_bus2bit"]
    Alt10Flexspi2Bus2bitBData2 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_ADDR10 of instance: ahb_sramc"]
    Alt12AhbSramcAddr10 = 0x0c,
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
    Alt0SemcCas = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMA00 of instance: flexpwm1"]
    Alt1Flexpwm1Pwma0 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: QTIMER7_TIMER0 of instance: qtimer7"]
    Alt2Qtimer7Timer0 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPSPI4_SDO of instance: lpspi4"]
    Alt3Lpspi4Sdo = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH2_TXD03 of instance: netc_pinmux"]
    Alt4NetcPinmuxEth2Txd3 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO24 of instance: gpio2"]
    Alt5Gpio2Io24 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO24 of instance: flexio1"]
    Alt8Flexio1Flexio24 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_ETH3_SLV_MDC of instance: netc"]
    Alt9NetcEth3SlvMdc = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: FLEXSPI2_BUS2BIT_B_DATA01 of instance: flexspi2_bus2bit"]
    Alt10Flexspi2Bus2bitBData1 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_ADDR15 of instance: ahb_sramc"]
    Alt12AhbSramcAddr15 = 0x0c,
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
    Alt0SemcRas = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMB00 of instance: flexpwm1"]
    Alt1Flexpwm1Pwmb0 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: QTIMER8_TIMER0 of instance: qtimer8"]
    Alt2Qtimer8Timer0 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPSPI4_PCS0 of instance: lpspi4"]
    Alt3Lpspi4Pcs0 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH2_TXD02 of instance: netc_pinmux"]
    Alt4NetcPinmuxEth2Txd2 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO25 of instance: gpio2"]
    Alt5Gpio2Io25 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXIO1_FLEXIO25 of instance: flexio1"]
    Alt8Flexio1Flexio25 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_ETH3_SLV_MDIO of instance: netc"]
    Alt9NetcEth3SlvMdio = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: FLEXSPI2_BUS2BIT_B_DATA00 of instance: flexspi2_bus2bit"]
    Alt10Flexspi2Bus2bitBData0 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_ADDR16 of instance: ahb_sramc"]
    Alt12AhbSramcAddr16 = 0x0c,
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
    Alt0SemcClk = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMA01 of instance: flexpwm1"]
    Alt1Flexpwm1Pwma1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XBAR1_XBAR_INOUT10 of instance: xbar1"]
    Alt2Xbar1XbarInout10 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: FLEXSPI2_BUS2BIT_A_SS1_B of instance: flexspi2_bus2bit"]
    Alt3Flexspi2Bus2bitASs1B = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH2_TXD01 of instance: netc_pinmux"]
    Alt4NetcPinmuxEth2Txd1 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO26 of instance: gpio2"]
    Alt5Gpio2Io26 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: ECAT_TX_DATA1_1 of instance: ecat"]
    Alt6EcatPt1Txd1 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: LPSPI6_SCK of instance: lpspi6"]
    Alt10Lpspi6Sck = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_WE of instance: ahb_sramc"]
    Alt12AhbSramcWe = 0x0c,
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
    Alt0SemcCke = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMB01 of instance: flexpwm1"]
    Alt1Flexpwm1Pwmb1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XBAR1_XBAR_INOUT11 of instance: xbar1"]
    Alt2Xbar1XbarInout11 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: FLEXSPI2_BUS2BIT_B_SS1_B of instance: flexspi2_bus2bit"]
    Alt3Flexspi2Bus2bitBSs1B = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH2_TXD00 of instance: netc_pinmux"]
    Alt4NetcPinmuxEth2Txd0 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO27 of instance: gpio2"]
    Alt5Gpio2Io27 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: ECAT_TX_DATA0_1 of instance: ecat"]
    Alt6EcatPt1Txd0 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPUART6_RI_B of instance: lpuart6"]
    Alt9Lpuart6RiB = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: LPSPI6_SDI of instance: lpspi6"]
    Alt10Lpspi6Sdi = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_OEB of instance: ahb_sramc"]
    Alt12AhbSramcOeb = 0x0c,
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
    Alt0SemcWe = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMB02 of instance: flexpwm1"]
    Alt1Flexpwm1Pwmb2 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XBAR1_XBAR_INOUT12 of instance: xbar1"]
    Alt2Xbar1XbarInout12 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: FLEXSPI2_BUS2BIT_B_SS0_B of instance: flexspi2_bus2bit"]
    Alt3Flexspi2Bus2bitBSs0B = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH2_TX_EN of instance: netc_pinmux"]
    Alt4NetcPinmuxEth2TxEn = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO28 of instance: gpio2"]
    Alt5Gpio2Io28 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: ECAT_TX_EN_1 of instance: ecat"]
    Alt6EcatPt1TxEn = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPUART6_DTR_B of instance: lpuart6"]
    Alt9Lpuart6DtrB = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: LPSPI6_SDO of instance: lpspi6"]
    Alt10Lpspi6Sdo = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_ADV of instance: ahb_sramc"]
    Alt12AhbSramcAdv = 0x0c,
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
    Alt0SemcCs0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMA02 of instance: flexpwm1"]
    Alt1Flexpwm1Pwma2 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XBAR1_XBAR_INOUT13 of instance: xbar1"]
    Alt2Xbar1XbarInout13 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: FLEXSPI2_BUS2BIT_B_DQS of instance: flexspi2_bus2bit"]
    Alt3Flexspi2Bus2bitBDqs = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH2_TX_CLK of instance: netc_pinmux"]
    Alt4NetcPinmuxEth2TxClk = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO29 of instance: gpio2"]
    Alt5Gpio2Io29 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: ECAT_TX_CLK_1 of instance: ecat"]
    Alt6EcatPt1TxClk = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPUART6_DCD_B of instance: lpuart6"]
    Alt9Lpuart6DcdB = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: LPSPI6_PCS0 of instance: lpspi6"]
    Alt10Lpspi6Pcs0 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_CS0 of instance: ahb_sramc"]
    Alt12AhbSramcCs0 = 0x0c,
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
    Alt0SemcData8 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM3_PWMA00 of instance: flexpwm3"]
    Alt1Flexpwm3Pwma0 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XBAR1_XBAR_INOUT14 of instance: xbar1"]
    Alt2Xbar1XbarInout14 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: FLEXSPI2_BUS2BIT_B_DATA03 of instance: flexspi2_bus2bit"]
    Alt3Flexspi2Bus2bitBData3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH2_RXD00 of instance: netc_pinmux"]
    Alt4NetcPinmuxEth2Rxd0 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO30 of instance: gpio2"]
    Alt5Gpio2Io30 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: ECAT_RX_DATA0_1 of instance: ecat"]
    Alt6EcatPt1Rxd0 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPUART6_DSR_B of instance: lpuart6"]
    Alt9Lpuart6DsrB = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: LPSPI6_PCS1 of instance: lpspi6"]
    Alt10Lpspi6Pcs1 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_DATA08 of instance: ahb_sramc"]
    Alt12AhbSramcData8 = 0x0c,
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
    Alt0SemcData9 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM3_PWMB00 of instance: flexpwm3"]
    Alt1Flexpwm3Pwmb0 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART6_TX of instance: lpuart6"]
    Alt2Lpuart6Tx = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: FLEXSPI2_BUS2BIT_B_DATA02 of instance: flexspi2_bus2bit"]
    Alt3Flexspi2Bus2bitBData2 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH2_RXD01 of instance: netc_pinmux"]
    Alt4NetcPinmuxEth2Rxd1 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO2_IO31 of instance: gpio2"]
    Alt5Gpio2Io31 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: ECAT_RX_DATA1_1 of instance: ecat"]
    Alt6EcatPt1Rxd1 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPSPI5_SCK of instance: lpspi5"]
    Alt9Lpspi5Sck = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: LPSPI6_PCS2 of instance: lpspi6"]
    Alt10Lpspi6Pcs2 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_DATA09 of instance: ahb_sramc"]
    Alt12AhbSramcData9 = 0x0c,
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
    Alt0SemcData10 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM3_PWMA01 of instance: flexpwm3"]
    Alt1Flexpwm3Pwma1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART6_RX of instance: lpuart6"]
    Alt2Lpuart6Rx = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: FLEXSPI2_BUS2BIT_B_DATA01 of instance: flexspi2_bus2bit"]
    Alt3Flexspi2Bus2bitBData1 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH2_RX_DV of instance: netc_pinmux"]
    Alt4NetcPinmuxEth2RxDv = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO00 of instance: gpio3"]
    Alt5Gpio3Io0 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: ECAT_RX_DV_1 of instance: ecat"]
    Alt6EcatPt1RxDv = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPSPI5_SDO of instance: lpspi5"]
    Alt9Lpspi5Sdo = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: LPSPI6_PCS3 of instance: lpspi6"]
    Alt10Lpspi6Pcs3 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_DATA10 of instance: ahb_sramc"]
    Alt12AhbSramcData10 = 0x0c,
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
    Alt0SemcData11 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM3_PWMB01 of instance: flexpwm3"]
    Alt1Flexpwm3Pwmb1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART6_CTS_B of instance: lpuart6"]
    Alt2Lpuart6CtsB = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: FLEXSPI2_BUS2BIT_B_DATA00 of instance: flexspi2_bus2bit"]
    Alt3Flexspi2Bus2bitBData0 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH2_RX_ER of instance: netc_pinmux"]
    Alt4NetcPinmuxEth2RxEr = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO01 of instance: gpio3"]
    Alt5Gpio3Io1 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: ECAT_RX_ER_1 of instance: ecat"]
    Alt6EcatPt1RxEr = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPSPI5_SDI of instance: lpspi5"]
    Alt9Lpspi5Sdi = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: NETC_PINMUX_ETH2_RX_CLK of instance: netc_pinmux"]
    Alt10NetcPinmuxEth2RxClk = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_DATA11 of instance: ahb_sramc"]
    Alt12AhbSramcData11 = 0x0c,
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
    Alt0SemcData12 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM3_PWMB02 of instance: flexpwm3"]
    Alt1Flexpwm3Pwmb2 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART6_RTS_B of instance: lpuart6"]
    Alt2Lpuart6RtsB = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: FLEXSPI2_BUS2BIT_B_SCLK of instance: flexspi2_bus2bit"]
    Alt3Flexspi2Bus2bitBSclk = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH2_RXD02 of instance: netc_pinmux"]
    Alt4NetcPinmuxEth2Rxd2 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO02 of instance: gpio3"]
    Alt5Gpio3Io2 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: ECAT_RX_DATA2_1 of instance: ecat"]
    Alt6EcatPt1Rxd2 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH0_TXD00 of instance: netc_pinmux"]
    Alt9NetcPinmuxEth0Txd0 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: LPSPI5_PCS0 of instance: lpspi5"]
    Alt10Lpspi5Pcs0 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_DATA12 of instance: ahb_sramc"]
    Alt12AhbSramcData12 = 0x0c,
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
    Alt0SemcData13 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM3_PWMA02 of instance: flexpwm3"]
    Alt1Flexpwm3Pwma2 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART5_TX of instance: lpuart5"]
    Alt2Lpuart5Tx = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: FLEXSPI2_BUS2BIT_A_DATA00 of instance: flexspi2_bus2bit"]
    Alt3Flexspi2Bus2bitAData0 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH2_RXD03 of instance: netc_pinmux"]
    Alt4NetcPinmuxEth2Rxd3 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO03 of instance: gpio3"]
    Alt5Gpio3Io3 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: ECAT_RX_DATA3_1 of instance: ecat"]
    Alt6EcatPt1Rxd3 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH0_TXD01 of instance: netc_pinmux"]
    Alt9NetcPinmuxEth0Txd1 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: LPSPI5_PCS1 of instance: lpspi5"]
    Alt10Lpspi5Pcs1 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_DATA13 of instance: ahb_sramc"]
    Alt12AhbSramcData13 = 0x0c,
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
    Alt0SemcData14 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMA00 of instance: flexpwm1"]
    Alt1Flexpwm1Pwma0 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART5_RX of instance: lpuart5"]
    Alt2Lpuart5Rx = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: FLEXSPI2_BUS2BIT_A_DATA01 of instance: flexspi2_bus2bit"]
    Alt3Flexspi2Bus2bitAData1 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH2_TXD03 of instance: netc_pinmux"]
    Alt4NetcPinmuxEth2Txd3 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO04 of instance: gpio3"]
    Alt5Gpio3Io4 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: ECAT_TX_DATA3_1 of instance: ecat"]
    Alt6EcatPt1Txd3 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH0_TX_EN of instance: netc_pinmux"]
    Alt9NetcPinmuxEth0TxEn = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_DATA14 of instance: ahb_sramc"]
    Alt12AhbSramcData14 = 0x0c,
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
    Alt0SemcData15 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMB00 of instance: flexpwm1"]
    Alt1Flexpwm1Pwmb0 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART5_CTS_B of instance: lpuart5"]
    Alt2Lpuart5CtsB = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: FLEXSPI2_BUS2BIT_A_DATA02 of instance: flexspi2_bus2bit"]
    Alt3Flexspi2Bus2bitAData2 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH2_TXD02 of instance: netc_pinmux"]
    Alt4NetcPinmuxEth2Txd2 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO05 of instance: gpio3"]
    Alt5Gpio3Io5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: ECAT_TX_DATA2_1 of instance: ecat"]
    Alt6EcatPt1Txd2 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH0_TX_CLK of instance: netc_pinmux"]
    Alt9NetcPinmuxEth0TxClk = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_DATA15 of instance: ahb_sramc"]
    Alt12AhbSramcData15 = 0x0c,
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
    Alt0SemcDm1 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMB03 of instance: flexpwm1"]
    Alt1Flexpwm1Pwmb3 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART5_RTS_B of instance: lpuart5"]
    Alt2Lpuart5RtsB = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: FLEXSPI2_BUS2BIT_A_DATA03 of instance: flexspi2_bus2bit"]
    Alt3Flexspi2Bus2bitAData3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH2_RX_CLK of instance: netc_pinmux"]
    Alt4NetcPinmuxEth2RxClk = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO06 of instance: gpio3"]
    Alt5Gpio3Io6 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: ECAT_RX_CLK_1 of instance: ecat"]
    Alt6EcatPt1RxClk = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH0_RXD00 of instance: netc_pinmux"]
    Alt9NetcPinmuxEth0Rxd0 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_UBB of instance: ahb_sramc"]
    Alt12AhbSramcUbb = 0x0c,
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
    Alt0SemcDqs = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXPWM1_PWMA03 of instance: flexpwm1"]
    Alt1Flexpwm1Pwma3 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XBAR1_XBAR_INOUT15 of instance: xbar1"]
    Alt2Xbar1XbarInout15 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: FLEXSPI2_BUS2BIT_A_SS0_B of instance: flexspi2_bus2bit"]
    Alt3Flexspi2Bus2bitASs0B = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH2_TX_ER of instance: netc_pinmux"]
    Alt4NetcPinmuxEth2TxEr = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO07 of instance: gpio3"]
    Alt5Gpio3Io7 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: QTIMER2_TIMER1 of instance: qtimer2"]
    Alt6Qtimer2Timer1 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH0_RXD01 of instance: netc_pinmux"]
    Alt9NetcPinmuxEth0Rxd1 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_CS1 of instance: ahb_sramc"]
    Alt12AhbSramcCs1 = 0x0c,
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
    Alt0SemcRdy = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: NETC_EMDC of instance: netc"]
    Alt1NetcEmdc = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: NETC_ETH2_SLV_MDC of instance: netc"]
    Alt2NetcEth2SlvMdc = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: FLEXSPI2_BUS2BIT_A_DQS of instance: flexspi2_bus2bit"]
    Alt3Flexspi2Bus2bitADqs = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_ETH2_CRS of instance: netc"]
    Alt4NetcEth2Crs = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO08 of instance: gpio3"]
    Alt5Gpio3Io8 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: QTIMER3_TIMER1 of instance: qtimer3"]
    Alt6Qtimer3Timer1 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH0_RX_DV of instance: netc_pinmux"]
    Alt9NetcPinmuxEth0RxDv = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_CS2 of instance: ahb_sramc"]
    Alt12AhbSramcCs2 = 0x0c,
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
    Alt0SemcCsx0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: NETC_EMDIO of instance: netc"]
    Alt1NetcEmdio = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: NETC_ETH2_SLV_MDIO of instance: netc"]
    Alt2NetcEth2SlvMdio = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: FLEXSPI2_BUS2BIT_A_SCLK of instance: flexspi2_bus2bit"]
    Alt3Flexspi2Bus2bitASclk = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_ETH2_COL of instance: netc"]
    Alt4NetcEth2Col = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO09 of instance: gpio3"]
    Alt5Gpio3Io9 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: QTIMER4_TIMER1 of instance: qtimer4"]
    Alt6Qtimer4Timer1 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH0_RX_ER of instance: netc_pinmux"]
    Alt9NetcPinmuxEth0RxEr = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: AHB_SRAMC_CS3 of instance: ahb_sramc"]
    Alt12AhbSramcCs3 = 0x0c,
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
    Alt0SemcData16 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: CCM_ENET_REF_CLK_25M of instance: ccm"]
    Alt1CcmEnetRefClk25m = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: QTIMER5_TIMER1 of instance: qtimer5"]
    Alt2Qtimer5Timer1 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_EMDC of instance: netc"]
    Alt3NetcEmdc = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH0_RX_CLK of instance: netc_pinmux"]
    Alt4NetcPinmuxEth0RxClk = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO10 of instance: gpio3"]
    Alt5Gpio3Io10 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_XBAR_INOUT20 of instance: xbar1"]
    Alt6Xbar1XbarInout20 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPSPI5_SCK of instance: lpspi5"]
    Alt8Lpspi5Sck = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPI2C3_SCL of instance: lpi2c3"]
    Alt9Lpi2c3Scl = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: FLEXPWM3_PWMA00 of instance: flexpwm3"]
    Alt10Flexpwm3Pwma0 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_RX_CLK_0 of instance: ecat"]
    Alt12EcatPt0RxClk = 0x0c,
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
    Alt0SemcData17 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: USDHC2_CD_B of instance: usdhc2"]
    Alt1Usdhc2CdB = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: QTIMER6_TIMER1 of instance: qtimer6"]
    Alt2Qtimer6Timer1 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_EMDIO of instance: netc"]
    Alt3NetcEmdio = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH0_RXD02 of instance: netc_pinmux"]
    Alt4NetcPinmuxEth0Rxd2 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO11 of instance: gpio3"]
    Alt5Gpio3Io11 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_XBAR_INOUT21 of instance: xbar1"]
    Alt6Xbar1XbarInout21 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPSPI5_PCS0 of instance: lpspi5"]
    Alt8Lpspi5Pcs0 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPI2C3_SDA of instance: lpi2c3"]
    Alt9Lpi2c3Sda = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: FLEXPWM3_PWMB00 of instance: flexpwm3"]
    Alt10Flexpwm3Pwmb0 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_RX_DATA2_0 of instance: ecat"]
    Alt12EcatPt0Rxd2 = 0x0c,
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
    Alt0SemcData18 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: USDHC2_WP of instance: usdhc2"]
    Alt1Usdhc2Wp = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: QTIMER7_TIMER1 of instance: qtimer7"]
    Alt2Qtimer7Timer1 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH0_RXD03 of instance: netc_pinmux"]
    Alt4NetcPinmuxEth0Rxd3 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO12 of instance: gpio3"]
    Alt5Gpio3Io12 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_XBAR_INOUT22 of instance: xbar1"]
    Alt6Xbar1XbarInout22 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPSPI5_SDO of instance: lpspi5"]
    Alt8Lpspi5Sdo = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: CCM_CLKO1 of instance: ccm"]
    Alt9CcmClko1 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: FLEXPWM3_PWMA01 of instance: flexpwm3"]
    Alt10Flexpwm3Pwma1 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_RX_DATA3_0 of instance: ecat"]
    Alt12EcatPt0Rxd3 = 0x0c,
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
    Alt0SemcData19 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: USDHC2_VSELECT of instance: usdhc2"]
    Alt1Usdhc2Vselect = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: QTIMER8_TIMER1 of instance: qtimer8"]
    Alt2Qtimer8Timer1 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH0_TXD02 of instance: netc_pinmux"]
    Alt4NetcPinmuxEth0Txd2 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO13 of instance: gpio3"]
    Alt5Gpio3Io13 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_XBAR_INOUT23 of instance: xbar1"]
    Alt6Xbar1XbarInout23 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPSPI5_SDI of instance: lpspi5"]
    Alt8Lpspi5Sdi = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_ETH3_CRS of instance: netc"]
    Alt9NetcEth3Crs = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: FLEXPWM3_PWMB01 of instance: flexpwm3"]
    Alt10Flexpwm3Pwmb1 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_TX_DATA2_0 of instance: ecat"]
    Alt12EcatPt0Txd2 = 0x0c,
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
    Alt0SemcData20 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: USDHC2_RESET_B of instance: usdhc2"]
    Alt1Usdhc2ResetB = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SAI2_MCLK of instance: sai2"]
    Alt2Sai2Mclk = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_PINMUX_ETH0_TXD03 of instance: netc_pinmux"]
    Alt4NetcPinmuxEth0Txd3 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO14 of instance: gpio3"]
    Alt5Gpio3Io14 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_XBAR_INOUT24 of instance: xbar1"]
    Alt6Xbar1XbarInout24 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPSPI3_SCK of instance: lpspi3"]
    Alt8Lpspi3Sck = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_ETH3_COL of instance: netc"]
    Alt9NetcEth3Col = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: FLEXPWM3_PWMB02 of instance: flexpwm3"]
    Alt10Flexpwm3Pwmb2 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_TX_DATA3_0 of instance: ecat"]
    Alt12EcatPt0Txd3 = 0x0c,
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
    Alt0SemcData21 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: NETC_ETH4_SLV_MDC of instance: netc"]
    Alt1NetcEth4SlvMdc = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SAI2_RX_SYNC of instance: sai2"]
    Alt2Sai2RxSync = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_PINMUX_ETH0_TXD00 of instance: netc_pinmux"]
    Alt3NetcPinmuxEth0Txd0 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_ETH4_CRS of instance: netc"]
    Alt4NetcEth4Crs = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO15 of instance: gpio3"]
    Alt5Gpio3Io15 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_XBAR_INOUT25 of instance: xbar1"]
    Alt6Xbar1XbarInout25 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPSPI3_PCS0 of instance: lpspi3"]
    Alt8Lpspi3Pcs0 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH3_TXD00 of instance: netc_pinmux"]
    Alt9NetcPinmuxEth3Txd0 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: FLEXPWM3_PWMA02 of instance: flexpwm3"]
    Alt10Flexpwm3Pwma2 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_TX_DATA0_0 of instance: ecat"]
    Alt12EcatPt0Txd0 = 0x0c,
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
    Alt0SemcData22 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: NETC_ETH4_SLV_MDIO of instance: netc"]
    Alt1NetcEth4SlvMdio = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SAI2_RX_BCLK of instance: sai2"]
    Alt2Sai2RxBclk = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_PINMUX_ETH0_TXD01 of instance: netc_pinmux"]
    Alt3NetcPinmuxEth0Txd1 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_ETH4_COL of instance: netc"]
    Alt4NetcEth4Col = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO16 of instance: gpio3"]
    Alt5Gpio3Io16 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_XBAR_INOUT26 of instance: xbar1"]
    Alt6Xbar1XbarInout26 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPSPI3_SDO of instance: lpspi3"]
    Alt8Lpspi3Sdo = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH3_TXD01 of instance: netc_pinmux"]
    Alt9NetcPinmuxEth3Txd1 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: FLEXPWM3_PWMB03 of instance: flexpwm3"]
    Alt10Flexpwm3Pwmb3 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_TX_DATA1_0 of instance: ecat"]
    Alt12EcatPt0Txd1 = 0x0c,
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
    Alt0SemcData23 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: NETC_PINMUX_ETH4_TX_ER of instance: netc_pinmux"]
    Alt1NetcPinmuxEth4TxEr = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SAI2_RX_DATA of instance: sai2"]
    Alt2Sai2RxData = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_PINMUX_ETH0_TX_EN of instance: netc_pinmux"]
    Alt3NetcPinmuxEth0TxEn = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO17 of instance: gpio3"]
    Alt5Gpio3Io17 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_XBAR_INOUT27 of instance: xbar1"]
    Alt6Xbar1XbarInout27 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPSPI3_SDI of instance: lpspi3"]
    Alt8Lpspi3Sdi = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH3_TX_EN of instance: netc_pinmux"]
    Alt9NetcPinmuxEth3TxEn = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: FLEXPWM3_PWMA03 of instance: flexpwm3"]
    Alt10Flexpwm3Pwma3 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_TX_EN_0 of instance: ecat"]
    Alt12EcatPt0TxEn = 0x0c,
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
    Alt0SemcDm2 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: NETC_PINMUX_ETH4_RX_CLK of instance: netc_pinmux"]
    Alt1NetcPinmuxEth4RxClk = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SAI2_TX_DATA of instance: sai2"]
    Alt2Sai2TxData = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_PINMUX_ETH0_TX_CLK of instance: netc_pinmux"]
    Alt3NetcPinmuxEth0TxClk = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO18 of instance: gpio3"]
    Alt5Gpio3Io18 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_XBAR_INOUT28 of instance: xbar1"]
    Alt6Xbar1XbarInout28 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPSPI3_PCS3 of instance: lpspi3"]
    Alt8Lpspi3Pcs3 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH3_TX_CLK of instance: netc_pinmux"]
    Alt9NetcPinmuxEth3TxClk = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: CCM_CLKO2 of instance: ccm"]
    Alt10CcmClko2 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_TX_CLK_0 of instance: ecat"]
    Alt12EcatPt0TxClk = 0x0c,
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
    Alt0SemcData24 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: NETC_PINMUX_ETH4_RXD03 of instance: netc_pinmux"]
    Alt1NetcPinmuxEth4Rxd3 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SAI2_TX_BCLK of instance: sai2"]
    Alt2Sai2TxBclk = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_PINMUX_ETH0_RXD00 of instance: netc_pinmux"]
    Alt3NetcPinmuxEth0Rxd0 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO19 of instance: gpio3"]
    Alt5Gpio3Io19 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_XBAR_INOUT29 of instance: xbar1"]
    Alt6Xbar1XbarInout29 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPSPI3_PCS2 of instance: lpspi3"]
    Alt8Lpspi3Pcs2 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH3_RXD00 of instance: netc_pinmux"]
    Alt9NetcPinmuxEth3Rxd0 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: QTIMER1_TIMER0 of instance: qtimer1"]
    Alt10Qtimer1Timer0 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_RX_DATA0_0 of instance: ecat"]
    Alt12EcatPt0Rxd0 = 0x0c,
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
    Alt0SemcData25 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: NETC_PINMUX_ETH4_RXD02 of instance: netc_pinmux"]
    Alt1NetcPinmuxEth4Rxd2 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SAI2_TX_SYNC of instance: sai2"]
    Alt2Sai2TxSync = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_PINMUX_ETH0_RXD01 of instance: netc_pinmux"]
    Alt3NetcPinmuxEth0Rxd1 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO20 of instance: gpio3"]
    Alt5Gpio3Io20 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_XBAR_INOUT30 of instance: xbar1"]
    Alt6Xbar1XbarInout30 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPSPI3_PCS1 of instance: lpspi3"]
    Alt8Lpspi3Pcs1 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH3_RXD01 of instance: netc_pinmux"]
    Alt9NetcPinmuxEth3Rxd1 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: QTIMER1_TIMER1 of instance: qtimer1"]
    Alt10Qtimer1Timer1 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_RX_DATA1_0 of instance: ecat"]
    Alt12EcatPt0Rxd1 = 0x0c,
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
    Alt0SemcData26 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: NETC_PINMUX_ETH4_TXD03 of instance: netc_pinmux"]
    Alt1NetcPinmuxEth4Txd3 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SPDIF_OUT of instance: spdif"]
    Alt2SpdifOut = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_PINMUX_ETH0_RX_DV of instance: netc_pinmux"]
    Alt3NetcPinmuxEth0RxDv = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPSPI5_PCS3 of instance: lpspi5"]
    Alt4Lpspi5Pcs3 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO21 of instance: gpio3"]
    Alt5Gpio3Io21 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_XBAR_INOUT31 of instance: xbar1"]
    Alt6Xbar1XbarInout31 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: SAI3_RX_SYNC of instance: sai3"]
    Alt8Sai3RxSync = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH3_RX_DV of instance: netc_pinmux"]
    Alt9NetcPinmuxEth3RxDv = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: QTIMER1_TIMER2 of instance: qtimer1"]
    Alt10Qtimer1Timer2 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_RX_DV_0 of instance: ecat"]
    Alt12EcatPt0RxDv = 0x0c,
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
    Alt0SemcData27 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: NETC_PINMUX_ETH4_TXD02 of instance: netc_pinmux"]
    Alt1NetcPinmuxEth4Txd2 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SPDIF_IN of instance: spdif"]
    Alt2SpdifIn = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_PINMUX_ETH0_RX_ER of instance: netc_pinmux"]
    Alt3NetcPinmuxEth0RxEr = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPSPI5_PCS2 of instance: lpspi5"]
    Alt4Lpspi5Pcs2 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO22 of instance: gpio3"]
    Alt5Gpio3Io22 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_XBAR_INOUT32 of instance: xbar1"]
    Alt6Xbar1XbarInout32 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: SAI3_RX_BCLK of instance: sai3"]
    Alt8Sai3RxBclk = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH3_RX_ER of instance: netc_pinmux"]
    Alt9NetcPinmuxEth3RxEr = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: QTIMER1_TIMER3 of instance: qtimer1"]
    Alt10Qtimer1Timer3 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_PT0_RX_ER of instance: ecat"]
    Alt12EcatPt0RxEr = 0x0c,
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
    Alt0SemcData28 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: NETC_PINMUX_ETH4_TXD00 of instance: netc_pinmux"]
    Alt1NetcPinmuxEth4Txd0 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART11_TX of instance: lpuart11"]
    Alt2Lpuart11Tx = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_PINMUX_ETH0_TXD03 of instance: netc_pinmux"]
    Alt3NetcPinmuxEth0Txd3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPSPI5_PCS1 of instance: lpspi5"]
    Alt4Lpspi5Pcs1 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO23 of instance: gpio3"]
    Alt5Gpio3Io23 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_XBAR_INOUT33 of instance: xbar1"]
    Alt6Xbar1XbarInout33 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: SAI3_RX_DATA of instance: sai3"]
    Alt8Sai3RxData = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH3_TXD03 of instance: netc_pinmux"]
    Alt9NetcPinmuxEth3Txd3 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: QTIMER2_TIMER0 of instance: qtimer2"]
    Alt10Qtimer2Timer0 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_TX_DATA0_1 of instance: ecat"]
    Alt12EcatPt1Txd0 = 0x0c,
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
    Alt0SemcData29 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: NETC_PINMUX_ETH4_TXD01 of instance: netc_pinmux"]
    Alt1NetcPinmuxEth4Txd1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART11_RX of instance: lpuart11"]
    Alt2Lpuart11Rx = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_PINMUX_ETH0_TXD02 of instance: netc_pinmux"]
    Alt3NetcPinmuxEth0Txd2 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPUART5_DSR_B of instance: lpuart5"]
    Alt4Lpuart5DsrB = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO24 of instance: gpio3"]
    Alt5Gpio3Io24 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_XBAR_INOUT34 of instance: xbar1"]
    Alt6Xbar1XbarInout34 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: SAI3_TX_DATA of instance: sai3"]
    Alt8Sai3TxData = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH3_TXD02 of instance: netc_pinmux"]
    Alt9NetcPinmuxEth3Txd2 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: QTIMER2_TIMER1 of instance: qtimer2"]
    Alt10Qtimer2Timer1 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_TX_DATA1_1 of instance: ecat"]
    Alt12EcatPt1Txd1 = 0x0c,
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
    Alt0SemcData30 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: NETC_PINMUX_ETH4_TX_EN of instance: netc_pinmux"]
    Alt1NetcPinmuxEth4TxEn = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART11_CTS_B of instance: lpuart11"]
    Alt2Lpuart11CtsB = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_PINMUX_ETH0_RX_CLK of instance: netc_pinmux"]
    Alt3NetcPinmuxEth0RxClk = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPUART5_DCD_B of instance: lpuart5"]
    Alt4Lpuart5DcdB = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO25 of instance: gpio3"]
    Alt5Gpio3Io25 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_XBAR_INOUT35 of instance: xbar1"]
    Alt6Xbar1XbarInout35 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: SAI3_TX_BCLK of instance: sai3"]
    Alt8Sai3TxBclk = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH3_RX_CLK of instance: netc_pinmux"]
    Alt9NetcPinmuxEth3RxClk = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: QTIMER2_TIMER2 of instance: qtimer2"]
    Alt10Qtimer2Timer2 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_TX_EN_1 of instance: ecat"]
    Alt12EcatPt1TxEn = 0x0c,
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
    Alt0SemcData31 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: NETC_PINMUX_ETH4_TX_CLK of instance: netc_pinmux"]
    Alt1NetcPinmuxEth4TxClk = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART11_RTS_B of instance: lpuart11"]
    Alt2Lpuart11RtsB = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_PINMUX_ETH0_RXD02 of instance: netc_pinmux"]
    Alt3NetcPinmuxEth0Rxd2 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPUART5_DTR_B of instance: lpuart5"]
    Alt4Lpuart5DtrB = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO26 of instance: gpio3"]
    Alt5Gpio3Io26 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_XBAR_INOUT14 of instance: xbar1"]
    Alt6Xbar1XbarInout14 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: SAI3_TX_SYNC of instance: sai3"]
    Alt8Sai3TxSync = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH3_RXD02 of instance: netc_pinmux"]
    Alt9NetcPinmuxEth3Rxd2 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: QTIMER2_TIMER3 of instance: qtimer2"]
    Alt10Qtimer2Timer3 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_TX_CLK_1 of instance: ecat"]
    Alt12EcatPt1TxClk = 0x0c,
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
    Alt0SemcDm3 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: NETC_PINMUX_ETH4_RXD00 of instance: netc_pinmux"]
    Alt1NetcPinmuxEth4Rxd0 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART5_TX of instance: lpuart5"]
    Alt2Lpuart5Tx = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_PINMUX_ETH0_RXD03 of instance: netc_pinmux"]
    Alt3NetcPinmuxEth0Rxd3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO27 of instance: gpio3"]
    Alt5Gpio3Io27 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_XBAR_INOUT15 of instance: xbar1"]
    Alt6Xbar1XbarInout15 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: SAI3_MCLK of instance: sai3"]
    Alt8Sai3Mclk = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH3_RXD03 of instance: netc_pinmux"]
    Alt9NetcPinmuxEth3Rxd3 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: QTIMER3_TIMER0 of instance: qtimer3"]
    Alt10Qtimer3Timer0 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_RX_DATA0_1 of instance: ecat"]
    Alt12EcatPt1Rxd0 = 0x0c,
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
    Alt0SemcDqs4 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: NETC_PINMUX_ETH4_RXD01 of instance: netc_pinmux"]
    Alt1NetcPinmuxEth4Rxd1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART5_RX of instance: lpuart5"]
    Alt2Lpuart5Rx = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_PINMUX_ETH0_TX_ER of instance: netc_pinmux"]
    Alt3NetcPinmuxEth0TxEr = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO28 of instance: gpio3"]
    Alt5Gpio3Io28 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_XBAR_INOUT16 of instance: xbar1"]
    Alt6Xbar1XbarInout16 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: EWM_EWM_OUT_B of instance: ewm"]
    Alt8EwmEwmOutB = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_PINMUX_ETH3_TX_ER of instance: netc_pinmux"]
    Alt9NetcPinmuxEth3TxEr = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: QTIMER3_TIMER1 of instance: qtimer3"]
    Alt10Qtimer3Timer1 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_RX_DATA1_1 of instance: ecat"]
    Alt12EcatPt1Rxd1 = 0x0c,
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
    Alt0SemcClkx0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: NETC_PINMUX_ETH4_RX_DV of instance: netc_pinmux"]
    Alt1NetcPinmuxEth4RxDv = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART5_CTS_B of instance: lpuart5"]
    Alt2Lpuart5CtsB = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_ETH0_CRS of instance: netc"]
    Alt3NetcEth0Crs = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_EMDC of instance: netc"]
    Alt4NetcEmdc = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO29 of instance: gpio3"]
    Alt5Gpio3Io29 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_XBAR_INOUT36 of instance: xbar1"]
    Alt6Xbar1XbarInout36 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPI2C3_SCL of instance: lpi2c3"]
    Alt8Lpi2c3Scl = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_ETH3_SLV_MDC of instance: netc"]
    Alt9NetcEth3SlvMdc = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: QTIMER3_TIMER2 of instance: qtimer3"]
    Alt10Qtimer3Timer2 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_RX_DV_1 of instance: ecat"]
    Alt12EcatPt1RxDv = 0x0c,
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
    Alt0SemcClkx1 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: NETC_PINMUX_ETH4_RX_ER of instance: netc_pinmux"]
    Alt1NetcPinmuxEth4RxEr = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: LPUART5_RTS_B of instance: lpuart5"]
    Alt2Lpuart5RtsB = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: NETC_ETH0_COL of instance: netc"]
    Alt3NetcEth0Col = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: NETC_EMDIO of instance: netc"]
    Alt4NetcEmdio = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO30 of instance: gpio3"]
    Alt5Gpio3Io30 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: XBAR1_XBAR_INOUT37 of instance: xbar1"]
    Alt6Xbar1XbarInout37 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: LPI2C3_SDA of instance: lpi2c3"]
    Alt8Lpi2c3Sda = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_ETH3_SLV_MDIO of instance: netc"]
    Alt9NetcEth3SlvMdio = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: QTIMER3_TIMER3 of instance: qtimer3"]
    Alt10Qtimer3Timer3 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_RX_ER_1 of instance: ecat"]
    Alt12EcatPt1RxEr = 0x0c,
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
    Alt0Usdhc1Cmd = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: SINC1_EMCLK02 of instance: sinc1"]
    Alt1Sinc1Emclk2 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XBAR1_XBAR_INOUT20 of instance: xbar1"]
    Alt2Xbar1XbarInout20 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPTMR2_ALT1 of instance: lptmr2"]
    Alt3Lptmr2Alt1 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: XSPI_SLV_CS of instance: xspi_slv"]
    Alt4XspiSlvCs = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO04 of instance: gpio5"]
    Alt5Gpio5Io4 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPSPI3_PCS0 of instance: lpspi3"]
    Alt6Lpspi3Pcs0 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: KPP_ROW07 of instance: kpp"]
    Alt8KppRow7 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: CCM_CLKO1 of instance: ccm"]
    Alt12CcmClko1 = 0x0c,
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
    Alt0Usdhc1Clk = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: SINC1_EMBIT02 of instance: sinc1"]
    Alt1Sinc1Embit2 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XBAR1_XBAR_INOUT21 of instance: xbar1"]
    Alt2Xbar1XbarInout21 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPTMR2_ALT2 of instance: lptmr2"]
    Alt3Lptmr2Alt2 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: XSPI_SLV_CLK of instance: xspi_slv"]
    Alt4XspiSlvClk = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO05 of instance: gpio5"]
    Alt5Gpio5Io5 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPSPI3_SCK of instance: lpspi3"]
    Alt6Lpspi3Sck = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: KPP_COL07 of instance: kpp"]
    Alt8KppCol7 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: CCM_CLKO2 of instance: ccm"]
    Alt12CcmClko2 = 0x0c,
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
    Alt0Usdhc1Data0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: SINC1_EMCLK03 of instance: sinc1"]
    Alt1Sinc1Emclk3 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XBAR1_XBAR_INOUT22 of instance: xbar1"]
    Alt2Xbar1XbarInout22 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPTMR2_ALT3 of instance: lptmr2"]
    Alt3Lptmr2Alt3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: XSPI_SLV_DATA04 of instance: xspi_slv"]
    Alt4XspiSlvData4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO06 of instance: gpio5"]
    Alt5Gpio5Io6 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPSPI3_SDO of instance: lpspi3"]
    Alt6Lpspi3Sdo = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: KPP_ROW06 of instance: kpp"]
    Alt8KppRow6 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: FLEXSPI1_BUS2BIT_A_SS1_B of instance: flexspi1_bus2bit"]
    Alt9Flexspi1Bus2bitASs1B = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_RESET_OUT of instance: ecat"]
    Alt12EcatResetOut = 0x0c,
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
    Alt0Usdhc1Data1 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: SINC1_EMBIT03 of instance: sinc1"]
    Alt1Sinc1Embit3 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XBAR1_XBAR_INOUT23 of instance: xbar1"]
    Alt2Xbar1XbarInout23 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPTMR3_ALT1 of instance: lptmr3"]
    Alt3Lptmr3Alt1 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: XSPI_SLV_DATA05 of instance: xspi_slv"]
    Alt4XspiSlvData5 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO07 of instance: gpio5"]
    Alt5Gpio5Io7 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPSPI3_SDI of instance: lpspi3"]
    Alt6Lpspi3Sdi = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: KPP_COL06 of instance: kpp"]
    Alt8KppCol6 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: FLEXSPI1_BUS2BIT_B_SS1_B of instance: flexspi1_bus2bit"]
    Alt9Flexspi1Bus2bitBSs1B = 0x09,
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
    Alt0Usdhc1Data2 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: SINC_FILTER_GLUE1_BREAK of instance: sinc_filter_glue1"]
    Alt1SincFilterGlue1Break = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SINC2_EMCLK02 of instance: sinc2"]
    Alt2Sinc2Emclk2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPTMR3_ALT2 of instance: lptmr3"]
    Alt3Lptmr3Alt2 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: XSPI_SLV_DATA06 of instance: xspi_slv"]
    Alt4XspiSlvData6 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO08 of instance: gpio5"]
    Alt5Gpio5Io8 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPSPI3_PCS1 of instance: lpspi3"]
    Alt6Lpspi3Pcs1 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: FLEXSPI1_BUS2BIT_B_SS0_B of instance: flexspi1_bus2bit"]
    Alt8Flexspi1Bus2bitBSs0B = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: FLEXSPI1_BUS2BIT_A_SS1_B of instance: flexspi1_bus2bit"]
    Alt9Flexspi1Bus2bitASs1B = 0x09,
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
    Alt0Usdhc1Data3 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: SINC2_EMBIT02 of instance: sinc2"]
    Alt2Sinc2Embit2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: LPTMR3_ALT3 of instance: lptmr3"]
    Alt3Lptmr3Alt3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: XSPI_SLV_DATA07 of instance: xspi_slv"]
    Alt4XspiSlvData7 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO09 of instance: gpio5"]
    Alt5Gpio5Io9 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPSPI3_PCS2 of instance: lpspi3"]
    Alt6Lpspi3Pcs2 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: FLEXSPI1_BUS2BIT_B_SS0_B of instance: flexspi1_bus2bit"]
    Alt9Flexspi1Bus2bitBSs0B = 0x09,
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
    Alt0Usdhc2Data3 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXSPI1_BUS2BIT_B_DATA04 of instance: flexspi1_bus2bit"]
    Alt1Flexspi1Bus2bitBData4 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XSPI_SLV_DATA04 of instance: xspi_slv"]
    Alt2XspiSlvData4 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: XBAR1_XBAR_INOUT17 of instance: xbar1"]
    Alt3Xbar1XbarInout17 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: KPP_ROW01 of instance: kpp"]
    Alt4KppRow1 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO10 of instance: gpio5"]
    Alt5Gpio5Io10 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPSPI3_PCS3 of instance: lpspi3"]
    Alt6Lpspi3Pcs3 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_1588_CLK of instance: netc_clkgen"]
    Alt8NetcClkgenTmr1588Clk = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPUART8_TX of instance: lpuart8"]
    Alt9Lpuart8Tx = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: MIC_BITSTREAM00 of instance: mic"]
    Alt12MicBitstream0 = 0x0c,
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
    Alt0Usdhc2Data2 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXSPI1_BUS2BIT_B_DATA05 of instance: flexspi1_bus2bit"]
    Alt1Flexspi1Bus2bitBData5 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XSPI_SLV_DATA05 of instance: xspi_slv"]
    Alt2XspiSlvData5 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER6_TIMER0 of instance: qtimer6"]
    Alt3Qtimer6Timer0 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: KPP_COL01 of instance: kpp"]
    Alt4KppCol1 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO11 of instance: gpio5"]
    Alt5Gpio5Io11 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_TMR_GCLK of instance: netc"]
    Alt8NetcTmr1588Gclk = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPUART8_RX of instance: lpuart8"]
    Alt9Lpuart8Rx = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: MIC_BITSTREAM01 of instance: mic"]
    Alt12MicBitstream1 = 0x0c,
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
    Alt0Usdhc2Data1 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXSPI1_BUS2BIT_B_DATA06 of instance: flexspi1_bus2bit"]
    Alt1Flexspi1Bus2bitBData6 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XSPI_SLV_DATA06 of instance: xspi_slv"]
    Alt2XspiSlvData6 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER6_TIMER1 of instance: qtimer6"]
    Alt3Qtimer6Timer1 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: KPP_ROW00 of instance: kpp"]
    Alt4KppRow0 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO12 of instance: gpio5"]
    Alt5Gpio5Io12 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_TMR_ALARM1 of instance: netc"]
    Alt8NetcTmr1588Alarm1 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPUART8_CTS_B of instance: lpuart8"]
    Alt9Lpuart8CtsB = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: MIC_BITSTREAM02 of instance: mic"]
    Alt12MicBitstream2 = 0x0c,
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
    Alt0Usdhc2Data0 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXSPI1_BUS2BIT_B_DATA07 of instance: flexspi1_bus2bit"]
    Alt1Flexspi1Bus2bitBData7 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XSPI_SLV_DATA07 of instance: xspi_slv"]
    Alt2XspiSlvData7 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER6_TIMER2 of instance: qtimer6"]
    Alt3Qtimer6Timer2 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: KPP_COL00 of instance: kpp"]
    Alt4KppCol0 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO13 of instance: gpio5"]
    Alt5Gpio5Io13 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_TMR_ALARM2 of instance: netc"]
    Alt8NetcTmr1588Alarm2 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: LPUART8_RTS_B of instance: lpuart8"]
    Alt9Lpuart8RtsB = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: MIC_BITSTREAM03 of instance: mic"]
    Alt12MicBitstream3 = 0x0c,
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
    Alt0Usdhc2Clk = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXSPI1_BUS2BIT_B_SS1_B of instance: flexspi1_bus2bit"]
    Alt1Flexspi1Bus2bitBSs1B = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER7_TIMER0 of instance: qtimer7"]
    Alt3Qtimer7Timer0 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: KPP_ROW03 of instance: kpp"]
    Alt4KppRow3 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO14 of instance: gpio5"]
    Alt5Gpio5Io14 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPUART5_RI_B of instance: lpuart5"]
    Alt6Lpuart5RiB = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_TMR_PP1 of instance: netc"]
    Alt8NetcTmr1588Pp1 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: MIC_CLK of instance: mic"]
    Alt12MicClk = 0x0c,
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
    Alt0Usdhc2Cmd = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXSPI1_BUS2BIT_B_DQS of instance: flexspi1_bus2bit"]
    Alt1Flexspi1Bus2bitBDqs = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XSPI_SLV_DQS of instance: xspi_slv"]
    Alt2XspiSlvDqs = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER7_TIMER1 of instance: qtimer7"]
    Alt3Qtimer7Timer1 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPSPI4_PCS3 of instance: lpspi4"]
    Alt4Lpspi4Pcs3 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO15 of instance: gpio5"]
    Alt5Gpio5Io15 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPUART5_DTR_B of instance: lpuart5"]
    Alt6Lpuart5DtrB = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_TMR_PP2 of instance: netc"]
    Alt8NetcTmr1588Pp2 = 0x08,
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
    Alt0Usdhc2ResetB = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXSPI1_BUS2BIT_B_SS0_B of instance: flexspi1_bus2bit"]
    Alt1Flexspi1Bus2bitBSs0B = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XSPI_SLV_CS of instance: xspi_slv"]
    Alt2XspiSlvCs = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER7_TIMER2 of instance: qtimer7"]
    Alt3Qtimer7Timer2 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPSPI4_PCS2 of instance: lpspi4"]
    Alt4Lpspi4Pcs2 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO16 of instance: gpio5"]
    Alt5Gpio5Io16 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPUART5_CTS_B of instance: lpuart5"]
    Alt6Lpuart5CtsB = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_TMR_PP3 of instance: netc"]
    Alt8NetcTmr1588Pp3 = 0x08,
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
    Alt0Usdhc2Strobe = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXSPI1_BUS2BIT_B_SCLK of instance: flexspi1_bus2bit"]
    Alt1Flexspi1Bus2bitBSclk = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XSPI_SLV_CLK of instance: xspi_slv"]
    Alt2XspiSlvClk = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER7_TIMER3 of instance: qtimer7"]
    Alt3Qtimer7Timer3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPSPI4_PCS1 of instance: lpspi4"]
    Alt4Lpspi4Pcs1 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO17 of instance: gpio5"]
    Alt5Gpio5Io17 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPUART5_RTS_B of instance: lpuart5"]
    Alt6Lpuart5RtsB = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_TMR_ALARM1 of instance: netc"]
    Alt8NetcTmr1588Alarm1 = 0x08,
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
    Alt0Usdhc2Data4 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXSPI1_BUS2BIT_B_DATA00 of instance: flexspi1_bus2bit"]
    Alt1Flexspi1Bus2bitBData0 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XSPI_SLV_DATA00 of instance: xspi_slv"]
    Alt2XspiSlvData0 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER8_TIMER0 of instance: qtimer8"]
    Alt3Qtimer8Timer0 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPSPI4_SCK of instance: lpspi4"]
    Alt4Lpspi4Sck = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO18 of instance: gpio5"]
    Alt5Gpio5Io18 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPUART5_TX of instance: lpuart5"]
    Alt6Lpuart5Tx = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_TMR_ALARM2 of instance: netc"]
    Alt8NetcTmr1588Alarm2 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_TMR_PP2 of instance: netc"]
    Alt9NetcTmr1588Pp2 = 0x09,
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
    Alt0Usdhc2Data5 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXSPI1_BUS2BIT_B_DATA01 of instance: flexspi1_bus2bit"]
    Alt1Flexspi1Bus2bitBData1 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XSPI_SLV_DATA01 of instance: xspi_slv"]
    Alt2XspiSlvData1 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER8_TIMER1 of instance: qtimer8"]
    Alt3Qtimer8Timer1 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPSPI4_PCS0 of instance: lpspi4"]
    Alt4Lpspi4Pcs0 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO19 of instance: gpio5"]
    Alt5Gpio5Io19 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPUART5_RX of instance: lpuart5"]
    Alt6Lpuart5Rx = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_TMR_PP1 of instance: netc"]
    Alt9NetcTmr1588Pp1 = 0x09,
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
    Alt0Usdhc2Data6 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXSPI1_BUS2BIT_B_DATA02 of instance: flexspi1_bus2bit"]
    Alt1Flexspi1Bus2bitBData2 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XSPI_SLV_DATA02 of instance: xspi_slv"]
    Alt2XspiSlvData2 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER8_TIMER2 of instance: qtimer8"]
    Alt3Qtimer8Timer2 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPSPI4_SDO of instance: lpspi4"]
    Alt4Lpspi4Sdo = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO20 of instance: gpio5"]
    Alt5Gpio5Io20 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPUART5_DCD_B of instance: lpuart5"]
    Alt6Lpuart5DcdB = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_TMR_TRIG2 of instance: netc"]
    Alt8NetcTmr1588Trig2 = 0x08,
    #[doc = "Select mux mode: ALT9 mux port: NETC_TMR_PP3 of instance: netc"]
    Alt9NetcTmr1588Pp3 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: NETC_EMDIO of instance: netc"]
    Alt10NetcEmdio = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_MDIO of instance: ecat"]
    Alt12EcatMdio = 0x0c,
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
    Alt0Usdhc2Data7 = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXSPI1_BUS2BIT_B_DATA03 of instance: flexspi1_bus2bit"]
    Alt1Flexspi1Bus2bitBData3 = 0x01,
    #[doc = "Select mux mode: ALT2 mux port: XSPI_SLV_DATA03 of instance: xspi_slv"]
    Alt2XspiSlvData3 = 0x02,
    #[doc = "Select mux mode: ALT3 mux port: QTIMER8_TIMER3 of instance: qtimer8"]
    Alt3Qtimer8Timer3 = 0x03,
    #[doc = "Select mux mode: ALT4 mux port: LPSPI4_SDI of instance: lpspi4"]
    Alt4Lpspi4Sdi = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO21 of instance: gpio5"]
    Alt5Gpio5Io21 = 0x05,
    #[doc = "Select mux mode: ALT6 mux port: LPUART5_DSR_B of instance: lpuart5"]
    Alt6Lpuart5DsrB = 0x06,
    #[doc = "Select mux mode: ALT7 mux port: SFA_ATX_CLK_OUT of instance: sfa"]
    Alt7SfaAtxClkOut = 0x07,
    #[doc = "Select mux mode: ALT8 mux port: NETC_TMR_TRIG1 of instance: netc"]
    Alt8NetcTmr1588Trig1 = 0x08,
    _RESERVED_9 = 0x09,
    #[doc = "Select mux mode: ALT10 mux port: NETC_EMDC of instance: netc"]
    Alt10NetcEmdc = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Select mux mode: ALT12 mux port: ECAT_MCLK of instance: ecat"]
    Alt12EcatMdc = 0x0c,
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
    Alt0Flexspi1Bus2bitADqs = 0x0,
    #[doc = "Select mux mode: ALT1 mux port: FLEXSPI1_BUS2BIT_B_DQS of instance: flexspi1_bus2bit"]
    Alt1Flexspi1Bus2bitBDqs = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO22 of instance: gpio5"]
    Alt5Gpio5Io22 = 0x05,
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
    SelectGpioAd06Alt0 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_30 for Mode: ALT1"]
    SelectGpioAd30Alt1 = 0x01,
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
    SelectGpioAd11Alt0 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_35 for Mode: ALT1"]
    SelectGpioAd35Alt1 = 0x01,
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
    SelectGpioAd09Alt0 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_33 for Mode: ALT1"]
    SelectGpioAd33Alt1 = 0x01,
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
    SelectGpioAd08Alt0 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_32 for Mode: ALT1"]
    SelectGpioAd32Alt1 = 0x01,
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
    SelectGpioAd32Alt4 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_08 for Mode: ALT2"]
    SelectGpioB108Alt2 = 0x01,
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
    SelectGpioAd33Alt4 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_09 for Mode: ALT2"]
    SelectGpioB109Alt2 = 0x01,
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
    SelectGpioEmcB201Alt1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_13 for Mode: ALT7"]
    SelectGpioAd13Alt7 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_26 for Mode: ALT9"]
    SelectGpioAd26Alt9 = 0x02,
    #[doc = "Selecting Pad: GPIO_AD_29 for Mode: ALT2"]
    SelectGpioAd29Alt2 = 0x03,
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
    SelectGpioEmcB202Alt1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_14 for Mode: ALT7"]
    SelectGpioAd14Alt7 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_27 for Mode: ALT9"]
    SelectGpioAd27Alt9 = 0x02,
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
    SelectGpioEmcB130Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_16 for Mode: ALT6"]
    SelectGpioEmcB216Alt6 = 0x01,
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
    SelectGpioEmcB139Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_B2_17 for Mode: ALT6"]
    SelectGpioEmcB217Alt6 = 0x01,
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
    SelectGpioAd33Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_00 for Mode: ALT3"]
    SelectGpioSdB200Alt3 = 0x01,
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
    SelectGpioAd12Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_34 for Mode: ALT2"]
    SelectGpioAd34Alt2 = 0x01,
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
    SelectGpioAd19Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_35 for Mode: ALT2"]
    SelectGpioAd35Alt2 = 0x01,
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
    SelectGpioEmcB200Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B1_00 for Mode: ALT2"]
    SelectGpioSdB100Alt2 = 0x01,
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
    SelectGpioEmcB201Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B1_01 for Mode: ALT2"]
    SelectGpioSdB101Alt2 = 0x01,
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
    SelectGpioEmcB202Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B1_02 for Mode: ALT2"]
    SelectGpioSdB102Alt2 = 0x01,
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
    SelectGpioEmcB203Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B1_03 for Mode: ALT2"]
    SelectGpioSdB103Alt2 = 0x01,
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
    SelectGpioEmcB204Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_30 for Mode: ALT9"]
    SelectGpioAd30Alt9 = 0x01,
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
    SelectGpioEmcB205Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_31 for Mode: ALT9"]
    SelectGpioAd31Alt9 = 0x01,
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
    SelectGpioEmcB206Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_00 for Mode: ALT4"]
    SelectGpioB100Alt4 = 0x01,
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
    SelectGpioEmcB207Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_01 for Mode: ALT4"]
    SelectGpioB101Alt4 = 0x01,
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
    SelectGpioEmcB208Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_02 for Mode: ALT4"]
    SelectGpioB102Alt4 = 0x01,
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
    SelectGpioEmcB209Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_03 for Mode: ALT4"]
    SelectGpioB103Alt4 = 0x01,
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
    SelectGpioEmcB210Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_04 for Mode: ALT4"]
    SelectGpioB104Alt4 = 0x01,
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
    SelectGpioEmcB211Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_05 for Mode: ALT4"]
    SelectGpioB105Alt4 = 0x01,
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
    SelectGpioEmcB212Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_06 for Mode: ALT4"]
    SelectGpioB106Alt4 = 0x01,
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
    SelectGpioEmcB213Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_07 for Mode: ALT4"]
    SelectGpioB107Alt4 = 0x01,
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
    SelectGpioEmcB214Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_10 for Mode: ALT4"]
    SelectGpioB110Alt4 = 0x01,
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
    SelectGpioEmcB215Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_11 for Mode: ALT4"]
    SelectGpioB111Alt4 = 0x01,
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
    SelectGpioEmcB219Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_08 for Mode: ALT4"]
    SelectGpioB108Alt4 = 0x01,
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
    SelectGpioEmcB220Alt6 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_09 for Mode: ALT4"]
    SelectGpioB109Alt4 = 0x01,
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
    SelectGpioSdB100Alt4 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_06 for Mode: ALT2"]
    SelectGpioSdB206Alt2 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_09 for Mode: ALT10"]
    SelectGpioB209Alt10 = 0x02,
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
    SelectGpioSdB205Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_07 for Mode: ALT10"]
    SelectGpioB207Alt10 = 0x01,
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
    SelectGpioSdB208Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_10 for Mode: ALT10"]
    SelectGpioB210Alt10 = 0x01,
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
    SelectGpioSdB209Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_11 for Mode: ALT10"]
    SelectGpioB211Alt10 = 0x01,
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
    SelectGpioSdB210Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_12 for Mode: ALT10"]
    SelectGpioB212Alt10 = 0x01,
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
    SelectGpioSdB211Alt2 = 0x0,
    #[doc = "Selecting Pad: GPIO_B2_13 for Mode: ALT10"]
    SelectGpioB213Alt10 = 0x01,
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
    SelectGpioSdB102Alt4 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_00 for Mode: ALT2"]
    SelectGpioSdB200Alt2 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_03 for Mode: ALT10"]
    SelectGpioB203Alt10 = 0x02,
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
    SelectGpioSdB103Alt4 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_01 for Mode: ALT2"]
    SelectGpioSdB201Alt2 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_04 for Mode: ALT10"]
    SelectGpioB204Alt10 = 0x02,
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
    SelectGpioSdB104Alt4 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_02 for Mode: ALT2"]
    SelectGpioSdB202Alt2 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_05 for Mode: ALT10"]
    SelectGpioB205Alt10 = 0x02,
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
    SelectGpioSdB105Alt4 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_03 for Mode: ALT2"]
    SelectGpioSdB203Alt2 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_06 for Mode: ALT10"]
    SelectGpioB206Alt10 = 0x02,
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
    SelectGpioSdB101Alt4 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B2_07 for Mode: ALT2"]
    SelectGpioSdB207Alt2 = 0x01,
    #[doc = "Selecting Pad: GPIO_B2_08 for Mode: ALT10"]
    SelectGpioB208Alt10 = 0x02,
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
