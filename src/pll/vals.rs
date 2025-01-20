#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BiasSelect {
    #[doc = "Used in SoCs with a bias current of 10uA"]
    Bais10 = 0x0,
    #[doc = "Used in SoCs with a bias current of 2uA"]
    Bais2 = 0x01,
}
impl BiasSelect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BiasSelect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BiasSelect {
    #[inline(always)]
    fn from(val: u8) -> BiasSelect {
        BiasSelect::from_bits(val)
    }
}
impl From<BiasSelect> for u8 {
    #[inline(always)]
    fn from(val: BiasSelect) -> u8 {
        BiasSelect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HoldRingOff {
    #[doc = "Normal operation"]
    Normal = 0x0,
    #[doc = "Initialize PLL start up"]
    Enable = 0x01,
}
impl HoldRingOff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HoldRingOff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HoldRingOff {
    #[inline(always)]
    fn from(val: u8) -> HoldRingOff {
        HoldRingOff::from_bits(val)
    }
}
impl From<HoldRingOff> for u8 {
    #[inline(always)]
    fn from(val: HoldRingOff) -> u8 {
        HoldRingOff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PostDivSel {
    #[doc = "Divide by 1"]
    Divide1 = 0x0,
    #[doc = "Divide by 2"]
    Divide2 = 0x01,
    #[doc = "Divide by 4"]
    Divide4 = 0x02,
    #[doc = "Divide by 8"]
    Divide8 = 0x03,
    #[doc = "Divide by 16"]
    Divide16 = 0x04,
    #[doc = "Divide by 32"]
    Divide32 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl PostDivSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PostDivSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PostDivSel {
    #[inline(always)]
    fn from(val: u8) -> PostDivSel {
        PostDivSel::from_bits(val)
    }
}
impl From<PostDivSel> for u8 {
    #[inline(always)]
    fn from(val: PostDivSel) -> u8 {
        PostDivSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Powerup {
    #[doc = "Power down the PLL"]
    Pdown = 0x0,
    #[doc = "Power Up the PLL"]
    Pup = 0x01,
}
impl Powerup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Powerup {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Powerup {
    #[inline(always)]
    fn from(val: u8) -> Powerup {
        Powerup::from_bits(val)
    }
}
impl From<Powerup> for u8 {
    #[inline(always)]
    fn from(val: Powerup) -> u8 {
        Powerup::to_bits(val)
    }
}
