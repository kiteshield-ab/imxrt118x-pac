#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EdgelockHdskCtrlPoffCntMode {
    #[doc = "Not use counter, raise Edgelock handshake done to GPC once get Edgelock ack"]
    VAL0 = 0x0,
    #[doc = "Delay after receiving Edgelock ack, delay cycle number is POFF_CNT_CFG"]
    VAL1 = 0x01,
    #[doc = "Ignore Edgelock ack, raise Edgelock handshake done to GPC when counting to POFF_CNT_CFG value"]
    VAL2 = 0x02,
    #[doc = "Time out mode, raise Edgelock handshake done to GPC when either Edgelock ack received or counting to POFF_CNT_CFG value"]
    VAL3 = 0x03,
}
impl EdgelockHdskCtrlPoffCntMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EdgelockHdskCtrlPoffCntMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EdgelockHdskCtrlPoffCntMode {
    #[inline(always)]
    fn from(val: u8) -> EdgelockHdskCtrlPoffCntMode {
        EdgelockHdskCtrlPoffCntMode::from_bits(val)
    }
}
impl From<EdgelockHdskCtrlPoffCntMode> for u8 {
    #[inline(always)]
    fn from(val: EdgelockHdskCtrlPoffCntMode) -> u8 {
        EdgelockHdskCtrlPoffCntMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EdgelockHdskCtrlPonCntMode {
    #[doc = "Not use counter, raise Edgelock handshake done to GPC once get Edgelock ack"]
    VAL0 = 0x0,
    #[doc = "Delay after receiving Edgelock ack, delay cycle number is PON_CNT_CFG"]
    VAL1 = 0x01,
    #[doc = "Ignore Edgelock ack, raise Edgelock handshake done to GPC when counting to PON_CNT_CFG value"]
    VAL2 = 0x02,
    #[doc = "Time out mode, raise Edgelock handshake done to GPC when either Edgelock ack received or counting to PON_CNT_CFG value"]
    VAL3 = 0x03,
}
impl EdgelockHdskCtrlPonCntMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EdgelockHdskCtrlPonCntMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EdgelockHdskCtrlPonCntMode {
    #[inline(always)]
    fn from(val: u8) -> EdgelockHdskCtrlPonCntMode {
        EdgelockHdskCtrlPonCntMode::from_bits(val)
    }
}
impl From<EdgelockHdskCtrlPonCntMode> for u8 {
    #[inline(always)]
    fn from(val: EdgelockHdskCtrlPonCntMode) -> u8 {
        EdgelockHdskCtrlPonCntMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EdgelockHdskSoft {
    #[doc = "Clear to 0 to trigger Edgelock handshake for power on."]
    TRIG0 = 0x0,
    #[doc = "Write to 1 to trigger Edgelock handshake for power off."]
    TRIG1 = 0x01,
}
impl EdgelockHdskSoft {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EdgelockHdskSoft {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EdgelockHdskSoft {
    #[inline(always)]
    fn from(val: u8) -> EdgelockHdskSoft {
        EdgelockHdskSoft::from_bits(val)
    }
}
impl From<EdgelockHdskSoft> for u8 {
    #[inline(always)]
    fn from(val: EdgelockHdskSoft) -> u8 {
        EdgelockHdskSoft::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IsoOnSoft {
    #[doc = "Clear to 0 to trigger isolation off"]
    TRIG0 = 0x0,
    #[doc = "Write 1 to trigger isolation on"]
    TRIG1 = 0x01,
}
impl IsoOnSoft {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IsoOnSoft {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IsoOnSoft {
    #[inline(always)]
    fn from(val: u8) -> IsoOnSoft {
        IsoOnSoft::from_bits(val)
    }
}
impl From<IsoOnSoft> for u8 {
    #[inline(always)]
    fn from(val: IsoOnSoft) -> u8 {
        IsoOnSoft::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpmSettingD0 {
    #[doc = "Not used. Do not write this value."]
    VAL0 = 0x0,
    #[doc = "power on when domain n is in RUN, off in WAIT/STOP/SUSPEND"]
    VAL1 = 0x01,
    #[doc = "power on when domain n is in RUN/WAIT, off in STOP/SUSPEND"]
    VAL2 = 0x02,
    #[doc = "power on when domain n is in RUN/WAIT/STOP, off in SUSPEND"]
    VAL3 = 0x03,
    #[doc = "power always on"]
    VALN_4 = 0x04,
    #[doc = "power always on"]
    VALN_5 = 0x05,
    #[doc = "power always on"]
    VALN_6 = 0x06,
    #[doc = "power always on"]
    VALN_7 = 0x07,
}
impl LpmSettingD0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpmSettingD0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpmSettingD0 {
    #[inline(always)]
    fn from(val: u8) -> LpmSettingD0 {
        LpmSettingD0::from_bits(val)
    }
}
impl From<LpmSettingD0> for u8 {
    #[inline(always)]
    fn from(val: LpmSettingD0) -> u8 {
        LpmSettingD0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpmSettingD1 {
    #[doc = "Not used. Do not write this value."]
    VAL0 = 0x0,
    #[doc = "power on when domain n is in RUN, off in WAIT/STOP/SUSPEND"]
    VAL1 = 0x01,
    #[doc = "power on when domain n is in RUN/WAIT, off in STOP/SUSPEND"]
    VAL2 = 0x02,
    #[doc = "power on when domain n is in RUN/WAIT/STOP, off in SUSPEND"]
    VAL3 = 0x03,
    #[doc = "power always on"]
    VALN_4 = 0x04,
    #[doc = "power always on"]
    VALN_5 = 0x05,
    #[doc = "power always on"]
    VALN_6 = 0x06,
    #[doc = "power always on"]
    VALN_7 = 0x07,
}
impl LpmSettingD1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpmSettingD1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpmSettingD1 {
    #[inline(always)]
    fn from(val: u8) -> LpmSettingD1 {
        LpmSettingD1::from_bits(val)
    }
}
impl From<LpmSettingD1> for u8 {
    #[inline(always)]
    fn from(val: LpmSettingD1) -> u8 {
        LpmSettingD1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpmSettingD10 {
    #[doc = "Not used. Do not write this value."]
    VAL0 = 0x0,
    #[doc = "power on when domain n is in RUN, off in WAIT/STOP/SUSPEND"]
    VAL1 = 0x01,
    #[doc = "power on when domain n is in RUN/WAIT, off in STOP/SUSPEND"]
    VAL2 = 0x02,
    #[doc = "power on when domain n is in RUN/WAIT/STOP, off in SUSPEND"]
    VAL3 = 0x03,
    #[doc = "power always on"]
    VALN_4 = 0x04,
    #[doc = "power always on"]
    VALN_5 = 0x05,
    #[doc = "power always on"]
    VALN_6 = 0x06,
    #[doc = "power always on"]
    VALN_7 = 0x07,
}
impl LpmSettingD10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpmSettingD10 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpmSettingD10 {
    #[inline(always)]
    fn from(val: u8) -> LpmSettingD10 {
        LpmSettingD10::from_bits(val)
    }
}
impl From<LpmSettingD10> for u8 {
    #[inline(always)]
    fn from(val: LpmSettingD10) -> u8 {
        LpmSettingD10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpmSettingD11 {
    #[doc = "Not used. Do not write this value."]
    VAL0 = 0x0,
    #[doc = "power on when domain n is in RUN, off in WAIT/STOP/SUSPEND"]
    VAL1 = 0x01,
    #[doc = "power on when domain n is in RUN/WAIT, off in STOP/SUSPEND"]
    VAL2 = 0x02,
    #[doc = "power on when domain n is in RUN/WAIT/STOP, off in SUSPEND"]
    VAL3 = 0x03,
    #[doc = "power always on"]
    VALN_4 = 0x04,
    #[doc = "power always on"]
    VALN_5 = 0x05,
    #[doc = "power always on"]
    VALN_6 = 0x06,
    #[doc = "power always on"]
    VALN_7 = 0x07,
}
impl LpmSettingD11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpmSettingD11 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpmSettingD11 {
    #[inline(always)]
    fn from(val: u8) -> LpmSettingD11 {
        LpmSettingD11::from_bits(val)
    }
}
impl From<LpmSettingD11> for u8 {
    #[inline(always)]
    fn from(val: LpmSettingD11) -> u8 {
        LpmSettingD11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpmSettingD12 {
    #[doc = "Not used. Do not write this value."]
    VAL0 = 0x0,
    #[doc = "power on when domain n is in RUN, off in WAIT/STOP/SUSPEND"]
    VAL1 = 0x01,
    #[doc = "power on when domain n is in RUN/WAIT, off in STOP/SUSPEND"]
    VAL2 = 0x02,
    #[doc = "power on when domain n is in RUN/WAIT/STOP, off in SUSPEND"]
    VAL3 = 0x03,
    #[doc = "power always on"]
    VALN_4 = 0x04,
    #[doc = "power always on"]
    VALN_5 = 0x05,
    #[doc = "power always on"]
    VALN_6 = 0x06,
    #[doc = "power always on"]
    VALN_7 = 0x07,
}
impl LpmSettingD12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpmSettingD12 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpmSettingD12 {
    #[inline(always)]
    fn from(val: u8) -> LpmSettingD12 {
        LpmSettingD12::from_bits(val)
    }
}
impl From<LpmSettingD12> for u8 {
    #[inline(always)]
    fn from(val: LpmSettingD12) -> u8 {
        LpmSettingD12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpmSettingD13 {
    #[doc = "Not used. Do not write this value."]
    VAL0 = 0x0,
    #[doc = "power on when domain n is in RUN, off in WAIT/STOP/SUSPEND"]
    VAL1 = 0x01,
    #[doc = "power on when domain n is in RUN/WAIT, off in STOP/SUSPEND"]
    VAL2 = 0x02,
    #[doc = "power on when domain n is in RUN/WAIT/STOP, off in SUSPEND"]
    VAL3 = 0x03,
    #[doc = "power always on"]
    VALN_4 = 0x04,
    #[doc = "power always on"]
    VALN_5 = 0x05,
    #[doc = "power always on"]
    VALN_6 = 0x06,
    #[doc = "power always on"]
    VALN_7 = 0x07,
}
impl LpmSettingD13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpmSettingD13 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpmSettingD13 {
    #[inline(always)]
    fn from(val: u8) -> LpmSettingD13 {
        LpmSettingD13::from_bits(val)
    }
}
impl From<LpmSettingD13> for u8 {
    #[inline(always)]
    fn from(val: LpmSettingD13) -> u8 {
        LpmSettingD13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpmSettingD14 {
    #[doc = "Not used. Do not write this value."]
    VAL0 = 0x0,
    #[doc = "power on when domain n is in RUN, off in WAIT/STOP/SUSPEND"]
    VAL1 = 0x01,
    #[doc = "power on when domain n is in RUN/WAIT, off in STOP/SUSPEND"]
    VAL2 = 0x02,
    #[doc = "power on when domain n is in RUN/WAIT/STOP, off in SUSPEND"]
    VAL3 = 0x03,
    #[doc = "power always on"]
    VALN_4 = 0x04,
    #[doc = "power always on"]
    VALN_5 = 0x05,
    #[doc = "power always on"]
    VALN_6 = 0x06,
    #[doc = "power always on"]
    VALN_7 = 0x07,
}
impl LpmSettingD14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpmSettingD14 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpmSettingD14 {
    #[inline(always)]
    fn from(val: u8) -> LpmSettingD14 {
        LpmSettingD14::from_bits(val)
    }
}
impl From<LpmSettingD14> for u8 {
    #[inline(always)]
    fn from(val: LpmSettingD14) -> u8 {
        LpmSettingD14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpmSettingD15 {
    #[doc = "Not used. Do not write this value."]
    VAL0 = 0x0,
    #[doc = "power on when domain n is in RUN, off in WAIT/STOP/SUSPEND"]
    VAL1 = 0x01,
    #[doc = "power on when domain n is in RUN/WAIT, off in STOP/SUSPEND"]
    VAL2 = 0x02,
    #[doc = "power on when domain n is in RUN/WAIT/STOP, off in SUSPEND"]
    VAL3 = 0x03,
    #[doc = "power always on"]
    VALN_4 = 0x04,
    #[doc = "power always on"]
    VALN_5 = 0x05,
    #[doc = "power always on"]
    VALN_6 = 0x06,
    #[doc = "power always on"]
    VALN_7 = 0x07,
}
impl LpmSettingD15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpmSettingD15 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpmSettingD15 {
    #[inline(always)]
    fn from(val: u8) -> LpmSettingD15 {
        LpmSettingD15::from_bits(val)
    }
}
impl From<LpmSettingD15> for u8 {
    #[inline(always)]
    fn from(val: LpmSettingD15) -> u8 {
        LpmSettingD15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpmSettingD2 {
    #[doc = "Not used. Do not write this value."]
    VAL0 = 0x0,
    #[doc = "power on when domain n is in RUN, off in WAIT/STOP/SUSPEND"]
    VAL1 = 0x01,
    #[doc = "power on when domain n is in RUN/WAIT, off in STOP/SUSPEND"]
    VAL2 = 0x02,
    #[doc = "power on when domain n is in RUN/WAIT/STOP, off in SUSPEND"]
    VAL3 = 0x03,
    #[doc = "power always on"]
    VALN_4 = 0x04,
    #[doc = "power always on"]
    VALN_5 = 0x05,
    #[doc = "power always on"]
    VALN_6 = 0x06,
    #[doc = "power always on"]
    VALN_7 = 0x07,
}
impl LpmSettingD2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpmSettingD2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpmSettingD2 {
    #[inline(always)]
    fn from(val: u8) -> LpmSettingD2 {
        LpmSettingD2::from_bits(val)
    }
}
impl From<LpmSettingD2> for u8 {
    #[inline(always)]
    fn from(val: LpmSettingD2) -> u8 {
        LpmSettingD2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpmSettingD3 {
    #[doc = "Not used. Do not write this value."]
    VAL0 = 0x0,
    #[doc = "power on when domain n is in RUN, off in WAIT/STOP/SUSPEND"]
    VAL1 = 0x01,
    #[doc = "power on when domain n is in RUN/WAIT, off in STOP/SUSPEND"]
    VAL2 = 0x02,
    #[doc = "power on when domain n is in RUN/WAIT/STOP, off in SUSPEND"]
    VAL3 = 0x03,
    #[doc = "power always on"]
    VALN_4 = 0x04,
    #[doc = "power always on"]
    VALN_5 = 0x05,
    #[doc = "power always on"]
    VALN_6 = 0x06,
    #[doc = "power always on"]
    VALN_7 = 0x07,
}
impl LpmSettingD3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpmSettingD3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpmSettingD3 {
    #[inline(always)]
    fn from(val: u8) -> LpmSettingD3 {
        LpmSettingD3::from_bits(val)
    }
}
impl From<LpmSettingD3> for u8 {
    #[inline(always)]
    fn from(val: LpmSettingD3) -> u8 {
        LpmSettingD3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpmSettingD4 {
    #[doc = "Not used. Do not write this value."]
    VAL0 = 0x0,
    #[doc = "power on when domain n is in RUN, off in WAIT/STOP/SUSPEND"]
    VAL1 = 0x01,
    #[doc = "power on when domain n is in RUN/WAIT, off in STOP/SUSPEND"]
    VAL2 = 0x02,
    #[doc = "power on when domain n is in RUN/WAIT/STOP, off in SUSPEND"]
    VAL3 = 0x03,
    #[doc = "power always on"]
    VALN_4 = 0x04,
    #[doc = "power always on"]
    VALN_5 = 0x05,
    #[doc = "power always on"]
    VALN_6 = 0x06,
    #[doc = "power always on"]
    VALN_7 = 0x07,
}
impl LpmSettingD4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpmSettingD4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpmSettingD4 {
    #[inline(always)]
    fn from(val: u8) -> LpmSettingD4 {
        LpmSettingD4::from_bits(val)
    }
}
impl From<LpmSettingD4> for u8 {
    #[inline(always)]
    fn from(val: LpmSettingD4) -> u8 {
        LpmSettingD4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpmSettingD5 {
    #[doc = "Not used. Do not write this value."]
    VAL0 = 0x0,
    #[doc = "power on when domain n is in RUN, off in WAIT/STOP/SUSPEND"]
    VAL1 = 0x01,
    #[doc = "power on when domain n is in RUN/WAIT, off in STOP/SUSPEND"]
    VAL2 = 0x02,
    #[doc = "power on when domain n is in RUN/WAIT/STOP, off in SUSPEND"]
    VAL3 = 0x03,
    #[doc = "power always on"]
    VALN_4 = 0x04,
    #[doc = "power always on"]
    VALN_5 = 0x05,
    #[doc = "power always on"]
    VALN_6 = 0x06,
    #[doc = "power always on"]
    VALN_7 = 0x07,
}
impl LpmSettingD5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpmSettingD5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpmSettingD5 {
    #[inline(always)]
    fn from(val: u8) -> LpmSettingD5 {
        LpmSettingD5::from_bits(val)
    }
}
impl From<LpmSettingD5> for u8 {
    #[inline(always)]
    fn from(val: LpmSettingD5) -> u8 {
        LpmSettingD5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpmSettingD6 {
    #[doc = "Not used. Do not write this value."]
    VAL0 = 0x0,
    #[doc = "power on when domain n is in RUN, off in WAIT/STOP/SUSPEND"]
    VAL1 = 0x01,
    #[doc = "power on when domain n is in RUN/WAIT, off in STOP/SUSPEND"]
    VAL2 = 0x02,
    #[doc = "power on when domain n is in RUN/WAIT/STOP, off in SUSPEND"]
    VAL3 = 0x03,
    #[doc = "power always on"]
    VALN_4 = 0x04,
    #[doc = "power always on"]
    VALN_5 = 0x05,
    #[doc = "power always on"]
    VALN_6 = 0x06,
    #[doc = "power always on"]
    VALN_7 = 0x07,
}
impl LpmSettingD6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpmSettingD6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpmSettingD6 {
    #[inline(always)]
    fn from(val: u8) -> LpmSettingD6 {
        LpmSettingD6::from_bits(val)
    }
}
impl From<LpmSettingD6> for u8 {
    #[inline(always)]
    fn from(val: LpmSettingD6) -> u8 {
        LpmSettingD6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpmSettingD7 {
    #[doc = "Not used. Do not write this value."]
    VAL0 = 0x0,
    #[doc = "power on when domain n is in RUN, off in WAIT/STOP/SUSPEND"]
    VAL1 = 0x01,
    #[doc = "power on when domain n is in RUN/WAIT, off in STOP/SUSPEND"]
    VAL2 = 0x02,
    #[doc = "power on when domain n is in RUN/WAIT/STOP, off in SUSPEND"]
    VAL3 = 0x03,
    #[doc = "power always on"]
    VALN_4 = 0x04,
    #[doc = "power always on"]
    VALN_5 = 0x05,
    #[doc = "power always on"]
    VALN_6 = 0x06,
    #[doc = "power always on"]
    VALN_7 = 0x07,
}
impl LpmSettingD7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpmSettingD7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpmSettingD7 {
    #[inline(always)]
    fn from(val: u8) -> LpmSettingD7 {
        LpmSettingD7::from_bits(val)
    }
}
impl From<LpmSettingD7> for u8 {
    #[inline(always)]
    fn from(val: LpmSettingD7) -> u8 {
        LpmSettingD7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpmSettingD8 {
    #[doc = "Not used. Do not write this value."]
    VAL0 = 0x0,
    #[doc = "power on when domain n is in RUN, off in WAIT/STOP/SUSPEND"]
    VAL1 = 0x01,
    #[doc = "power on when domain n is in RUN/WAIT, off in STOP/SUSPEND"]
    VAL2 = 0x02,
    #[doc = "power on when domain n is in RUN/WAIT/STOP, off in SUSPEND"]
    VAL3 = 0x03,
    #[doc = "power always on"]
    VALN_4 = 0x04,
    #[doc = "power always on"]
    VALN_5 = 0x05,
    #[doc = "power always on"]
    VALN_6 = 0x06,
    #[doc = "power always on"]
    VALN_7 = 0x07,
}
impl LpmSettingD8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpmSettingD8 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpmSettingD8 {
    #[inline(always)]
    fn from(val: u8) -> LpmSettingD8 {
        LpmSettingD8::from_bits(val)
    }
}
impl From<LpmSettingD8> for u8 {
    #[inline(always)]
    fn from(val: LpmSettingD8) -> u8 {
        LpmSettingD8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpmSettingD9 {
    #[doc = "Not used. Do not write this value."]
    VAL0 = 0x0,
    #[doc = "power on when domain n is in RUN, off in WAIT/STOP/SUSPEND"]
    VAL1 = 0x01,
    #[doc = "power on when domain n is in RUN/WAIT, off in STOP/SUSPEND"]
    VAL2 = 0x02,
    #[doc = "power on when domain n is in RUN/WAIT/STOP, off in SUSPEND"]
    VAL3 = 0x03,
    #[doc = "power always on"]
    VALN_4 = 0x04,
    #[doc = "power always on"]
    VALN_5 = 0x05,
    #[doc = "power always on"]
    VALN_6 = 0x06,
    #[doc = "power always on"]
    VALN_7 = 0x07,
}
impl LpmSettingD9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpmSettingD9 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpmSettingD9 {
    #[inline(always)]
    fn from(val: u8) -> LpmSettingD9 {
        LpmSettingD9::from_bits(val)
    }
}
impl From<LpmSettingD9> for u8 {
    #[inline(always)]
    fn from(val: LpmSettingD9) -> u8 {
        LpmSettingD9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdnSoft {
    #[doc = "Clear to 0 to trigger a power up sequence."]
    TRIG0 = 0x0,
    #[doc = "Write 1 to trigger a power down sequence."]
    TRIG1 = 0x01,
}
impl PdnSoft {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PdnSoft {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PdnSoft {
    #[inline(always)]
    fn from(val: u8) -> PdnSoft {
        PdnSoft::from_bits(val)
    }
}
impl From<PdnSoft> for u8 {
    #[inline(always)]
    fn from(val: PdnSoft) -> u8 {
        PdnSoft::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PoffBusy {
    #[doc = "Does not send Edgelock handshake for power down."]
    EN = 0x0,
    #[doc = "Sends Edgelock handshake for power down."]
    DIS = 0x01,
}
impl PoffBusy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PoffBusy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PoffBusy {
    #[inline(always)]
    fn from(val: u8) -> PoffBusy {
        PoffBusy::from_bits(val)
    }
}
impl From<PoffBusy> for u8 {
    #[inline(always)]
    fn from(val: PoffBusy) -> u8 {
        PoffBusy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PswCtrlPoffCntMode {
    #[doc = "Not use counter, raise power_on/off done to GPC once get psw ack"]
    VAL0 = 0x0,
    #[doc = "Delay after receiving psw ack, delay cycle number is POFF_CNT_CFG. When POFF_CNT_CFG value in HF/LF is different, bigger value will be used."]
    VAL1 = 0x01,
    #[doc = "Ignore psw ack, raise power_on/off done to GPC when counting to POFF_CNT_CFG value. When POFF_CNT_CFG value in HF/LF is different, bigger value will be used."]
    VAL2 = 0x02,
    #[doc = "Time out mode, raise power_on/off done to GPC when either psw ack received or counting to POFF_CNT_CFG value. When POFF_CNT_CFG value in HF/LF is different, bigger value will be used."]
    VAL3 = 0x03,
}
impl PswCtrlPoffCntMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PswCtrlPoffCntMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PswCtrlPoffCntMode {
    #[inline(always)]
    fn from(val: u8) -> PswCtrlPoffCntMode {
        PswCtrlPoffCntMode::from_bits(val)
    }
}
impl From<PswCtrlPoffCntMode> for u8 {
    #[inline(always)]
    fn from(val: PswCtrlPoffCntMode) -> u8 {
        PswCtrlPoffCntMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PswCtrlPonCntMode {
    #[doc = "Not use counter, raise power_on/off done to GPC once get psw ack"]
    VAL0 = 0x0,
    #[doc = "Delay after receiving psw ack, delay cycle number is PON_CNT_CFG"]
    VAL1 = 0x01,
    #[doc = "Ignore psw ack, raise power_on/off done to GPC when counting to PON_CNT_CFG value"]
    VAL2 = 0x02,
    #[doc = "Time out mode, raise power_on/off done to GPC when either psw ack received or counting to PON_CNT_CFG value"]
    VAL3 = 0x03,
}
impl PswCtrlPonCntMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PswCtrlPonCntMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PswCtrlPonCntMode {
    #[inline(always)]
    fn from(val: u8) -> PswCtrlPonCntMode {
        PswCtrlPonCntMode::from_bits(val)
    }
}
impl From<PswCtrlPonCntMode> for u8 {
    #[inline(always)]
    fn from(val: PswCtrlPonCntMode) -> u8 {
        PswCtrlPonCntMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PswOffSoft {
    #[doc = "Clear to 0 to trigger power switch on"]
    TRIG0 = 0x0,
    #[doc = "Write 1 to trigger power switch off"]
    TRIG1 = 0x01,
}
impl PswOffSoft {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PswOffSoft {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PswOffSoft {
    #[inline(always)]
    fn from(val: u8) -> PswOffSoft {
        PswOffSoft::from_bits(val)
    }
}
impl From<PswOffSoft> for u8 {
    #[inline(always)]
    fn from(val: PswOffSoft) -> u8 {
        PswOffSoft::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PswStat {
    #[doc = "power switch on"]
    ON = 0x0,
    #[doc = "power switch off"]
    OFF = 0x01,
}
impl PswStat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PswStat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PswStat {
    #[inline(always)]
    fn from(val: u8) -> PswStat {
        PswStat::from_bits(val)
    }
}
impl From<PswStat> for u8 {
    #[inline(always)]
    fn from(val: PswStat) -> u8 {
        PswStat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RstCtrlSoft {
    #[doc = "Clear to 0 to trigger reset deassert"]
    TRIG0 = 0x0,
    #[doc = "Write 1 to trigger reset assert"]
    TRIG1 = 0x01,
}
impl RstCtrlSoft {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RstCtrlSoft {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RstCtrlSoft {
    #[inline(always)]
    fn from(val: u8) -> RstCtrlSoft {
        RstCtrlSoft::from_bits(val)
    }
}
impl From<RstCtrlSoft> for u8 {
    #[inline(always)]
    fn from(val: RstCtrlSoft) -> u8 {
        RstCtrlSoft::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RstStat {
    #[doc = "reset asserted"]
    ASSERT = 0x0,
    #[doc = "reset released"]
    DEASSERT = 0x01,
}
impl RstStat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RstStat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RstStat {
    #[inline(always)]
    fn from(val: u8) -> RstStat {
        RstStat::from_bits(val)
    }
}
impl From<RstStat> for u8 {
    #[inline(always)]
    fn from(val: RstStat) -> u8 {
        RstStat::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct WhiteList(pub u16);
impl WhiteList {
    #[doc = "Core with domain ID=0 can write SRC MIX SLICE registers."]
    pub const DID0: Self = Self(0x01);
    #[doc = "Core with domain ID=1 can write SRC MIX SLICE registers."]
    pub const DID1: Self = Self(0x02);
    #[doc = "Core with domain ID=2 can write SRC MIX SLICE registers."]
    pub const DID2: Self = Self(0x04);
    #[doc = "Core with domain ID=3 can write SRC MIX SLICE registers."]
    pub const DID3: Self = Self(0x08);
    #[doc = "Core with domain ID=4 can write SRC MIX SLICE registers."]
    pub const DID4: Self = Self(0x10);
    #[doc = "Core with domain ID=5 can write SRC MIX SLICE registers."]
    pub const DID5: Self = Self(0x20);
    #[doc = "Core with domain ID=6 can write SRC MIX SLICE registers."]
    pub const DID6: Self = Self(0x40);
    #[doc = "Core with domain ID=7 can write SRC MIX SLICE registers."]
    pub const DID7: Self = Self(0x80);
    #[doc = "Core with domain ID=8 can write SRC MIX SLICE registers."]
    pub const DID8: Self = Self(0x0100);
    #[doc = "Core with domain ID=9 can write SRC MIX SLICE registers."]
    pub const DID9: Self = Self(0x0200);
    #[doc = "Core with domain ID=10 can write SRC MIX SLICE registers."]
    pub const DID10: Self = Self(0x0400);
    #[doc = "Core with domain ID=11 can write SRC MIX SLICE registers."]
    pub const DID11: Self = Self(0x0800);
    #[doc = "Core with domain ID=12 can write SRC MIX SLICE registers."]
    pub const DID12: Self = Self(0x1000);
    #[doc = "Core with domain ID=13 can write SRC MIX SLICE registers."]
    pub const DID13: Self = Self(0x2000);
    #[doc = "Core with domain ID=14 can write SRC MIX SLICE registers."]
    pub const DID14: Self = Self(0x4000);
    #[doc = "Core with domain ID=15 can write SRC MIX SLICE registers."]
    pub const DID15: Self = Self(0x8000);
}
impl WhiteList {
    pub const fn from_bits(val: u16) -> WhiteList {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for WhiteList {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("DID0"),
            0x02 => f.write_str("DID1"),
            0x04 => f.write_str("DID2"),
            0x08 => f.write_str("DID3"),
            0x10 => f.write_str("DID4"),
            0x20 => f.write_str("DID5"),
            0x40 => f.write_str("DID6"),
            0x80 => f.write_str("DID7"),
            0x0100 => f.write_str("DID8"),
            0x0200 => f.write_str("DID9"),
            0x0400 => f.write_str("DID10"),
            0x0800 => f.write_str("DID11"),
            0x1000 => f.write_str("DID12"),
            0x2000 => f.write_str("DID13"),
            0x4000 => f.write_str("DID14"),
            0x8000 => f.write_str("DID15"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WhiteList {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "DID0"),
            0x02 => defmt::write!(f, "DID1"),
            0x04 => defmt::write!(f, "DID2"),
            0x08 => defmt::write!(f, "DID3"),
            0x10 => defmt::write!(f, "DID4"),
            0x20 => defmt::write!(f, "DID5"),
            0x40 => defmt::write!(f, "DID6"),
            0x80 => defmt::write!(f, "DID7"),
            0x0100 => defmt::write!(f, "DID8"),
            0x0200 => defmt::write!(f, "DID9"),
            0x0400 => defmt::write!(f, "DID10"),
            0x0800 => defmt::write!(f, "DID11"),
            0x1000 => defmt::write!(f, "DID12"),
            0x2000 => defmt::write!(f, "DID13"),
            0x4000 => defmt::write!(f, "DID14"),
            0x8000 => defmt::write!(f, "DID15"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for WhiteList {
    #[inline(always)]
    fn from(val: u16) -> WhiteList {
        WhiteList::from_bits(val)
    }
}
impl From<WhiteList> for u16 {
    #[inline(always)]
    fn from(val: WhiteList) -> u16 {
        WhiteList::to_bits(val)
    }
}
