#[doc = "Authentication Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AuthenCtrl(pub u32);
impl AuthenCtrl {
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
    #[doc = "Lock Trust Zone Non Secure(TZ_NS) and Trust Zone User(TZ_USER) bits"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_tz(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Lock Trust Zone Non Secure(TZ_NS) and Trust Zone User(TZ_USER) bits"]
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
            lock_cfg: bool,
            tz_user: bool,
            tz_ns: bool,
            lock_tz: bool,
            lock_list: bool,
            white_list: super::vals::WhiteList,
        }
        let proxy = AuthenCtrl {
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
#[doc = "SRC Boot Mode Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sbmr1(pub u32);
impl Sbmr1 {
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn boot_cfg1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_boot_cfg1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn boot_cfg2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_boot_cfg2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn boot_cfg3(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_boot_cfg3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn boot_cfg4(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_boot_cfg4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Sbmr1 {
    #[inline(always)]
    fn default() -> Sbmr1 {
        Sbmr1(0u64 as u32)
    }
}
impl core::fmt::Debug for Sbmr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sbmr1")
            .field("boot_cfg1", &self.boot_cfg1())
            .field("boot_cfg2", &self.boot_cfg2())
            .field("boot_cfg3", &self.boot_cfg3())
            .field("boot_cfg4", &self.boot_cfg4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sbmr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sbmr1 {
            boot_cfg1: u8,
            boot_cfg2: u8,
            boot_cfg3: u8,
            boot_cfg4: u8,
        }
        let proxy = Sbmr1 {
            boot_cfg1: self.boot_cfg1(),
            boot_cfg2: self.boot_cfg2(),
            boot_cfg3: self.boot_cfg3(),
            boot_cfg4: self.boot_cfg4(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SRC Boot Mode Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sbmr2(pub u32);
impl Sbmr2 {
    #[doc = "IPP_BOOT_MODE\\[5:4\\] reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn ipp_boot_mode(&self) -> super::vals::IppBootMode {
        let val = (self.0 >> 24usize) & 0x3f;
        super::vals::IppBootMode::from_bits(val as u8)
    }
    #[doc = "IPP_BOOT_MODE\\[5:4\\] reserved"]
    #[inline(always)]
    pub const fn set_ipp_boot_mode(&mut self, val: super::vals::IppBootMode) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val.to_bits() as u32) & 0x3f) << 24usize);
    }
}
impl Default for Sbmr2 {
    #[inline(always)]
    fn default() -> Sbmr2 {
        Sbmr2(134217728u64 as u32)
    }
}
impl core::fmt::Debug for Sbmr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sbmr2")
            .field("ipp_boot_mode", &self.ipp_boot_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sbmr2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sbmr2 {
            ipp_boot_mode: super::vals::IppBootMode,
        }
        let proxy = Sbmr2 {
            ipp_boot_mode: self.ipp_boot_mode(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SRC Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr(pub u32);
impl Scr {
    #[doc = "Boot release M7"]
    #[must_use]
    #[inline(always)]
    pub const fn bt_release_m7(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Boot release M7"]
    #[inline(always)]
    pub const fn set_bt_release_m7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Scr {
    #[inline(always)]
    fn default() -> Scr {
        Scr(0u64 as u32)
    }
}
impl core::fmt::Debug for Scr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scr")
            .field("bt_release_m7", &self.bt_release_m7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Scr {
            bt_release_m7: bool,
        }
        let proxy = Scr {
            bt_release_m7: self.bt_release_m7(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SRC Reset Mask Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srmask(pub u32);
impl Srmask {
    #[doc = "WDOG1 reset mask"]
    #[must_use]
    #[inline(always)]
    pub const fn wdog1_mask(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "WDOG1 reset mask"]
    #[inline(always)]
    pub const fn set_wdog1_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "WDOG2 reset mask"]
    #[must_use]
    #[inline(always)]
    pub const fn wdog2_mask(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "WDOG2 reset mask"]
    #[inline(always)]
    pub const fn set_wdog2_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "WDOG3 reset mask"]
    #[must_use]
    #[inline(always)]
    pub const fn wdog3_mask(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "WDOG3 reset mask"]
    #[inline(always)]
    pub const fn set_wdog3_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "WDOG4 reset mask"]
    #[must_use]
    #[inline(always)]
    pub const fn wdog4_mask(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "WDOG4 reset mask"]
    #[inline(always)]
    pub const fn set_wdog4_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "WDOG5 reset mask"]
    #[must_use]
    #[inline(always)]
    pub const fn wdog5_mask(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "WDOG5 reset mask"]
    #[inline(always)]
    pub const fn set_wdog5_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "TempSense reset mask"]
    #[must_use]
    #[inline(always)]
    pub const fn tempsense_mask(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "TempSense reset mask"]
    #[inline(always)]
    pub const fn set_tempsense_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Edgelock reset mask"]
    #[must_use]
    #[inline(always)]
    pub const fn edgelock_mask(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Edgelock reset mask"]
    #[inline(always)]
    pub const fn set_edgelock_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "JTAGSW reset mask"]
    #[must_use]
    #[inline(always)]
    pub const fn jtagsw_mask(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "JTAGSW reset mask"]
    #[inline(always)]
    pub const fn set_jtagsw_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "CM33 reset mask"]
    #[must_use]
    #[inline(always)]
    pub const fn cm33_reset_mask(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "CM33 reset mask"]
    #[inline(always)]
    pub const fn set_cm33_reset_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "CM33 lockup mask"]
    #[must_use]
    #[inline(always)]
    pub const fn cm33_lockup_mask(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "CM33 lockup mask"]
    #[inline(always)]
    pub const fn set_cm33_lockup_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CM7 reset mask"]
    #[must_use]
    #[inline(always)]
    pub const fn cm7_reset_mask(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "CM7 reset mask"]
    #[inline(always)]
    pub const fn set_cm7_reset_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "CM7 lockup reset mask"]
    #[must_use]
    #[inline(always)]
    pub const fn cm7_lockup_mask(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "CM7 lockup reset mask"]
    #[inline(always)]
    pub const fn set_cm7_lockup_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "DCDC over voltage mask"]
    #[must_use]
    #[inline(always)]
    pub const fn dcdc_ovvt_mask(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "DCDC over voltage mask"]
    #[inline(always)]
    pub const fn set_dcdc_ovvt_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "ECAT reset output mask"]
    #[must_use]
    #[inline(always)]
    pub const fn ecat_rsto_mask(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "ECAT reset output mask"]
    #[inline(always)]
    pub const fn set_ecat_rsto_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock WDOG1_MASK"]
    #[must_use]
    #[inline(always)]
    pub const fn wdog1_mask_locked(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Lock WDOG1_MASK"]
    #[inline(always)]
    pub const fn set_wdog1_mask_locked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Lock WDOG2_MASK"]
    #[must_use]
    #[inline(always)]
    pub const fn wdog2_mask_locked(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Lock WDOG2_MASK"]
    #[inline(always)]
    pub const fn set_wdog2_mask_locked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Lock WDOG3_MASK"]
    #[must_use]
    #[inline(always)]
    pub const fn wdog3_mask_locked(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Lock WDOG3_MASK"]
    #[inline(always)]
    pub const fn set_wdog3_mask_locked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Lock WDOG4_MASK"]
    #[must_use]
    #[inline(always)]
    pub const fn wdog4_mask_locked(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Lock WDOG4_MASK"]
    #[inline(always)]
    pub const fn set_wdog4_mask_locked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Lock WDOG5_MASK"]
    #[must_use]
    #[inline(always)]
    pub const fn wdog5_mask_locked(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Lock WDOG5_MASK"]
    #[inline(always)]
    pub const fn set_wdog5_mask_locked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Lock TEMPSENSE_MASK"]
    #[must_use]
    #[inline(always)]
    pub const fn tempsense_mask_locked(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Lock TEMPSENSE_MASK"]
    #[inline(always)]
    pub const fn set_tempsense_mask_locked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Lock EDGELOCK_MASK"]
    #[must_use]
    #[inline(always)]
    pub const fn edgelock_mask_locked(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Lock EDGELOCK_MASK"]
    #[inline(always)]
    pub const fn set_edgelock_mask_locked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Lock JTAGSW_MASK"]
    #[must_use]
    #[inline(always)]
    pub const fn jtagsw_mask_locked(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Lock JTAGSW_MASK"]
    #[inline(always)]
    pub const fn set_jtagsw_mask_locked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Lock CM33_RESET_MASK"]
    #[must_use]
    #[inline(always)]
    pub const fn cm33_reset_mask_locked(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Lock CM33_RESET_MASK"]
    #[inline(always)]
    pub const fn set_cm33_reset_mask_locked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Lock CM33_LOCKUP_MASK"]
    #[must_use]
    #[inline(always)]
    pub const fn cm33_lockup_mask_locked(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Lock CM33_LOCKUP_MASK"]
    #[inline(always)]
    pub const fn set_cm33_lockup_mask_locked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Lock CM7 reset mask bit"]
    #[must_use]
    #[inline(always)]
    pub const fn cm7_reset_mask_locked(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Lock CM7 reset mask bit"]
    #[inline(always)]
    pub const fn set_cm7_reset_mask_locked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Lock CM7_LOCKUP_MASK"]
    #[must_use]
    #[inline(always)]
    pub const fn cm7_lockup_mask_locked(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Lock CM7_LOCKUP_MASK"]
    #[inline(always)]
    pub const fn set_cm7_lockup_mask_locked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Lock DCDC_OVVT_MASK"]
    #[must_use]
    #[inline(always)]
    pub const fn dcdc_ovvt_mask_locked(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Lock DCDC_OVVT_MASK"]
    #[inline(always)]
    pub const fn set_dcdc_ovvt_mask_locked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Lock ECAT_RSTO_MASK"]
    #[must_use]
    #[inline(always)]
    pub const fn ecat_rsto_mask_locked(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Lock ECAT_RSTO_MASK"]
    #[inline(always)]
    pub const fn set_ecat_rsto_mask_locked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for Srmask {
    #[inline(always)]
    fn default() -> Srmask {
        Srmask(16319u64 as u32)
    }
}
impl core::fmt::Debug for Srmask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Srmask")
            .field("wdog1_mask", &self.wdog1_mask())
            .field("wdog2_mask", &self.wdog2_mask())
            .field("wdog3_mask", &self.wdog3_mask())
            .field("wdog4_mask", &self.wdog4_mask())
            .field("wdog5_mask", &self.wdog5_mask())
            .field("tempsense_mask", &self.tempsense_mask())
            .field("edgelock_mask", &self.edgelock_mask())
            .field("jtagsw_mask", &self.jtagsw_mask())
            .field("cm33_reset_mask", &self.cm33_reset_mask())
            .field("cm33_lockup_mask", &self.cm33_lockup_mask())
            .field("cm7_reset_mask", &self.cm7_reset_mask())
            .field("cm7_lockup_mask", &self.cm7_lockup_mask())
            .field("dcdc_ovvt_mask", &self.dcdc_ovvt_mask())
            .field("ecat_rsto_mask", &self.ecat_rsto_mask())
            .field("wdog1_mask_locked", &self.wdog1_mask_locked())
            .field("wdog2_mask_locked", &self.wdog2_mask_locked())
            .field("wdog3_mask_locked", &self.wdog3_mask_locked())
            .field("wdog4_mask_locked", &self.wdog4_mask_locked())
            .field("wdog5_mask_locked", &self.wdog5_mask_locked())
            .field("tempsense_mask_locked", &self.tempsense_mask_locked())
            .field("edgelock_mask_locked", &self.edgelock_mask_locked())
            .field("jtagsw_mask_locked", &self.jtagsw_mask_locked())
            .field("cm33_reset_mask_locked", &self.cm33_reset_mask_locked())
            .field("cm33_lockup_mask_locked", &self.cm33_lockup_mask_locked())
            .field("cm7_reset_mask_locked", &self.cm7_reset_mask_locked())
            .field("cm7_lockup_mask_locked", &self.cm7_lockup_mask_locked())
            .field("dcdc_ovvt_mask_locked", &self.dcdc_ovvt_mask_locked())
            .field("ecat_rsto_mask_locked", &self.ecat_rsto_mask_locked())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Srmask {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Srmask {
            wdog1_mask: bool,
            wdog2_mask: bool,
            wdog3_mask: bool,
            wdog4_mask: bool,
            wdog5_mask: bool,
            tempsense_mask: bool,
            edgelock_mask: bool,
            jtagsw_mask: bool,
            cm33_reset_mask: bool,
            cm33_lockup_mask: bool,
            cm7_reset_mask: bool,
            cm7_lockup_mask: bool,
            dcdc_ovvt_mask: bool,
            ecat_rsto_mask: bool,
            wdog1_mask_locked: bool,
            wdog2_mask_locked: bool,
            wdog3_mask_locked: bool,
            wdog4_mask_locked: bool,
            wdog5_mask_locked: bool,
            tempsense_mask_locked: bool,
            edgelock_mask_locked: bool,
            jtagsw_mask_locked: bool,
            cm33_reset_mask_locked: bool,
            cm33_lockup_mask_locked: bool,
            cm7_reset_mask_locked: bool,
            cm7_lockup_mask_locked: bool,
            dcdc_ovvt_mask_locked: bool,
            ecat_rsto_mask_locked: bool,
        }
        let proxy = Srmask {
            wdog1_mask: self.wdog1_mask(),
            wdog2_mask: self.wdog2_mask(),
            wdog3_mask: self.wdog3_mask(),
            wdog4_mask: self.wdog4_mask(),
            wdog5_mask: self.wdog5_mask(),
            tempsense_mask: self.tempsense_mask(),
            edgelock_mask: self.edgelock_mask(),
            jtagsw_mask: self.jtagsw_mask(),
            cm33_reset_mask: self.cm33_reset_mask(),
            cm33_lockup_mask: self.cm33_lockup_mask(),
            cm7_reset_mask: self.cm7_reset_mask(),
            cm7_lockup_mask: self.cm7_lockup_mask(),
            dcdc_ovvt_mask: self.dcdc_ovvt_mask(),
            ecat_rsto_mask: self.ecat_rsto_mask(),
            wdog1_mask_locked: self.wdog1_mask_locked(),
            wdog2_mask_locked: self.wdog2_mask_locked(),
            wdog3_mask_locked: self.wdog3_mask_locked(),
            wdog4_mask_locked: self.wdog4_mask_locked(),
            wdog5_mask_locked: self.wdog5_mask_locked(),
            tempsense_mask_locked: self.tempsense_mask_locked(),
            edgelock_mask_locked: self.edgelock_mask_locked(),
            jtagsw_mask_locked: self.jtagsw_mask_locked(),
            cm33_reset_mask_locked: self.cm33_reset_mask_locked(),
            cm33_lockup_mask_locked: self.cm33_lockup_mask_locked(),
            cm7_reset_mask_locked: self.cm7_reset_mask_locked(),
            cm7_lockup_mask_locked: self.cm7_lockup_mask_locked(),
            dcdc_ovvt_mask_locked: self.dcdc_ovvt_mask_locked(),
            ecat_rsto_mask_locked: self.ecat_rsto_mask_locked(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SRC Reset Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srsr(pub u32);
impl Srsr {
    #[doc = "Indicates whether the reset was the result of POR."]
    #[must_use]
    #[inline(always)]
    pub const fn por_rst(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether the reset was the result of POR."]
    #[inline(always)]
    pub const fn set_por_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Time-out reset. Indicates whether the reset was the result of the watchdog1 time-out event."]
    #[must_use]
    #[inline(always)]
    pub const fn wdog1_rst_b(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Time-out reset. Indicates whether the reset was the result of the watchdog1 time-out event."]
    #[inline(always)]
    pub const fn set_wdog1_rst_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Time-out reset. Indicates whether the reset was the result of the watchdog2 time-out event."]
    #[must_use]
    #[inline(always)]
    pub const fn wdog2_rst_b(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Time-out reset. Indicates whether the reset was the result of the watchdog2 time-out event."]
    #[inline(always)]
    pub const fn set_wdog2_rst_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Time-out reset. Indicates whether the reset was the result of the watchdog3 time-out"]
    #[must_use]
    #[inline(always)]
    pub const fn wdog3_rst_b(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Time-out reset. Indicates whether the reset was the result of the watchdog3 time-out"]
    #[inline(always)]
    pub const fn set_wdog3_rst_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Time-out reset. Indicates whether the reset was the result of the watchdog4 time-out"]
    #[must_use]
    #[inline(always)]
    pub const fn wdog4_rst_b(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Time-out reset. Indicates whether the reset was the result of the watchdog4 time-out"]
    #[inline(always)]
    pub const fn set_wdog4_rst_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Time-out reset. Indicates whether the reset was the result of the watchdog5 time-out"]
    #[must_use]
    #[inline(always)]
    pub const fn wdog5_rst_b(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Time-out reset. Indicates whether the reset was the result of the watchdog5 time-out"]
    #[inline(always)]
    pub const fn set_wdog5_rst_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Temper Sensor software reset. Indicates whether the reset was the result of software reset from on-chip Temperature Sensor."]
    #[must_use]
    #[inline(always)]
    pub const fn tempsense_rst_b(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Temper Sensor software reset. Indicates whether the reset was the result of software reset from on-chip Temperature Sensor."]
    #[inline(always)]
    pub const fn set_tempsense_rst_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Indicates whether the reset was the result of the Edgelock's reset input."]
    #[must_use]
    #[inline(always)]
    pub const fn edgelock_reset_b(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether the reset was the result of the Edgelock's reset input."]
    #[inline(always)]
    pub const fn set_edgelock_reset_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "JTAG software reset. Indicates whether the reset was the result of software reset from JTAG."]
    #[must_use]
    #[inline(always)]
    pub const fn jtag_sw_rst(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "JTAG software reset. Indicates whether the reset was the result of software reset from JTAG."]
    #[inline(always)]
    pub const fn set_jtag_sw_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Indicates whether reset was the result of cm33 reset request"]
    #[must_use]
    #[inline(always)]
    pub const fn cm33_request(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether reset was the result of cm33 reset request"]
    #[inline(always)]
    pub const fn set_cm33_request(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Indicates a reset has been caused by cm33 CPU lockup"]
    #[must_use]
    #[inline(always)]
    pub const fn cm33_lockup(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates a reset has been caused by cm33 CPU lockup"]
    #[inline(always)]
    pub const fn set_cm33_lockup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Indicates whether reset was the result of cm7 reset request"]
    #[must_use]
    #[inline(always)]
    pub const fn cm7_request(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether reset was the result of cm7 reset request"]
    #[inline(always)]
    pub const fn set_cm7_request(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Indicates a reset has been caused by CM7 CPU"]
    #[must_use]
    #[inline(always)]
    pub const fn cm7_lockup(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates a reset has been caused by CM7 CPU"]
    #[inline(always)]
    pub const fn set_cm7_lockup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Indicates a reset has been caused by DCDC over voltage"]
    #[must_use]
    #[inline(always)]
    pub const fn dcdc_ovvt(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates a reset has been caused by DCDC over voltage"]
    #[inline(always)]
    pub const fn set_dcdc_ovvt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Indicates a reset has been caused by ECAT reset output"]
    #[must_use]
    #[inline(always)]
    pub const fn ecat_rsto(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates a reset has been caused by ECAT reset output"]
    #[inline(always)]
    pub const fn set_ecat_rsto(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Indicates whether the reset was the result of chip PAD POR_B."]
    #[must_use]
    #[inline(always)]
    pub const fn ipp_por_b(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether the reset was the result of chip PAD POR_B."]
    #[inline(always)]
    pub const fn set_ipp_por_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Srsr {
    #[inline(always)]
    fn default() -> Srsr {
        Srsr(1u64 as u32)
    }
}
impl core::fmt::Debug for Srsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Srsr")
            .field("por_rst", &self.por_rst())
            .field("wdog1_rst_b", &self.wdog1_rst_b())
            .field("wdog2_rst_b", &self.wdog2_rst_b())
            .field("wdog3_rst_b", &self.wdog3_rst_b())
            .field("wdog4_rst_b", &self.wdog4_rst_b())
            .field("wdog5_rst_b", &self.wdog5_rst_b())
            .field("tempsense_rst_b", &self.tempsense_rst_b())
            .field("edgelock_reset_b", &self.edgelock_reset_b())
            .field("jtag_sw_rst", &self.jtag_sw_rst())
            .field("cm33_request", &self.cm33_request())
            .field("cm33_lockup", &self.cm33_lockup())
            .field("cm7_request", &self.cm7_request())
            .field("cm7_lockup", &self.cm7_lockup())
            .field("dcdc_ovvt", &self.dcdc_ovvt())
            .field("ecat_rsto", &self.ecat_rsto())
            .field("ipp_por_b", &self.ipp_por_b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Srsr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Srsr {
            por_rst: bool,
            wdog1_rst_b: bool,
            wdog2_rst_b: bool,
            wdog3_rst_b: bool,
            wdog4_rst_b: bool,
            wdog5_rst_b: bool,
            tempsense_rst_b: bool,
            edgelock_reset_b: bool,
            jtag_sw_rst: bool,
            cm33_request: bool,
            cm33_lockup: bool,
            cm7_request: bool,
            cm7_lockup: bool,
            dcdc_ovvt: bool,
            ecat_rsto: bool,
            ipp_por_b: bool,
        }
        let proxy = Srsr {
            por_rst: self.por_rst(),
            wdog1_rst_b: self.wdog1_rst_b(),
            wdog2_rst_b: self.wdog2_rst_b(),
            wdog3_rst_b: self.wdog3_rst_b(),
            wdog4_rst_b: self.wdog4_rst_b(),
            wdog5_rst_b: self.wdog5_rst_b(),
            tempsense_rst_b: self.tempsense_rst_b(),
            edgelock_reset_b: self.edgelock_reset_b(),
            jtag_sw_rst: self.jtag_sw_rst(),
            cm33_request: self.cm33_request(),
            cm33_lockup: self.cm33_lockup(),
            cm7_request: self.cm7_request(),
            cm7_lockup: self.cm7_lockup(),
            dcdc_ovvt: self.dcdc_ovvt(),
            ecat_rsto: self.ecat_rsto(),
            ipp_por_b: self.ipp_por_b(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SRC Reset Status Register backup in BBSM domain"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SrsrBbsm(pub u32);
impl SrsrBbsm {
    #[doc = "Indicates whether the reset was the result of power up or chip PAD POR_B."]
    #[must_use]
    #[inline(always)]
    pub const fn por_rst(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether the reset was the result of power up or chip PAD POR_B."]
    #[inline(always)]
    pub const fn set_por_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Time-out reset. Indicates whether the reset was the result of the watchdog1 time-out event."]
    #[must_use]
    #[inline(always)]
    pub const fn wdog1_rst_b(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Time-out reset. Indicates whether the reset was the result of the watchdog1 time-out event."]
    #[inline(always)]
    pub const fn set_wdog1_rst_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Time-out reset. Indicates whether the reset was the result of the watchdog2 time-out event."]
    #[must_use]
    #[inline(always)]
    pub const fn wdog2_rst_b(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Time-out reset. Indicates whether the reset was the result of the watchdog2 time-out event."]
    #[inline(always)]
    pub const fn set_wdog2_rst_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Time-out reset. Indicates whether the reset was the result of the watchdog3 time-out"]
    #[must_use]
    #[inline(always)]
    pub const fn wdog3_rst_b(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Time-out reset. Indicates whether the reset was the result of the watchdog3 time-out"]
    #[inline(always)]
    pub const fn set_wdog3_rst_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Time-out reset. Indicates whether the reset was the result of the watchdog4 time-out"]
    #[must_use]
    #[inline(always)]
    pub const fn wdog4_rst_b(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Time-out reset. Indicates whether the reset was the result of the watchdog4 time-out"]
    #[inline(always)]
    pub const fn set_wdog4_rst_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Time-out reset. Indicates whether the reset was the result of the watchdog5 time-out"]
    #[must_use]
    #[inline(always)]
    pub const fn wdog5_rst_b(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Time-out reset. Indicates whether the reset was the result of the watchdog5 time-out"]
    #[inline(always)]
    pub const fn set_wdog5_rst_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "TempSensor software reset. Indicates whether the reset was the result of software reset from on-chip Temperature Sensor."]
    #[must_use]
    #[inline(always)]
    pub const fn tempsense_rst_b(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "TempSensor software reset. Indicates whether the reset was the result of software reset from on-chip Temperature Sensor."]
    #[inline(always)]
    pub const fn set_tempsense_rst_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Indicates whether the reset was the result of the Edgelock's reset input."]
    #[must_use]
    #[inline(always)]
    pub const fn edgelock_reset_b(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether the reset was the result of the Edgelock's reset input."]
    #[inline(always)]
    pub const fn set_edgelock_reset_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "JTAG software reset. Indicates whether the reset was the result of software reset from JTAG."]
    #[must_use]
    #[inline(always)]
    pub const fn jtag_sw_rst(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "JTAG software reset. Indicates whether the reset was the result of software reset from JTAG."]
    #[inline(always)]
    pub const fn set_jtag_sw_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Indicates whether reset was the result of cm33 reset request"]
    #[must_use]
    #[inline(always)]
    pub const fn cm33_request(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether reset was the result of cm33 reset request"]
    #[inline(always)]
    pub const fn set_cm33_request(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Indicates a reset has been caused by cm33 CPU lockup"]
    #[must_use]
    #[inline(always)]
    pub const fn cm33_lockup(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates a reset has been caused by cm33 CPU lockup"]
    #[inline(always)]
    pub const fn set_cm33_lockup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Indicates whether reset was the result of cm7 reset request"]
    #[must_use]
    #[inline(always)]
    pub const fn cm7_request(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether reset was the result of cm7 reset request"]
    #[inline(always)]
    pub const fn set_cm7_request(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Indicates a reset has been caused by CM7 CPU"]
    #[must_use]
    #[inline(always)]
    pub const fn cm7_lockup(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates a reset has been caused by CM7 CPU"]
    #[inline(always)]
    pub const fn set_cm7_lockup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Indicates a reset has been caused by DCDC over voltage"]
    #[must_use]
    #[inline(always)]
    pub const fn dcdc_ovvt(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates a reset has been caused by DCDC over voltage"]
    #[inline(always)]
    pub const fn set_dcdc_ovvt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Indicates a reset has been caused by ECAT reset output"]
    #[must_use]
    #[inline(always)]
    pub const fn ecat_rsto(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates a reset has been caused by ECAT reset output"]
    #[inline(always)]
    pub const fn set_ecat_rsto(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for SrsrBbsm {
    #[inline(always)]
    fn default() -> SrsrBbsm {
        SrsrBbsm(1u64 as u32)
    }
}
impl core::fmt::Debug for SrsrBbsm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SrsrBbsm")
            .field("por_rst", &self.por_rst())
            .field("wdog1_rst_b", &self.wdog1_rst_b())
            .field("wdog2_rst_b", &self.wdog2_rst_b())
            .field("wdog3_rst_b", &self.wdog3_rst_b())
            .field("wdog4_rst_b", &self.wdog4_rst_b())
            .field("wdog5_rst_b", &self.wdog5_rst_b())
            .field("tempsense_rst_b", &self.tempsense_rst_b())
            .field("edgelock_reset_b", &self.edgelock_reset_b())
            .field("jtag_sw_rst", &self.jtag_sw_rst())
            .field("cm33_request", &self.cm33_request())
            .field("cm33_lockup", &self.cm33_lockup())
            .field("cm7_request", &self.cm7_request())
            .field("cm7_lockup", &self.cm7_lockup())
            .field("dcdc_ovvt", &self.dcdc_ovvt())
            .field("ecat_rsto", &self.ecat_rsto())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SrsrBbsm {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SrsrBbsm {
            por_rst: bool,
            wdog1_rst_b: bool,
            wdog2_rst_b: bool,
            wdog3_rst_b: bool,
            wdog4_rst_b: bool,
            wdog5_rst_b: bool,
            tempsense_rst_b: bool,
            edgelock_reset_b: bool,
            jtag_sw_rst: bool,
            cm33_request: bool,
            cm33_lockup: bool,
            cm7_request: bool,
            cm7_lockup: bool,
            dcdc_ovvt: bool,
            ecat_rsto: bool,
        }
        let proxy = SrsrBbsm {
            por_rst: self.por_rst(),
            wdog1_rst_b: self.wdog1_rst_b(),
            wdog2_rst_b: self.wdog2_rst_b(),
            wdog3_rst_b: self.wdog3_rst_b(),
            wdog4_rst_b: self.wdog4_rst_b(),
            wdog5_rst_b: self.wdog5_rst_b(),
            tempsense_rst_b: self.tempsense_rst_b(),
            edgelock_reset_b: self.edgelock_reset_b(),
            jtag_sw_rst: self.jtag_sw_rst(),
            cm33_request: self.cm33_request(),
            cm33_lockup: self.cm33_lockup(),
            cm7_request: self.cm7_request(),
            cm7_lockup: self.cm7_lockup(),
            dcdc_ovvt: self.dcdc_ovvt(),
            ecat_rsto: self.ecat_rsto(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SRC Reset Trigger Mode Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srtmr(pub u32);
impl Srtmr {
    #[doc = "Wdog1 reset trigger mode configuration, locked by LOCK_CFG field"]
    #[must_use]
    #[inline(always)]
    pub const fn wdog1_trig_mode(&self) -> super::vals::Wdog1TrigMode {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Wdog1TrigMode::from_bits(val as u8)
    }
    #[doc = "Wdog1 reset trigger mode configuration, locked by LOCK_CFG field"]
    #[inline(always)]
    pub const fn set_wdog1_trig_mode(&mut self, val: super::vals::Wdog1TrigMode) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Wdog2 reset trigger mode configuration, locked by LOCK_CFG field"]
    #[must_use]
    #[inline(always)]
    pub const fn wdog2_trig_mode(&self) -> super::vals::Wdog2TrigMode {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Wdog2TrigMode::from_bits(val as u8)
    }
    #[doc = "Wdog2 reset trigger mode configuration, locked by LOCK_CFG field"]
    #[inline(always)]
    pub const fn set_wdog2_trig_mode(&mut self, val: super::vals::Wdog2TrigMode) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Wdog3 reset trigger mode configuration, locked by LOCK_CFG field"]
    #[must_use]
    #[inline(always)]
    pub const fn wdog3_trig_mode(&self) -> super::vals::Wdog3TrigMode {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Wdog3TrigMode::from_bits(val as u8)
    }
    #[doc = "Wdog3 reset trigger mode configuration, locked by LOCK_CFG field"]
    #[inline(always)]
    pub const fn set_wdog3_trig_mode(&mut self, val: super::vals::Wdog3TrigMode) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Wdog4 reset trigger mode configuration, locked by LOCK_CFG field"]
    #[must_use]
    #[inline(always)]
    pub const fn wdog4_trig_mode(&self) -> super::vals::Wdog4TrigMode {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Wdog4TrigMode::from_bits(val as u8)
    }
    #[doc = "Wdog4 reset trigger mode configuration, locked by LOCK_CFG field"]
    #[inline(always)]
    pub const fn set_wdog4_trig_mode(&mut self, val: super::vals::Wdog4TrigMode) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Wdog5 reset trigger mode configuration, locked by LOCK_CFG field"]
    #[must_use]
    #[inline(always)]
    pub const fn wdog5_trig_mode(&self) -> super::vals::Wdog5TrigMode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Wdog5TrigMode::from_bits(val as u8)
    }
    #[doc = "Wdog5 reset trigger mode configuration, locked by LOCK_CFG field"]
    #[inline(always)]
    pub const fn set_wdog5_trig_mode(&mut self, val: super::vals::Wdog5TrigMode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "TempSense reset trigger mode configuration, locked by LOCK_CFG field"]
    #[must_use]
    #[inline(always)]
    pub const fn tempsense_trig_mode(&self) -> super::vals::TempsenseTrigMode {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::TempsenseTrigMode::from_bits(val as u8)
    }
    #[doc = "TempSense reset trigger mode configuration, locked by LOCK_CFG field"]
    #[inline(always)]
    pub const fn set_tempsense_trig_mode(&mut self, val: super::vals::TempsenseTrigMode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Edgelock reset trigger mode configuration, locked by LOCK_CFG field"]
    #[must_use]
    #[inline(always)]
    pub const fn edgelock_trig_mode(&self) -> super::vals::EdgelockTrigMode {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::EdgelockTrigMode::from_bits(val as u8)
    }
    #[doc = "Edgelock reset trigger mode configuration, locked by LOCK_CFG field"]
    #[inline(always)]
    pub const fn set_edgelock_trig_mode(&mut self, val: super::vals::EdgelockTrigMode) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Jtagsw reset trigger mode configuration, locked by LOCK_CFG field"]
    #[must_use]
    #[inline(always)]
    pub const fn jtagsw_trig_mode(&self) -> super::vals::JtagswTrigMode {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::JtagswTrigMode::from_bits(val as u8)
    }
    #[doc = "Jtagsw reset trigger mode configuration, locked by LOCK_CFG field"]
    #[inline(always)]
    pub const fn set_jtagsw_trig_mode(&mut self, val: super::vals::JtagswTrigMode) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "CM33 reset trigger mode configuration, locked by LOCK_CFG field."]
    #[must_use]
    #[inline(always)]
    pub const fn cm33_reset_trig_mode(&self) -> super::vals::Cm33ResetTrigMode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Cm33ResetTrigMode::from_bits(val as u8)
    }
    #[doc = "CM33 reset trigger mode configuration, locked by LOCK_CFG field."]
    #[inline(always)]
    pub const fn set_cm33_reset_trig_mode(&mut self, val: super::vals::Cm33ResetTrigMode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "CM33 lockup trigger mode configuration, locked by LOCK_CFG field."]
    #[must_use]
    #[inline(always)]
    pub const fn cm33_lockup_trig_mode(&self) -> super::vals::Cm33LockupTrigMode {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Cm33LockupTrigMode::from_bits(val as u8)
    }
    #[doc = "CM33 lockup trigger mode configuration, locked by LOCK_CFG field."]
    #[inline(always)]
    pub const fn set_cm33_lockup_trig_mode(&mut self, val: super::vals::Cm33LockupTrigMode) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "CM7 reset trigger mode configuration, locked by LOCK_CFG field"]
    #[must_use]
    #[inline(always)]
    pub const fn cm7_reset_trig_mode(&self) -> super::vals::Cm7ResetTrigMode {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Cm7ResetTrigMode::from_bits(val as u8)
    }
    #[doc = "CM7 reset trigger mode configuration, locked by LOCK_CFG field"]
    #[inline(always)]
    pub const fn set_cm7_reset_trig_mode(&mut self, val: super::vals::Cm7ResetTrigMode) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "CM7 lockup trigger mode configuration, locked by LOCK_CFG field"]
    #[must_use]
    #[inline(always)]
    pub const fn cm7_lockup_trig_mode(&self) -> super::vals::Cm7LockupTrigMode {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Cm7LockupTrigMode::from_bits(val as u8)
    }
    #[doc = "CM7 lockup trigger mode configuration, locked by LOCK_CFG field"]
    #[inline(always)]
    pub const fn set_cm7_lockup_trig_mode(&mut self, val: super::vals::Cm7LockupTrigMode) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "DCDC over voltage trigger mode configuration, locked by LOCK_CFG field"]
    #[must_use]
    #[inline(always)]
    pub const fn dcdc_ovvt_trig_mode(&self) -> super::vals::DcdcOvvtTrigMode {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::DcdcOvvtTrigMode::from_bits(val as u8)
    }
    #[doc = "DCDC over voltage trigger mode configuration, locked by LOCK_CFG field"]
    #[inline(always)]
    pub const fn set_dcdc_ovvt_trig_mode(&mut self, val: super::vals::DcdcOvvtTrigMode) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "ECAT reset output mode configuration, locked by LOCK_CFG field"]
    #[must_use]
    #[inline(always)]
    pub const fn ecat_rsto_trig_mode(&self) -> super::vals::EcatRstoTrigMode {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::EcatRstoTrigMode::from_bits(val as u8)
    }
    #[doc = "ECAT reset output mode configuration, locked by LOCK_CFG field"]
    #[inline(always)]
    pub const fn set_ecat_rsto_trig_mode(&mut self, val: super::vals::EcatRstoTrigMode) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
}
impl Default for Srtmr {
    #[inline(always)]
    fn default() -> Srtmr {
        Srtmr(0u64 as u32)
    }
}
impl core::fmt::Debug for Srtmr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Srtmr")
            .field("wdog1_trig_mode", &self.wdog1_trig_mode())
            .field("wdog2_trig_mode", &self.wdog2_trig_mode())
            .field("wdog3_trig_mode", &self.wdog3_trig_mode())
            .field("wdog4_trig_mode", &self.wdog4_trig_mode())
            .field("wdog5_trig_mode", &self.wdog5_trig_mode())
            .field("tempsense_trig_mode", &self.tempsense_trig_mode())
            .field("edgelock_trig_mode", &self.edgelock_trig_mode())
            .field("jtagsw_trig_mode", &self.jtagsw_trig_mode())
            .field("cm33_reset_trig_mode", &self.cm33_reset_trig_mode())
            .field("cm33_lockup_trig_mode", &self.cm33_lockup_trig_mode())
            .field("cm7_reset_trig_mode", &self.cm7_reset_trig_mode())
            .field("cm7_lockup_trig_mode", &self.cm7_lockup_trig_mode())
            .field("dcdc_ovvt_trig_mode", &self.dcdc_ovvt_trig_mode())
            .field("ecat_rsto_trig_mode", &self.ecat_rsto_trig_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Srtmr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Srtmr {
            wdog1_trig_mode: super::vals::Wdog1TrigMode,
            wdog2_trig_mode: super::vals::Wdog2TrigMode,
            wdog3_trig_mode: super::vals::Wdog3TrigMode,
            wdog4_trig_mode: super::vals::Wdog4TrigMode,
            wdog5_trig_mode: super::vals::Wdog5TrigMode,
            tempsense_trig_mode: super::vals::TempsenseTrigMode,
            edgelock_trig_mode: super::vals::EdgelockTrigMode,
            jtagsw_trig_mode: super::vals::JtagswTrigMode,
            cm33_reset_trig_mode: super::vals::Cm33ResetTrigMode,
            cm33_lockup_trig_mode: super::vals::Cm33LockupTrigMode,
            cm7_reset_trig_mode: super::vals::Cm7ResetTrigMode,
            cm7_lockup_trig_mode: super::vals::Cm7LockupTrigMode,
            dcdc_ovvt_trig_mode: super::vals::DcdcOvvtTrigMode,
            ecat_rsto_trig_mode: super::vals::EcatRstoTrigMode,
        }
        let proxy = Srtmr {
            wdog1_trig_mode: self.wdog1_trig_mode(),
            wdog2_trig_mode: self.wdog2_trig_mode(),
            wdog3_trig_mode: self.wdog3_trig_mode(),
            wdog4_trig_mode: self.wdog4_trig_mode(),
            wdog5_trig_mode: self.wdog5_trig_mode(),
            tempsense_trig_mode: self.tempsense_trig_mode(),
            edgelock_trig_mode: self.edgelock_trig_mode(),
            jtagsw_trig_mode: self.jtagsw_trig_mode(),
            cm33_reset_trig_mode: self.cm33_reset_trig_mode(),
            cm33_lockup_trig_mode: self.cm33_lockup_trig_mode(),
            cm7_reset_trig_mode: self.cm7_reset_trig_mode(),
            cm7_lockup_trig_mode: self.cm7_lockup_trig_mode(),
            dcdc_ovvt_trig_mode: self.dcdc_ovvt_trig_mode(),
            ecat_rsto_trig_mode: self.ecat_rsto_trig_mode(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
