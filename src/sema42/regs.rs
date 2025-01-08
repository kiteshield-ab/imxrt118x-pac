#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate0(pub u8);
impl Gate0 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate0Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate0Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate0Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate0 {
    #[inline(always)]
    fn default() -> Gate0 {
        Gate0(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate0")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate0 {
            gtfsm: super::vals::Gate0Gtfsm,
        }
        let proxy = Gate0 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate1(pub u8);
impl Gate1 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate1Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate1Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate1Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate1 {
    #[inline(always)]
    fn default() -> Gate1 {
        Gate1(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate1")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate1 {
            gtfsm: super::vals::Gate1Gtfsm,
        }
        let proxy = Gate1 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate10(pub u8);
impl Gate10 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate10Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate10Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate10Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate10 {
    #[inline(always)]
    fn default() -> Gate10 {
        Gate10(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate10")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate10 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate10 {
            gtfsm: super::vals::Gate10Gtfsm,
        }
        let proxy = Gate10 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate11(pub u8);
impl Gate11 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate11Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate11Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate11Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate11 {
    #[inline(always)]
    fn default() -> Gate11 {
        Gate11(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate11")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate11 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate11 {
            gtfsm: super::vals::Gate11Gtfsm,
        }
        let proxy = Gate11 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate12(pub u8);
impl Gate12 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate12Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate12Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate12Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate12 {
    #[inline(always)]
    fn default() -> Gate12 {
        Gate12(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate12")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate12 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate12 {
            gtfsm: super::vals::Gate12Gtfsm,
        }
        let proxy = Gate12 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate13(pub u8);
impl Gate13 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate13Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate13Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate13Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate13 {
    #[inline(always)]
    fn default() -> Gate13 {
        Gate13(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate13")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate13 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate13 {
            gtfsm: super::vals::Gate13Gtfsm,
        }
        let proxy = Gate13 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate14(pub u8);
impl Gate14 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate14Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate14Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate14Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate14 {
    #[inline(always)]
    fn default() -> Gate14 {
        Gate14(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate14")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate14 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate14 {
            gtfsm: super::vals::Gate14Gtfsm,
        }
        let proxy = Gate14 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate15(pub u8);
impl Gate15 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate15Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate15Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate15Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate15 {
    #[inline(always)]
    fn default() -> Gate15 {
        Gate15(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate15")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate15 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate15 {
            gtfsm: super::vals::Gate15Gtfsm,
        }
        let proxy = Gate15 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate16(pub u8);
impl Gate16 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate16Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate16Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate16Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate16 {
    #[inline(always)]
    fn default() -> Gate16 {
        Gate16(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate16 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate16")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate16 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate16 {
            gtfsm: super::vals::Gate16Gtfsm,
        }
        let proxy = Gate16 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate17(pub u8);
impl Gate17 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate17Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate17Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate17Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate17 {
    #[inline(always)]
    fn default() -> Gate17 {
        Gate17(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate17 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate17")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate17 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate17 {
            gtfsm: super::vals::Gate17Gtfsm,
        }
        let proxy = Gate17 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate18(pub u8);
impl Gate18 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate18Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate18Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate18Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate18 {
    #[inline(always)]
    fn default() -> Gate18 {
        Gate18(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate18 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate18")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate18 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate18 {
            gtfsm: super::vals::Gate18Gtfsm,
        }
        let proxy = Gate18 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate19(pub u8);
impl Gate19 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate19Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate19Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate19Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate19 {
    #[inline(always)]
    fn default() -> Gate19 {
        Gate19(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate19 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate19")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate19 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate19 {
            gtfsm: super::vals::Gate19Gtfsm,
        }
        let proxy = Gate19 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate2(pub u8);
impl Gate2 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate2Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate2Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate2Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate2 {
    #[inline(always)]
    fn default() -> Gate2 {
        Gate2(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate2")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate2 {
            gtfsm: super::vals::Gate2Gtfsm,
        }
        let proxy = Gate2 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate20(pub u8);
impl Gate20 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate20Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate20Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate20Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate20 {
    #[inline(always)]
    fn default() -> Gate20 {
        Gate20(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate20 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate20")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate20 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate20 {
            gtfsm: super::vals::Gate20Gtfsm,
        }
        let proxy = Gate20 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate21(pub u8);
impl Gate21 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate21Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate21Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate21Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate21 {
    #[inline(always)]
    fn default() -> Gate21 {
        Gate21(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate21 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate21")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate21 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate21 {
            gtfsm: super::vals::Gate21Gtfsm,
        }
        let proxy = Gate21 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate22(pub u8);
impl Gate22 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate22Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate22Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate22Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate22 {
    #[inline(always)]
    fn default() -> Gate22 {
        Gate22(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate22 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate22")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate22 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate22 {
            gtfsm: super::vals::Gate22Gtfsm,
        }
        let proxy = Gate22 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate23(pub u8);
impl Gate23 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate23Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate23Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate23Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate23 {
    #[inline(always)]
    fn default() -> Gate23 {
        Gate23(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate23 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate23")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate23 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate23 {
            gtfsm: super::vals::Gate23Gtfsm,
        }
        let proxy = Gate23 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate24(pub u8);
impl Gate24 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate24Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate24Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate24Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate24 {
    #[inline(always)]
    fn default() -> Gate24 {
        Gate24(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate24 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate24")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate24 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate24 {
            gtfsm: super::vals::Gate24Gtfsm,
        }
        let proxy = Gate24 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate25(pub u8);
impl Gate25 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate25Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate25Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate25Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate25 {
    #[inline(always)]
    fn default() -> Gate25 {
        Gate25(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate25 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate25")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate25 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate25 {
            gtfsm: super::vals::Gate25Gtfsm,
        }
        let proxy = Gate25 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate26(pub u8);
impl Gate26 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate26Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate26Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate26Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate26 {
    #[inline(always)]
    fn default() -> Gate26 {
        Gate26(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate26 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate26")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate26 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate26 {
            gtfsm: super::vals::Gate26Gtfsm,
        }
        let proxy = Gate26 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate27(pub u8);
impl Gate27 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate27Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate27Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate27Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate27 {
    #[inline(always)]
    fn default() -> Gate27 {
        Gate27(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate27 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate27")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate27 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate27 {
            gtfsm: super::vals::Gate27Gtfsm,
        }
        let proxy = Gate27 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate28(pub u8);
impl Gate28 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate28Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate28Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate28Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate28 {
    #[inline(always)]
    fn default() -> Gate28 {
        Gate28(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate28 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate28")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate28 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate28 {
            gtfsm: super::vals::Gate28Gtfsm,
        }
        let proxy = Gate28 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate29(pub u8);
impl Gate29 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate29Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate29Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate29Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate29 {
    #[inline(always)]
    fn default() -> Gate29 {
        Gate29(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate29 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate29")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate29 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate29 {
            gtfsm: super::vals::Gate29Gtfsm,
        }
        let proxy = Gate29 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate3(pub u8);
impl Gate3 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate3Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate3Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate3Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate3 {
    #[inline(always)]
    fn default() -> Gate3 {
        Gate3(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate3")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate3 {
            gtfsm: super::vals::Gate3Gtfsm,
        }
        let proxy = Gate3 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate30(pub u8);
impl Gate30 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate30Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate30Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate30Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate30 {
    #[inline(always)]
    fn default() -> Gate30 {
        Gate30(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate30 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate30")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate30 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate30 {
            gtfsm: super::vals::Gate30Gtfsm,
        }
        let proxy = Gate30 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate31(pub u8);
impl Gate31 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate31Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate31Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate31Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate31 {
    #[inline(always)]
    fn default() -> Gate31 {
        Gate31(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate31 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate31")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate31 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate31 {
            gtfsm: super::vals::Gate31Gtfsm,
        }
        let proxy = Gate31 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate32(pub u8);
impl Gate32 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate32Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate32Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate32Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate32 {
    #[inline(always)]
    fn default() -> Gate32 {
        Gate32(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate32")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate32 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate32 {
            gtfsm: super::vals::Gate32Gtfsm,
        }
        let proxy = Gate32 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate33(pub u8);
impl Gate33 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate33Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate33Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate33Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate33 {
    #[inline(always)]
    fn default() -> Gate33 {
        Gate33(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate33 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate33")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate33 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate33 {
            gtfsm: super::vals::Gate33Gtfsm,
        }
        let proxy = Gate33 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate34(pub u8);
impl Gate34 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate34Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate34Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate34Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate34 {
    #[inline(always)]
    fn default() -> Gate34 {
        Gate34(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate34 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate34")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate34 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate34 {
            gtfsm: super::vals::Gate34Gtfsm,
        }
        let proxy = Gate34 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate35(pub u8);
impl Gate35 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate35Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate35Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate35Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate35 {
    #[inline(always)]
    fn default() -> Gate35 {
        Gate35(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate35 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate35")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate35 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate35 {
            gtfsm: super::vals::Gate35Gtfsm,
        }
        let proxy = Gate35 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate36(pub u8);
impl Gate36 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate36Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate36Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate36Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate36 {
    #[inline(always)]
    fn default() -> Gate36 {
        Gate36(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate36 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate36")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate36 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate36 {
            gtfsm: super::vals::Gate36Gtfsm,
        }
        let proxy = Gate36 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate37(pub u8);
impl Gate37 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate37Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate37Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate37Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate37 {
    #[inline(always)]
    fn default() -> Gate37 {
        Gate37(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate37 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate37")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate37 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate37 {
            gtfsm: super::vals::Gate37Gtfsm,
        }
        let proxy = Gate37 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate38(pub u8);
impl Gate38 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate38Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate38Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate38Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate38 {
    #[inline(always)]
    fn default() -> Gate38 {
        Gate38(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate38 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate38")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate38 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate38 {
            gtfsm: super::vals::Gate38Gtfsm,
        }
        let proxy = Gate38 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate39(pub u8);
impl Gate39 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate39Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate39Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate39Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate39 {
    #[inline(always)]
    fn default() -> Gate39 {
        Gate39(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate39 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate39")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate39 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate39 {
            gtfsm: super::vals::Gate39Gtfsm,
        }
        let proxy = Gate39 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate4(pub u8);
impl Gate4 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate4Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate4Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate4Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate4 {
    #[inline(always)]
    fn default() -> Gate4 {
        Gate4(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate4")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate4 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate4 {
            gtfsm: super::vals::Gate4Gtfsm,
        }
        let proxy = Gate4 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate40(pub u8);
impl Gate40 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate40Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate40Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate40Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate40 {
    #[inline(always)]
    fn default() -> Gate40 {
        Gate40(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate40 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate40")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate40 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate40 {
            gtfsm: super::vals::Gate40Gtfsm,
        }
        let proxy = Gate40 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate41(pub u8);
impl Gate41 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate41Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate41Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate41Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate41 {
    #[inline(always)]
    fn default() -> Gate41 {
        Gate41(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate41 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate41")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate41 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate41 {
            gtfsm: super::vals::Gate41Gtfsm,
        }
        let proxy = Gate41 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate42(pub u8);
impl Gate42 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate42Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate42Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate42Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate42 {
    #[inline(always)]
    fn default() -> Gate42 {
        Gate42(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate42 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate42")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate42 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate42 {
            gtfsm: super::vals::Gate42Gtfsm,
        }
        let proxy = Gate42 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate43(pub u8);
impl Gate43 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate43Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate43Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate43Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate43 {
    #[inline(always)]
    fn default() -> Gate43 {
        Gate43(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate43 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate43")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate43 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate43 {
            gtfsm: super::vals::Gate43Gtfsm,
        }
        let proxy = Gate43 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate44(pub u8);
impl Gate44 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate44Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate44Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate44Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate44 {
    #[inline(always)]
    fn default() -> Gate44 {
        Gate44(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate44 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate44")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate44 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate44 {
            gtfsm: super::vals::Gate44Gtfsm,
        }
        let proxy = Gate44 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate45(pub u8);
impl Gate45 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate45Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate45Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate45Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate45 {
    #[inline(always)]
    fn default() -> Gate45 {
        Gate45(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate45 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate45")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate45 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate45 {
            gtfsm: super::vals::Gate45Gtfsm,
        }
        let proxy = Gate45 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate46(pub u8);
impl Gate46 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate46Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate46Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate46Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate46 {
    #[inline(always)]
    fn default() -> Gate46 {
        Gate46(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate46 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate46")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate46 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate46 {
            gtfsm: super::vals::Gate46Gtfsm,
        }
        let proxy = Gate46 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate47(pub u8);
impl Gate47 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate47Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate47Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate47Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate47 {
    #[inline(always)]
    fn default() -> Gate47 {
        Gate47(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate47 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate47")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate47 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate47 {
            gtfsm: super::vals::Gate47Gtfsm,
        }
        let proxy = Gate47 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate48(pub u8);
impl Gate48 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate48Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate48Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate48Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate48 {
    #[inline(always)]
    fn default() -> Gate48 {
        Gate48(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate48 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate48")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate48 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate48 {
            gtfsm: super::vals::Gate48Gtfsm,
        }
        let proxy = Gate48 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate49(pub u8);
impl Gate49 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate49Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate49Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate49Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate49 {
    #[inline(always)]
    fn default() -> Gate49 {
        Gate49(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate49 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate49")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate49 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate49 {
            gtfsm: super::vals::Gate49Gtfsm,
        }
        let proxy = Gate49 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate5(pub u8);
impl Gate5 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate5Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate5Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate5Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate5 {
    #[inline(always)]
    fn default() -> Gate5 {
        Gate5(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate5")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate5 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate5 {
            gtfsm: super::vals::Gate5Gtfsm,
        }
        let proxy = Gate5 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate50(pub u8);
impl Gate50 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate50Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate50Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate50Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate50 {
    #[inline(always)]
    fn default() -> Gate50 {
        Gate50(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate50 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate50")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate50 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate50 {
            gtfsm: super::vals::Gate50Gtfsm,
        }
        let proxy = Gate50 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate51(pub u8);
impl Gate51 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate51Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate51Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate51Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate51 {
    #[inline(always)]
    fn default() -> Gate51 {
        Gate51(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate51 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate51")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate51 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate51 {
            gtfsm: super::vals::Gate51Gtfsm,
        }
        let proxy = Gate51 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate52(pub u8);
impl Gate52 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate52Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate52Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate52Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate52 {
    #[inline(always)]
    fn default() -> Gate52 {
        Gate52(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate52 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate52")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate52 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate52 {
            gtfsm: super::vals::Gate52Gtfsm,
        }
        let proxy = Gate52 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate53(pub u8);
impl Gate53 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate53Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate53Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate53Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate53 {
    #[inline(always)]
    fn default() -> Gate53 {
        Gate53(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate53 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate53")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate53 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate53 {
            gtfsm: super::vals::Gate53Gtfsm,
        }
        let proxy = Gate53 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate54(pub u8);
impl Gate54 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate54Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate54Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate54Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate54 {
    #[inline(always)]
    fn default() -> Gate54 {
        Gate54(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate54 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate54")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate54 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate54 {
            gtfsm: super::vals::Gate54Gtfsm,
        }
        let proxy = Gate54 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate55(pub u8);
impl Gate55 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate55Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate55Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate55Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate55 {
    #[inline(always)]
    fn default() -> Gate55 {
        Gate55(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate55 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate55")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate55 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate55 {
            gtfsm: super::vals::Gate55Gtfsm,
        }
        let proxy = Gate55 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate56(pub u8);
impl Gate56 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate56Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate56Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate56Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate56 {
    #[inline(always)]
    fn default() -> Gate56 {
        Gate56(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate56 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate56")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate56 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate56 {
            gtfsm: super::vals::Gate56Gtfsm,
        }
        let proxy = Gate56 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate57(pub u8);
impl Gate57 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate57Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate57Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate57Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate57 {
    #[inline(always)]
    fn default() -> Gate57 {
        Gate57(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate57 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate57")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate57 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate57 {
            gtfsm: super::vals::Gate57Gtfsm,
        }
        let proxy = Gate57 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate58(pub u8);
impl Gate58 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate58Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate58Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate58Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate58 {
    #[inline(always)]
    fn default() -> Gate58 {
        Gate58(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate58 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate58")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate58 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate58 {
            gtfsm: super::vals::Gate58Gtfsm,
        }
        let proxy = Gate58 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate59(pub u8);
impl Gate59 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate59Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate59Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate59Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate59 {
    #[inline(always)]
    fn default() -> Gate59 {
        Gate59(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate59 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate59")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate59 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate59 {
            gtfsm: super::vals::Gate59Gtfsm,
        }
        let proxy = Gate59 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate6(pub u8);
impl Gate6 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate6Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate6Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate6Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate6 {
    #[inline(always)]
    fn default() -> Gate6 {
        Gate6(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate6")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate6 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate6 {
            gtfsm: super::vals::Gate6Gtfsm,
        }
        let proxy = Gate6 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate60(pub u8);
impl Gate60 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate60Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate60Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate60Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate60 {
    #[inline(always)]
    fn default() -> Gate60 {
        Gate60(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate60 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate60")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate60 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate60 {
            gtfsm: super::vals::Gate60Gtfsm,
        }
        let proxy = Gate60 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate61(pub u8);
impl Gate61 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate61Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate61Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate61Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate61 {
    #[inline(always)]
    fn default() -> Gate61 {
        Gate61(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate61 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate61")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate61 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate61 {
            gtfsm: super::vals::Gate61Gtfsm,
        }
        let proxy = Gate61 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate62(pub u8);
impl Gate62 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate62Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate62Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate62Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate62 {
    #[inline(always)]
    fn default() -> Gate62 {
        Gate62(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate62 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate62")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate62 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate62 {
            gtfsm: super::vals::Gate62Gtfsm,
        }
        let proxy = Gate62 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate63(pub u8);
impl Gate63 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate63Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate63Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate63Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate63 {
    #[inline(always)]
    fn default() -> Gate63 {
        Gate63(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate63 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate63")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate63 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate63 {
            gtfsm: super::vals::Gate63Gtfsm,
        }
        let proxy = Gate63 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate7(pub u8);
impl Gate7 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate7Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate7Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate7Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate7 {
    #[inline(always)]
    fn default() -> Gate7 {
        Gate7(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate7")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate7 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate7 {
            gtfsm: super::vals::Gate7Gtfsm,
        }
        let proxy = Gate7 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate8(pub u8);
impl Gate8 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate8Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate8Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate8Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate8 {
    #[inline(always)]
    fn default() -> Gate8 {
        Gate8(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate8")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate8 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate8 {
            gtfsm: super::vals::Gate8Gtfsm,
        }
        let proxy = Gate8 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate9(pub u8);
impl Gate9 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate9Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate9Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate9Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate9 {
    #[inline(always)]
    fn default() -> Gate9 {
        Gate9(0u64 as u8)
    }
}
impl core::fmt::Debug for Gate9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate9")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate9 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gate9 {
            gtfsm: super::vals::Gate9Gtfsm,
        }
        let proxy = Gate9 {
            gtfsm: self.gtfsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Reset Gate Read"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RstgtR(pub u16);
impl RstgtR {
    #[doc = "Reset Gate Number"]
    #[must_use]
    #[inline(always)]
    pub const fn rstgtn(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Reset Gate Number"]
    #[inline(always)]
    pub const fn set_rstgtn(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Reset Gate Domain"]
    #[must_use]
    #[inline(always)]
    pub const fn rstgms(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Reset Gate Domain"]
    #[inline(always)]
    pub const fn set_rstgms(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
    #[doc = "Reset Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn rstgsm(&self) -> super::vals::Rstgsm {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Rstgsm::from_bits(val as u8)
    }
    #[doc = "Reset Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_rstgsm(&mut self, val: super::vals::Rstgsm) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
}
impl Default for RstgtR {
    #[inline(always)]
    fn default() -> RstgtR {
        RstgtR(0u64 as u16)
    }
}
impl core::fmt::Debug for RstgtR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RstgtR")
            .field("rstgtn", &self.rstgtn())
            .field("rstgms", &self.rstgms())
            .field("rstgsm", &self.rstgsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RstgtR {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct RstgtR {
            rstgtn: u8,
            rstgms: u8,
            rstgsm: super::vals::Rstgsm,
        }
        let proxy = RstgtR {
            rstgtn: self.rstgtn(),
            rstgms: self.rstgms(),
            rstgsm: self.rstgsm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Reset Gate Write"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RstgtW(pub u16);
impl RstgtW {
    #[doc = "Reset Gate Number"]
    #[must_use]
    #[inline(always)]
    pub const fn rstgtn(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Reset Gate Number"]
    #[inline(always)]
    pub const fn set_rstgtn(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Reset Gate Data Pattern"]
    #[must_use]
    #[inline(always)]
    pub const fn rstgdp(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Reset Gate Data Pattern"]
    #[inline(always)]
    pub const fn set_rstgdp(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for RstgtW {
    #[inline(always)]
    fn default() -> RstgtW {
        RstgtW(0u64 as u16)
    }
}
impl core::fmt::Debug for RstgtW {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RstgtW")
            .field("rstgtn", &self.rstgtn())
            .field("rstgdp", &self.rstgdp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RstgtW {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct RstgtW {
            rstgtn: u8,
            rstgdp: u8,
        }
        let proxy = RstgtW {
            rstgtn: self.rstgtn(),
            rstgdp: self.rstgdp(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
