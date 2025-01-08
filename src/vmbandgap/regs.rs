#[doc = "Analog Control Register CTRL0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl0(pub u32);
impl Ctrl0 {
    #[doc = "Master power-down for bandgap module"]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_pwd(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Master power-down for bandgap module"]
    #[inline(always)]
    pub const fn set_reftop_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Power-down for bandgap voltage-reference buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_linregref_pwd(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Power-down for bandgap voltage-reference buffer"]
    #[inline(always)]
    pub const fn set_reftop_linregref_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Power-down VBGUP detector in bandgap"]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_pwdvbgup(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Power-down VBGUP detector in bandgap"]
    #[inline(always)]
    pub const fn set_reftop_pwdvbgup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Low-power control bit"]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_lowpower(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Low-power control bit"]
    #[inline(always)]
    pub const fn set_reftop_lowpower(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "bandgap self-bias control bit"]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_selfbiasoff(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "bandgap self-bias control bit"]
    #[inline(always)]
    pub const fn set_reftop_selfbiasoff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Ctrl0 {
    #[inline(always)]
    fn default() -> Ctrl0 {
        Ctrl0(0u64 as u32)
    }
}
impl core::fmt::Debug for Ctrl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl0")
            .field("reftop_pwd", &self.reftop_pwd())
            .field("reftop_linregref_pwd", &self.reftop_linregref_pwd())
            .field("reftop_pwdvbgup", &self.reftop_pwdvbgup())
            .field("reftop_lowpower", &self.reftop_lowpower())
            .field("reftop_selfbiasoff", &self.reftop_selfbiasoff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ctrl0 {
            reftop_pwd: bool,
            reftop_linregref_pwd: bool,
            reftop_pwdvbgup: bool,
            reftop_lowpower: bool,
            reftop_selfbiasoff: bool,
        }
        let proxy = Ctrl0 {
            reftop_pwd: self.reftop_pwd(),
            reftop_linregref_pwd: self.reftop_linregref_pwd(),
            reftop_pwdvbgup: self.reftop_pwdvbgup(),
            reftop_lowpower: self.reftop_lowpower(),
            reftop_selfbiasoff: self.reftop_selfbiasoff(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Analog Control Register CTRL0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl0Clr(pub u32);
impl Ctrl0Clr {
    #[doc = "Master power-down for bandgap module"]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_pwd(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Master power-down for bandgap module"]
    #[inline(always)]
    pub const fn set_reftop_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Power-down for bandgap voltage-reference buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_linregref_pwd(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Power-down for bandgap voltage-reference buffer"]
    #[inline(always)]
    pub const fn set_reftop_linregref_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Power-down VBGUP detector in bandgap"]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_pwdvbgup(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Power-down VBGUP detector in bandgap"]
    #[inline(always)]
    pub const fn set_reftop_pwdvbgup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Low-power control bit"]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_lowpower(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Low-power control bit"]
    #[inline(always)]
    pub const fn set_reftop_lowpower(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "bandgap self-bias control bit"]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_selfbiasoff(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "bandgap self-bias control bit"]
    #[inline(always)]
    pub const fn set_reftop_selfbiasoff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Ctrl0Clr {
    #[inline(always)]
    fn default() -> Ctrl0Clr {
        Ctrl0Clr(0u64 as u32)
    }
}
impl core::fmt::Debug for Ctrl0Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl0Clr")
            .field("reftop_pwd", &self.reftop_pwd())
            .field("reftop_linregref_pwd", &self.reftop_linregref_pwd())
            .field("reftop_pwdvbgup", &self.reftop_pwdvbgup())
            .field("reftop_lowpower", &self.reftop_lowpower())
            .field("reftop_selfbiasoff", &self.reftop_selfbiasoff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl0Clr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ctrl0Clr {
            reftop_pwd: bool,
            reftop_linregref_pwd: bool,
            reftop_pwdvbgup: bool,
            reftop_lowpower: bool,
            reftop_selfbiasoff: bool,
        }
        let proxy = Ctrl0Clr {
            reftop_pwd: self.reftop_pwd(),
            reftop_linregref_pwd: self.reftop_linregref_pwd(),
            reftop_pwdvbgup: self.reftop_pwdvbgup(),
            reftop_lowpower: self.reftop_lowpower(),
            reftop_selfbiasoff: self.reftop_selfbiasoff(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Analog Control Register CTRL0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl0Set(pub u32);
impl Ctrl0Set {
    #[doc = "Master power-down for bandgap module"]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_pwd(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Master power-down for bandgap module"]
    #[inline(always)]
    pub const fn set_reftop_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Power-down for bandgap voltage-reference buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_linregref_pwd(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Power-down for bandgap voltage-reference buffer"]
    #[inline(always)]
    pub const fn set_reftop_linregref_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Power-down VBGUP detector in bandgap"]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_pwdvbgup(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Power-down VBGUP detector in bandgap"]
    #[inline(always)]
    pub const fn set_reftop_pwdvbgup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Low-power control bit"]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_lowpower(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Low-power control bit"]
    #[inline(always)]
    pub const fn set_reftop_lowpower(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "bandgap self-bias control bit"]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_selfbiasoff(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "bandgap self-bias control bit"]
    #[inline(always)]
    pub const fn set_reftop_selfbiasoff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Ctrl0Set {
    #[inline(always)]
    fn default() -> Ctrl0Set {
        Ctrl0Set(0u64 as u32)
    }
}
impl core::fmt::Debug for Ctrl0Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl0Set")
            .field("reftop_pwd", &self.reftop_pwd())
            .field("reftop_linregref_pwd", &self.reftop_linregref_pwd())
            .field("reftop_pwdvbgup", &self.reftop_pwdvbgup())
            .field("reftop_lowpower", &self.reftop_lowpower())
            .field("reftop_selfbiasoff", &self.reftop_selfbiasoff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl0Set {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ctrl0Set {
            reftop_pwd: bool,
            reftop_linregref_pwd: bool,
            reftop_pwdvbgup: bool,
            reftop_lowpower: bool,
            reftop_selfbiasoff: bool,
        }
        let proxy = Ctrl0Set {
            reftop_pwd: self.reftop_pwd(),
            reftop_linregref_pwd: self.reftop_linregref_pwd(),
            reftop_pwdvbgup: self.reftop_pwdvbgup(),
            reftop_lowpower: self.reftop_lowpower(),
            reftop_selfbiasoff: self.reftop_selfbiasoff(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Analog Control Register CTRL0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl0Tog(pub u32);
impl Ctrl0Tog {
    #[doc = "Master power-down for bandgap module"]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_pwd(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Master power-down for bandgap module"]
    #[inline(always)]
    pub const fn set_reftop_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Power-down for bandgap voltage-reference buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_linregref_pwd(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Power-down for bandgap voltage-reference buffer"]
    #[inline(always)]
    pub const fn set_reftop_linregref_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Power-down VBGUP detector in bandgap"]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_pwdvbgup(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Power-down VBGUP detector in bandgap"]
    #[inline(always)]
    pub const fn set_reftop_pwdvbgup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Low-power control bit"]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_lowpower(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Low-power control bit"]
    #[inline(always)]
    pub const fn set_reftop_lowpower(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "bandgap self-bias control bit"]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_selfbiasoff(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "bandgap self-bias control bit"]
    #[inline(always)]
    pub const fn set_reftop_selfbiasoff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Ctrl0Tog {
    #[inline(always)]
    fn default() -> Ctrl0Tog {
        Ctrl0Tog(0u64 as u32)
    }
}
impl core::fmt::Debug for Ctrl0Tog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl0Tog")
            .field("reftop_pwd", &self.reftop_pwd())
            .field("reftop_linregref_pwd", &self.reftop_linregref_pwd())
            .field("reftop_pwdvbgup", &self.reftop_pwdvbgup())
            .field("reftop_lowpower", &self.reftop_lowpower())
            .field("reftop_selfbiasoff", &self.reftop_selfbiasoff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl0Tog {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ctrl0Tog {
            reftop_pwd: bool,
            reftop_linregref_pwd: bool,
            reftop_pwdvbgup: bool,
            reftop_lowpower: bool,
            reftop_selfbiasoff: bool,
        }
        let proxy = Ctrl0Tog {
            reftop_pwd: self.reftop_pwd(),
            reftop_linregref_pwd: self.reftop_linregref_pwd(),
            reftop_pwdvbgup: self.reftop_pwdvbgup(),
            reftop_lowpower: self.reftop_lowpower(),
            reftop_selfbiasoff: self.reftop_selfbiasoff(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Analog Status Register STAT0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat0(pub u32);
impl Stat0 {
    #[doc = "Brief description here"]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_vbgup(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Brief description here"]
    #[inline(always)]
    pub const fn set_reftop_vbgup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Brief description here"]
    #[must_use]
    #[inline(always)]
    pub const fn vdd1_porb(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Brief description here"]
    #[inline(always)]
    pub const fn set_vdd1_porb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Brief description here"]
    #[must_use]
    #[inline(always)]
    pub const fn vdd2_porb(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Brief description here"]
    #[inline(always)]
    pub const fn set_vdd2_porb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Brief description here"]
    #[must_use]
    #[inline(always)]
    pub const fn vdd3_porb(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Brief description here"]
    #[inline(always)]
    pub const fn set_vdd3_porb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Stat0 {
    #[inline(always)]
    fn default() -> Stat0 {
        Stat0(0u64 as u32)
    }
}
impl core::fmt::Debug for Stat0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stat0")
            .field("reftop_vbgup", &self.reftop_vbgup())
            .field("vdd1_porb", &self.vdd1_porb())
            .field("vdd2_porb", &self.vdd2_porb())
            .field("vdd3_porb", &self.vdd3_porb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stat0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Stat0 {
            reftop_vbgup: bool,
            vdd1_porb: bool,
            vdd2_porb: bool,
            vdd3_porb: bool,
        }
        let proxy = Stat0 {
            reftop_vbgup: self.reftop_vbgup(),
            vdd1_porb: self.vdd1_porb(),
            vdd2_porb: self.vdd2_porb(),
            vdd3_porb: self.vdd3_porb(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Analog Status Register STAT0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat0Clr(pub u32);
impl Stat0Clr {
    #[doc = "Brief description here"]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_vbgup(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Brief description here"]
    #[inline(always)]
    pub const fn set_reftop_vbgup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Brief description here"]
    #[must_use]
    #[inline(always)]
    pub const fn vdd1_porb(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Brief description here"]
    #[inline(always)]
    pub const fn set_vdd1_porb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Brief description here"]
    #[must_use]
    #[inline(always)]
    pub const fn vdd2_porb(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Brief description here"]
    #[inline(always)]
    pub const fn set_vdd2_porb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Brief description here"]
    #[must_use]
    #[inline(always)]
    pub const fn vdd3_porb(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Brief description here"]
    #[inline(always)]
    pub const fn set_vdd3_porb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Stat0Clr {
    #[inline(always)]
    fn default() -> Stat0Clr {
        Stat0Clr(0u64 as u32)
    }
}
impl core::fmt::Debug for Stat0Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stat0Clr")
            .field("reftop_vbgup", &self.reftop_vbgup())
            .field("vdd1_porb", &self.vdd1_porb())
            .field("vdd2_porb", &self.vdd2_porb())
            .field("vdd3_porb", &self.vdd3_porb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stat0Clr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Stat0Clr {
            reftop_vbgup: bool,
            vdd1_porb: bool,
            vdd2_porb: bool,
            vdd3_porb: bool,
        }
        let proxy = Stat0Clr {
            reftop_vbgup: self.reftop_vbgup(),
            vdd1_porb: self.vdd1_porb(),
            vdd2_porb: self.vdd2_porb(),
            vdd3_porb: self.vdd3_porb(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Analog Status Register STAT0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat0Set(pub u32);
impl Stat0Set {
    #[doc = "Brief description here"]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_vbgup(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Brief description here"]
    #[inline(always)]
    pub const fn set_reftop_vbgup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Brief description here"]
    #[must_use]
    #[inline(always)]
    pub const fn vdd1_porb(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Brief description here"]
    #[inline(always)]
    pub const fn set_vdd1_porb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Brief description here"]
    #[must_use]
    #[inline(always)]
    pub const fn vdd2_porb(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Brief description here"]
    #[inline(always)]
    pub const fn set_vdd2_porb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Brief description here"]
    #[must_use]
    #[inline(always)]
    pub const fn vdd3_porb(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Brief description here"]
    #[inline(always)]
    pub const fn set_vdd3_porb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Stat0Set {
    #[inline(always)]
    fn default() -> Stat0Set {
        Stat0Set(0u64 as u32)
    }
}
impl core::fmt::Debug for Stat0Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stat0Set")
            .field("reftop_vbgup", &self.reftop_vbgup())
            .field("vdd1_porb", &self.vdd1_porb())
            .field("vdd2_porb", &self.vdd2_porb())
            .field("vdd3_porb", &self.vdd3_porb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stat0Set {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Stat0Set {
            reftop_vbgup: bool,
            vdd1_porb: bool,
            vdd2_porb: bool,
            vdd3_porb: bool,
        }
        let proxy = Stat0Set {
            reftop_vbgup: self.reftop_vbgup(),
            vdd1_porb: self.vdd1_porb(),
            vdd2_porb: self.vdd2_porb(),
            vdd3_porb: self.vdd3_porb(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Analog Status Register STAT0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat0Tog(pub u32);
impl Stat0Tog {
    #[doc = "Brief description here"]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_vbgup(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Brief description here"]
    #[inline(always)]
    pub const fn set_reftop_vbgup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Brief description here"]
    #[must_use]
    #[inline(always)]
    pub const fn vdd1_porb(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Brief description here"]
    #[inline(always)]
    pub const fn set_vdd1_porb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Brief description here"]
    #[must_use]
    #[inline(always)]
    pub const fn vdd2_porb(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Brief description here"]
    #[inline(always)]
    pub const fn set_vdd2_porb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Brief description here"]
    #[must_use]
    #[inline(always)]
    pub const fn vdd3_porb(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Brief description here"]
    #[inline(always)]
    pub const fn set_vdd3_porb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Stat0Tog {
    #[inline(always)]
    fn default() -> Stat0Tog {
        Stat0Tog(0u64 as u32)
    }
}
impl core::fmt::Debug for Stat0Tog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stat0Tog")
            .field("reftop_vbgup", &self.reftop_vbgup())
            .field("vdd1_porb", &self.vdd1_porb())
            .field("vdd2_porb", &self.vdd2_porb())
            .field("vdd3_porb", &self.vdd3_porb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stat0Tog {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Stat0Tog {
            reftop_vbgup: bool,
            vdd1_porb: bool,
            vdd2_porb: bool,
            vdd3_porb: bool,
        }
        let proxy = Stat0Tog {
            reftop_vbgup: self.reftop_vbgup(),
            vdd1_porb: self.vdd1_porb(),
            vdd2_porb: self.vdd2_porb(),
            vdd3_porb: self.vdd3_porb(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
