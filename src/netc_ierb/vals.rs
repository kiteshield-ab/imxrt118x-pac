#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum E0tgslrZeroLookahead {
    #[doc = "Use MIN_LOOKAHEAD value"]
    Use = 0x0,
    #[doc = "If a gate control list is configured or when time specific departure is enabled on any traffic class (PTCaTSDR\\[TSDE\\] set to 1, where a corresponds to the traffic class number), use MIN_LOOKAHEAD value, otherwise use value of zero"]
    Zero = 0x01,
}
impl E0tgslrZeroLookahead {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> E0tgslrZeroLookahead {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for E0tgslrZeroLookahead {
    #[inline(always)]
    fn from(val: u8) -> E0tgslrZeroLookahead {
        E0tgslrZeroLookahead::from_bits(val)
    }
}
impl From<E0tgslrZeroLookahead> for u8 {
    #[inline(always)]
    fn from(val: E0tgslrZeroLookahead) -> u8 {
        E0tgslrZeroLookahead::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum E1tgslrZeroLookahead {
    #[doc = "Use MIN_LOOKAHEAD value"]
    Use = 0x0,
    #[doc = "If a gate control list is configured or when time specific departure is enabled on any traffic class (PTCaTSDR\\[TSDE\\] set to 1, where a corresponds to the traffic class number), use MIN_LOOKAHEAD value, otherwise use value of zero"]
    Zero = 0x01,
}
impl E1tgslrZeroLookahead {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> E1tgslrZeroLookahead {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for E1tgslrZeroLookahead {
    #[inline(always)]
    fn from(val: u8) -> E1tgslrZeroLookahead {
        E1tgslrZeroLookahead::from_bits(val)
    }
}
impl From<E1tgslrZeroLookahead> for u8 {
    #[inline(always)]
    fn from(val: E1tgslrZeroLookahead) -> u8 {
        E1tgslrZeroLookahead::to_bits(val)
    }
}
