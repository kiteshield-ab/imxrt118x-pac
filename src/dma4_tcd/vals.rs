#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Instr {
    #[doc = "Data access for DMA transfers"]
    Data = 0x0,
    #[doc = "Instruction access for DMA transfers"]
    Instr = 0x01,
}
impl Instr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Instr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Instr {
    #[inline(always)]
    fn from(val: u8) -> Instr {
        Instr::from_bits(val)
    }
}
impl From<Instr> for u8 {
    #[inline(always)]
    fn from(val: Instr) -> u8 {
        Instr::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Signext(pub u8);
impl Signext {
    #[doc = "disabled"]
    pub const DISABLE: Self = Self(0x0);
    #[doc = "A non-zero value specifying the sign extend bit position"]
    pub const ENABLE: Self = Self(0x01);
}
impl Signext {
    pub const fn from_bits(val: u8) -> Signext {
        Self(val & 0x3f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Signext {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("DISABLE"),
            0x01 => f.write_str("ENABLE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Signext {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "DISABLE"),
            0x01 => defmt::write!(f, "ENABLE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Signext {
    #[inline(always)]
    fn from(val: u8) -> Signext {
        Signext::from_bits(val)
    }
}
impl From<Signext> for u8 {
    #[inline(always)]
    fn from(val: Signext) -> u8 {
        Signext::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swap {
    #[doc = "disabled"]
    Disable = 0x0,
    #[doc = "read with 8-bit swap"]
    ReadSwap8 = 0x01,
    #[doc = "read with 16-bit swap"]
    ReadSwap16 = 0x02,
    #[doc = "read with 32-bit swap"]
    ReadSwap32 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "write with 8-bit swap"]
    WriteSwap8 = 0x09,
    #[doc = "write with 16-bit swap"]
    WriteSwap16 = 0x0a,
    #[doc = "write with 32-bit swap"]
    WriteSwap32 = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Swap {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swap {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swap {
    #[inline(always)]
    fn from(val: u8) -> Swap {
        Swap::from_bits(val)
    }
}
impl From<Swap> for u8 {
    #[inline(always)]
    fn from(val: Swap) -> u8 {
        Swap::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmc {
    #[doc = "Read/Write"]
    Normal = 0x0,
    #[doc = "Read Only"]
    ReadOnly = 0x01,
    #[doc = "Write Only"]
    WriteOnly = 0x02,
    _RESERVED_3 = 0x03,
}
impl Tmc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmc {
    #[inline(always)]
    fn from(val: u8) -> Tmc {
        Tmc::from_bits(val)
    }
}
impl From<Tmc> for u8 {
    #[inline(always)]
    fn from(val: Tmc) -> u8 {
        Tmc::to_bits(val)
    }
}
