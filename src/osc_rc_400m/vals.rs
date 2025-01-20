#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clk1mErr {
    #[doc = "No error."]
    NoError = 0x0,
    #[doc = "Indicates error."]
    IndicatesError = 0x01,
}
impl Clk1mErr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clk1mErr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clk1mErr {
    #[inline(always)]
    fn from(val: u8) -> Clk1mErr {
        Clk1mErr::from_bits(val)
    }
}
impl From<Clk1mErr> for u8 {
    #[inline(always)]
    fn from(val: Clk1mErr) -> u8 {
        Clk1mErr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClrErr {
    #[doc = "Normal operation"]
    Enable = 0x0,
    #[doc = "Clears the error flag CLK1M_ERR in status register STAT0."]
    Disable = 0x01,
}
impl ClrErr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClrErr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClrErr {
    #[inline(always)]
    fn from(val: u8) -> ClrErr {
        ClrErr::from_bits(val)
    }
}
impl From<ClrErr> for u8 {
    #[inline(always)]
    fn from(val: ClrErr) -> u8 {
        ClrErr::to_bits(val)
    }
}
