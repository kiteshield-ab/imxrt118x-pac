#[doc = "MPC Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MifCtrl(pub u32);
impl MifCtrl {
    #[doc = "Memory low power pins controlled by SW"]
    #[must_use]
    #[inline(always)]
    pub const fn sw_ctrl_pin(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Memory low power pins controlled by SW"]
    #[inline(always)]
    pub const fn set_sw_ctrl_pin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Memory power status will be considered when determining slice power status."]
    #[must_use]
    #[inline(always)]
    pub const fn mem_pwr_st_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Memory power status will be considered when determining slice power status."]
    #[inline(always)]
    pub const fn set_mem_pwr_st_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Configuration lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_cfg(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Configuration lock"]
    #[inline(always)]
    pub const fn set_lock_cfg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for MifCtrl {
    #[inline(always)]
    fn default() -> MifCtrl {
        MifCtrl(0u64 as u32)
    }
}
impl core::fmt::Debug for MifCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MifCtrl")
            .field("sw_ctrl_pin", &self.sw_ctrl_pin())
            .field("mem_pwr_st_en", &self.mem_pwr_st_en())
            .field("lock_cfg", &self.lock_cfg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MifCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MifCtrl {
            sw_ctrl_pin: bool,
            mem_pwr_st_en: bool,
            lock_cfg: bool,
        }
        let proxy = MifCtrl {
            sw_ctrl_pin: self.sw_ctrl_pin(),
            mem_pwr_st_en: self.mem_pwr_st_en(),
            lock_cfg: self.lock_cfg(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "MIF Delay of IG"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MifDlyIg(pub u32);
impl MifDlyIg {
    #[doc = "Delay before asserting signal to high, locked by LOCK_CFG field"]
    #[must_use]
    #[inline(always)]
    pub const fn pre_hi_dly(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Delay before asserting signal to high, locked by LOCK_CFG field"]
    #[inline(always)]
    pub const fn set_pre_hi_dly(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Delay before de-asserting signal to low, locked by LOCK_CFG field"]
    #[must_use]
    #[inline(always)]
    pub const fn pre_lo_dly(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Delay before de-asserting signal to low, locked by LOCK_CFG field"]
    #[inline(always)]
    pub const fn set_pre_lo_dly(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MifDlyIg {
    #[inline(always)]
    fn default() -> MifDlyIg {
        MifDlyIg(0u64 as u32)
    }
}
impl core::fmt::Debug for MifDlyIg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MifDlyIg")
            .field("pre_hi_dly", &self.pre_hi_dly())
            .field("pre_lo_dly", &self.pre_lo_dly())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MifDlyIg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MifDlyIg {
            pre_hi_dly: u16,
            pre_lo_dly: u16,
        }
        let proxy = MifDlyIg {
            pre_hi_dly: self.pre_hi_dly(),
            pre_lo_dly: self.pre_lo_dly(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "MIF Delay of PD_B"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MifDlyPdB(pub u32);
impl MifDlyPdB {
    #[doc = "Delay before asserting signal to high, locked by LOCK_CFG field"]
    #[must_use]
    #[inline(always)]
    pub const fn pre_hi_dly(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Delay before asserting signal to high, locked by LOCK_CFG field"]
    #[inline(always)]
    pub const fn set_pre_hi_dly(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Delay before de-asserting signal to low, locked by LOCK_CFG field"]
    #[must_use]
    #[inline(always)]
    pub const fn pre_lo_dly(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Delay before de-asserting signal to low, locked by LOCK_CFG field"]
    #[inline(always)]
    pub const fn set_pre_lo_dly(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MifDlyPdB {
    #[inline(always)]
    fn default() -> MifDlyPdB {
        MifDlyPdB(0u64 as u32)
    }
}
impl core::fmt::Debug for MifDlyPdB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MifDlyPdB")
            .field("pre_hi_dly", &self.pre_hi_dly())
            .field("pre_lo_dly", &self.pre_lo_dly())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MifDlyPdB {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MifDlyPdB {
            pre_hi_dly: u16,
            pre_lo_dly: u16,
        }
        let proxy = MifDlyPdB {
            pre_hi_dly: self.pre_hi_dly(),
            pre_lo_dly: self.pre_lo_dly(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "MIF Delay of WLPD"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MifDlyWlpd(pub u32);
impl MifDlyWlpd {
    #[doc = "Delay before asserting signal to high, locked by LOCK_CFG field"]
    #[must_use]
    #[inline(always)]
    pub const fn pre_hi_dly(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Delay before asserting signal to high, locked by LOCK_CFG field"]
    #[inline(always)]
    pub const fn set_pre_hi_dly(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Delay before de-asserting signal to low, locked by LOCK_CFG field"]
    #[must_use]
    #[inline(always)]
    pub const fn pre_lo_dly(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Delay before de-asserting signal to low, locked by LOCK_CFG field"]
    #[inline(always)]
    pub const fn set_pre_lo_dly(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MifDlyWlpd {
    #[inline(always)]
    fn default() -> MifDlyWlpd {
        MifDlyWlpd(0u64 as u32)
    }
}
impl core::fmt::Debug for MifDlyWlpd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MifDlyWlpd")
            .field("pre_hi_dly", &self.pre_hi_dly())
            .field("pre_lo_dly", &self.pre_lo_dly())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MifDlyWlpd {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MifDlyWlpd {
            pre_hi_dly: u16,
            pre_lo_dly: u16,
        }
        let proxy = MifDlyWlpd {
            pre_hi_dly: self.pre_hi_dly(),
            pre_lo_dly: self.pre_lo_dly(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "MIF MLPL control of IG"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MifMlplIg(pub u32);
impl MifMlplIg {
    #[doc = "Signal behavior at 8 different MLPL settings"]
    #[must_use]
    #[inline(always)]
    pub const fn mlpl_ctrl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Signal behavior at 8 different MLPL settings"]
    #[inline(always)]
    pub const fn set_mlpl_ctrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Software control IG"]
    #[must_use]
    #[inline(always)]
    pub const fn sw_ig(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Software control IG"]
    #[inline(always)]
    pub const fn set_sw_ig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for MifMlplIg {
    #[inline(always)]
    fn default() -> MifMlplIg {
        MifMlplIg(16u64 as u32)
    }
}
impl core::fmt::Debug for MifMlplIg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MifMlplIg")
            .field("mlpl_ctrl", &self.mlpl_ctrl())
            .field("sw_ig", &self.sw_ig())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MifMlplIg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MifMlplIg {
            mlpl_ctrl: u8,
            sw_ig: bool,
        }
        let proxy = MifMlplIg {
            mlpl_ctrl: self.mlpl_ctrl(),
            sw_ig: self.sw_ig(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "MIF MLPL control of PD_B"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MifMlplPdB(pub u32);
impl MifMlplPdB {
    #[doc = "Signal behavior at 8 different MLPL settings"]
    #[must_use]
    #[inline(always)]
    pub const fn mlpl_ctrl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Signal behavior at 8 different MLPL settings"]
    #[inline(always)]
    pub const fn set_mlpl_ctrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "software control PD_B"]
    #[must_use]
    #[inline(always)]
    pub const fn sw_pd_b(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "software control PD_B"]
    #[inline(always)]
    pub const fn set_sw_pd_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for MifMlplPdB {
    #[inline(always)]
    fn default() -> MifMlplPdB {
        MifMlplPdB(27u64 as u32)
    }
}
impl core::fmt::Debug for MifMlplPdB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MifMlplPdB")
            .field("mlpl_ctrl", &self.mlpl_ctrl())
            .field("sw_pd_b", &self.sw_pd_b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MifMlplPdB {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MifMlplPdB {
            mlpl_ctrl: u8,
            sw_pd_b: bool,
        }
        let proxy = MifMlplPdB {
            mlpl_ctrl: self.mlpl_ctrl(),
            sw_pd_b: self.sw_pd_b(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "MIF MLPL control of WLPD"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MifMlplWlpd(pub u32);
impl MifMlplWlpd {
    #[doc = "Signal behavior at 8 different MLPL settings"]
    #[must_use]
    #[inline(always)]
    pub const fn mlpl_ctrl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Signal behavior at 8 different MLPL settings"]
    #[inline(always)]
    pub const fn set_mlpl_ctrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Software control WLPD"]
    #[must_use]
    #[inline(always)]
    pub const fn sw_wlpd(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Software control WLPD"]
    #[inline(always)]
    pub const fn set_sw_wlpd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for MifMlplWlpd {
    #[inline(always)]
    fn default() -> MifMlplWlpd {
        MifMlplWlpd(6u64 as u32)
    }
}
impl core::fmt::Debug for MifMlplWlpd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MifMlplWlpd")
            .field("mlpl_ctrl", &self.mlpl_ctrl())
            .field("sw_wlpd", &self.sw_wlpd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MifMlplWlpd {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MifMlplWlpd {
            mlpl_ctrl: u8,
            sw_wlpd: bool,
        }
        let proxy = MifMlplWlpd {
            mlpl_ctrl: self.mlpl_ctrl(),
            sw_wlpd: self.sw_wlpd(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "MIF Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MifStat(pub u32);
impl MifStat {
    #[doc = "Current state of CURRENT_MLPL"]
    #[must_use]
    #[inline(always)]
    pub const fn mlpl_state(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Current state of CURRENT_MLPL"]
    #[inline(always)]
    pub const fn set_mlpl_state(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Current state of IG"]
    #[must_use]
    #[inline(always)]
    pub const fn ig_state(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Current state of IG"]
    #[inline(always)]
    pub const fn set_ig_state(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Current state of WLPD"]
    #[must_use]
    #[inline(always)]
    pub const fn wlpd_state(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Current state of WLPD"]
    #[inline(always)]
    pub const fn set_wlpd_state(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Current state of PD_B"]
    #[must_use]
    #[inline(always)]
    pub const fn pd_b_state(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Current state of PD_B"]
    #[inline(always)]
    pub const fn set_pd_b_state(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for MifStat {
    #[inline(always)]
    fn default() -> MifStat {
        MifStat(32u64 as u32)
    }
}
impl core::fmt::Debug for MifStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MifStat")
            .field("mlpl_state", &self.mlpl_state())
            .field("ig_state", &self.ig_state())
            .field("wlpd_state", &self.wlpd_state())
            .field("pd_b_state", &self.pd_b_state())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MifStat {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MifStat {
            mlpl_state: u8,
            ig_state: bool,
            wlpd_state: bool,
            pd_b_state: bool,
        }
        let proxy = MifStat {
            mlpl_state: self.mlpl_state(),
            ig_state: self.ig_state(),
            wlpd_state: self.wlpd_state(),
            pd_b_state: self.pd_b_state(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
