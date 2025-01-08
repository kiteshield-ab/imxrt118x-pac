#[doc = "Crossbar Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl0(pub u16);
impl Ctrl0 {
    #[doc = "DMA Enable for XBAR_OUT0"]
    #[must_use]
    #[inline(always)]
    pub const fn den0(&self) -> super::vals::Den0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Den0::from_bits(val as u8)
    }
    #[doc = "DMA Enable for XBAR_OUT0"]
    #[inline(always)]
    pub const fn set_den0(&mut self, val: super::vals::Den0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "Interrupt Enable for XBAR_OUT0"]
    #[must_use]
    #[inline(always)]
    pub const fn ien0(&self) -> super::vals::Ien0 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ien0::from_bits(val as u8)
    }
    #[doc = "Interrupt Enable for XBAR_OUT0"]
    #[inline(always)]
    pub const fn set_ien0(&mut self, val: super::vals::Ien0) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Active edge for edge detection on XBAR_OUT0"]
    #[must_use]
    #[inline(always)]
    pub const fn edge0(&self) -> super::vals::Edge0 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Edge0::from_bits(val as u8)
    }
    #[doc = "Active edge for edge detection on XBAR_OUT0"]
    #[inline(always)]
    pub const fn set_edge0(&mut self, val: super::vals::Edge0) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Edge detection status for XBAR_OUT0"]
    #[must_use]
    #[inline(always)]
    pub const fn sts0(&self) -> super::vals::Sts0 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Sts0::from_bits(val as u8)
    }
    #[doc = "Edge detection status for XBAR_OUT0"]
    #[inline(always)]
    pub const fn set_sts0(&mut self, val: super::vals::Sts0) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "DMA Enable for XBAR_OUT1"]
    #[must_use]
    #[inline(always)]
    pub const fn den1(&self) -> super::vals::Den1 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Den1::from_bits(val as u8)
    }
    #[doc = "DMA Enable for XBAR_OUT1"]
    #[inline(always)]
    pub const fn set_den1(&mut self, val: super::vals::Den1) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "Interrupt Enable for XBAR_OUT1"]
    #[must_use]
    #[inline(always)]
    pub const fn ien1(&self) -> super::vals::Ien1 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Ien1::from_bits(val as u8)
    }
    #[doc = "Interrupt Enable for XBAR_OUT1"]
    #[inline(always)]
    pub const fn set_ien1(&mut self, val: super::vals::Ien1) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
    }
    #[doc = "Active edge for edge detection on XBAR_OUT1"]
    #[must_use]
    #[inline(always)]
    pub const fn edge1(&self) -> super::vals::Edge1 {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Edge1::from_bits(val as u8)
    }
    #[doc = "Active edge for edge detection on XBAR_OUT1"]
    #[inline(always)]
    pub const fn set_edge1(&mut self, val: super::vals::Edge1) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u16) & 0x03) << 10usize);
    }
    #[doc = "Edge detection status for XBAR_OUT1"]
    #[must_use]
    #[inline(always)]
    pub const fn sts1(&self) -> super::vals::Sts1 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Sts1::from_bits(val as u8)
    }
    #[doc = "Edge detection status for XBAR_OUT1"]
    #[inline(always)]
    pub const fn set_sts1(&mut self, val: super::vals::Sts1) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
}
impl Default for Ctrl0 {
    #[inline(always)]
    fn default() -> Ctrl0 {
        Ctrl0(0u64 as u16)
    }
}
impl core::fmt::Debug for Ctrl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl0")
            .field("den0", &self.den0())
            .field("ien0", &self.ien0())
            .field("edge0", &self.edge0())
            .field("sts0", &self.sts0())
            .field("den1", &self.den1())
            .field("ien1", &self.ien1())
            .field("edge1", &self.edge1())
            .field("sts1", &self.sts1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ctrl0 {
            den0: super::vals::Den0,
            ien0: super::vals::Ien0,
            edge0: super::vals::Edge0,
            sts0: super::vals::Sts0,
            den1: super::vals::Den1,
            ien1: super::vals::Ien1,
            edge1: super::vals::Edge1,
            sts1: super::vals::Sts1,
        }
        let proxy = Ctrl0 {
            den0: self.den0(),
            ien0: self.ien0(),
            edge0: self.edge0(),
            sts0: self.sts0(),
            den1: self.den1(),
            ien1: self.ien1(),
            edge1: self.edge1(),
            sts1: self.sts1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl1(pub u16);
impl Ctrl1 {
    #[doc = "DMA Enable for XBAR_OUT2"]
    #[must_use]
    #[inline(always)]
    pub const fn den2(&self) -> super::vals::Den2 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Den2::from_bits(val as u8)
    }
    #[doc = "DMA Enable for XBAR_OUT2"]
    #[inline(always)]
    pub const fn set_den2(&mut self, val: super::vals::Den2) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "Interrupt Enable for XBAR_OUT2"]
    #[must_use]
    #[inline(always)]
    pub const fn ien2(&self) -> super::vals::Ien2 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ien2::from_bits(val as u8)
    }
    #[doc = "Interrupt Enable for XBAR_OUT2"]
    #[inline(always)]
    pub const fn set_ien2(&mut self, val: super::vals::Ien2) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Active edge for edge detection on XBAR_OUT2"]
    #[must_use]
    #[inline(always)]
    pub const fn edge2(&self) -> super::vals::Edge2 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Edge2::from_bits(val as u8)
    }
    #[doc = "Active edge for edge detection on XBAR_OUT2"]
    #[inline(always)]
    pub const fn set_edge2(&mut self, val: super::vals::Edge2) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Edge detection status for XBAR_OUT2"]
    #[must_use]
    #[inline(always)]
    pub const fn sts2(&self) -> super::vals::Sts2 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Sts2::from_bits(val as u8)
    }
    #[doc = "Edge detection status for XBAR_OUT2"]
    #[inline(always)]
    pub const fn set_sts2(&mut self, val: super::vals::Sts2) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "DMA Enable for XBAR_OUT3"]
    #[must_use]
    #[inline(always)]
    pub const fn den3(&self) -> super::vals::Den3 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Den3::from_bits(val as u8)
    }
    #[doc = "DMA Enable for XBAR_OUT3"]
    #[inline(always)]
    pub const fn set_den3(&mut self, val: super::vals::Den3) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "Interrupt Enable for XBAR_OUT3"]
    #[must_use]
    #[inline(always)]
    pub const fn ien3(&self) -> super::vals::Ien3 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Ien3::from_bits(val as u8)
    }
    #[doc = "Interrupt Enable for XBAR_OUT3"]
    #[inline(always)]
    pub const fn set_ien3(&mut self, val: super::vals::Ien3) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
    }
    #[doc = "Active edge for edge detection on XBAR_OUT3"]
    #[must_use]
    #[inline(always)]
    pub const fn edge3(&self) -> super::vals::Edge3 {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Edge3::from_bits(val as u8)
    }
    #[doc = "Active edge for edge detection on XBAR_OUT3"]
    #[inline(always)]
    pub const fn set_edge3(&mut self, val: super::vals::Edge3) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u16) & 0x03) << 10usize);
    }
    #[doc = "Edge detection status for XBAR_OUT3"]
    #[must_use]
    #[inline(always)]
    pub const fn sts3(&self) -> super::vals::Sts3 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Sts3::from_bits(val as u8)
    }
    #[doc = "Edge detection status for XBAR_OUT3"]
    #[inline(always)]
    pub const fn set_sts3(&mut self, val: super::vals::Sts3) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
}
impl Default for Ctrl1 {
    #[inline(always)]
    fn default() -> Ctrl1 {
        Ctrl1(0u64 as u16)
    }
}
impl core::fmt::Debug for Ctrl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl1")
            .field("den2", &self.den2())
            .field("ien2", &self.ien2())
            .field("edge2", &self.edge2())
            .field("sts2", &self.sts2())
            .field("den3", &self.den3())
            .field("ien3", &self.ien3())
            .field("edge3", &self.edge3())
            .field("sts3", &self.sts3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ctrl1 {
            den2: super::vals::Den2,
            ien2: super::vals::Ien2,
            edge2: super::vals::Edge2,
            sts2: super::vals::Sts2,
            den3: super::vals::Den3,
            ien3: super::vals::Ien3,
            edge3: super::vals::Edge3,
            sts3: super::vals::Sts3,
        }
        let proxy = Ctrl1 {
            den2: self.den2(),
            ien2: self.ien2(),
            edge2: self.edge2(),
            sts2: self.sts2(),
            den3: self.den3(),
            ien3: self.ien3(),
            edge3: self.edge3(),
            sts3: self.sts3(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel0(pub u16);
impl Sel0 {
    #[doc = "SEL0"]
    #[must_use]
    #[inline(always)]
    pub const fn sel0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL0"]
    #[inline(always)]
    pub const fn set_sel0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL1"]
    #[must_use]
    #[inline(always)]
    pub const fn sel1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL1"]
    #[inline(always)]
    pub const fn set_sel1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel0 {
    #[inline(always)]
    fn default() -> Sel0 {
        Sel0(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel0")
            .field("sel0", &self.sel0())
            .field("sel1", &self.sel1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel0 {
            sel0: u8,
            sel1: u8,
        }
        let proxy = Sel0 {
            sel0: self.sel0(),
            sel1: self.sel1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel1(pub u16);
impl Sel1 {
    #[doc = "SEL2"]
    #[must_use]
    #[inline(always)]
    pub const fn sel2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL2"]
    #[inline(always)]
    pub const fn set_sel2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL3"]
    #[must_use]
    #[inline(always)]
    pub const fn sel3(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL3"]
    #[inline(always)]
    pub const fn set_sel3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel1 {
    #[inline(always)]
    fn default() -> Sel1 {
        Sel1(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel1")
            .field("sel2", &self.sel2())
            .field("sel3", &self.sel3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel1 {
            sel2: u8,
            sel3: u8,
        }
        let proxy = Sel1 {
            sel2: self.sel2(),
            sel3: self.sel3(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel10(pub u16);
impl Sel10 {
    #[doc = "SEL20"]
    #[must_use]
    #[inline(always)]
    pub const fn sel20(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL20"]
    #[inline(always)]
    pub const fn set_sel20(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL21"]
    #[must_use]
    #[inline(always)]
    pub const fn sel21(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL21"]
    #[inline(always)]
    pub const fn set_sel21(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel10 {
    #[inline(always)]
    fn default() -> Sel10 {
        Sel10(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel10")
            .field("sel20", &self.sel20())
            .field("sel21", &self.sel21())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel10 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel10 {
            sel20: u8,
            sel21: u8,
        }
        let proxy = Sel10 {
            sel20: self.sel20(),
            sel21: self.sel21(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel100(pub u16);
impl Sel100 {
    #[doc = "SEL200"]
    #[must_use]
    #[inline(always)]
    pub const fn sel200(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL200"]
    #[inline(always)]
    pub const fn set_sel200(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL201"]
    #[must_use]
    #[inline(always)]
    pub const fn sel201(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL201"]
    #[inline(always)]
    pub const fn set_sel201(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel100 {
    #[inline(always)]
    fn default() -> Sel100 {
        Sel100(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel100 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel100")
            .field("sel200", &self.sel200())
            .field("sel201", &self.sel201())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel100 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel100 {
            sel200: u8,
            sel201: u8,
        }
        let proxy = Sel100 {
            sel200: self.sel200(),
            sel201: self.sel201(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel101(pub u16);
impl Sel101 {
    #[doc = "SEL202"]
    #[must_use]
    #[inline(always)]
    pub const fn sel202(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL202"]
    #[inline(always)]
    pub const fn set_sel202(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL203"]
    #[must_use]
    #[inline(always)]
    pub const fn sel203(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL203"]
    #[inline(always)]
    pub const fn set_sel203(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel101 {
    #[inline(always)]
    fn default() -> Sel101 {
        Sel101(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel101 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel101")
            .field("sel202", &self.sel202())
            .field("sel203", &self.sel203())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel101 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel101 {
            sel202: u8,
            sel203: u8,
        }
        let proxy = Sel101 {
            sel202: self.sel202(),
            sel203: self.sel203(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel102(pub u16);
impl Sel102 {
    #[doc = "SEL204"]
    #[must_use]
    #[inline(always)]
    pub const fn sel204(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL204"]
    #[inline(always)]
    pub const fn set_sel204(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL205"]
    #[must_use]
    #[inline(always)]
    pub const fn sel205(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL205"]
    #[inline(always)]
    pub const fn set_sel205(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel102 {
    #[inline(always)]
    fn default() -> Sel102 {
        Sel102(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel102 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel102")
            .field("sel204", &self.sel204())
            .field("sel205", &self.sel205())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel102 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel102 {
            sel204: u8,
            sel205: u8,
        }
        let proxy = Sel102 {
            sel204: self.sel204(),
            sel205: self.sel205(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel103(pub u16);
impl Sel103 {
    #[doc = "SEL206"]
    #[must_use]
    #[inline(always)]
    pub const fn sel206(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL206"]
    #[inline(always)]
    pub const fn set_sel206(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL207"]
    #[must_use]
    #[inline(always)]
    pub const fn sel207(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL207"]
    #[inline(always)]
    pub const fn set_sel207(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel103 {
    #[inline(always)]
    fn default() -> Sel103 {
        Sel103(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel103 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel103")
            .field("sel206", &self.sel206())
            .field("sel207", &self.sel207())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel103 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel103 {
            sel206: u8,
            sel207: u8,
        }
        let proxy = Sel103 {
            sel206: self.sel206(),
            sel207: self.sel207(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel104(pub u16);
impl Sel104 {
    #[doc = "SEL208"]
    #[must_use]
    #[inline(always)]
    pub const fn sel208(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL208"]
    #[inline(always)]
    pub const fn set_sel208(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL209"]
    #[must_use]
    #[inline(always)]
    pub const fn sel209(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL209"]
    #[inline(always)]
    pub const fn set_sel209(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel104 {
    #[inline(always)]
    fn default() -> Sel104 {
        Sel104(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel104 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel104")
            .field("sel208", &self.sel208())
            .field("sel209", &self.sel209())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel104 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel104 {
            sel208: u8,
            sel209: u8,
        }
        let proxy = Sel104 {
            sel208: self.sel208(),
            sel209: self.sel209(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel105(pub u16);
impl Sel105 {
    #[doc = "SEL210"]
    #[must_use]
    #[inline(always)]
    pub const fn sel210(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL210"]
    #[inline(always)]
    pub const fn set_sel210(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL211"]
    #[must_use]
    #[inline(always)]
    pub const fn sel211(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL211"]
    #[inline(always)]
    pub const fn set_sel211(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel105 {
    #[inline(always)]
    fn default() -> Sel105 {
        Sel105(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel105 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel105")
            .field("sel210", &self.sel210())
            .field("sel211", &self.sel211())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel105 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel105 {
            sel210: u8,
            sel211: u8,
        }
        let proxy = Sel105 {
            sel210: self.sel210(),
            sel211: self.sel211(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel106(pub u16);
impl Sel106 {
    #[doc = "SEL212"]
    #[must_use]
    #[inline(always)]
    pub const fn sel212(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL212"]
    #[inline(always)]
    pub const fn set_sel212(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL213"]
    #[must_use]
    #[inline(always)]
    pub const fn sel213(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL213"]
    #[inline(always)]
    pub const fn set_sel213(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel106 {
    #[inline(always)]
    fn default() -> Sel106 {
        Sel106(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel106 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel106")
            .field("sel212", &self.sel212())
            .field("sel213", &self.sel213())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel106 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel106 {
            sel212: u8,
            sel213: u8,
        }
        let proxy = Sel106 {
            sel212: self.sel212(),
            sel213: self.sel213(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel107(pub u16);
impl Sel107 {
    #[doc = "SEL214"]
    #[must_use]
    #[inline(always)]
    pub const fn sel214(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL214"]
    #[inline(always)]
    pub const fn set_sel214(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL215"]
    #[must_use]
    #[inline(always)]
    pub const fn sel215(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL215"]
    #[inline(always)]
    pub const fn set_sel215(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel107 {
    #[inline(always)]
    fn default() -> Sel107 {
        Sel107(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel107 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel107")
            .field("sel214", &self.sel214())
            .field("sel215", &self.sel215())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel107 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel107 {
            sel214: u8,
            sel215: u8,
        }
        let proxy = Sel107 {
            sel214: self.sel214(),
            sel215: self.sel215(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel108(pub u16);
impl Sel108 {
    #[doc = "SEL216"]
    #[must_use]
    #[inline(always)]
    pub const fn sel216(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL216"]
    #[inline(always)]
    pub const fn set_sel216(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL217"]
    #[must_use]
    #[inline(always)]
    pub const fn sel217(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL217"]
    #[inline(always)]
    pub const fn set_sel217(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel108 {
    #[inline(always)]
    fn default() -> Sel108 {
        Sel108(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel108 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel108")
            .field("sel216", &self.sel216())
            .field("sel217", &self.sel217())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel108 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel108 {
            sel216: u8,
            sel217: u8,
        }
        let proxy = Sel108 {
            sel216: self.sel216(),
            sel217: self.sel217(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel109(pub u16);
impl Sel109 {
    #[doc = "SEL218"]
    #[must_use]
    #[inline(always)]
    pub const fn sel218(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL218"]
    #[inline(always)]
    pub const fn set_sel218(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL219"]
    #[must_use]
    #[inline(always)]
    pub const fn sel219(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL219"]
    #[inline(always)]
    pub const fn set_sel219(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel109 {
    #[inline(always)]
    fn default() -> Sel109 {
        Sel109(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel109 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel109")
            .field("sel218", &self.sel218())
            .field("sel219", &self.sel219())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel109 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel109 {
            sel218: u8,
            sel219: u8,
        }
        let proxy = Sel109 {
            sel218: self.sel218(),
            sel219: self.sel219(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel11(pub u16);
impl Sel11 {
    #[doc = "SEL22"]
    #[must_use]
    #[inline(always)]
    pub const fn sel22(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL22"]
    #[inline(always)]
    pub const fn set_sel22(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL23"]
    #[must_use]
    #[inline(always)]
    pub const fn sel23(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL23"]
    #[inline(always)]
    pub const fn set_sel23(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel11 {
    #[inline(always)]
    fn default() -> Sel11 {
        Sel11(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel11")
            .field("sel22", &self.sel22())
            .field("sel23", &self.sel23())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel11 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel11 {
            sel22: u8,
            sel23: u8,
        }
        let proxy = Sel11 {
            sel22: self.sel22(),
            sel23: self.sel23(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel110(pub u16);
impl Sel110 {
    #[doc = "SEL220"]
    #[must_use]
    #[inline(always)]
    pub const fn sel220(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL220"]
    #[inline(always)]
    pub const fn set_sel220(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
}
impl Default for Sel110 {
    #[inline(always)]
    fn default() -> Sel110 {
        Sel110(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel110 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel110")
            .field("sel220", &self.sel220())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel110 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel110 {
            sel220: u8,
        }
        let proxy = Sel110 {
            sel220: self.sel220(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel12(pub u16);
impl Sel12 {
    #[doc = "SEL24"]
    #[must_use]
    #[inline(always)]
    pub const fn sel24(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL24"]
    #[inline(always)]
    pub const fn set_sel24(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL25"]
    #[must_use]
    #[inline(always)]
    pub const fn sel25(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL25"]
    #[inline(always)]
    pub const fn set_sel25(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel12 {
    #[inline(always)]
    fn default() -> Sel12 {
        Sel12(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel12")
            .field("sel24", &self.sel24())
            .field("sel25", &self.sel25())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel12 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel12 {
            sel24: u8,
            sel25: u8,
        }
        let proxy = Sel12 {
            sel24: self.sel24(),
            sel25: self.sel25(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel13(pub u16);
impl Sel13 {
    #[doc = "SEL26"]
    #[must_use]
    #[inline(always)]
    pub const fn sel26(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL26"]
    #[inline(always)]
    pub const fn set_sel26(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL27"]
    #[must_use]
    #[inline(always)]
    pub const fn sel27(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL27"]
    #[inline(always)]
    pub const fn set_sel27(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel13 {
    #[inline(always)]
    fn default() -> Sel13 {
        Sel13(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel13")
            .field("sel26", &self.sel26())
            .field("sel27", &self.sel27())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel13 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel13 {
            sel26: u8,
            sel27: u8,
        }
        let proxy = Sel13 {
            sel26: self.sel26(),
            sel27: self.sel27(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel14(pub u16);
impl Sel14 {
    #[doc = "SEL28"]
    #[must_use]
    #[inline(always)]
    pub const fn sel28(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL28"]
    #[inline(always)]
    pub const fn set_sel28(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL29"]
    #[must_use]
    #[inline(always)]
    pub const fn sel29(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL29"]
    #[inline(always)]
    pub const fn set_sel29(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel14 {
    #[inline(always)]
    fn default() -> Sel14 {
        Sel14(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel14")
            .field("sel28", &self.sel28())
            .field("sel29", &self.sel29())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel14 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel14 {
            sel28: u8,
            sel29: u8,
        }
        let proxy = Sel14 {
            sel28: self.sel28(),
            sel29: self.sel29(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel15(pub u16);
impl Sel15 {
    #[doc = "SEL30"]
    #[must_use]
    #[inline(always)]
    pub const fn sel30(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL30"]
    #[inline(always)]
    pub const fn set_sel30(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL31"]
    #[must_use]
    #[inline(always)]
    pub const fn sel31(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL31"]
    #[inline(always)]
    pub const fn set_sel31(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel15 {
    #[inline(always)]
    fn default() -> Sel15 {
        Sel15(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel15")
            .field("sel30", &self.sel30())
            .field("sel31", &self.sel31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel15 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel15 {
            sel30: u8,
            sel31: u8,
        }
        let proxy = Sel15 {
            sel30: self.sel30(),
            sel31: self.sel31(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel16(pub u16);
impl Sel16 {
    #[doc = "SEL32"]
    #[must_use]
    #[inline(always)]
    pub const fn sel32(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL32"]
    #[inline(always)]
    pub const fn set_sel32(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL33"]
    #[must_use]
    #[inline(always)]
    pub const fn sel33(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL33"]
    #[inline(always)]
    pub const fn set_sel33(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel16 {
    #[inline(always)]
    fn default() -> Sel16 {
        Sel16(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel16 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel16")
            .field("sel32", &self.sel32())
            .field("sel33", &self.sel33())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel16 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel16 {
            sel32: u8,
            sel33: u8,
        }
        let proxy = Sel16 {
            sel32: self.sel32(),
            sel33: self.sel33(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel17(pub u16);
impl Sel17 {
    #[doc = "SEL34"]
    #[must_use]
    #[inline(always)]
    pub const fn sel34(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL34"]
    #[inline(always)]
    pub const fn set_sel34(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL35"]
    #[must_use]
    #[inline(always)]
    pub const fn sel35(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL35"]
    #[inline(always)]
    pub const fn set_sel35(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel17 {
    #[inline(always)]
    fn default() -> Sel17 {
        Sel17(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel17 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel17")
            .field("sel34", &self.sel34())
            .field("sel35", &self.sel35())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel17 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel17 {
            sel34: u8,
            sel35: u8,
        }
        let proxy = Sel17 {
            sel34: self.sel34(),
            sel35: self.sel35(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel18(pub u16);
impl Sel18 {
    #[doc = "SEL36"]
    #[must_use]
    #[inline(always)]
    pub const fn sel36(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL36"]
    #[inline(always)]
    pub const fn set_sel36(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL37"]
    #[must_use]
    #[inline(always)]
    pub const fn sel37(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL37"]
    #[inline(always)]
    pub const fn set_sel37(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel18 {
    #[inline(always)]
    fn default() -> Sel18 {
        Sel18(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel18 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel18")
            .field("sel36", &self.sel36())
            .field("sel37", &self.sel37())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel18 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel18 {
            sel36: u8,
            sel37: u8,
        }
        let proxy = Sel18 {
            sel36: self.sel36(),
            sel37: self.sel37(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel19(pub u16);
impl Sel19 {
    #[doc = "SEL38"]
    #[must_use]
    #[inline(always)]
    pub const fn sel38(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL38"]
    #[inline(always)]
    pub const fn set_sel38(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL39"]
    #[must_use]
    #[inline(always)]
    pub const fn sel39(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL39"]
    #[inline(always)]
    pub const fn set_sel39(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel19 {
    #[inline(always)]
    fn default() -> Sel19 {
        Sel19(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel19 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel19")
            .field("sel38", &self.sel38())
            .field("sel39", &self.sel39())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel19 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel19 {
            sel38: u8,
            sel39: u8,
        }
        let proxy = Sel19 {
            sel38: self.sel38(),
            sel39: self.sel39(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel2(pub u16);
impl Sel2 {
    #[doc = "SEL4"]
    #[must_use]
    #[inline(always)]
    pub const fn sel4(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL4"]
    #[inline(always)]
    pub const fn set_sel4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL5"]
    #[must_use]
    #[inline(always)]
    pub const fn sel5(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL5"]
    #[inline(always)]
    pub const fn set_sel5(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel2 {
    #[inline(always)]
    fn default() -> Sel2 {
        Sel2(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel2")
            .field("sel4", &self.sel4())
            .field("sel5", &self.sel5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel2 {
            sel4: u8,
            sel5: u8,
        }
        let proxy = Sel2 {
            sel4: self.sel4(),
            sel5: self.sel5(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel20(pub u16);
impl Sel20 {
    #[doc = "SEL40"]
    #[must_use]
    #[inline(always)]
    pub const fn sel40(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL40"]
    #[inline(always)]
    pub const fn set_sel40(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL41"]
    #[must_use]
    #[inline(always)]
    pub const fn sel41(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL41"]
    #[inline(always)]
    pub const fn set_sel41(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel20 {
    #[inline(always)]
    fn default() -> Sel20 {
        Sel20(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel20 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel20")
            .field("sel40", &self.sel40())
            .field("sel41", &self.sel41())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel20 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel20 {
            sel40: u8,
            sel41: u8,
        }
        let proxy = Sel20 {
            sel40: self.sel40(),
            sel41: self.sel41(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel21(pub u16);
impl Sel21 {
    #[doc = "SEL42"]
    #[must_use]
    #[inline(always)]
    pub const fn sel42(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL42"]
    #[inline(always)]
    pub const fn set_sel42(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL43"]
    #[must_use]
    #[inline(always)]
    pub const fn sel43(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL43"]
    #[inline(always)]
    pub const fn set_sel43(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel21 {
    #[inline(always)]
    fn default() -> Sel21 {
        Sel21(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel21 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel21")
            .field("sel42", &self.sel42())
            .field("sel43", &self.sel43())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel21 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel21 {
            sel42: u8,
            sel43: u8,
        }
        let proxy = Sel21 {
            sel42: self.sel42(),
            sel43: self.sel43(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel22(pub u16);
impl Sel22 {
    #[doc = "SEL44"]
    #[must_use]
    #[inline(always)]
    pub const fn sel44(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL44"]
    #[inline(always)]
    pub const fn set_sel44(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL45"]
    #[must_use]
    #[inline(always)]
    pub const fn sel45(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL45"]
    #[inline(always)]
    pub const fn set_sel45(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel22 {
    #[inline(always)]
    fn default() -> Sel22 {
        Sel22(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel22 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel22")
            .field("sel44", &self.sel44())
            .field("sel45", &self.sel45())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel22 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel22 {
            sel44: u8,
            sel45: u8,
        }
        let proxy = Sel22 {
            sel44: self.sel44(),
            sel45: self.sel45(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel23(pub u16);
impl Sel23 {
    #[doc = "SEL46"]
    #[must_use]
    #[inline(always)]
    pub const fn sel46(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL46"]
    #[inline(always)]
    pub const fn set_sel46(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL47"]
    #[must_use]
    #[inline(always)]
    pub const fn sel47(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL47"]
    #[inline(always)]
    pub const fn set_sel47(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel23 {
    #[inline(always)]
    fn default() -> Sel23 {
        Sel23(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel23 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel23")
            .field("sel46", &self.sel46())
            .field("sel47", &self.sel47())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel23 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel23 {
            sel46: u8,
            sel47: u8,
        }
        let proxy = Sel23 {
            sel46: self.sel46(),
            sel47: self.sel47(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel24(pub u16);
impl Sel24 {
    #[doc = "SEL48"]
    #[must_use]
    #[inline(always)]
    pub const fn sel48(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL48"]
    #[inline(always)]
    pub const fn set_sel48(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL49"]
    #[must_use]
    #[inline(always)]
    pub const fn sel49(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL49"]
    #[inline(always)]
    pub const fn set_sel49(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel24 {
    #[inline(always)]
    fn default() -> Sel24 {
        Sel24(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel24 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel24")
            .field("sel48", &self.sel48())
            .field("sel49", &self.sel49())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel24 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel24 {
            sel48: u8,
            sel49: u8,
        }
        let proxy = Sel24 {
            sel48: self.sel48(),
            sel49: self.sel49(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel25(pub u16);
impl Sel25 {
    #[doc = "SEL50"]
    #[must_use]
    #[inline(always)]
    pub const fn sel50(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL50"]
    #[inline(always)]
    pub const fn set_sel50(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL51"]
    #[must_use]
    #[inline(always)]
    pub const fn sel51(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL51"]
    #[inline(always)]
    pub const fn set_sel51(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel25 {
    #[inline(always)]
    fn default() -> Sel25 {
        Sel25(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel25 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel25")
            .field("sel50", &self.sel50())
            .field("sel51", &self.sel51())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel25 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel25 {
            sel50: u8,
            sel51: u8,
        }
        let proxy = Sel25 {
            sel50: self.sel50(),
            sel51: self.sel51(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel26(pub u16);
impl Sel26 {
    #[doc = "SEL52"]
    #[must_use]
    #[inline(always)]
    pub const fn sel52(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL52"]
    #[inline(always)]
    pub const fn set_sel52(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL53"]
    #[must_use]
    #[inline(always)]
    pub const fn sel53(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL53"]
    #[inline(always)]
    pub const fn set_sel53(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel26 {
    #[inline(always)]
    fn default() -> Sel26 {
        Sel26(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel26 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel26")
            .field("sel52", &self.sel52())
            .field("sel53", &self.sel53())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel26 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel26 {
            sel52: u8,
            sel53: u8,
        }
        let proxy = Sel26 {
            sel52: self.sel52(),
            sel53: self.sel53(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel27(pub u16);
impl Sel27 {
    #[doc = "SEL54"]
    #[must_use]
    #[inline(always)]
    pub const fn sel54(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL54"]
    #[inline(always)]
    pub const fn set_sel54(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL55"]
    #[must_use]
    #[inline(always)]
    pub const fn sel55(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL55"]
    #[inline(always)]
    pub const fn set_sel55(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel27 {
    #[inline(always)]
    fn default() -> Sel27 {
        Sel27(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel27 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel27")
            .field("sel54", &self.sel54())
            .field("sel55", &self.sel55())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel27 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel27 {
            sel54: u8,
            sel55: u8,
        }
        let proxy = Sel27 {
            sel54: self.sel54(),
            sel55: self.sel55(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel28(pub u16);
impl Sel28 {
    #[doc = "SEL56"]
    #[must_use]
    #[inline(always)]
    pub const fn sel56(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL56"]
    #[inline(always)]
    pub const fn set_sel56(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL57"]
    #[must_use]
    #[inline(always)]
    pub const fn sel57(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL57"]
    #[inline(always)]
    pub const fn set_sel57(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel28 {
    #[inline(always)]
    fn default() -> Sel28 {
        Sel28(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel28 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel28")
            .field("sel56", &self.sel56())
            .field("sel57", &self.sel57())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel28 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel28 {
            sel56: u8,
            sel57: u8,
        }
        let proxy = Sel28 {
            sel56: self.sel56(),
            sel57: self.sel57(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel29(pub u16);
impl Sel29 {
    #[doc = "SEL58"]
    #[must_use]
    #[inline(always)]
    pub const fn sel58(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL58"]
    #[inline(always)]
    pub const fn set_sel58(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL59"]
    #[must_use]
    #[inline(always)]
    pub const fn sel59(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL59"]
    #[inline(always)]
    pub const fn set_sel59(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel29 {
    #[inline(always)]
    fn default() -> Sel29 {
        Sel29(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel29 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel29")
            .field("sel58", &self.sel58())
            .field("sel59", &self.sel59())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel29 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel29 {
            sel58: u8,
            sel59: u8,
        }
        let proxy = Sel29 {
            sel58: self.sel58(),
            sel59: self.sel59(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel3(pub u16);
impl Sel3 {
    #[doc = "SEL6"]
    #[must_use]
    #[inline(always)]
    pub const fn sel6(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL6"]
    #[inline(always)]
    pub const fn set_sel6(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL7"]
    #[must_use]
    #[inline(always)]
    pub const fn sel7(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL7"]
    #[inline(always)]
    pub const fn set_sel7(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel3 {
    #[inline(always)]
    fn default() -> Sel3 {
        Sel3(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel3")
            .field("sel6", &self.sel6())
            .field("sel7", &self.sel7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel3 {
            sel6: u8,
            sel7: u8,
        }
        let proxy = Sel3 {
            sel6: self.sel6(),
            sel7: self.sel7(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel30(pub u16);
impl Sel30 {
    #[doc = "SEL60"]
    #[must_use]
    #[inline(always)]
    pub const fn sel60(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL60"]
    #[inline(always)]
    pub const fn set_sel60(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL61"]
    #[must_use]
    #[inline(always)]
    pub const fn sel61(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL61"]
    #[inline(always)]
    pub const fn set_sel61(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel30 {
    #[inline(always)]
    fn default() -> Sel30 {
        Sel30(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel30 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel30")
            .field("sel60", &self.sel60())
            .field("sel61", &self.sel61())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel30 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel30 {
            sel60: u8,
            sel61: u8,
        }
        let proxy = Sel30 {
            sel60: self.sel60(),
            sel61: self.sel61(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel31(pub u16);
impl Sel31 {
    #[doc = "SEL62"]
    #[must_use]
    #[inline(always)]
    pub const fn sel62(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL62"]
    #[inline(always)]
    pub const fn set_sel62(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL63"]
    #[must_use]
    #[inline(always)]
    pub const fn sel63(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL63"]
    #[inline(always)]
    pub const fn set_sel63(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel31 {
    #[inline(always)]
    fn default() -> Sel31 {
        Sel31(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel31 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel31")
            .field("sel62", &self.sel62())
            .field("sel63", &self.sel63())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel31 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel31 {
            sel62: u8,
            sel63: u8,
        }
        let proxy = Sel31 {
            sel62: self.sel62(),
            sel63: self.sel63(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel32(pub u16);
impl Sel32 {
    #[doc = "SEL64"]
    #[must_use]
    #[inline(always)]
    pub const fn sel64(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL64"]
    #[inline(always)]
    pub const fn set_sel64(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL65"]
    #[must_use]
    #[inline(always)]
    pub const fn sel65(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL65"]
    #[inline(always)]
    pub const fn set_sel65(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel32 {
    #[inline(always)]
    fn default() -> Sel32 {
        Sel32(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel32")
            .field("sel64", &self.sel64())
            .field("sel65", &self.sel65())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel32 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel32 {
            sel64: u8,
            sel65: u8,
        }
        let proxy = Sel32 {
            sel64: self.sel64(),
            sel65: self.sel65(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel33(pub u16);
impl Sel33 {
    #[doc = "SEL66"]
    #[must_use]
    #[inline(always)]
    pub const fn sel66(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL66"]
    #[inline(always)]
    pub const fn set_sel66(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL67"]
    #[must_use]
    #[inline(always)]
    pub const fn sel67(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL67"]
    #[inline(always)]
    pub const fn set_sel67(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel33 {
    #[inline(always)]
    fn default() -> Sel33 {
        Sel33(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel33 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel33")
            .field("sel66", &self.sel66())
            .field("sel67", &self.sel67())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel33 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel33 {
            sel66: u8,
            sel67: u8,
        }
        let proxy = Sel33 {
            sel66: self.sel66(),
            sel67: self.sel67(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel34(pub u16);
impl Sel34 {
    #[doc = "SEL68"]
    #[must_use]
    #[inline(always)]
    pub const fn sel68(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL68"]
    #[inline(always)]
    pub const fn set_sel68(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL69"]
    #[must_use]
    #[inline(always)]
    pub const fn sel69(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL69"]
    #[inline(always)]
    pub const fn set_sel69(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel34 {
    #[inline(always)]
    fn default() -> Sel34 {
        Sel34(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel34 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel34")
            .field("sel68", &self.sel68())
            .field("sel69", &self.sel69())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel34 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel34 {
            sel68: u8,
            sel69: u8,
        }
        let proxy = Sel34 {
            sel68: self.sel68(),
            sel69: self.sel69(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel35(pub u16);
impl Sel35 {
    #[doc = "SEL70"]
    #[must_use]
    #[inline(always)]
    pub const fn sel70(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL70"]
    #[inline(always)]
    pub const fn set_sel70(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL71"]
    #[must_use]
    #[inline(always)]
    pub const fn sel71(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL71"]
    #[inline(always)]
    pub const fn set_sel71(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel35 {
    #[inline(always)]
    fn default() -> Sel35 {
        Sel35(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel35 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel35")
            .field("sel70", &self.sel70())
            .field("sel71", &self.sel71())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel35 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel35 {
            sel70: u8,
            sel71: u8,
        }
        let proxy = Sel35 {
            sel70: self.sel70(),
            sel71: self.sel71(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel36(pub u16);
impl Sel36 {
    #[doc = "SEL72"]
    #[must_use]
    #[inline(always)]
    pub const fn sel72(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL72"]
    #[inline(always)]
    pub const fn set_sel72(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL73"]
    #[must_use]
    #[inline(always)]
    pub const fn sel73(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL73"]
    #[inline(always)]
    pub const fn set_sel73(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel36 {
    #[inline(always)]
    fn default() -> Sel36 {
        Sel36(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel36 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel36")
            .field("sel72", &self.sel72())
            .field("sel73", &self.sel73())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel36 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel36 {
            sel72: u8,
            sel73: u8,
        }
        let proxy = Sel36 {
            sel72: self.sel72(),
            sel73: self.sel73(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel37(pub u16);
impl Sel37 {
    #[doc = "SEL74"]
    #[must_use]
    #[inline(always)]
    pub const fn sel74(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL74"]
    #[inline(always)]
    pub const fn set_sel74(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL75"]
    #[must_use]
    #[inline(always)]
    pub const fn sel75(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL75"]
    #[inline(always)]
    pub const fn set_sel75(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel37 {
    #[inline(always)]
    fn default() -> Sel37 {
        Sel37(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel37 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel37")
            .field("sel74", &self.sel74())
            .field("sel75", &self.sel75())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel37 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel37 {
            sel74: u8,
            sel75: u8,
        }
        let proxy = Sel37 {
            sel74: self.sel74(),
            sel75: self.sel75(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel38(pub u16);
impl Sel38 {
    #[doc = "SEL76"]
    #[must_use]
    #[inline(always)]
    pub const fn sel76(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL76"]
    #[inline(always)]
    pub const fn set_sel76(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL77"]
    #[must_use]
    #[inline(always)]
    pub const fn sel77(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL77"]
    #[inline(always)]
    pub const fn set_sel77(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel38 {
    #[inline(always)]
    fn default() -> Sel38 {
        Sel38(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel38 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel38")
            .field("sel76", &self.sel76())
            .field("sel77", &self.sel77())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel38 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel38 {
            sel76: u8,
            sel77: u8,
        }
        let proxy = Sel38 {
            sel76: self.sel76(),
            sel77: self.sel77(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel39(pub u16);
impl Sel39 {
    #[doc = "SEL78"]
    #[must_use]
    #[inline(always)]
    pub const fn sel78(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL78"]
    #[inline(always)]
    pub const fn set_sel78(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL79"]
    #[must_use]
    #[inline(always)]
    pub const fn sel79(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL79"]
    #[inline(always)]
    pub const fn set_sel79(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel39 {
    #[inline(always)]
    fn default() -> Sel39 {
        Sel39(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel39 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel39")
            .field("sel78", &self.sel78())
            .field("sel79", &self.sel79())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel39 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel39 {
            sel78: u8,
            sel79: u8,
        }
        let proxy = Sel39 {
            sel78: self.sel78(),
            sel79: self.sel79(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel4(pub u16);
impl Sel4 {
    #[doc = "SEL8"]
    #[must_use]
    #[inline(always)]
    pub const fn sel8(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL8"]
    #[inline(always)]
    pub const fn set_sel8(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL9"]
    #[must_use]
    #[inline(always)]
    pub const fn sel9(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL9"]
    #[inline(always)]
    pub const fn set_sel9(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel4 {
    #[inline(always)]
    fn default() -> Sel4 {
        Sel4(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel4")
            .field("sel8", &self.sel8())
            .field("sel9", &self.sel9())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel4 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel4 {
            sel8: u8,
            sel9: u8,
        }
        let proxy = Sel4 {
            sel8: self.sel8(),
            sel9: self.sel9(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel40(pub u16);
impl Sel40 {
    #[doc = "SEL80"]
    #[must_use]
    #[inline(always)]
    pub const fn sel80(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL80"]
    #[inline(always)]
    pub const fn set_sel80(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL81"]
    #[must_use]
    #[inline(always)]
    pub const fn sel81(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL81"]
    #[inline(always)]
    pub const fn set_sel81(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel40 {
    #[inline(always)]
    fn default() -> Sel40 {
        Sel40(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel40 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel40")
            .field("sel80", &self.sel80())
            .field("sel81", &self.sel81())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel40 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel40 {
            sel80: u8,
            sel81: u8,
        }
        let proxy = Sel40 {
            sel80: self.sel80(),
            sel81: self.sel81(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel41(pub u16);
impl Sel41 {
    #[doc = "SEL82"]
    #[must_use]
    #[inline(always)]
    pub const fn sel82(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL82"]
    #[inline(always)]
    pub const fn set_sel82(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL83"]
    #[must_use]
    #[inline(always)]
    pub const fn sel83(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL83"]
    #[inline(always)]
    pub const fn set_sel83(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel41 {
    #[inline(always)]
    fn default() -> Sel41 {
        Sel41(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel41 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel41")
            .field("sel82", &self.sel82())
            .field("sel83", &self.sel83())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel41 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel41 {
            sel82: u8,
            sel83: u8,
        }
        let proxy = Sel41 {
            sel82: self.sel82(),
            sel83: self.sel83(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel42(pub u16);
impl Sel42 {
    #[doc = "SEL84"]
    #[must_use]
    #[inline(always)]
    pub const fn sel84(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL84"]
    #[inline(always)]
    pub const fn set_sel84(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL85"]
    #[must_use]
    #[inline(always)]
    pub const fn sel85(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL85"]
    #[inline(always)]
    pub const fn set_sel85(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel42 {
    #[inline(always)]
    fn default() -> Sel42 {
        Sel42(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel42 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel42")
            .field("sel84", &self.sel84())
            .field("sel85", &self.sel85())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel42 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel42 {
            sel84: u8,
            sel85: u8,
        }
        let proxy = Sel42 {
            sel84: self.sel84(),
            sel85: self.sel85(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel43(pub u16);
impl Sel43 {
    #[doc = "SEL86"]
    #[must_use]
    #[inline(always)]
    pub const fn sel86(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL86"]
    #[inline(always)]
    pub const fn set_sel86(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL87"]
    #[must_use]
    #[inline(always)]
    pub const fn sel87(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL87"]
    #[inline(always)]
    pub const fn set_sel87(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel43 {
    #[inline(always)]
    fn default() -> Sel43 {
        Sel43(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel43 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel43")
            .field("sel86", &self.sel86())
            .field("sel87", &self.sel87())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel43 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel43 {
            sel86: u8,
            sel87: u8,
        }
        let proxy = Sel43 {
            sel86: self.sel86(),
            sel87: self.sel87(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel44(pub u16);
impl Sel44 {
    #[doc = "SEL88"]
    #[must_use]
    #[inline(always)]
    pub const fn sel88(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL88"]
    #[inline(always)]
    pub const fn set_sel88(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL89"]
    #[must_use]
    #[inline(always)]
    pub const fn sel89(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL89"]
    #[inline(always)]
    pub const fn set_sel89(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel44 {
    #[inline(always)]
    fn default() -> Sel44 {
        Sel44(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel44 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel44")
            .field("sel88", &self.sel88())
            .field("sel89", &self.sel89())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel44 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel44 {
            sel88: u8,
            sel89: u8,
        }
        let proxy = Sel44 {
            sel88: self.sel88(),
            sel89: self.sel89(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel45(pub u16);
impl Sel45 {
    #[doc = "SEL90"]
    #[must_use]
    #[inline(always)]
    pub const fn sel90(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL90"]
    #[inline(always)]
    pub const fn set_sel90(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL91"]
    #[must_use]
    #[inline(always)]
    pub const fn sel91(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL91"]
    #[inline(always)]
    pub const fn set_sel91(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel45 {
    #[inline(always)]
    fn default() -> Sel45 {
        Sel45(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel45 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel45")
            .field("sel90", &self.sel90())
            .field("sel91", &self.sel91())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel45 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel45 {
            sel90: u8,
            sel91: u8,
        }
        let proxy = Sel45 {
            sel90: self.sel90(),
            sel91: self.sel91(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel46(pub u16);
impl Sel46 {
    #[doc = "SEL92"]
    #[must_use]
    #[inline(always)]
    pub const fn sel92(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL92"]
    #[inline(always)]
    pub const fn set_sel92(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL93"]
    #[must_use]
    #[inline(always)]
    pub const fn sel93(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL93"]
    #[inline(always)]
    pub const fn set_sel93(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel46 {
    #[inline(always)]
    fn default() -> Sel46 {
        Sel46(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel46 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel46")
            .field("sel92", &self.sel92())
            .field("sel93", &self.sel93())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel46 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel46 {
            sel92: u8,
            sel93: u8,
        }
        let proxy = Sel46 {
            sel92: self.sel92(),
            sel93: self.sel93(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel47(pub u16);
impl Sel47 {
    #[doc = "SEL94"]
    #[must_use]
    #[inline(always)]
    pub const fn sel94(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL94"]
    #[inline(always)]
    pub const fn set_sel94(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL95"]
    #[must_use]
    #[inline(always)]
    pub const fn sel95(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL95"]
    #[inline(always)]
    pub const fn set_sel95(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel47 {
    #[inline(always)]
    fn default() -> Sel47 {
        Sel47(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel47 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel47")
            .field("sel94", &self.sel94())
            .field("sel95", &self.sel95())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel47 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel47 {
            sel94: u8,
            sel95: u8,
        }
        let proxy = Sel47 {
            sel94: self.sel94(),
            sel95: self.sel95(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel48(pub u16);
impl Sel48 {
    #[doc = "SEL96"]
    #[must_use]
    #[inline(always)]
    pub const fn sel96(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL96"]
    #[inline(always)]
    pub const fn set_sel96(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL97"]
    #[must_use]
    #[inline(always)]
    pub const fn sel97(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL97"]
    #[inline(always)]
    pub const fn set_sel97(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel48 {
    #[inline(always)]
    fn default() -> Sel48 {
        Sel48(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel48 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel48")
            .field("sel96", &self.sel96())
            .field("sel97", &self.sel97())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel48 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel48 {
            sel96: u8,
            sel97: u8,
        }
        let proxy = Sel48 {
            sel96: self.sel96(),
            sel97: self.sel97(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel49(pub u16);
impl Sel49 {
    #[doc = "SEL98"]
    #[must_use]
    #[inline(always)]
    pub const fn sel98(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL98"]
    #[inline(always)]
    pub const fn set_sel98(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL99"]
    #[must_use]
    #[inline(always)]
    pub const fn sel99(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL99"]
    #[inline(always)]
    pub const fn set_sel99(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel49 {
    #[inline(always)]
    fn default() -> Sel49 {
        Sel49(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel49 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel49")
            .field("sel98", &self.sel98())
            .field("sel99", &self.sel99())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel49 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel49 {
            sel98: u8,
            sel99: u8,
        }
        let proxy = Sel49 {
            sel98: self.sel98(),
            sel99: self.sel99(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel5(pub u16);
impl Sel5 {
    #[doc = "SEL10"]
    #[must_use]
    #[inline(always)]
    pub const fn sel10(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL10"]
    #[inline(always)]
    pub const fn set_sel10(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL11"]
    #[must_use]
    #[inline(always)]
    pub const fn sel11(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL11"]
    #[inline(always)]
    pub const fn set_sel11(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel5 {
    #[inline(always)]
    fn default() -> Sel5 {
        Sel5(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel5")
            .field("sel10", &self.sel10())
            .field("sel11", &self.sel11())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel5 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel5 {
            sel10: u8,
            sel11: u8,
        }
        let proxy = Sel5 {
            sel10: self.sel10(),
            sel11: self.sel11(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel50(pub u16);
impl Sel50 {
    #[doc = "SEL100"]
    #[must_use]
    #[inline(always)]
    pub const fn sel100(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL100"]
    #[inline(always)]
    pub const fn set_sel100(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL101"]
    #[must_use]
    #[inline(always)]
    pub const fn sel101(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL101"]
    #[inline(always)]
    pub const fn set_sel101(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel50 {
    #[inline(always)]
    fn default() -> Sel50 {
        Sel50(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel50 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel50")
            .field("sel100", &self.sel100())
            .field("sel101", &self.sel101())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel50 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel50 {
            sel100: u8,
            sel101: u8,
        }
        let proxy = Sel50 {
            sel100: self.sel100(),
            sel101: self.sel101(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel51(pub u16);
impl Sel51 {
    #[doc = "SEL102"]
    #[must_use]
    #[inline(always)]
    pub const fn sel102(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL102"]
    #[inline(always)]
    pub const fn set_sel102(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL103"]
    #[must_use]
    #[inline(always)]
    pub const fn sel103(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL103"]
    #[inline(always)]
    pub const fn set_sel103(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel51 {
    #[inline(always)]
    fn default() -> Sel51 {
        Sel51(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel51 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel51")
            .field("sel102", &self.sel102())
            .field("sel103", &self.sel103())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel51 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel51 {
            sel102: u8,
            sel103: u8,
        }
        let proxy = Sel51 {
            sel102: self.sel102(),
            sel103: self.sel103(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel52(pub u16);
impl Sel52 {
    #[doc = "SEL104"]
    #[must_use]
    #[inline(always)]
    pub const fn sel104(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL104"]
    #[inline(always)]
    pub const fn set_sel104(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL105"]
    #[must_use]
    #[inline(always)]
    pub const fn sel105(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL105"]
    #[inline(always)]
    pub const fn set_sel105(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel52 {
    #[inline(always)]
    fn default() -> Sel52 {
        Sel52(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel52 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel52")
            .field("sel104", &self.sel104())
            .field("sel105", &self.sel105())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel52 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel52 {
            sel104: u8,
            sel105: u8,
        }
        let proxy = Sel52 {
            sel104: self.sel104(),
            sel105: self.sel105(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel53(pub u16);
impl Sel53 {
    #[doc = "SEL106"]
    #[must_use]
    #[inline(always)]
    pub const fn sel106(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL106"]
    #[inline(always)]
    pub const fn set_sel106(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL107"]
    #[must_use]
    #[inline(always)]
    pub const fn sel107(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL107"]
    #[inline(always)]
    pub const fn set_sel107(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel53 {
    #[inline(always)]
    fn default() -> Sel53 {
        Sel53(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel53 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel53")
            .field("sel106", &self.sel106())
            .field("sel107", &self.sel107())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel53 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel53 {
            sel106: u8,
            sel107: u8,
        }
        let proxy = Sel53 {
            sel106: self.sel106(),
            sel107: self.sel107(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel54(pub u16);
impl Sel54 {
    #[doc = "SEL108"]
    #[must_use]
    #[inline(always)]
    pub const fn sel108(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL108"]
    #[inline(always)]
    pub const fn set_sel108(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL109"]
    #[must_use]
    #[inline(always)]
    pub const fn sel109(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL109"]
    #[inline(always)]
    pub const fn set_sel109(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel54 {
    #[inline(always)]
    fn default() -> Sel54 {
        Sel54(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel54 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel54")
            .field("sel108", &self.sel108())
            .field("sel109", &self.sel109())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel54 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel54 {
            sel108: u8,
            sel109: u8,
        }
        let proxy = Sel54 {
            sel108: self.sel108(),
            sel109: self.sel109(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel55(pub u16);
impl Sel55 {
    #[doc = "SEL110"]
    #[must_use]
    #[inline(always)]
    pub const fn sel110(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL110"]
    #[inline(always)]
    pub const fn set_sel110(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL111"]
    #[must_use]
    #[inline(always)]
    pub const fn sel111(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL111"]
    #[inline(always)]
    pub const fn set_sel111(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel55 {
    #[inline(always)]
    fn default() -> Sel55 {
        Sel55(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel55 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel55")
            .field("sel110", &self.sel110())
            .field("sel111", &self.sel111())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel55 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel55 {
            sel110: u8,
            sel111: u8,
        }
        let proxy = Sel55 {
            sel110: self.sel110(),
            sel111: self.sel111(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel56(pub u16);
impl Sel56 {
    #[doc = "SEL112"]
    #[must_use]
    #[inline(always)]
    pub const fn sel112(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL112"]
    #[inline(always)]
    pub const fn set_sel112(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL113"]
    #[must_use]
    #[inline(always)]
    pub const fn sel113(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL113"]
    #[inline(always)]
    pub const fn set_sel113(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel56 {
    #[inline(always)]
    fn default() -> Sel56 {
        Sel56(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel56 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel56")
            .field("sel112", &self.sel112())
            .field("sel113", &self.sel113())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel56 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel56 {
            sel112: u8,
            sel113: u8,
        }
        let proxy = Sel56 {
            sel112: self.sel112(),
            sel113: self.sel113(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel57(pub u16);
impl Sel57 {
    #[doc = "SEL114"]
    #[must_use]
    #[inline(always)]
    pub const fn sel114(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL114"]
    #[inline(always)]
    pub const fn set_sel114(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL115"]
    #[must_use]
    #[inline(always)]
    pub const fn sel115(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL115"]
    #[inline(always)]
    pub const fn set_sel115(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel57 {
    #[inline(always)]
    fn default() -> Sel57 {
        Sel57(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel57 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel57")
            .field("sel114", &self.sel114())
            .field("sel115", &self.sel115())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel57 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel57 {
            sel114: u8,
            sel115: u8,
        }
        let proxy = Sel57 {
            sel114: self.sel114(),
            sel115: self.sel115(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel58(pub u16);
impl Sel58 {
    #[doc = "SEL116"]
    #[must_use]
    #[inline(always)]
    pub const fn sel116(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL116"]
    #[inline(always)]
    pub const fn set_sel116(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL117"]
    #[must_use]
    #[inline(always)]
    pub const fn sel117(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL117"]
    #[inline(always)]
    pub const fn set_sel117(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel58 {
    #[inline(always)]
    fn default() -> Sel58 {
        Sel58(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel58 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel58")
            .field("sel116", &self.sel116())
            .field("sel117", &self.sel117())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel58 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel58 {
            sel116: u8,
            sel117: u8,
        }
        let proxy = Sel58 {
            sel116: self.sel116(),
            sel117: self.sel117(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel59(pub u16);
impl Sel59 {
    #[doc = "SEL118"]
    #[must_use]
    #[inline(always)]
    pub const fn sel118(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL118"]
    #[inline(always)]
    pub const fn set_sel118(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL119"]
    #[must_use]
    #[inline(always)]
    pub const fn sel119(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL119"]
    #[inline(always)]
    pub const fn set_sel119(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel59 {
    #[inline(always)]
    fn default() -> Sel59 {
        Sel59(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel59 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel59")
            .field("sel118", &self.sel118())
            .field("sel119", &self.sel119())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel59 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel59 {
            sel118: u8,
            sel119: u8,
        }
        let proxy = Sel59 {
            sel118: self.sel118(),
            sel119: self.sel119(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel6(pub u16);
impl Sel6 {
    #[doc = "SEL12"]
    #[must_use]
    #[inline(always)]
    pub const fn sel12(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL12"]
    #[inline(always)]
    pub const fn set_sel12(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL13"]
    #[must_use]
    #[inline(always)]
    pub const fn sel13(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL13"]
    #[inline(always)]
    pub const fn set_sel13(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel6 {
    #[inline(always)]
    fn default() -> Sel6 {
        Sel6(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel6")
            .field("sel12", &self.sel12())
            .field("sel13", &self.sel13())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel6 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel6 {
            sel12: u8,
            sel13: u8,
        }
        let proxy = Sel6 {
            sel12: self.sel12(),
            sel13: self.sel13(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel60(pub u16);
impl Sel60 {
    #[doc = "SEL120"]
    #[must_use]
    #[inline(always)]
    pub const fn sel120(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL120"]
    #[inline(always)]
    pub const fn set_sel120(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL121"]
    #[must_use]
    #[inline(always)]
    pub const fn sel121(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL121"]
    #[inline(always)]
    pub const fn set_sel121(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel60 {
    #[inline(always)]
    fn default() -> Sel60 {
        Sel60(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel60 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel60")
            .field("sel120", &self.sel120())
            .field("sel121", &self.sel121())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel60 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel60 {
            sel120: u8,
            sel121: u8,
        }
        let proxy = Sel60 {
            sel120: self.sel120(),
            sel121: self.sel121(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel61(pub u16);
impl Sel61 {
    #[doc = "SEL122"]
    #[must_use]
    #[inline(always)]
    pub const fn sel122(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL122"]
    #[inline(always)]
    pub const fn set_sel122(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL123"]
    #[must_use]
    #[inline(always)]
    pub const fn sel123(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL123"]
    #[inline(always)]
    pub const fn set_sel123(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel61 {
    #[inline(always)]
    fn default() -> Sel61 {
        Sel61(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel61 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel61")
            .field("sel122", &self.sel122())
            .field("sel123", &self.sel123())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel61 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel61 {
            sel122: u8,
            sel123: u8,
        }
        let proxy = Sel61 {
            sel122: self.sel122(),
            sel123: self.sel123(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel62(pub u16);
impl Sel62 {
    #[doc = "SEL124"]
    #[must_use]
    #[inline(always)]
    pub const fn sel124(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL124"]
    #[inline(always)]
    pub const fn set_sel124(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL125"]
    #[must_use]
    #[inline(always)]
    pub const fn sel125(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL125"]
    #[inline(always)]
    pub const fn set_sel125(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel62 {
    #[inline(always)]
    fn default() -> Sel62 {
        Sel62(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel62 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel62")
            .field("sel124", &self.sel124())
            .field("sel125", &self.sel125())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel62 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel62 {
            sel124: u8,
            sel125: u8,
        }
        let proxy = Sel62 {
            sel124: self.sel124(),
            sel125: self.sel125(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel63(pub u16);
impl Sel63 {
    #[doc = "SEL126"]
    #[must_use]
    #[inline(always)]
    pub const fn sel126(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL126"]
    #[inline(always)]
    pub const fn set_sel126(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL127"]
    #[must_use]
    #[inline(always)]
    pub const fn sel127(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL127"]
    #[inline(always)]
    pub const fn set_sel127(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel63 {
    #[inline(always)]
    fn default() -> Sel63 {
        Sel63(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel63 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel63")
            .field("sel126", &self.sel126())
            .field("sel127", &self.sel127())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel63 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel63 {
            sel126: u8,
            sel127: u8,
        }
        let proxy = Sel63 {
            sel126: self.sel126(),
            sel127: self.sel127(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel64(pub u16);
impl Sel64 {
    #[doc = "SEL128"]
    #[must_use]
    #[inline(always)]
    pub const fn sel128(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL128"]
    #[inline(always)]
    pub const fn set_sel128(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL129"]
    #[must_use]
    #[inline(always)]
    pub const fn sel129(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL129"]
    #[inline(always)]
    pub const fn set_sel129(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel64 {
    #[inline(always)]
    fn default() -> Sel64 {
        Sel64(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel64 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel64")
            .field("sel128", &self.sel128())
            .field("sel129", &self.sel129())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel64 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel64 {
            sel128: u8,
            sel129: u8,
        }
        let proxy = Sel64 {
            sel128: self.sel128(),
            sel129: self.sel129(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel65(pub u16);
impl Sel65 {
    #[doc = "SEL130"]
    #[must_use]
    #[inline(always)]
    pub const fn sel130(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL130"]
    #[inline(always)]
    pub const fn set_sel130(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL131"]
    #[must_use]
    #[inline(always)]
    pub const fn sel131(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL131"]
    #[inline(always)]
    pub const fn set_sel131(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel65 {
    #[inline(always)]
    fn default() -> Sel65 {
        Sel65(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel65 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel65")
            .field("sel130", &self.sel130())
            .field("sel131", &self.sel131())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel65 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel65 {
            sel130: u8,
            sel131: u8,
        }
        let proxy = Sel65 {
            sel130: self.sel130(),
            sel131: self.sel131(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel66(pub u16);
impl Sel66 {
    #[doc = "SEL132"]
    #[must_use]
    #[inline(always)]
    pub const fn sel132(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL132"]
    #[inline(always)]
    pub const fn set_sel132(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL133"]
    #[must_use]
    #[inline(always)]
    pub const fn sel133(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL133"]
    #[inline(always)]
    pub const fn set_sel133(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel66 {
    #[inline(always)]
    fn default() -> Sel66 {
        Sel66(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel66 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel66")
            .field("sel132", &self.sel132())
            .field("sel133", &self.sel133())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel66 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel66 {
            sel132: u8,
            sel133: u8,
        }
        let proxy = Sel66 {
            sel132: self.sel132(),
            sel133: self.sel133(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel67(pub u16);
impl Sel67 {
    #[doc = "SEL134"]
    #[must_use]
    #[inline(always)]
    pub const fn sel134(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL134"]
    #[inline(always)]
    pub const fn set_sel134(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL135"]
    #[must_use]
    #[inline(always)]
    pub const fn sel135(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL135"]
    #[inline(always)]
    pub const fn set_sel135(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel67 {
    #[inline(always)]
    fn default() -> Sel67 {
        Sel67(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel67 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel67")
            .field("sel134", &self.sel134())
            .field("sel135", &self.sel135())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel67 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel67 {
            sel134: u8,
            sel135: u8,
        }
        let proxy = Sel67 {
            sel134: self.sel134(),
            sel135: self.sel135(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel68(pub u16);
impl Sel68 {
    #[doc = "SEL136"]
    #[must_use]
    #[inline(always)]
    pub const fn sel136(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL136"]
    #[inline(always)]
    pub const fn set_sel136(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL137"]
    #[must_use]
    #[inline(always)]
    pub const fn sel137(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL137"]
    #[inline(always)]
    pub const fn set_sel137(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel68 {
    #[inline(always)]
    fn default() -> Sel68 {
        Sel68(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel68 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel68")
            .field("sel136", &self.sel136())
            .field("sel137", &self.sel137())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel68 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel68 {
            sel136: u8,
            sel137: u8,
        }
        let proxy = Sel68 {
            sel136: self.sel136(),
            sel137: self.sel137(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel69(pub u16);
impl Sel69 {
    #[doc = "SEL138"]
    #[must_use]
    #[inline(always)]
    pub const fn sel138(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL138"]
    #[inline(always)]
    pub const fn set_sel138(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL139"]
    #[must_use]
    #[inline(always)]
    pub const fn sel139(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL139"]
    #[inline(always)]
    pub const fn set_sel139(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel69 {
    #[inline(always)]
    fn default() -> Sel69 {
        Sel69(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel69 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel69")
            .field("sel138", &self.sel138())
            .field("sel139", &self.sel139())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel69 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel69 {
            sel138: u8,
            sel139: u8,
        }
        let proxy = Sel69 {
            sel138: self.sel138(),
            sel139: self.sel139(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel7(pub u16);
impl Sel7 {
    #[doc = "SEL14"]
    #[must_use]
    #[inline(always)]
    pub const fn sel14(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL14"]
    #[inline(always)]
    pub const fn set_sel14(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL15"]
    #[must_use]
    #[inline(always)]
    pub const fn sel15(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL15"]
    #[inline(always)]
    pub const fn set_sel15(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel7 {
    #[inline(always)]
    fn default() -> Sel7 {
        Sel7(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel7")
            .field("sel14", &self.sel14())
            .field("sel15", &self.sel15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel7 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel7 {
            sel14: u8,
            sel15: u8,
        }
        let proxy = Sel7 {
            sel14: self.sel14(),
            sel15: self.sel15(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel70(pub u16);
impl Sel70 {
    #[doc = "SEL140"]
    #[must_use]
    #[inline(always)]
    pub const fn sel140(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL140"]
    #[inline(always)]
    pub const fn set_sel140(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL141"]
    #[must_use]
    #[inline(always)]
    pub const fn sel141(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL141"]
    #[inline(always)]
    pub const fn set_sel141(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel70 {
    #[inline(always)]
    fn default() -> Sel70 {
        Sel70(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel70 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel70")
            .field("sel140", &self.sel140())
            .field("sel141", &self.sel141())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel70 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel70 {
            sel140: u8,
            sel141: u8,
        }
        let proxy = Sel70 {
            sel140: self.sel140(),
            sel141: self.sel141(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel71(pub u16);
impl Sel71 {
    #[doc = "SEL142"]
    #[must_use]
    #[inline(always)]
    pub const fn sel142(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL142"]
    #[inline(always)]
    pub const fn set_sel142(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL143"]
    #[must_use]
    #[inline(always)]
    pub const fn sel143(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL143"]
    #[inline(always)]
    pub const fn set_sel143(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel71 {
    #[inline(always)]
    fn default() -> Sel71 {
        Sel71(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel71 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel71")
            .field("sel142", &self.sel142())
            .field("sel143", &self.sel143())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel71 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel71 {
            sel142: u8,
            sel143: u8,
        }
        let proxy = Sel71 {
            sel142: self.sel142(),
            sel143: self.sel143(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel72(pub u16);
impl Sel72 {
    #[doc = "SEL144"]
    #[must_use]
    #[inline(always)]
    pub const fn sel144(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL144"]
    #[inline(always)]
    pub const fn set_sel144(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL145"]
    #[must_use]
    #[inline(always)]
    pub const fn sel145(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL145"]
    #[inline(always)]
    pub const fn set_sel145(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel72 {
    #[inline(always)]
    fn default() -> Sel72 {
        Sel72(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel72 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel72")
            .field("sel144", &self.sel144())
            .field("sel145", &self.sel145())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel72 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel72 {
            sel144: u8,
            sel145: u8,
        }
        let proxy = Sel72 {
            sel144: self.sel144(),
            sel145: self.sel145(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel73(pub u16);
impl Sel73 {
    #[doc = "SEL146"]
    #[must_use]
    #[inline(always)]
    pub const fn sel146(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL146"]
    #[inline(always)]
    pub const fn set_sel146(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL147"]
    #[must_use]
    #[inline(always)]
    pub const fn sel147(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL147"]
    #[inline(always)]
    pub const fn set_sel147(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel73 {
    #[inline(always)]
    fn default() -> Sel73 {
        Sel73(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel73 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel73")
            .field("sel146", &self.sel146())
            .field("sel147", &self.sel147())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel73 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel73 {
            sel146: u8,
            sel147: u8,
        }
        let proxy = Sel73 {
            sel146: self.sel146(),
            sel147: self.sel147(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel74(pub u16);
impl Sel74 {
    #[doc = "SEL148"]
    #[must_use]
    #[inline(always)]
    pub const fn sel148(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL148"]
    #[inline(always)]
    pub const fn set_sel148(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL149"]
    #[must_use]
    #[inline(always)]
    pub const fn sel149(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL149"]
    #[inline(always)]
    pub const fn set_sel149(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel74 {
    #[inline(always)]
    fn default() -> Sel74 {
        Sel74(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel74 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel74")
            .field("sel148", &self.sel148())
            .field("sel149", &self.sel149())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel74 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel74 {
            sel148: u8,
            sel149: u8,
        }
        let proxy = Sel74 {
            sel148: self.sel148(),
            sel149: self.sel149(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel75(pub u16);
impl Sel75 {
    #[doc = "SEL150"]
    #[must_use]
    #[inline(always)]
    pub const fn sel150(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL150"]
    #[inline(always)]
    pub const fn set_sel150(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL151"]
    #[must_use]
    #[inline(always)]
    pub const fn sel151(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL151"]
    #[inline(always)]
    pub const fn set_sel151(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel75 {
    #[inline(always)]
    fn default() -> Sel75 {
        Sel75(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel75 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel75")
            .field("sel150", &self.sel150())
            .field("sel151", &self.sel151())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel75 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel75 {
            sel150: u8,
            sel151: u8,
        }
        let proxy = Sel75 {
            sel150: self.sel150(),
            sel151: self.sel151(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel76(pub u16);
impl Sel76 {
    #[doc = "SEL152"]
    #[must_use]
    #[inline(always)]
    pub const fn sel152(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL152"]
    #[inline(always)]
    pub const fn set_sel152(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL153"]
    #[must_use]
    #[inline(always)]
    pub const fn sel153(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL153"]
    #[inline(always)]
    pub const fn set_sel153(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel76 {
    #[inline(always)]
    fn default() -> Sel76 {
        Sel76(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel76 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel76")
            .field("sel152", &self.sel152())
            .field("sel153", &self.sel153())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel76 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel76 {
            sel152: u8,
            sel153: u8,
        }
        let proxy = Sel76 {
            sel152: self.sel152(),
            sel153: self.sel153(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel77(pub u16);
impl Sel77 {
    #[doc = "SEL154"]
    #[must_use]
    #[inline(always)]
    pub const fn sel154(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL154"]
    #[inline(always)]
    pub const fn set_sel154(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL155"]
    #[must_use]
    #[inline(always)]
    pub const fn sel155(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL155"]
    #[inline(always)]
    pub const fn set_sel155(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel77 {
    #[inline(always)]
    fn default() -> Sel77 {
        Sel77(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel77 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel77")
            .field("sel154", &self.sel154())
            .field("sel155", &self.sel155())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel77 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel77 {
            sel154: u8,
            sel155: u8,
        }
        let proxy = Sel77 {
            sel154: self.sel154(),
            sel155: self.sel155(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel78(pub u16);
impl Sel78 {
    #[doc = "SEL156"]
    #[must_use]
    #[inline(always)]
    pub const fn sel156(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL156"]
    #[inline(always)]
    pub const fn set_sel156(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL157"]
    #[must_use]
    #[inline(always)]
    pub const fn sel157(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL157"]
    #[inline(always)]
    pub const fn set_sel157(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel78 {
    #[inline(always)]
    fn default() -> Sel78 {
        Sel78(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel78 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel78")
            .field("sel156", &self.sel156())
            .field("sel157", &self.sel157())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel78 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel78 {
            sel156: u8,
            sel157: u8,
        }
        let proxy = Sel78 {
            sel156: self.sel156(),
            sel157: self.sel157(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel79(pub u16);
impl Sel79 {
    #[doc = "SEL158"]
    #[must_use]
    #[inline(always)]
    pub const fn sel158(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL158"]
    #[inline(always)]
    pub const fn set_sel158(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL159"]
    #[must_use]
    #[inline(always)]
    pub const fn sel159(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL159"]
    #[inline(always)]
    pub const fn set_sel159(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel79 {
    #[inline(always)]
    fn default() -> Sel79 {
        Sel79(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel79 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel79")
            .field("sel158", &self.sel158())
            .field("sel159", &self.sel159())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel79 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel79 {
            sel158: u8,
            sel159: u8,
        }
        let proxy = Sel79 {
            sel158: self.sel158(),
            sel159: self.sel159(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel8(pub u16);
impl Sel8 {
    #[doc = "SEL16"]
    #[must_use]
    #[inline(always)]
    pub const fn sel16(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL16"]
    #[inline(always)]
    pub const fn set_sel16(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL17"]
    #[must_use]
    #[inline(always)]
    pub const fn sel17(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL17"]
    #[inline(always)]
    pub const fn set_sel17(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel8 {
    #[inline(always)]
    fn default() -> Sel8 {
        Sel8(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel8")
            .field("sel16", &self.sel16())
            .field("sel17", &self.sel17())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel8 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel8 {
            sel16: u8,
            sel17: u8,
        }
        let proxy = Sel8 {
            sel16: self.sel16(),
            sel17: self.sel17(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel80(pub u16);
impl Sel80 {
    #[doc = "SEL160"]
    #[must_use]
    #[inline(always)]
    pub const fn sel160(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL160"]
    #[inline(always)]
    pub const fn set_sel160(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL161"]
    #[must_use]
    #[inline(always)]
    pub const fn sel161(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL161"]
    #[inline(always)]
    pub const fn set_sel161(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel80 {
    #[inline(always)]
    fn default() -> Sel80 {
        Sel80(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel80 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel80")
            .field("sel160", &self.sel160())
            .field("sel161", &self.sel161())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel80 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel80 {
            sel160: u8,
            sel161: u8,
        }
        let proxy = Sel80 {
            sel160: self.sel160(),
            sel161: self.sel161(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel81(pub u16);
impl Sel81 {
    #[doc = "SEL162"]
    #[must_use]
    #[inline(always)]
    pub const fn sel162(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL162"]
    #[inline(always)]
    pub const fn set_sel162(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL163"]
    #[must_use]
    #[inline(always)]
    pub const fn sel163(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL163"]
    #[inline(always)]
    pub const fn set_sel163(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel81 {
    #[inline(always)]
    fn default() -> Sel81 {
        Sel81(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel81 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel81")
            .field("sel162", &self.sel162())
            .field("sel163", &self.sel163())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel81 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel81 {
            sel162: u8,
            sel163: u8,
        }
        let proxy = Sel81 {
            sel162: self.sel162(),
            sel163: self.sel163(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel82(pub u16);
impl Sel82 {
    #[doc = "SEL164"]
    #[must_use]
    #[inline(always)]
    pub const fn sel164(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL164"]
    #[inline(always)]
    pub const fn set_sel164(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL165"]
    #[must_use]
    #[inline(always)]
    pub const fn sel165(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL165"]
    #[inline(always)]
    pub const fn set_sel165(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel82 {
    #[inline(always)]
    fn default() -> Sel82 {
        Sel82(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel82 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel82")
            .field("sel164", &self.sel164())
            .field("sel165", &self.sel165())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel82 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel82 {
            sel164: u8,
            sel165: u8,
        }
        let proxy = Sel82 {
            sel164: self.sel164(),
            sel165: self.sel165(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel83(pub u16);
impl Sel83 {
    #[doc = "SEL166"]
    #[must_use]
    #[inline(always)]
    pub const fn sel166(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL166"]
    #[inline(always)]
    pub const fn set_sel166(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL167"]
    #[must_use]
    #[inline(always)]
    pub const fn sel167(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL167"]
    #[inline(always)]
    pub const fn set_sel167(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel83 {
    #[inline(always)]
    fn default() -> Sel83 {
        Sel83(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel83 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel83")
            .field("sel166", &self.sel166())
            .field("sel167", &self.sel167())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel83 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel83 {
            sel166: u8,
            sel167: u8,
        }
        let proxy = Sel83 {
            sel166: self.sel166(),
            sel167: self.sel167(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel84(pub u16);
impl Sel84 {
    #[doc = "SEL168"]
    #[must_use]
    #[inline(always)]
    pub const fn sel168(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL168"]
    #[inline(always)]
    pub const fn set_sel168(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL169"]
    #[must_use]
    #[inline(always)]
    pub const fn sel169(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL169"]
    #[inline(always)]
    pub const fn set_sel169(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel84 {
    #[inline(always)]
    fn default() -> Sel84 {
        Sel84(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel84 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel84")
            .field("sel168", &self.sel168())
            .field("sel169", &self.sel169())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel84 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel84 {
            sel168: u8,
            sel169: u8,
        }
        let proxy = Sel84 {
            sel168: self.sel168(),
            sel169: self.sel169(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel85(pub u16);
impl Sel85 {
    #[doc = "SEL170"]
    #[must_use]
    #[inline(always)]
    pub const fn sel170(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL170"]
    #[inline(always)]
    pub const fn set_sel170(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL171"]
    #[must_use]
    #[inline(always)]
    pub const fn sel171(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL171"]
    #[inline(always)]
    pub const fn set_sel171(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel85 {
    #[inline(always)]
    fn default() -> Sel85 {
        Sel85(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel85 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel85")
            .field("sel170", &self.sel170())
            .field("sel171", &self.sel171())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel85 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel85 {
            sel170: u8,
            sel171: u8,
        }
        let proxy = Sel85 {
            sel170: self.sel170(),
            sel171: self.sel171(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel86(pub u16);
impl Sel86 {
    #[doc = "SEL172"]
    #[must_use]
    #[inline(always)]
    pub const fn sel172(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL172"]
    #[inline(always)]
    pub const fn set_sel172(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL173"]
    #[must_use]
    #[inline(always)]
    pub const fn sel173(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL173"]
    #[inline(always)]
    pub const fn set_sel173(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel86 {
    #[inline(always)]
    fn default() -> Sel86 {
        Sel86(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel86 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel86")
            .field("sel172", &self.sel172())
            .field("sel173", &self.sel173())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel86 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel86 {
            sel172: u8,
            sel173: u8,
        }
        let proxy = Sel86 {
            sel172: self.sel172(),
            sel173: self.sel173(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel87(pub u16);
impl Sel87 {
    #[doc = "SEL174"]
    #[must_use]
    #[inline(always)]
    pub const fn sel174(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL174"]
    #[inline(always)]
    pub const fn set_sel174(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL175"]
    #[must_use]
    #[inline(always)]
    pub const fn sel175(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL175"]
    #[inline(always)]
    pub const fn set_sel175(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel87 {
    #[inline(always)]
    fn default() -> Sel87 {
        Sel87(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel87 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel87")
            .field("sel174", &self.sel174())
            .field("sel175", &self.sel175())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel87 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel87 {
            sel174: u8,
            sel175: u8,
        }
        let proxy = Sel87 {
            sel174: self.sel174(),
            sel175: self.sel175(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel88(pub u16);
impl Sel88 {
    #[doc = "SEL176"]
    #[must_use]
    #[inline(always)]
    pub const fn sel176(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL176"]
    #[inline(always)]
    pub const fn set_sel176(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL177"]
    #[must_use]
    #[inline(always)]
    pub const fn sel177(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL177"]
    #[inline(always)]
    pub const fn set_sel177(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel88 {
    #[inline(always)]
    fn default() -> Sel88 {
        Sel88(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel88 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel88")
            .field("sel176", &self.sel176())
            .field("sel177", &self.sel177())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel88 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel88 {
            sel176: u8,
            sel177: u8,
        }
        let proxy = Sel88 {
            sel176: self.sel176(),
            sel177: self.sel177(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel89(pub u16);
impl Sel89 {
    #[doc = "SEL178"]
    #[must_use]
    #[inline(always)]
    pub const fn sel178(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL178"]
    #[inline(always)]
    pub const fn set_sel178(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL179"]
    #[must_use]
    #[inline(always)]
    pub const fn sel179(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL179"]
    #[inline(always)]
    pub const fn set_sel179(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel89 {
    #[inline(always)]
    fn default() -> Sel89 {
        Sel89(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel89 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel89")
            .field("sel178", &self.sel178())
            .field("sel179", &self.sel179())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel89 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel89 {
            sel178: u8,
            sel179: u8,
        }
        let proxy = Sel89 {
            sel178: self.sel178(),
            sel179: self.sel179(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel9(pub u16);
impl Sel9 {
    #[doc = "SEL18"]
    #[must_use]
    #[inline(always)]
    pub const fn sel18(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL18"]
    #[inline(always)]
    pub const fn set_sel18(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL19"]
    #[must_use]
    #[inline(always)]
    pub const fn sel19(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL19"]
    #[inline(always)]
    pub const fn set_sel19(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel9 {
    #[inline(always)]
    fn default() -> Sel9 {
        Sel9(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel9")
            .field("sel18", &self.sel18())
            .field("sel19", &self.sel19())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel9 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel9 {
            sel18: u8,
            sel19: u8,
        }
        let proxy = Sel9 {
            sel18: self.sel18(),
            sel19: self.sel19(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel90(pub u16);
impl Sel90 {
    #[doc = "SEL180"]
    #[must_use]
    #[inline(always)]
    pub const fn sel180(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL180"]
    #[inline(always)]
    pub const fn set_sel180(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL181"]
    #[must_use]
    #[inline(always)]
    pub const fn sel181(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL181"]
    #[inline(always)]
    pub const fn set_sel181(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel90 {
    #[inline(always)]
    fn default() -> Sel90 {
        Sel90(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel90 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel90")
            .field("sel180", &self.sel180())
            .field("sel181", &self.sel181())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel90 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel90 {
            sel180: u8,
            sel181: u8,
        }
        let proxy = Sel90 {
            sel180: self.sel180(),
            sel181: self.sel181(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel91(pub u16);
impl Sel91 {
    #[doc = "SEL182"]
    #[must_use]
    #[inline(always)]
    pub const fn sel182(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL182"]
    #[inline(always)]
    pub const fn set_sel182(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL183"]
    #[must_use]
    #[inline(always)]
    pub const fn sel183(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL183"]
    #[inline(always)]
    pub const fn set_sel183(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel91 {
    #[inline(always)]
    fn default() -> Sel91 {
        Sel91(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel91 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel91")
            .field("sel182", &self.sel182())
            .field("sel183", &self.sel183())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel91 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel91 {
            sel182: u8,
            sel183: u8,
        }
        let proxy = Sel91 {
            sel182: self.sel182(),
            sel183: self.sel183(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel92(pub u16);
impl Sel92 {
    #[doc = "SEL184"]
    #[must_use]
    #[inline(always)]
    pub const fn sel184(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL184"]
    #[inline(always)]
    pub const fn set_sel184(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL185"]
    #[must_use]
    #[inline(always)]
    pub const fn sel185(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL185"]
    #[inline(always)]
    pub const fn set_sel185(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel92 {
    #[inline(always)]
    fn default() -> Sel92 {
        Sel92(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel92 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel92")
            .field("sel184", &self.sel184())
            .field("sel185", &self.sel185())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel92 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel92 {
            sel184: u8,
            sel185: u8,
        }
        let proxy = Sel92 {
            sel184: self.sel184(),
            sel185: self.sel185(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel93(pub u16);
impl Sel93 {
    #[doc = "SEL186"]
    #[must_use]
    #[inline(always)]
    pub const fn sel186(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL186"]
    #[inline(always)]
    pub const fn set_sel186(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL187"]
    #[must_use]
    #[inline(always)]
    pub const fn sel187(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL187"]
    #[inline(always)]
    pub const fn set_sel187(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel93 {
    #[inline(always)]
    fn default() -> Sel93 {
        Sel93(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel93 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel93")
            .field("sel186", &self.sel186())
            .field("sel187", &self.sel187())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel93 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel93 {
            sel186: u8,
            sel187: u8,
        }
        let proxy = Sel93 {
            sel186: self.sel186(),
            sel187: self.sel187(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel94(pub u16);
impl Sel94 {
    #[doc = "SEL188"]
    #[must_use]
    #[inline(always)]
    pub const fn sel188(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL188"]
    #[inline(always)]
    pub const fn set_sel188(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL189"]
    #[must_use]
    #[inline(always)]
    pub const fn sel189(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL189"]
    #[inline(always)]
    pub const fn set_sel189(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel94 {
    #[inline(always)]
    fn default() -> Sel94 {
        Sel94(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel94 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel94")
            .field("sel188", &self.sel188())
            .field("sel189", &self.sel189())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel94 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel94 {
            sel188: u8,
            sel189: u8,
        }
        let proxy = Sel94 {
            sel188: self.sel188(),
            sel189: self.sel189(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel95(pub u16);
impl Sel95 {
    #[doc = "SEL190"]
    #[must_use]
    #[inline(always)]
    pub const fn sel190(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL190"]
    #[inline(always)]
    pub const fn set_sel190(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL191"]
    #[must_use]
    #[inline(always)]
    pub const fn sel191(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL191"]
    #[inline(always)]
    pub const fn set_sel191(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel95 {
    #[inline(always)]
    fn default() -> Sel95 {
        Sel95(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel95 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel95")
            .field("sel190", &self.sel190())
            .field("sel191", &self.sel191())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel95 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel95 {
            sel190: u8,
            sel191: u8,
        }
        let proxy = Sel95 {
            sel190: self.sel190(),
            sel191: self.sel191(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel96(pub u16);
impl Sel96 {
    #[doc = "SEL192"]
    #[must_use]
    #[inline(always)]
    pub const fn sel192(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL192"]
    #[inline(always)]
    pub const fn set_sel192(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL193"]
    #[must_use]
    #[inline(always)]
    pub const fn sel193(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL193"]
    #[inline(always)]
    pub const fn set_sel193(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel96 {
    #[inline(always)]
    fn default() -> Sel96 {
        Sel96(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel96 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel96")
            .field("sel192", &self.sel192())
            .field("sel193", &self.sel193())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel96 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel96 {
            sel192: u8,
            sel193: u8,
        }
        let proxy = Sel96 {
            sel192: self.sel192(),
            sel193: self.sel193(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel97(pub u16);
impl Sel97 {
    #[doc = "SEL194"]
    #[must_use]
    #[inline(always)]
    pub const fn sel194(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL194"]
    #[inline(always)]
    pub const fn set_sel194(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL195"]
    #[must_use]
    #[inline(always)]
    pub const fn sel195(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL195"]
    #[inline(always)]
    pub const fn set_sel195(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel97 {
    #[inline(always)]
    fn default() -> Sel97 {
        Sel97(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel97 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel97")
            .field("sel194", &self.sel194())
            .field("sel195", &self.sel195())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel97 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel97 {
            sel194: u8,
            sel195: u8,
        }
        let proxy = Sel97 {
            sel194: self.sel194(),
            sel195: self.sel195(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel98(pub u16);
impl Sel98 {
    #[doc = "SEL196"]
    #[must_use]
    #[inline(always)]
    pub const fn sel196(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL196"]
    #[inline(always)]
    pub const fn set_sel196(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL197"]
    #[must_use]
    #[inline(always)]
    pub const fn sel197(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL197"]
    #[inline(always)]
    pub const fn set_sel197(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel98 {
    #[inline(always)]
    fn default() -> Sel98 {
        Sel98(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel98 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel98")
            .field("sel196", &self.sel196())
            .field("sel197", &self.sel197())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel98 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel98 {
            sel196: u8,
            sel197: u8,
        }
        let proxy = Sel98 {
            sel196: self.sel196(),
            sel197: self.sel197(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crossbar Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sel99(pub u16);
impl Sel99 {
    #[doc = "SEL198"]
    #[must_use]
    #[inline(always)]
    pub const fn sel198(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SEL198"]
    #[inline(always)]
    pub const fn set_sel198(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "SEL199"]
    #[must_use]
    #[inline(always)]
    pub const fn sel199(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SEL199"]
    #[inline(always)]
    pub const fn set_sel199(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sel99 {
    #[inline(always)]
    fn default() -> Sel99 {
        Sel99(0u64 as u16)
    }
}
impl core::fmt::Debug for Sel99 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sel99")
            .field("sel198", &self.sel198())
            .field("sel199", &self.sel199())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sel99 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sel99 {
            sel198: u8,
            sel199: u8,
        }
        let proxy = Sel99 {
            sel198: self.sel198(),
            sel199: self.sel199(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
