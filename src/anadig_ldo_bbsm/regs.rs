#[doc = "PMU_LDO_AON_ANA_REGISTER"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PmuLdoAonAna(pub u32);
impl PmuLdoAonAna {
    #[doc = "reg_lp_en"]
    #[must_use]
    #[inline(always)]
    pub const fn reg_lp_en(&self) -> super::vals::RegLpEn {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RegLpEn::from_bits(val as u8)
    }
    #[doc = "reg_lp_en"]
    #[inline(always)]
    pub const fn set_reg_lp_en(&mut self, val: super::vals::RegLpEn) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "reg_disable"]
    #[must_use]
    #[inline(always)]
    pub const fn reg_disable(&self) -> super::vals::RegDisable {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::RegDisable::from_bits(val as u8)
    }
    #[doc = "reg_disable"]
    #[inline(always)]
    pub const fn set_reg_disable(&mut self, val: super::vals::RegDisable) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "pull_down_2ma_en"]
    #[must_use]
    #[inline(always)]
    pub const fn pull_down_2ma_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "pull_down_2ma_en"]
    #[inline(always)]
    pub const fn set_pull_down_2ma_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "standby_en"]
    #[must_use]
    #[inline(always)]
    pub const fn standby_en(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "standby_en"]
    #[inline(always)]
    pub const fn set_standby_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "always_4ma_pulldown_en"]
    #[must_use]
    #[inline(always)]
    pub const fn always_4ma_pulldown_en(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "always_4ma_pulldown_en"]
    #[inline(always)]
    pub const fn set_always_4ma_pulldown_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Track Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn track_mode_en(&self) -> super::vals::TrackModeEn {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::TrackModeEn::from_bits(val as u8)
    }
    #[doc = "Track Mode Enable"]
    #[inline(always)]
    pub const fn set_track_mode_en(&mut self, val: super::vals::TrackModeEn) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "pull_down_20ua_en"]
    #[must_use]
    #[inline(always)]
    pub const fn pull_down_20ua_en(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "pull_down_20ua_en"]
    #[inline(always)]
    pub const fn set_pull_down_20ua_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for PmuLdoAonAna {
    #[inline(always)]
    fn default() -> PmuLdoAonAna {
        PmuLdoAonAna(264u64 as u32)
    }
}
impl core::fmt::Debug for PmuLdoAonAna {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PmuLdoAonAna")
            .field("reg_lp_en", &self.reg_lp_en())
            .field("reg_disable", &self.reg_disable())
            .field("pull_down_2ma_en", &self.pull_down_2ma_en())
            .field("standby_en", &self.standby_en())
            .field("always_4ma_pulldown_en", &self.always_4ma_pulldown_en())
            .field("track_mode_en", &self.track_mode_en())
            .field("pull_down_20ua_en", &self.pull_down_20ua_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PmuLdoAonAna {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PmuLdoAonAna {
            reg_lp_en: super::vals::RegLpEn,
            reg_disable: super::vals::RegDisable,
            pull_down_2ma_en: bool,
            standby_en: bool,
            always_4ma_pulldown_en: bool,
            track_mode_en: super::vals::TrackModeEn,
            pull_down_20ua_en: bool,
        }
        let proxy = PmuLdoAonAna {
            reg_lp_en: self.reg_lp_en(),
            reg_disable: self.reg_disable(),
            pull_down_2ma_en: self.pull_down_2ma_en(),
            standby_en: self.standby_en(),
            always_4ma_pulldown_en: self.always_4ma_pulldown_en(),
            track_mode_en: self.track_mode_en(),
            pull_down_20ua_en: self.pull_down_20ua_en(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PMU_LDO_AON_DIG_REGISTER"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PmuLdoAonDig(pub u32);
impl PmuLdoAonDig {
    #[doc = "ENABLE_ILIMIT"]
    #[must_use]
    #[inline(always)]
    pub const fn reg_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "ENABLE_ILIMIT"]
    #[inline(always)]
    pub const fn set_reg_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "standby_en"]
    #[must_use]
    #[inline(always)]
    pub const fn standby_en(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "standby_en"]
    #[inline(always)]
    pub const fn set_standby_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "VOLTAGE_SELECT"]
    #[must_use]
    #[inline(always)]
    pub const fn voltage_select(&self) -> super::vals::VoltageSelect {
        let val = (self.0 >> 20usize) & 0x1f;
        super::vals::VoltageSelect::from_bits(val as u8)
    }
    #[doc = "VOLTAGE_SELECT"]
    #[inline(always)]
    pub const fn set_voltage_select(&mut self, val: super::vals::VoltageSelect) {
        self.0 = (self.0 & !(0x1f << 20usize)) | (((val.to_bits() as u32) & 0x1f) << 20usize);
    }
}
impl Default for PmuLdoAonDig {
    #[inline(always)]
    fn default() -> PmuLdoAonDig {
        PmuLdoAonDig(19930117u64 as u32)
    }
}
impl core::fmt::Debug for PmuLdoAonDig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PmuLdoAonDig")
            .field("reg_en", &self.reg_en())
            .field("standby_en", &self.standby_en())
            .field("voltage_select", &self.voltage_select())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PmuLdoAonDig {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PmuLdoAonDig {
            reg_en: bool,
            standby_en: bool,
            voltage_select: super::vals::VoltageSelect,
        }
        let proxy = PmuLdoAonDig {
            reg_en: self.reg_en(),
            standby_en: self.standby_en(),
            voltage_select: self.voltage_select(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
