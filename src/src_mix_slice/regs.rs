#[doc = "Authentication Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AuthenCtrl(pub u32);
impl AuthenCtrl {
    #[doc = "HW low power mode"]
    #[must_use]
    #[inline(always)]
    pub const fn lpm_mode(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "HW low power mode"]
    #[inline(always)]
    pub const fn set_lpm_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Configuration lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_cfg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Configuration lock"]
    #[inline(always)]
    pub const fn set_lock_cfg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Allow user mode write"]
    #[must_use]
    #[inline(always)]
    pub const fn tz_user(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Allow user mode write"]
    #[inline(always)]
    pub const fn set_tz_user(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Allow non-secure mode access"]
    #[must_use]
    #[inline(always)]
    pub const fn tz_ns(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Allow non-secure mode access"]
    #[inline(always)]
    pub const fn set_tz_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Lock TZ_NS and TZ_USER"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_tz(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Lock TZ_NS and TZ_USER"]
    #[inline(always)]
    pub const fn set_lock_tz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "White list lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_list(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "White list lock"]
    #[inline(always)]
    pub const fn set_lock_list(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Domain ID white list"]
    #[must_use]
    #[inline(always)]
    pub const fn white_list(&self) -> super::vals::WhiteList {
        let val = (self.0 >> 16usize) & 0xffff;
        super::vals::WhiteList::from_bits(val as u16)
    }
    #[doc = "Domain ID white list"]
    #[inline(always)]
    pub const fn set_white_list(&mut self, val: super::vals::WhiteList) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val.to_bits() as u32) & 0xffff) << 16usize);
    }
}
impl Default for AuthenCtrl {
    #[inline(always)]
    fn default() -> AuthenCtrl {
        AuthenCtrl(4294901760u64 as u32)
    }
}
impl core::fmt::Debug for AuthenCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AuthenCtrl")
            .field("lpm_mode", &self.lpm_mode())
            .field("lock_cfg", &self.lock_cfg())
            .field("tz_user", &self.tz_user())
            .field("tz_ns", &self.tz_ns())
            .field("lock_tz", &self.lock_tz())
            .field("lock_list", &self.lock_list())
            .field("white_list", &self.white_list())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AuthenCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct AuthenCtrl {
            lpm_mode: bool,
            lock_cfg: bool,
            tz_user: bool,
            tz_ns: bool,
            lock_tz: bool,
            lock_list: bool,
            white_list: super::vals::WhiteList,
        }
        let proxy = AuthenCtrl {
            lpm_mode: self.lpm_mode(),
            lock_cfg: self.lock_cfg(),
            tz_user: self.tz_user(),
            tz_ns: self.tz_ns(),
            lock_tz: self.lock_tz(),
            lock_list: self.lock_list(),
            white_list: self.white_list(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Edgelock Handshake Counter Config"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EdgelockHdskCntCfg(pub u32);
impl EdgelockHdskCntCfg {
    #[doc = "Edgelock handshake ack count for power up config: usage depends on CNT_MODE, locked by LOCK_CFG field"]
    #[must_use]
    #[inline(always)]
    pub const fn pon_cnt_cfg(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Edgelock handshake ack count for power up config: usage depends on CNT_MODE, locked by LOCK_CFG field"]
    #[inline(always)]
    pub const fn set_pon_cnt_cfg(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Edgelock handshake ack count for power down config: usage depends on CNT_MODE, locked by LOCK_CFG field"]
    #[must_use]
    #[inline(always)]
    pub const fn poff_cnt_cfg(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Edgelock handshake ack count for power down config: usage depends on CNT_MODE, locked by LOCK_CFG field"]
    #[inline(always)]
    pub const fn set_poff_cnt_cfg(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for EdgelockHdskCntCfg {
    #[inline(always)]
    fn default() -> EdgelockHdskCntCfg {
        EdgelockHdskCntCfg(786444000u64 as u32)
    }
}
impl core::fmt::Debug for EdgelockHdskCntCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EdgelockHdskCntCfg")
            .field("pon_cnt_cfg", &self.pon_cnt_cfg())
            .field("poff_cnt_cfg", &self.poff_cnt_cfg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EdgelockHdskCntCfg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EdgelockHdskCntCfg {
            pon_cnt_cfg: u16,
            poff_cnt_cfg: u16,
        }
        let proxy = EdgelockHdskCntCfg {
            pon_cnt_cfg: self.pon_cnt_cfg(),
            poff_cnt_cfg: self.poff_cnt_cfg(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Edgelock Handshake Counter Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EdgelockHdskCntStat(pub u32);
impl EdgelockHdskCntStat {
    #[doc = "Edgelock handshake for power up acknowledge count, record the delay from stat change to acknowledge received"]
    #[must_use]
    #[inline(always)]
    pub const fn pon_cnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Edgelock handshake for power up acknowledge count, record the delay from stat change to acknowledge received"]
    #[inline(always)]
    pub const fn set_pon_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Edgelock handshake for power down acknowledge count, record the delay from stat change to acknowledge received"]
    #[must_use]
    #[inline(always)]
    pub const fn poff_cnt(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Edgelock handshake for power down acknowledge count, record the delay from stat change to acknowledge received"]
    #[inline(always)]
    pub const fn set_poff_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for EdgelockHdskCntStat {
    #[inline(always)]
    fn default() -> EdgelockHdskCntStat {
        EdgelockHdskCntStat(0u64 as u32)
    }
}
impl core::fmt::Debug for EdgelockHdskCntStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EdgelockHdskCntStat")
            .field("pon_cnt", &self.pon_cnt())
            .field("poff_cnt", &self.poff_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EdgelockHdskCntStat {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EdgelockHdskCntStat {
            pon_cnt: u16,
            poff_cnt: u16,
        }
        let proxy = EdgelockHdskCntStat {
            pon_cnt: self.pon_cnt(),
            poff_cnt: self.poff_cnt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Edgelock Handshake Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EdgelockHdskCtrl(pub u32);
impl EdgelockHdskCtrl {
    #[doc = "Configures the acknowledge counter for power up working mode, locked by LOCK_CFG field"]
    #[must_use]
    #[inline(always)]
    pub const fn pon_cnt_mode(&self) -> super::vals::EdgelockHdskCtrlPonCntMode {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::EdgelockHdskCtrlPonCntMode::from_bits(val as u8)
    }
    #[doc = "Configures the acknowledge counter for power up working mode, locked by LOCK_CFG field"]
    #[inline(always)]
    pub const fn set_pon_cnt_mode(&mut self, val: super::vals::EdgelockHdskCtrlPonCntMode) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Configures the acknowledge counter for power down working mode, locked by LOCK_CFG field"]
    #[must_use]
    #[inline(always)]
    pub const fn poff_cnt_mode(&self) -> super::vals::EdgelockHdskCtrlPoffCntMode {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::EdgelockHdskCtrlPoffCntMode::from_bits(val as u8)
    }
    #[doc = "Configures the acknowledge counter for power down working mode, locked by LOCK_CFG field"]
    #[inline(always)]
    pub const fn set_poff_cnt_mode(&mut self, val: super::vals::EdgelockHdskCtrlPoffCntMode) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for EdgelockHdskCtrl {
    #[inline(always)]
    fn default() -> EdgelockHdskCtrl {
        EdgelockHdskCtrl(2147516416u64 as u32)
    }
}
impl core::fmt::Debug for EdgelockHdskCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EdgelockHdskCtrl")
            .field("pon_cnt_mode", &self.pon_cnt_mode())
            .field("poff_cnt_mode", &self.poff_cnt_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EdgelockHdskCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EdgelockHdskCtrl {
            pon_cnt_mode: super::vals::EdgelockHdskCtrlPonCntMode,
            poff_cnt_mode: super::vals::EdgelockHdskCtrlPoffCntMode,
        }
        let proxy = EdgelockHdskCtrl {
            pon_cnt_mode: self.pon_cnt_mode(),
            poff_cnt_mode: self.poff_cnt_mode(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Edgelock Handshake Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EdgelockHdskStat(pub u32);
impl EdgelockHdskStat {
    #[doc = "Indicates this mix power up sequence has accepted Edgelock ack"]
    #[must_use]
    #[inline(always)]
    pub const fn pon_ack(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates this mix power up sequence has accepted Edgelock ack"]
    #[inline(always)]
    pub const fn set_pon_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Busy requesting Edgelock handshake for power up"]
    #[must_use]
    #[inline(always)]
    pub const fn pon_busy(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Busy requesting Edgelock handshake for power up"]
    #[inline(always)]
    pub const fn set_pon_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Indicates this mix power down sequence has accepted Edgelock ack"]
    #[must_use]
    #[inline(always)]
    pub const fn poff_ack(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates this mix power down sequence has accepted Edgelock ack"]
    #[inline(always)]
    pub const fn set_poff_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Busy requesting Edgelock handshake for power down"]
    #[must_use]
    #[inline(always)]
    pub const fn poff_busy(&self) -> super::vals::PoffBusy {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::PoffBusy::from_bits(val as u8)
    }
    #[doc = "Busy requesting Edgelock handshake for power down"]
    #[inline(always)]
    pub const fn set_poff_busy(&mut self, val: super::vals::PoffBusy) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
}
impl Default for EdgelockHdskStat {
    #[inline(always)]
    fn default() -> EdgelockHdskStat {
        EdgelockHdskStat(1u64 as u32)
    }
}
impl core::fmt::Debug for EdgelockHdskStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EdgelockHdskStat")
            .field("pon_ack", &self.pon_ack())
            .field("pon_busy", &self.pon_busy())
            .field("poff_ack", &self.poff_ack())
            .field("poff_busy", &self.poff_busy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EdgelockHdskStat {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EdgelockHdskStat {
            pon_ack: bool,
            pon_busy: bool,
            poff_ack: bool,
            poff_busy: super::vals::PoffBusy,
        }
        let proxy = EdgelockHdskStat {
            pon_ack: self.pon_ack(),
            pon_busy: self.pon_busy(),
            poff_ack: self.poff_ack(),
            poff_busy: self.poff_busy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Function Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FuncStat(pub u32);
impl FuncStat {
    #[doc = "power switch status"]
    #[must_use]
    #[inline(always)]
    pub const fn psw_stat(&self) -> super::vals::PswStat {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::PswStat::from_bits(val as u8)
    }
    #[doc = "power switch status"]
    #[inline(always)]
    pub const fn set_psw_stat(&mut self, val: super::vals::PswStat) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "reset status"]
    #[must_use]
    #[inline(always)]
    pub const fn rst_stat(&self) -> super::vals::RstStat {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::RstStat::from_bits(val as u8)
    }
    #[doc = "reset status"]
    #[inline(always)]
    pub const fn set_rst_stat(&mut self, val: super::vals::RstStat) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "isolation status"]
    #[must_use]
    #[inline(always)]
    pub const fn iso_stat(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "isolation status"]
    #[inline(always)]
    pub const fn set_iso_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Edgelock handshake status"]
    #[must_use]
    #[inline(always)]
    pub const fn edgelock_hdsk_stat(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Edgelock handshake status"]
    #[inline(always)]
    pub const fn set_edgelock_hdsk_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for FuncStat {
    #[inline(always)]
    fn default() -> FuncStat {
        FuncStat(4u64 as u32)
    }
}
impl core::fmt::Debug for FuncStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FuncStat")
            .field("psw_stat", &self.psw_stat())
            .field("rst_stat", &self.rst_stat())
            .field("iso_stat", &self.iso_stat())
            .field("edgelock_hdsk_stat", &self.edgelock_hdsk_stat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FuncStat {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct FuncStat {
            psw_stat: super::vals::PswStat,
            rst_stat: super::vals::RstStat,
            iso_stat: bool,
            edgelock_hdsk_stat: bool,
        }
        let proxy = FuncStat {
            psw_stat: self.psw_stat(),
            rst_stat: self.rst_stat(),
            iso_stat: self.iso_stat(),
            edgelock_hdsk_stat: self.edgelock_hdsk_stat(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ISO Delay Pre control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IsoDlyPre(pub u32);
impl IsoDlyPre {
    #[doc = "Delay from receiving iso off request to isolation disable, locked by LOCK_CFG field"]
    #[must_use]
    #[inline(always)]
    pub const fn iso_off_cnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Delay from receiving iso off request to isolation disable, locked by LOCK_CFG field"]
    #[inline(always)]
    pub const fn set_iso_off_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Delay from receiving iso_on request to isolation enable, locked by LOCK_CFG field"]
    #[must_use]
    #[inline(always)]
    pub const fn iso_on_cnt(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Delay from receiving iso_on request to isolation enable, locked by LOCK_CFG field"]
    #[inline(always)]
    pub const fn set_iso_on_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for IsoDlyPre {
    #[inline(always)]
    fn default() -> IsoDlyPre {
        IsoDlyPre(0u64 as u32)
    }
}
impl core::fmt::Debug for IsoDlyPre {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IsoDlyPre")
            .field("iso_off_cnt", &self.iso_off_cnt())
            .field("iso_on_cnt", &self.iso_on_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IsoDlyPre {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct IsoDlyPre {
            iso_off_cnt: u16,
            iso_on_cnt: u16,
        }
        let proxy = IsoDlyPre {
            iso_off_cnt: self.iso_off_cnt(),
            iso_on_cnt: self.iso_on_cnt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Low Power Mode Setting 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpmSetting0(pub u32);
impl LpmSetting0 {
    #[doc = "LPM setting of current domain"]
    #[must_use]
    #[inline(always)]
    pub const fn lpm_setting_cd(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "LPM setting of current domain"]
    #[inline(always)]
    pub const fn set_lpm_setting_cd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for LpmSetting0 {
    #[inline(always)]
    fn default() -> LpmSetting0 {
        LpmSetting0(3u64 as u32)
    }
}
impl core::fmt::Debug for LpmSetting0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LpmSetting0")
            .field("lpm_setting_cd", &self.lpm_setting_cd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LpmSetting0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct LpmSetting0 {
            lpm_setting_cd: u8,
        }
        let proxy = LpmSetting0 {
            lpm_setting_cd: self.lpm_setting_cd(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Low Power Mode Setting 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpmSetting1(pub u32);
impl LpmSetting1 {
    #[doc = "LPM setting of domain 0"]
    #[must_use]
    #[inline(always)]
    pub const fn lpm_setting_d0(&self) -> super::vals::LpmSettingD0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::LpmSettingD0::from_bits(val as u8)
    }
    #[doc = "LPM setting of domain 0"]
    #[inline(always)]
    pub const fn set_lpm_setting_d0(&mut self, val: super::vals::LpmSettingD0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "LPM setting of domain 1"]
    #[must_use]
    #[inline(always)]
    pub const fn lpm_setting_d1(&self) -> super::vals::LpmSettingD1 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::LpmSettingD1::from_bits(val as u8)
    }
    #[doc = "LPM setting of domain 1"]
    #[inline(always)]
    pub const fn set_lpm_setting_d1(&mut self, val: super::vals::LpmSettingD1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "LPM setting of domain 2"]
    #[must_use]
    #[inline(always)]
    pub const fn lpm_setting_d2(&self) -> super::vals::LpmSettingD2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::LpmSettingD2::from_bits(val as u8)
    }
    #[doc = "LPM setting of domain 2"]
    #[inline(always)]
    pub const fn set_lpm_setting_d2(&mut self, val: super::vals::LpmSettingD2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "LPM setting of domain 3"]
    #[must_use]
    #[inline(always)]
    pub const fn lpm_setting_d3(&self) -> super::vals::LpmSettingD3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::LpmSettingD3::from_bits(val as u8)
    }
    #[doc = "LPM setting of domain 3"]
    #[inline(always)]
    pub const fn set_lpm_setting_d3(&mut self, val: super::vals::LpmSettingD3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "LPM setting of domain 4"]
    #[must_use]
    #[inline(always)]
    pub const fn lpm_setting_d4(&self) -> super::vals::LpmSettingD4 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::LpmSettingD4::from_bits(val as u8)
    }
    #[doc = "LPM setting of domain 4"]
    #[inline(always)]
    pub const fn set_lpm_setting_d4(&mut self, val: super::vals::LpmSettingD4) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "LPM setting of domain 5"]
    #[must_use]
    #[inline(always)]
    pub const fn lpm_setting_d5(&self) -> super::vals::LpmSettingD5 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::LpmSettingD5::from_bits(val as u8)
    }
    #[doc = "LPM setting of domain 5"]
    #[inline(always)]
    pub const fn set_lpm_setting_d5(&mut self, val: super::vals::LpmSettingD5) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "LPM setting of domain 6"]
    #[must_use]
    #[inline(always)]
    pub const fn lpm_setting_d6(&self) -> super::vals::LpmSettingD6 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::LpmSettingD6::from_bits(val as u8)
    }
    #[doc = "LPM setting of domain 6"]
    #[inline(always)]
    pub const fn set_lpm_setting_d6(&mut self, val: super::vals::LpmSettingD6) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "LPM setting of domain 7"]
    #[must_use]
    #[inline(always)]
    pub const fn lpm_setting_d7(&self) -> super::vals::LpmSettingD7 {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::LpmSettingD7::from_bits(val as u8)
    }
    #[doc = "LPM setting of domain 7"]
    #[inline(always)]
    pub const fn set_lpm_setting_d7(&mut self, val: super::vals::LpmSettingD7) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
}
impl Default for LpmSetting1 {
    #[inline(always)]
    fn default() -> LpmSetting1 {
        LpmSetting1(858993459u64 as u32)
    }
}
impl core::fmt::Debug for LpmSetting1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LpmSetting1")
            .field("lpm_setting_d0", &self.lpm_setting_d0())
            .field("lpm_setting_d1", &self.lpm_setting_d1())
            .field("lpm_setting_d2", &self.lpm_setting_d2())
            .field("lpm_setting_d3", &self.lpm_setting_d3())
            .field("lpm_setting_d4", &self.lpm_setting_d4())
            .field("lpm_setting_d5", &self.lpm_setting_d5())
            .field("lpm_setting_d6", &self.lpm_setting_d6())
            .field("lpm_setting_d7", &self.lpm_setting_d7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LpmSetting1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct LpmSetting1 {
            lpm_setting_d0: super::vals::LpmSettingD0,
            lpm_setting_d1: super::vals::LpmSettingD1,
            lpm_setting_d2: super::vals::LpmSettingD2,
            lpm_setting_d3: super::vals::LpmSettingD3,
            lpm_setting_d4: super::vals::LpmSettingD4,
            lpm_setting_d5: super::vals::LpmSettingD5,
            lpm_setting_d6: super::vals::LpmSettingD6,
            lpm_setting_d7: super::vals::LpmSettingD7,
        }
        let proxy = LpmSetting1 {
            lpm_setting_d0: self.lpm_setting_d0(),
            lpm_setting_d1: self.lpm_setting_d1(),
            lpm_setting_d2: self.lpm_setting_d2(),
            lpm_setting_d3: self.lpm_setting_d3(),
            lpm_setting_d4: self.lpm_setting_d4(),
            lpm_setting_d5: self.lpm_setting_d5(),
            lpm_setting_d6: self.lpm_setting_d6(),
            lpm_setting_d7: self.lpm_setting_d7(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Low Power Mode Setting 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpmSetting2(pub u32);
impl LpmSetting2 {
    #[doc = "LPM setting of domain 8"]
    #[must_use]
    #[inline(always)]
    pub const fn lpm_setting_d8(&self) -> super::vals::LpmSettingD8 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::LpmSettingD8::from_bits(val as u8)
    }
    #[doc = "LPM setting of domain 8"]
    #[inline(always)]
    pub const fn set_lpm_setting_d8(&mut self, val: super::vals::LpmSettingD8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "LPM setting of domain 9"]
    #[must_use]
    #[inline(always)]
    pub const fn lpm_setting_d9(&self) -> super::vals::LpmSettingD9 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::LpmSettingD9::from_bits(val as u8)
    }
    #[doc = "LPM setting of domain 9"]
    #[inline(always)]
    pub const fn set_lpm_setting_d9(&mut self, val: super::vals::LpmSettingD9) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "LPM setting of domain 10"]
    #[must_use]
    #[inline(always)]
    pub const fn lpm_setting_d10(&self) -> super::vals::LpmSettingD10 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::LpmSettingD10::from_bits(val as u8)
    }
    #[doc = "LPM setting of domain 10"]
    #[inline(always)]
    pub const fn set_lpm_setting_d10(&mut self, val: super::vals::LpmSettingD10) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "LPM setting of domain 11"]
    #[must_use]
    #[inline(always)]
    pub const fn lpm_setting_d11(&self) -> super::vals::LpmSettingD11 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::LpmSettingD11::from_bits(val as u8)
    }
    #[doc = "LPM setting of domain 11"]
    #[inline(always)]
    pub const fn set_lpm_setting_d11(&mut self, val: super::vals::LpmSettingD11) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "LPM setting of domain 12"]
    #[must_use]
    #[inline(always)]
    pub const fn lpm_setting_d12(&self) -> super::vals::LpmSettingD12 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::LpmSettingD12::from_bits(val as u8)
    }
    #[doc = "LPM setting of domain 12"]
    #[inline(always)]
    pub const fn set_lpm_setting_d12(&mut self, val: super::vals::LpmSettingD12) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "LPM setting of domain 13"]
    #[must_use]
    #[inline(always)]
    pub const fn lpm_setting_d13(&self) -> super::vals::LpmSettingD13 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::LpmSettingD13::from_bits(val as u8)
    }
    #[doc = "LPM setting of domain 13"]
    #[inline(always)]
    pub const fn set_lpm_setting_d13(&mut self, val: super::vals::LpmSettingD13) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "LPM setting of domain 14"]
    #[must_use]
    #[inline(always)]
    pub const fn lpm_setting_d14(&self) -> super::vals::LpmSettingD14 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::LpmSettingD14::from_bits(val as u8)
    }
    #[doc = "LPM setting of domain 14"]
    #[inline(always)]
    pub const fn set_lpm_setting_d14(&mut self, val: super::vals::LpmSettingD14) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "LPM setting of domain 15"]
    #[must_use]
    #[inline(always)]
    pub const fn lpm_setting_d15(&self) -> super::vals::LpmSettingD15 {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::LpmSettingD15::from_bits(val as u8)
    }
    #[doc = "LPM setting of domain 15"]
    #[inline(always)]
    pub const fn set_lpm_setting_d15(&mut self, val: super::vals::LpmSettingD15) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
}
impl Default for LpmSetting2 {
    #[inline(always)]
    fn default() -> LpmSetting2 {
        LpmSetting2(858993459u64 as u32)
    }
}
impl core::fmt::Debug for LpmSetting2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LpmSetting2")
            .field("lpm_setting_d8", &self.lpm_setting_d8())
            .field("lpm_setting_d9", &self.lpm_setting_d9())
            .field("lpm_setting_d10", &self.lpm_setting_d10())
            .field("lpm_setting_d11", &self.lpm_setting_d11())
            .field("lpm_setting_d12", &self.lpm_setting_d12())
            .field("lpm_setting_d13", &self.lpm_setting_d13())
            .field("lpm_setting_d14", &self.lpm_setting_d14())
            .field("lpm_setting_d15", &self.lpm_setting_d15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LpmSetting2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct LpmSetting2 {
            lpm_setting_d8: super::vals::LpmSettingD8,
            lpm_setting_d9: super::vals::LpmSettingD9,
            lpm_setting_d10: super::vals::LpmSettingD10,
            lpm_setting_d11: super::vals::LpmSettingD11,
            lpm_setting_d12: super::vals::LpmSettingD12,
            lpm_setting_d13: super::vals::LpmSettingD13,
            lpm_setting_d14: super::vals::LpmSettingD14,
            lpm_setting_d15: super::vals::LpmSettingD15,
        }
        let proxy = LpmSetting2 {
            lpm_setting_d8: self.lpm_setting_d8(),
            lpm_setting_d9: self.lpm_setting_d9(),
            lpm_setting_d10: self.lpm_setting_d10(),
            lpm_setting_d11: self.lpm_setting_d11(),
            lpm_setting_d12: self.lpm_setting_d12(),
            lpm_setting_d13: self.lpm_setting_d13(),
            lpm_setting_d14: self.lpm_setting_d14(),
            lpm_setting_d15: self.lpm_setting_d15(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Memory Low Power Level Config"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MlplCfg(pub u32);
impl MlplCfg {
    #[doc = "Memory(MEM Type I with PSW or MEM Type I except for CM33PLATFORM_CACHE or CM7PLATFORM_CACHE, depending on which MIX of the register) low power level when hw power down"]
    #[must_use]
    #[inline(always)]
    pub const fn mlpl_hw_pdn_lmem(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Memory(MEM Type I with PSW or MEM Type I except for CM33PLATFORM_CACHE or CM7PLATFORM_CACHE, depending on which MIX of the register) low power level when hw power down"]
    #[inline(always)]
    pub const fn set_mlpl_hw_pdn_lmem(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Memory(MEM Type I with PSW or MEM Type I except for CM33PLATFORM_CACHE or CM7PLATFORM_CACHE, depending on which MIX of the register) low power level when hw power up"]
    #[must_use]
    #[inline(always)]
    pub const fn mlpl_hw_pup_lmem(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Memory(MEM Type I with PSW or MEM Type I except for CM33PLATFORM_CACHE or CM7PLATFORM_CACHE, depending on which MIX of the register) low power level when hw power up"]
    #[inline(always)]
    pub const fn set_mlpl_hw_pup_lmem(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Memory(MEM Type I with PSW or MEM Type I except for CM33PLATFORM_CACHE or CM7PLATFORM_CACHE, depending on which MIX of the register) low power level when sw trigger memory update power level"]
    #[must_use]
    #[inline(always)]
    pub const fn mlpl_sw_lmem(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "Memory(MEM Type I with PSW or MEM Type I except for CM33PLATFORM_CACHE or CM7PLATFORM_CACHE, depending on which MIX of the register) low power level when sw trigger memory update power level"]
    #[inline(always)]
    pub const fn set_mlpl_sw_lmem(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Memory(MEM Type II or MEM Type I's CM33PLATFORM_CACHE or CM7PLATFORM_CACHE, depending on which MIX of the register) low power level when hw power down"]
    #[must_use]
    #[inline(always)]
    pub const fn mlpl_hw_pdn_cache(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Memory(MEM Type II or MEM Type I's CM33PLATFORM_CACHE or CM7PLATFORM_CACHE, depending on which MIX of the register) low power level when hw power down"]
    #[inline(always)]
    pub const fn set_mlpl_hw_pdn_cache(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Memory(MEM Type II or MEM Type I's CM33PLATFORM_CACHE or CM7PLATFORM_CACHE, depending on which MIX of the register) low power level when hw power up"]
    #[must_use]
    #[inline(always)]
    pub const fn mlpl_hw_pup_cache(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "Memory(MEM Type II or MEM Type I's CM33PLATFORM_CACHE or CM7PLATFORM_CACHE, depending on which MIX of the register) low power level when hw power up"]
    #[inline(always)]
    pub const fn set_mlpl_hw_pup_cache(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    #[doc = "Memory(MEM Type II or MEM Type I's CM33PLATFORM_CACHE or CM7PLATFORM_CACHE, depending on which MIX of the register) low power level when sw trigger memory update power level"]
    #[must_use]
    #[inline(always)]
    pub const fn mlpl_sw_cache(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x07;
        val as u8
    }
    #[doc = "Memory(MEM Type II or MEM Type I's CM33PLATFORM_CACHE or CM7PLATFORM_CACHE, depending on which MIX of the register) low power level when sw trigger memory update power level"]
    #[inline(always)]
    pub const fn set_mlpl_sw_cache(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
    }
}
impl Default for MlplCfg {
    #[inline(always)]
    fn default() -> MlplCfg {
        MlplCfg(808923191u64 as u32)
    }
}
impl core::fmt::Debug for MlplCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MlplCfg")
            .field("mlpl_hw_pdn_lmem", &self.mlpl_hw_pdn_lmem())
            .field("mlpl_hw_pup_lmem", &self.mlpl_hw_pup_lmem())
            .field("mlpl_sw_lmem", &self.mlpl_sw_lmem())
            .field("mlpl_hw_pdn_cache", &self.mlpl_hw_pdn_cache())
            .field("mlpl_hw_pup_cache", &self.mlpl_hw_pup_cache())
            .field("mlpl_sw_cache", &self.mlpl_sw_cache())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MlplCfg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MlplCfg {
            mlpl_hw_pdn_lmem: u8,
            mlpl_hw_pup_lmem: u8,
            mlpl_sw_lmem: u8,
            mlpl_hw_pdn_cache: u8,
            mlpl_hw_pup_cache: u8,
            mlpl_sw_cache: u8,
        }
        let proxy = MlplCfg {
            mlpl_hw_pdn_lmem: self.mlpl_hw_pdn_lmem(),
            mlpl_hw_pup_lmem: self.mlpl_hw_pup_lmem(),
            mlpl_sw_lmem: self.mlpl_sw_lmem(),
            mlpl_hw_pdn_cache: self.mlpl_hw_pdn_cache(),
            mlpl_hw_pup_cache: self.mlpl_hw_pup_cache(),
            mlpl_sw_cache: self.mlpl_sw_cache(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Memory Low Power Level Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MlplStat(pub u32);
impl MlplStat {
    #[doc = "Current memory (MEM Type I with PSW or MEM Type I except for CM33PLATFORM_CACHE or CM7PLATFORM_CACHE, depending on which MIX of the register) low power level"]
    #[must_use]
    #[inline(always)]
    pub const fn current_mlpl_lmem(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Current memory (MEM Type I with PSW or MEM Type I except for CM33PLATFORM_CACHE or CM7PLATFORM_CACHE, depending on which MIX of the register) low power level"]
    #[inline(always)]
    pub const fn set_current_mlpl_lmem(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Busy requesting low power level of memory (MEM Type I with PSW or MEM Type I except for CM33PLATFORM_CACHE or CM7PLATFORM_CACHE, depending on which MIX of the register)"]
    #[must_use]
    #[inline(always)]
    pub const fn lmem_busy(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Busy requesting low power level of memory (MEM Type I with PSW or MEM Type I except for CM33PLATFORM_CACHE or CM7PLATFORM_CACHE, depending on which MIX of the register)"]
    #[inline(always)]
    pub const fn set_lmem_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Current memory (MEM Type II or MEM Type I's CM33PLATFORM_CACHE or CM7PLATFORM_CACHE, depending on which MIX of the register) low power level"]
    #[must_use]
    #[inline(always)]
    pub const fn current_mlpl_cache(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Current memory (MEM Type II or MEM Type I's CM33PLATFORM_CACHE or CM7PLATFORM_CACHE, depending on which MIX of the register) low power level"]
    #[inline(always)]
    pub const fn set_current_mlpl_cache(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Busy requesting low power level of memory (MEM Type II or MEM Type I's CM33PLATFORM_CACHE or CM7PLATFORM_CACHE, depending on which MIX of the register)"]
    #[must_use]
    #[inline(always)]
    pub const fn cache_busy(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Busy requesting low power level of memory (MEM Type II or MEM Type I's CM33PLATFORM_CACHE or CM7PLATFORM_CACHE, depending on which MIX of the register)"]
    #[inline(always)]
    pub const fn set_cache_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for MlplStat {
    #[inline(always)]
    fn default() -> MlplStat {
        MlplStat(51u64 as u32)
    }
}
impl core::fmt::Debug for MlplStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MlplStat")
            .field("current_mlpl_lmem", &self.current_mlpl_lmem())
            .field("lmem_busy", &self.lmem_busy())
            .field("current_mlpl_cache", &self.current_mlpl_cache())
            .field("cache_busy", &self.cache_busy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MlplStat {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MlplStat {
            current_mlpl_lmem: u8,
            lmem_busy: bool,
            current_mlpl_cache: u8,
            cache_busy: bool,
        }
        let proxy = MlplStat {
            current_mlpl_lmem: self.current_mlpl_lmem(),
            lmem_busy: self.lmem_busy(),
            current_mlpl_cache: self.current_mlpl_cache(),
            cache_busy: self.cache_busy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Memory Low Power Level Trigger Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MlplTrigCtrl(pub u32);
impl MlplTrigCtrl {
    #[doc = "Software trigger local memory (MEM Type I with PSW or MEM Type I except for CM33PLATFORM_CACHE or CM7PLATFORM_CACHE, depending on which MIX of the register) update low power level"]
    #[must_use]
    #[inline(always)]
    pub const fn sw_trig_lmem_mlpl_trans(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Software trigger local memory (MEM Type I with PSW or MEM Type I except for CM33PLATFORM_CACHE or CM7PLATFORM_CACHE, depending on which MIX of the register) update low power level"]
    #[inline(always)]
    pub const fn set_sw_trig_lmem_mlpl_trans(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Software trigger CACHE memory (MEM Type II or MEM Type I's CM33PLATFORM_CACHE or CM7PLATFORM_CACHE, depending on which MIX of the register) update low power level"]
    #[must_use]
    #[inline(always)]
    pub const fn sw_trig_cache_mlpl_trans(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Software trigger CACHE memory (MEM Type II or MEM Type I's CM33PLATFORM_CACHE or CM7PLATFORM_CACHE, depending on which MIX of the register) update low power level"]
    #[inline(always)]
    pub const fn set_sw_trig_cache_mlpl_trans(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for MlplTrigCtrl {
    #[inline(always)]
    fn default() -> MlplTrigCtrl {
        MlplTrigCtrl(0u64 as u32)
    }
}
impl core::fmt::Debug for MlplTrigCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MlplTrigCtrl")
            .field("sw_trig_lmem_mlpl_trans", &self.sw_trig_lmem_mlpl_trans())
            .field("sw_trig_cache_mlpl_trans", &self.sw_trig_cache_mlpl_trans())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MlplTrigCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MlplTrigCtrl {
            sw_trig_lmem_mlpl_trans: bool,
            sw_trig_cache_mlpl_trans: bool,
        }
        let proxy = MlplTrigCtrl {
            sw_trig_lmem_mlpl_trans: self.sw_trig_lmem_mlpl_trans(),
            sw_trig_cache_mlpl_trans: self.sw_trig_cache_mlpl_trans(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PSW Counter Config for HF"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PswCntCfgHf(pub u32);
impl PswCntCfgHf {
    #[doc = "PUP HF Count config: usage depends on CNT_MODE, locked by LOCK_CFG field"]
    #[must_use]
    #[inline(always)]
    pub const fn pon_cnt_cfg(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "PUP HF Count config: usage depends on CNT_MODE, locked by LOCK_CFG field"]
    #[inline(always)]
    pub const fn set_pon_cnt_cfg(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "PDN HF Count config: usage depends on CNT_MODE, locked by LOCK_CFG field"]
    #[must_use]
    #[inline(always)]
    pub const fn poff_cnt_cfg(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "PDN HF Count config: usage depends on CNT_MODE, locked by LOCK_CFG field"]
    #[inline(always)]
    pub const fn set_poff_cnt_cfg(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for PswCntCfgHf {
    #[inline(always)]
    fn default() -> PswCntCfgHf {
        PswCntCfgHf(16u64 as u32)
    }
}
impl core::fmt::Debug for PswCntCfgHf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PswCntCfgHf")
            .field("pon_cnt_cfg", &self.pon_cnt_cfg())
            .field("poff_cnt_cfg", &self.poff_cnt_cfg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PswCntCfgHf {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PswCntCfgHf {
            pon_cnt_cfg: u16,
            poff_cnt_cfg: u16,
        }
        let proxy = PswCntCfgHf {
            pon_cnt_cfg: self.pon_cnt_cfg(),
            poff_cnt_cfg: self.poff_cnt_cfg(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PSW Counter Config for LF"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PswCntCfgLf(pub u32);
impl PswCntCfgLf {
    #[doc = "PUP LF Count config: usage depends on CNT_MODE, locked by LOCK_CFG field"]
    #[must_use]
    #[inline(always)]
    pub const fn pon_cnt_cfg(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "PUP LF Count config: usage depends on CNT_MODE, locked by LOCK_CFG field"]
    #[inline(always)]
    pub const fn set_pon_cnt_cfg(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "PDN LF Count config: usage depends on CNT_MODE, locked by LOCK_CFG field"]
    #[must_use]
    #[inline(always)]
    pub const fn poff_cnt_cfg(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "PDN LF Count config: usage depends on CNT_MODE, locked by LOCK_CFG field"]
    #[inline(always)]
    pub const fn set_poff_cnt_cfg(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for PswCntCfgLf {
    #[inline(always)]
    fn default() -> PswCntCfgLf {
        PswCntCfgLf(16u64 as u32)
    }
}
impl core::fmt::Debug for PswCntCfgLf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PswCntCfgLf")
            .field("pon_cnt_cfg", &self.pon_cnt_cfg())
            .field("poff_cnt_cfg", &self.poff_cnt_cfg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PswCntCfgLf {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PswCntCfgLf {
            pon_cnt_cfg: u16,
            poff_cnt_cfg: u16,
        }
        let proxy = PswCntCfgLf {
            pon_cnt_cfg: self.pon_cnt_cfg(),
            poff_cnt_cfg: self.poff_cnt_cfg(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PSW Counter Status for HF"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PswCntStatHf(pub u32);
impl PswCntStatHf {
    #[doc = "HF PSW acknowledge count for power up, record the delay from power switch change to acknowledge received"]
    #[must_use]
    #[inline(always)]
    pub const fn pon_cnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "HF PSW acknowledge count for power up, record the delay from power switch change to acknowledge received"]
    #[inline(always)]
    pub const fn set_pon_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "HF PSW acknowledge count for power down, record the delay from power switch change to acknowledge received"]
    #[must_use]
    #[inline(always)]
    pub const fn poff_cnt(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "HF PSW acknowledge count for power down, record the delay from power switch change to acknowledge received"]
    #[inline(always)]
    pub const fn set_poff_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for PswCntStatHf {
    #[inline(always)]
    fn default() -> PswCntStatHf {
        PswCntStatHf(0u64 as u32)
    }
}
impl core::fmt::Debug for PswCntStatHf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PswCntStatHf")
            .field("pon_cnt", &self.pon_cnt())
            .field("poff_cnt", &self.poff_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PswCntStatHf {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PswCntStatHf {
            pon_cnt: u16,
            poff_cnt: u16,
        }
        let proxy = PswCntStatHf {
            pon_cnt: self.pon_cnt(),
            poff_cnt: self.poff_cnt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PSW Counter Status for LF"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PswCntStatLf(pub u32);
impl PswCntStatLf {
    #[doc = "LF PSW acknowledge count for power up, record the delay from power switch change to acknowledge received"]
    #[must_use]
    #[inline(always)]
    pub const fn pon_cnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "LF PSW acknowledge count for power up, record the delay from power switch change to acknowledge received"]
    #[inline(always)]
    pub const fn set_pon_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "LF PSW acknowledge count for power down, record the delay from power switch change to acknowledge received"]
    #[must_use]
    #[inline(always)]
    pub const fn poff_cnt(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "LF PSW acknowledge count for power down, record the delay from power switch change to acknowledge received"]
    #[inline(always)]
    pub const fn set_poff_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for PswCntStatLf {
    #[inline(always)]
    fn default() -> PswCntStatLf {
        PswCntStatLf(0u64 as u32)
    }
}
impl core::fmt::Debug for PswCntStatLf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PswCntStatLf")
            .field("pon_cnt", &self.pon_cnt())
            .field("poff_cnt", &self.poff_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PswCntStatLf {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PswCntStatLf {
            pon_cnt: u16,
            poff_cnt: u16,
        }
        let proxy = PswCntStatLf {
            pon_cnt: self.pon_cnt(),
            poff_cnt: self.poff_cnt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PSW Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PswCtrl(pub u32);
impl PswCtrl {
    #[doc = "Acknowledge for low fanout value is inverted from power switch control, locked by LOCK_CFG field"]
    #[must_use]
    #[inline(always)]
    pub const fn ack_invert_lf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Acknowledge for low fanout value is inverted from power switch control, locked by LOCK_CFG field"]
    #[inline(always)]
    pub const fn set_ack_invert_lf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Acknowledge for high fanout value is inverted from power switch control, locked by LOCK_CFG field"]
    #[must_use]
    #[inline(always)]
    pub const fn ack_invert_hf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Acknowledge for high fanout value is inverted from power switch control, locked by LOCK_CFG field"]
    #[inline(always)]
    pub const fn set_ack_invert_hf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Configure the acknowledge counter for power up working mode, locked by LOCK_CFG field"]
    #[must_use]
    #[inline(always)]
    pub const fn pon_cnt_mode(&self) -> super::vals::PswCtrlPonCntMode {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::PswCtrlPonCntMode::from_bits(val as u8)
    }
    #[doc = "Configure the acknowledge counter for power up working mode, locked by LOCK_CFG field"]
    #[inline(always)]
    pub const fn set_pon_cnt_mode(&mut self, val: super::vals::PswCtrlPonCntMode) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Configures the acknowledge counter for power down working mode, locked by LOCK_CFG field"]
    #[must_use]
    #[inline(always)]
    pub const fn poff_cnt_mode(&self) -> super::vals::PswCtrlPoffCntMode {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::PswCtrlPoffCntMode::from_bits(val as u8)
    }
    #[doc = "Configures the acknowledge counter for power down working mode, locked by LOCK_CFG field"]
    #[inline(always)]
    pub const fn set_poff_cnt_mode(&mut self, val: super::vals::PswCtrlPoffCntMode) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for PswCtrl {
    #[inline(always)]
    fn default() -> PswCtrl {
        PswCtrl(2147516416u64 as u32)
    }
}
impl core::fmt::Debug for PswCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PswCtrl")
            .field("ack_invert_lf", &self.ack_invert_lf())
            .field("ack_invert_hf", &self.ack_invert_hf())
            .field("pon_cnt_mode", &self.pon_cnt_mode())
            .field("poff_cnt_mode", &self.poff_cnt_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PswCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PswCtrl {
            ack_invert_lf: bool,
            ack_invert_hf: bool,
            pon_cnt_mode: super::vals::PswCtrlPonCntMode,
            poff_cnt_mode: super::vals::PswCtrlPoffCntMode,
        }
        let proxy = PswCtrl {
            ack_invert_lf: self.ack_invert_lf(),
            ack_invert_hf: self.ack_invert_hf(),
            pon_cnt_mode: self.pon_cnt_mode(),
            poff_cnt_mode: self.poff_cnt_mode(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PSW Delay Pre for HF"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PswDlyPreHf(pub u32);
impl PswDlyPreHf {
    #[doc = "Delay from receiving power on hf request to power switch turn on, locked by LOCK_CFG field"]
    #[must_use]
    #[inline(always)]
    pub const fn psw_on(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Delay from receiving power on hf request to power switch turn on, locked by LOCK_CFG field"]
    #[inline(always)]
    pub const fn set_psw_on(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Delay from receiving power off hf request to power switch shut off, locked by LOCK_CFG field"]
    #[must_use]
    #[inline(always)]
    pub const fn psw_off(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Delay from receiving power off hf request to power switch shut off, locked by LOCK_CFG field"]
    #[inline(always)]
    pub const fn set_psw_off(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for PswDlyPreHf {
    #[inline(always)]
    fn default() -> PswDlyPreHf {
        PswDlyPreHf(0u64 as u32)
    }
}
impl core::fmt::Debug for PswDlyPreHf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PswDlyPreHf")
            .field("psw_on", &self.psw_on())
            .field("psw_off", &self.psw_off())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PswDlyPreHf {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PswDlyPreHf {
            psw_on: u16,
            psw_off: u16,
        }
        let proxy = PswDlyPreHf {
            psw_on: self.psw_on(),
            psw_off: self.psw_off(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PSW Delay Pre for LF"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PswDlyPreLf(pub u32);
impl PswDlyPreLf {
    #[doc = "Delay from receiving power on lf request to power switch turn on, locked by LOCK_CFG field"]
    #[must_use]
    #[inline(always)]
    pub const fn psw_on(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Delay from receiving power on lf request to power switch turn on, locked by LOCK_CFG field"]
    #[inline(always)]
    pub const fn set_psw_on(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Delay from receiving power off lf request to power switch shut off, locked by LOCK_CFG field"]
    #[must_use]
    #[inline(always)]
    pub const fn psw_off(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Delay from receiving power off lf request to power switch shut off, locked by LOCK_CFG field"]
    #[inline(always)]
    pub const fn set_psw_off(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for PswDlyPreLf {
    #[inline(always)]
    fn default() -> PswDlyPreLf {
        PswDlyPreLf(0u64 as u32)
    }
}
impl core::fmt::Debug for PswDlyPreLf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PswDlyPreLf")
            .field("psw_on", &self.psw_on())
            .field("psw_off", &self.psw_off())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PswDlyPreLf {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PswDlyPreLf {
            psw_on: u16,
            psw_off: u16,
        }
        let proxy = PswDlyPreLf {
            psw_on: self.psw_on(),
            psw_off: self.psw_off(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PSW Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PswStat(pub u32);
impl PswStat {
    #[doc = "Power switch acknowledge status for low fanout"]
    #[must_use]
    #[inline(always)]
    pub const fn ack_stat_lf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Power switch acknowledge status for low fanout"]
    #[inline(always)]
    pub const fn set_ack_stat_lf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Power switch acknowledge status for high fanout"]
    #[must_use]
    #[inline(always)]
    pub const fn ack_stat_hf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Power switch acknowledge status for high fanout"]
    #[inline(always)]
    pub const fn set_ack_stat_hf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for PswStat {
    #[inline(always)]
    fn default() -> PswStat {
        PswStat(0u64 as u32)
    }
}
impl core::fmt::Debug for PswStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PswStat")
            .field("ack_stat_lf", &self.ack_stat_lf())
            .field("ack_stat_hf", &self.ack_stat_hf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PswStat {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PswStat {
            ack_stat_lf: bool,
            ack_stat_hf: bool,
        }
        let proxy = PswStat {
            ack_stat_lf: self.ack_stat_lf(),
            ack_stat_hf: self.ack_stat_hf(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SLICE Software Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SliceSwCtrl(pub u32);
impl SliceSwCtrl {
    #[doc = "Software power off control. This field can only be updated while LPM_MODE=0."]
    #[must_use]
    #[inline(always)]
    pub const fn psw_off_soft(&self) -> super::vals::PswOffSoft {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::PswOffSoft::from_bits(val as u8)
    }
    #[doc = "Software power off control. This field can only be updated while LPM_MODE=0."]
    #[inline(always)]
    pub const fn set_psw_off_soft(&mut self, val: super::vals::PswOffSoft) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Software reset control. This field can only be updated while LPM_MODE=0."]
    #[must_use]
    #[inline(always)]
    pub const fn rst_ctrl_soft(&self) -> super::vals::RstCtrlSoft {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::RstCtrlSoft::from_bits(val as u8)
    }
    #[doc = "Software reset control. This field can only be updated while LPM_MODE=0."]
    #[inline(always)]
    pub const fn set_rst_ctrl_soft(&mut self, val: super::vals::RstCtrlSoft) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Software isolation on control. This field can only be updated while LPM_MODE=0."]
    #[must_use]
    #[inline(always)]
    pub const fn iso_on_soft(&self) -> super::vals::IsoOnSoft {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::IsoOnSoft::from_bits(val as u8)
    }
    #[doc = "Software isolation on control. This field can only be updated while LPM_MODE=0."]
    #[inline(always)]
    pub const fn set_iso_on_soft(&mut self, val: super::vals::IsoOnSoft) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Software Edgelock handshake control. This field can only be updated while LPM_MODE=0."]
    #[must_use]
    #[inline(always)]
    pub const fn edgelock_hdsk_soft(&self) -> super::vals::EdgelockHdskSoft {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::EdgelockHdskSoft::from_bits(val as u8)
    }
    #[doc = "Software Edgelock handshake control. This field can only be updated while LPM_MODE=0."]
    #[inline(always)]
    pub const fn set_edgelock_hdsk_soft(&mut self, val: super::vals::EdgelockHdskSoft) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "By flow sequence, Software power down sequence includes Edgelock handshake for power down, isolation on, reset assert, power off; Software power up sequence includes power up, reset deassert, isolation off, Edgelock handshake for power up. This bit can only be updated while LPM_MODE=0."]
    #[must_use]
    #[inline(always)]
    pub const fn pdn_soft(&self) -> super::vals::PdnSoft {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PdnSoft::from_bits(val as u8)
    }
    #[doc = "By flow sequence, Software power down sequence includes Edgelock handshake for power down, isolation on, reset assert, power off; Software power up sequence includes power up, reset deassert, isolation off, Edgelock handshake for power up. This bit can only be updated while LPM_MODE=0."]
    #[inline(always)]
    pub const fn set_pdn_soft(&mut self, val: super::vals::PdnSoft) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SliceSwCtrl {
    #[inline(always)]
    fn default() -> SliceSwCtrl {
        SliceSwCtrl(0u64 as u32)
    }
}
impl core::fmt::Debug for SliceSwCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SliceSwCtrl")
            .field("psw_off_soft", &self.psw_off_soft())
            .field("rst_ctrl_soft", &self.rst_ctrl_soft())
            .field("iso_on_soft", &self.iso_on_soft())
            .field("edgelock_hdsk_soft", &self.edgelock_hdsk_soft())
            .field("pdn_soft", &self.pdn_soft())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SliceSwCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SliceSwCtrl {
            psw_off_soft: super::vals::PswOffSoft,
            rst_ctrl_soft: super::vals::RstCtrlSoft,
            iso_on_soft: super::vals::IsoOnSoft,
            edgelock_hdsk_soft: super::vals::EdgelockHdskSoft,
            pdn_soft: super::vals::PdnSoft,
        }
        let proxy = SliceSwCtrl {
            psw_off_soft: self.psw_off_soft(),
            rst_ctrl_soft: self.rst_ctrl_soft(),
            iso_on_soft: self.iso_on_soft(),
            edgelock_hdsk_soft: self.edgelock_hdsk_soft(),
            pdn_soft: self.pdn_soft(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "UPI Status 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UpiStat0(pub u32);
impl UpiStat0 {
    #[doc = "CPU mode transfer to trigger isolation change request of 16 domains"]
    #[must_use]
    #[inline(always)]
    pub const fn upi_iso_request(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "CPU mode transfer to trigger isolation change request of 16 domains"]
    #[inline(always)]
    pub const fn set_upi_iso_request(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "CPU mode transfer to trigger Edgelock handshake request of 16 domains"]
    #[must_use]
    #[inline(always)]
    pub const fn upi_edgelock_hdsk_request(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "CPU mode transfer to trigger Edgelock handshake request of 16 domains"]
    #[inline(always)]
    pub const fn set_upi_edgelock_hdsk_request(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for UpiStat0 {
    #[inline(always)]
    fn default() -> UpiStat0 {
        UpiStat0(0u64 as u32)
    }
}
impl core::fmt::Debug for UpiStat0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UpiStat0")
            .field("upi_iso_request", &self.upi_iso_request())
            .field(
                "upi_edgelock_hdsk_request",
                &self.upi_edgelock_hdsk_request(),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UpiStat0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct UpiStat0 {
            upi_iso_request: u16,
            upi_edgelock_hdsk_request: u16,
        }
        let proxy = UpiStat0 {
            upi_iso_request: self.upi_iso_request(),
            upi_edgelock_hdsk_request: self.upi_edgelock_hdsk_request(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "UPI Status 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UpiStat1(pub u32);
impl UpiStat1 {
    #[doc = "CPU mode transfer to trigger power switch request of 16 domains"]
    #[must_use]
    #[inline(always)]
    pub const fn upi_power_request(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "CPU mode transfer to trigger power switch request of 16 domains"]
    #[inline(always)]
    pub const fn set_upi_power_request(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "CPU mode transfer to trigger reset change request of 16 domains"]
    #[must_use]
    #[inline(always)]
    pub const fn upi_reset_request(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "CPU mode transfer to trigger reset change request of 16 domains"]
    #[inline(always)]
    pub const fn set_upi_reset_request(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for UpiStat1 {
    #[inline(always)]
    fn default() -> UpiStat1 {
        UpiStat1(0u64 as u32)
    }
}
impl core::fmt::Debug for UpiStat1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UpiStat1")
            .field("upi_power_request", &self.upi_power_request())
            .field("upi_reset_request", &self.upi_reset_request())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UpiStat1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct UpiStat1 {
            upi_power_request: u16,
            upi_reset_request: u16,
        }
        let proxy = UpiStat1 {
            upi_power_request: self.upi_power_request(),
            upi_reset_request: self.upi_reset_request(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
