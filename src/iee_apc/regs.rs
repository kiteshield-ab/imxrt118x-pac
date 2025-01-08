#[doc = "Access control for IEE APC registers of region (n)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Region0AccCtl(pub u32);
impl Region0AccCtl {
    #[doc = "Allowed domain ID"]
    #[must_use]
    #[inline(always)]
    pub const fn allow_did(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Allowed domain ID"]
    #[inline(always)]
    pub const fn set_allow_did(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Lock bit for the lower half word"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_l(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit for the lower half word"]
    #[inline(always)]
    pub const fn set_lock_l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Allow nonsecure mode access"]
    #[must_use]
    #[inline(always)]
    pub const fn allow_ns(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Allow nonsecure mode access"]
    #[inline(always)]
    pub const fn set_allow_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Allow user mode access"]
    #[must_use]
    #[inline(always)]
    pub const fn allow_user(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Allow user mode access"]
    #[inline(always)]
    pub const fn set_allow_user(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Lock bit for the higher half word"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_h(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit for the higher half word"]
    #[inline(always)]
    pub const fn set_lock_h(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Region0AccCtl {
    #[inline(always)]
    fn default() -> Region0AccCtl {
        Region0AccCtl(0u64 as u32)
    }
}
impl core::fmt::Debug for Region0AccCtl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Region0AccCtl")
            .field("allow_did", &self.allow_did())
            .field("lock_l", &self.lock_l())
            .field("allow_ns", &self.allow_ns())
            .field("allow_user", &self.allow_user())
            .field("lock_h", &self.lock_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Region0AccCtl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Region0AccCtl {
            allow_did: u8,
            lock_l: bool,
            allow_ns: bool,
            allow_user: bool,
            lock_h: bool,
        }
        let proxy = Region0AccCtl {
            allow_did: self.allow_did(),
            lock_l: self.lock_l(),
            allow_ns: self.allow_ns(),
            allow_user: self.allow_user(),
            lock_h: self.lock_h(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Start address of IEE region (n)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Region0BotAddr(pub u32);
impl Region0BotAddr {
    #[doc = "Start address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    #[must_use]
    #[inline(always)]
    pub const fn bot_addr(&self) -> u32 {
        let val = (self.0 >> 6usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "Start address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    #[inline(always)]
    pub const fn set_bot_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 6usize)) | (((val as u32) & 0x03ff_ffff) << 6usize);
    }
}
impl Default for Region0BotAddr {
    #[inline(always)]
    fn default() -> Region0BotAddr {
        Region0BotAddr(0u64 as u32)
    }
}
impl core::fmt::Debug for Region0BotAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Region0BotAddr")
            .field("bot_addr", &self.bot_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Region0BotAddr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Region0BotAddr {
            bot_addr: u32,
        }
        let proxy = Region0BotAddr {
            bot_addr: self.bot_addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Region enable for region (n)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Region0Ena(pub u32);
impl Region0Ena {
    #[doc = "Enable this region"]
    #[must_use]
    #[inline(always)]
    pub const fn encrypt_enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable this region"]
    #[inline(always)]
    pub const fn set_encrypt_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Region0Ena {
    #[inline(always)]
    fn default() -> Region0Ena {
        Region0Ena(0u64 as u32)
    }
}
impl core::fmt::Debug for Region0Ena {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Region0Ena")
            .field("encrypt_enable", &self.encrypt_enable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Region0Ena {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Region0Ena {
            encrypt_enable: bool,
        }
        let proxy = Region0Ena {
            encrypt_enable: self.encrypt_enable(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "End address of IEE region (n)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Region0TopAddr(pub u32);
impl Region0TopAddr {
    #[doc = "End address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    #[must_use]
    #[inline(always)]
    pub const fn top_addr(&self) -> u32 {
        let val = (self.0 >> 6usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "End address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    #[inline(always)]
    pub const fn set_top_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 6usize)) | (((val as u32) & 0x03ff_ffff) << 6usize);
    }
}
impl Default for Region0TopAddr {
    #[inline(always)]
    fn default() -> Region0TopAddr {
        Region0TopAddr(0u64 as u32)
    }
}
impl core::fmt::Debug for Region0TopAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Region0TopAddr")
            .field("top_addr", &self.top_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Region0TopAddr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Region0TopAddr {
            top_addr: u32,
        }
        let proxy = Region0TopAddr {
            top_addr: self.top_addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Access control for IEE APC registers of region (n)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Region1AccCtl(pub u32);
impl Region1AccCtl {
    #[doc = "Allowed domain ID"]
    #[must_use]
    #[inline(always)]
    pub const fn allow_did(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Allowed domain ID"]
    #[inline(always)]
    pub const fn set_allow_did(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Lock bit for the lower half word"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_l(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit for the lower half word"]
    #[inline(always)]
    pub const fn set_lock_l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Allow nonsecure mode access"]
    #[must_use]
    #[inline(always)]
    pub const fn allow_ns(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Allow nonsecure mode access"]
    #[inline(always)]
    pub const fn set_allow_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Allow user mode access"]
    #[must_use]
    #[inline(always)]
    pub const fn allow_user(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Allow user mode access"]
    #[inline(always)]
    pub const fn set_allow_user(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Lock bit for the higher half word"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_h(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit for the higher half word"]
    #[inline(always)]
    pub const fn set_lock_h(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Region1AccCtl {
    #[inline(always)]
    fn default() -> Region1AccCtl {
        Region1AccCtl(0u64 as u32)
    }
}
impl core::fmt::Debug for Region1AccCtl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Region1AccCtl")
            .field("allow_did", &self.allow_did())
            .field("lock_l", &self.lock_l())
            .field("allow_ns", &self.allow_ns())
            .field("allow_user", &self.allow_user())
            .field("lock_h", &self.lock_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Region1AccCtl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Region1AccCtl {
            allow_did: u8,
            lock_l: bool,
            allow_ns: bool,
            allow_user: bool,
            lock_h: bool,
        }
        let proxy = Region1AccCtl {
            allow_did: self.allow_did(),
            lock_l: self.lock_l(),
            allow_ns: self.allow_ns(),
            allow_user: self.allow_user(),
            lock_h: self.lock_h(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Start address of IEE region (n)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Region1BotAddr(pub u32);
impl Region1BotAddr {
    #[doc = "Start address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    #[must_use]
    #[inline(always)]
    pub const fn bot_addr(&self) -> u32 {
        let val = (self.0 >> 6usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "Start address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    #[inline(always)]
    pub const fn set_bot_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 6usize)) | (((val as u32) & 0x03ff_ffff) << 6usize);
    }
}
impl Default for Region1BotAddr {
    #[inline(always)]
    fn default() -> Region1BotAddr {
        Region1BotAddr(0u64 as u32)
    }
}
impl core::fmt::Debug for Region1BotAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Region1BotAddr")
            .field("bot_addr", &self.bot_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Region1BotAddr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Region1BotAddr {
            bot_addr: u32,
        }
        let proxy = Region1BotAddr {
            bot_addr: self.bot_addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Region enable for region (n)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Region1Ena(pub u32);
impl Region1Ena {
    #[doc = "Enable this region"]
    #[must_use]
    #[inline(always)]
    pub const fn encrypt_enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable this region"]
    #[inline(always)]
    pub const fn set_encrypt_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Region1Ena {
    #[inline(always)]
    fn default() -> Region1Ena {
        Region1Ena(0u64 as u32)
    }
}
impl core::fmt::Debug for Region1Ena {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Region1Ena")
            .field("encrypt_enable", &self.encrypt_enable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Region1Ena {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Region1Ena {
            encrypt_enable: bool,
        }
        let proxy = Region1Ena {
            encrypt_enable: self.encrypt_enable(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "End address of IEE region (n)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Region1TopAddr(pub u32);
impl Region1TopAddr {
    #[doc = "End address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    #[must_use]
    #[inline(always)]
    pub const fn top_addr(&self) -> u32 {
        let val = (self.0 >> 6usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "End address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    #[inline(always)]
    pub const fn set_top_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 6usize)) | (((val as u32) & 0x03ff_ffff) << 6usize);
    }
}
impl Default for Region1TopAddr {
    #[inline(always)]
    fn default() -> Region1TopAddr {
        Region1TopAddr(0u64 as u32)
    }
}
impl core::fmt::Debug for Region1TopAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Region1TopAddr")
            .field("top_addr", &self.top_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Region1TopAddr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Region1TopAddr {
            top_addr: u32,
        }
        let proxy = Region1TopAddr {
            top_addr: self.top_addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Access control for IEE APC registers of region (n)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Region2AccCtl(pub u32);
impl Region2AccCtl {
    #[doc = "Allowed domain ID"]
    #[must_use]
    #[inline(always)]
    pub const fn allow_did(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Allowed domain ID"]
    #[inline(always)]
    pub const fn set_allow_did(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Lock bit for the lower half word"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_l(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit for the lower half word"]
    #[inline(always)]
    pub const fn set_lock_l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Allow nonsecure mode access"]
    #[must_use]
    #[inline(always)]
    pub const fn allow_ns(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Allow nonsecure mode access"]
    #[inline(always)]
    pub const fn set_allow_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Allow user mode access"]
    #[must_use]
    #[inline(always)]
    pub const fn allow_user(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Allow user mode access"]
    #[inline(always)]
    pub const fn set_allow_user(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Lock bit for the higher half word"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_h(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit for the higher half word"]
    #[inline(always)]
    pub const fn set_lock_h(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Region2AccCtl {
    #[inline(always)]
    fn default() -> Region2AccCtl {
        Region2AccCtl(0u64 as u32)
    }
}
impl core::fmt::Debug for Region2AccCtl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Region2AccCtl")
            .field("allow_did", &self.allow_did())
            .field("lock_l", &self.lock_l())
            .field("allow_ns", &self.allow_ns())
            .field("allow_user", &self.allow_user())
            .field("lock_h", &self.lock_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Region2AccCtl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Region2AccCtl {
            allow_did: u8,
            lock_l: bool,
            allow_ns: bool,
            allow_user: bool,
            lock_h: bool,
        }
        let proxy = Region2AccCtl {
            allow_did: self.allow_did(),
            lock_l: self.lock_l(),
            allow_ns: self.allow_ns(),
            allow_user: self.allow_user(),
            lock_h: self.lock_h(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Start address of IEE region (n)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Region2BotAddr(pub u32);
impl Region2BotAddr {
    #[doc = "Start address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    #[must_use]
    #[inline(always)]
    pub const fn bot_addr(&self) -> u32 {
        let val = (self.0 >> 6usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "Start address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    #[inline(always)]
    pub const fn set_bot_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 6usize)) | (((val as u32) & 0x03ff_ffff) << 6usize);
    }
}
impl Default for Region2BotAddr {
    #[inline(always)]
    fn default() -> Region2BotAddr {
        Region2BotAddr(0u64 as u32)
    }
}
impl core::fmt::Debug for Region2BotAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Region2BotAddr")
            .field("bot_addr", &self.bot_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Region2BotAddr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Region2BotAddr {
            bot_addr: u32,
        }
        let proxy = Region2BotAddr {
            bot_addr: self.bot_addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Region enable for region (n)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Region2Ena(pub u32);
impl Region2Ena {
    #[doc = "Enable this region"]
    #[must_use]
    #[inline(always)]
    pub const fn encrypt_enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable this region"]
    #[inline(always)]
    pub const fn set_encrypt_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Region2Ena {
    #[inline(always)]
    fn default() -> Region2Ena {
        Region2Ena(0u64 as u32)
    }
}
impl core::fmt::Debug for Region2Ena {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Region2Ena")
            .field("encrypt_enable", &self.encrypt_enable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Region2Ena {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Region2Ena {
            encrypt_enable: bool,
        }
        let proxy = Region2Ena {
            encrypt_enable: self.encrypt_enable(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "End address of IEE region (n)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Region2TopAddr(pub u32);
impl Region2TopAddr {
    #[doc = "End address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    #[must_use]
    #[inline(always)]
    pub const fn top_addr(&self) -> u32 {
        let val = (self.0 >> 6usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "End address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    #[inline(always)]
    pub const fn set_top_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 6usize)) | (((val as u32) & 0x03ff_ffff) << 6usize);
    }
}
impl Default for Region2TopAddr {
    #[inline(always)]
    fn default() -> Region2TopAddr {
        Region2TopAddr(0u64 as u32)
    }
}
impl core::fmt::Debug for Region2TopAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Region2TopAddr")
            .field("top_addr", &self.top_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Region2TopAddr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Region2TopAddr {
            top_addr: u32,
        }
        let proxy = Region2TopAddr {
            top_addr: self.top_addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Access control for IEE APC registers of region (n)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Region3AccCtl(pub u32);
impl Region3AccCtl {
    #[doc = "Allowed domain ID"]
    #[must_use]
    #[inline(always)]
    pub const fn allow_did(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Allowed domain ID"]
    #[inline(always)]
    pub const fn set_allow_did(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Lock bit for the lower half word"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_l(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit for the lower half word"]
    #[inline(always)]
    pub const fn set_lock_l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Allow nonsecure mode access"]
    #[must_use]
    #[inline(always)]
    pub const fn allow_ns(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Allow nonsecure mode access"]
    #[inline(always)]
    pub const fn set_allow_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Allow user mode access"]
    #[must_use]
    #[inline(always)]
    pub const fn allow_user(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Allow user mode access"]
    #[inline(always)]
    pub const fn set_allow_user(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Lock bit for the higher half word"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_h(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit for the higher half word"]
    #[inline(always)]
    pub const fn set_lock_h(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Region3AccCtl {
    #[inline(always)]
    fn default() -> Region3AccCtl {
        Region3AccCtl(0u64 as u32)
    }
}
impl core::fmt::Debug for Region3AccCtl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Region3AccCtl")
            .field("allow_did", &self.allow_did())
            .field("lock_l", &self.lock_l())
            .field("allow_ns", &self.allow_ns())
            .field("allow_user", &self.allow_user())
            .field("lock_h", &self.lock_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Region3AccCtl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Region3AccCtl {
            allow_did: u8,
            lock_l: bool,
            allow_ns: bool,
            allow_user: bool,
            lock_h: bool,
        }
        let proxy = Region3AccCtl {
            allow_did: self.allow_did(),
            lock_l: self.lock_l(),
            allow_ns: self.allow_ns(),
            allow_user: self.allow_user(),
            lock_h: self.lock_h(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Start address of IEE region (n)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Region3BotAddr(pub u32);
impl Region3BotAddr {
    #[doc = "Start address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    #[must_use]
    #[inline(always)]
    pub const fn bot_addr(&self) -> u32 {
        let val = (self.0 >> 6usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "Start address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    #[inline(always)]
    pub const fn set_bot_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 6usize)) | (((val as u32) & 0x03ff_ffff) << 6usize);
    }
}
impl Default for Region3BotAddr {
    #[inline(always)]
    fn default() -> Region3BotAddr {
        Region3BotAddr(0u64 as u32)
    }
}
impl core::fmt::Debug for Region3BotAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Region3BotAddr")
            .field("bot_addr", &self.bot_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Region3BotAddr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Region3BotAddr {
            bot_addr: u32,
        }
        let proxy = Region3BotAddr {
            bot_addr: self.bot_addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Region enable for region (n)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Region3Ena(pub u32);
impl Region3Ena {
    #[doc = "Enable this region"]
    #[must_use]
    #[inline(always)]
    pub const fn encrypt_enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable this region"]
    #[inline(always)]
    pub const fn set_encrypt_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Region3Ena {
    #[inline(always)]
    fn default() -> Region3Ena {
        Region3Ena(0u64 as u32)
    }
}
impl core::fmt::Debug for Region3Ena {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Region3Ena")
            .field("encrypt_enable", &self.encrypt_enable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Region3Ena {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Region3Ena {
            encrypt_enable: bool,
        }
        let proxy = Region3Ena {
            encrypt_enable: self.encrypt_enable(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "End address of IEE region (n)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Region3TopAddr(pub u32);
impl Region3TopAddr {
    #[doc = "End address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    #[must_use]
    #[inline(always)]
    pub const fn top_addr(&self) -> u32 {
        let val = (self.0 >> 6usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "End address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    #[inline(always)]
    pub const fn set_top_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 6usize)) | (((val as u32) & 0x03ff_ffff) << 6usize);
    }
}
impl Default for Region3TopAddr {
    #[inline(always)]
    fn default() -> Region3TopAddr {
        Region3TopAddr(0u64 as u32)
    }
}
impl core::fmt::Debug for Region3TopAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Region3TopAddr")
            .field("top_addr", &self.top_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Region3TopAddr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Region3TopAddr {
            top_addr: u32,
        }
        let proxy = Region3TopAddr {
            top_addr: self.top_addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Access control for IEE APC registers of region (n)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Region4AccCtl(pub u32);
impl Region4AccCtl {
    #[doc = "Allowed domain ID"]
    #[must_use]
    #[inline(always)]
    pub const fn allow_did(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Allowed domain ID"]
    #[inline(always)]
    pub const fn set_allow_did(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Lock bit for the lower half word"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_l(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit for the lower half word"]
    #[inline(always)]
    pub const fn set_lock_l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Allow nonsecure mode access"]
    #[must_use]
    #[inline(always)]
    pub const fn allow_ns(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Allow nonsecure mode access"]
    #[inline(always)]
    pub const fn set_allow_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Allow user mode access"]
    #[must_use]
    #[inline(always)]
    pub const fn allow_user(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Allow user mode access"]
    #[inline(always)]
    pub const fn set_allow_user(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Lock bit for the higher half word"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_h(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit for the higher half word"]
    #[inline(always)]
    pub const fn set_lock_h(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Region4AccCtl {
    #[inline(always)]
    fn default() -> Region4AccCtl {
        Region4AccCtl(0u64 as u32)
    }
}
impl core::fmt::Debug for Region4AccCtl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Region4AccCtl")
            .field("allow_did", &self.allow_did())
            .field("lock_l", &self.lock_l())
            .field("allow_ns", &self.allow_ns())
            .field("allow_user", &self.allow_user())
            .field("lock_h", &self.lock_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Region4AccCtl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Region4AccCtl {
            allow_did: u8,
            lock_l: bool,
            allow_ns: bool,
            allow_user: bool,
            lock_h: bool,
        }
        let proxy = Region4AccCtl {
            allow_did: self.allow_did(),
            lock_l: self.lock_l(),
            allow_ns: self.allow_ns(),
            allow_user: self.allow_user(),
            lock_h: self.lock_h(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Start address of IEE region (n)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Region4BotAddr(pub u32);
impl Region4BotAddr {
    #[doc = "Start address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    #[must_use]
    #[inline(always)]
    pub const fn bot_addr(&self) -> u32 {
        let val = (self.0 >> 6usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "Start address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    #[inline(always)]
    pub const fn set_bot_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 6usize)) | (((val as u32) & 0x03ff_ffff) << 6usize);
    }
}
impl Default for Region4BotAddr {
    #[inline(always)]
    fn default() -> Region4BotAddr {
        Region4BotAddr(0u64 as u32)
    }
}
impl core::fmt::Debug for Region4BotAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Region4BotAddr")
            .field("bot_addr", &self.bot_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Region4BotAddr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Region4BotAddr {
            bot_addr: u32,
        }
        let proxy = Region4BotAddr {
            bot_addr: self.bot_addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Region enable for region (n)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Region4Ena(pub u32);
impl Region4Ena {
    #[doc = "Enable this region"]
    #[must_use]
    #[inline(always)]
    pub const fn encrypt_enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable this region"]
    #[inline(always)]
    pub const fn set_encrypt_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Region4Ena {
    #[inline(always)]
    fn default() -> Region4Ena {
        Region4Ena(0u64 as u32)
    }
}
impl core::fmt::Debug for Region4Ena {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Region4Ena")
            .field("encrypt_enable", &self.encrypt_enable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Region4Ena {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Region4Ena {
            encrypt_enable: bool,
        }
        let proxy = Region4Ena {
            encrypt_enable: self.encrypt_enable(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "End address of IEE region (n)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Region4TopAddr(pub u32);
impl Region4TopAddr {
    #[doc = "End address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    #[must_use]
    #[inline(always)]
    pub const fn top_addr(&self) -> u32 {
        let val = (self.0 >> 6usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "End address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    #[inline(always)]
    pub const fn set_top_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 6usize)) | (((val as u32) & 0x03ff_ffff) << 6usize);
    }
}
impl Default for Region4TopAddr {
    #[inline(always)]
    fn default() -> Region4TopAddr {
        Region4TopAddr(0u64 as u32)
    }
}
impl core::fmt::Debug for Region4TopAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Region4TopAddr")
            .field("top_addr", &self.top_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Region4TopAddr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Region4TopAddr {
            top_addr: u32,
        }
        let proxy = Region4TopAddr {
            top_addr: self.top_addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Access control for IEE APC registers of region (n)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Region5AccCtl(pub u32);
impl Region5AccCtl {
    #[doc = "Allowed domain ID"]
    #[must_use]
    #[inline(always)]
    pub const fn allow_did(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Allowed domain ID"]
    #[inline(always)]
    pub const fn set_allow_did(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Lock bit for the lower half word"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_l(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit for the lower half word"]
    #[inline(always)]
    pub const fn set_lock_l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Allow nonsecure mode access"]
    #[must_use]
    #[inline(always)]
    pub const fn allow_ns(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Allow nonsecure mode access"]
    #[inline(always)]
    pub const fn set_allow_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Allow user mode access"]
    #[must_use]
    #[inline(always)]
    pub const fn allow_user(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Allow user mode access"]
    #[inline(always)]
    pub const fn set_allow_user(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Lock bit for the higher half word"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_h(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit for the higher half word"]
    #[inline(always)]
    pub const fn set_lock_h(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Region5AccCtl {
    #[inline(always)]
    fn default() -> Region5AccCtl {
        Region5AccCtl(0u64 as u32)
    }
}
impl core::fmt::Debug for Region5AccCtl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Region5AccCtl")
            .field("allow_did", &self.allow_did())
            .field("lock_l", &self.lock_l())
            .field("allow_ns", &self.allow_ns())
            .field("allow_user", &self.allow_user())
            .field("lock_h", &self.lock_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Region5AccCtl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Region5AccCtl {
            allow_did: u8,
            lock_l: bool,
            allow_ns: bool,
            allow_user: bool,
            lock_h: bool,
        }
        let proxy = Region5AccCtl {
            allow_did: self.allow_did(),
            lock_l: self.lock_l(),
            allow_ns: self.allow_ns(),
            allow_user: self.allow_user(),
            lock_h: self.lock_h(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Start address of IEE region (n)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Region5BotAddr(pub u32);
impl Region5BotAddr {
    #[doc = "Start address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    #[must_use]
    #[inline(always)]
    pub const fn bot_addr(&self) -> u32 {
        let val = (self.0 >> 6usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "Start address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    #[inline(always)]
    pub const fn set_bot_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 6usize)) | (((val as u32) & 0x03ff_ffff) << 6usize);
    }
}
impl Default for Region5BotAddr {
    #[inline(always)]
    fn default() -> Region5BotAddr {
        Region5BotAddr(0u64 as u32)
    }
}
impl core::fmt::Debug for Region5BotAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Region5BotAddr")
            .field("bot_addr", &self.bot_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Region5BotAddr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Region5BotAddr {
            bot_addr: u32,
        }
        let proxy = Region5BotAddr {
            bot_addr: self.bot_addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Region enable for region (n)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Region5Ena(pub u32);
impl Region5Ena {
    #[doc = "Enable this region"]
    #[must_use]
    #[inline(always)]
    pub const fn encrypt_enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable this region"]
    #[inline(always)]
    pub const fn set_encrypt_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Region5Ena {
    #[inline(always)]
    fn default() -> Region5Ena {
        Region5Ena(0u64 as u32)
    }
}
impl core::fmt::Debug for Region5Ena {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Region5Ena")
            .field("encrypt_enable", &self.encrypt_enable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Region5Ena {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Region5Ena {
            encrypt_enable: bool,
        }
        let proxy = Region5Ena {
            encrypt_enable: self.encrypt_enable(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "End address of IEE region (n)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Region5TopAddr(pub u32);
impl Region5TopAddr {
    #[doc = "End address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    #[must_use]
    #[inline(always)]
    pub const fn top_addr(&self) -> u32 {
        let val = (self.0 >> 6usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "End address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    #[inline(always)]
    pub const fn set_top_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 6usize)) | (((val as u32) & 0x03ff_ffff) << 6usize);
    }
}
impl Default for Region5TopAddr {
    #[inline(always)]
    fn default() -> Region5TopAddr {
        Region5TopAddr(0u64 as u32)
    }
}
impl core::fmt::Debug for Region5TopAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Region5TopAddr")
            .field("top_addr", &self.top_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Region5TopAddr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Region5TopAddr {
            top_addr: u32,
        }
        let proxy = Region5TopAddr {
            top_addr: self.top_addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Access control for IEE APC registers of region (n)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Region6AccCtl(pub u32);
impl Region6AccCtl {
    #[doc = "Allowed domain ID"]
    #[must_use]
    #[inline(always)]
    pub const fn allow_did(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Allowed domain ID"]
    #[inline(always)]
    pub const fn set_allow_did(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Lock bit for the lower half word"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_l(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit for the lower half word"]
    #[inline(always)]
    pub const fn set_lock_l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Allow nonsecure mode access"]
    #[must_use]
    #[inline(always)]
    pub const fn allow_ns(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Allow nonsecure mode access"]
    #[inline(always)]
    pub const fn set_allow_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Allow user mode access"]
    #[must_use]
    #[inline(always)]
    pub const fn allow_user(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Allow user mode access"]
    #[inline(always)]
    pub const fn set_allow_user(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Lock bit for the higher half word"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_h(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit for the higher half word"]
    #[inline(always)]
    pub const fn set_lock_h(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Region6AccCtl {
    #[inline(always)]
    fn default() -> Region6AccCtl {
        Region6AccCtl(0u64 as u32)
    }
}
impl core::fmt::Debug for Region6AccCtl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Region6AccCtl")
            .field("allow_did", &self.allow_did())
            .field("lock_l", &self.lock_l())
            .field("allow_ns", &self.allow_ns())
            .field("allow_user", &self.allow_user())
            .field("lock_h", &self.lock_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Region6AccCtl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Region6AccCtl {
            allow_did: u8,
            lock_l: bool,
            allow_ns: bool,
            allow_user: bool,
            lock_h: bool,
        }
        let proxy = Region6AccCtl {
            allow_did: self.allow_did(),
            lock_l: self.lock_l(),
            allow_ns: self.allow_ns(),
            allow_user: self.allow_user(),
            lock_h: self.lock_h(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Start address of IEE region (n)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Region6BotAddr(pub u32);
impl Region6BotAddr {
    #[doc = "Start address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    #[must_use]
    #[inline(always)]
    pub const fn bot_addr(&self) -> u32 {
        let val = (self.0 >> 6usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "Start address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    #[inline(always)]
    pub const fn set_bot_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 6usize)) | (((val as u32) & 0x03ff_ffff) << 6usize);
    }
}
impl Default for Region6BotAddr {
    #[inline(always)]
    fn default() -> Region6BotAddr {
        Region6BotAddr(0u64 as u32)
    }
}
impl core::fmt::Debug for Region6BotAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Region6BotAddr")
            .field("bot_addr", &self.bot_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Region6BotAddr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Region6BotAddr {
            bot_addr: u32,
        }
        let proxy = Region6BotAddr {
            bot_addr: self.bot_addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Region enable for region (n)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Region6Ena(pub u32);
impl Region6Ena {
    #[doc = "Enable this region"]
    #[must_use]
    #[inline(always)]
    pub const fn encrypt_enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable this region"]
    #[inline(always)]
    pub const fn set_encrypt_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Region6Ena {
    #[inline(always)]
    fn default() -> Region6Ena {
        Region6Ena(0u64 as u32)
    }
}
impl core::fmt::Debug for Region6Ena {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Region6Ena")
            .field("encrypt_enable", &self.encrypt_enable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Region6Ena {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Region6Ena {
            encrypt_enable: bool,
        }
        let proxy = Region6Ena {
            encrypt_enable: self.encrypt_enable(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "End address of IEE region (n)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Region6TopAddr(pub u32);
impl Region6TopAddr {
    #[doc = "End address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    #[must_use]
    #[inline(always)]
    pub const fn top_addr(&self) -> u32 {
        let val = (self.0 >> 6usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "End address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    #[inline(always)]
    pub const fn set_top_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 6usize)) | (((val as u32) & 0x03ff_ffff) << 6usize);
    }
}
impl Default for Region6TopAddr {
    #[inline(always)]
    fn default() -> Region6TopAddr {
        Region6TopAddr(0u64 as u32)
    }
}
impl core::fmt::Debug for Region6TopAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Region6TopAddr")
            .field("top_addr", &self.top_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Region6TopAddr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Region6TopAddr {
            top_addr: u32,
        }
        let proxy = Region6TopAddr {
            top_addr: self.top_addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Access control for IEE APC registers of region (n)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Region7AccCtl(pub u32);
impl Region7AccCtl {
    #[doc = "Allowed domain ID"]
    #[must_use]
    #[inline(always)]
    pub const fn allow_did(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Allowed domain ID"]
    #[inline(always)]
    pub const fn set_allow_did(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Lock bit for the lower half word"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_l(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit for the lower half word"]
    #[inline(always)]
    pub const fn set_lock_l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Allow nonsecure mode access"]
    #[must_use]
    #[inline(always)]
    pub const fn allow_ns(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Allow nonsecure mode access"]
    #[inline(always)]
    pub const fn set_allow_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Allow user mode access"]
    #[must_use]
    #[inline(always)]
    pub const fn allow_user(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Allow user mode access"]
    #[inline(always)]
    pub const fn set_allow_user(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Lock bit for the higher half word"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_h(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit for the higher half word"]
    #[inline(always)]
    pub const fn set_lock_h(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Region7AccCtl {
    #[inline(always)]
    fn default() -> Region7AccCtl {
        Region7AccCtl(0u64 as u32)
    }
}
impl core::fmt::Debug for Region7AccCtl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Region7AccCtl")
            .field("allow_did", &self.allow_did())
            .field("lock_l", &self.lock_l())
            .field("allow_ns", &self.allow_ns())
            .field("allow_user", &self.allow_user())
            .field("lock_h", &self.lock_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Region7AccCtl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Region7AccCtl {
            allow_did: u8,
            lock_l: bool,
            allow_ns: bool,
            allow_user: bool,
            lock_h: bool,
        }
        let proxy = Region7AccCtl {
            allow_did: self.allow_did(),
            lock_l: self.lock_l(),
            allow_ns: self.allow_ns(),
            allow_user: self.allow_user(),
            lock_h: self.lock_h(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Start address of IEE region (n)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Region7BotAddr(pub u32);
impl Region7BotAddr {
    #[doc = "Start address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    #[must_use]
    #[inline(always)]
    pub const fn bot_addr(&self) -> u32 {
        let val = (self.0 >> 6usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "Start address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    #[inline(always)]
    pub const fn set_bot_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 6usize)) | (((val as u32) & 0x03ff_ffff) << 6usize);
    }
}
impl Default for Region7BotAddr {
    #[inline(always)]
    fn default() -> Region7BotAddr {
        Region7BotAddr(0u64 as u32)
    }
}
impl core::fmt::Debug for Region7BotAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Region7BotAddr")
            .field("bot_addr", &self.bot_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Region7BotAddr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Region7BotAddr {
            bot_addr: u32,
        }
        let proxy = Region7BotAddr {
            bot_addr: self.bot_addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Region enable for region (n)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Region7Ena(pub u32);
impl Region7Ena {
    #[doc = "Enable this region"]
    #[must_use]
    #[inline(always)]
    pub const fn encrypt_enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable this region"]
    #[inline(always)]
    pub const fn set_encrypt_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Region7Ena {
    #[inline(always)]
    fn default() -> Region7Ena {
        Region7Ena(0u64 as u32)
    }
}
impl core::fmt::Debug for Region7Ena {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Region7Ena")
            .field("encrypt_enable", &self.encrypt_enable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Region7Ena {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Region7Ena {
            encrypt_enable: bool,
        }
        let proxy = Region7Ena {
            encrypt_enable: self.encrypt_enable(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "End address of IEE region (n)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Region7TopAddr(pub u32);
impl Region7TopAddr {
    #[doc = "End address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    #[must_use]
    #[inline(always)]
    pub const fn top_addr(&self) -> u32 {
        let val = (self.0 >> 6usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "End address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    #[inline(always)]
    pub const fn set_top_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 6usize)) | (((val as u32) & 0x03ff_ffff) << 6usize);
    }
}
impl Default for Region7TopAddr {
    #[inline(always)]
    fn default() -> Region7TopAddr {
        Region7TopAddr(0u64 as u32)
    }
}
impl core::fmt::Debug for Region7TopAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Region7TopAddr")
            .field("top_addr", &self.top_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Region7TopAddr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Region7TopAddr {
            top_addr: u32,
        }
        let proxy = Region7TopAddr {
            top_addr: self.top_addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
