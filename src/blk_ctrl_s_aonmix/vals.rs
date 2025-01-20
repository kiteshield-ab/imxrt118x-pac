#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cm33IrqMask0M(pub u32);
impl Cm33IrqMask0M {
    #[doc = "Mask IRQ"]
    pub const ENABLE: Self = Self(0x0);
    #[doc = "No Mask IRQ"]
    pub const DISABLE: Self = Self(0x01);
}
impl Cm33IrqMask0M {
    pub const fn from_bits(val: u32) -> Cm33IrqMask0M {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Cm33IrqMask0M {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("ENABLE"),
            0x01 => f.write_str("DISABLE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cm33IrqMask0M {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "ENABLE"),
            0x01 => defmt::write!(f, "DISABLE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Cm33IrqMask0M {
    #[inline(always)]
    fn from(val: u32) -> Cm33IrqMask0M {
        Cm33IrqMask0M::from_bits(val)
    }
}
impl From<Cm33IrqMask0M> for u32 {
    #[inline(always)]
    fn from(val: Cm33IrqMask0M) -> u32 {
        Cm33IrqMask0M::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cm33IrqMask1M(pub u32);
impl Cm33IrqMask1M {
    #[doc = "Mask IRQ"]
    pub const ENABLE: Self = Self(0x0);
    #[doc = "No Mask IRQ"]
    pub const DISABLE: Self = Self(0x01);
}
impl Cm33IrqMask1M {
    pub const fn from_bits(val: u32) -> Cm33IrqMask1M {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Cm33IrqMask1M {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("ENABLE"),
            0x01 => f.write_str("DISABLE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cm33IrqMask1M {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "ENABLE"),
            0x01 => defmt::write!(f, "DISABLE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Cm33IrqMask1M {
    #[inline(always)]
    fn from(val: u32) -> Cm33IrqMask1M {
        Cm33IrqMask1M::from_bits(val)
    }
}
impl From<Cm33IrqMask1M> for u32 {
    #[inline(always)]
    fn from(val: Cm33IrqMask1M) -> u32 {
        Cm33IrqMask1M::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cm33IrqMask2M(pub u32);
impl Cm33IrqMask2M {
    #[doc = "Mask IRQ"]
    pub const ENABLE: Self = Self(0x0);
    #[doc = "No Mask IRQ"]
    pub const DISABLE: Self = Self(0x01);
}
impl Cm33IrqMask2M {
    pub const fn from_bits(val: u32) -> Cm33IrqMask2M {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Cm33IrqMask2M {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("ENABLE"),
            0x01 => f.write_str("DISABLE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cm33IrqMask2M {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "ENABLE"),
            0x01 => defmt::write!(f, "DISABLE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Cm33IrqMask2M {
    #[inline(always)]
    fn from(val: u32) -> Cm33IrqMask2M {
        Cm33IrqMask2M::from_bits(val)
    }
}
impl From<Cm33IrqMask2M> for u32 {
    #[inline(always)]
    fn from(val: Cm33IrqMask2M) -> u32 {
        Cm33IrqMask2M::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cm33IrqMask3M(pub u32);
impl Cm33IrqMask3M {
    #[doc = "Mask IRQ"]
    pub const ENABLE: Self = Self(0x0);
    #[doc = "No Mask IRQ"]
    pub const DISABLE: Self = Self(0x01);
}
impl Cm33IrqMask3M {
    pub const fn from_bits(val: u32) -> Cm33IrqMask3M {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Cm33IrqMask3M {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("ENABLE"),
            0x01 => f.write_str("DISABLE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cm33IrqMask3M {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "ENABLE"),
            0x01 => defmt::write!(f, "DISABLE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Cm33IrqMask3M {
    #[inline(always)]
    fn from(val: u32) -> Cm33IrqMask3M {
        Cm33IrqMask3M::from_bits(val)
    }
}
impl From<Cm33IrqMask3M> for u32 {
    #[inline(always)]
    fn from(val: Cm33IrqMask3M) -> u32 {
        Cm33IrqMask3M::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cm33IrqMask4M(pub u32);
impl Cm33IrqMask4M {
    #[doc = "Mask IRQ"]
    pub const ENABLE: Self = Self(0x0);
    #[doc = "No Mask IRQ"]
    pub const DISABLE: Self = Self(0x01);
}
impl Cm33IrqMask4M {
    pub const fn from_bits(val: u32) -> Cm33IrqMask4M {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Cm33IrqMask4M {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("ENABLE"),
            0x01 => f.write_str("DISABLE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cm33IrqMask4M {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "ENABLE"),
            0x01 => defmt::write!(f, "DISABLE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Cm33IrqMask4M {
    #[inline(always)]
    fn from(val: u32) -> Cm33IrqMask4M {
        Cm33IrqMask4M::from_bits(val)
    }
}
impl From<Cm33IrqMask4M> for u32 {
    #[inline(always)]
    fn from(val: Cm33IrqMask4M) -> u32 {
        Cm33IrqMask4M::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cm33IrqMask5M(pub u32);
impl Cm33IrqMask5M {
    #[doc = "Mask IRQ"]
    pub const ENABLE: Self = Self(0x0);
    #[doc = "No Mask IRQ"]
    pub const DISABLE: Self = Self(0x01);
}
impl Cm33IrqMask5M {
    pub const fn from_bits(val: u32) -> Cm33IrqMask5M {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Cm33IrqMask5M {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("ENABLE"),
            0x01 => f.write_str("DISABLE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cm33IrqMask5M {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "ENABLE"),
            0x01 => defmt::write!(f, "DISABLE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Cm33IrqMask5M {
    #[inline(always)]
    fn from(val: u32) -> Cm33IrqMask5M {
        Cm33IrqMask5M::from_bits(val)
    }
}
impl From<Cm33IrqMask5M> for u32 {
    #[inline(always)]
    fn from(val: Cm33IrqMask5M) -> u32 {
        Cm33IrqMask5M::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cm33IrqMask6M(pub u32);
impl Cm33IrqMask6M {
    #[doc = "Mask IRQ"]
    pub const ENABLE: Self = Self(0x0);
    #[doc = "No Mask IRQ"]
    pub const DISABLE: Self = Self(0x01);
}
impl Cm33IrqMask6M {
    pub const fn from_bits(val: u32) -> Cm33IrqMask6M {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Cm33IrqMask6M {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("ENABLE"),
            0x01 => f.write_str("DISABLE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cm33IrqMask6M {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "ENABLE"),
            0x01 => defmt::write!(f, "DISABLE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Cm33IrqMask6M {
    #[inline(always)]
    fn from(val: u32) -> Cm33IrqMask6M {
        Cm33IrqMask6M::from_bits(val)
    }
}
impl From<Cm33IrqMask6M> for u32 {
    #[inline(always)]
    fn from(val: Cm33IrqMask6M) -> u32 {
        Cm33IrqMask6M::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cm33IrqMask7M(pub u32);
impl Cm33IrqMask7M {
    #[doc = "Mask IRQ"]
    pub const ENABLE: Self = Self(0x0);
    #[doc = "No Mask IRQ"]
    pub const DISABLE: Self = Self(0x01);
}
impl Cm33IrqMask7M {
    pub const fn from_bits(val: u32) -> Cm33IrqMask7M {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Cm33IrqMask7M {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("ENABLE"),
            0x01 => f.write_str("DISABLE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cm33IrqMask7M {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "ENABLE"),
            0x01 => defmt::write!(f, "DISABLE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Cm33IrqMask7M {
    #[inline(always)]
    fn from(val: u32) -> Cm33IrqMask7M {
        Cm33IrqMask7M::from_bits(val)
    }
}
impl From<Cm33IrqMask7M> for u32 {
    #[inline(always)]
    fn from(val: Cm33IrqMask7M) -> u32 {
        Cm33IrqMask7M::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cm7IrqMask0M(pub u32);
impl Cm7IrqMask0M {
    #[doc = "Mask IRQ"]
    pub const ENABLE: Self = Self(0x0);
    #[doc = "No Mask IRQ"]
    pub const DISABLE: Self = Self(0x01);
}
impl Cm7IrqMask0M {
    pub const fn from_bits(val: u32) -> Cm7IrqMask0M {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Cm7IrqMask0M {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("ENABLE"),
            0x01 => f.write_str("DISABLE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cm7IrqMask0M {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "ENABLE"),
            0x01 => defmt::write!(f, "DISABLE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Cm7IrqMask0M {
    #[inline(always)]
    fn from(val: u32) -> Cm7IrqMask0M {
        Cm7IrqMask0M::from_bits(val)
    }
}
impl From<Cm7IrqMask0M> for u32 {
    #[inline(always)]
    fn from(val: Cm7IrqMask0M) -> u32 {
        Cm7IrqMask0M::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cm7IrqMask1M(pub u32);
impl Cm7IrqMask1M {
    #[doc = "Mask IRQ"]
    pub const ENABLE: Self = Self(0x0);
    #[doc = "No Mask IRQ"]
    pub const DISABLE: Self = Self(0x01);
}
impl Cm7IrqMask1M {
    pub const fn from_bits(val: u32) -> Cm7IrqMask1M {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Cm7IrqMask1M {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("ENABLE"),
            0x01 => f.write_str("DISABLE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cm7IrqMask1M {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "ENABLE"),
            0x01 => defmt::write!(f, "DISABLE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Cm7IrqMask1M {
    #[inline(always)]
    fn from(val: u32) -> Cm7IrqMask1M {
        Cm7IrqMask1M::from_bits(val)
    }
}
impl From<Cm7IrqMask1M> for u32 {
    #[inline(always)]
    fn from(val: Cm7IrqMask1M) -> u32 {
        Cm7IrqMask1M::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cm7IrqMask2M(pub u32);
impl Cm7IrqMask2M {
    #[doc = "Mask IRQ"]
    pub const ENABLE: Self = Self(0x0);
    #[doc = "No Mask IRQ"]
    pub const DISABLE: Self = Self(0x01);
}
impl Cm7IrqMask2M {
    pub const fn from_bits(val: u32) -> Cm7IrqMask2M {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Cm7IrqMask2M {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("ENABLE"),
            0x01 => f.write_str("DISABLE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cm7IrqMask2M {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "ENABLE"),
            0x01 => defmt::write!(f, "DISABLE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Cm7IrqMask2M {
    #[inline(always)]
    fn from(val: u32) -> Cm7IrqMask2M {
        Cm7IrqMask2M::from_bits(val)
    }
}
impl From<Cm7IrqMask2M> for u32 {
    #[inline(always)]
    fn from(val: Cm7IrqMask2M) -> u32 {
        Cm7IrqMask2M::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cm7IrqMask3M(pub u32);
impl Cm7IrqMask3M {
    #[doc = "Mask IRQ"]
    pub const ENABLE: Self = Self(0x0);
    #[doc = "No Mask IRQ"]
    pub const DISABLE: Self = Self(0x01);
}
impl Cm7IrqMask3M {
    pub const fn from_bits(val: u32) -> Cm7IrqMask3M {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Cm7IrqMask3M {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("ENABLE"),
            0x01 => f.write_str("DISABLE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cm7IrqMask3M {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "ENABLE"),
            0x01 => defmt::write!(f, "DISABLE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Cm7IrqMask3M {
    #[inline(always)]
    fn from(val: u32) -> Cm7IrqMask3M {
        Cm7IrqMask3M::from_bits(val)
    }
}
impl From<Cm7IrqMask3M> for u32 {
    #[inline(always)]
    fn from(val: Cm7IrqMask3M) -> u32 {
        Cm7IrqMask3M::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cm7IrqMask4M(pub u32);
impl Cm7IrqMask4M {
    #[doc = "Mask IRQ"]
    pub const ENABLE: Self = Self(0x0);
    #[doc = "No Mask IRQ"]
    pub const DISABLE: Self = Self(0x01);
}
impl Cm7IrqMask4M {
    pub const fn from_bits(val: u32) -> Cm7IrqMask4M {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Cm7IrqMask4M {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("ENABLE"),
            0x01 => f.write_str("DISABLE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cm7IrqMask4M {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "ENABLE"),
            0x01 => defmt::write!(f, "DISABLE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Cm7IrqMask4M {
    #[inline(always)]
    fn from(val: u32) -> Cm7IrqMask4M {
        Cm7IrqMask4M::from_bits(val)
    }
}
impl From<Cm7IrqMask4M> for u32 {
    #[inline(always)]
    fn from(val: Cm7IrqMask4M) -> u32 {
        Cm7IrqMask4M::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cm7IrqMask5M(pub u32);
impl Cm7IrqMask5M {
    #[doc = "Mask IRQ"]
    pub const ENABLE: Self = Self(0x0);
    #[doc = "No Mask IRQ"]
    pub const DISABLE: Self = Self(0x01);
}
impl Cm7IrqMask5M {
    pub const fn from_bits(val: u32) -> Cm7IrqMask5M {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Cm7IrqMask5M {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("ENABLE"),
            0x01 => f.write_str("DISABLE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cm7IrqMask5M {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "ENABLE"),
            0x01 => defmt::write!(f, "DISABLE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Cm7IrqMask5M {
    #[inline(always)]
    fn from(val: u32) -> Cm7IrqMask5M {
        Cm7IrqMask5M::from_bits(val)
    }
}
impl From<Cm7IrqMask5M> for u32 {
    #[inline(always)]
    fn from(val: Cm7IrqMask5M) -> u32 {
        Cm7IrqMask5M::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cm7IrqMask6M(pub u32);
impl Cm7IrqMask6M {
    #[doc = "Mask IRQ"]
    pub const ENABLE: Self = Self(0x0);
    #[doc = "No Mask IRQ"]
    pub const DISABLE: Self = Self(0x01);
}
impl Cm7IrqMask6M {
    pub const fn from_bits(val: u32) -> Cm7IrqMask6M {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Cm7IrqMask6M {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("ENABLE"),
            0x01 => f.write_str("DISABLE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cm7IrqMask6M {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "ENABLE"),
            0x01 => defmt::write!(f, "DISABLE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Cm7IrqMask6M {
    #[inline(always)]
    fn from(val: u32) -> Cm7IrqMask6M {
        Cm7IrqMask6M::from_bits(val)
    }
}
impl From<Cm7IrqMask6M> for u32 {
    #[inline(always)]
    fn from(val: Cm7IrqMask6M) -> u32 {
        Cm7IrqMask6M::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cm7IrqMask7M(pub u32);
impl Cm7IrqMask7M {
    #[doc = "Mask IRQ"]
    pub const ENABLE: Self = Self(0x0);
    #[doc = "No Mask IRQ"]
    pub const DISABLE: Self = Self(0x01);
}
impl Cm7IrqMask7M {
    pub const fn from_bits(val: u32) -> Cm7IrqMask7M {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Cm7IrqMask7M {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("ENABLE"),
            0x01 => f.write_str("DISABLE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cm7IrqMask7M {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "ENABLE"),
            0x01 => defmt::write!(f, "DISABLE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Cm7IrqMask7M {
    #[inline(always)]
    fn from(val: u32) -> Cm7IrqMask7M {
        Cm7IrqMask7M::from_bits(val)
    }
}
impl From<Cm7IrqMask7M> for u32 {
    #[inline(always)]
    fn from(val: Cm7IrqMask7M) -> u32 {
        Cm7IrqMask7M::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DapCtr {
    #[doc = "DAP access is not granted by ROM"]
    DapNo = 0x0,
    #[doc = "DAP access is granted by ROM"]
    DapYes = 0x01,
}
impl DapCtr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DapCtr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DapCtr {
    #[inline(always)]
    fn from(val: u8) -> DapCtr {
        DapCtr::from_bits(val)
    }
}
impl From<DapCtr> for u8 {
    #[inline(always)]
    fn from(val: DapCtr) -> u8 {
        DapCtr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EdgelockHaltAck {
    #[doc = "EdgeLock is not fully halted and its clocks must be enabled"]
    EdgelockNotHalted = 0x0,
    #[doc = "EdgeLock is fully halted indicating clocks may be removed"]
    EdgelockHalted = 0x01,
}
impl EdgelockHaltAck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EdgelockHaltAck {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EdgelockHaltAck {
    #[inline(always)]
    fn from(val: u8) -> EdgelockHaltAck {
        EdgelockHaltAck::from_bits(val)
    }
}
impl From<EdgelockHaltAck> for u8 {
    #[inline(always)]
    fn from(val: EdgelockHaltAck) -> u8 {
        EdgelockHaltAck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EdgelockHaltExitIrqClr {
    #[doc = "Remove the clear signal. This bit is not self-clearing and need SW to clear."]
    RemoveClr = 0x0,
    #[doc = "Clear EdgeLock halt exit interrupt"]
    IntClr = 0x01,
}
impl EdgelockHaltExitIrqClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EdgelockHaltExitIrqClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EdgelockHaltExitIrqClr {
    #[inline(always)]
    fn from(val: u8) -> EdgelockHaltExitIrqClr {
        EdgelockHaltExitIrqClr::from_bits(val)
    }
}
impl From<EdgelockHaltExitIrqClr> for u8 {
    #[inline(always)]
    fn from(val: EdgelockHaltExitIrqClr) -> u8 {
        EdgelockHaltExitIrqClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ForceRoundRobin {
    #[doc = "Enable force round robin(default)"]
    Enable = 0x0,
    #[doc = "Disable force round robin"]
    Disable = 0x01,
}
impl ForceRoundRobin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ForceRoundRobin {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ForceRoundRobin {
    #[inline(always)]
    fn from(val: u8) -> ForceRoundRobin {
        ForceRoundRobin::from_bits(val)
    }
}
impl From<ForceRoundRobin> for u8 {
    #[inline(always)]
    fn from(val: ForceRoundRobin) -> u8 {
        ForceRoundRobin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M0HighPriority {
    #[doc = "Default Priority"]
    Enable = 0x0,
    #[doc = "High Priority"]
    Disable = 0x01,
}
impl M0HighPriority {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M0HighPriority {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M0HighPriority {
    #[inline(always)]
    fn from(val: u8) -> M0HighPriority {
        M0HighPriority::from_bits(val)
    }
}
impl From<M0HighPriority> for u8 {
    #[inline(always)]
    fn from(val: M0HighPriority) -> u8 {
        M0HighPriority::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M1HighPriority {
    #[doc = "Default Priority"]
    Enable = 0x0,
    #[doc = "High Priority"]
    Disable = 0x01,
}
impl M1HighPriority {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M1HighPriority {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M1HighPriority {
    #[inline(always)]
    fn from(val: u8) -> M1HighPriority {
        M1HighPriority::from_bits(val)
    }
}
impl From<M1HighPriority> for u8 {
    #[inline(always)]
    fn from(val: M1HighPriority) -> u8 {
        M1HighPriority::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M2HighPriority {
    #[doc = "Default Priority"]
    Enable = 0x0,
    #[doc = "High Priority"]
    Disable = 0x01,
}
impl M2HighPriority {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M2HighPriority {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M2HighPriority {
    #[inline(always)]
    fn from(val: u8) -> M2HighPriority {
        M2HighPriority::from_bits(val)
    }
}
impl From<M2HighPriority> for u8 {
    #[inline(always)]
    fn from(val: M2HighPriority) -> u8 {
        M2HighPriority::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M33CfgTcmSize {
    #[doc = "Regular TCM, 128KB Code TCM and 128KB Sys TCM"]
    TcmSize3 = 0x0,
    #[doc = "Double Code TCM, 256KB Code TCM"]
    TcmSize2 = 0x01,
    #[doc = "Double Sys TCM, 256KB Sys TCM"]
    TcmSize1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl M33CfgTcmSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M33CfgTcmSize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M33CfgTcmSize {
    #[inline(always)]
    fn from(val: u8) -> M33CfgTcmSize {
        M33CfgTcmSize::from_bits(val)
    }
}
impl From<M33CfgTcmSize> for u8 {
    #[inline(always)]
    fn from(val: M33CfgTcmSize) -> u8 {
        M33CfgTcmSize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M3HighPriority {
    #[doc = "Default Priority"]
    Enable = 0x0,
    #[doc = "High Priority"]
    Disable = 0x01,
}
impl M3HighPriority {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M3HighPriority {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M3HighPriority {
    #[inline(always)]
    fn from(val: u8) -> M3HighPriority {
        M3HighPriority::from_bits(val)
    }
}
impl From<M3HighPriority> for u8 {
    #[inline(always)]
    fn from(val: M3HighPriority) -> u8 {
        M3HighPriority::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M4HighPriority {
    #[doc = "Default Priority"]
    Enable = 0x0,
    #[doc = "High Priority"]
    Disable = 0x01,
}
impl M4HighPriority {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M4HighPriority {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M4HighPriority {
    #[inline(always)]
    fn from(val: u8) -> M4HighPriority {
        M4HighPriority::from_bits(val)
    }
}
impl From<M4HighPriority> for u8 {
    #[inline(always)]
    fn from(val: M4HighPriority) -> u8 {
        M4HighPriority::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M5HighPriority {
    #[doc = "Default Priority"]
    Enable = 0x0,
    #[doc = "High Priority"]
    Disable = 0x01,
}
impl M5HighPriority {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M5HighPriority {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M5HighPriority {
    #[inline(always)]
    fn from(val: u8) -> M5HighPriority {
        M5HighPriority::from_bits(val)
    }
}
impl From<M5HighPriority> for u8 {
    #[inline(always)]
    fn from(val: M5HighPriority) -> u8 {
        M5HighPriority::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M6HighPriority {
    #[doc = "Default Priority"]
    Enable = 0x0,
    #[doc = "High Priority"]
    Disable = 0x01,
}
impl M6HighPriority {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M6HighPriority {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M6HighPriority {
    #[inline(always)]
    fn from(val: u8) -> M6HighPriority {
        M6HighPriority::from_bits(val)
    }
}
impl From<M6HighPriority> for u8 {
    #[inline(always)]
    fn from(val: M6HighPriority) -> u8 {
        M6HighPriority::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7CfgTcmSize {
    #[doc = "Regular TCM, 256KB ITCM and 256KB DTCM"]
    RegItcm = 0x0,
    #[doc = "Double ITCM, 512KB ITCM"]
    DoubleItcm = 0x01,
    #[doc = "Double DTCM, 512KB DTCM"]
    DoubleDtcm = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "HALF ITCM, 128KB ITCM and 384KB DTCM"]
    HalfItcm = 0x04,
    #[doc = "HALF DTCM, 384KB ITCM and 128KB DTCM"]
    HalfDtcm = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl M7CfgTcmSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7CfgTcmSize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7CfgTcmSize {
    #[inline(always)]
    fn from(val: u8) -> M7CfgTcmSize {
        M7CfgTcmSize::from_bits(val)
    }
}
impl From<M7CfgTcmSize> for u8 {
    #[inline(always)]
    fn from(val: M7CfgTcmSize) -> u8 {
        M7CfgTcmSize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ocram1InitDone {
    #[doc = "OCRAM1 memory is under initialization"]
    OcramInit = 0x0,
    #[doc = "OCRAM1 memory initialization is complete"]
    OcramInitDone = 0x01,
}
impl Ocram1InitDone {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ocram1InitDone {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ocram1InitDone {
    #[inline(always)]
    fn from(val: u8) -> Ocram1InitDone {
        Ocram1InitDone::from_bits(val)
    }
}
impl From<Ocram1InitDone> for u8 {
    #[inline(always)]
    fn from(val: Ocram1InitDone) -> u8 {
        Ocram1InitDone::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ocram2InitDone {
    #[doc = "OCRAM2 memory is under initialization"]
    OcramInit = 0x0,
    #[doc = "OCRAM2 memory initialization is complete"]
    OcramInitDone = 0x01,
}
impl Ocram2InitDone {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ocram2InitDone {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ocram2InitDone {
    #[inline(always)]
    fn from(val: u8) -> Ocram2InitDone {
        Ocram2InitDone::from_bits(val)
    }
}
impl From<Ocram2InitDone> for u8 {
    #[inline(always)]
    fn from(val: Ocram2InitDone) -> u8 {
        Ocram2InitDone::to_bits(val)
    }
}
