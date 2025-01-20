#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AllowNonsecure {
    #[doc = "Do not allow non-secure write access"]
    Prevent = 0x0,
    #[doc = "Allow non-secure write access"]
    Allow = 0x01,
}
impl AllowNonsecure {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AllowNonsecure {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AllowNonsecure {
    #[inline(always)]
    fn from(val: u8) -> AllowNonsecure {
        AllowNonsecure::from_bits(val)
    }
}
impl From<AllowNonsecure> for u8 {
    #[inline(always)]
    fn from(val: AllowNonsecure) -> u8 {
        AllowNonsecure::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AllowUser {
    #[doc = "Do not allow user write access"]
    Prevent = 0x0,
    #[doc = "Allow user write access"]
    Allow = 0x01,
}
impl AllowUser {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AllowUser {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AllowUser {
    #[inline(always)]
    fn from(val: u8) -> AllowUser {
        AllowUser::from_bits(val)
    }
}
impl From<AllowUser> for u8 {
    #[inline(always)]
    fn from(val: AllowUser) -> u8 {
        AllowUser::to_bits(val)
    }
}
