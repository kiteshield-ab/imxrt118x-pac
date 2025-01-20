#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cm33LockupTrigMode {
    #[doc = "Level-sensitive: System stays in reset until the reset source deasserts."]
    Level = 0x0,
    #[doc = "Edge-sensitive: System resets once, even if the reset source remains asserted."]
    Edge = 0x01,
}
impl Cm33LockupTrigMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cm33LockupTrigMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cm33LockupTrigMode {
    #[inline(always)]
    fn from(val: u8) -> Cm33LockupTrigMode {
        Cm33LockupTrigMode::from_bits(val)
    }
}
impl From<Cm33LockupTrigMode> for u8 {
    #[inline(always)]
    fn from(val: Cm33LockupTrigMode) -> u8 {
        Cm33LockupTrigMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cm33ResetTrigMode {
    #[doc = "Level-sensitive: System stays in reset until the reset source deasserts."]
    Level = 0x0,
    #[doc = "Edge-sensitive: System resets once, even if the reset source remains asserted."]
    Edge = 0x01,
}
impl Cm33ResetTrigMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cm33ResetTrigMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cm33ResetTrigMode {
    #[inline(always)]
    fn from(val: u8) -> Cm33ResetTrigMode {
        Cm33ResetTrigMode::from_bits(val)
    }
}
impl From<Cm33ResetTrigMode> for u8 {
    #[inline(always)]
    fn from(val: Cm33ResetTrigMode) -> u8 {
        Cm33ResetTrigMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cm7LockupTrigMode {
    #[doc = "Level-sensitive: System stays in reset until the reset source deasserts."]
    Level = 0x0,
    #[doc = "Edge-sensitive: System resets once, even if the reset source remains asserted."]
    Edge = 0x01,
}
impl Cm7LockupTrigMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cm7LockupTrigMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cm7LockupTrigMode {
    #[inline(always)]
    fn from(val: u8) -> Cm7LockupTrigMode {
        Cm7LockupTrigMode::from_bits(val)
    }
}
impl From<Cm7LockupTrigMode> for u8 {
    #[inline(always)]
    fn from(val: Cm7LockupTrigMode) -> u8 {
        Cm7LockupTrigMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cm7ResetTrigMode {
    #[doc = "Level-sensitive: System stays in reset until the reset source deasserts."]
    Level = 0x0,
    #[doc = "Edge-sensitive: System resets once, even if the reset source remains asserted."]
    Edge = 0x01,
}
impl Cm7ResetTrigMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cm7ResetTrigMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cm7ResetTrigMode {
    #[inline(always)]
    fn from(val: u8) -> Cm7ResetTrigMode {
        Cm7ResetTrigMode::from_bits(val)
    }
}
impl From<Cm7ResetTrigMode> for u8 {
    #[inline(always)]
    fn from(val: Cm7ResetTrigMode) -> u8 {
        Cm7ResetTrigMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcdcOvvtTrigMode {
    #[doc = "Level-sensitive: System stays in reset until the reset source deasserts."]
    Level = 0x0,
    #[doc = "Edge-sensitive: System resets once, even if the reset source remains asserted."]
    Edge = 0x01,
}
impl DcdcOvvtTrigMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcdcOvvtTrigMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcdcOvvtTrigMode {
    #[inline(always)]
    fn from(val: u8) -> DcdcOvvtTrigMode {
        DcdcOvvtTrigMode::from_bits(val)
    }
}
impl From<DcdcOvvtTrigMode> for u8 {
    #[inline(always)]
    fn from(val: DcdcOvvtTrigMode) -> u8 {
        DcdcOvvtTrigMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EcatRstoTrigMode {
    #[doc = "Level-sensitive: System stays in reset until the reset source deasserts."]
    Level = 0x0,
    #[doc = "Edge-sensitive: System resets once, even if the reset source remains asserted."]
    Edge = 0x01,
}
impl EcatRstoTrigMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EcatRstoTrigMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EcatRstoTrigMode {
    #[inline(always)]
    fn from(val: u8) -> EcatRstoTrigMode {
        EcatRstoTrigMode::from_bits(val)
    }
}
impl From<EcatRstoTrigMode> for u8 {
    #[inline(always)]
    fn from(val: EcatRstoTrigMode) -> u8 {
        EcatRstoTrigMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EdgelockTrigMode {
    #[doc = "Level-sensitive: System stays in reset until the reset source deasserts."]
    Level = 0x0,
    #[doc = "Edge-sensitive: System resets once, even if the reset source remains asserted."]
    Edge = 0x01,
}
impl EdgelockTrigMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EdgelockTrigMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EdgelockTrigMode {
    #[inline(always)]
    fn from(val: u8) -> EdgelockTrigMode {
        EdgelockTrigMode::from_bits(val)
    }
}
impl From<EdgelockTrigMode> for u8 {
    #[inline(always)]
    fn from(val: EdgelockTrigMode) -> u8 {
        EdgelockTrigMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IppBootMode {
    #[doc = "Boot from internal Fuses"]
    Bootopt0 = 0x0,
    #[doc = "Serial Downloader: USB1 or LPUART1"]
    Bootopt1 = 0x01,
    #[doc = "USDHC1 8-bit eMMC 5.1"]
    Bootopt2 = 0x02,
    #[doc = "USDHC2 4-bit SD 3.0"]
    Bootopt3 = 0x03,
    #[doc = "FlexSPI Serial NOR with SFDP (JESD-216) discoverable parameters"]
    Bootopt4 = 0x04,
    #[doc = "FlexSPI Serial NAND 2k page"]
    Bootopt5 = 0x05,
    #[doc = "FlexSPI Serial NAND 4k page"]
    Bootopt6 = 0x06,
    #[doc = "Test mode/Infinite loop mode"]
    Bootopt7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl IppBootMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IppBootMode {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IppBootMode {
    #[inline(always)]
    fn from(val: u8) -> IppBootMode {
        IppBootMode::from_bits(val)
    }
}
impl From<IppBootMode> for u8 {
    #[inline(always)]
    fn from(val: IppBootMode) -> u8 {
        IppBootMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum JtagswTrigMode {
    #[doc = "Level-sensitive: System stays in reset until the reset source deasserts."]
    Level = 0x0,
    #[doc = "Edge-sensitive: System resets once, even if the reset source remains asserted."]
    Edge = 0x01,
}
impl JtagswTrigMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> JtagswTrigMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for JtagswTrigMode {
    #[inline(always)]
    fn from(val: u8) -> JtagswTrigMode {
        JtagswTrigMode::from_bits(val)
    }
}
impl From<JtagswTrigMode> for u8 {
    #[inline(always)]
    fn from(val: JtagswTrigMode) -> u8 {
        JtagswTrigMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TempsenseTrigMode {
    #[doc = "Level-sensitive: System stays in reset until the reset source deasserts."]
    Level = 0x0,
    #[doc = "Edge-sensitive: System resets once, even if the reset source remains asserted."]
    Edge = 0x01,
}
impl TempsenseTrigMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TempsenseTrigMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TempsenseTrigMode {
    #[inline(always)]
    fn from(val: u8) -> TempsenseTrigMode {
        TempsenseTrigMode::from_bits(val)
    }
}
impl From<TempsenseTrigMode> for u8 {
    #[inline(always)]
    fn from(val: TempsenseTrigMode) -> u8 {
        TempsenseTrigMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdog1TrigMode {
    #[doc = "Level-sensitive: System stays in reset until the reset source deasserts."]
    Level = 0x0,
    #[doc = "Edge-sensitive: System resets once, even if the reset source remains asserted."]
    Edge = 0x01,
}
impl Wdog1TrigMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdog1TrigMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdog1TrigMode {
    #[inline(always)]
    fn from(val: u8) -> Wdog1TrigMode {
        Wdog1TrigMode::from_bits(val)
    }
}
impl From<Wdog1TrigMode> for u8 {
    #[inline(always)]
    fn from(val: Wdog1TrigMode) -> u8 {
        Wdog1TrigMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdog2TrigMode {
    #[doc = "Level-sensitive: System stays in reset until the reset source deasserts."]
    Level = 0x0,
    #[doc = "Edge-sensitive: System resets once, even if the reset source remains asserted."]
    Edge = 0x01,
}
impl Wdog2TrigMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdog2TrigMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdog2TrigMode {
    #[inline(always)]
    fn from(val: u8) -> Wdog2TrigMode {
        Wdog2TrigMode::from_bits(val)
    }
}
impl From<Wdog2TrigMode> for u8 {
    #[inline(always)]
    fn from(val: Wdog2TrigMode) -> u8 {
        Wdog2TrigMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdog3TrigMode {
    #[doc = "Level-sensitive: System stays in reset until the reset source deasserts."]
    Level = 0x0,
    #[doc = "Edge-sensitive: System resets once, even if the reset source remains asserted."]
    Edge = 0x01,
}
impl Wdog3TrigMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdog3TrigMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdog3TrigMode {
    #[inline(always)]
    fn from(val: u8) -> Wdog3TrigMode {
        Wdog3TrigMode::from_bits(val)
    }
}
impl From<Wdog3TrigMode> for u8 {
    #[inline(always)]
    fn from(val: Wdog3TrigMode) -> u8 {
        Wdog3TrigMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdog4TrigMode {
    #[doc = "Level-sensitive: System stays in reset until the reset source deasserts."]
    Level = 0x0,
    #[doc = "Edge-sensitive: System resets once, even if the reset source remains asserted."]
    Edge = 0x01,
}
impl Wdog4TrigMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdog4TrigMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdog4TrigMode {
    #[inline(always)]
    fn from(val: u8) -> Wdog4TrigMode {
        Wdog4TrigMode::from_bits(val)
    }
}
impl From<Wdog4TrigMode> for u8 {
    #[inline(always)]
    fn from(val: Wdog4TrigMode) -> u8 {
        Wdog4TrigMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdog5TrigMode {
    #[doc = "Level-sensitive: System stays in reset until the reset source deasserts."]
    Level = 0x0,
    #[doc = "Edge-sensitive: System resets once, even if the reset source remains asserted."]
    Edge = 0x01,
}
impl Wdog5TrigMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdog5TrigMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdog5TrigMode {
    #[inline(always)]
    fn from(val: u8) -> Wdog5TrigMode {
        Wdog5TrigMode::from_bits(val)
    }
}
impl From<Wdog5TrigMode> for u8 {
    #[inline(always)]
    fn from(val: Wdog5TrigMode) -> u8 {
        Wdog5TrigMode::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct WhiteList(pub u16);
impl WhiteList {
    #[doc = "Core with domain ID=0 can write General registers."]
    pub const DOMAIN0: Self = Self(0x01);
    #[doc = "Core with domain ID=1 can write General registers."]
    pub const DOMAIN1: Self = Self(0x02);
    #[doc = "Core with domain ID=2 can write General registers."]
    pub const DOMAIN2: Self = Self(0x04);
    #[doc = "Core with domain ID=3 can write General registers."]
    pub const DOMAIN3: Self = Self(0x08);
    #[doc = "Core with domain ID=4 can write General registers."]
    pub const DOMAIN4: Self = Self(0x10);
    #[doc = "Core with domain ID=5 can write General registers."]
    pub const DOMAIN5: Self = Self(0x20);
    #[doc = "Core with domain ID=6 can write General registers."]
    pub const DOMAIN6: Self = Self(0x40);
    #[doc = "Core with domain ID=7 can write General registers."]
    pub const DOMAIN7: Self = Self(0x80);
    #[doc = "Core with domain ID=8 can write General registers."]
    pub const DOMAIN8: Self = Self(0x0100);
    #[doc = "Core with domain ID=9 can write General registers."]
    pub const DOMAIN9: Self = Self(0x0200);
    #[doc = "Core with domain ID=10 can write General registers."]
    pub const DOMAIN10: Self = Self(0x0400);
    #[doc = "Core with domain ID=11 can write General registers."]
    pub const DOMAIN11: Self = Self(0x0800);
    #[doc = "Core with domain ID=12 can write General registers."]
    pub const DOMAIN12: Self = Self(0x1000);
    #[doc = "Core with domain ID=13 can write General registers."]
    pub const DOMAIN13: Self = Self(0x2000);
    #[doc = "Core with domain ID=14 can write General registers."]
    pub const DOMAIN14: Self = Self(0x4000);
    #[doc = "Core with domain ID=15 can write General registers."]
    pub const DOMAIN15: Self = Self(0x8000);
}
impl WhiteList {
    pub const fn from_bits(val: u16) -> WhiteList {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for WhiteList {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("DOMAIN0"),
            0x02 => f.write_str("DOMAIN1"),
            0x04 => f.write_str("DOMAIN2"),
            0x08 => f.write_str("DOMAIN3"),
            0x10 => f.write_str("DOMAIN4"),
            0x20 => f.write_str("DOMAIN5"),
            0x40 => f.write_str("DOMAIN6"),
            0x80 => f.write_str("DOMAIN7"),
            0x0100 => f.write_str("DOMAIN8"),
            0x0200 => f.write_str("DOMAIN9"),
            0x0400 => f.write_str("DOMAIN10"),
            0x0800 => f.write_str("DOMAIN11"),
            0x1000 => f.write_str("DOMAIN12"),
            0x2000 => f.write_str("DOMAIN13"),
            0x4000 => f.write_str("DOMAIN14"),
            0x8000 => f.write_str("DOMAIN15"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WhiteList {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "DOMAIN0"),
            0x02 => defmt::write!(f, "DOMAIN1"),
            0x04 => defmt::write!(f, "DOMAIN2"),
            0x08 => defmt::write!(f, "DOMAIN3"),
            0x10 => defmt::write!(f, "DOMAIN4"),
            0x20 => defmt::write!(f, "DOMAIN5"),
            0x40 => defmt::write!(f, "DOMAIN6"),
            0x80 => defmt::write!(f, "DOMAIN7"),
            0x0100 => defmt::write!(f, "DOMAIN8"),
            0x0200 => defmt::write!(f, "DOMAIN9"),
            0x0400 => defmt::write!(f, "DOMAIN10"),
            0x0800 => defmt::write!(f, "DOMAIN11"),
            0x1000 => defmt::write!(f, "DOMAIN12"),
            0x2000 => defmt::write!(f, "DOMAIN13"),
            0x4000 => defmt::write!(f, "DOMAIN14"),
            0x8000 => defmt::write!(f, "DOMAIN15"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for WhiteList {
    #[inline(always)]
    fn from(val: u16) -> WhiteList {
        WhiteList::from_bits(val)
    }
}
impl From<WhiteList> for u16 {
    #[inline(always)]
    fn from(val: WhiteList) -> u16 {
        WhiteList::to_bits(val)
    }
}
