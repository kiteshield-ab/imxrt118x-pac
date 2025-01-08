#[doc = "24MHz OSC Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Osc24mCtrl(pub u32);
impl Osc24mCtrl {
    #[doc = "24MHz OSC Bypass Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bypass_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "24MHz OSC Bypass Enable"]
    #[inline(always)]
    pub const fn set_bypass_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "24MHz OSC Low-Power Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lp_en(&self) -> super::vals::LpEn {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::LpEn::from_bits(val as u8)
    }
    #[doc = "24MHz OSC Low-Power Mode Enable"]
    #[inline(always)]
    pub const fn set_lp_en(&mut self, val: super::vals::LpEn) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "24MHz OSC Comparator Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn osc_comp_mode(&self) -> super::vals::OscCompMode {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::OscCompMode::from_bits(val as u8)
    }
    #[doc = "24MHz OSC Comparator Mode"]
    #[inline(always)]
    pub const fn set_osc_comp_mode(&mut self, val: super::vals::OscCompMode) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "24MHz OSC Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn osc_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "24MHz OSC Enable"]
    #[inline(always)]
    pub const fn set_osc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "24MHz OSC Gate Control"]
    #[must_use]
    #[inline(always)]
    pub const fn osc_24m_gate(&self) -> super::vals::Osc24mGate {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Osc24mGate::from_bits(val as u8)
    }
    #[doc = "24MHz OSC Gate Control"]
    #[inline(always)]
    pub const fn set_osc_24m_gate(&mut self, val: super::vals::Osc24mGate) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "24MHz OSC Stable"]
    #[must_use]
    #[inline(always)]
    pub const fn osc_24m_stable(&self) -> super::vals::Osc24mStable {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Osc24mStable::from_bits(val as u8)
    }
    #[doc = "24MHz OSC Stable"]
    #[inline(always)]
    pub const fn set_osc_24m_stable(&mut self, val: super::vals::Osc24mStable) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "24MHz OSC Control Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn osc_24m_control_mode(&self) -> super::vals::Osc24mControlMode {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Osc24mControlMode::from_bits(val as u8)
    }
    #[doc = "24MHz OSC Control Mode"]
    #[inline(always)]
    pub const fn set_osc_24m_control_mode(&mut self, val: super::vals::Osc24mControlMode) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Osc24mCtrl {
    #[inline(always)]
    fn default() -> Osc24mCtrl {
        Osc24mCtrl(128u64 as u32)
    }
}
impl core::fmt::Debug for Osc24mCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Osc24mCtrl")
            .field("bypass_en", &self.bypass_en())
            .field("lp_en", &self.lp_en())
            .field("osc_comp_mode", &self.osc_comp_mode())
            .field("osc_en", &self.osc_en())
            .field("osc_24m_gate", &self.osc_24m_gate())
            .field("osc_24m_stable", &self.osc_24m_stable())
            .field("osc_24m_control_mode", &self.osc_24m_control_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Osc24mCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Osc24mCtrl {
            bypass_en: bool,
            lp_en: super::vals::LpEn,
            osc_comp_mode: super::vals::OscCompMode,
            osc_en: bool,
            osc_24m_gate: super::vals::Osc24mGate,
            osc_24m_stable: super::vals::Osc24mStable,
            osc_24m_control_mode: super::vals::Osc24mControlMode,
        }
        let proxy = Osc24mCtrl {
            bypass_en: self.bypass_en(),
            lp_en: self.lp_en(),
            osc_comp_mode: self.osc_comp_mode(),
            osc_en: self.osc_en(),
            osc_24m_gate: self.osc_24m_gate(),
            osc_24m_stable: self.osc_24m_stable(),
            osc_24m_control_mode: self.osc_24m_control_mode(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "400MHz RCOSC Control0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Osc400mCtrl0(pub u32);
impl Osc400mCtrl0 {
    #[doc = "400MHz OSC AI BUSY"]
    #[must_use]
    #[inline(always)]
    pub const fn osc400m_ai_busy(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "400MHz OSC AI BUSY"]
    #[inline(always)]
    pub const fn set_osc400m_ai_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Osc400mCtrl0 {
    #[inline(always)]
    fn default() -> Osc400mCtrl0 {
        Osc400mCtrl0(2147483648u64 as u32)
    }
}
impl core::fmt::Debug for Osc400mCtrl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Osc400mCtrl0")
            .field("osc400m_ai_busy", &self.osc400m_ai_busy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Osc400mCtrl0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Osc400mCtrl0 {
            osc400m_ai_busy: bool,
        }
        let proxy = Osc400mCtrl0 {
            osc400m_ai_busy: self.osc400m_ai_busy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "400MHz RCOSC Control1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Osc400mCtrl1(pub u32);
impl Osc400mCtrl1 {
    #[doc = "Power down control for 400MHz RCOSC"]
    #[must_use]
    #[inline(always)]
    pub const fn pwd(&self) -> super::vals::Pwd {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pwd::from_bits(val as u8)
    }
    #[doc = "Power down control for 400MHz RCOSC"]
    #[inline(always)]
    pub const fn set_pwd(&mut self, val: super::vals::Pwd) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Clock gate control for 400MHz RCOSC"]
    #[must_use]
    #[inline(always)]
    pub const fn clkgate_400meg(&self) -> super::vals::Clkgate400meg {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Clkgate400meg::from_bits(val as u8)
    }
    #[doc = "Clock gate control for 400MHz RCOSC"]
    #[inline(always)]
    pub const fn set_clkgate_400meg(&mut self, val: super::vals::Clkgate400meg) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "400MHz RCOSC Control mode"]
    #[must_use]
    #[inline(always)]
    pub const fn rc_400m_control_mode(&self) -> super::vals::Rc400mControlMode {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Rc400mControlMode::from_bits(val as u8)
    }
    #[doc = "400MHz RCOSC Control mode"]
    #[inline(always)]
    pub const fn set_rc_400m_control_mode(&mut self, val: super::vals::Rc400mControlMode) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Osc400mCtrl1 {
    #[inline(always)]
    fn default() -> Osc400mCtrl1 {
        Osc400mCtrl1(1u64 as u32)
    }
}
impl core::fmt::Debug for Osc400mCtrl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Osc400mCtrl1")
            .field("pwd", &self.pwd())
            .field("clkgate_400meg", &self.clkgate_400meg())
            .field("rc_400m_control_mode", &self.rc_400m_control_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Osc400mCtrl1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Osc400mCtrl1 {
            pwd: super::vals::Pwd,
            clkgate_400meg: super::vals::Clkgate400meg,
            rc_400m_control_mode: super::vals::Rc400mControlMode,
        }
        let proxy = Osc400mCtrl1 {
            pwd: self.pwd(),
            clkgate_400meg: self.clkgate_400meg(),
            rc_400m_control_mode: self.rc_400m_control_mode(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "24MHz RCOSC Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OscRc24mCtrl(pub u32);
impl OscRc24mCtrl {
    #[doc = "RC24M Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ten(&self) -> super::vals::Ten {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ten::from_bits(val as u8)
    }
    #[doc = "RC24M Enable"]
    #[inline(always)]
    pub const fn set_ten(&mut self, val: super::vals::Ten) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "source_sel_24M"]
    #[must_use]
    #[inline(always)]
    pub const fn source_sel_24m(&self) -> super::vals::SourceSel24m {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::SourceSel24m::from_bits(val as u8)
    }
    #[doc = "source_sel_24M"]
    #[inline(always)]
    pub const fn set_source_sel_24m(&mut self, val: super::vals::SourceSel24m) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "RCOSC Control Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn rc_24m_control_mode(&self) -> super::vals::Rc24mControlMode {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Rc24mControlMode::from_bits(val as u8)
    }
    #[doc = "RCOSC Control Mode"]
    #[inline(always)]
    pub const fn set_rc_24m_control_mode(&mut self, val: super::vals::Rc24mControlMode) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for OscRc24mCtrl {
    #[inline(always)]
    fn default() -> OscRc24mCtrl {
        OscRc24mCtrl(7930354u64 as u32)
    }
}
impl core::fmt::Debug for OscRc24mCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OscRc24mCtrl")
            .field("ten", &self.ten())
            .field("source_sel_24m", &self.source_sel_24m())
            .field("rc_24m_control_mode", &self.rc_24m_control_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OscRc24mCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct OscRc24mCtrl {
            ten: super::vals::Ten,
            source_sel_24m: super::vals::SourceSel24m,
            rc_24m_control_mode: super::vals::Rc24mControlMode,
        }
        let proxy = OscRc24mCtrl {
            ten: self.ten(),
            source_sel_24m: self.source_sel_24m(),
            rc_24m_control_mode: self.rc_24m_control_mode(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
