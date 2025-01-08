#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BlkholeModeB {
    #[doc = "AON Domain SSI master will enter into blackhole mode"]
    ENABLE = 0x0,
    #[doc = "AON Domain SSI master will exit from blackhole mode"]
    DISABLE = 0x01,
}
impl BlkholeModeB {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BlkholeModeB {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BlkholeModeB {
    #[inline(always)]
    fn from(val: u8) -> BlkholeModeB {
        BlkholeModeB::from_bits(val)
    }
}
impl From<BlkholeModeB> for u8 {
    #[inline(always)]
    fn from(val: BlkholeModeB) -> u8 {
        BlkholeModeB::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcdcInLowVol {
    #[doc = "Voltage on DCDC_IN is higher than 2.6V"]
    NO = 0x0,
    #[doc = "Voltage on DCDC_IN is lower than 2.6V"]
    OVER = 0x01,
}
impl DcdcInLowVol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcdcInLowVol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcdcInLowVol {
    #[inline(always)]
    fn from(val: u8) -> DcdcInLowVol {
        DcdcInLowVol::from_bits(val)
    }
}
impl From<DcdcInLowVol> for u8 {
    #[inline(always)]
    fn from(val: DcdcInLowVol) -> u8 {
        DcdcInLowVol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcdcOverCur {
    #[doc = "No Overcurrent on DCDC output"]
    NO = 0x0,
    #[doc = "Overcurrent on DCDC output"]
    OVER = 0x01,
}
impl DcdcOverCur {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcdcOverCur {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcdcOverCur {
    #[inline(always)]
    fn from(val: u8) -> DcdcOverCur {
        DcdcOverCur::from_bits(val)
    }
}
impl From<DcdcOverCur> for u8 {
    #[inline(always)]
    fn from(val: DcdcOverCur) -> u8 {
        DcdcOverCur::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcdcOverVol {
    #[doc = "No Overvoltage on DCDC VDDLP0 or VDDLP8 output"]
    NO = 0x0,
    #[doc = "Overvoltage on DCDC VDDLP0 or VDDLP8 output"]
    OVERVOLTAGE = 0x01,
}
impl DcdcOverVol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcdcOverVol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcdcOverVol {
    #[inline(always)]
    fn from(val: u8) -> DcdcOverVol {
        DcdcOverVol::from_bits(val)
    }
}
impl From<DcdcOverVol> for u8 {
    #[inline(always)]
    fn from(val: DcdcOverVol) -> u8 {
        DcdcOverVol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcdcStatusCaptClr {
    #[doc = "No change"]
    OVER = 0x0,
    #[doc = "Clear the 3 bits of DCDC captured status: DCDC_OVER_VOL, DCDC_OVER_CUR, and DCDC_IN_LOW_VOL"]
    NO = 0x01,
}
impl DcdcStatusCaptClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcdcStatusCaptClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcdcStatusCaptClr {
    #[inline(always)]
    fn from(val: u8) -> DcdcStatusCaptClr {
        DcdcStatusCaptClr::from_bits(val)
    }
}
impl From<DcdcStatusCaptClr> for u8 {
    #[inline(always)]
    fn from(val: DcdcStatusCaptClr) -> u8 {
        DcdcStatusCaptClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3cOnChipStrongPullDis {
    #[doc = "On-chip strong pull is enabled"]
    ENABLE = 0x0,
    #[doc = "On-chip strong pull is disabled"]
    DISABLE = 0x01,
}
impl I3cOnChipStrongPullDis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3cOnChipStrongPullDis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3cOnChipStrongPullDis {
    #[inline(always)]
    fn from(val: u8) -> I3cOnChipStrongPullDis {
        I3cOnChipStrongPullDis::from_bits(val)
    }
}
impl From<I3cOnChipStrongPullDis> for u8 {
    #[inline(always)]
    fn from(val: I3cOnChipStrongPullDis) -> u8 {
        I3cOnChipStrongPullDis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Can1 {
    #[doc = "CAN1 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "CAN1 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Can1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Can1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Can1 {
    #[inline(always)]
    fn from(val: u8) -> M33Can1 {
        M33Can1::from_bits(val)
    }
}
impl From<M33Can1> for u8 {
    #[inline(always)]
    fn from(val: M33Can1) -> u8 {
        M33Can1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Can3 {
    #[doc = "CAN3 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "CAN3 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Can3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Can3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Can3 {
    #[inline(always)]
    fn from(val: u8) -> M33Can3 {
        M33Can3::from_bits(val)
    }
}
impl From<M33Can3> for u8 {
    #[inline(always)]
    fn from(val: M33Can3) -> u8 {
        M33Can3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Edma3 {
    #[doc = "EDMA3 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "EDMA3 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Edma3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Edma3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Edma3 {
    #[inline(always)]
    fn from(val: u8) -> M33Edma3 {
        M33Edma3::from_bits(val)
    }
}
impl From<M33Edma3> for u8 {
    #[inline(always)]
    fn from(val: M33Edma3) -> u8 {
        M33Edma3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Gpt1 {
    #[doc = "GPT1 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "GPT1 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Gpt1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Gpt1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Gpt1 {
    #[inline(always)]
    fn from(val: u8) -> M33Gpt1 {
        M33Gpt1::from_bits(val)
    }
}
impl From<M33Gpt1> for u8 {
    #[inline(always)]
    fn from(val: M33Gpt1) -> u8 {
        M33Gpt1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33I3c1 {
    #[doc = "I3C1 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "I3C1 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33I3c1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33I3c1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33I3c1 {
    #[inline(always)]
    fn from(val: u8) -> M33I3c1 {
        M33I3c1::from_bits(val)
    }
}
impl From<M33I3c1> for u8 {
    #[inline(always)]
    fn from(val: M33I3c1) -> u8 {
        M33I3c1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Lpi2c1 {
    #[doc = "LPI2C1 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "LPI2C1 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Lpi2c1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Lpi2c1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Lpi2c1 {
    #[inline(always)]
    fn from(val: u8) -> M33Lpi2c1 {
        M33Lpi2c1::from_bits(val)
    }
}
impl From<M33Lpi2c1> for u8 {
    #[inline(always)]
    fn from(val: M33Lpi2c1) -> u8 {
        M33Lpi2c1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Lpi2c2 {
    #[doc = "LPI2C2 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "LPI2C2 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Lpi2c2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Lpi2c2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Lpi2c2 {
    #[inline(always)]
    fn from(val: u8) -> M33Lpi2c2 {
        M33Lpi2c2::from_bits(val)
    }
}
impl From<M33Lpi2c2> for u8 {
    #[inline(always)]
    fn from(val: M33Lpi2c2) -> u8 {
        M33Lpi2c2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Lpit1 {
    #[doc = "LPIT1 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "LPIT1 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Lpit1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Lpit1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Lpit1 {
    #[inline(always)]
    fn from(val: u8) -> M33Lpit1 {
        M33Lpit1::from_bits(val)
    }
}
impl From<M33Lpit1> for u8 {
    #[inline(always)]
    fn from(val: M33Lpit1) -> u8 {
        M33Lpit1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Lpspi1 {
    #[doc = "LPSPI1 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "LPSPI1 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Lpspi1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Lpspi1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Lpspi1 {
    #[inline(always)]
    fn from(val: u8) -> M33Lpspi1 {
        M33Lpspi1::from_bits(val)
    }
}
impl From<M33Lpspi1> for u8 {
    #[inline(always)]
    fn from(val: M33Lpspi1) -> u8 {
        M33Lpspi1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Lpspi2 {
    #[doc = "LPSPI2 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "LPSPI2 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Lpspi2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Lpspi2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Lpspi2 {
    #[inline(always)]
    fn from(val: u8) -> M33Lpspi2 {
        M33Lpspi2::from_bits(val)
    }
}
impl From<M33Lpspi2> for u8 {
    #[inline(always)]
    fn from(val: M33Lpspi2) -> u8 {
        M33Lpspi2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Lptmr1 {
    #[doc = "LPTMR1 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "LPTMR1 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Lptmr1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Lptmr1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Lptmr1 {
    #[inline(always)]
    fn from(val: u8) -> M33Lptmr1 {
        M33Lptmr1::from_bits(val)
    }
}
impl From<M33Lptmr1> for u8 {
    #[inline(always)]
    fn from(val: M33Lptmr1) -> u8 {
        M33Lptmr1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Sai1 {
    #[doc = "SAI1 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "SAI1 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Sai1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Sai1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Sai1 {
    #[inline(always)]
    fn from(val: u8) -> M33Sai1 {
        M33Sai1::from_bits(val)
    }
}
impl From<M33Sai1> for u8 {
    #[inline(always)]
    fn from(val: M33Sai1) -> u8 {
        M33Sai1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33SleepSel {
    #[doc = "Select SLEEPING as request source"]
    SLEEPING = 0x0,
    #[doc = "Select SLEEPDEEP as request source"]
    SLEEPDEEP = 0x01,
}
impl M33SleepSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33SleepSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33SleepSel {
    #[inline(always)]
    fn from(val: u8) -> M33SleepSel {
        M33SleepSel::from_bits(val)
    }
}
impl From<M33SleepSel> for u8 {
    #[inline(always)]
    fn from(val: M33SleepSel) -> u8 {
        M33SleepSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Tpm1 {
    #[doc = "TPM1 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "TPM1 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Tpm1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Tpm1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Tpm1 {
    #[inline(always)]
    fn from(val: u8) -> M33Tpm1 {
        M33Tpm1::from_bits(val)
    }
}
impl From<M33Tpm1> for u8 {
    #[inline(always)]
    fn from(val: M33Tpm1) -> u8 {
        M33Tpm1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Tpm2 {
    #[doc = "TPM2 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "TPM2 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Tpm2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Tpm2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Tpm2 {
    #[inline(always)]
    fn from(val: u8) -> M33Tpm2 {
        M33Tpm2::from_bits(val)
    }
}
impl From<M33Tpm2> for u8 {
    #[inline(always)]
    fn from(val: M33Tpm2) -> u8 {
        M33Tpm2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Wdog1 {
    #[doc = "WDOG1 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "WDOG1 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Wdog1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Wdog1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Wdog1 {
    #[inline(always)]
    fn from(val: u8) -> M33Wdog1 {
        M33Wdog1::from_bits(val)
    }
}
impl From<M33Wdog1> for u8 {
    #[inline(always)]
    fn from(val: M33Wdog1) -> u8 {
        M33Wdog1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33Wdog2 {
    #[doc = "WDOG2 does not enter debug halted mode with CM33"]
    MASK = 0x0,
    #[doc = "WDOG2 enters debug halted mode when CM33 is debug halted"]
    UNMASK = 0x01,
}
impl M33Wdog2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33Wdog2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33Wdog2 {
    #[inline(always)]
    fn from(val: u8) -> M33Wdog2 {
        M33Wdog2::from_bits(val)
    }
}
impl From<M33Wdog2> for u8 {
    #[inline(always)]
    fn from(val: M33Wdog2) -> u8 {
        M33Wdog2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Can1 {
    #[doc = "CAN1 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "CAN1 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Can1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Can1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Can1 {
    #[inline(always)]
    fn from(val: u8) -> M7Can1 {
        M7Can1::from_bits(val)
    }
}
impl From<M7Can1> for u8 {
    #[inline(always)]
    fn from(val: M7Can1) -> u8 {
        M7Can1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Can3 {
    #[doc = "CAN3 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "CAN3 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Can3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Can3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Can3 {
    #[inline(always)]
    fn from(val: u8) -> M7Can3 {
        M7Can3::from_bits(val)
    }
}
impl From<M7Can3> for u8 {
    #[inline(always)]
    fn from(val: M7Can3) -> u8 {
        M7Can3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Edma3 {
    #[doc = "EDMA3 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "EDMA3 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Edma3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Edma3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Edma3 {
    #[inline(always)]
    fn from(val: u8) -> M7Edma3 {
        M7Edma3::from_bits(val)
    }
}
impl From<M7Edma3> for u8 {
    #[inline(always)]
    fn from(val: M7Edma3) -> u8 {
        M7Edma3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Gpt1 {
    #[doc = "GPT1 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "GPT1 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Gpt1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Gpt1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Gpt1 {
    #[inline(always)]
    fn from(val: u8) -> M7Gpt1 {
        M7Gpt1::from_bits(val)
    }
}
impl From<M7Gpt1> for u8 {
    #[inline(always)]
    fn from(val: M7Gpt1) -> u8 {
        M7Gpt1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7I3c1 {
    #[doc = "I3C1 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "I3C1 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7I3c1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7I3c1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7I3c1 {
    #[inline(always)]
    fn from(val: u8) -> M7I3c1 {
        M7I3c1::from_bits(val)
    }
}
impl From<M7I3c1> for u8 {
    #[inline(always)]
    fn from(val: M7I3c1) -> u8 {
        M7I3c1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Lpi2c1 {
    #[doc = "LPI2C1 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "LPI2C1 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Lpi2c1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Lpi2c1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Lpi2c1 {
    #[inline(always)]
    fn from(val: u8) -> M7Lpi2c1 {
        M7Lpi2c1::from_bits(val)
    }
}
impl From<M7Lpi2c1> for u8 {
    #[inline(always)]
    fn from(val: M7Lpi2c1) -> u8 {
        M7Lpi2c1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Lpi2c2 {
    #[doc = "LPI2C2 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "LPI2C2 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Lpi2c2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Lpi2c2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Lpi2c2 {
    #[inline(always)]
    fn from(val: u8) -> M7Lpi2c2 {
        M7Lpi2c2::from_bits(val)
    }
}
impl From<M7Lpi2c2> for u8 {
    #[inline(always)]
    fn from(val: M7Lpi2c2) -> u8 {
        M7Lpi2c2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Lpit1 {
    #[doc = "LPIT1 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "LPIT1 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Lpit1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Lpit1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Lpit1 {
    #[inline(always)]
    fn from(val: u8) -> M7Lpit1 {
        M7Lpit1::from_bits(val)
    }
}
impl From<M7Lpit1> for u8 {
    #[inline(always)]
    fn from(val: M7Lpit1) -> u8 {
        M7Lpit1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Lpspi1 {
    #[doc = "LPSPI1 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "LPSPI1 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Lpspi1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Lpspi1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Lpspi1 {
    #[inline(always)]
    fn from(val: u8) -> M7Lpspi1 {
        M7Lpspi1::from_bits(val)
    }
}
impl From<M7Lpspi1> for u8 {
    #[inline(always)]
    fn from(val: M7Lpspi1) -> u8 {
        M7Lpspi1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Lpspi2 {
    #[doc = "LPSPI2 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "LPSPI2 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Lpspi2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Lpspi2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Lpspi2 {
    #[inline(always)]
    fn from(val: u8) -> M7Lpspi2 {
        M7Lpspi2::from_bits(val)
    }
}
impl From<M7Lpspi2> for u8 {
    #[inline(always)]
    fn from(val: M7Lpspi2) -> u8 {
        M7Lpspi2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Lptmr1 {
    #[doc = "LPTMR1 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "LPTMR1 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Lptmr1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Lptmr1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Lptmr1 {
    #[inline(always)]
    fn from(val: u8) -> M7Lptmr1 {
        M7Lptmr1::from_bits(val)
    }
}
impl From<M7Lptmr1> for u8 {
    #[inline(always)]
    fn from(val: M7Lptmr1) -> u8 {
        M7Lptmr1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Sai1 {
    #[doc = "SAI1 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "SAI1 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Sai1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Sai1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Sai1 {
    #[inline(always)]
    fn from(val: u8) -> M7Sai1 {
        M7Sai1::from_bits(val)
    }
}
impl From<M7Sai1> for u8 {
    #[inline(always)]
    fn from(val: M7Sai1) -> u8 {
        M7Sai1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7SleepSel {
    #[doc = "Select SLEEPING as request source"]
    SLEEPING = 0x0,
    #[doc = "Select SLEEPDEEP as request source"]
    SLEEPDEEP = 0x01,
}
impl M7SleepSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7SleepSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7SleepSel {
    #[inline(always)]
    fn from(val: u8) -> M7SleepSel {
        M7SleepSel::from_bits(val)
    }
}
impl From<M7SleepSel> for u8 {
    #[inline(always)]
    fn from(val: M7SleepSel) -> u8 {
        M7SleepSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Tpm1 {
    #[doc = "TPM1 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "TPM1 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Tpm1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Tpm1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Tpm1 {
    #[inline(always)]
    fn from(val: u8) -> M7Tpm1 {
        M7Tpm1::from_bits(val)
    }
}
impl From<M7Tpm1> for u8 {
    #[inline(always)]
    fn from(val: M7Tpm1) -> u8 {
        M7Tpm1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Tpm2 {
    #[doc = "TPM2 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "TPM2 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Tpm2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Tpm2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Tpm2 {
    #[inline(always)]
    fn from(val: u8) -> M7Tpm2 {
        M7Tpm2::from_bits(val)
    }
}
impl From<M7Tpm2> for u8 {
    #[inline(always)]
    fn from(val: M7Tpm2) -> u8 {
        M7Tpm2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Wdog1 {
    #[doc = "WDOG1 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "WDOG1 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Wdog1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Wdog1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Wdog1 {
    #[inline(always)]
    fn from(val: u8) -> M7Wdog1 {
        M7Wdog1::from_bits(val)
    }
}
impl From<M7Wdog1> for u8 {
    #[inline(always)]
    fn from(val: M7Wdog1) -> u8 {
        M7Wdog1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7Wdog2 {
    #[doc = "WDOG2 does not enter debug halted mode with CM7"]
    MASK = 0x0,
    #[doc = "WDOG2 enters debug halted mode when CM7 is debug halted"]
    UNMASK = 0x01,
}
impl M7Wdog2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7Wdog2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7Wdog2 {
    #[inline(always)]
    fn from(val: u8) -> M7Wdog2 {
        M7Wdog2::from_bits(val)
    }
}
impl From<M7Wdog2> for u8 {
    #[inline(always)]
    fn from(val: M7Wdog2) -> u8 {
        M7Wdog2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OcotpBusy {
    #[doc = "OCOTP is not busy"]
    ENABLE = 0x0,
    #[doc = "OCOTP is busy"]
    DISABLE = 0x01,
}
impl OcotpBusy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OcotpBusy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OcotpBusy {
    #[inline(always)]
    fn from(val: u8) -> OcotpBusy {
        OcotpBusy::from_bits(val)
    }
}
impl From<OcotpBusy> for u8 {
    #[inline(always)]
    fn from(val: OcotpBusy) -> u8 {
        OcotpBusy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OcotpCalibrated {
    #[doc = "OCOTP is not calibrated"]
    ENABLE = 0x0,
    #[doc = "OCOTP is calibrated"]
    DISABLE = 0x01,
}
impl OcotpCalibrated {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OcotpCalibrated {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OcotpCalibrated {
    #[inline(always)]
    fn from(val: u8) -> OcotpCalibrated {
        OcotpCalibrated::from_bits(val)
    }
}
impl From<OcotpCalibrated> for u8 {
    #[inline(always)]
    fn from(val: OcotpCalibrated) -> u8 {
        OcotpCalibrated::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OsccaFuseReadDis {
    #[doc = "Read is allowed"]
    ENABLE = 0x0,
    #[doc = "Read is not allowed"]
    DISABLE = 0x01,
}
impl OsccaFuseReadDis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OsccaFuseReadDis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OsccaFuseReadDis {
    #[inline(always)]
    fn from(val: u8) -> OsccaFuseReadDis {
        OsccaFuseReadDis::from_bits(val)
    }
}
impl From<OsccaFuseReadDis> for u8 {
    #[inline(always)]
    fn from(val: OsccaFuseReadDis) -> u8 {
        OsccaFuseReadDis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1MclkDir {
    #[doc = "SAI1_MCLK is input signal"]
    INPUT = 0x0,
    #[doc = "SAI1_MCLK is output signal"]
    OUTPUT = 0x01,
}
impl Sai1MclkDir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1MclkDir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1MclkDir {
    #[inline(always)]
    fn from(val: u8) -> Sai1MclkDir {
        Sai1MclkDir::from_bits(val)
    }
}
impl From<Sai1MclkDir> for u8 {
    #[inline(always)]
    fn from(val: Sai1MclkDir) -> u8 {
        Sai1MclkDir::to_bits(val)
    }
}
