#[doc = "Slot Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SlotCtrl(pub u32);
impl SlotCtrl {
    #[doc = "Domain ID of the slot to be locked"]
    #[must_use]
    #[inline(always)]
    pub const fn locked_domain_id(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain ID of the slot to be locked"]
    #[inline(always)]
    pub const fn set_locked_domain_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Lock domain ID of this slot"]
    #[must_use]
    #[inline(always)]
    pub const fn domain_lock(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Lock domain ID of this slot"]
    #[inline(always)]
    pub const fn set_domain_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Allow non-secure write access to this domain control register or domain register"]
    #[must_use]
    #[inline(always)]
    pub const fn allow_nonsecure(&self) -> super::vals::AllowNonsecure {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::AllowNonsecure::from_bits(val as u8)
    }
    #[doc = "Allow non-secure write access to this domain control register or domain register"]
    #[inline(always)]
    pub const fn set_allow_nonsecure(&mut self, val: super::vals::AllowNonsecure) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Allow user write access to this domain control register or domain register"]
    #[must_use]
    #[inline(always)]
    pub const fn allow_user(&self) -> super::vals::AllowUser {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::AllowUser::from_bits(val as u8)
    }
    #[doc = "Allow user write access to this domain control register or domain register"]
    #[inline(always)]
    pub const fn set_allow_user(&mut self, val: super::vals::AllowUser) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Lock control of this slot"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_control(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Lock control of this slot"]
    #[inline(always)]
    pub const fn set_lock_control(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SlotCtrl {
    #[inline(always)]
    fn default() -> SlotCtrl {
        SlotCtrl(2u64 as u32)
    }
}
impl core::fmt::Debug for SlotCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SlotCtrl")
            .field("locked_domain_id", &self.locked_domain_id())
            .field("domain_lock", &self.domain_lock())
            .field("allow_nonsecure", &self.allow_nonsecure())
            .field("allow_user", &self.allow_user())
            .field("lock_control", &self.lock_control())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SlotCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SlotCtrl {
            locked_domain_id: u8,
            domain_lock: bool,
            allow_nonsecure: super::vals::AllowNonsecure,
            allow_user: super::vals::AllowUser,
            lock_control: bool,
        }
        let proxy = SlotCtrl {
            locked_domain_id: self.locked_domain_id(),
            domain_lock: self.domain_lock(),
            allow_nonsecure: self.allow_nonsecure(),
            allow_user: self.allow_user(),
            lock_control: self.lock_control(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
