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
