#[doc = "AXBS_AON_CTRL"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AxbsAonCtrl(pub u32);
impl AxbsAonCtrl {
    #[doc = "AXBS_AON Force Round Robin"]
    #[must_use]
    #[inline(always)]
    pub const fn force_round_robin(&self) -> super::vals::ForceRoundRobin {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ForceRoundRobin::from_bits(val as u8)
    }
    #[doc = "AXBS_AON Force Round Robin"]
    #[inline(always)]
    pub const fn set_force_round_robin(&mut self, val: super::vals::ForceRoundRobin) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "M0 High Priority Control Bit"]
    #[must_use]
    #[inline(always)]
    pub const fn m0_high_priority(&self) -> super::vals::M0HighPriority {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::M0HighPriority::from_bits(val as u8)
    }
    #[doc = "M0 High Priority Control Bit"]
    #[inline(always)]
    pub const fn set_m0_high_priority(&mut self, val: super::vals::M0HighPriority) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "M1 High Priority Control Bit"]
    #[must_use]
    #[inline(always)]
    pub const fn m1_high_priority(&self) -> super::vals::M1HighPriority {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::M1HighPriority::from_bits(val as u8)
    }
    #[doc = "M1 High Priority Control Bit"]
    #[inline(always)]
    pub const fn set_m1_high_priority(&mut self, val: super::vals::M1HighPriority) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "M2 High Priority Control Bit"]
    #[must_use]
    #[inline(always)]
    pub const fn m2_high_priority(&self) -> super::vals::M2HighPriority {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::M2HighPriority::from_bits(val as u8)
    }
    #[doc = "M2 High Priority Control Bit"]
    #[inline(always)]
    pub const fn set_m2_high_priority(&mut self, val: super::vals::M2HighPriority) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "M3 High Priority Control Bit"]
    #[must_use]
    #[inline(always)]
    pub const fn m3_high_priority(&self) -> super::vals::M3HighPriority {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::M3HighPriority::from_bits(val as u8)
    }
    #[doc = "M3 High Priority Control Bit"]
    #[inline(always)]
    pub const fn set_m3_high_priority(&mut self, val: super::vals::M3HighPriority) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "M4 High Priority Control Bit"]
    #[must_use]
    #[inline(always)]
    pub const fn m4_high_priority(&self) -> super::vals::M4HighPriority {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::M4HighPriority::from_bits(val as u8)
    }
    #[doc = "M4 High Priority Control Bit"]
    #[inline(always)]
    pub const fn set_m4_high_priority(&mut self, val: super::vals::M4HighPriority) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "M5 High Priority Control Bit"]
    #[must_use]
    #[inline(always)]
    pub const fn m5_high_priority(&self) -> super::vals::M5HighPriority {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::M5HighPriority::from_bits(val as u8)
    }
    #[doc = "M5 High Priority Control Bit"]
    #[inline(always)]
    pub const fn set_m5_high_priority(&mut self, val: super::vals::M5HighPriority) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "M6 High Priority Control Bit"]
    #[must_use]
    #[inline(always)]
    pub const fn m6_high_priority(&self) -> super::vals::M6HighPriority {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::M6HighPriority::from_bits(val as u8)
    }
    #[doc = "M6 High Priority Control Bit"]
    #[inline(always)]
    pub const fn set_m6_high_priority(&mut self, val: super::vals::M6HighPriority) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
}
impl Default for AxbsAonCtrl {
    #[inline(always)]
    fn default() -> AxbsAonCtrl {
        AxbsAonCtrl(1u64 as u32)
    }
}
impl core::fmt::Debug for AxbsAonCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AxbsAonCtrl")
            .field("force_round_robin", &self.force_round_robin())
            .field("m0_high_priority", &self.m0_high_priority())
            .field("m1_high_priority", &self.m1_high_priority())
            .field("m2_high_priority", &self.m2_high_priority())
            .field("m3_high_priority", &self.m3_high_priority())
            .field("m4_high_priority", &self.m4_high_priority())
            .field("m5_high_priority", &self.m5_high_priority())
            .field("m6_high_priority", &self.m6_high_priority())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AxbsAonCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct AxbsAonCtrl {
            force_round_robin: super::vals::ForceRoundRobin,
            m0_high_priority: super::vals::M0HighPriority,
            m1_high_priority: super::vals::M1HighPriority,
            m2_high_priority: super::vals::M2HighPriority,
            m3_high_priority: super::vals::M3HighPriority,
            m4_high_priority: super::vals::M4HighPriority,
            m5_high_priority: super::vals::M5HighPriority,
            m6_high_priority: super::vals::M6HighPriority,
        }
        let proxy = AxbsAonCtrl {
            force_round_robin: self.force_round_robin(),
            m0_high_priority: self.m0_high_priority(),
            m1_high_priority: self.m1_high_priority(),
            m2_high_priority: self.m2_high_priority(),
            m3_high_priority: self.m3_high_priority(),
            m4_high_priority: self.m4_high_priority(),
            m5_high_priority: self.m5_high_priority(),
            m6_high_priority: self.m6_high_priority(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "CM33_IRQ_MASK0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm33IrqMask0(pub u32);
impl Cm33IrqMask0 {
    #[doc = "CM33 IRQ MASK"]
    #[must_use]
    #[inline(always)]
    pub const fn m(&self) -> super::vals::Cm33IrqMask0M {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Cm33IrqMask0M::from_bits(val as u32)
    }
    #[doc = "CM33 IRQ MASK"]
    #[inline(always)]
    pub const fn set_m(&mut self, val: super::vals::Cm33IrqMask0M) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cm33IrqMask0 {
    #[inline(always)]
    fn default() -> Cm33IrqMask0 {
        Cm33IrqMask0(4294967295u64 as u32)
    }
}
impl core::fmt::Debug for Cm33IrqMask0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cm33IrqMask0")
            .field("m", &self.m())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cm33IrqMask0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cm33IrqMask0 {
            m: super::vals::Cm33IrqMask0M,
        }
        let proxy = Cm33IrqMask0 { m: self.m() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "CM33 IRQ MASK1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm33IrqMask1(pub u32);
impl Cm33IrqMask1 {
    #[doc = "CM33 IRQ MASK"]
    #[must_use]
    #[inline(always)]
    pub const fn m(&self) -> super::vals::Cm33IrqMask1M {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Cm33IrqMask1M::from_bits(val as u32)
    }
    #[doc = "CM33 IRQ MASK"]
    #[inline(always)]
    pub const fn set_m(&mut self, val: super::vals::Cm33IrqMask1M) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cm33IrqMask1 {
    #[inline(always)]
    fn default() -> Cm33IrqMask1 {
        Cm33IrqMask1(4294967295u64 as u32)
    }
}
impl core::fmt::Debug for Cm33IrqMask1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cm33IrqMask1")
            .field("m", &self.m())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cm33IrqMask1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cm33IrqMask1 {
            m: super::vals::Cm33IrqMask1M,
        }
        let proxy = Cm33IrqMask1 { m: self.m() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "CM33_IRQ_MASK2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm33IrqMask2(pub u32);
impl Cm33IrqMask2 {
    #[doc = "CM33 IRQ MASK"]
    #[must_use]
    #[inline(always)]
    pub const fn m(&self) -> super::vals::Cm33IrqMask2M {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Cm33IrqMask2M::from_bits(val as u32)
    }
    #[doc = "CM33 IRQ MASK"]
    #[inline(always)]
    pub const fn set_m(&mut self, val: super::vals::Cm33IrqMask2M) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cm33IrqMask2 {
    #[inline(always)]
    fn default() -> Cm33IrqMask2 {
        Cm33IrqMask2(4294967295u64 as u32)
    }
}
impl core::fmt::Debug for Cm33IrqMask2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cm33IrqMask2")
            .field("m", &self.m())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cm33IrqMask2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cm33IrqMask2 {
            m: super::vals::Cm33IrqMask2M,
        }
        let proxy = Cm33IrqMask2 { m: self.m() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "CM33_IRQ_MASK3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm33IrqMask3(pub u32);
impl Cm33IrqMask3 {
    #[doc = "CM33 IRQ MASK"]
    #[must_use]
    #[inline(always)]
    pub const fn m(&self) -> super::vals::Cm33IrqMask3M {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Cm33IrqMask3M::from_bits(val as u32)
    }
    #[doc = "CM33 IRQ MASK"]
    #[inline(always)]
    pub const fn set_m(&mut self, val: super::vals::Cm33IrqMask3M) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cm33IrqMask3 {
    #[inline(always)]
    fn default() -> Cm33IrqMask3 {
        Cm33IrqMask3(4294967295u64 as u32)
    }
}
impl core::fmt::Debug for Cm33IrqMask3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cm33IrqMask3")
            .field("m", &self.m())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cm33IrqMask3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cm33IrqMask3 {
            m: super::vals::Cm33IrqMask3M,
        }
        let proxy = Cm33IrqMask3 { m: self.m() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "CM33_IRQ_MASK4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm33IrqMask4(pub u32);
impl Cm33IrqMask4 {
    #[doc = "CM33 IRQ MASK"]
    #[must_use]
    #[inline(always)]
    pub const fn m(&self) -> super::vals::Cm33IrqMask4M {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Cm33IrqMask4M::from_bits(val as u32)
    }
    #[doc = "CM33 IRQ MASK"]
    #[inline(always)]
    pub const fn set_m(&mut self, val: super::vals::Cm33IrqMask4M) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cm33IrqMask4 {
    #[inline(always)]
    fn default() -> Cm33IrqMask4 {
        Cm33IrqMask4(4294967295u64 as u32)
    }
}
impl core::fmt::Debug for Cm33IrqMask4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cm33IrqMask4")
            .field("m", &self.m())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cm33IrqMask4 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cm33IrqMask4 {
            m: super::vals::Cm33IrqMask4M,
        }
        let proxy = Cm33IrqMask4 { m: self.m() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "CM33_IRQ_MASK5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm33IrqMask5(pub u32);
impl Cm33IrqMask5 {
    #[doc = "CM33 IRQ MASK"]
    #[must_use]
    #[inline(always)]
    pub const fn m(&self) -> super::vals::Cm33IrqMask5M {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Cm33IrqMask5M::from_bits(val as u32)
    }
    #[doc = "CM33 IRQ MASK"]
    #[inline(always)]
    pub const fn set_m(&mut self, val: super::vals::Cm33IrqMask5M) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cm33IrqMask5 {
    #[inline(always)]
    fn default() -> Cm33IrqMask5 {
        Cm33IrqMask5(4294967295u64 as u32)
    }
}
impl core::fmt::Debug for Cm33IrqMask5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cm33IrqMask5")
            .field("m", &self.m())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cm33IrqMask5 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cm33IrqMask5 {
            m: super::vals::Cm33IrqMask5M,
        }
        let proxy = Cm33IrqMask5 { m: self.m() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "CM33_IRQ_MASK6"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm33IrqMask6(pub u32);
impl Cm33IrqMask6 {
    #[doc = "CM33 IRQ MASK"]
    #[must_use]
    #[inline(always)]
    pub const fn m(&self) -> super::vals::Cm33IrqMask6M {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Cm33IrqMask6M::from_bits(val as u32)
    }
    #[doc = "CM33 IRQ MASK"]
    #[inline(always)]
    pub const fn set_m(&mut self, val: super::vals::Cm33IrqMask6M) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cm33IrqMask6 {
    #[inline(always)]
    fn default() -> Cm33IrqMask6 {
        Cm33IrqMask6(4294967295u64 as u32)
    }
}
impl core::fmt::Debug for Cm33IrqMask6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cm33IrqMask6")
            .field("m", &self.m())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cm33IrqMask6 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cm33IrqMask6 {
            m: super::vals::Cm33IrqMask6M,
        }
        let proxy = Cm33IrqMask6 { m: self.m() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "CM33_IRQ_MASK7"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm33IrqMask7(pub u32);
impl Cm33IrqMask7 {
    #[doc = "CM33 IRQ MASK"]
    #[must_use]
    #[inline(always)]
    pub const fn m(&self) -> super::vals::Cm33IrqMask7M {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Cm33IrqMask7M::from_bits(val as u32)
    }
    #[doc = "CM33 IRQ MASK"]
    #[inline(always)]
    pub const fn set_m(&mut self, val: super::vals::Cm33IrqMask7M) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cm33IrqMask7 {
    #[inline(always)]
    fn default() -> Cm33IrqMask7 {
        Cm33IrqMask7(4294967295u64 as u32)
    }
}
impl core::fmt::Debug for Cm33IrqMask7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cm33IrqMask7")
            .field("m", &self.m())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cm33IrqMask7 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cm33IrqMask7 {
            m: super::vals::Cm33IrqMask7M,
        }
        let proxy = Cm33IrqMask7 { m: self.m() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "CM7_IRQ_MASK0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm7IrqMask0(pub u32);
impl Cm7IrqMask0 {
    #[doc = "CM7 IRQ MASK"]
    #[must_use]
    #[inline(always)]
    pub const fn m(&self) -> super::vals::Cm7IrqMask0M {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Cm7IrqMask0M::from_bits(val as u32)
    }
    #[doc = "CM7 IRQ MASK"]
    #[inline(always)]
    pub const fn set_m(&mut self, val: super::vals::Cm7IrqMask0M) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cm7IrqMask0 {
    #[inline(always)]
    fn default() -> Cm7IrqMask0 {
        Cm7IrqMask0(4294967295u64 as u32)
    }
}
impl core::fmt::Debug for Cm7IrqMask0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cm7IrqMask0").field("m", &self.m()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cm7IrqMask0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cm7IrqMask0 {
            m: super::vals::Cm7IrqMask0M,
        }
        let proxy = Cm7IrqMask0 { m: self.m() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "CM7_IRQ_MASK1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm7IrqMask1(pub u32);
impl Cm7IrqMask1 {
    #[doc = "CM7 IRQ MASK"]
    #[must_use]
    #[inline(always)]
    pub const fn m(&self) -> super::vals::Cm7IrqMask1M {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Cm7IrqMask1M::from_bits(val as u32)
    }
    #[doc = "CM7 IRQ MASK"]
    #[inline(always)]
    pub const fn set_m(&mut self, val: super::vals::Cm7IrqMask1M) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cm7IrqMask1 {
    #[inline(always)]
    fn default() -> Cm7IrqMask1 {
        Cm7IrqMask1(4294967295u64 as u32)
    }
}
impl core::fmt::Debug for Cm7IrqMask1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cm7IrqMask1").field("m", &self.m()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cm7IrqMask1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cm7IrqMask1 {
            m: super::vals::Cm7IrqMask1M,
        }
        let proxy = Cm7IrqMask1 { m: self.m() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "CM7_IRQ_MASK2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm7IrqMask2(pub u32);
impl Cm7IrqMask2 {
    #[doc = "CM7 IRQ MASK"]
    #[must_use]
    #[inline(always)]
    pub const fn m(&self) -> super::vals::Cm7IrqMask2M {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Cm7IrqMask2M::from_bits(val as u32)
    }
    #[doc = "CM7 IRQ MASK"]
    #[inline(always)]
    pub const fn set_m(&mut self, val: super::vals::Cm7IrqMask2M) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cm7IrqMask2 {
    #[inline(always)]
    fn default() -> Cm7IrqMask2 {
        Cm7IrqMask2(4294967295u64 as u32)
    }
}
impl core::fmt::Debug for Cm7IrqMask2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cm7IrqMask2").field("m", &self.m()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cm7IrqMask2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cm7IrqMask2 {
            m: super::vals::Cm7IrqMask2M,
        }
        let proxy = Cm7IrqMask2 { m: self.m() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "CM7_IRQ_MASK3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm7IrqMask3(pub u32);
impl Cm7IrqMask3 {
    #[doc = "CM7 IRQ MASK"]
    #[must_use]
    #[inline(always)]
    pub const fn m(&self) -> super::vals::Cm7IrqMask3M {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Cm7IrqMask3M::from_bits(val as u32)
    }
    #[doc = "CM7 IRQ MASK"]
    #[inline(always)]
    pub const fn set_m(&mut self, val: super::vals::Cm7IrqMask3M) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cm7IrqMask3 {
    #[inline(always)]
    fn default() -> Cm7IrqMask3 {
        Cm7IrqMask3(4294967295u64 as u32)
    }
}
impl core::fmt::Debug for Cm7IrqMask3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cm7IrqMask3").field("m", &self.m()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cm7IrqMask3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cm7IrqMask3 {
            m: super::vals::Cm7IrqMask3M,
        }
        let proxy = Cm7IrqMask3 { m: self.m() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "CM7_IRQ_MASK4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm7IrqMask4(pub u32);
impl Cm7IrqMask4 {
    #[doc = "CM7 IRQ MASK"]
    #[must_use]
    #[inline(always)]
    pub const fn m(&self) -> super::vals::Cm7IrqMask4M {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Cm7IrqMask4M::from_bits(val as u32)
    }
    #[doc = "CM7 IRQ MASK"]
    #[inline(always)]
    pub const fn set_m(&mut self, val: super::vals::Cm7IrqMask4M) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cm7IrqMask4 {
    #[inline(always)]
    fn default() -> Cm7IrqMask4 {
        Cm7IrqMask4(4294967295u64 as u32)
    }
}
impl core::fmt::Debug for Cm7IrqMask4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cm7IrqMask4").field("m", &self.m()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cm7IrqMask4 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cm7IrqMask4 {
            m: super::vals::Cm7IrqMask4M,
        }
        let proxy = Cm7IrqMask4 { m: self.m() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "CM7_IRQ_MASK5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm7IrqMask5(pub u32);
impl Cm7IrqMask5 {
    #[doc = "CM7 IRQ MASK"]
    #[must_use]
    #[inline(always)]
    pub const fn m(&self) -> super::vals::Cm7IrqMask5M {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Cm7IrqMask5M::from_bits(val as u32)
    }
    #[doc = "CM7 IRQ MASK"]
    #[inline(always)]
    pub const fn set_m(&mut self, val: super::vals::Cm7IrqMask5M) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cm7IrqMask5 {
    #[inline(always)]
    fn default() -> Cm7IrqMask5 {
        Cm7IrqMask5(4294967295u64 as u32)
    }
}
impl core::fmt::Debug for Cm7IrqMask5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cm7IrqMask5").field("m", &self.m()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cm7IrqMask5 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cm7IrqMask5 {
            m: super::vals::Cm7IrqMask5M,
        }
        let proxy = Cm7IrqMask5 { m: self.m() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "CM7_IRQ_MASK6"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm7IrqMask6(pub u32);
impl Cm7IrqMask6 {
    #[doc = "CM7 IRQ MASK"]
    #[must_use]
    #[inline(always)]
    pub const fn m(&self) -> super::vals::Cm7IrqMask6M {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Cm7IrqMask6M::from_bits(val as u32)
    }
    #[doc = "CM7 IRQ MASK"]
    #[inline(always)]
    pub const fn set_m(&mut self, val: super::vals::Cm7IrqMask6M) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cm7IrqMask6 {
    #[inline(always)]
    fn default() -> Cm7IrqMask6 {
        Cm7IrqMask6(4294967295u64 as u32)
    }
}
impl core::fmt::Debug for Cm7IrqMask6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cm7IrqMask6").field("m", &self.m()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cm7IrqMask6 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cm7IrqMask6 {
            m: super::vals::Cm7IrqMask6M,
        }
        let proxy = Cm7IrqMask6 { m: self.m() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "CM7_IRQ_MASK7"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm7IrqMask7(pub u32);
impl Cm7IrqMask7 {
    #[doc = "CM7 IRQ MASK"]
    #[must_use]
    #[inline(always)]
    pub const fn m(&self) -> super::vals::Cm7IrqMask7M {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Cm7IrqMask7M::from_bits(val as u32)
    }
    #[doc = "CM7 IRQ MASK"]
    #[inline(always)]
    pub const fn set_m(&mut self, val: super::vals::Cm7IrqMask7M) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cm7IrqMask7 {
    #[inline(always)]
    fn default() -> Cm7IrqMask7 {
        Cm7IrqMask7(4294967295u64 as u32)
    }
}
impl core::fmt::Debug for Cm7IrqMask7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cm7IrqMask7").field("m", &self.m()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cm7IrqMask7 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cm7IrqMask7 {
            m: super::vals::Cm7IrqMask7M,
        }
        let proxy = Cm7IrqMask7 { m: self.m() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "DAP Access Sticky Bit"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DapAccessStkybit(pub u32);
impl DapAccessStkybit {
    #[doc = "DAP access grant bit controlled by Cortex-M33 ROM, once set \"1\" will kept \"1\" unless there is a reset."]
    #[must_use]
    #[inline(always)]
    pub const fn dap_ctr(&self) -> super::vals::DapCtr {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::DapCtr::from_bits(val as u8)
    }
    #[doc = "DAP access grant bit controlled by Cortex-M33 ROM, once set \"1\" will kept \"1\" unless there is a reset."]
    #[inline(always)]
    pub const fn set_dap_ctr(&mut self, val: super::vals::DapCtr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for DapAccessStkybit {
    #[inline(always)]
    fn default() -> DapAccessStkybit {
        DapAccessStkybit(0u64 as u32)
    }
}
impl core::fmt::Debug for DapAccessStkybit {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DapAccessStkybit")
            .field("dap_ctr", &self.dap_ctr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DapAccessStkybit {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DapAccessStkybit {
            dap_ctr: super::vals::DapCtr,
        }
        let proxy = DapAccessStkybit {
            dap_ctr: self.dap_ctr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ECC memory hardware initialization"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EccMemInit(pub u32);
impl EccMemInit {
    #[doc = "OCRAM1 initialization status"]
    #[must_use]
    #[inline(always)]
    pub const fn ocram1_init_done(&self) -> super::vals::Ocram1InitDone {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Ocram1InitDone::from_bits(val as u8)
    }
    #[doc = "OCRAM1 initialization status"]
    #[inline(always)]
    pub const fn set_ocram1_init_done(&mut self, val: super::vals::Ocram1InitDone) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "OCRAM2 initialization status"]
    #[must_use]
    #[inline(always)]
    pub const fn ocram2_init_done(&self) -> super::vals::Ocram2InitDone {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Ocram2InitDone::from_bits(val as u8)
    }
    #[doc = "OCRAM2 initialization status"]
    #[inline(always)]
    pub const fn set_ocram2_init_done(&mut self, val: super::vals::Ocram2InitDone) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for EccMemInit {
    #[inline(always)]
    fn default() -> EccMemInit {
        EccMemInit(0u64 as u32)
    }
}
impl core::fmt::Debug for EccMemInit {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EccMemInit")
            .field("ocram1_init_done", &self.ocram1_init_done())
            .field("ocram2_init_done", &self.ocram2_init_done())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EccMemInit {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EccMemInit {
            ocram1_init_done: super::vals::Ocram1InitDone,
            ocram2_init_done: super::vals::Ocram2InitDone,
        }
        let proxy = EccMemInit {
            ocram1_init_done: self.ocram1_init_done(),
            ocram2_init_done: self.ocram2_init_done(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "EdgeLock halt status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EdgelockHaltSt(pub u32);
impl EdgelockHaltSt {
    #[doc = "EdgeLock halt and clock status"]
    #[must_use]
    #[inline(always)]
    pub const fn edgelock_halt_ack(&self) -> super::vals::EdgelockHaltAck {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::EdgelockHaltAck::from_bits(val as u8)
    }
    #[doc = "EdgeLock halt and clock status"]
    #[inline(always)]
    pub const fn set_edgelock_halt_ack(&mut self, val: super::vals::EdgelockHaltAck) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "EdgeLock halt exit interrupt clear"]
    #[must_use]
    #[inline(always)]
    pub const fn edgelock_halt_exit_irq_clr(&self) -> super::vals::EdgelockHaltExitIrqClr {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::EdgelockHaltExitIrqClr::from_bits(val as u8)
    }
    #[doc = "EdgeLock halt exit interrupt clear"]
    #[inline(always)]
    pub const fn set_edgelock_halt_exit_irq_clr(
        &mut self,
        val: super::vals::EdgelockHaltExitIrqClr,
    ) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
}
impl Default for EdgelockHaltSt {
    #[inline(always)]
    fn default() -> EdgelockHaltSt {
        EdgelockHaltSt(0u64 as u32)
    }
}
impl core::fmt::Debug for EdgelockHaltSt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EdgelockHaltSt")
            .field("edgelock_halt_ack", &self.edgelock_halt_ack())
            .field(
                "edgelock_halt_exit_irq_clr",
                &self.edgelock_halt_exit_irq_clr(),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EdgelockHaltSt {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EdgelockHaltSt {
            edgelock_halt_ack: super::vals::EdgelockHaltAck,
            edgelock_halt_exit_irq_clr: super::vals::EdgelockHaltExitIrqClr,
        }
        let proxy = EdgelockHaltSt {
            edgelock_halt_ack: self.edgelock_halt_ack(),
            edgelock_halt_exit_irq_clr: self.edgelock_halt_exit_irq_clr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "EdgeLock IRQ request mask"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EdgelockIrqMask(pub u32);
impl EdgelockIrqMask {
    #[doc = "EdgeLock Wdog reset interrupt mask"]
    #[must_use]
    #[inline(always)]
    pub const fn wdg_reset(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "EdgeLock Wdog reset interrupt mask"]
    #[inline(always)]
    pub const fn set_wdg_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "EdgeLock PUF reset interrupt mask"]
    #[must_use]
    #[inline(always)]
    pub const fn puf_reset(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "EdgeLock PUF reset interrupt mask"]
    #[inline(always)]
    pub const fn set_puf_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "EdgeLock LMDA life cycle bricked interrupt mask"]
    #[must_use]
    #[inline(always)]
    pub const fn lc_bricked(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "EdgeLock LMDA life cycle bricked interrupt mask"]
    #[inline(always)]
    pub const fn set_lc_bricked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "EdgeLock system failure interrupt mask"]
    #[must_use]
    #[inline(always)]
    pub const fn lmda_sys_fail(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "EdgeLock system failure interrupt mask"]
    #[inline(always)]
    pub const fn set_lmda_sys_fail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "EdgeLock 32k clock loss interrupt mask"]
    #[must_use]
    #[inline(always)]
    pub const fn noclk_32k(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "EdgeLock 32k clock loss interrupt mask"]
    #[inline(always)]
    pub const fn set_noclk_32k(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "EdgeLock LMDA reset request interrupt mask"]
    #[must_use]
    #[inline(always)]
    pub const fn lmda_reset_req(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "EdgeLock LMDA reset request interrupt mask"]
    #[inline(always)]
    pub const fn set_lmda_reset_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "EdgeLock LMDA reset request from 32k clock domain interrupt mask"]
    #[must_use]
    #[inline(always)]
    pub const fn lmda_32k_reset_req(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "EdgeLock LMDA reset request from 32k clock domain interrupt mask"]
    #[inline(always)]
    pub const fn set_lmda_32k_reset_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "EdgeLock cm33 root clock loss interrupt mask"]
    #[must_use]
    #[inline(always)]
    pub const fn noclk_ref1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "EdgeLock cm33 root clock loss interrupt mask"]
    #[inline(always)]
    pub const fn set_noclk_ref1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "EdgeLock OSC 24Mhz clock loss interrupt mask"]
    #[must_use]
    #[inline(always)]
    pub const fn noclk_ref2(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "EdgeLock OSC 24Mhz clock loss interrupt mask"]
    #[inline(always)]
    pub const fn set_noclk_ref2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for EdgelockIrqMask {
    #[inline(always)]
    fn default() -> EdgelockIrqMask {
        EdgelockIrqMask(503u64 as u32)
    }
}
impl core::fmt::Debug for EdgelockIrqMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EdgelockIrqMask")
            .field("wdg_reset", &self.wdg_reset())
            .field("puf_reset", &self.puf_reset())
            .field("lc_bricked", &self.lc_bricked())
            .field("lmda_sys_fail", &self.lmda_sys_fail())
            .field("noclk_32k", &self.noclk_32k())
            .field("lmda_reset_req", &self.lmda_reset_req())
            .field("lmda_32k_reset_req", &self.lmda_32k_reset_req())
            .field("noclk_ref1", &self.noclk_ref1())
            .field("noclk_ref2", &self.noclk_ref2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EdgelockIrqMask {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EdgelockIrqMask {
            wdg_reset: bool,
            puf_reset: bool,
            lc_bricked: bool,
            lmda_sys_fail: bool,
            noclk_32k: bool,
            lmda_reset_req: bool,
            lmda_32k_reset_req: bool,
            noclk_ref1: bool,
            noclk_ref2: bool,
        }
        let proxy = EdgelockIrqMask {
            wdg_reset: self.wdg_reset(),
            puf_reset: self.puf_reset(),
            lc_bricked: self.lc_bricked(),
            lmda_sys_fail: self.lmda_sys_fail(),
            noclk_32k: self.noclk_32k(),
            lmda_reset_req: self.lmda_reset_req(),
            lmda_32k_reset_req: self.lmda_32k_reset_req(),
            noclk_ref1: self.noclk_ref1(),
            noclk_ref2: self.noclk_ref2(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "EdgeLock reset request mask"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EdgelockResetReqMask(pub u32);
impl EdgelockResetReqMask {
    #[doc = "EdgeLock Wdog reset mask"]
    #[must_use]
    #[inline(always)]
    pub const fn wdg_reset(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "EdgeLock Wdog reset mask"]
    #[inline(always)]
    pub const fn set_wdg_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "EdgeLock PUF reset mask"]
    #[must_use]
    #[inline(always)]
    pub const fn puf_reset(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "EdgeLock PUF reset mask"]
    #[inline(always)]
    pub const fn set_puf_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "EdgeLock LMDA life cycle bricked reset mask"]
    #[must_use]
    #[inline(always)]
    pub const fn lc_bricked(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "EdgeLock LMDA life cycle bricked reset mask"]
    #[inline(always)]
    pub const fn set_lc_bricked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "EdgeLock system failure reset mask"]
    #[must_use]
    #[inline(always)]
    pub const fn lmda_sys_fail(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "EdgeLock system failure reset mask"]
    #[inline(always)]
    pub const fn set_lmda_sys_fail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "EdgeLock 32k clock loss reset mask"]
    #[must_use]
    #[inline(always)]
    pub const fn noclk_32k(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "EdgeLock 32k clock loss reset mask"]
    #[inline(always)]
    pub const fn set_noclk_32k(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "EdgeLock LMDA reset request mask"]
    #[must_use]
    #[inline(always)]
    pub const fn lmda_reset_req(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "EdgeLock LMDA reset request mask"]
    #[inline(always)]
    pub const fn set_lmda_reset_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "EdgeLock LMDA reset request from 32k clock domain mask"]
    #[must_use]
    #[inline(always)]
    pub const fn lmda_32k_reset_req(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "EdgeLock LMDA reset request from 32k clock domain mask"]
    #[inline(always)]
    pub const fn set_lmda_32k_reset_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "EdgeLock CM33 root clock loss reset mask"]
    #[must_use]
    #[inline(always)]
    pub const fn noclk_ref1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "EdgeLock CM33 root clock loss reset mask"]
    #[inline(always)]
    pub const fn set_noclk_ref1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "EdgeLock OSC 24Mhz clock loss reset mask"]
    #[must_use]
    #[inline(always)]
    pub const fn noclk_ref2(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "EdgeLock OSC 24Mhz clock loss reset mask"]
    #[inline(always)]
    pub const fn set_noclk_ref2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for EdgelockResetReqMask {
    #[inline(always)]
    fn default() -> EdgelockResetReqMask {
        EdgelockResetReqMask(402u64 as u32)
    }
}
impl core::fmt::Debug for EdgelockResetReqMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EdgelockResetReqMask")
            .field("wdg_reset", &self.wdg_reset())
            .field("puf_reset", &self.puf_reset())
            .field("lc_bricked", &self.lc_bricked())
            .field("lmda_sys_fail", &self.lmda_sys_fail())
            .field("noclk_32k", &self.noclk_32k())
            .field("lmda_reset_req", &self.lmda_reset_req())
            .field("lmda_32k_reset_req", &self.lmda_32k_reset_req())
            .field("noclk_ref1", &self.noclk_ref1())
            .field("noclk_ref2", &self.noclk_ref2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EdgelockResetReqMask {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EdgelockResetReqMask {
            wdg_reset: bool,
            puf_reset: bool,
            lc_bricked: bool,
            lmda_sys_fail: bool,
            noclk_32k: bool,
            lmda_reset_req: bool,
            lmda_32k_reset_req: bool,
            noclk_ref1: bool,
            noclk_ref2: bool,
        }
        let proxy = EdgelockResetReqMask {
            wdg_reset: self.wdg_reset(),
            puf_reset: self.puf_reset(),
            lc_bricked: self.lc_bricked(),
            lmda_sys_fail: self.lmda_sys_fail(),
            noclk_32k: self.noclk_32k(),
            lmda_reset_req: self.lmda_reset_req(),
            lmda_32k_reset_req: self.lmda_32k_reset_req(),
            noclk_ref1: self.noclk_ref1(),
            noclk_ref2: self.noclk_ref2(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "IOMUXC_AON domain configure"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IomuxcAonDomainCfg(pub u32);
impl IomuxcAonDomainCfg {
    #[doc = "Domain ID 0"]
    #[must_use]
    #[inline(always)]
    pub const fn did0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain ID 0"]
    #[inline(always)]
    pub const fn set_did0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Domain ID 1"]
    #[must_use]
    #[inline(always)]
    pub const fn did1(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain ID 1"]
    #[inline(always)]
    pub const fn set_did1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Domain ID 2"]
    #[must_use]
    #[inline(always)]
    pub const fn did2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain ID 2"]
    #[inline(always)]
    pub const fn set_did2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Domain ID 3"]
    #[must_use]
    #[inline(always)]
    pub const fn did3(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain ID 3"]
    #[inline(always)]
    pub const fn set_did3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Lock bit"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for IomuxcAonDomainCfg {
    #[inline(always)]
    fn default() -> IomuxcAonDomainCfg {
        IomuxcAonDomainCfg(1056u64 as u32)
    }
}
impl core::fmt::Debug for IomuxcAonDomainCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IomuxcAonDomainCfg")
            .field("did0", &self.did0())
            .field("did1", &self.did1())
            .field("did2", &self.did2())
            .field("did3", &self.did3())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IomuxcAonDomainCfg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct IomuxcAonDomainCfg {
            did0: u8,
            did1: u8,
            did2: u8,
            did3: u8,
            lock: bool,
        }
        let proxy = IomuxcAonDomainCfg {
            did0: self.did0(),
            did1: self.did1(),
            did2: self.did2(),
            did3: self.did3(),
            lock: self.lock(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "IOMUXC domain configure"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IomuxcDomainCfg(pub u32);
impl IomuxcDomainCfg {
    #[doc = "Domain ID 0"]
    #[must_use]
    #[inline(always)]
    pub const fn did0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain ID 0"]
    #[inline(always)]
    pub const fn set_did0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Domain ID 1"]
    #[must_use]
    #[inline(always)]
    pub const fn did1(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain ID 1"]
    #[inline(always)]
    pub const fn set_did1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Domain ID 2"]
    #[must_use]
    #[inline(always)]
    pub const fn did2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain ID 2"]
    #[inline(always)]
    pub const fn set_did2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Domain ID 3"]
    #[must_use]
    #[inline(always)]
    pub const fn did3(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain ID 3"]
    #[inline(always)]
    pub const fn set_did3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Lock bit"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for IomuxcDomainCfg {
    #[inline(always)]
    fn default() -> IomuxcDomainCfg {
        IomuxcDomainCfg(1056u64 as u32)
    }
}
impl core::fmt::Debug for IomuxcDomainCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IomuxcDomainCfg")
            .field("did0", &self.did0())
            .field("did1", &self.did1())
            .field("did2", &self.did2())
            .field("did3", &self.did3())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IomuxcDomainCfg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct IomuxcDomainCfg {
            did0: u8,
            did1: u8,
            did2: u8,
            did3: u8,
            lock: bool,
        }
        let proxy = IomuxcDomainCfg {
            did0: self.did0(),
            did1: self.did1(),
            did2: self.did2(),
            did3: self.did3(),
            lock: self.lock(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Low power handshake enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpHandshake(pub u32);
impl LpHandshake {
    #[doc = "CM33 reset handshake enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cm33_reset_hs_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CM33 reset handshake enable"]
    #[inline(always)]
    pub const fn set_cm33_reset_hs_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "CM7 reset handshake enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cm7_reset_hs_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "CM7 reset handshake enable"]
    #[inline(always)]
    pub const fn set_cm7_reset_hs_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CM7 suspend exit reset handshake enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cm7_suspend_hs_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CM7 suspend exit reset handshake enable"]
    #[inline(always)]
    pub const fn set_cm7_suspend_hs_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "AONMIX reset handshake enable"]
    #[must_use]
    #[inline(always)]
    pub const fn aonmix_reset_hs_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "AONMIX reset handshake enable"]
    #[inline(always)]
    pub const fn set_aonmix_reset_hs_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Wakeupmix reset handshake enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wakeupmix_reset_hs_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Wakeupmix reset handshake enable"]
    #[inline(always)]
    pub const fn set_wakeupmix_reset_hs_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Megamix reset handshake enable"]
    #[must_use]
    #[inline(always)]
    pub const fn megamix_reset_hs_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Megamix reset handshake enable"]
    #[inline(always)]
    pub const fn set_megamix_reset_hs_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Megamix low power mode exit reset handshake enable"]
    #[must_use]
    #[inline(always)]
    pub const fn megamix_lpm_hs_en(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Megamix low power mode exit reset handshake enable"]
    #[inline(always)]
    pub const fn set_megamix_lpm_hs_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "EDGELOCK clock off handshake enable"]
    #[must_use]
    #[inline(always)]
    pub const fn edgelock_clk_off_hs_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "EDGELOCK clock off handshake enable"]
    #[inline(always)]
    pub const fn set_edgelock_clk_off_hs_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "CM33 clock off handshake enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cm33_clk_off_hs_en(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "CM33 clock off handshake enable"]
    #[inline(always)]
    pub const fn set_cm33_clk_off_hs_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "CM7 clock off handshake enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cm7_clk_off_hs_en(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "CM7 clock off handshake enable"]
    #[inline(always)]
    pub const fn set_cm7_clk_off_hs_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "TRDC clock off handshake enable"]
    #[must_use]
    #[inline(always)]
    pub const fn trdc_clk_off_hs_en(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "TRDC clock off handshake enable"]
    #[inline(always)]
    pub const fn set_trdc_clk_off_hs_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "IEE clock off handshake enable"]
    #[must_use]
    #[inline(always)]
    pub const fn iee_clk_off_hs_en(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "IEE clock off handshake enable"]
    #[inline(always)]
    pub const fn set_iee_clk_off_hs_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "OTFAD1 clock off handshake enable"]
    #[must_use]
    #[inline(always)]
    pub const fn otfad1_clk_off_hs_en(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "OTFAD1 clock off handshake enable"]
    #[inline(always)]
    pub const fn set_otfad1_clk_off_hs_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "OTFAD2 clock off handshake enable"]
    #[must_use]
    #[inline(always)]
    pub const fn otfad2_clk_off_hs_en(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "OTFAD2 clock off handshake enable"]
    #[inline(always)]
    pub const fn set_otfad2_clk_off_hs_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "EDGELOCK clock on handshake enable"]
    #[must_use]
    #[inline(always)]
    pub const fn edgelock_clk_on_hs_en(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "EDGELOCK clock on handshake enable"]
    #[inline(always)]
    pub const fn set_edgelock_clk_on_hs_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "CM33 clock on handshake enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cm33_clk_on_hs_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "CM33 clock on handshake enable"]
    #[inline(always)]
    pub const fn set_cm33_clk_on_hs_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "CM7 clock on handshake enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cm7_clk_on_hs_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "CM7 clock on handshake enable"]
    #[inline(always)]
    pub const fn set_cm7_clk_on_hs_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "TRDC clock on handshake enable"]
    #[must_use]
    #[inline(always)]
    pub const fn trdc_clk_on_hs_en(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "TRDC clock on handshake enable"]
    #[inline(always)]
    pub const fn set_trdc_clk_on_hs_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "IEE clock on handshake enable"]
    #[must_use]
    #[inline(always)]
    pub const fn iee_clk_on_hs_en(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "IEE clock on handshake enable"]
    #[inline(always)]
    pub const fn set_iee_clk_on_hs_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "OTFAD1 clock on handshake enable"]
    #[must_use]
    #[inline(always)]
    pub const fn otfad1_clk_on_hs_en(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "OTFAD1 clock on handshake enable"]
    #[inline(always)]
    pub const fn set_otfad1_clk_on_hs_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "OTFAD2 clock on handshake enable"]
    #[must_use]
    #[inline(always)]
    pub const fn otfad2_clk_on_hs_en(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "OTFAD2 clock on handshake enable"]
    #[inline(always)]
    pub const fn set_otfad2_clk_on_hs_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for LpHandshake {
    #[inline(always)]
    fn default() -> LpHandshake {
        LpHandshake(0u64 as u32)
    }
}
impl core::fmt::Debug for LpHandshake {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LpHandshake")
            .field("cm33_reset_hs_en", &self.cm33_reset_hs_en())
            .field("cm7_reset_hs_en", &self.cm7_reset_hs_en())
            .field("cm7_suspend_hs_en", &self.cm7_suspend_hs_en())
            .field("aonmix_reset_hs_en", &self.aonmix_reset_hs_en())
            .field("wakeupmix_reset_hs_en", &self.wakeupmix_reset_hs_en())
            .field("megamix_reset_hs_en", &self.megamix_reset_hs_en())
            .field("megamix_lpm_hs_en", &self.megamix_lpm_hs_en())
            .field("edgelock_clk_off_hs_en", &self.edgelock_clk_off_hs_en())
            .field("cm33_clk_off_hs_en", &self.cm33_clk_off_hs_en())
            .field("cm7_clk_off_hs_en", &self.cm7_clk_off_hs_en())
            .field("trdc_clk_off_hs_en", &self.trdc_clk_off_hs_en())
            .field("iee_clk_off_hs_en", &self.iee_clk_off_hs_en())
            .field("otfad1_clk_off_hs_en", &self.otfad1_clk_off_hs_en())
            .field("otfad2_clk_off_hs_en", &self.otfad2_clk_off_hs_en())
            .field("edgelock_clk_on_hs_en", &self.edgelock_clk_on_hs_en())
            .field("cm33_clk_on_hs_en", &self.cm33_clk_on_hs_en())
            .field("cm7_clk_on_hs_en", &self.cm7_clk_on_hs_en())
            .field("trdc_clk_on_hs_en", &self.trdc_clk_on_hs_en())
            .field("iee_clk_on_hs_en", &self.iee_clk_on_hs_en())
            .field("otfad1_clk_on_hs_en", &self.otfad1_clk_on_hs_en())
            .field("otfad2_clk_on_hs_en", &self.otfad2_clk_on_hs_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LpHandshake {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct LpHandshake {
            cm33_reset_hs_en: bool,
            cm7_reset_hs_en: bool,
            cm7_suspend_hs_en: bool,
            aonmix_reset_hs_en: bool,
            wakeupmix_reset_hs_en: bool,
            megamix_reset_hs_en: bool,
            megamix_lpm_hs_en: bool,
            edgelock_clk_off_hs_en: bool,
            cm33_clk_off_hs_en: bool,
            cm7_clk_off_hs_en: bool,
            trdc_clk_off_hs_en: bool,
            iee_clk_off_hs_en: bool,
            otfad1_clk_off_hs_en: bool,
            otfad2_clk_off_hs_en: bool,
            edgelock_clk_on_hs_en: bool,
            cm33_clk_on_hs_en: bool,
            cm7_clk_on_hs_en: bool,
            trdc_clk_on_hs_en: bool,
            iee_clk_on_hs_en: bool,
            otfad1_clk_on_hs_en: bool,
            otfad2_clk_on_hs_en: bool,
        }
        let proxy = LpHandshake {
            cm33_reset_hs_en: self.cm33_reset_hs_en(),
            cm7_reset_hs_en: self.cm7_reset_hs_en(),
            cm7_suspend_hs_en: self.cm7_suspend_hs_en(),
            aonmix_reset_hs_en: self.aonmix_reset_hs_en(),
            wakeupmix_reset_hs_en: self.wakeupmix_reset_hs_en(),
            megamix_reset_hs_en: self.megamix_reset_hs_en(),
            megamix_lpm_hs_en: self.megamix_lpm_hs_en(),
            edgelock_clk_off_hs_en: self.edgelock_clk_off_hs_en(),
            cm33_clk_off_hs_en: self.cm33_clk_off_hs_en(),
            cm7_clk_off_hs_en: self.cm7_clk_off_hs_en(),
            trdc_clk_off_hs_en: self.trdc_clk_off_hs_en(),
            iee_clk_off_hs_en: self.iee_clk_off_hs_en(),
            otfad1_clk_off_hs_en: self.otfad1_clk_off_hs_en(),
            otfad2_clk_off_hs_en: self.otfad2_clk_off_hs_en(),
            edgelock_clk_on_hs_en: self.edgelock_clk_on_hs_en(),
            cm33_clk_on_hs_en: self.cm33_clk_on_hs_en(),
            cm7_clk_on_hs_en: self.cm7_clk_on_hs_en(),
            trdc_clk_on_hs_en: self.trdc_clk_on_hs_en(),
            iee_clk_on_hs_en: self.iee_clk_on_hs_en(),
            otfad1_clk_on_hs_en: self.otfad1_clk_on_hs_en(),
            otfad2_clk_on_hs_en: self.otfad2_clk_on_hs_en(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "M33 Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct M33Cfg(pub u32);
impl M33Cfg {
    #[doc = "M33 CPU WAIT"]
    #[must_use]
    #[inline(always)]
    pub const fn wait(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "M33 CPU WAIT"]
    #[inline(always)]
    pub const fn set_wait(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "M33 TCM SIZE"]
    #[must_use]
    #[inline(always)]
    pub const fn tcm_size(&self) -> super::vals::M33CfgTcmSize {
        let val = (self.0 >> 3usize) & 0x03;
        super::vals::M33CfgTcmSize::from_bits(val as u8)
    }
    #[doc = "M33 TCM SIZE"]
    #[inline(always)]
    pub const fn set_tcm_size(&mut self, val: super::vals::M33CfgTcmSize) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
    }
    #[doc = "Force CM33 core clock on in WAIT mode"]
    #[must_use]
    #[inline(always)]
    pub const fn coreclk_force_on(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Force CM33 core clock on in WAIT mode"]
    #[inline(always)]
    pub const fn set_coreclk_force_on(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for M33Cfg {
    #[inline(always)]
    fn default() -> M33Cfg {
        M33Cfg(4u64 as u32)
    }
}
impl core::fmt::Debug for M33Cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M33Cfg")
            .field("wait", &self.wait())
            .field("tcm_size", &self.tcm_size())
            .field("coreclk_force_on", &self.coreclk_force_on())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for M33Cfg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct M33Cfg {
            wait: bool,
            tcm_size: super::vals::M33CfgTcmSize,
            coreclk_force_on: bool,
        }
        let proxy = M33Cfg {
            wait: self.wait(),
            tcm_size: self.tcm_size(),
            coreclk_force_on: self.coreclk_force_on(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "M33 INITNSVTOR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct M33Initnsvtor(pub u32);
impl M33Initnsvtor {
    #[doc = "M33 INITNSVTOR"]
    #[must_use]
    #[inline(always)]
    pub const fn initnsvtor(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "M33 INITNSVTOR"]
    #[inline(always)]
    pub const fn set_initnsvtor(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 0usize)) | (((val as u32) & 0x01ff_ffff) << 0usize);
    }
}
impl Default for M33Initnsvtor {
    #[inline(always)]
    fn default() -> M33Initnsvtor {
        M33Initnsvtor(0u64 as u32)
    }
}
impl core::fmt::Debug for M33Initnsvtor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M33Initnsvtor")
            .field("initnsvtor", &self.initnsvtor())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for M33Initnsvtor {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct M33Initnsvtor {
            initnsvtor: u32,
        }
        let proxy = M33Initnsvtor {
            initnsvtor: self.initnsvtor(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "M33 INITSVTOR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct M33Initsvtor(pub u32);
impl M33Initsvtor {
    #[doc = "M33 INITSVTOR"]
    #[must_use]
    #[inline(always)]
    pub const fn initsvtor(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "M33 INITSVTOR"]
    #[inline(always)]
    pub const fn set_initsvtor(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 0usize)) | (((val as u32) & 0x01ff_ffff) << 0usize);
    }
}
impl Default for M33Initsvtor {
    #[inline(always)]
    fn default() -> M33Initsvtor {
        M33Initsvtor(0u64 as u32)
    }
}
impl core::fmt::Debug for M33Initsvtor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M33Initsvtor")
            .field("initsvtor", &self.initsvtor())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for M33Initsvtor {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct M33Initsvtor {
            initsvtor: u32,
        }
        let proxy = M33Initsvtor {
            initsvtor: self.initsvtor(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "M7 Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct M7Cfg(pub u32);
impl M7Cfg {
    #[doc = "M7 TCM SIZE"]
    #[must_use]
    #[inline(always)]
    pub const fn tcm_size(&self) -> super::vals::M7CfgTcmSize {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::M7CfgTcmSize::from_bits(val as u8)
    }
    #[doc = "M7 TCM SIZE"]
    #[inline(always)]
    pub const fn set_tcm_size(&mut self, val: super::vals::M7CfgTcmSize) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "M7 CPUWAIT"]
    #[must_use]
    #[inline(always)]
    pub const fn wait(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "M7 CPUWAIT"]
    #[inline(always)]
    pub const fn set_wait(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Force CM7 core clock on in WAIT mode"]
    #[must_use]
    #[inline(always)]
    pub const fn coreclk_force_on(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Force CM7 core clock on in WAIT mode"]
    #[inline(always)]
    pub const fn set_coreclk_force_on(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "CM7 platform AHB clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hclk_force_on(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CM7 platform AHB clock enable"]
    #[inline(always)]
    pub const fn set_hclk_force_on(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "M7 INITVTOR\\[31:7\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn initvtor(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "M7 INITVTOR\\[31:7\\]"]
    #[inline(always)]
    pub const fn set_initvtor(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for M7Cfg {
    #[inline(always)]
    fn default() -> M7Cfg {
        M7Cfg(16u64 as u32)
    }
}
impl core::fmt::Debug for M7Cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M7Cfg")
            .field("tcm_size", &self.tcm_size())
            .field("wait", &self.wait())
            .field("coreclk_force_on", &self.coreclk_force_on())
            .field("hclk_force_on", &self.hclk_force_on())
            .field("initvtor", &self.initvtor())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for M7Cfg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct M7Cfg {
            tcm_size: super::vals::M7CfgTcmSize,
            wait: bool,
            coreclk_force_on: bool,
            hclk_force_on: bool,
            initvtor: u32,
        }
        let proxy = M7Cfg {
            tcm_size: self.tcm_size(),
            wait: self.wait(),
            coreclk_force_on: self.coreclk_force_on(),
            hclk_force_on: self.hclk_force_on(),
            initvtor: self.initvtor(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NMI control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NmiCtrl(pub u32);
impl NmiCtrl {
    #[doc = "Mask CM7 NMI pin input"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_nmi_mask(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Mask CM7 NMI pin input"]
    #[inline(always)]
    pub const fn set_m7_nmi_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Mask CM33 NMI pin input"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_nmi_mask(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Mask CM33 NMI pin input"]
    #[inline(always)]
    pub const fn set_m33_nmi_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for NmiCtrl {
    #[inline(always)]
    fn default() -> NmiCtrl {
        NmiCtrl(0u64 as u32)
    }
}
impl core::fmt::Debug for NmiCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NmiCtrl")
            .field("m7_nmi_mask", &self.m7_nmi_mask())
            .field("m33_nmi_mask", &self.m33_nmi_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NmiCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NmiCtrl {
            m7_nmi_mask: bool,
            m33_nmi_mask: bool,
        }
        let proxy = NmiCtrl {
            m7_nmi_mask: self.m7_nmi_mask(),
            m33_nmi_mask: self.m33_nmi_mask(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "s401_ipi_noclk_ref1 clear control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct S401NoclkClearCtrl(pub u32);
impl S401NoclkClearCtrl {
    #[doc = "clear the interrupt or reset source"]
    #[must_use]
    #[inline(always)]
    pub const fn ref1_slow_clear(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "clear the interrupt or reset source"]
    #[inline(always)]
    pub const fn set_ref1_slow_clear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for S401NoclkClearCtrl {
    #[inline(always)]
    fn default() -> S401NoclkClearCtrl {
        S401NoclkClearCtrl(0u64 as u32)
    }
}
impl core::fmt::Debug for S401NoclkClearCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S401NoclkClearCtrl")
            .field("ref1_slow_clear", &self.ref1_slow_clear())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for S401NoclkClearCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct S401NoclkClearCtrl {
            ref1_slow_clear: bool,
        }
        let proxy = S401NoclkClearCtrl {
            ref1_slow_clear: self.ref1_slow_clear(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
