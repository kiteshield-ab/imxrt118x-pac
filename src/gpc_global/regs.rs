#[doc = "GPC Global Authentication Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AuthenCtrl(pub u32);
impl AuthenCtrl {
    #[doc = "Configuration lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_cfg(&self) -> super::vals::LockCfg {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::LockCfg::from_bits(val as u8)
    }
    #[doc = "Configuration lock"]
    #[inline(always)]
    pub const fn set_lock_cfg(&mut self, val: super::vals::LockCfg) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Allow user mode access"]
    #[must_use]
    #[inline(always)]
    pub const fn user(&self) -> super::vals::User {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::User::from_bits(val as u8)
    }
    #[doc = "Allow user mode access"]
    #[inline(always)]
    pub const fn set_user(&mut self, val: super::vals::User) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Allow non-secure mode access"]
    #[must_use]
    #[inline(always)]
    pub const fn nonsecure(&self) -> super::vals::Nonsecure {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Nonsecure::from_bits(val as u8)
    }
    #[doc = "Allow non-secure mode access"]
    #[inline(always)]
    pub const fn set_nonsecure(&mut self, val: super::vals::Nonsecure) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Lock NONSECURE and USER"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_setting(&self) -> super::vals::LockSetting {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::LockSetting::from_bits(val as u8)
    }
    #[doc = "Lock NONSECURE and USER"]
    #[inline(always)]
    pub const fn set_lock_setting(&mut self, val: super::vals::LockSetting) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "White list lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_list(&self) -> super::vals::LockList {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::LockList::from_bits(val as u8)
    }
    #[doc = "White list lock"]
    #[inline(always)]
    pub const fn set_lock_list(&mut self, val: super::vals::LockList) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Domain ID white list"]
    #[must_use]
    #[inline(always)]
    pub const fn white_list(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Domain ID white list"]
    #[inline(always)]
    pub const fn set_white_list(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
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
            .field("user", &self.user())
            .field("nonsecure", &self.nonsecure())
            .field("lock_setting", &self.lock_setting())
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
            lock_cfg: super::vals::LockCfg,
            user: super::vals::User,
            nonsecure: super::vals::Nonsecure,
            lock_setting: super::vals::LockSetting,
            lock_list: super::vals::LockList,
            white_list: u16,
        }
        let proxy = AuthenCtrl {
            lock_cfg: self.lock_cfg(),
            user: self.user(),
            nonsecure: self.nonsecure(),
            lock_setting: self.lock_setting(),
            lock_list: self.lock_list(),
            white_list: self.white_list(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "GPC CPU0 domain assignment"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpcCpu0Domain(pub u32);
impl GpcCpu0Domain {
    #[doc = "CPU0 domain assignment"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_domain(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "CPU0 domain assignment"]
    #[inline(always)]
    pub const fn set_cpu0_domain(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for GpcCpu0Domain {
    #[inline(always)]
    fn default() -> GpcCpu0Domain {
        GpcCpu0Domain(0u64 as u32)
    }
}
impl core::fmt::Debug for GpcCpu0Domain {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GpcCpu0Domain")
            .field("cpu0_domain", &self.cpu0_domain())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GpcCpu0Domain {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct GpcCpu0Domain {
            cpu0_domain: u16,
        }
        let proxy = GpcCpu0Domain {
            cpu0_domain: self.cpu0_domain(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "GPC CPU1 domain assignment"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpcCpu1Domain(pub u32);
impl GpcCpu1Domain {
    #[doc = "CPU1 domain assignment"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1_domain(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "CPU1 domain assignment"]
    #[inline(always)]
    pub const fn set_cpu1_domain(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for GpcCpu1Domain {
    #[inline(always)]
    fn default() -> GpcCpu1Domain {
        GpcCpu1Domain(0u64 as u32)
    }
}
impl core::fmt::Debug for GpcCpu1Domain {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GpcCpu1Domain")
            .field("cpu1_domain", &self.cpu1_domain())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GpcCpu1Domain {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct GpcCpu1Domain {
            cpu1_domain: u16,
        }
        let proxy = GpcCpu1Domain {
            cpu1_domain: self.cpu1_domain(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "GPC master CPU configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpcMaster(pub u32);
impl GpcMaster {
    #[doc = "Setting to 1 means CPU0 is the master CPU of its domain"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_master(&self) -> super::vals::Cpu0Master {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Cpu0Master::from_bits(val as u8)
    }
    #[doc = "Setting to 1 means CPU0 is the master CPU of its domain"]
    #[inline(always)]
    pub const fn set_cpu0_master(&mut self, val: super::vals::Cpu0Master) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Setting to 1 means CPU1 is the master CPU of its domain"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1_master(&self) -> super::vals::Cpu1Master {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Cpu1Master::from_bits(val as u8)
    }
    #[doc = "Setting to 1 means CPU1 is the master CPU of its domain"]
    #[inline(always)]
    pub const fn set_cpu1_master(&mut self, val: super::vals::Cpu1Master) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for GpcMaster {
    #[inline(always)]
    fn default() -> GpcMaster {
        GpcMaster(0u64 as u32)
    }
}
impl core::fmt::Debug for GpcMaster {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GpcMaster")
            .field("cpu0_master", &self.cpu0_master())
            .field("cpu1_master", &self.cpu1_master())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GpcMaster {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct GpcMaster {
            cpu0_master: super::vals::Cpu0Master,
            cpu1_master: super::vals::Cpu1Master,
        }
        let proxy = GpcMaster {
            cpu0_master: self.cpu0_master(),
            cpu1_master: self.cpu1_master(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "RCOSC control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpcRoscCtrl(pub u32);
impl GpcRoscCtrl {
    #[doc = "Shut off the 24 MHz RCOSC clock when system sleep"]
    #[must_use]
    #[inline(always)]
    pub const fn rosc_off_en(&self) -> super::vals::RoscOffEn {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RoscOffEn::from_bits(val as u8)
    }
    #[doc = "Shut off the 24 MHz RCOSC clock when system sleep"]
    #[inline(always)]
    pub const fn set_rosc_off_en(&mut self, val: super::vals::RoscOffEn) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for GpcRoscCtrl {
    #[inline(always)]
    fn default() -> GpcRoscCtrl {
        GpcRoscCtrl(0u64 as u32)
    }
}
impl core::fmt::Debug for GpcRoscCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GpcRoscCtrl")
            .field("rosc_off_en", &self.rosc_off_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GpcRoscCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct GpcRoscCtrl {
            rosc_off_en: super::vals::RoscOffEn,
        }
        let proxy = GpcRoscCtrl {
            rosc_off_en: self.rosc_off_en(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
