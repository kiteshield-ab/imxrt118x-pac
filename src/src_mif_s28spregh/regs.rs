#[doc = "MPC Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MifCtrl(pub u32);
impl MifCtrl {
    #[doc = "Memory low power pins controlled by software (SW)"]
    #[must_use]
    #[inline(always)]
    pub const fn sw_ctrl_pin(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Memory low power pins controlled by software (SW)"]
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
#[doc = "MIF Delay of HS"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MifDlyHs(pub u32);
impl MifDlyHs {
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
impl Default for MifDlyHs {
    #[inline(always)]
    fn default() -> MifDlyHs {
        MifDlyHs(0u64 as u32)
    }
}
impl core::fmt::Debug for MifDlyHs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MifDlyHs")
            .field("pre_hi_dly", &self.pre_hi_dly())
            .field("pre_lo_dly", &self.pre_lo_dly())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MifDlyHs {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MifDlyHs {
            pre_hi_dly: u16,
            pre_lo_dly: u16,
        }
        let proxy = MifDlyHs {
            pre_hi_dly: self.pre_hi_dly(),
            pre_lo_dly: self.pre_lo_dly(),
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
#[doc = "MIF Delay of LS"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MifDlyLs(pub u32);
impl MifDlyLs {
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
impl Default for MifDlyLs {
    #[inline(always)]
    fn default() -> MifDlyLs {
        MifDlyLs(0u64 as u32)
    }
}
impl core::fmt::Debug for MifDlyLs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MifDlyLs")
            .field("pre_hi_dly", &self.pre_hi_dly())
            .field("pre_lo_dly", &self.pre_lo_dly())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MifDlyLs {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MifDlyLs {
            pre_hi_dly: u16,
            pre_lo_dly: u16,
        }
        let proxy = MifDlyLs {
            pre_hi_dly: self.pre_hi_dly(),
            pre_lo_dly: self.pre_lo_dly(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "MIF Delay of STDBY"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MifDlyStdby(pub u32);
impl MifDlyStdby {
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
impl Default for MifDlyStdby {
    #[inline(always)]
    fn default() -> MifDlyStdby {
        MifDlyStdby(0u64 as u32)
    }
}
impl core::fmt::Debug for MifDlyStdby {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MifDlyStdby")
            .field("pre_hi_dly", &self.pre_hi_dly())
            .field("pre_lo_dly", &self.pre_lo_dly())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MifDlyStdby {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MifDlyStdby {
            pre_hi_dly: u16,
            pre_lo_dly: u16,
        }
        let proxy = MifDlyStdby {
            pre_hi_dly: self.pre_hi_dly(),
            pre_lo_dly: self.pre_lo_dly(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "MIF MLPL control of HS"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MifMlplHs(pub u32);
impl MifMlplHs {
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
    #[doc = "software control HS"]
    #[must_use]
    #[inline(always)]
    pub const fn sw_hs(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "software control HS"]
    #[inline(always)]
    pub const fn set_sw_hs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for MifMlplHs {
    #[inline(always)]
    fn default() -> MifMlplHs {
        MifMlplHs(2u64 as u32)
    }
}
impl core::fmt::Debug for MifMlplHs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MifMlplHs")
            .field("mlpl_ctrl", &self.mlpl_ctrl())
            .field("sw_hs", &self.sw_hs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MifMlplHs {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MifMlplHs {
            mlpl_ctrl: u8,
            sw_hs: bool,
        }
        let proxy = MifMlplHs {
            mlpl_ctrl: self.mlpl_ctrl(),
            sw_hs: self.sw_hs(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "MIF MLPL control of Input Gating (IG)"]
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
#[doc = "MIF MLPL control of LS"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MifMlplLs(pub u32);
impl MifMlplLs {
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
    #[doc = "Software control LS"]
    #[must_use]
    #[inline(always)]
    pub const fn sw_ls(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Software control LS"]
    #[inline(always)]
    pub const fn set_sw_ls(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for MifMlplLs {
    #[inline(always)]
    fn default() -> MifMlplLs {
        MifMlplLs(0u64 as u32)
    }
}
impl core::fmt::Debug for MifMlplLs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MifMlplLs")
            .field("mlpl_ctrl", &self.mlpl_ctrl())
            .field("sw_ls", &self.sw_ls())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MifMlplLs {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MifMlplLs {
            mlpl_ctrl: u8,
            sw_ls: bool,
        }
        let proxy = MifMlplLs {
            mlpl_ctrl: self.mlpl_ctrl(),
            sw_ls: self.sw_ls(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "MIF MLPL control of STDBY"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MifMlplStdby(pub u32);
impl MifMlplStdby {
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
    #[doc = "Software control STDBY"]
    #[must_use]
    #[inline(always)]
    pub const fn sw_stdby(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Software control STDBY"]
    #[inline(always)]
    pub const fn set_sw_stdby(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for MifMlplStdby {
    #[inline(always)]
    fn default() -> MifMlplStdby {
        MifMlplStdby(0u64 as u32)
    }
}
impl core::fmt::Debug for MifMlplStdby {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MifMlplStdby")
            .field("mlpl_ctrl", &self.mlpl_ctrl())
            .field("sw_stdby", &self.sw_stdby())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MifMlplStdby {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MifMlplStdby {
            mlpl_ctrl: u8,
            sw_stdby: bool,
        }
        let proxy = MifMlplStdby {
            mlpl_ctrl: self.mlpl_ctrl(),
            sw_stdby: self.sw_stdby(),
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
    #[doc = "Current state of LS"]
    #[must_use]
    #[inline(always)]
    pub const fn ls_state(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Current state of LS"]
    #[inline(always)]
    pub const fn set_ls_state(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Current state of HS"]
    #[must_use]
    #[inline(always)]
    pub const fn hs_state(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Current state of HS"]
    #[inline(always)]
    pub const fn set_hs_state(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Current state of IG"]
    #[must_use]
    #[inline(always)]
    pub const fn ig_state(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Current state of IG"]
    #[inline(always)]
    pub const fn set_ig_state(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Current state of STDBY"]
    #[must_use]
    #[inline(always)]
    pub const fn stdby_state(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Current state of STDBY"]
    #[inline(always)]
    pub const fn set_stdby_state(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for MifStat {
    #[inline(always)]
    fn default() -> MifStat {
        MifStat(0u64 as u32)
    }
}
impl core::fmt::Debug for MifStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MifStat")
            .field("mlpl_state", &self.mlpl_state())
            .field("ls_state", &self.ls_state())
            .field("hs_state", &self.hs_state())
            .field("ig_state", &self.ig_state())
            .field("stdby_state", &self.stdby_state())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MifStat {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MifStat {
            mlpl_state: u8,
            ls_state: bool,
            hs_state: bool,
            ig_state: bool,
            stdby_state: bool,
        }
        let proxy = MifStat {
            mlpl_state: self.mlpl_state(),
            ls_state: self.ls_state(),
            hs_state: self.hs_state(),
            ig_state: self.ig_state(),
            stdby_state: self.stdby_state(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
