#[doc = "BBNSM Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BbnsmCtrl(pub u32);
impl BbnsmCtrl {
    #[doc = "Real-Time Counter Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rtc_en(&self) -> super::vals::RtcEn {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::RtcEn::from_bits(val as u8)
    }
    #[doc = "Real-Time Counter Enable"]
    #[inline(always)]
    pub const fn set_rtc_en(&mut self, val: super::vals::RtcEn) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Time Alarm Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ta_en(&self) -> super::vals::TaEn {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::TaEn::from_bits(val as u8)
    }
    #[doc = "Time Alarm Enable"]
    #[inline(always)]
    pub const fn set_ta_en(&mut self, val: super::vals::TaEn) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Calibration Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cal_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Calibration Enable"]
    #[inline(always)]
    pub const fn set_cal_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Calibration Value"]
    #[must_use]
    #[inline(always)]
    pub const fn cal_val(&self) -> super::vals::CalVal {
        let val = (self.0 >> 8usize) & 0x1f;
        super::vals::CalVal::from_bits(val as u8)
    }
    #[doc = "Calibration Value"]
    #[inline(always)]
    pub const fn set_cal_val(&mut self, val: super::vals::CalVal) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
    }
    #[doc = "Button Press Timeout"]
    #[must_use]
    #[inline(always)]
    pub const fn btn_timeout(&self) -> super::vals::BtnTimeout {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::BtnTimeout::from_bits(val as u8)
    }
    #[doc = "Button Press Timeout"]
    #[inline(always)]
    pub const fn set_btn_timeout(&mut self, val: super::vals::BtnTimeout) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Debounce Time"]
    #[must_use]
    #[inline(always)]
    pub const fn debounce(&self) -> super::vals::Debounce {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::Debounce::from_bits(val as u8)
    }
    #[doc = "Debounce Time"]
    #[inline(always)]
    pub const fn set_debounce(&mut self, val: super::vals::Debounce) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Turn-On Time"]
    #[must_use]
    #[inline(always)]
    pub const fn turn_on_time(&self) -> super::vals::TurnOnTime {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::TurnOnTime::from_bits(val as u8)
    }
    #[doc = "Turn-On Time"]
    #[inline(always)]
    pub const fn set_turn_on_time(&mut self, val: super::vals::TurnOnTime) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "PMIC On Request Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pk_en(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "PMIC On Request Enable"]
    #[inline(always)]
    pub const fn set_pk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "PMIC On Request Override"]
    #[must_use]
    #[inline(always)]
    pub const fn pk_ovr(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "PMIC On Request Override"]
    #[inline(always)]
    pub const fn set_pk_ovr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Dumb PMIC Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dp_en(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Dumb PMIC Enable"]
    #[inline(always)]
    pub const fn set_dp_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Turn Off System Power"]
    #[must_use]
    #[inline(always)]
    pub const fn tosp(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Turn Off System Power"]
    #[inline(always)]
    pub const fn set_tosp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for BbnsmCtrl {
    #[inline(always)]
    fn default() -> BbnsmCtrl {
        BbnsmCtrl(16777221u64 as u32)
    }
}
impl core::fmt::Debug for BbnsmCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BbnsmCtrl")
            .field("rtc_en", &self.rtc_en())
            .field("ta_en", &self.ta_en())
            .field("cal_en", &self.cal_en())
            .field("cal_val", &self.cal_val())
            .field("btn_timeout", &self.btn_timeout())
            .field("debounce", &self.debounce())
            .field("turn_on_time", &self.turn_on_time())
            .field("pk_en", &self.pk_en())
            .field("pk_ovr", &self.pk_ovr())
            .field("dp_en", &self.dp_en())
            .field("tosp", &self.tosp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BbnsmCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct BbnsmCtrl {
            rtc_en: super::vals::RtcEn,
            ta_en: super::vals::TaEn,
            cal_en: bool,
            cal_val: super::vals::CalVal,
            btn_timeout: super::vals::BtnTimeout,
            debounce: super::vals::Debounce,
            turn_on_time: super::vals::TurnOnTime,
            pk_en: bool,
            pk_ovr: bool,
            dp_en: bool,
            tosp: bool,
        }
        let proxy = BbnsmCtrl {
            rtc_en: self.rtc_en(),
            ta_en: self.ta_en(),
            cal_en: self.cal_en(),
            cal_val: self.cal_val(),
            btn_timeout: self.btn_timeout(),
            debounce: self.debounce(),
            turn_on_time: self.turn_on_time(),
            pk_en: self.pk_en(),
            pk_ovr: self.pk_ovr(),
            dp_en: self.dp_en(),
            tosp: self.tosp(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "BBNSM Events Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BbnsmEvents(pub u32);
impl BbnsmEvents {
    #[doc = "Real-Time Counter Rollover Event"]
    #[must_use]
    #[inline(always)]
    pub const fn rtc_roll(&self) -> super::vals::RtcRoll {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::RtcRoll::from_bits(val as u8)
    }
    #[doc = "Real-Time Counter Rollover Event"]
    #[inline(always)]
    pub const fn set_rtc_roll(&mut self, val: super::vals::RtcRoll) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Time Alarm Event"]
    #[must_use]
    #[inline(always)]
    pub const fn ta(&self) -> super::vals::Ta {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Ta::from_bits(val as u8)
    }
    #[doc = "Time Alarm Event"]
    #[inline(always)]
    pub const fn set_ta(&mut self, val: super::vals::Ta) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Emergency Off Event"]
    #[must_use]
    #[inline(always)]
    pub const fn emg_off(&self) -> super::vals::EmgOff {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::EmgOff::from_bits(val as u8)
    }
    #[doc = "Emergency Off Event"]
    #[inline(always)]
    pub const fn set_emg_off(&mut self, val: super::vals::EmgOff) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Set Power Off Event"]
    #[must_use]
    #[inline(always)]
    pub const fn pwr_off(&self) -> super::vals::PwrOff {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::PwrOff::from_bits(val as u8)
    }
    #[doc = "Set Power Off Event"]
    #[inline(always)]
    pub const fn set_pwr_off(&mut self, val: super::vals::PwrOff) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Set Power On Event"]
    #[must_use]
    #[inline(always)]
    pub const fn pwr_on(&self) -> super::vals::PwrOn {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PwrOn::from_bits(val as u8)
    }
    #[doc = "Set Power On Event"]
    #[inline(always)]
    pub const fn set_pwr_on(&mut self, val: super::vals::PwrOn) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
}
impl Default for BbnsmEvents {
    #[inline(always)]
    fn default() -> BbnsmEvents {
        BbnsmEvents(5u64 as u32)
    }
}
impl core::fmt::Debug for BbnsmEvents {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BbnsmEvents")
            .field("rtc_roll", &self.rtc_roll())
            .field("ta", &self.ta())
            .field("emg_off", &self.emg_off())
            .field("pwr_off", &self.pwr_off())
            .field("pwr_on", &self.pwr_on())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BbnsmEvents {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct BbnsmEvents {
            rtc_roll: super::vals::RtcRoll,
            ta: super::vals::Ta,
            emg_off: super::vals::EmgOff,
            pwr_off: super::vals::PwrOff,
            pwr_on: super::vals::PwrOn,
        }
        let proxy = BbnsmEvents {
            rtc_roll: self.rtc_roll(),
            ta: self.ta(),
            emg_off: self.emg_off(),
            pwr_off: self.pwr_off(),
            pwr_on: self.pwr_on(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "BBNSM Features Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BbnsmFeatures(pub u32);
impl BbnsmFeatures {
    #[doc = "GPR Register Array Size"]
    #[must_use]
    #[inline(always)]
    pub const fn gpr_sz(&self) -> super::vals::GprSz {
        let val = (self.0 >> 2usize) & 0x3f;
        super::vals::GprSz::from_bits(val as u8)
    }
    #[doc = "GPR Register Array Size"]
    #[inline(always)]
    pub const fn set_gpr_sz(&mut self, val: super::vals::GprSz) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val.to_bits() as u32) & 0x3f) << 2usize);
    }
}
impl Default for BbnsmFeatures {
    #[inline(always)]
    fn default() -> BbnsmFeatures {
        BbnsmFeatures(32u64 as u32)
    }
}
impl core::fmt::Debug for BbnsmFeatures {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BbnsmFeatures")
            .field("gpr_sz", &self.gpr_sz())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BbnsmFeatures {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct BbnsmFeatures {
            gpr_sz: super::vals::GprSz,
        }
        let proxy = BbnsmFeatures {
            gpr_sz: self.gpr_sz(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "BBNSM Interrupt Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BbnsmIntEn(pub u32);
impl BbnsmIntEn {
    #[doc = "Real-Time Counter Rollover Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rtc_int_en(&self) -> super::vals::RtcIntEn {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::RtcIntEn::from_bits(val as u8)
    }
    #[doc = "Real-Time Counter Rollover Interrupt Enable"]
    #[inline(always)]
    pub const fn set_rtc_int_en(&mut self, val: super::vals::RtcIntEn) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Time Alarm Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ta_int_en(&self) -> super::vals::TaIntEn {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::TaIntEn::from_bits(val as u8)
    }
    #[doc = "Time Alarm Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ta_int_en(&mut self, val: super::vals::TaIntEn) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for BbnsmIntEn {
    #[inline(always)]
    fn default() -> BbnsmIntEn {
        BbnsmIntEn(5u64 as u32)
    }
}
impl core::fmt::Debug for BbnsmIntEn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BbnsmIntEn")
            .field("rtc_int_en", &self.rtc_int_en())
            .field("ta_int_en", &self.ta_int_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BbnsmIntEn {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct BbnsmIntEn {
            rtc_int_en: super::vals::RtcIntEn,
            ta_int_en: super::vals::TaIntEn,
        }
        let proxy = BbnsmIntEn {
            rtc_int_en: self.rtc_int_en(),
            ta_int_en: self.ta_int_en(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "BBNSM External Pad Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BbnsmPadCtrl(pub u32);
impl BbnsmPadCtrl {
    #[doc = "Control I/O Pads"]
    #[must_use]
    #[inline(always)]
    pub const fn pad_ctrl0(&self) -> super::vals::PadCtrl0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::PadCtrl0::from_bits(val as u8)
    }
    #[doc = "Control I/O Pads"]
    #[inline(always)]
    pub const fn set_pad_ctrl0(&mut self, val: super::vals::PadCtrl0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Control I/O Pads"]
    #[must_use]
    #[inline(always)]
    pub const fn pad_ctrl1(&self) -> super::vals::PadCtrl1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::PadCtrl1::from_bits(val as u8)
    }
    #[doc = "Control I/O Pads"]
    #[inline(always)]
    pub const fn set_pad_ctrl1(&mut self, val: super::vals::PadCtrl1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Control I/O Pads"]
    #[must_use]
    #[inline(always)]
    pub const fn pad_ctrl2(&self) -> super::vals::PadCtrl2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::PadCtrl2::from_bits(val as u8)
    }
    #[doc = "Control I/O Pads"]
    #[inline(always)]
    pub const fn set_pad_ctrl2(&mut self, val: super::vals::PadCtrl2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Control I/O Pads"]
    #[must_use]
    #[inline(always)]
    pub const fn pad_ctrl3(&self) -> super::vals::PadCtrl3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::PadCtrl3::from_bits(val as u8)
    }
    #[doc = "Control I/O Pads"]
    #[inline(always)]
    pub const fn set_pad_ctrl3(&mut self, val: super::vals::PadCtrl3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Control I/O Pads"]
    #[must_use]
    #[inline(always)]
    pub const fn pad_ctrl4(&self) -> super::vals::PadCtrl4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::PadCtrl4::from_bits(val as u8)
    }
    #[doc = "Control I/O Pads"]
    #[inline(always)]
    pub const fn set_pad_ctrl4(&mut self, val: super::vals::PadCtrl4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Control I/O Pads"]
    #[must_use]
    #[inline(always)]
    pub const fn pad_ctrl5(&self) -> super::vals::PadCtrl5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::PadCtrl5::from_bits(val as u8)
    }
    #[doc = "Control I/O Pads"]
    #[inline(always)]
    pub const fn set_pad_ctrl5(&mut self, val: super::vals::PadCtrl5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Control I/O Pads"]
    #[must_use]
    #[inline(always)]
    pub const fn pad_ctrl6(&self) -> super::vals::PadCtrl6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PadCtrl6::from_bits(val as u8)
    }
    #[doc = "Control I/O Pads"]
    #[inline(always)]
    pub const fn set_pad_ctrl6(&mut self, val: super::vals::PadCtrl6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Control I/O Pads"]
    #[must_use]
    #[inline(always)]
    pub const fn pad_ctrl7(&self) -> super::vals::PadCtrl7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::PadCtrl7::from_bits(val as u8)
    }
    #[doc = "Control I/O Pads"]
    #[inline(always)]
    pub const fn set_pad_ctrl7(&mut self, val: super::vals::PadCtrl7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Control I/O Pads"]
    #[must_use]
    #[inline(always)]
    pub const fn pad_ctrl8(&self) -> super::vals::PadCtrl8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PadCtrl8::from_bits(val as u8)
    }
    #[doc = "Control I/O Pads"]
    #[inline(always)]
    pub const fn set_pad_ctrl8(&mut self, val: super::vals::PadCtrl8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Control I/O Pads"]
    #[must_use]
    #[inline(always)]
    pub const fn pad_ctrl9(&self) -> super::vals::PadCtrl9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PadCtrl9::from_bits(val as u8)
    }
    #[doc = "Control I/O Pads"]
    #[inline(always)]
    pub const fn set_pad_ctrl9(&mut self, val: super::vals::PadCtrl9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Control I/O Pads"]
    #[must_use]
    #[inline(always)]
    pub const fn pad_ctrl10(&self) -> super::vals::PadCtrl10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::PadCtrl10::from_bits(val as u8)
    }
    #[doc = "Control I/O Pads"]
    #[inline(always)]
    pub const fn set_pad_ctrl10(&mut self, val: super::vals::PadCtrl10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Control I/O Pads"]
    #[must_use]
    #[inline(always)]
    pub const fn pad_ctrl11(&self) -> super::vals::PadCtrl11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::PadCtrl11::from_bits(val as u8)
    }
    #[doc = "Control I/O Pads"]
    #[inline(always)]
    pub const fn set_pad_ctrl11(&mut self, val: super::vals::PadCtrl11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Control I/O Pads"]
    #[must_use]
    #[inline(always)]
    pub const fn pad_ctrl12(&self) -> super::vals::PadCtrl12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::PadCtrl12::from_bits(val as u8)
    }
    #[doc = "Control I/O Pads"]
    #[inline(always)]
    pub const fn set_pad_ctrl12(&mut self, val: super::vals::PadCtrl12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Control I/O Pads"]
    #[must_use]
    #[inline(always)]
    pub const fn pad_ctrl13(&self) -> super::vals::PadCtrl13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::PadCtrl13::from_bits(val as u8)
    }
    #[doc = "Control I/O Pads"]
    #[inline(always)]
    pub const fn set_pad_ctrl13(&mut self, val: super::vals::PadCtrl13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Control I/O Pads"]
    #[must_use]
    #[inline(always)]
    pub const fn pad_ctrl14(&self) -> super::vals::PadCtrl14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::PadCtrl14::from_bits(val as u8)
    }
    #[doc = "Control I/O Pads"]
    #[inline(always)]
    pub const fn set_pad_ctrl14(&mut self, val: super::vals::PadCtrl14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Control I/O Pads"]
    #[must_use]
    #[inline(always)]
    pub const fn pad_ctrl15(&self) -> super::vals::PadCtrl15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::PadCtrl15::from_bits(val as u8)
    }
    #[doc = "Control I/O Pads"]
    #[inline(always)]
    pub const fn set_pad_ctrl15(&mut self, val: super::vals::PadCtrl15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for BbnsmPadCtrl {
    #[inline(always)]
    fn default() -> BbnsmPadCtrl {
        BbnsmPadCtrl(0u64 as u32)
    }
}
impl core::fmt::Debug for BbnsmPadCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BbnsmPadCtrl")
            .field("pad_ctrl0", &self.pad_ctrl0())
            .field("pad_ctrl1", &self.pad_ctrl1())
            .field("pad_ctrl2", &self.pad_ctrl2())
            .field("pad_ctrl3", &self.pad_ctrl3())
            .field("pad_ctrl4", &self.pad_ctrl4())
            .field("pad_ctrl5", &self.pad_ctrl5())
            .field("pad_ctrl6", &self.pad_ctrl6())
            .field("pad_ctrl7", &self.pad_ctrl7())
            .field("pad_ctrl8", &self.pad_ctrl8())
            .field("pad_ctrl9", &self.pad_ctrl9())
            .field("pad_ctrl10", &self.pad_ctrl10())
            .field("pad_ctrl11", &self.pad_ctrl11())
            .field("pad_ctrl12", &self.pad_ctrl12())
            .field("pad_ctrl13", &self.pad_ctrl13())
            .field("pad_ctrl14", &self.pad_ctrl14())
            .field("pad_ctrl15", &self.pad_ctrl15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BbnsmPadCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct BbnsmPadCtrl {
            pad_ctrl0: super::vals::PadCtrl0,
            pad_ctrl1: super::vals::PadCtrl1,
            pad_ctrl2: super::vals::PadCtrl2,
            pad_ctrl3: super::vals::PadCtrl3,
            pad_ctrl4: super::vals::PadCtrl4,
            pad_ctrl5: super::vals::PadCtrl5,
            pad_ctrl6: super::vals::PadCtrl6,
            pad_ctrl7: super::vals::PadCtrl7,
            pad_ctrl8: super::vals::PadCtrl8,
            pad_ctrl9: super::vals::PadCtrl9,
            pad_ctrl10: super::vals::PadCtrl10,
            pad_ctrl11: super::vals::PadCtrl11,
            pad_ctrl12: super::vals::PadCtrl12,
            pad_ctrl13: super::vals::PadCtrl13,
            pad_ctrl14: super::vals::PadCtrl14,
            pad_ctrl15: super::vals::PadCtrl15,
        }
        let proxy = BbnsmPadCtrl {
            pad_ctrl0: self.pad_ctrl0(),
            pad_ctrl1: self.pad_ctrl1(),
            pad_ctrl2: self.pad_ctrl2(),
            pad_ctrl3: self.pad_ctrl3(),
            pad_ctrl4: self.pad_ctrl4(),
            pad_ctrl5: self.pad_ctrl5(),
            pad_ctrl6: self.pad_ctrl6(),
            pad_ctrl7: self.pad_ctrl7(),
            pad_ctrl8: self.pad_ctrl8(),
            pad_ctrl9: self.pad_ctrl9(),
            pad_ctrl10: self.pad_ctrl10(),
            pad_ctrl11: self.pad_ctrl11(),
            pad_ctrl12: self.pad_ctrl12(),
            pad_ctrl13: self.pad_ctrl13(),
            pad_ctrl14: self.pad_ctrl14(),
            pad_ctrl15: self.pad_ctrl15(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "BBNSM Real-Time Counter MS Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BbnsmRtcMs(pub u32);
impl BbnsmRtcMs {
    #[doc = "Real-Time Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn rtc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Real-Time Counter"]
    #[inline(always)]
    pub const fn set_rtc(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
}
impl Default for BbnsmRtcMs {
    #[inline(always)]
    fn default() -> BbnsmRtcMs {
        BbnsmRtcMs(0u64 as u32)
    }
}
impl core::fmt::Debug for BbnsmRtcMs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BbnsmRtcMs")
            .field("rtc", &self.rtc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BbnsmRtcMs {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct BbnsmRtcMs {
            rtc: u16,
        }
        let proxy = BbnsmRtcMs { rtc: self.rtc() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "BBNSM Version ID Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BbnsmVid(pub u32);
impl BbnsmVid {
    #[doc = "BBNSM IP ID"]
    #[must_use]
    #[inline(always)]
    pub const fn bbnsm_ipid(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "BBNSM IP ID"]
    #[inline(always)]
    pub const fn set_bbnsm_ipid(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "BBNSM Revision"]
    #[must_use]
    #[inline(always)]
    pub const fn bbnsm_rev(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "BBNSM Revision"]
    #[inline(always)]
    pub const fn set_bbnsm_rev(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "BBNSM Version ID"]
    #[must_use]
    #[inline(always)]
    pub const fn bbnsm_vid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "BBNSM Version ID"]
    #[inline(always)]
    pub const fn set_bbnsm_vid(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for BbnsmVid {
    #[inline(always)]
    fn default() -> BbnsmVid {
        BbnsmVid(65854u64 as u32)
    }
}
impl core::fmt::Debug for BbnsmVid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BbnsmVid")
            .field("bbnsm_ipid", &self.bbnsm_ipid())
            .field("bbnsm_rev", &self.bbnsm_rev())
            .field("bbnsm_vid", &self.bbnsm_vid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BbnsmVid {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct BbnsmVid {
            bbnsm_ipid: u8,
            bbnsm_rev: u8,
            bbnsm_vid: u8,
        }
        let proxy = BbnsmVid {
            bbnsm_ipid: self.bbnsm_ipid(),
            bbnsm_rev: self.bbnsm_rev(),
            bbnsm_vid: self.bbnsm_vid(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
