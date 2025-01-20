#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlControlBf4 {
    #[doc = "No Ack of Error Ind in AL status register"]
    BfVal0 = 0x0,
    #[doc = "Ack of Error Ind in AL status register"]
    BfVal1 = 0x01,
}
impl AlControlBf4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AlControlBf4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AlControlBf4 {
    #[inline(always)]
    fn from(val: u8) -> AlControlBf4 {
        AlControlBf4::from_bits(val)
    }
}
impl From<AlControlBf4> for u8 {
    #[inline(always)]
    fn from(val: AlControlBf4) -> u8 {
        AlControlBf4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlControlBf5 {
    #[doc = "No request"]
    BfVal0 = 0x0,
    #[doc = "Device Identification request"]
    BfVal1 = 0x01,
}
impl AlControlBf5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AlControlBf5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AlControlBf5 {
    #[inline(always)]
    fn from(val: u8) -> AlControlBf5 {
        AlControlBf5::from_bits(val)
    }
}
impl From<AlControlBf5> for u8 {
    #[inline(always)]
    fn from(val: AlControlBf5) -> u8 {
        AlControlBf5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlControlPdiBf4 {
    #[doc = "No Ack of Error Ind in AL status register"]
    BfVal0 = 0x0,
    #[doc = "Ack of Error Ind in AL status register"]
    BfVal1 = 0x01,
}
impl AlControlPdiBf4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AlControlPdiBf4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AlControlPdiBf4 {
    #[inline(always)]
    fn from(val: u8) -> AlControlPdiBf4 {
        AlControlPdiBf4::from_bits(val)
    }
}
impl From<AlControlPdiBf4> for u8 {
    #[inline(always)]
    fn from(val: AlControlPdiBf4) -> u8 {
        AlControlPdiBf4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlControlPdiBf5 {
    #[doc = "No request"]
    BfVal0 = 0x0,
    #[doc = "Device Identification request"]
    BfVal1 = 0x01,
}
impl AlControlPdiBf5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AlControlPdiBf5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AlControlPdiBf5 {
    #[inline(always)]
    fn from(val: u8) -> AlControlPdiBf5 {
        AlControlPdiBf5::from_bits(val)
    }
}
impl From<AlControlPdiBf5> for u8 {
    #[inline(always)]
    fn from(val: AlControlPdiBf5) -> u8 {
        AlControlPdiBf5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlEventRequestBf0 {
    #[doc = "No AL Control Register change"]
    BfVal0 = 0x0,
    #[doc = "AL Control Register has been written3"]
    BfVal1 = 0x01,
}
impl AlEventRequestBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AlEventRequestBf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AlEventRequestBf0 {
    #[inline(always)]
    fn from(val: u8) -> AlEventRequestBf0 {
        AlEventRequestBf0::from_bits(val)
    }
}
impl From<AlEventRequestBf0> for u8 {
    #[inline(always)]
    fn from(val: AlEventRequestBf0) -> u8 {
        AlEventRequestBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlEventRequestBf1 {
    #[doc = "No change on DC Latch Inputs"]
    BfVal0 = 0x0,
    #[doc = "At least one change on DC Latch Inputs"]
    BfVal1 = 0x01,
}
impl AlEventRequestBf1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AlEventRequestBf1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AlEventRequestBf1 {
    #[inline(always)]
    fn from(val: u8) -> AlEventRequestBf1 {
        AlEventRequestBf1::from_bits(val)
    }
}
impl From<AlEventRequestBf1> for u8 {
    #[inline(always)]
    fn from(val: AlEventRequestBf1) -> u8 {
        AlEventRequestBf1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlEventRequestBf10 {
    #[doc = "No SyncManager2 interrupt"]
    BfVal0 = 0x0,
    #[doc = "SyncManager 2 interrupt pending"]
    BfVal1 = 0x01,
}
impl AlEventRequestBf10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AlEventRequestBf10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AlEventRequestBf10 {
    #[inline(always)]
    fn from(val: u8) -> AlEventRequestBf10 {
        AlEventRequestBf10::from_bits(val)
    }
}
impl From<AlEventRequestBf10> for u8 {
    #[inline(always)]
    fn from(val: AlEventRequestBf10) -> u8 {
        AlEventRequestBf10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlEventRequestBf11 {
    #[doc = "No SyncManager 3 interrupt"]
    BfVal0 = 0x0,
    #[doc = "SyncManager 3 interrupt pending"]
    BfVal1 = 0x01,
}
impl AlEventRequestBf11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AlEventRequestBf11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AlEventRequestBf11 {
    #[inline(always)]
    fn from(val: u8) -> AlEventRequestBf11 {
        AlEventRequestBf11::from_bits(val)
    }
}
impl From<AlEventRequestBf11> for u8 {
    #[inline(always)]
    fn from(val: AlEventRequestBf11) -> u8 {
        AlEventRequestBf11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlEventRequestBf12 {
    #[doc = "No SyncManager4 interrupt"]
    BfVal0 = 0x0,
    #[doc = "SyncManager 4 interrupt pending"]
    BfVal1 = 0x01,
}
impl AlEventRequestBf12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AlEventRequestBf12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AlEventRequestBf12 {
    #[inline(always)]
    fn from(val: u8) -> AlEventRequestBf12 {
        AlEventRequestBf12::from_bits(val)
    }
}
impl From<AlEventRequestBf12> for u8 {
    #[inline(always)]
    fn from(val: AlEventRequestBf12) -> u8 {
        AlEventRequestBf12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlEventRequestBf13 {
    #[doc = "No SyncManager 5 interrupt"]
    BfVal0 = 0x0,
    #[doc = "SyncManager 5 interrupt pending"]
    BfVal1 = 0x01,
}
impl AlEventRequestBf13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AlEventRequestBf13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AlEventRequestBf13 {
    #[inline(always)]
    fn from(val: u8) -> AlEventRequestBf13 {
        AlEventRequestBf13::from_bits(val)
    }
}
impl From<AlEventRequestBf13> for u8 {
    #[inline(always)]
    fn from(val: AlEventRequestBf13) -> u8 {
        AlEventRequestBf13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlEventRequestBf14 {
    #[doc = "No SyncManager 6 interrupt"]
    BfVal0 = 0x0,
    #[doc = "SyncManager6 interrupt pending"]
    BfVal1 = 0x01,
}
impl AlEventRequestBf14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AlEventRequestBf14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AlEventRequestBf14 {
    #[inline(always)]
    fn from(val: u8) -> AlEventRequestBf14 {
        AlEventRequestBf14::from_bits(val)
    }
}
impl From<AlEventRequestBf14> for u8 {
    #[inline(always)]
    fn from(val: AlEventRequestBf14) -> u8 {
        AlEventRequestBf14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlEventRequestBf15 {
    #[doc = "No SyncManager 7 interrupt"]
    BfVal0 = 0x0,
    #[doc = "SyncManager 7 interrupt pending"]
    BfVal1 = 0x01,
}
impl AlEventRequestBf15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AlEventRequestBf15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AlEventRequestBf15 {
    #[inline(always)]
    fn from(val: u8) -> AlEventRequestBf15 {
        AlEventRequestBf15::from_bits(val)
    }
}
impl From<AlEventRequestBf15> for u8 {
    #[inline(always)]
    fn from(val: AlEventRequestBf15) -> u8 {
        AlEventRequestBf15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlEventRequestBf4 {
    #[doc = "No change in any SyncManager"]
    BfVal0 = 0x0,
    #[doc = "At least one SyncManager changed"]
    BfVal1 = 0x01,
}
impl AlEventRequestBf4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AlEventRequestBf4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AlEventRequestBf4 {
    #[inline(always)]
    fn from(val: u8) -> AlEventRequestBf4 {
        AlEventRequestBf4::from_bits(val)
    }
}
impl From<AlEventRequestBf4> for u8 {
    #[inline(always)]
    fn from(val: AlEventRequestBf4) -> u8 {
        AlEventRequestBf4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlEventRequestBf5 {
    #[doc = "No command pending"]
    BfVal0 = 0x0,
    #[doc = "EEPROM command pending"]
    BfVal1 = 0x01,
}
impl AlEventRequestBf5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AlEventRequestBf5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AlEventRequestBf5 {
    #[inline(always)]
    fn from(val: u8) -> AlEventRequestBf5 {
        AlEventRequestBf5::from_bits(val)
    }
}
impl From<AlEventRequestBf5> for u8 {
    #[inline(always)]
    fn from(val: AlEventRequestBf5) -> u8 {
        AlEventRequestBf5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlEventRequestBf6 {
    #[doc = "Has not expired"]
    BfVal0 = 0x0,
    #[doc = "Has expired"]
    BfVal1 = 0x01,
}
impl AlEventRequestBf6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AlEventRequestBf6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AlEventRequestBf6 {
    #[inline(always)]
    fn from(val: u8) -> AlEventRequestBf6 {
        AlEventRequestBf6::from_bits(val)
    }
}
impl From<AlEventRequestBf6> for u8 {
    #[inline(always)]
    fn from(val: AlEventRequestBf6) -> u8 {
        AlEventRequestBf6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlEventRequestBf8 {
    #[doc = "No SyncManager 0 interrupt"]
    BfVal0 = 0x0,
    #[doc = "SyncManager 0 interrupt pending"]
    BfVal1 = 0x01,
}
impl AlEventRequestBf8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AlEventRequestBf8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AlEventRequestBf8 {
    #[inline(always)]
    fn from(val: u8) -> AlEventRequestBf8 {
        AlEventRequestBf8::from_bits(val)
    }
}
impl From<AlEventRequestBf8> for u8 {
    #[inline(always)]
    fn from(val: AlEventRequestBf8) -> u8 {
        AlEventRequestBf8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlEventRequestBf9 {
    #[doc = "No SyncManager 1 interrupt"]
    BfVal0 = 0x0,
    #[doc = "SyncManager 1 interrupt pending"]
    BfVal1 = 0x01,
}
impl AlEventRequestBf9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AlEventRequestBf9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AlEventRequestBf9 {
    #[inline(always)]
    fn from(val: u8) -> AlEventRequestBf9 {
        AlEventRequestBf9::from_bits(val)
    }
}
impl From<AlEventRequestBf9> for u8 {
    #[inline(always)]
    fn from(val: AlEventRequestBf9) -> u8 {
        AlEventRequestBf9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlStatusBf4 {
    #[doc = "Device is in State as requested or Flag cleared by command"]
    BfVal0 = 0x0,
    #[doc = "Device has not entered requested State or changed State as result of a local action"]
    BfVal1 = 0x01,
}
impl AlStatusBf4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AlStatusBf4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AlStatusBf4 {
    #[inline(always)]
    fn from(val: u8) -> AlStatusBf4 {
        AlStatusBf4::from_bits(val)
    }
}
impl From<AlStatusBf4> for u8 {
    #[inline(always)]
    fn from(val: AlStatusBf4) -> u8 {
        AlStatusBf4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlStatusBf5 {
    #[doc = "Device Identification not valid"]
    BfVal0 = 0x0,
    #[doc = "Device Identification loaded"]
    BfVal1 = 0x01,
}
impl AlStatusBf5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AlStatusBf5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AlStatusBf5 {
    #[inline(always)]
    fn from(val: u8) -> AlStatusBf5 {
        AlStatusBf5::from_bits(val)
    }
}
impl From<AlStatusBf5> for u8 {
    #[inline(always)]
    fn from(val: AlStatusBf5) -> u8 {
        AlStatusBf5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlStatusPdiBf4 {
    #[doc = "Device is in State as requested or Flag cleared by command"]
    BfVal0 = 0x0,
    #[doc = "Device has not entered requested State or changed State as result of a local action"]
    BfVal1 = 0x01,
}
impl AlStatusPdiBf4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AlStatusPdiBf4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AlStatusPdiBf4 {
    #[inline(always)]
    fn from(val: u8) -> AlStatusPdiBf4 {
        AlStatusPdiBf4::from_bits(val)
    }
}
impl From<AlStatusPdiBf4> for u8 {
    #[inline(always)]
    fn from(val: AlStatusPdiBf4) -> u8 {
        AlStatusPdiBf4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlStatusPdiBf5 {
    #[doc = "Device Identification not valid"]
    BfVal0 = 0x0,
    #[doc = "Device Identification loaded"]
    BfVal1 = 0x01,
}
impl AlStatusPdiBf5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AlStatusPdiBf5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AlStatusPdiBf5 {
    #[inline(always)]
    fn from(val: u8) -> AlStatusPdiBf5 {
        AlStatusPdiBf5::from_bits(val)
    }
}
impl From<AlStatusPdiBf5> for u8 {
    #[inline(always)]
    fn from(val: AlStatusPdiBf5) -> u8 {
        AlStatusPdiBf5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AsynchronousSynchronousMicrocontrollerBf0 {
    #[doc = "no error"]
    BfVal0 = 0x0,
    #[doc = "error detected"]
    BfVal1 = 0x01,
}
impl AsynchronousSynchronousMicrocontrollerBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AsynchronousSynchronousMicrocontrollerBf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AsynchronousSynchronousMicrocontrollerBf0 {
    #[inline(always)]
    fn from(val: u8) -> AsynchronousSynchronousMicrocontrollerBf0 {
        AsynchronousSynchronousMicrocontrollerBf0::from_bits(val)
    }
}
impl From<AsynchronousSynchronousMicrocontrollerBf0> for u8 {
    #[inline(always)]
    fn from(val: AsynchronousSynchronousMicrocontrollerBf0) -> u8 {
        AsynchronousSynchronousMicrocontrollerBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AsynchronousSynchronousMicrocontrollerBf1 {
    #[doc = "no error"]
    BfVal0 = 0x0,
    #[doc = "error detected"]
    BfVal1 = 0x01,
}
impl AsynchronousSynchronousMicrocontrollerBf1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AsynchronousSynchronousMicrocontrollerBf1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AsynchronousSynchronousMicrocontrollerBf1 {
    #[inline(always)]
    fn from(val: u8) -> AsynchronousSynchronousMicrocontrollerBf1 {
        AsynchronousSynchronousMicrocontrollerBf1::from_bits(val)
    }
}
impl From<AsynchronousSynchronousMicrocontrollerBf1> for u8 {
    #[inline(always)]
    fn from(val: AsynchronousSynchronousMicrocontrollerBf1) -> u8 {
        AsynchronousSynchronousMicrocontrollerBf1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AsynchronousSynchronousMicrocontrollerBf2 {
    #[doc = "no error"]
    BfVal0 = 0x0,
    #[doc = "error detected"]
    BfVal1 = 0x01,
}
impl AsynchronousSynchronousMicrocontrollerBf2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AsynchronousSynchronousMicrocontrollerBf2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AsynchronousSynchronousMicrocontrollerBf2 {
    #[inline(always)]
    fn from(val: u8) -> AsynchronousSynchronousMicrocontrollerBf2 {
        AsynchronousSynchronousMicrocontrollerBf2::from_bits(val)
    }
}
impl From<AsynchronousSynchronousMicrocontrollerBf2> for u8 {
    #[inline(always)]
    fn from(val: AsynchronousSynchronousMicrocontrollerBf2) -> u8 {
        AsynchronousSynchronousMicrocontrollerBf2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AsynchronousSynchronousMicrocontrollerBf3 {
    #[doc = "no error"]
    BfVal0 = 0x0,
    #[doc = "error detected"]
    BfVal1 = 0x01,
}
impl AsynchronousSynchronousMicrocontrollerBf3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AsynchronousSynchronousMicrocontrollerBf3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AsynchronousSynchronousMicrocontrollerBf3 {
    #[inline(always)]
    fn from(val: u8) -> AsynchronousSynchronousMicrocontrollerBf3 {
        AsynchronousSynchronousMicrocontrollerBf3::from_bits(val)
    }
}
impl From<AsynchronousSynchronousMicrocontrollerBf3> for u8 {
    #[inline(always)]
    fn from(val: AsynchronousSynchronousMicrocontrollerBf3) -> u8 {
        AsynchronousSynchronousMicrocontrollerBf3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bf31 {
    #[doc = "Local copy of System Time less than received System Time"]
    BfVal0 = 0x0,
    #[doc = "Local copy of System Time greater than or equal to received System Time"]
    BfVal3 = 0x01,
}
impl Bf31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bf31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bf31 {
    #[inline(always)]
    fn from(val: u8) -> Bf31 {
        Bf31::from_bits(val)
    }
}
impl From<Bf31> for u8 {
    #[inline(always)]
    fn from(val: Bf31) -> u8 {
        Bf31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CyclicUnitControlBf0 {
    #[doc = "ECAT-controlled"]
    BfVal0 = 0x0,
    #[doc = "PDI-controlled"]
    BfVal1 = 0x01,
}
impl CyclicUnitControlBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CyclicUnitControlBf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CyclicUnitControlBf0 {
    #[inline(always)]
    fn from(val: u8) -> CyclicUnitControlBf0 {
        CyclicUnitControlBf0::from_bits(val)
    }
}
impl From<CyclicUnitControlBf0> for u8 {
    #[inline(always)]
    fn from(val: CyclicUnitControlBf0) -> u8 {
        CyclicUnitControlBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CyclicUnitControlBf4 {
    #[doc = "ECAT-controlled"]
    BfVal0 = 0x0,
    #[doc = "PDI-controlled"]
    BfVal1 = 0x01,
}
impl CyclicUnitControlBf4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CyclicUnitControlBf4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CyclicUnitControlBf4 {
    #[inline(always)]
    fn from(val: u8) -> CyclicUnitControlBf4 {
        CyclicUnitControlBf4::from_bits(val)
    }
}
impl From<CyclicUnitControlBf4> for u8 {
    #[inline(always)]
    fn from(val: CyclicUnitControlBf4) -> u8 {
        CyclicUnitControlBf4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CyclicUnitControlBf5 {
    #[doc = "ECAT-controlled"]
    BfVal0 = 0x0,
    #[doc = "PDI-controlled"]
    BfVal1 = 0x01,
}
impl CyclicUnitControlBf5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CyclicUnitControlBf5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CyclicUnitControlBf5 {
    #[inline(always)]
    fn from(val: u8) -> CyclicUnitControlBf5 {
        CyclicUnitControlBf5::from_bits(val)
    }
}
impl From<CyclicUnitControlBf5> for u8 {
    #[inline(always)]
    fn from(val: CyclicUnitControlBf5) -> u8 {
        CyclicUnitControlBf5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CyclicUnitControlPdiBf0 {
    #[doc = "ECAT-controlled"]
    BfVal0 = 0x0,
    #[doc = "PDI-controlled"]
    BfVal1 = 0x01,
}
impl CyclicUnitControlPdiBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CyclicUnitControlPdiBf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CyclicUnitControlPdiBf0 {
    #[inline(always)]
    fn from(val: u8) -> CyclicUnitControlPdiBf0 {
        CyclicUnitControlPdiBf0::from_bits(val)
    }
}
impl From<CyclicUnitControlPdiBf0> for u8 {
    #[inline(always)]
    fn from(val: CyclicUnitControlPdiBf0) -> u8 {
        CyclicUnitControlPdiBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CyclicUnitControlPdiBf4 {
    #[doc = "ECAT-controlled"]
    BfVal0 = 0x0,
    #[doc = "PDI-controlled"]
    BfVal1 = 0x01,
}
impl CyclicUnitControlPdiBf4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CyclicUnitControlPdiBf4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CyclicUnitControlPdiBf4 {
    #[inline(always)]
    fn from(val: u8) -> CyclicUnitControlPdiBf4 {
        CyclicUnitControlPdiBf4::from_bits(val)
    }
}
impl From<CyclicUnitControlPdiBf4> for u8 {
    #[inline(always)]
    fn from(val: CyclicUnitControlPdiBf4) -> u8 {
        CyclicUnitControlPdiBf4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CyclicUnitControlPdiBf5 {
    #[doc = "ECAT-controlled"]
    BfVal0 = 0x0,
    #[doc = "PDI-controlled"]
    BfVal1 = 0x01,
}
impl CyclicUnitControlPdiBf5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CyclicUnitControlPdiBf5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CyclicUnitControlPdiBf5 {
    #[inline(always)]
    fn from(val: u8) -> CyclicUnitControlPdiBf5 {
        CyclicUnitControlPdiBf5::from_bits(val)
    }
}
impl From<CyclicUnitControlPdiBf5> for u8 {
    #[inline(always)]
    fn from(val: CyclicUnitControlPdiBf5) -> u8 {
        CyclicUnitControlPdiBf5::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EcatEventMaskBf0(pub u16);
impl EcatEventMaskBf0 {
    #[doc = "Corresponding ECAT Event Request register bit is not mapped"]
    pub const BF_VAL_0: Self = Self(0x0);
    #[doc = "Corresponding ECAT Event Request register bit is mapped"]
    pub const BF_VAL_1: Self = Self(0x01);
}
impl EcatEventMaskBf0 {
    pub const fn from_bits(val: u16) -> EcatEventMaskBf0 {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for EcatEventMaskBf0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("BF_VAL_0"),
            0x01 => f.write_str("BF_VAL_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EcatEventMaskBf0 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "BF_VAL_0"),
            0x01 => defmt::write!(f, "BF_VAL_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for EcatEventMaskBf0 {
    #[inline(always)]
    fn from(val: u16) -> EcatEventMaskBf0 {
        EcatEventMaskBf0::from_bits(val)
    }
}
impl From<EcatEventMaskBf0> for u16 {
    #[inline(always)]
    fn from(val: EcatEventMaskBf0) -> u16 {
        EcatEventMaskBf0::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EcatEventMaskPdiBf0(pub u16);
impl EcatEventMaskPdiBf0 {
    #[doc = "Corresponding ECAT Event Request register bit is not mapped"]
    pub const BF_VAL_0: Self = Self(0x0);
    #[doc = "Corresponding ECAT Event Request register bit is mapped"]
    pub const BF_VAL_1: Self = Self(0x01);
}
impl EcatEventMaskPdiBf0 {
    pub const fn from_bits(val: u16) -> EcatEventMaskPdiBf0 {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for EcatEventMaskPdiBf0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("BF_VAL_0"),
            0x01 => f.write_str("BF_VAL_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EcatEventMaskPdiBf0 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "BF_VAL_0"),
            0x01 => defmt::write!(f, "BF_VAL_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for EcatEventMaskPdiBf0 {
    #[inline(always)]
    fn from(val: u16) -> EcatEventMaskPdiBf0 {
        EcatEventMaskPdiBf0::from_bits(val)
    }
}
impl From<EcatEventMaskPdiBf0> for u16 {
    #[inline(always)]
    fn from(val: EcatEventMaskPdiBf0) -> u16 {
        EcatEventMaskPdiBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EcatEventRequestBf0 {
    #[doc = "No change on DC Latch Inputs"]
    BfVal0 = 0x0,
    #[doc = "At least one change on DC Latch Inputs"]
    BfVal1 = 0x01,
}
impl EcatEventRequestBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EcatEventRequestBf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EcatEventRequestBf0 {
    #[inline(always)]
    fn from(val: u8) -> EcatEventRequestBf0 {
        EcatEventRequestBf0::from_bits(val)
    }
}
impl From<EcatEventRequestBf0> for u8 {
    #[inline(always)]
    fn from(val: EcatEventRequestBf0) -> u8 {
        EcatEventRequestBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EcatEventRequestBf10 {
    #[doc = "No Sync Channel 6 event"]
    BfVal0 = 0x0,
    #[doc = "Sync Channel 6 event pending"]
    BfVal1 = 0x01,
}
impl EcatEventRequestBf10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EcatEventRequestBf10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EcatEventRequestBf10 {
    #[inline(always)]
    fn from(val: u8) -> EcatEventRequestBf10 {
        EcatEventRequestBf10::from_bits(val)
    }
}
impl From<EcatEventRequestBf10> for u8 {
    #[inline(always)]
    fn from(val: EcatEventRequestBf10) -> u8 {
        EcatEventRequestBf10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EcatEventRequestBf11 {
    #[doc = "No Sync Channel 7 event"]
    BfVal0 = 0x0,
    #[doc = "Sync Channel 7 event pending"]
    BfVal1 = 0x01,
}
impl EcatEventRequestBf11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EcatEventRequestBf11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EcatEventRequestBf11 {
    #[inline(always)]
    fn from(val: u8) -> EcatEventRequestBf11 {
        EcatEventRequestBf11::from_bits(val)
    }
}
impl From<EcatEventRequestBf11> for u8 {
    #[inline(always)]
    fn from(val: EcatEventRequestBf11) -> u8 {
        EcatEventRequestBf11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EcatEventRequestBf2 {
    #[doc = "No change in DL Status"]
    BfVal0 = 0x0,
    #[doc = "DL Status change"]
    BfVal1 = 0x01,
}
impl EcatEventRequestBf2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EcatEventRequestBf2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EcatEventRequestBf2 {
    #[inline(always)]
    fn from(val: u8) -> EcatEventRequestBf2 {
        EcatEventRequestBf2::from_bits(val)
    }
}
impl From<EcatEventRequestBf2> for u8 {
    #[inline(always)]
    fn from(val: EcatEventRequestBf2) -> u8 {
        EcatEventRequestBf2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EcatEventRequestBf3 {
    #[doc = "No change in AL Status"]
    BfVal0 = 0x0,
    #[doc = "AL Status change"]
    BfVal1 = 0x01,
}
impl EcatEventRequestBf3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EcatEventRequestBf3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EcatEventRequestBf3 {
    #[inline(always)]
    fn from(val: u8) -> EcatEventRequestBf3 {
        EcatEventRequestBf3::from_bits(val)
    }
}
impl From<EcatEventRequestBf3> for u8 {
    #[inline(always)]
    fn from(val: EcatEventRequestBf3) -> u8 {
        EcatEventRequestBf3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EcatEventRequestBf4 {
    #[doc = "No Sync Channel 0 event"]
    BfVal0 = 0x0,
    #[doc = "Sync Channel 0 event pending"]
    BfVal1 = 0x01,
}
impl EcatEventRequestBf4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EcatEventRequestBf4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EcatEventRequestBf4 {
    #[inline(always)]
    fn from(val: u8) -> EcatEventRequestBf4 {
        EcatEventRequestBf4::from_bits(val)
    }
}
impl From<EcatEventRequestBf4> for u8 {
    #[inline(always)]
    fn from(val: EcatEventRequestBf4) -> u8 {
        EcatEventRequestBf4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EcatEventRequestBf5 {
    #[doc = "No Sync Channel 1 event"]
    BfVal0 = 0x0,
    #[doc = "Sync Channel 1 event pending"]
    BfVal1 = 0x01,
}
impl EcatEventRequestBf5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EcatEventRequestBf5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EcatEventRequestBf5 {
    #[inline(always)]
    fn from(val: u8) -> EcatEventRequestBf5 {
        EcatEventRequestBf5::from_bits(val)
    }
}
impl From<EcatEventRequestBf5> for u8 {
    #[inline(always)]
    fn from(val: EcatEventRequestBf5) -> u8 {
        EcatEventRequestBf5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EcatEventRequestBf6 {
    #[doc = "No Sync Channel 2 event"]
    BfVal0 = 0x0,
    #[doc = "Sync Channel 2 event pending"]
    BfVal1 = 0x01,
}
impl EcatEventRequestBf6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EcatEventRequestBf6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EcatEventRequestBf6 {
    #[inline(always)]
    fn from(val: u8) -> EcatEventRequestBf6 {
        EcatEventRequestBf6::from_bits(val)
    }
}
impl From<EcatEventRequestBf6> for u8 {
    #[inline(always)]
    fn from(val: EcatEventRequestBf6) -> u8 {
        EcatEventRequestBf6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EcatEventRequestBf7 {
    #[doc = "No Sync Channel 3 event"]
    BfVal0 = 0x0,
    #[doc = "Sync Channel 3 event pending"]
    BfVal1 = 0x01,
}
impl EcatEventRequestBf7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EcatEventRequestBf7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EcatEventRequestBf7 {
    #[inline(always)]
    fn from(val: u8) -> EcatEventRequestBf7 {
        EcatEventRequestBf7::from_bits(val)
    }
}
impl From<EcatEventRequestBf7> for u8 {
    #[inline(always)]
    fn from(val: EcatEventRequestBf7) -> u8 {
        EcatEventRequestBf7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EcatEventRequestBf8 {
    #[doc = "No Sync Channel 4 event"]
    BfVal0 = 0x0,
    #[doc = "Sync Channel 4 event pending"]
    BfVal1 = 0x01,
}
impl EcatEventRequestBf8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EcatEventRequestBf8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EcatEventRequestBf8 {
    #[inline(always)]
    fn from(val: u8) -> EcatEventRequestBf8 {
        EcatEventRequestBf8::from_bits(val)
    }
}
impl From<EcatEventRequestBf8> for u8 {
    #[inline(always)]
    fn from(val: EcatEventRequestBf8) -> u8 {
        EcatEventRequestBf8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EcatEventRequestBf9 {
    #[doc = "No Sync Channel 5 event"]
    BfVal0 = 0x0,
    #[doc = "Sync Channel 5 event pending"]
    BfVal1 = 0x01,
}
impl EcatEventRequestBf9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EcatEventRequestBf9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EcatEventRequestBf9 {
    #[inline(always)]
    fn from(val: u8) -> EcatEventRequestBf9 {
        EcatEventRequestBf9::from_bits(val)
    }
}
impl From<EcatEventRequestBf9> for u8 {
    #[inline(always)]
    fn from(val: EcatEventRequestBf9) -> u8 {
        EcatEventRequestBf9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EepromConfigurationBf0 {
    #[doc = "no"]
    BfVal0 = 0x0,
    #[doc = "yes (PDI has EEPROM control)"]
    BfVal1 = 0x01,
}
impl EepromConfigurationBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EepromConfigurationBf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EepromConfigurationBf0 {
    #[inline(always)]
    fn from(val: u8) -> EepromConfigurationBf0 {
        EepromConfigurationBf0::from_bits(val)
    }
}
impl From<EepromConfigurationBf0> for u8 {
    #[inline(always)]
    fn from(val: EepromConfigurationBf0) -> u8 {
        EepromConfigurationBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EepromConfigurationBf1 {
    #[doc = "Do not change Bit 0x0501\\[0\\]"]
    BfVal0 = 0x0,
    #[doc = "Reset Bit 0x0501\\[0\\] to 0"]
    BfVal1 = 0x01,
}
impl EepromConfigurationBf1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EepromConfigurationBf1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EepromConfigurationBf1 {
    #[inline(always)]
    fn from(val: u8) -> EepromConfigurationBf1 {
        EepromConfigurationBf1::from_bits(val)
    }
}
impl From<EepromConfigurationBf1> for u8 {
    #[inline(always)]
    fn from(val: EepromConfigurationBf1) -> u8 {
        EepromConfigurationBf1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EepromConfigurationPdiBf0 {
    #[doc = "no"]
    BfVal0 = 0x0,
    #[doc = "yes (PDI has EEPROM control)"]
    BfVal1 = 0x01,
}
impl EepromConfigurationPdiBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EepromConfigurationPdiBf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EepromConfigurationPdiBf0 {
    #[inline(always)]
    fn from(val: u8) -> EepromConfigurationPdiBf0 {
        EepromConfigurationPdiBf0::from_bits(val)
    }
}
impl From<EepromConfigurationPdiBf0> for u8 {
    #[inline(always)]
    fn from(val: EepromConfigurationPdiBf0) -> u8 {
        EepromConfigurationPdiBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EepromConfigurationPdiBf1 {
    #[doc = "Do not change Bit 0x0501\\[0\\]"]
    BfVal0 = 0x0,
    #[doc = "Reset Bit 0x0501\\[0\\] to 0"]
    BfVal1 = 0x01,
}
impl EepromConfigurationPdiBf1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EepromConfigurationPdiBf1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EepromConfigurationPdiBf1 {
    #[inline(always)]
    fn from(val: u8) -> EepromConfigurationPdiBf1 {
        EepromConfigurationPdiBf1::from_bits(val)
    }
}
impl From<EepromConfigurationPdiBf1> for u8 {
    #[inline(always)]
    fn from(val: EepromConfigurationPdiBf1) -> u8 {
        EepromConfigurationPdiBf1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EepromControlStatusBf0 {
    #[doc = "Write requests are disabled"]
    BfVal0 = 0x0,
    #[doc = "Write requests are enabled"]
    BfVal1 = 0x01,
}
impl EepromControlStatusBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EepromControlStatusBf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EepromControlStatusBf0 {
    #[inline(always)]
    fn from(val: u8) -> EepromControlStatusBf0 {
        EepromControlStatusBf0::from_bits(val)
    }
}
impl From<EepromControlStatusBf0> for u8 {
    #[inline(always)]
    fn from(val: EepromControlStatusBf0) -> u8 {
        EepromControlStatusBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EepromControlStatusBf11 {
    #[doc = "Checksum ok"]
    BfVal0 = 0x0,
    #[doc = "Checksum error"]
    BfVal1 = 0x01,
}
impl EepromControlStatusBf11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EepromControlStatusBf11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EepromControlStatusBf11 {
    #[inline(always)]
    fn from(val: u8) -> EepromControlStatusBf11 {
        EepromControlStatusBf11::from_bits(val)
    }
}
impl From<EepromControlStatusBf11> for u8 {
    #[inline(always)]
    fn from(val: EepromControlStatusBf11) -> u8 {
        EepromControlStatusBf11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EepromControlStatusBf12 {
    #[doc = "EEPROM loaded, device information ok"]
    BfVal0 = 0x0,
    #[doc = "EEPROM not loaded, device information not available (EEPROM loading in progress or finished with a failure)"]
    BfVal1 = 0x01,
}
impl EepromControlStatusBf12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EepromControlStatusBf12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EepromControlStatusBf12 {
    #[inline(always)]
    fn from(val: u8) -> EepromControlStatusBf12 {
        EepromControlStatusBf12::from_bits(val)
    }
}
impl From<EepromControlStatusBf12> for u8 {
    #[inline(always)]
    fn from(val: EepromControlStatusBf12) -> u8 {
        EepromControlStatusBf12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EepromControlStatusBf13 {
    #[doc = "No error"]
    BfVal0 = 0x0,
    #[doc = "Missing EEPROM acknowledge or invalid command"]
    BfVal1 = 0x01,
}
impl EepromControlStatusBf13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EepromControlStatusBf13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EepromControlStatusBf13 {
    #[inline(always)]
    fn from(val: u8) -> EepromControlStatusBf13 {
        EepromControlStatusBf13::from_bits(val)
    }
}
impl From<EepromControlStatusBf13> for u8 {
    #[inline(always)]
    fn from(val: EepromControlStatusBf13) -> u8 {
        EepromControlStatusBf13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EepromControlStatusBf14 {
    #[doc = "No error"]
    BfVal0 = 0x0,
    #[doc = "Write Command without Write enable"]
    BfVal1 = 0x01,
}
impl EepromControlStatusBf14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EepromControlStatusBf14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EepromControlStatusBf14 {
    #[inline(always)]
    fn from(val: u8) -> EepromControlStatusBf14 {
        EepromControlStatusBf14::from_bits(val)
    }
}
impl From<EepromControlStatusBf14> for u8 {
    #[inline(always)]
    fn from(val: EepromControlStatusBf14) -> u8 {
        EepromControlStatusBf14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EepromControlStatusBf15 {
    #[doc = "EEPROM Interface is idle"]
    BfVal0 = 0x0,
    #[doc = "EEPROM Interface is busy"]
    BfVal1 = 0x01,
}
impl EepromControlStatusBf15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EepromControlStatusBf15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EepromControlStatusBf15 {
    #[inline(always)]
    fn from(val: u8) -> EepromControlStatusBf15 {
        EepromControlStatusBf15::from_bits(val)
    }
}
impl From<EepromControlStatusBf15> for u8 {
    #[inline(always)]
    fn from(val: EepromControlStatusBf15) -> u8 {
        EepromControlStatusBf15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EepromControlStatusBf5 {
    #[doc = "Normal operation (I2C interface used)"]
    BfVal0 = 0x0,
    #[doc = "PDI emulates EEPROM (I2C not used)"]
    BfVal1 = 0x01,
}
impl EepromControlStatusBf5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EepromControlStatusBf5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EepromControlStatusBf5 {
    #[inline(always)]
    fn from(val: u8) -> EepromControlStatusBf5 {
        EepromControlStatusBf5::from_bits(val)
    }
}
impl From<EepromControlStatusBf5> for u8 {
    #[inline(always)]
    fn from(val: EepromControlStatusBf5) -> u8 {
        EepromControlStatusBf5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EepromControlStatusBf6 {
    #[doc = "4 Bytes"]
    BfVal0 = 0x0,
    #[doc = "8 Bytes"]
    BfVal1 = 0x01,
}
impl EepromControlStatusBf6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EepromControlStatusBf6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EepromControlStatusBf6 {
    #[inline(always)]
    fn from(val: u8) -> EepromControlStatusBf6 {
        EepromControlStatusBf6::from_bits(val)
    }
}
impl From<EepromControlStatusBf6> for u8 {
    #[inline(always)]
    fn from(val: EepromControlStatusBf6) -> u8 {
        EepromControlStatusBf6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EepromControlStatusBf7 {
    #[doc = "1 address byte (1Kbit to 16Kbit EEPROMs)"]
    BfVal0 = 0x0,
    #[doc = "2 address bytes (32Kbit to 4 Mbit EEPROMs)"]
    BfVal1 = 0x01,
}
impl EepromControlStatusBf7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EepromControlStatusBf7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EepromControlStatusBf7 {
    #[inline(always)]
    fn from(val: u8) -> EepromControlStatusBf7 {
        EepromControlStatusBf7::from_bits(val)
    }
}
impl From<EepromControlStatusBf7> for u8 {
    #[inline(always)]
    fn from(val: EepromControlStatusBf7) -> u8 {
        EepromControlStatusBf7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EepromControlStatusBf8 {
    #[doc = "No command/EEPROM idle (clear error bits)"]
    BfVal0 = 0x0,
    #[doc = "Read"]
    BfVal1 = 0x01,
    #[doc = "Write"]
    BfVal2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Reload"]
    BfVal3 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl EepromControlStatusBf8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EepromControlStatusBf8 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EepromControlStatusBf8 {
    #[inline(always)]
    fn from(val: u8) -> EepromControlStatusBf8 {
        EepromControlStatusBf8::from_bits(val)
    }
}
impl From<EepromControlStatusBf8> for u8 {
    #[inline(always)]
    fn from(val: EepromControlStatusBf8) -> u8 {
        EepromControlStatusBf8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EepromControlStatusPdiBf0 {
    #[doc = "Write requests are disabled"]
    BfVal0 = 0x0,
    #[doc = "Write requests are enabled"]
    BfVal1 = 0x01,
}
impl EepromControlStatusPdiBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EepromControlStatusPdiBf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EepromControlStatusPdiBf0 {
    #[inline(always)]
    fn from(val: u8) -> EepromControlStatusPdiBf0 {
        EepromControlStatusPdiBf0::from_bits(val)
    }
}
impl From<EepromControlStatusPdiBf0> for u8 {
    #[inline(always)]
    fn from(val: EepromControlStatusPdiBf0) -> u8 {
        EepromControlStatusPdiBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EepromControlStatusPdiBf11 {
    #[doc = "Checksum ok"]
    BfVal0 = 0x0,
    #[doc = "Checksum error"]
    BfVal1 = 0x01,
}
impl EepromControlStatusPdiBf11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EepromControlStatusPdiBf11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EepromControlStatusPdiBf11 {
    #[inline(always)]
    fn from(val: u8) -> EepromControlStatusPdiBf11 {
        EepromControlStatusPdiBf11::from_bits(val)
    }
}
impl From<EepromControlStatusPdiBf11> for u8 {
    #[inline(always)]
    fn from(val: EepromControlStatusPdiBf11) -> u8 {
        EepromControlStatusPdiBf11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EepromControlStatusPdiBf12 {
    #[doc = "EEPROM loaded, device information ok"]
    BfVal0 = 0x0,
    #[doc = "EEPROM not loaded, device information not available (EEPROM loading in progress or finished with a failure)"]
    BfVal1 = 0x01,
}
impl EepromControlStatusPdiBf12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EepromControlStatusPdiBf12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EepromControlStatusPdiBf12 {
    #[inline(always)]
    fn from(val: u8) -> EepromControlStatusPdiBf12 {
        EepromControlStatusPdiBf12::from_bits(val)
    }
}
impl From<EepromControlStatusPdiBf12> for u8 {
    #[inline(always)]
    fn from(val: EepromControlStatusPdiBf12) -> u8 {
        EepromControlStatusPdiBf12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EepromControlStatusPdiBf13 {
    #[doc = "No error"]
    BfVal0 = 0x0,
    #[doc = "Missing EEPROM acknowledge or invalid command"]
    BfVal1 = 0x01,
}
impl EepromControlStatusPdiBf13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EepromControlStatusPdiBf13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EepromControlStatusPdiBf13 {
    #[inline(always)]
    fn from(val: u8) -> EepromControlStatusPdiBf13 {
        EepromControlStatusPdiBf13::from_bits(val)
    }
}
impl From<EepromControlStatusPdiBf13> for u8 {
    #[inline(always)]
    fn from(val: EepromControlStatusPdiBf13) -> u8 {
        EepromControlStatusPdiBf13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EepromControlStatusPdiBf14 {
    #[doc = "No error"]
    BfVal0 = 0x0,
    #[doc = "Write Command without Write enable"]
    BfVal1 = 0x01,
}
impl EepromControlStatusPdiBf14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EepromControlStatusPdiBf14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EepromControlStatusPdiBf14 {
    #[inline(always)]
    fn from(val: u8) -> EepromControlStatusPdiBf14 {
        EepromControlStatusPdiBf14::from_bits(val)
    }
}
impl From<EepromControlStatusPdiBf14> for u8 {
    #[inline(always)]
    fn from(val: EepromControlStatusPdiBf14) -> u8 {
        EepromControlStatusPdiBf14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EepromControlStatusPdiBf15 {
    #[doc = "EEPROM Interface is idle"]
    BfVal0 = 0x0,
    #[doc = "EEPROM Interface is busy"]
    BfVal1 = 0x01,
}
impl EepromControlStatusPdiBf15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EepromControlStatusPdiBf15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EepromControlStatusPdiBf15 {
    #[inline(always)]
    fn from(val: u8) -> EepromControlStatusPdiBf15 {
        EepromControlStatusPdiBf15::from_bits(val)
    }
}
impl From<EepromControlStatusPdiBf15> for u8 {
    #[inline(always)]
    fn from(val: EepromControlStatusPdiBf15) -> u8 {
        EepromControlStatusPdiBf15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EepromControlStatusPdiBf5 {
    #[doc = "Normal operation (I2C interface used)"]
    BfVal0 = 0x0,
    #[doc = "PDI emulates EEPROM (I2C not used)"]
    BfVal1 = 0x01,
}
impl EepromControlStatusPdiBf5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EepromControlStatusPdiBf5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EepromControlStatusPdiBf5 {
    #[inline(always)]
    fn from(val: u8) -> EepromControlStatusPdiBf5 {
        EepromControlStatusPdiBf5::from_bits(val)
    }
}
impl From<EepromControlStatusPdiBf5> for u8 {
    #[inline(always)]
    fn from(val: EepromControlStatusPdiBf5) -> u8 {
        EepromControlStatusPdiBf5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EepromControlStatusPdiBf6 {
    #[doc = "4 Bytes"]
    BfVal0 = 0x0,
    #[doc = "8 Bytes"]
    BfVal1 = 0x01,
}
impl EepromControlStatusPdiBf6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EepromControlStatusPdiBf6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EepromControlStatusPdiBf6 {
    #[inline(always)]
    fn from(val: u8) -> EepromControlStatusPdiBf6 {
        EepromControlStatusPdiBf6::from_bits(val)
    }
}
impl From<EepromControlStatusPdiBf6> for u8 {
    #[inline(always)]
    fn from(val: EepromControlStatusPdiBf6) -> u8 {
        EepromControlStatusPdiBf6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EepromControlStatusPdiBf7 {
    #[doc = "1 address byte (1Kbit to 16Kbit EEPROMs)"]
    BfVal0 = 0x0,
    #[doc = "2 address bytes (32Kbit to 4 Mbit EEPROMs)"]
    BfVal1 = 0x01,
}
impl EepromControlStatusPdiBf7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EepromControlStatusPdiBf7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EepromControlStatusPdiBf7 {
    #[inline(always)]
    fn from(val: u8) -> EepromControlStatusPdiBf7 {
        EepromControlStatusPdiBf7::from_bits(val)
    }
}
impl From<EepromControlStatusPdiBf7> for u8 {
    #[inline(always)]
    fn from(val: EepromControlStatusPdiBf7) -> u8 {
        EepromControlStatusPdiBf7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EepromControlStatusPdiBf8 {
    #[doc = "No command/EEPROM idle (clear error bits)"]
    BfVal0 = 0x0,
    #[doc = "Read"]
    BfVal1 = 0x01,
    #[doc = "Write"]
    BfVal2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Reload"]
    BfVal3 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl EepromControlStatusPdiBf8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EepromControlStatusPdiBf8 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EepromControlStatusPdiBf8 {
    #[inline(always)]
    fn from(val: u8) -> EepromControlStatusPdiBf8 {
        EepromControlStatusPdiBf8::from_bits(val)
    }
}
impl From<EepromControlStatusPdiBf8> for u8 {
    #[inline(always)]
    fn from(val: EepromControlStatusPdiBf8) -> u8 {
        EepromControlStatusPdiBf8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ErrLedOverrideBf4 {
    #[doc = "Override disabled"]
    BfVal0 = 0x0,
    #[doc = "Override enabled"]
    BfVal1 = 0x01,
}
impl ErrLedOverrideBf4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ErrLedOverrideBf4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ErrLedOverrideBf4 {
    #[inline(always)]
    fn from(val: u8) -> ErrLedOverrideBf4 {
        ErrLedOverrideBf4::from_bits(val)
    }
}
impl From<ErrLedOverrideBf4> for u8 {
    #[inline(always)]
    fn from(val: ErrLedOverrideBf4) -> u8 {
        ErrLedOverrideBf4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EscConfigurationBf0 {
    #[doc = "AL status register has to be set by PDI"]
    BfVal0 = 0x0,
    #[doc = "AL status register will be set to value written to AL control register"]
    BfVal1 = 0x01,
}
impl EscConfigurationBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EscConfigurationBf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EscConfigurationBf0 {
    #[inline(always)]
    fn from(val: u8) -> EscConfigurationBf0 {
        EscConfigurationBf0::from_bits(val)
    }
}
impl From<EscConfigurationBf0> for u8 {
    #[inline(always)]
    fn from(val: EscConfigurationBf0) -> u8 {
        EscConfigurationBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EscConfigurationBf1 {
    #[doc = "disabled (if bits \\[7:4\\]=0)"]
    BfVal0 = 0x0,
    #[doc = "enabled at all ports (overrides bits \\[7:4\\])"]
    BfVal1 = 0x01,
}
impl EscConfigurationBf1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EscConfigurationBf1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EscConfigurationBf1 {
    #[inline(always)]
    fn from(val: u8) -> EscConfigurationBf1 {
        EscConfigurationBf1::from_bits(val)
    }
}
impl From<EscConfigurationBf1> for u8 {
    #[inline(always)]
    fn from(val: EscConfigurationBf1) -> u8 {
        EscConfigurationBf1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EscConfigurationBf2 {
    #[doc = "disabled (power saving)"]
    BfVal0 = 0x0,
    #[doc = "enabled"]
    BfVal1 = 0x01,
}
impl EscConfigurationBf2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EscConfigurationBf2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EscConfigurationBf2 {
    #[inline(always)]
    fn from(val: u8) -> EscConfigurationBf2 {
        EscConfigurationBf2::from_bits(val)
    }
}
impl From<EscConfigurationBf2> for u8 {
    #[inline(always)]
    fn from(val: EscConfigurationBf2) -> u8 {
        EscConfigurationBf2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EscConfigurationBf3 {
    #[doc = "disabled (power saving)"]
    BfVal0 = 0x0,
    #[doc = "enabled"]
    BfVal1 = 0x01,
}
impl EscConfigurationBf3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EscConfigurationBf3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EscConfigurationBf3 {
    #[inline(always)]
    fn from(val: u8) -> EscConfigurationBf3 {
        EscConfigurationBf3::from_bits(val)
    }
}
impl From<EscConfigurationBf3> for u8 {
    #[inline(always)]
    fn from(val: EscConfigurationBf3) -> u8 {
        EscConfigurationBf3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EscConfigurationBf4 {
    #[doc = "disabled (if bit 1=0)"]
    BfVal0 = 0x0,
    #[doc = "enabled"]
    BfVal1 = 0x01,
}
impl EscConfigurationBf4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EscConfigurationBf4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EscConfigurationBf4 {
    #[inline(always)]
    fn from(val: u8) -> EscConfigurationBf4 {
        EscConfigurationBf4::from_bits(val)
    }
}
impl From<EscConfigurationBf4> for u8 {
    #[inline(always)]
    fn from(val: EscConfigurationBf4) -> u8 {
        EscConfigurationBf4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EscConfigurationBf5 {
    #[doc = "disabled (if bit 1=0)"]
    BfVal0 = 0x0,
    #[doc = "enabled"]
    BfVal1 = 0x01,
}
impl EscConfigurationBf5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EscConfigurationBf5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EscConfigurationBf5 {
    #[inline(always)]
    fn from(val: u8) -> EscConfigurationBf5 {
        EscConfigurationBf5::from_bits(val)
    }
}
impl From<EscConfigurationBf5> for u8 {
    #[inline(always)]
    fn from(val: EscConfigurationBf5) -> u8 {
        EscConfigurationBf5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EscDlControlBf0 {
    #[doc = "EtherCAT frames are processed, non-EtherCAT frames are forwarded without processing or modification."]
    BfVal0 = 0x0,
    #[doc = "EtherCAT frames are processed, non-EtherCAT frames are destroyed."]
    BfVal1 = 0x01,
}
impl EscDlControlBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EscDlControlBf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EscDlControlBf0 {
    #[inline(always)]
    fn from(val: u8) -> EscDlControlBf0 {
        EscDlControlBf0::from_bits(val)
    }
}
impl From<EscDlControlBf0> for u8 {
    #[inline(always)]
    fn from(val: EscDlControlBf0) -> u8 {
        EscDlControlBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EscDlControlBf1 {
    #[doc = "permanent use"]
    BfVal0 = 0x0,
    #[doc = "use for about 1 second, then revert to previous settings"]
    BfVal1 = 0x01,
}
impl EscDlControlBf1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EscDlControlBf1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EscDlControlBf1 {
    #[inline(always)]
    fn from(val: u8) -> EscDlControlBf1 {
        EscDlControlBf1::from_bits(val)
    }
}
impl From<EscDlControlBf1> for u8 {
    #[inline(always)]
    fn from(val: EscDlControlBf1) -> u8 {
        EscDlControlBf1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EscDlControlBf10 {
    #[doc = "Auto"]
    BfVal0 = 0x0,
    #[doc = "Auto Close"]
    BfVal1 = 0x01,
    #[doc = "Open"]
    BfVal2 = 0x02,
    #[doc = "Closed"]
    BfVal3 = 0x03,
}
impl EscDlControlBf10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EscDlControlBf10 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EscDlControlBf10 {
    #[inline(always)]
    fn from(val: u8) -> EscDlControlBf10 {
        EscDlControlBf10::from_bits(val)
    }
}
impl From<EscDlControlBf10> for u8 {
    #[inline(always)]
    fn from(val: EscDlControlBf10) -> u8 {
        EscDlControlBf10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EscDlControlBf24 {
    #[doc = "Ignore Station Alias"]
    BfVal0 = 0x0,
    #[doc = "Alias can be used for all configured address command types (FPRD, FPWR, ...)"]
    BfVal1 = 0x01,
}
impl EscDlControlBf24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EscDlControlBf24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EscDlControlBf24 {
    #[inline(always)]
    fn from(val: u8) -> EscDlControlBf24 {
        EscDlControlBf24::from_bits(val)
    }
}
impl From<EscDlControlBf24> for u8 {
    #[inline(always)]
    fn from(val: EscDlControlBf24) -> u8 {
        EscDlControlBf24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EscDlControlBf8 {
    #[doc = "Auto"]
    BfVal0 = 0x0,
    #[doc = "Auto Close"]
    BfVal1 = 0x01,
    #[doc = "Open"]
    BfVal2 = 0x02,
    #[doc = "Closed"]
    BfVal3 = 0x03,
}
impl EscDlControlBf8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EscDlControlBf8 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EscDlControlBf8 {
    #[inline(always)]
    fn from(val: u8) -> EscDlControlBf8 {
        EscDlControlBf8::from_bits(val)
    }
}
impl From<EscDlControlBf8> for u8 {
    #[inline(always)]
    fn from(val: EscDlControlBf8) -> u8 {
        EscDlControlBf8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EscDlControlPdiBf0 {
    #[doc = "EtherCAT frames are processed, non-EtherCAT frames are forwarded without processing or modification."]
    BfVal0 = 0x0,
    #[doc = "EtherCAT frames are processed, non-EtherCAT frames are destroyed."]
    BfVal1 = 0x01,
}
impl EscDlControlPdiBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EscDlControlPdiBf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EscDlControlPdiBf0 {
    #[inline(always)]
    fn from(val: u8) -> EscDlControlPdiBf0 {
        EscDlControlPdiBf0::from_bits(val)
    }
}
impl From<EscDlControlPdiBf0> for u8 {
    #[inline(always)]
    fn from(val: EscDlControlPdiBf0) -> u8 {
        EscDlControlPdiBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EscDlControlPdiBf1 {
    #[doc = "permanent use"]
    BfVal0 = 0x0,
    #[doc = "use for about 1 second, then revert to previous settings"]
    BfVal1 = 0x01,
}
impl EscDlControlPdiBf1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EscDlControlPdiBf1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EscDlControlPdiBf1 {
    #[inline(always)]
    fn from(val: u8) -> EscDlControlPdiBf1 {
        EscDlControlPdiBf1::from_bits(val)
    }
}
impl From<EscDlControlPdiBf1> for u8 {
    #[inline(always)]
    fn from(val: EscDlControlPdiBf1) -> u8 {
        EscDlControlPdiBf1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EscDlControlPdiBf10 {
    #[doc = "Auto"]
    BfVal0 = 0x0,
    #[doc = "Auto Close"]
    BfVal1 = 0x01,
    #[doc = "Open"]
    BfVal2 = 0x02,
    #[doc = "Closed"]
    BfVal3 = 0x03,
}
impl EscDlControlPdiBf10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EscDlControlPdiBf10 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EscDlControlPdiBf10 {
    #[inline(always)]
    fn from(val: u8) -> EscDlControlPdiBf10 {
        EscDlControlPdiBf10::from_bits(val)
    }
}
impl From<EscDlControlPdiBf10> for u8 {
    #[inline(always)]
    fn from(val: EscDlControlPdiBf10) -> u8 {
        EscDlControlPdiBf10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EscDlControlPdiBf24 {
    #[doc = "Ignore Station Alias"]
    BfVal0 = 0x0,
    #[doc = "Alias can be used for all configured address command types (FPRD, FPWR, ...)"]
    BfVal1 = 0x01,
}
impl EscDlControlPdiBf24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EscDlControlPdiBf24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EscDlControlPdiBf24 {
    #[inline(always)]
    fn from(val: u8) -> EscDlControlPdiBf24 {
        EscDlControlPdiBf24::from_bits(val)
    }
}
impl From<EscDlControlPdiBf24> for u8 {
    #[inline(always)]
    fn from(val: EscDlControlPdiBf24) -> u8 {
        EscDlControlPdiBf24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EscDlControlPdiBf8 {
    #[doc = "Auto"]
    BfVal0 = 0x0,
    #[doc = "Auto Close"]
    BfVal1 = 0x01,
    #[doc = "Open"]
    BfVal2 = 0x02,
    #[doc = "Closed"]
    BfVal3 = 0x03,
}
impl EscDlControlPdiBf8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EscDlControlPdiBf8 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EscDlControlPdiBf8 {
    #[inline(always)]
    fn from(val: u8) -> EscDlControlPdiBf8 {
        EscDlControlPdiBf8::from_bits(val)
    }
}
impl From<EscDlControlPdiBf8> for u8 {
    #[inline(always)]
    fn from(val: EscDlControlPdiBf8) -> u8 {
        EscDlControlPdiBf8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EscDlStatusBf0 {
    #[doc = "PDI operational/EEPROM loaded correctly:"]
    BfVal0 = 0x0,
    #[doc = "EEPROM loaded correctly, PDI operational (access to Process Data RAM)"]
    BfVal1 = 0x01,
}
impl EscDlStatusBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EscDlStatusBf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EscDlStatusBf0 {
    #[inline(always)]
    fn from(val: u8) -> EscDlStatusBf0 {
        EscDlStatusBf0::from_bits(val)
    }
}
impl From<EscDlStatusBf0> for u8 {
    #[inline(always)]
    fn from(val: EscDlStatusBf0) -> u8 {
        EscDlStatusBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EscDlStatusBf1 {
    #[doc = "Watchdog expired"]
    BfVal0 = 0x0,
    #[doc = "Watchdog reloaded"]
    BfVal1 = 0x01,
}
impl EscDlStatusBf1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EscDlStatusBf1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EscDlStatusBf1 {
    #[inline(always)]
    fn from(val: u8) -> EscDlStatusBf1 {
        EscDlStatusBf1::from_bits(val)
    }
}
impl From<EscDlStatusBf1> for u8 {
    #[inline(always)]
    fn from(val: EscDlStatusBf1) -> u8 {
        EscDlStatusBf1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EscDlStatusBf10 {
    #[doc = "Open"]
    BfVal0 = 0x0,
    #[doc = "Closed"]
    BfVal1 = 0x01,
}
impl EscDlStatusBf10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EscDlStatusBf10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EscDlStatusBf10 {
    #[inline(always)]
    fn from(val: u8) -> EscDlStatusBf10 {
        EscDlStatusBf10::from_bits(val)
    }
}
impl From<EscDlStatusBf10> for u8 {
    #[inline(always)]
    fn from(val: EscDlStatusBf10) -> u8 {
        EscDlStatusBf10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EscDlStatusBf11 {
    #[doc = "No stable communication"]
    BfVal0 = 0x0,
    #[doc = "Communication established"]
    BfVal1 = 0x01,
}
impl EscDlStatusBf11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EscDlStatusBf11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EscDlStatusBf11 {
    #[inline(always)]
    fn from(val: u8) -> EscDlStatusBf11 {
        EscDlStatusBf11::from_bits(val)
    }
}
impl From<EscDlStatusBf11> for u8 {
    #[inline(always)]
    fn from(val: EscDlStatusBf11) -> u8 {
        EscDlStatusBf11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EscDlStatusBf2 {
    #[doc = "Deactivated for all ports"]
    BfVal0 = 0x0,
    #[doc = "Activated for at least one port"]
    BfVal1 = 0x01,
}
impl EscDlStatusBf2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EscDlStatusBf2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EscDlStatusBf2 {
    #[inline(always)]
    fn from(val: u8) -> EscDlStatusBf2 {
        EscDlStatusBf2::from_bits(val)
    }
}
impl From<EscDlStatusBf2> for u8 {
    #[inline(always)]
    fn from(val: EscDlStatusBf2) -> u8 {
        EscDlStatusBf2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EscDlStatusBf4 {
    #[doc = "No link"]
    BfVal0 = 0x0,
    #[doc = "Link detected"]
    BfVal1 = 0x01,
}
impl EscDlStatusBf4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EscDlStatusBf4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EscDlStatusBf4 {
    #[inline(always)]
    fn from(val: u8) -> EscDlStatusBf4 {
        EscDlStatusBf4::from_bits(val)
    }
}
impl From<EscDlStatusBf4> for u8 {
    #[inline(always)]
    fn from(val: EscDlStatusBf4) -> u8 {
        EscDlStatusBf4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EscDlStatusBf5 {
    #[doc = "No link"]
    BfVal0 = 0x0,
    #[doc = "Link detected"]
    BfVal1 = 0x01,
}
impl EscDlStatusBf5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EscDlStatusBf5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EscDlStatusBf5 {
    #[inline(always)]
    fn from(val: u8) -> EscDlStatusBf5 {
        EscDlStatusBf5::from_bits(val)
    }
}
impl From<EscDlStatusBf5> for u8 {
    #[inline(always)]
    fn from(val: EscDlStatusBf5) -> u8 {
        EscDlStatusBf5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EscDlStatusBf8 {
    #[doc = "Open"]
    BfVal0 = 0x0,
    #[doc = "Closed"]
    BfVal1 = 0x01,
}
impl EscDlStatusBf8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EscDlStatusBf8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EscDlStatusBf8 {
    #[inline(always)]
    fn from(val: u8) -> EscDlStatusBf8 {
        EscDlStatusBf8::from_bits(val)
    }
}
impl From<EscDlStatusBf8> for u8 {
    #[inline(always)]
    fn from(val: EscDlStatusBf8) -> u8 {
        EscDlStatusBf8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EscDlStatusBf9 {
    #[doc = "No stable communication"]
    BfVal0 = 0x0,
    #[doc = "Communication established"]
    BfVal1 = 0x01,
}
impl EscDlStatusBf9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EscDlStatusBf9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EscDlStatusBf9 {
    #[inline(always)]
    fn from(val: u8) -> EscDlStatusBf9 {
        EscDlStatusBf9::from_bits(val)
    }
}
impl From<EscDlStatusBf9> for u8 {
    #[inline(always)]
    fn from(val: EscDlStatusBf9) -> u8 {
        EscDlStatusBf9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EscFeaturesSupportedBf0 {
    #[doc = "Bit oriented"]
    BfVal0 = 0x0,
    #[doc = "Byte oriented"]
    BfVal1 = 0x01,
}
impl EscFeaturesSupportedBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EscFeaturesSupportedBf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EscFeaturesSupportedBf0 {
    #[inline(always)]
    fn from(val: u8) -> EscFeaturesSupportedBf0 {
        EscFeaturesSupportedBf0::from_bits(val)
    }
}
impl From<EscFeaturesSupportedBf0> for u8 {
    #[inline(always)]
    fn from(val: EscFeaturesSupportedBf0) -> u8 {
        EscFeaturesSupportedBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EscFeaturesSupportedBf1 {
    #[doc = "allowed"]
    BfVal0 = 0x0,
    #[doc = "not supported"]
    BfVal1 = 0x01,
}
impl EscFeaturesSupportedBf1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EscFeaturesSupportedBf1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EscFeaturesSupportedBf1 {
    #[inline(always)]
    fn from(val: u8) -> EscFeaturesSupportedBf1 {
        EscFeaturesSupportedBf1::from_bits(val)
    }
}
impl From<EscFeaturesSupportedBf1> for u8 {
    #[inline(always)]
    fn from(val: EscFeaturesSupportedBf1) -> u8 {
        EscFeaturesSupportedBf1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EscFeaturesSupportedBf10 {
    #[doc = "Supported"]
    BfVal0 = 0x0,
    #[doc = "Not supported"]
    BfVal1 = 0x01,
}
impl EscFeaturesSupportedBf10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EscFeaturesSupportedBf10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EscFeaturesSupportedBf10 {
    #[inline(always)]
    fn from(val: u8) -> EscFeaturesSupportedBf10 {
        EscFeaturesSupportedBf10::from_bits(val)
    }
}
impl From<EscFeaturesSupportedBf10> for u8 {
    #[inline(always)]
    fn from(val: EscFeaturesSupportedBf10) -> u8 {
        EscFeaturesSupportedBf10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EscFeaturesSupportedBf11 {
    #[doc = "Variable configuration"]
    BfVal0 = 0x0,
    #[doc = "Fixed configuration (refer to documentation of supporting ESCs)"]
    BfVal1 = 0x01,
}
impl EscFeaturesSupportedBf11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EscFeaturesSupportedBf11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EscFeaturesSupportedBf11 {
    #[inline(always)]
    fn from(val: u8) -> EscFeaturesSupportedBf11 {
        EscFeaturesSupportedBf11::from_bits(val)
    }
}
impl From<EscFeaturesSupportedBf11> for u8 {
    #[inline(always)]
    fn from(val: EscFeaturesSupportedBf11) -> u8 {
        EscFeaturesSupportedBf11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EscFeaturesSupportedBf2 {
    #[doc = "Not available"]
    BfVal0 = 0x0,
    #[doc = "Available"]
    BfVal1 = 0x01,
}
impl EscFeaturesSupportedBf2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EscFeaturesSupportedBf2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EscFeaturesSupportedBf2 {
    #[inline(always)]
    fn from(val: u8) -> EscFeaturesSupportedBf2 {
        EscFeaturesSupportedBf2::from_bits(val)
    }
}
impl From<EscFeaturesSupportedBf2> for u8 {
    #[inline(always)]
    fn from(val: EscFeaturesSupportedBf2) -> u8 {
        EscFeaturesSupportedBf2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EscFeaturesSupportedBf3 {
    #[doc = "32 bit"]
    BfVal0 = 0x0,
    #[doc = "64 bit"]
    BfVal1 = 0x01,
}
impl EscFeaturesSupportedBf3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EscFeaturesSupportedBf3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EscFeaturesSupportedBf3 {
    #[inline(always)]
    fn from(val: u8) -> EscFeaturesSupportedBf3 {
        EscFeaturesSupportedBf3::from_bits(val)
    }
}
impl From<EscFeaturesSupportedBf3> for u8 {
    #[inline(always)]
    fn from(val: EscFeaturesSupportedBf3) -> u8 {
        EscFeaturesSupportedBf3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EscFeaturesSupportedBf6 {
    #[doc = "Not available"]
    BfVal0 = 0x0,
    #[doc = "Available"]
    BfVal1 = 0x01,
}
impl EscFeaturesSupportedBf6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EscFeaturesSupportedBf6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EscFeaturesSupportedBf6 {
    #[inline(always)]
    fn from(val: u8) -> EscFeaturesSupportedBf6 {
        EscFeaturesSupportedBf6::from_bits(val)
    }
}
impl From<EscFeaturesSupportedBf6> for u8 {
    #[inline(always)]
    fn from(val: EscFeaturesSupportedBf6) -> u8 {
        EscFeaturesSupportedBf6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EscFeaturesSupportedBf7 {
    #[doc = "Not supported"]
    BfVal0 = 0x0,
    #[doc = "Supported, frames with wrong FCS and additional nibble will be counted separately in Forwarded RX Error Counter"]
    BfVal1 = 0x01,
}
impl EscFeaturesSupportedBf7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EscFeaturesSupportedBf7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EscFeaturesSupportedBf7 {
    #[inline(always)]
    fn from(val: u8) -> EscFeaturesSupportedBf7 {
        EscFeaturesSupportedBf7::from_bits(val)
    }
}
impl From<EscFeaturesSupportedBf7> for u8 {
    #[inline(always)]
    fn from(val: EscFeaturesSupportedBf7) -> u8 {
        EscFeaturesSupportedBf7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EscFeaturesSupportedBf8 {
    #[doc = "Not available"]
    BfVal0 = 0x0,
    #[doc = "Available"]
    BfVal1 = 0x01,
}
impl EscFeaturesSupportedBf8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EscFeaturesSupportedBf8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EscFeaturesSupportedBf8 {
    #[inline(always)]
    fn from(val: u8) -> EscFeaturesSupportedBf8 {
        EscFeaturesSupportedBf8::from_bits(val)
    }
}
impl From<EscFeaturesSupportedBf8> for u8 {
    #[inline(always)]
    fn from(val: EscFeaturesSupportedBf8) -> u8 {
        EscFeaturesSupportedBf8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EscFeaturesSupportedBf9 {
    #[doc = "Supported"]
    BfVal0 = 0x0,
    #[doc = "Not supported"]
    BfVal1 = 0x01,
}
impl EscFeaturesSupportedBf9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EscFeaturesSupportedBf9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EscFeaturesSupportedBf9 {
    #[inline(always)]
    fn from(val: u8) -> EscFeaturesSupportedBf9 {
        EscFeaturesSupportedBf9::from_bits(val)
    }
}
impl From<EscFeaturesSupportedBf9> for u8 {
    #[inline(always)]
    fn from(val: EscFeaturesSupportedBf9) -> u8 {
        EscFeaturesSupportedBf9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EscWriteProtectionBf0 {
    #[doc = "Protection disabled"]
    BfVal0 = 0x0,
    #[doc = "Protection enabled"]
    BfVal1 = 0x01,
}
impl EscWriteProtectionBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EscWriteProtectionBf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EscWriteProtectionBf0 {
    #[inline(always)]
    fn from(val: u8) -> EscWriteProtectionBf0 {
        EscWriteProtectionBf0::from_bits(val)
    }
}
impl From<EscWriteProtectionBf0> for u8 {
    #[inline(always)]
    fn from(val: EscWriteProtectionBf0) -> u8 {
        EscWriteProtectionBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EscWriteProtectionPdiBf0 {
    #[doc = "Protection disabled"]
    BfVal0 = 0x0,
    #[doc = "Protection enabled"]
    BfVal1 = 0x01,
}
impl EscWriteProtectionPdiBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EscWriteProtectionPdiBf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EscWriteProtectionPdiBf0 {
    #[inline(always)]
    fn from(val: u8) -> EscWriteProtectionPdiBf0 {
        EscWriteProtectionPdiBf0::from_bits(val)
    }
}
impl From<EscWriteProtectionPdiBf0> for u8 {
    #[inline(always)]
    fn from(val: EscWriteProtectionPdiBf0) -> u8 {
        EscWriteProtectionPdiBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FmmuActivateBf0 {
    #[doc = "FMMU deactivated"]
    BfVal0 = 0x0,
    #[doc = "FMMU activated. FMMU checks logically addressed blocks to be mapped according to configured mapping"]
    BfVal1 = 0x01,
}
impl FmmuActivateBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FmmuActivateBf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FmmuActivateBf0 {
    #[inline(always)]
    fn from(val: u8) -> FmmuActivateBf0 {
        FmmuActivateBf0::from_bits(val)
    }
}
impl From<FmmuActivateBf0> for u8 {
    #[inline(always)]
    fn from(val: FmmuActivateBf0) -> u8 {
        FmmuActivateBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FmmuActivatePdiBf0 {
    #[doc = "FMMU deactivated"]
    BfVal0 = 0x0,
    #[doc = "FMMU activated. FMMU checks logically addressed blocks to be mapped according to configured mapping"]
    BfVal1 = 0x01,
}
impl FmmuActivatePdiBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FmmuActivatePdiBf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FmmuActivatePdiBf0 {
    #[inline(always)]
    fn from(val: u8) -> FmmuActivatePdiBf0 {
        FmmuActivatePdiBf0::from_bits(val)
    }
}
impl From<FmmuActivatePdiBf0> for u8 {
    #[inline(always)]
    fn from(val: FmmuActivatePdiBf0) -> u8 {
        FmmuActivatePdiBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FmmuTypeBf0 {
    #[doc = "Ignore mapping for read accesses"]
    BfVal0 = 0x0,
    #[doc = "Use mapping for read accesses"]
    BfVal1 = 0x01,
}
impl FmmuTypeBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FmmuTypeBf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FmmuTypeBf0 {
    #[inline(always)]
    fn from(val: u8) -> FmmuTypeBf0 {
        FmmuTypeBf0::from_bits(val)
    }
}
impl From<FmmuTypeBf0> for u8 {
    #[inline(always)]
    fn from(val: FmmuTypeBf0) -> u8 {
        FmmuTypeBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FmmuTypeBf1 {
    #[doc = "Ignore mapping for write accesses"]
    BfVal0 = 0x0,
    #[doc = "Use mapping for write accesses"]
    BfVal1 = 0x01,
}
impl FmmuTypeBf1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FmmuTypeBf1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FmmuTypeBf1 {
    #[inline(always)]
    fn from(val: u8) -> FmmuTypeBf1 {
        FmmuTypeBf1::from_bits(val)
    }
}
impl From<FmmuTypeBf1> for u8 {
    #[inline(always)]
    fn from(val: FmmuTypeBf1) -> u8 {
        FmmuTypeBf1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FmmuTypePdiBf0 {
    #[doc = "Ignore mapping for read accesses"]
    BfVal0 = 0x0,
    #[doc = "Use mapping for read accesses"]
    BfVal1 = 0x01,
}
impl FmmuTypePdiBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FmmuTypePdiBf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FmmuTypePdiBf0 {
    #[inline(always)]
    fn from(val: u8) -> FmmuTypePdiBf0 {
        FmmuTypePdiBf0::from_bits(val)
    }
}
impl From<FmmuTypePdiBf0> for u8 {
    #[inline(always)]
    fn from(val: FmmuTypePdiBf0) -> u8 {
        FmmuTypePdiBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FmmuTypePdiBf1 {
    #[doc = "Ignore mapping for write accesses"]
    BfVal0 = 0x0,
    #[doc = "Use mapping for write accesses"]
    BfVal1 = 0x01,
}
impl FmmuTypePdiBf1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FmmuTypePdiBf1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FmmuTypePdiBf1 {
    #[inline(always)]
    fn from(val: u8) -> FmmuTypePdiBf1 {
        FmmuTypePdiBf1::from_bits(val)
    }
}
impl From<FmmuTypePdiBf1> for u8 {
    #[inline(always)]
    fn from(val: FmmuTypePdiBf1) -> u8 {
        FmmuTypePdiBf1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Latch0ControlBf0 {
    #[doc = "Continuous Latch active"]
    BfVal0 = 0x0,
    #[doc = "Single event (only first event active)"]
    BfVal1 = 0x01,
}
impl Latch0ControlBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Latch0ControlBf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Latch0ControlBf0 {
    #[inline(always)]
    fn from(val: u8) -> Latch0ControlBf0 {
        Latch0ControlBf0::from_bits(val)
    }
}
impl From<Latch0ControlBf0> for u8 {
    #[inline(always)]
    fn from(val: Latch0ControlBf0) -> u8 {
        Latch0ControlBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Latch0ControlBf1 {
    #[doc = "Continuous Latch active"]
    BfVal0 = 0x0,
    #[doc = "Single event (only first event active)"]
    BfVal1 = 0x01,
}
impl Latch0ControlBf1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Latch0ControlBf1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Latch0ControlBf1 {
    #[inline(always)]
    fn from(val: u8) -> Latch0ControlBf1 {
        Latch0ControlBf1::from_bits(val)
    }
}
impl From<Latch0ControlBf1> for u8 {
    #[inline(always)]
    fn from(val: Latch0ControlBf1) -> u8 {
        Latch0ControlBf1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Latch0StatusBf0 {
    #[doc = "Positive edge not detected or continuous mode"]
    BfVal0 = 0x0,
    #[doc = "Positive edge detected in single event mode only."]
    BfVal1 = 0x01,
}
impl Latch0StatusBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Latch0StatusBf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Latch0StatusBf0 {
    #[inline(always)]
    fn from(val: u8) -> Latch0StatusBf0 {
        Latch0StatusBf0::from_bits(val)
    }
}
impl From<Latch0StatusBf0> for u8 {
    #[inline(always)]
    fn from(val: Latch0StatusBf0) -> u8 {
        Latch0StatusBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Latch0StatusBf1 {
    #[doc = "Negative edge not detected or continuous mode"]
    BfVal0 = 0x0,
    #[doc = "Negative edge detected in single event mode only"]
    BfVal1 = 0x01,
}
impl Latch0StatusBf1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Latch0StatusBf1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Latch0StatusBf1 {
    #[inline(always)]
    fn from(val: u8) -> Latch0StatusBf1 {
        Latch0StatusBf1::from_bits(val)
    }
}
impl From<Latch0StatusBf1> for u8 {
    #[inline(always)]
    fn from(val: Latch0StatusBf1) -> u8 {
        Latch0StatusBf1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Latch1ControlBf0 {
    #[doc = "Continuous Latch active"]
    BfVal0 = 0x0,
    #[doc = "Single event (only first event active)"]
    BfVal1 = 0x01,
}
impl Latch1ControlBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Latch1ControlBf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Latch1ControlBf0 {
    #[inline(always)]
    fn from(val: u8) -> Latch1ControlBf0 {
        Latch1ControlBf0::from_bits(val)
    }
}
impl From<Latch1ControlBf0> for u8 {
    #[inline(always)]
    fn from(val: Latch1ControlBf0) -> u8 {
        Latch1ControlBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Latch1ControlBf1 {
    #[doc = "Continuous Latch active"]
    BfVal0 = 0x0,
    #[doc = "Single event (only first event active)"]
    BfVal1 = 0x01,
}
impl Latch1ControlBf1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Latch1ControlBf1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Latch1ControlBf1 {
    #[inline(always)]
    fn from(val: u8) -> Latch1ControlBf1 {
        Latch1ControlBf1::from_bits(val)
    }
}
impl From<Latch1ControlBf1> for u8 {
    #[inline(always)]
    fn from(val: Latch1ControlBf1) -> u8 {
        Latch1ControlBf1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Latch1StatusBf0 {
    #[doc = "Positive edge not detected or continuous mode"]
    BfVal0 = 0x0,
    #[doc = "Positive edge detected in single event mode only."]
    BfVal1 = 0x01,
}
impl Latch1StatusBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Latch1StatusBf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Latch1StatusBf0 {
    #[inline(always)]
    fn from(val: u8) -> Latch1StatusBf0 {
        Latch1StatusBf0::from_bits(val)
    }
}
impl From<Latch1StatusBf0> for u8 {
    #[inline(always)]
    fn from(val: Latch1StatusBf0) -> u8 {
        Latch1StatusBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Latch1StatusBf1 {
    #[doc = "Negative edge not detected or continuous mode"]
    BfVal0 = 0x0,
    #[doc = "Negative edge detected in single event mode only."]
    BfVal1 = 0x01,
}
impl Latch1StatusBf1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Latch1StatusBf1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Latch1StatusBf1 {
    #[inline(always)]
    fn from(val: u8) -> Latch1StatusBf1 {
        Latch1StatusBf1::from_bits(val)
    }
}
impl From<Latch1StatusBf1> for u8 {
    #[inline(always)]
    fn from(val: Latch1StatusBf1) -> u8 {
        Latch1StatusBf1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiiManagementControlOrStatusBf0 {
    #[doc = "Write disabled"]
    BfVal0 = 0x0,
    #[doc = "Write enabled"]
    BfVal1 = 0x01,
}
impl MiiManagementControlOrStatusBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiiManagementControlOrStatusBf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiiManagementControlOrStatusBf0 {
    #[inline(always)]
    fn from(val: u8) -> MiiManagementControlOrStatusBf0 {
        MiiManagementControlOrStatusBf0::from_bits(val)
    }
}
impl From<MiiManagementControlOrStatusBf0> for u8 {
    #[inline(always)]
    fn from(val: MiiManagementControlOrStatusBf0) -> u8 {
        MiiManagementControlOrStatusBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiiManagementControlOrStatusBf1 {
    #[doc = "Only ECAT control"]
    BfVal0 = 0x0,
    #[doc = "PDI control possible"]
    BfVal1 = 0x01,
}
impl MiiManagementControlOrStatusBf1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiiManagementControlOrStatusBf1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiiManagementControlOrStatusBf1 {
    #[inline(always)]
    fn from(val: u8) -> MiiManagementControlOrStatusBf1 {
        MiiManagementControlOrStatusBf1::from_bits(val)
    }
}
impl From<MiiManagementControlOrStatusBf1> for u8 {
    #[inline(always)]
    fn from(val: MiiManagementControlOrStatusBf1) -> u8 {
        MiiManagementControlOrStatusBf1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiiManagementControlOrStatusBf13 {
    #[doc = "No read error"]
    BfVal0 = 0x0,
    #[doc = "Read error occurred (PHY or register not available)"]
    BfVal1 = 0x01,
}
impl MiiManagementControlOrStatusBf13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiiManagementControlOrStatusBf13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiiManagementControlOrStatusBf13 {
    #[inline(always)]
    fn from(val: u8) -> MiiManagementControlOrStatusBf13 {
        MiiManagementControlOrStatusBf13::from_bits(val)
    }
}
impl From<MiiManagementControlOrStatusBf13> for u8 {
    #[inline(always)]
    fn from(val: MiiManagementControlOrStatusBf13) -> u8 {
        MiiManagementControlOrStatusBf13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiiManagementControlOrStatusBf14 {
    #[doc = "Last Command was successful"]
    BfVal0 = 0x0,
    #[doc = "Invalid command or write command without Write Enable"]
    BfVal1 = 0x01,
}
impl MiiManagementControlOrStatusBf14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiiManagementControlOrStatusBf14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiiManagementControlOrStatusBf14 {
    #[inline(always)]
    fn from(val: u8) -> MiiManagementControlOrStatusBf14 {
        MiiManagementControlOrStatusBf14::from_bits(val)
    }
}
impl From<MiiManagementControlOrStatusBf14> for u8 {
    #[inline(always)]
    fn from(val: MiiManagementControlOrStatusBf14) -> u8 {
        MiiManagementControlOrStatusBf14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiiManagementControlOrStatusBf15 {
    #[doc = "MII Management Interface is idle"]
    BfVal0 = 0x0,
    #[doc = "MII Management Interface is busy"]
    BfVal1 = 0x01,
}
impl MiiManagementControlOrStatusBf15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiiManagementControlOrStatusBf15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiiManagementControlOrStatusBf15 {
    #[inline(always)]
    fn from(val: u8) -> MiiManagementControlOrStatusBf15 {
        MiiManagementControlOrStatusBf15::from_bits(val)
    }
}
impl From<MiiManagementControlOrStatusBf15> for u8 {
    #[inline(always)]
    fn from(val: MiiManagementControlOrStatusBf15) -> u8 {
        MiiManagementControlOrStatusBf15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiiManagementControlOrStatusBf2 {
    #[doc = "Disabled for all ports"]
    BfVal0 = 0x0,
    #[doc = "Enabled for at least one MII port, refer to PHY Port Status (0x0518 ff.) for details"]
    BfVal1 = 0x01,
}
impl MiiManagementControlOrStatusBf2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiiManagementControlOrStatusBf2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiiManagementControlOrStatusBf2 {
    #[inline(always)]
    fn from(val: u8) -> MiiManagementControlOrStatusBf2 {
        MiiManagementControlOrStatusBf2::from_bits(val)
    }
}
impl From<MiiManagementControlOrStatusBf2> for u8 {
    #[inline(always)]
    fn from(val: MiiManagementControlOrStatusBf2) -> u8 {
        MiiManagementControlOrStatusBf2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiiManagementControlOrStatusBf8 {
    #[doc = "No command/MI idle (clear error bits)"]
    BfVal0 = 0x0,
    #[doc = "Read"]
    BfVal1 = 0x01,
    #[doc = "Write"]
    BfVal2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiiManagementControlOrStatusBf8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiiManagementControlOrStatusBf8 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiiManagementControlOrStatusBf8 {
    #[inline(always)]
    fn from(val: u8) -> MiiManagementControlOrStatusBf8 {
        MiiManagementControlOrStatusBf8::from_bits(val)
    }
}
impl From<MiiManagementControlOrStatusBf8> for u8 {
    #[inline(always)]
    fn from(val: MiiManagementControlOrStatusBf8) -> u8 {
        MiiManagementControlOrStatusBf8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiiManagementControlOrStatusPdiBf0 {
    #[doc = "Write disabled"]
    BfVal0 = 0x0,
    #[doc = "Write enabled"]
    BfVal1 = 0x01,
}
impl MiiManagementControlOrStatusPdiBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiiManagementControlOrStatusPdiBf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiiManagementControlOrStatusPdiBf0 {
    #[inline(always)]
    fn from(val: u8) -> MiiManagementControlOrStatusPdiBf0 {
        MiiManagementControlOrStatusPdiBf0::from_bits(val)
    }
}
impl From<MiiManagementControlOrStatusPdiBf0> for u8 {
    #[inline(always)]
    fn from(val: MiiManagementControlOrStatusPdiBf0) -> u8 {
        MiiManagementControlOrStatusPdiBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiiManagementControlOrStatusPdiBf1 {
    #[doc = "Only ECAT control"]
    BfVal0 = 0x0,
    #[doc = "PDI control possible"]
    BfVal1 = 0x01,
}
impl MiiManagementControlOrStatusPdiBf1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiiManagementControlOrStatusPdiBf1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiiManagementControlOrStatusPdiBf1 {
    #[inline(always)]
    fn from(val: u8) -> MiiManagementControlOrStatusPdiBf1 {
        MiiManagementControlOrStatusPdiBf1::from_bits(val)
    }
}
impl From<MiiManagementControlOrStatusPdiBf1> for u8 {
    #[inline(always)]
    fn from(val: MiiManagementControlOrStatusPdiBf1) -> u8 {
        MiiManagementControlOrStatusPdiBf1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiiManagementControlOrStatusPdiBf13 {
    #[doc = "No read error"]
    BfVal0 = 0x0,
    #[doc = "Read error occurred (PHY or register not available)"]
    BfVal1 = 0x01,
}
impl MiiManagementControlOrStatusPdiBf13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiiManagementControlOrStatusPdiBf13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiiManagementControlOrStatusPdiBf13 {
    #[inline(always)]
    fn from(val: u8) -> MiiManagementControlOrStatusPdiBf13 {
        MiiManagementControlOrStatusPdiBf13::from_bits(val)
    }
}
impl From<MiiManagementControlOrStatusPdiBf13> for u8 {
    #[inline(always)]
    fn from(val: MiiManagementControlOrStatusPdiBf13) -> u8 {
        MiiManagementControlOrStatusPdiBf13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiiManagementControlOrStatusPdiBf14 {
    #[doc = "Last Command was successful"]
    BfVal0 = 0x0,
    #[doc = "Invalid command or write command without Write Enable"]
    BfVal1 = 0x01,
}
impl MiiManagementControlOrStatusPdiBf14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiiManagementControlOrStatusPdiBf14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiiManagementControlOrStatusPdiBf14 {
    #[inline(always)]
    fn from(val: u8) -> MiiManagementControlOrStatusPdiBf14 {
        MiiManagementControlOrStatusPdiBf14::from_bits(val)
    }
}
impl From<MiiManagementControlOrStatusPdiBf14> for u8 {
    #[inline(always)]
    fn from(val: MiiManagementControlOrStatusPdiBf14) -> u8 {
        MiiManagementControlOrStatusPdiBf14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiiManagementControlOrStatusPdiBf15 {
    #[doc = "MII Management Interface is idle"]
    BfVal0 = 0x0,
    #[doc = "MII Management Interface is busy"]
    BfVal1 = 0x01,
}
impl MiiManagementControlOrStatusPdiBf15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiiManagementControlOrStatusPdiBf15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiiManagementControlOrStatusPdiBf15 {
    #[inline(always)]
    fn from(val: u8) -> MiiManagementControlOrStatusPdiBf15 {
        MiiManagementControlOrStatusPdiBf15::from_bits(val)
    }
}
impl From<MiiManagementControlOrStatusPdiBf15> for u8 {
    #[inline(always)]
    fn from(val: MiiManagementControlOrStatusPdiBf15) -> u8 {
        MiiManagementControlOrStatusPdiBf15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiiManagementControlOrStatusPdiBf2 {
    #[doc = "Disabled for all ports"]
    BfVal0 = 0x0,
    #[doc = "Enabled for at least one MII port, refer to PHY Port Status (0x0518 ff.) for details"]
    BfVal1 = 0x01,
}
impl MiiManagementControlOrStatusPdiBf2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiiManagementControlOrStatusPdiBf2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiiManagementControlOrStatusPdiBf2 {
    #[inline(always)]
    fn from(val: u8) -> MiiManagementControlOrStatusPdiBf2 {
        MiiManagementControlOrStatusPdiBf2::from_bits(val)
    }
}
impl From<MiiManagementControlOrStatusPdiBf2> for u8 {
    #[inline(always)]
    fn from(val: MiiManagementControlOrStatusPdiBf2) -> u8 {
        MiiManagementControlOrStatusPdiBf2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiiManagementControlOrStatusPdiBf8 {
    #[doc = "No command/MI idle (clear error bits)"]
    BfVal0 = 0x0,
    #[doc = "Read"]
    BfVal1 = 0x01,
    #[doc = "Write"]
    BfVal2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiiManagementControlOrStatusPdiBf8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiiManagementControlOrStatusPdiBf8 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiiManagementControlOrStatusPdiBf8 {
    #[inline(always)]
    fn from(val: u8) -> MiiManagementControlOrStatusPdiBf8 {
        MiiManagementControlOrStatusPdiBf8::from_bits(val)
    }
}
impl From<MiiManagementControlOrStatusPdiBf8> for u8 {
    #[inline(always)]
    fn from(val: MiiManagementControlOrStatusPdiBf8) -> u8 {
        MiiManagementControlOrStatusPdiBf8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiiManagementEcatAccessStateBf0 {
    #[doc = "ECAT enables PDI takeover of MII management interface"]
    BfVal0 = 0x0,
    #[doc = "ECAT claims exclusive access to MII management interface"]
    BfVal1 = 0x01,
}
impl MiiManagementEcatAccessStateBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiiManagementEcatAccessStateBf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiiManagementEcatAccessStateBf0 {
    #[inline(always)]
    fn from(val: u8) -> MiiManagementEcatAccessStateBf0 {
        MiiManagementEcatAccessStateBf0::from_bits(val)
    }
}
impl From<MiiManagementEcatAccessStateBf0> for u8 {
    #[inline(always)]
    fn from(val: MiiManagementEcatAccessStateBf0) -> u8 {
        MiiManagementEcatAccessStateBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiiManagementEcatAccessStatePdiBf0 {
    #[doc = "ECAT enables PDI takeover of MII management interface"]
    BfVal0 = 0x0,
    #[doc = "ECAT claims exclusive access to MII management interface"]
    BfVal1 = 0x01,
}
impl MiiManagementEcatAccessStatePdiBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiiManagementEcatAccessStatePdiBf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiiManagementEcatAccessStatePdiBf0 {
    #[inline(always)]
    fn from(val: u8) -> MiiManagementEcatAccessStatePdiBf0 {
        MiiManagementEcatAccessStatePdiBf0::from_bits(val)
    }
}
impl From<MiiManagementEcatAccessStatePdiBf0> for u8 {
    #[inline(always)]
    fn from(val: MiiManagementEcatAccessStatePdiBf0) -> u8 {
        MiiManagementEcatAccessStatePdiBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiiManagementPdiAccessStateBf0 {
    #[doc = "ECAT has access to MII management"]
    BfVal0 = 0x0,
    #[doc = "PDI has access to MII management"]
    BfVal1 = 0x01,
}
impl MiiManagementPdiAccessStateBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiiManagementPdiAccessStateBf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiiManagementPdiAccessStateBf0 {
    #[inline(always)]
    fn from(val: u8) -> MiiManagementPdiAccessStateBf0 {
        MiiManagementPdiAccessStateBf0::from_bits(val)
    }
}
impl From<MiiManagementPdiAccessStateBf0> for u8 {
    #[inline(always)]
    fn from(val: MiiManagementPdiAccessStateBf0) -> u8 {
        MiiManagementPdiAccessStateBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiiManagementPdiAccessStateBf1 {
    #[doc = "Do not change Bit 0x0517\\[0\\]"]
    BfVal0 = 0x0,
    #[doc = "Reset Bit 0x0517\\[0\\] to 0"]
    BfVal1 = 0x01,
}
impl MiiManagementPdiAccessStateBf1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiiManagementPdiAccessStateBf1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiiManagementPdiAccessStateBf1 {
    #[inline(always)]
    fn from(val: u8) -> MiiManagementPdiAccessStateBf1 {
        MiiManagementPdiAccessStateBf1::from_bits(val)
    }
}
impl From<MiiManagementPdiAccessStateBf1> for u8 {
    #[inline(always)]
    fn from(val: MiiManagementPdiAccessStateBf1) -> u8 {
        MiiManagementPdiAccessStateBf1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiiManagementPdiAccessStatePdiBf0 {
    #[doc = "ECAT has access to MII management"]
    BfVal0 = 0x0,
    #[doc = "PDI has access to MII management"]
    BfVal1 = 0x01,
}
impl MiiManagementPdiAccessStatePdiBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiiManagementPdiAccessStatePdiBf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiiManagementPdiAccessStatePdiBf0 {
    #[inline(always)]
    fn from(val: u8) -> MiiManagementPdiAccessStatePdiBf0 {
        MiiManagementPdiAccessStatePdiBf0::from_bits(val)
    }
}
impl From<MiiManagementPdiAccessStatePdiBf0> for u8 {
    #[inline(always)]
    fn from(val: MiiManagementPdiAccessStatePdiBf0) -> u8 {
        MiiManagementPdiAccessStatePdiBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiiManagementPdiAccessStatePdiBf1 {
    #[doc = "Do not change Bit 0x0517\\[0\\]"]
    BfVal0 = 0x0,
    #[doc = "Reset Bit 0x0517\\[0\\] to 0"]
    BfVal1 = 0x01,
}
impl MiiManagementPdiAccessStatePdiBf1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiiManagementPdiAccessStatePdiBf1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiiManagementPdiAccessStatePdiBf1 {
    #[inline(always)]
    fn from(val: u8) -> MiiManagementPdiAccessStatePdiBf1 {
        MiiManagementPdiAccessStatePdiBf1::from_bits(val)
    }
}
impl From<MiiManagementPdiAccessStatePdiBf1> for u8 {
    #[inline(always)]
    fn from(val: MiiManagementPdiAccessStatePdiBf1) -> u8 {
        MiiManagementPdiAccessStatePdiBf1::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct PdiAlEventMaskBf0(pub u32);
impl PdiAlEventMaskBf0 {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    pub const BF_VAL_0: Self = Self(0x0);
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    pub const BF_VAL_1: Self = Self(0x01);
}
impl PdiAlEventMaskBf0 {
    pub const fn from_bits(val: u32) -> PdiAlEventMaskBf0 {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for PdiAlEventMaskBf0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("BF_VAL_0"),
            0x01 => f.write_str("BF_VAL_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PdiAlEventMaskBf0 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "BF_VAL_0"),
            0x01 => defmt::write!(f, "BF_VAL_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for PdiAlEventMaskBf0 {
    #[inline(always)]
    fn from(val: u32) -> PdiAlEventMaskBf0 {
        PdiAlEventMaskBf0::from_bits(val)
    }
}
impl From<PdiAlEventMaskBf0> for u32 {
    #[inline(always)]
    fn from(val: PdiAlEventMaskBf0) -> u32 {
        PdiAlEventMaskBf0::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct PdiAlEventMaskPdiBf0(pub u32);
impl PdiAlEventMaskPdiBf0 {
    #[doc = "Corresponding AL Event Request register bit is not mapped"]
    pub const BF_VAL_0: Self = Self(0x0);
    #[doc = "Corresponding AL Event Request register bit is mapped"]
    pub const BF_VAL_1: Self = Self(0x01);
}
impl PdiAlEventMaskPdiBf0 {
    pub const fn from_bits(val: u32) -> PdiAlEventMaskPdiBf0 {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for PdiAlEventMaskPdiBf0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("BF_VAL_0"),
            0x01 => f.write_str("BF_VAL_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PdiAlEventMaskPdiBf0 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "BF_VAL_0"),
            0x01 => defmt::write!(f, "BF_VAL_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for PdiAlEventMaskPdiBf0 {
    #[inline(always)]
    fn from(val: u32) -> PdiAlEventMaskPdiBf0 {
        PdiAlEventMaskPdiBf0::from_bits(val)
    }
}
impl From<PdiAlEventMaskPdiBf0> for u32 {
    #[inline(always)]
    fn from(val: PdiAlEventMaskPdiBf0) -> u32 {
        PdiAlEventMaskPdiBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdiControlBf0 {
    #[doc = "Interface deactivated (no PDI)"]
    BfVal0 = 0x0,
    #[doc = "4 Digital Input"]
    BfVal1 = 0x01,
    #[doc = "4 Digital Output"]
    BfVal2 = 0x02,
    #[doc = "2 Digital Input and 2 Digital Output"]
    BfVal3 = 0x03,
    #[doc = "Digital I/O"]
    BfVal4 = 0x04,
    #[doc = "SPI Slave"]
    BfVal5 = 0x05,
    #[doc = "Oversampling I/O"]
    BfVal6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "16 Bit asynchronous Microcontroller interface"]
    BfVal8 = 0x08,
    #[doc = "8 Bit asynchronous Microcontroller interface"]
    BfVal9 = 0x09,
    #[doc = "16 Bit synchronous Microcontroller interface"]
    BfVal10 = 0x0a,
    #[doc = "8 Bit synchronous Microcontroller interface"]
    BfVal11 = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "32 Digital Input and 0 Digital Output"]
    BfVal12 = 0x10,
    #[doc = "24 Digital Input and 8 Digital Output"]
    BfVal13 = 0x11,
    #[doc = "16 Digital Input and 16 Digital Output"]
    BfVal14 = 0x12,
    #[doc = "8 Digital Input and 24 Digital Output"]
    BfVal15 = 0x13,
    #[doc = "0 Digital Input and 32 Digital Output"]
    BfVal16 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
    _RESERVED_40 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
    #[doc = "On-chip bus."]
    BfVal17 = 0x80,
    _RESERVED_81 = 0x81,
    _RESERVED_82 = 0x82,
    _RESERVED_83 = 0x83,
    _RESERVED_84 = 0x84,
    _RESERVED_85 = 0x85,
    _RESERVED_86 = 0x86,
    _RESERVED_87 = 0x87,
    _RESERVED_88 = 0x88,
    _RESERVED_89 = 0x89,
    _RESERVED_8a = 0x8a,
    _RESERVED_8b = 0x8b,
    _RESERVED_8c = 0x8c,
    _RESERVED_8d = 0x8d,
    _RESERVED_8e = 0x8e,
    _RESERVED_8f = 0x8f,
    _RESERVED_90 = 0x90,
    _RESERVED_91 = 0x91,
    _RESERVED_92 = 0x92,
    _RESERVED_93 = 0x93,
    _RESERVED_94 = 0x94,
    _RESERVED_95 = 0x95,
    _RESERVED_96 = 0x96,
    _RESERVED_97 = 0x97,
    _RESERVED_98 = 0x98,
    _RESERVED_99 = 0x99,
    _RESERVED_9a = 0x9a,
    _RESERVED_9b = 0x9b,
    _RESERVED_9c = 0x9c,
    _RESERVED_9d = 0x9d,
    _RESERVED_9e = 0x9e,
    _RESERVED_9f = 0x9f,
    _RESERVED_a0 = 0xa0,
    _RESERVED_a1 = 0xa1,
    _RESERVED_a2 = 0xa2,
    _RESERVED_a3 = 0xa3,
    _RESERVED_a4 = 0xa4,
    _RESERVED_a5 = 0xa5,
    _RESERVED_a6 = 0xa6,
    _RESERVED_a7 = 0xa7,
    _RESERVED_a8 = 0xa8,
    _RESERVED_a9 = 0xa9,
    _RESERVED_aa = 0xaa,
    _RESERVED_ab = 0xab,
    _RESERVED_ac = 0xac,
    _RESERVED_ad = 0xad,
    _RESERVED_ae = 0xae,
    _RESERVED_af = 0xaf,
    _RESERVED_b0 = 0xb0,
    _RESERVED_b1 = 0xb1,
    _RESERVED_b2 = 0xb2,
    _RESERVED_b3 = 0xb3,
    _RESERVED_b4 = 0xb4,
    _RESERVED_b5 = 0xb5,
    _RESERVED_b6 = 0xb6,
    _RESERVED_b7 = 0xb7,
    _RESERVED_b8 = 0xb8,
    _RESERVED_b9 = 0xb9,
    _RESERVED_ba = 0xba,
    _RESERVED_bb = 0xbb,
    _RESERVED_bc = 0xbc,
    _RESERVED_bd = 0xbd,
    _RESERVED_be = 0xbe,
    _RESERVED_bf = 0xbf,
    _RESERVED_c0 = 0xc0,
    _RESERVED_c1 = 0xc1,
    _RESERVED_c2 = 0xc2,
    _RESERVED_c3 = 0xc3,
    _RESERVED_c4 = 0xc4,
    _RESERVED_c5 = 0xc5,
    _RESERVED_c6 = 0xc6,
    _RESERVED_c7 = 0xc7,
    _RESERVED_c8 = 0xc8,
    _RESERVED_c9 = 0xc9,
    _RESERVED_ca = 0xca,
    _RESERVED_cb = 0xcb,
    _RESERVED_cc = 0xcc,
    _RESERVED_cd = 0xcd,
    _RESERVED_ce = 0xce,
    _RESERVED_cf = 0xcf,
    _RESERVED_d0 = 0xd0,
    _RESERVED_d1 = 0xd1,
    _RESERVED_d2 = 0xd2,
    _RESERVED_d3 = 0xd3,
    _RESERVED_d4 = 0xd4,
    _RESERVED_d5 = 0xd5,
    _RESERVED_d6 = 0xd6,
    _RESERVED_d7 = 0xd7,
    _RESERVED_d8 = 0xd8,
    _RESERVED_d9 = 0xd9,
    _RESERVED_da = 0xda,
    _RESERVED_db = 0xdb,
    _RESERVED_dc = 0xdc,
    _RESERVED_dd = 0xdd,
    _RESERVED_de = 0xde,
    _RESERVED_df = 0xdf,
    _RESERVED_e0 = 0xe0,
    _RESERVED_e1 = 0xe1,
    _RESERVED_e2 = 0xe2,
    _RESERVED_e3 = 0xe3,
    _RESERVED_e4 = 0xe4,
    _RESERVED_e5 = 0xe5,
    _RESERVED_e6 = 0xe6,
    _RESERVED_e7 = 0xe7,
    _RESERVED_e8 = 0xe8,
    _RESERVED_e9 = 0xe9,
    _RESERVED_ea = 0xea,
    _RESERVED_eb = 0xeb,
    _RESERVED_ec = 0xec,
    _RESERVED_ed = 0xed,
    _RESERVED_ee = 0xee,
    _RESERVED_ef = 0xef,
    _RESERVED_f0 = 0xf0,
    _RESERVED_f1 = 0xf1,
    _RESERVED_f2 = 0xf2,
    _RESERVED_f3 = 0xf3,
    _RESERVED_f4 = 0xf4,
    _RESERVED_f5 = 0xf5,
    _RESERVED_f6 = 0xf6,
    _RESERVED_f7 = 0xf7,
    _RESERVED_f8 = 0xf8,
    _RESERVED_f9 = 0xf9,
    _RESERVED_fa = 0xfa,
    _RESERVED_fb = 0xfb,
    _RESERVED_fc = 0xfc,
    _RESERVED_fd = 0xfd,
    _RESERVED_fe = 0xfe,
    _RESERVED_ff = 0xff,
}
impl PdiControlBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PdiControlBf0 {
        unsafe { core::mem::transmute(val & 0xff) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PdiControlBf0 {
    #[inline(always)]
    fn from(val: u8) -> PdiControlBf0 {
        PdiControlBf0::from_bits(val)
    }
}
impl From<PdiControlBf0> for u8 {
    #[inline(always)]
    fn from(val: PdiControlBf0) -> u8 {
        PdiControlBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdiInformationBf0 {
    #[doc = "Disabled"]
    BfVal0 = 0x0,
    #[doc = "Enabled"]
    BfVal1 = 0x01,
}
impl PdiInformationBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PdiInformationBf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PdiInformationBf0 {
    #[inline(always)]
    fn from(val: u8) -> PdiInformationBf0 {
        PdiInformationBf0::from_bits(val)
    }
}
impl From<PdiInformationBf0> for u8 {
    #[inline(always)]
    fn from(val: PdiInformationBf0) -> u8 {
        PdiInformationBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdiInformationBf1 {
    #[doc = "not loaded"]
    BfVal0 = 0x0,
    #[doc = "loaded"]
    BfVal1 = 0x01,
}
impl PdiInformationBf1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PdiInformationBf1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PdiInformationBf1 {
    #[inline(always)]
    fn from(val: u8) -> PdiInformationBf1 {
        PdiInformationBf1::from_bits(val)
    }
}
impl From<PdiInformationBf1> for u8 {
    #[inline(always)]
    fn from(val: PdiInformationBf1) -> u8 {
        PdiInformationBf1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdiInformationBf2 {
    #[doc = "PDI not active"]
    BfVal0 = 0x0,
    #[doc = "PDI active"]
    BfVal1 = 0x01,
}
impl PdiInformationBf2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PdiInformationBf2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PdiInformationBf2 {
    #[inline(always)]
    fn from(val: u8) -> PdiInformationBf2 {
        PdiInformationBf2::from_bits(val)
    }
}
impl From<PdiInformationBf2> for u8 {
    #[inline(always)]
    fn from(val: PdiInformationBf2) -> u8 {
        PdiInformationBf2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdiInformationBf3 {
    #[doc = "PDI configuration ok"]
    BfVal0 = 0x0,
    #[doc = "PDI configuration invalid"]
    BfVal1 = 0x01,
}
impl PdiInformationBf3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PdiInformationBf3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PdiInformationBf3 {
    #[inline(always)]
    fn from(val: u8) -> PdiInformationBf3 {
        PdiInformationBf3::from_bits(val)
    }
}
impl From<PdiInformationBf3> for u8 {
    #[inline(always)]
    fn from(val: PdiInformationBf3) -> u8 {
        PdiInformationBf3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdiOnChipBusConfigurationBf0 {
    #[doc = "asynchronous"]
    BfVal0 = 0x0,
    #[doc = "synchronous multiplication factor (N * 25 MHz)"]
    BfVal11 = 0x01,
    #[doc = "synchronous multiplication factor (N * 25 MHz)"]
    BfVal12 = 0x02,
    #[doc = "synchronous multiplication factor (N * 25 MHz)"]
    BfVal13 = 0x03,
    #[doc = "synchronous multiplication factor (N * 25 MHz)"]
    BfVal14 = 0x04,
    #[doc = "synchronous multiplication factor (N * 25 MHz)"]
    BfVal15 = 0x05,
    #[doc = "synchronous multiplication factor (N * 25 MHz)"]
    BfVal16 = 0x06,
    #[doc = "synchronous multiplication factor (N * 25 MHz)"]
    BfVal17 = 0x07,
    #[doc = "synchronous multiplication factor (N * 25 MHz)"]
    BfVal18 = 0x08,
    #[doc = "synchronous multiplication factor (N * 25 MHz)"]
    BfVal19 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
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
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl PdiOnChipBusConfigurationBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PdiOnChipBusConfigurationBf0 {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PdiOnChipBusConfigurationBf0 {
    #[inline(always)]
    fn from(val: u8) -> PdiOnChipBusConfigurationBf0 {
        PdiOnChipBusConfigurationBf0::from_bits(val)
    }
}
impl From<PdiOnChipBusConfigurationBf0> for u8 {
    #[inline(always)]
    fn from(val: PdiOnChipBusConfigurationBf0) -> u8 {
        PdiOnChipBusConfigurationBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdiOnChipBusConfigurationBf5 {
    #[doc = "Intel Avalon"]
    BfVal0 = 0x0,
    #[doc = "AXI 010: Xilinx PLB v4.6"]
    BfVal1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Xilinx OPB"]
    BfVal2 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl PdiOnChipBusConfigurationBf5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PdiOnChipBusConfigurationBf5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PdiOnChipBusConfigurationBf5 {
    #[inline(always)]
    fn from(val: u8) -> PdiOnChipBusConfigurationBf5 {
        PdiOnChipBusConfigurationBf5::from_bits(val)
    }
}
impl From<PdiOnChipBusConfigurationBf5> for u8 {
    #[inline(always)]
    fn from(val: PdiOnChipBusConfigurationBf5) -> u8 {
        PdiOnChipBusConfigurationBf5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdiOnChipBusExtendedConfigurationBf0 {
    #[doc = "4 cycles"]
    BfVal0 = 0x0,
    #[doc = "1 cycle (typical)"]
    BfVal1 = 0x01,
    #[doc = "2 cycles"]
    BfVal2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PdiOnChipBusExtendedConfigurationBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PdiOnChipBusExtendedConfigurationBf0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PdiOnChipBusExtendedConfigurationBf0 {
    #[inline(always)]
    fn from(val: u8) -> PdiOnChipBusExtendedConfigurationBf0 {
        PdiOnChipBusExtendedConfigurationBf0::from_bits(val)
    }
}
impl From<PdiOnChipBusExtendedConfigurationBf0> for u8 {
    #[inline(always)]
    fn from(val: PdiOnChipBusExtendedConfigurationBf0) -> u8 {
        PdiOnChipBusExtendedConfigurationBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdiOnChipBusExtendedConfigurationBf8 {
    #[doc = "AXI3"]
    BfVal0 = 0x0,
    #[doc = "AXI4"]
    BfVal1 = 0x01,
    #[doc = "AXI4 LITE"]
    BfVal2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl PdiOnChipBusExtendedConfigurationBf8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PdiOnChipBusExtendedConfigurationBf8 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PdiOnChipBusExtendedConfigurationBf8 {
    #[inline(always)]
    fn from(val: u8) -> PdiOnChipBusExtendedConfigurationBf8 {
        PdiOnChipBusExtendedConfigurationBf8::from_bits(val)
    }
}
impl From<PdiOnChipBusExtendedConfigurationBf8> for u8 {
    #[inline(always)]
    fn from(val: PdiOnChipBusExtendedConfigurationBf8) -> u8 {
        PdiOnChipBusExtendedConfigurationBf8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PhyAddressBf7 {
    #[doc = "Register 0x0510\\[7:3\\] shows PHY address of port 0 (this is also the PHY address offset, if the PHY addresses are consecutive)"]
    BfVal0 = 0x0,
    #[doc = "Register 0x0510\\[7:3\\] shows PHY address of port 0x0512\\[4:0\\] (valid values 0-3)"]
    BfVal1 = 0x01,
}
impl PhyAddressBf7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PhyAddressBf7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PhyAddressBf7 {
    #[inline(always)]
    fn from(val: u8) -> PhyAddressBf7 {
        PhyAddressBf7::from_bits(val)
    }
}
impl From<PhyAddressBf7> for u8 {
    #[inline(always)]
    fn from(val: PhyAddressBf7) -> u8 {
        PhyAddressBf7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PhyPortStatusBf0 {
    #[doc = "No physical link"]
    BfVal0 = 0x0,
    #[doc = "Physical link detected"]
    BfVal1 = 0x01,
}
impl PhyPortStatusBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PhyPortStatusBf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PhyPortStatusBf0 {
    #[inline(always)]
    fn from(val: u8) -> PhyPortStatusBf0 {
        PhyPortStatusBf0::from_bits(val)
    }
}
impl From<PhyPortStatusBf0> for u8 {
    #[inline(always)]
    fn from(val: PhyPortStatusBf0) -> u8 {
        PhyPortStatusBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PhyPortStatusBf1 {
    #[doc = "No link"]
    BfVal0 = 0x0,
    #[doc = "Link detected"]
    BfVal1 = 0x01,
}
impl PhyPortStatusBf1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PhyPortStatusBf1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PhyPortStatusBf1 {
    #[inline(always)]
    fn from(val: u8) -> PhyPortStatusBf1 {
        PhyPortStatusBf1::from_bits(val)
    }
}
impl From<PhyPortStatusBf1> for u8 {
    #[inline(always)]
    fn from(val: PhyPortStatusBf1) -> u8 {
        PhyPortStatusBf1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PhyPortStatusBf2 {
    #[doc = "No error"]
    BfVal0 = 0x0,
    #[doc = "Link error, link inhibited"]
    BfVal1 = 0x01,
}
impl PhyPortStatusBf2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PhyPortStatusBf2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PhyPortStatusBf2 {
    #[inline(always)]
    fn from(val: u8) -> PhyPortStatusBf2 {
        PhyPortStatusBf2::from_bits(val)
    }
}
impl From<PhyPortStatusBf2> for u8 {
    #[inline(always)]
    fn from(val: PhyPortStatusBf2) -> u8 {
        PhyPortStatusBf2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PhyPortStatusBf3 {
    #[doc = "No read error occurred"]
    BfVal0 = 0x0,
    #[doc = "A read error has occurred"]
    BfVal1 = 0x01,
}
impl PhyPortStatusBf3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PhyPortStatusBf3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PhyPortStatusBf3 {
    #[inline(always)]
    fn from(val: u8) -> PhyPortStatusBf3 {
        PhyPortStatusBf3::from_bits(val)
    }
}
impl From<PhyPortStatusBf3> for u8 {
    #[inline(always)]
    fn from(val: PhyPortStatusBf3) -> u8 {
        PhyPortStatusBf3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PhyPortStatusBf4 {
    #[doc = "No error detected"]
    BfVal0 = 0x0,
    #[doc = "Link partner error"]
    BfVal1 = 0x01,
}
impl PhyPortStatusBf4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PhyPortStatusBf4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PhyPortStatusBf4 {
    #[inline(always)]
    fn from(val: u8) -> PhyPortStatusBf4 {
        PhyPortStatusBf4::from_bits(val)
    }
}
impl From<PhyPortStatusBf4> for u8 {
    #[inline(always)]
    fn from(val: PhyPortStatusBf4) -> u8 {
        PhyPortStatusBf4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PhyPortStatusBf5 {
    #[doc = "No update"]
    BfVal0 = 0x0,
    #[doc = "PHY configuration was updated"]
    BfVal1 = 0x01,
}
impl PhyPortStatusBf5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PhyPortStatusBf5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PhyPortStatusBf5 {
    #[inline(always)]
    fn from(val: u8) -> PhyPortStatusBf5 {
        PhyPortStatusBf5::from_bits(val)
    }
}
impl From<PhyPortStatusBf5> for u8 {
    #[inline(always)]
    fn from(val: PhyPortStatusBf5) -> u8 {
        PhyPortStatusBf5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegisterEepromPdiAccessStateBf0 {
    #[doc = "PDI releases EEPROM access"]
    BfVal0 = 0x0,
    #[doc = "PDI takes EEPROM access (PDI has EEPROM control)"]
    BfVal1 = 0x01,
}
impl RegisterEepromPdiAccessStateBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegisterEepromPdiAccessStateBf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegisterEepromPdiAccessStateBf0 {
    #[inline(always)]
    fn from(val: u8) -> RegisterEepromPdiAccessStateBf0 {
        RegisterEepromPdiAccessStateBf0::from_bits(val)
    }
}
impl From<RegisterEepromPdiAccessStateBf0> for u8 {
    #[inline(always)]
    fn from(val: RegisterEepromPdiAccessStateBf0) -> u8 {
        RegisterEepromPdiAccessStateBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegisterEepromPdiAccessStatePdiBf0 {
    #[doc = "PDI releases EEPROM access"]
    BfVal0 = 0x0,
    #[doc = "PDI takes EEPROM access (PDI has EEPROM control)"]
    BfVal1 = 0x01,
}
impl RegisterEepromPdiAccessStatePdiBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegisterEepromPdiAccessStatePdiBf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegisterEepromPdiAccessStatePdiBf0 {
    #[inline(always)]
    fn from(val: u8) -> RegisterEepromPdiAccessStatePdiBf0 {
        RegisterEepromPdiAccessStatePdiBf0::from_bits(val)
    }
}
impl From<RegisterEepromPdiAccessStatePdiBf0> for u8 {
    #[inline(always)]
    fn from(val: RegisterEepromPdiAccessStatePdiBf0) -> u8 {
        RegisterEepromPdiAccessStatePdiBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegisterWriteProtectionBf0 {
    #[doc = "Protection disabled"]
    BfVal0 = 0x0,
    #[doc = "Protection enabled"]
    BfVal1 = 0x01,
}
impl RegisterWriteProtectionBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegisterWriteProtectionBf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegisterWriteProtectionBf0 {
    #[inline(always)]
    fn from(val: u8) -> RegisterWriteProtectionBf0 {
        RegisterWriteProtectionBf0::from_bits(val)
    }
}
impl From<RegisterWriteProtectionBf0> for u8 {
    #[inline(always)]
    fn from(val: RegisterWriteProtectionBf0) -> u8 {
        RegisterWriteProtectionBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegisterWriteProtectionPdiBf0 {
    #[doc = "Protection disabled"]
    BfVal0 = 0x0,
    #[doc = "Protection enabled"]
    BfVal1 = 0x01,
}
impl RegisterWriteProtectionPdiBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegisterWriteProtectionPdiBf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegisterWriteProtectionPdiBf0 {
    #[inline(always)]
    fn from(val: u8) -> RegisterWriteProtectionPdiBf0 {
        RegisterWriteProtectionPdiBf0::from_bits(val)
    }
}
impl From<RegisterWriteProtectionPdiBf0> for u8 {
    #[inline(always)]
    fn from(val: RegisterWriteProtectionPdiBf0) -> u8 {
        RegisterWriteProtectionPdiBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RunLedOverrideBf4 {
    #[doc = "Override disabled"]
    BfVal0 = 0x0,
    #[doc = "Override enabled"]
    BfVal1 = 0x01,
}
impl RunLedOverrideBf4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RunLedOverrideBf4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RunLedOverrideBf4 {
    #[inline(always)]
    fn from(val: u8) -> RunLedOverrideBf4 {
        RunLedOverrideBf4::from_bits(val)
    }
}
impl From<RunLedOverrideBf4> for u8 {
    #[inline(always)]
    fn from(val: RunLedOverrideBf4) -> u8 {
        RunLedOverrideBf4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SyncLatch1And0PdiConfigurationBf0 {
    #[doc = "Push-Pull active low"]
    BfVal0 = 0x0,
    #[doc = "Open Drain (active low)"]
    BfVal1 = 0x01,
    #[doc = "Push-Pull active high"]
    BfVal2 = 0x02,
    #[doc = "Open Source (active high)"]
    BfVal3 = 0x03,
}
impl SyncLatch1And0PdiConfigurationBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SyncLatch1And0PdiConfigurationBf0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SyncLatch1And0PdiConfigurationBf0 {
    #[inline(always)]
    fn from(val: u8) -> SyncLatch1And0PdiConfigurationBf0 {
        SyncLatch1And0PdiConfigurationBf0::from_bits(val)
    }
}
impl From<SyncLatch1And0PdiConfigurationBf0> for u8 {
    #[inline(always)]
    fn from(val: SyncLatch1And0PdiConfigurationBf0) -> u8 {
        SyncLatch1And0PdiConfigurationBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SyncLatch1And0PdiConfigurationBf2 {
    #[doc = "LATCH0 Input"]
    BfVal0 = 0x0,
    #[doc = "SYNC0 Output"]
    BfVal1 = 0x01,
}
impl SyncLatch1And0PdiConfigurationBf2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SyncLatch1And0PdiConfigurationBf2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SyncLatch1And0PdiConfigurationBf2 {
    #[inline(always)]
    fn from(val: u8) -> SyncLatch1And0PdiConfigurationBf2 {
        SyncLatch1And0PdiConfigurationBf2::from_bits(val)
    }
}
impl From<SyncLatch1And0PdiConfigurationBf2> for u8 {
    #[inline(always)]
    fn from(val: SyncLatch1And0PdiConfigurationBf2) -> u8 {
        SyncLatch1And0PdiConfigurationBf2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SyncLatch1And0PdiConfigurationBf3 {
    #[doc = "Disabled"]
    BfVal0 = 0x0,
    #[doc = "Enabled"]
    BfVal1 = 0x01,
}
impl SyncLatch1And0PdiConfigurationBf3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SyncLatch1And0PdiConfigurationBf3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SyncLatch1And0PdiConfigurationBf3 {
    #[inline(always)]
    fn from(val: u8) -> SyncLatch1And0PdiConfigurationBf3 {
        SyncLatch1And0PdiConfigurationBf3::from_bits(val)
    }
}
impl From<SyncLatch1And0PdiConfigurationBf3> for u8 {
    #[inline(always)]
    fn from(val: SyncLatch1And0PdiConfigurationBf3) -> u8 {
        SyncLatch1And0PdiConfigurationBf3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SyncLatch1And0PdiConfigurationBf4 {
    #[doc = "Push-Pull active low"]
    BfVal0 = 0x0,
    #[doc = "Open Drain (active low)"]
    BfVal1 = 0x01,
    #[doc = "Push-Pull active high"]
    BfVal2 = 0x02,
    #[doc = "Open Source (active high)"]
    BfVal3 = 0x03,
}
impl SyncLatch1And0PdiConfigurationBf4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SyncLatch1And0PdiConfigurationBf4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SyncLatch1And0PdiConfigurationBf4 {
    #[inline(always)]
    fn from(val: u8) -> SyncLatch1And0PdiConfigurationBf4 {
        SyncLatch1And0PdiConfigurationBf4::from_bits(val)
    }
}
impl From<SyncLatch1And0PdiConfigurationBf4> for u8 {
    #[inline(always)]
    fn from(val: SyncLatch1And0PdiConfigurationBf4) -> u8 {
        SyncLatch1And0PdiConfigurationBf4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SyncLatch1And0PdiConfigurationBf6 {
    #[doc = "LATCH1 input"]
    BfVal0 = 0x0,
    #[doc = "SYNC1 output"]
    BfVal1 = 0x01,
}
impl SyncLatch1And0PdiConfigurationBf6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SyncLatch1And0PdiConfigurationBf6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SyncLatch1And0PdiConfigurationBf6 {
    #[inline(always)]
    fn from(val: u8) -> SyncLatch1And0PdiConfigurationBf6 {
        SyncLatch1And0PdiConfigurationBf6::from_bits(val)
    }
}
impl From<SyncLatch1And0PdiConfigurationBf6> for u8 {
    #[inline(always)]
    fn from(val: SyncLatch1And0PdiConfigurationBf6) -> u8 {
        SyncLatch1And0PdiConfigurationBf6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SyncLatch1And0PdiConfigurationBf7 {
    #[doc = "Disabled"]
    BfVal0 = 0x0,
    #[doc = "Enabled"]
    BfVal1 = 0x01,
}
impl SyncLatch1And0PdiConfigurationBf7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SyncLatch1And0PdiConfigurationBf7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SyncLatch1And0PdiConfigurationBf7 {
    #[inline(always)]
    fn from(val: u8) -> SyncLatch1And0PdiConfigurationBf7 {
        SyncLatch1And0PdiConfigurationBf7::from_bits(val)
    }
}
impl From<SyncLatch1And0PdiConfigurationBf7> for u8 {
    #[inline(always)]
    fn from(val: SyncLatch1And0PdiConfigurationBf7) -> u8 {
        SyncLatch1And0PdiConfigurationBf7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SyncmanagerActivateBf0 {
    #[doc = "Disable: Access to Memory without SyncManager control"]
    BfVal0 = 0x0,
    #[doc = "Enable: SyncManager is active and controls Memory area set in configuration"]
    BfVal1 = 0x01,
}
impl SyncmanagerActivateBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SyncmanagerActivateBf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SyncmanagerActivateBf0 {
    #[inline(always)]
    fn from(val: u8) -> SyncmanagerActivateBf0 {
        SyncmanagerActivateBf0::from_bits(val)
    }
}
impl From<SyncmanagerActivateBf0> for u8 {
    #[inline(always)]
    fn from(val: SyncmanagerActivateBf0) -> u8 {
        SyncmanagerActivateBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SyncmanagerActivateBf6 {
    #[doc = "No"]
    BfVal0 = 0x0,
    #[doc = "Generate Latch event when EtherCAT master issues a buffer exchange"]
    BfVal1 = 0x01,
}
impl SyncmanagerActivateBf6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SyncmanagerActivateBf6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SyncmanagerActivateBf6 {
    #[inline(always)]
    fn from(val: u8) -> SyncmanagerActivateBf6 {
        SyncmanagerActivateBf6::from_bits(val)
    }
}
impl From<SyncmanagerActivateBf6> for u8 {
    #[inline(always)]
    fn from(val: SyncmanagerActivateBf6) -> u8 {
        SyncmanagerActivateBf6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SyncmanagerActivateBf7 {
    #[doc = "No"]
    BfVal0 = 0x0,
    #[doc = "Generate Latch events when PDI issues a buffer exchange or when PDI accesses buffer start address"]
    BfVal1 = 0x01,
}
impl SyncmanagerActivateBf7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SyncmanagerActivateBf7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SyncmanagerActivateBf7 {
    #[inline(always)]
    fn from(val: u8) -> SyncmanagerActivateBf7 {
        SyncmanagerActivateBf7::from_bits(val)
    }
}
impl From<SyncmanagerActivateBf7> for u8 {
    #[inline(always)]
    fn from(val: SyncmanagerActivateBf7) -> u8 {
        SyncmanagerActivateBf7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SyncmanagerActivatePdiBf0 {
    #[doc = "Disable: Access to Memory without SyncManager control"]
    BfVal0 = 0x0,
    #[doc = "Enable: SyncManager is active and controls Memory area set in configuration"]
    BfVal1 = 0x01,
}
impl SyncmanagerActivatePdiBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SyncmanagerActivatePdiBf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SyncmanagerActivatePdiBf0 {
    #[inline(always)]
    fn from(val: u8) -> SyncmanagerActivatePdiBf0 {
        SyncmanagerActivatePdiBf0::from_bits(val)
    }
}
impl From<SyncmanagerActivatePdiBf0> for u8 {
    #[inline(always)]
    fn from(val: SyncmanagerActivatePdiBf0) -> u8 {
        SyncmanagerActivatePdiBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SyncmanagerActivatePdiBf6 {
    #[doc = "No"]
    BfVal0 = 0x0,
    #[doc = "Generate Latch event when EtherCAT master issues a buffer exchange"]
    BfVal1 = 0x01,
}
impl SyncmanagerActivatePdiBf6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SyncmanagerActivatePdiBf6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SyncmanagerActivatePdiBf6 {
    #[inline(always)]
    fn from(val: u8) -> SyncmanagerActivatePdiBf6 {
        SyncmanagerActivatePdiBf6::from_bits(val)
    }
}
impl From<SyncmanagerActivatePdiBf6> for u8 {
    #[inline(always)]
    fn from(val: SyncmanagerActivatePdiBf6) -> u8 {
        SyncmanagerActivatePdiBf6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SyncmanagerActivatePdiBf7 {
    #[doc = "No"]
    BfVal0 = 0x0,
    #[doc = "Generate Latch events when PDI issues a buffer exchange or when PDI accesses buffer start address"]
    BfVal1 = 0x01,
}
impl SyncmanagerActivatePdiBf7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SyncmanagerActivatePdiBf7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SyncmanagerActivatePdiBf7 {
    #[inline(always)]
    fn from(val: u8) -> SyncmanagerActivatePdiBf7 {
        SyncmanagerActivatePdiBf7::from_bits(val)
    }
}
impl From<SyncmanagerActivatePdiBf7> for u8 {
    #[inline(always)]
    fn from(val: SyncmanagerActivatePdiBf7) -> u8 {
        SyncmanagerActivatePdiBf7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SyncmanagerControlRegisterBf0 {
    #[doc = "Buffered (3 buffer mode)"]
    BfVal0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Mailbox (Single buffer mode)"]
    BfVal2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl SyncmanagerControlRegisterBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SyncmanagerControlRegisterBf0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SyncmanagerControlRegisterBf0 {
    #[inline(always)]
    fn from(val: u8) -> SyncmanagerControlRegisterBf0 {
        SyncmanagerControlRegisterBf0::from_bits(val)
    }
}
impl From<SyncmanagerControlRegisterBf0> for u8 {
    #[inline(always)]
    fn from(val: SyncmanagerControlRegisterBf0) -> u8 {
        SyncmanagerControlRegisterBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SyncmanagerControlRegisterBf2 {
    #[doc = "Read: ECAT read access, PDI write access."]
    BfVal0 = 0x0,
    #[doc = "Write: ECAT write access, PDI read access."]
    BfVal1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl SyncmanagerControlRegisterBf2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SyncmanagerControlRegisterBf2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SyncmanagerControlRegisterBf2 {
    #[inline(always)]
    fn from(val: u8) -> SyncmanagerControlRegisterBf2 {
        SyncmanagerControlRegisterBf2::from_bits(val)
    }
}
impl From<SyncmanagerControlRegisterBf2> for u8 {
    #[inline(always)]
    fn from(val: SyncmanagerControlRegisterBf2) -> u8 {
        SyncmanagerControlRegisterBf2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SyncmanagerControlRegisterBf4 {
    #[doc = "Disabled"]
    BfVal0 = 0x0,
    #[doc = "Enabled"]
    BfVal1 = 0x01,
}
impl SyncmanagerControlRegisterBf4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SyncmanagerControlRegisterBf4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SyncmanagerControlRegisterBf4 {
    #[inline(always)]
    fn from(val: u8) -> SyncmanagerControlRegisterBf4 {
        SyncmanagerControlRegisterBf4::from_bits(val)
    }
}
impl From<SyncmanagerControlRegisterBf4> for u8 {
    #[inline(always)]
    fn from(val: SyncmanagerControlRegisterBf4) -> u8 {
        SyncmanagerControlRegisterBf4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SyncmanagerControlRegisterBf5 {
    #[doc = "Disabled"]
    BfVal0 = 0x0,
    #[doc = "Enabled"]
    BfVal1 = 0x01,
}
impl SyncmanagerControlRegisterBf5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SyncmanagerControlRegisterBf5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SyncmanagerControlRegisterBf5 {
    #[inline(always)]
    fn from(val: u8) -> SyncmanagerControlRegisterBf5 {
        SyncmanagerControlRegisterBf5::from_bits(val)
    }
}
impl From<SyncmanagerControlRegisterBf5> for u8 {
    #[inline(always)]
    fn from(val: SyncmanagerControlRegisterBf5) -> u8 {
        SyncmanagerControlRegisterBf5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SyncmanagerControlRegisterBf6 {
    #[doc = "Disabled"]
    BfVal0 = 0x0,
    #[doc = "Enabled"]
    BfVal1 = 0x01,
}
impl SyncmanagerControlRegisterBf6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SyncmanagerControlRegisterBf6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SyncmanagerControlRegisterBf6 {
    #[inline(always)]
    fn from(val: u8) -> SyncmanagerControlRegisterBf6 {
        SyncmanagerControlRegisterBf6::from_bits(val)
    }
}
impl From<SyncmanagerControlRegisterBf6> for u8 {
    #[inline(always)]
    fn from(val: SyncmanagerControlRegisterBf6) -> u8 {
        SyncmanagerControlRegisterBf6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SyncmanagerControlRegisterPdiBf0 {
    #[doc = "Buffered (3 buffer mode)"]
    BfVal0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Mailbox (Single buffer mode)"]
    BfVal2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl SyncmanagerControlRegisterPdiBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SyncmanagerControlRegisterPdiBf0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SyncmanagerControlRegisterPdiBf0 {
    #[inline(always)]
    fn from(val: u8) -> SyncmanagerControlRegisterPdiBf0 {
        SyncmanagerControlRegisterPdiBf0::from_bits(val)
    }
}
impl From<SyncmanagerControlRegisterPdiBf0> for u8 {
    #[inline(always)]
    fn from(val: SyncmanagerControlRegisterPdiBf0) -> u8 {
        SyncmanagerControlRegisterPdiBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SyncmanagerControlRegisterPdiBf2 {
    #[doc = "Read: ECAT read access, PDI write access."]
    BfVal0 = 0x0,
    #[doc = "Write: ECAT write access, PDI read access."]
    BfVal1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl SyncmanagerControlRegisterPdiBf2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SyncmanagerControlRegisterPdiBf2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SyncmanagerControlRegisterPdiBf2 {
    #[inline(always)]
    fn from(val: u8) -> SyncmanagerControlRegisterPdiBf2 {
        SyncmanagerControlRegisterPdiBf2::from_bits(val)
    }
}
impl From<SyncmanagerControlRegisterPdiBf2> for u8 {
    #[inline(always)]
    fn from(val: SyncmanagerControlRegisterPdiBf2) -> u8 {
        SyncmanagerControlRegisterPdiBf2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SyncmanagerControlRegisterPdiBf4 {
    #[doc = "Disabled"]
    BfVal0 = 0x0,
    #[doc = "Enabled"]
    BfVal1 = 0x01,
}
impl SyncmanagerControlRegisterPdiBf4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SyncmanagerControlRegisterPdiBf4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SyncmanagerControlRegisterPdiBf4 {
    #[inline(always)]
    fn from(val: u8) -> SyncmanagerControlRegisterPdiBf4 {
        SyncmanagerControlRegisterPdiBf4::from_bits(val)
    }
}
impl From<SyncmanagerControlRegisterPdiBf4> for u8 {
    #[inline(always)]
    fn from(val: SyncmanagerControlRegisterPdiBf4) -> u8 {
        SyncmanagerControlRegisterPdiBf4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SyncmanagerControlRegisterPdiBf5 {
    #[doc = "Disabled"]
    BfVal0 = 0x0,
    #[doc = "Enabled"]
    BfVal1 = 0x01,
}
impl SyncmanagerControlRegisterPdiBf5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SyncmanagerControlRegisterPdiBf5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SyncmanagerControlRegisterPdiBf5 {
    #[inline(always)]
    fn from(val: u8) -> SyncmanagerControlRegisterPdiBf5 {
        SyncmanagerControlRegisterPdiBf5::from_bits(val)
    }
}
impl From<SyncmanagerControlRegisterPdiBf5> for u8 {
    #[inline(always)]
    fn from(val: SyncmanagerControlRegisterPdiBf5) -> u8 {
        SyncmanagerControlRegisterPdiBf5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SyncmanagerControlRegisterPdiBf6 {
    #[doc = "Disabled"]
    BfVal0 = 0x0,
    #[doc = "Enabled"]
    BfVal1 = 0x01,
}
impl SyncmanagerControlRegisterPdiBf6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SyncmanagerControlRegisterPdiBf6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SyncmanagerControlRegisterPdiBf6 {
    #[inline(always)]
    fn from(val: u8) -> SyncmanagerControlRegisterPdiBf6 {
        SyncmanagerControlRegisterPdiBf6::from_bits(val)
    }
}
impl From<SyncmanagerControlRegisterPdiBf6> for u8 {
    #[inline(always)]
    fn from(val: SyncmanagerControlRegisterPdiBf6) -> u8 {
        SyncmanagerControlRegisterPdiBf6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SyncmanagerStatusBf0 {
    #[doc = "Interrupt cleared after first byte of buffer was read"]
    BfVal1 = 0x0,
    #[doc = "Interrupt after buffer was completely and successfully written"]
    BfVal0 = 0x01,
}
impl SyncmanagerStatusBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SyncmanagerStatusBf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SyncmanagerStatusBf0 {
    #[inline(always)]
    fn from(val: u8) -> SyncmanagerStatusBf0 {
        SyncmanagerStatusBf0::from_bits(val)
    }
}
impl From<SyncmanagerStatusBf0> for u8 {
    #[inline(always)]
    fn from(val: SyncmanagerStatusBf0) -> u8 {
        SyncmanagerStatusBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SyncmanagerStatusBf1 {
    #[doc = "Interrupt cleared after first byte of buffer was written"]
    BfVal1 = 0x0,
    #[doc = "Interrupt after buffer was completely and successfully read"]
    BfVal0 = 0x01,
}
impl SyncmanagerStatusBf1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SyncmanagerStatusBf1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SyncmanagerStatusBf1 {
    #[inline(always)]
    fn from(val: u8) -> SyncmanagerStatusBf1 {
        SyncmanagerStatusBf1::from_bits(val)
    }
}
impl From<SyncmanagerStatusBf1> for u8 {
    #[inline(always)]
    fn from(val: SyncmanagerStatusBf1) -> u8 {
        SyncmanagerStatusBf1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SyncmanagerStatusBf3 {
    #[doc = "Mailbox empty"]
    BfVal0 = 0x0,
    #[doc = "Mailbox full"]
    BfVal1 = 0x01,
}
impl SyncmanagerStatusBf3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SyncmanagerStatusBf3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SyncmanagerStatusBf3 {
    #[inline(always)]
    fn from(val: u8) -> SyncmanagerStatusBf3 {
        SyncmanagerStatusBf3::from_bits(val)
    }
}
impl From<SyncmanagerStatusBf3> for u8 {
    #[inline(always)]
    fn from(val: SyncmanagerStatusBf3) -> u8 {
        SyncmanagerStatusBf3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SyncmanagerStatusBf4 {
    #[doc = "1st buffer"]
    BfVal0 = 0x0,
    #[doc = "2nd buffer"]
    BfVal1 = 0x01,
    #[doc = "3rd buffer"]
    BfVal2 = 0x02,
    #[doc = "(no buffer written)"]
    BfVal3 = 0x03,
}
impl SyncmanagerStatusBf4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SyncmanagerStatusBf4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SyncmanagerStatusBf4 {
    #[inline(always)]
    fn from(val: u8) -> SyncmanagerStatusBf4 {
        SyncmanagerStatusBf4::from_bits(val)
    }
}
impl From<SyncmanagerStatusBf4> for u8 {
    #[inline(always)]
    fn from(val: SyncmanagerStatusBf4) -> u8 {
        SyncmanagerStatusBf4::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UniPulseLengthOfSyncsignalsBf0(pub u16);
impl UniPulseLengthOfSyncsignalsBf0 {
    #[doc = "Acknowledge mode: SyncSignal will be cleared by reading SYNC\\[1:0\\] Status register"]
    pub const BF_VAL_0: Self = Self(0x0);
}
impl UniPulseLengthOfSyncsignalsBf0 {
    pub const fn from_bits(val: u16) -> UniPulseLengthOfSyncsignalsBf0 {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for UniPulseLengthOfSyncsignalsBf0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("BF_VAL_0"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UniPulseLengthOfSyncsignalsBf0 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "BF_VAL_0"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for UniPulseLengthOfSyncsignalsBf0 {
    #[inline(always)]
    fn from(val: u16) -> UniPulseLengthOfSyncsignalsBf0 {
        UniPulseLengthOfSyncsignalsBf0::from_bits(val)
    }
}
impl From<UniPulseLengthOfSyncsignalsBf0> for u16 {
    #[inline(always)]
    fn from(val: UniPulseLengthOfSyncsignalsBf0) -> u16 {
        UniPulseLengthOfSyncsignalsBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UnitActivationRegisterBf0 {
    #[doc = "Deactivated"]
    BfVal0 = 0x0,
    #[doc = "Activated"]
    BfVal1 = 0x01,
}
impl UnitActivationRegisterBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UnitActivationRegisterBf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UnitActivationRegisterBf0 {
    #[inline(always)]
    fn from(val: u8) -> UnitActivationRegisterBf0 {
        UnitActivationRegisterBf0::from_bits(val)
    }
}
impl From<UnitActivationRegisterBf0> for u8 {
    #[inline(always)]
    fn from(val: UnitActivationRegisterBf0) -> u8 {
        UnitActivationRegisterBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UnitActivationRegisterBf1 {
    #[doc = "Deactivated"]
    BfVal0 = 0x0,
    #[doc = "SYNC0 pulse is generated"]
    BfVal1 = 0x01,
}
impl UnitActivationRegisterBf1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UnitActivationRegisterBf1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UnitActivationRegisterBf1 {
    #[inline(always)]
    fn from(val: u8) -> UnitActivationRegisterBf1 {
        UnitActivationRegisterBf1::from_bits(val)
    }
}
impl From<UnitActivationRegisterBf1> for u8 {
    #[inline(always)]
    fn from(val: UnitActivationRegisterBf1) -> u8 {
        UnitActivationRegisterBf1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UnitActivationRegisterBf2 {
    #[doc = "Deactivated"]
    BfVal0 = 0x0,
    #[doc = "SYNC1 pulse is generated"]
    BfVal1 = 0x01,
}
impl UnitActivationRegisterBf2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UnitActivationRegisterBf2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UnitActivationRegisterBf2 {
    #[inline(always)]
    fn from(val: u8) -> UnitActivationRegisterBf2 {
        UnitActivationRegisterBf2::from_bits(val)
    }
}
impl From<UnitActivationRegisterBf2> for u8 {
    #[inline(always)]
    fn from(val: UnitActivationRegisterBf2) -> u8 {
        UnitActivationRegisterBf2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UnitActivationRegisterBf3 {
    #[doc = "Disabled"]
    BfVal0 = 0x0,
    #[doc = "Auto-activation enabled. 0x0981\\[0\\] is set automatically after Start Time is written"]
    BfVal1 = 0x01,
}
impl UnitActivationRegisterBf3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UnitActivationRegisterBf3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UnitActivationRegisterBf3 {
    #[inline(always)]
    fn from(val: u8) -> UnitActivationRegisterBf3 {
        UnitActivationRegisterBf3::from_bits(val)
    }
}
impl From<UnitActivationRegisterBf3> for u8 {
    #[inline(always)]
    fn from(val: UnitActivationRegisterBf3) -> u8 {
        UnitActivationRegisterBf3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UnitActivationRegisterBf4 {
    #[doc = "No extension"]
    BfVal0 = 0x0,
    #[doc = "Extend 32 bit written Start Time to 64 bit"]
    BfVal1 = 0x01,
}
impl UnitActivationRegisterBf4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UnitActivationRegisterBf4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UnitActivationRegisterBf4 {
    #[inline(always)]
    fn from(val: u8) -> UnitActivationRegisterBf4 {
        UnitActivationRegisterBf4::from_bits(val)
    }
}
impl From<UnitActivationRegisterBf4> for u8 {
    #[inline(always)]
    fn from(val: UnitActivationRegisterBf4) -> u8 {
        UnitActivationRegisterBf4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UnitActivationRegisterBf5 {
    #[doc = "Disabled. SyncSignal generation if Start Time is reached."]
    BfVal0 = 0x0,
    #[doc = "Immediate SyncSignal generation if Start Time is outside near future (see 0x0981\\[6\\])"]
    BfVal1 = 0x01,
}
impl UnitActivationRegisterBf5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UnitActivationRegisterBf5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UnitActivationRegisterBf5 {
    #[inline(always)]
    fn from(val: u8) -> UnitActivationRegisterBf5 {
        UnitActivationRegisterBf5::from_bits(val)
    }
}
impl From<UnitActivationRegisterBf5> for u8 {
    #[inline(always)]
    fn from(val: UnitActivationRegisterBf5) -> u8 {
        UnitActivationRegisterBf5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UnitActivationRegisterBf6 {
    #[doc = "1/2 DC width future (2^31ns or 2^63ns)"]
    BfVal0 = 0x0,
    #[doc = "~2.1 sec. future (2^31ns)"]
    BfVal1 = 0x01,
}
impl UnitActivationRegisterBf6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UnitActivationRegisterBf6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UnitActivationRegisterBf6 {
    #[inline(always)]
    fn from(val: u8) -> UnitActivationRegisterBf6 {
        UnitActivationRegisterBf6::from_bits(val)
    }
}
impl From<UnitActivationRegisterBf6> for u8 {
    #[inline(always)]
    fn from(val: UnitActivationRegisterBf6) -> u8 {
        UnitActivationRegisterBf6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UnitActivationRegisterBf7 {
    #[doc = "Deactivated"]
    BfVal0 = 0x0,
    #[doc = "Immediately generate one ping only on SYNC0-1 according to 0x0981\\[2:1\\] for debugging."]
    BfVal1 = 0x01,
}
impl UnitActivationRegisterBf7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UnitActivationRegisterBf7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UnitActivationRegisterBf7 {
    #[inline(always)]
    fn from(val: u8) -> UnitActivationRegisterBf7 {
        UnitActivationRegisterBf7::from_bits(val)
    }
}
impl From<UnitActivationRegisterBf7> for u8 {
    #[inline(always)]
    fn from(val: UnitActivationRegisterBf7) -> u8 {
        UnitActivationRegisterBf7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UnitActivationStatusBf0 {
    #[doc = "First SYNC0 pulse is not pending"]
    BfVal0 = 0x0,
    #[doc = "First SYNC0 pulse is pending"]
    BfVal1 = 0x01,
}
impl UnitActivationStatusBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UnitActivationStatusBf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UnitActivationStatusBf0 {
    #[inline(always)]
    fn from(val: u8) -> UnitActivationStatusBf0 {
        UnitActivationStatusBf0::from_bits(val)
    }
}
impl From<UnitActivationStatusBf0> for u8 {
    #[inline(always)]
    fn from(val: UnitActivationStatusBf0) -> u8 {
        UnitActivationStatusBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UnitActivationStatusBf1 {
    #[doc = "First SYNC1 pulse is not pending"]
    BfVal0 = 0x0,
    #[doc = "First SYNC1 pulse is pending"]
    BfVal1 = 0x01,
}
impl UnitActivationStatusBf1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UnitActivationStatusBf1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UnitActivationStatusBf1 {
    #[inline(always)]
    fn from(val: u8) -> UnitActivationStatusBf1 {
        UnitActivationStatusBf1::from_bits(val)
    }
}
impl From<UnitActivationStatusBf1> for u8 {
    #[inline(always)]
    fn from(val: UnitActivationStatusBf1) -> u8 {
        UnitActivationStatusBf1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UnitActivationStatusBf2 {
    #[doc = "Start Time was within near future"]
    BfVal0 = 0x0,
    #[doc = "Start Time was out of near future (0x0981\\[6\\])"]
    BfVal1 = 0x01,
}
impl UnitActivationStatusBf2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UnitActivationStatusBf2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UnitActivationStatusBf2 {
    #[inline(always)]
    fn from(val: u8) -> UnitActivationStatusBf2 {
        UnitActivationStatusBf2::from_bits(val)
    }
}
impl From<UnitActivationStatusBf2> for u8 {
    #[inline(always)]
    fn from(val: UnitActivationStatusBf2) -> u8 {
        UnitActivationStatusBf2::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UnitSync0CycleTimeBf0(pub u32);
impl UnitSync0CycleTimeBf0 {
    #[doc = "Single shot mode, generate only one SYNC0 pulse."]
    pub const BF_VAL_0: Self = Self(0x0);
}
impl UnitSync0CycleTimeBf0 {
    pub const fn from_bits(val: u32) -> UnitSync0CycleTimeBf0 {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for UnitSync0CycleTimeBf0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("BF_VAL_0"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UnitSync0CycleTimeBf0 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "BF_VAL_0"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for UnitSync0CycleTimeBf0 {
    #[inline(always)]
    fn from(val: u32) -> UnitSync0CycleTimeBf0 {
        UnitSync0CycleTimeBf0::from_bits(val)
    }
}
impl From<UnitSync0CycleTimeBf0> for u32 {
    #[inline(always)]
    fn from(val: UnitSync0CycleTimeBf0) -> u32 {
        UnitSync0CycleTimeBf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WatchdogStatusProcessDataBf0 {
    #[doc = "Watchdog Process Data expired"]
    BfVal0 = 0x0,
    #[doc = "Watchdog Process Data is active or disabled"]
    BfVal1 = 0x01,
}
impl WatchdogStatusProcessDataBf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WatchdogStatusProcessDataBf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WatchdogStatusProcessDataBf0 {
    #[inline(always)]
    fn from(val: u8) -> WatchdogStatusProcessDataBf0 {
        WatchdogStatusProcessDataBf0::from_bits(val)
    }
}
impl From<WatchdogStatusProcessDataBf0> for u8 {
    #[inline(always)]
    fn from(val: WatchdogStatusProcessDataBf0) -> u8 {
        WatchdogStatusProcessDataBf0::to_bits(val)
    }
}
