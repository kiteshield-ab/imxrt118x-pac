#[doc = "PWM Source Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtsrcsel(pub u16);
impl Dtsrcsel {
    #[doc = "Submodule 0 PWM45 Control Select"]
    #[must_use]
    #[inline(always)]
    pub const fn sm0sel45(&self) -> super::vals::Sm0sel45 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sm0sel45::from_bits(val as u8)
    }
    #[doc = "Submodule 0 PWM45 Control Select"]
    #[inline(always)]
    pub const fn set_sm0sel45(&mut self, val: super::vals::Sm0sel45) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Submodule 0 PWM23 Control Select"]
    #[must_use]
    #[inline(always)]
    pub const fn sm0sel23(&self) -> super::vals::Sm0sel23 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Sm0sel23::from_bits(val as u8)
    }
    #[doc = "Submodule 0 PWM23 Control Select"]
    #[inline(always)]
    pub const fn set_sm0sel23(&mut self, val: super::vals::Sm0sel23) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Submodule 1 PWM45 Control Select"]
    #[must_use]
    #[inline(always)]
    pub const fn sm1sel45(&self) -> super::vals::Sm1sel45 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Sm1sel45::from_bits(val as u8)
    }
    #[doc = "Submodule 1 PWM45 Control Select"]
    #[inline(always)]
    pub const fn set_sm1sel45(&mut self, val: super::vals::Sm1sel45) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Submodule 1 PWM23 Control Select"]
    #[must_use]
    #[inline(always)]
    pub const fn sm1sel23(&self) -> super::vals::Sm1sel23 {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Sm1sel23::from_bits(val as u8)
    }
    #[doc = "Submodule 1 PWM23 Control Select"]
    #[inline(always)]
    pub const fn set_sm1sel23(&mut self, val: super::vals::Sm1sel23) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "Submodule 2 PWM45 Control Select"]
    #[must_use]
    #[inline(always)]
    pub const fn sm2sel45(&self) -> super::vals::Sm2sel45 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Sm2sel45::from_bits(val as u8)
    }
    #[doc = "Submodule 2 PWM45 Control Select"]
    #[inline(always)]
    pub const fn set_sm2sel45(&mut self, val: super::vals::Sm2sel45) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "Submodule 2 PWM23 Control Select"]
    #[must_use]
    #[inline(always)]
    pub const fn sm2sel23(&self) -> super::vals::Sm2sel23 {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Sm2sel23::from_bits(val as u8)
    }
    #[doc = "Submodule 2 PWM23 Control Select"]
    #[inline(always)]
    pub const fn set_sm2sel23(&mut self, val: super::vals::Sm2sel23) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u16) & 0x03) << 10usize);
    }
    #[doc = "Submodule 3 PWM45 Control Select"]
    #[must_use]
    #[inline(always)]
    pub const fn sm3sel45(&self) -> super::vals::Sm3sel45 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Sm3sel45::from_bits(val as u8)
    }
    #[doc = "Submodule 3 PWM45 Control Select"]
    #[inline(always)]
    pub const fn set_sm3sel45(&mut self, val: super::vals::Sm3sel45) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
    #[doc = "Submodule 3 PWM23 Control Select"]
    #[must_use]
    #[inline(always)]
    pub const fn sm3sel23(&self) -> super::vals::Sm3sel23 {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Sm3sel23::from_bits(val as u8)
    }
    #[doc = "Submodule 3 PWM23 Control Select"]
    #[inline(always)]
    pub const fn set_sm3sel23(&mut self, val: super::vals::Sm3sel23) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Dtsrcsel {
    #[inline(always)]
    fn default() -> Dtsrcsel {
        Dtsrcsel(0u64 as u16)
    }
}
impl core::fmt::Debug for Dtsrcsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dtsrcsel")
            .field("sm0sel45", &self.sm0sel45())
            .field("sm0sel23", &self.sm0sel23())
            .field("sm1sel45", &self.sm1sel45())
            .field("sm1sel23", &self.sm1sel23())
            .field("sm2sel45", &self.sm2sel45())
            .field("sm2sel23", &self.sm2sel23())
            .field("sm3sel45", &self.sm3sel45())
            .field("sm3sel23", &self.sm3sel23())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dtsrcsel {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Dtsrcsel {
            sm0sel45: super::vals::Sm0sel45,
            sm0sel23: super::vals::Sm0sel23,
            sm1sel45: super::vals::Sm1sel45,
            sm1sel23: super::vals::Sm1sel23,
            sm2sel45: super::vals::Sm2sel45,
            sm2sel23: super::vals::Sm2sel23,
            sm3sel45: super::vals::Sm3sel45,
            sm3sel23: super::vals::Sm3sel23,
        }
        let proxy = Dtsrcsel {
            sm0sel45: self.sm0sel45(),
            sm0sel23: self.sm0sel23(),
            sm1sel45: self.sm1sel45(),
            sm1sel23: self.sm1sel23(),
            sm2sel45: self.sm2sel45(),
            sm2sel23: self.sm2sel23(),
            sm3sel45: self.sm3sel45(),
            sm3sel23: self.sm3sel23(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fault Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fctrl0(pub u16);
impl Fctrl0 {
    #[doc = "Fault Interrupt Enables"]
    #[must_use]
    #[inline(always)]
    pub const fn fie(&self) -> super::vals::Fie {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Fie::from_bits(val as u8)
    }
    #[doc = "Fault Interrupt Enables"]
    #[inline(always)]
    pub const fn set_fie(&mut self, val: super::vals::Fie) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u16) & 0x0f) << 0usize);
    }
    #[doc = "Fault Safety Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn fsafe(&self) -> super::vals::Fsafe {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::Fsafe::from_bits(val as u8)
    }
    #[doc = "Fault Safety Mode"]
    #[inline(always)]
    pub const fn set_fsafe(&mut self, val: super::vals::Fsafe) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u16) & 0x0f) << 4usize);
    }
    #[doc = "Automatic Fault Clearing"]
    #[must_use]
    #[inline(always)]
    pub const fn fauto(&self) -> super::vals::Fauto {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Fauto::from_bits(val as u8)
    }
    #[doc = "Automatic Fault Clearing"]
    #[inline(always)]
    pub const fn set_fauto(&mut self, val: super::vals::Fauto) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u16) & 0x0f) << 8usize);
    }
    #[doc = "Fault Level"]
    #[must_use]
    #[inline(always)]
    pub const fn flvl(&self) -> super::vals::Flvl {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Flvl::from_bits(val as u8)
    }
    #[doc = "Fault Level"]
    #[inline(always)]
    pub const fn set_flvl(&mut self, val: super::vals::Flvl) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u16) & 0x0f) << 12usize);
    }
}
impl Default for Fctrl0 {
    #[inline(always)]
    fn default() -> Fctrl0 {
        Fctrl0(0u64 as u16)
    }
}
impl core::fmt::Debug for Fctrl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fctrl0")
            .field("fie", &self.fie())
            .field("fsafe", &self.fsafe())
            .field("fauto", &self.fauto())
            .field("flvl", &self.flvl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fctrl0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Fctrl0 {
            fie: super::vals::Fie,
            fsafe: super::vals::Fsafe,
            fauto: super::vals::Fauto,
            flvl: super::vals::Flvl,
        }
        let proxy = Fctrl0 {
            fie: self.fie(),
            fsafe: self.fsafe(),
            fauto: self.fauto(),
            flvl: self.flvl(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fault Control 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fctrl20(pub u16);
impl Fctrl20 {
    #[doc = "No Combinational Path From Fault Input To PWM Output"]
    #[must_use]
    #[inline(always)]
    pub const fn nocomb(&self) -> super::vals::Nocomb {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Nocomb::from_bits(val as u8)
    }
    #[doc = "No Combinational Path From Fault Input To PWM Output"]
    #[inline(always)]
    pub const fn set_nocomb(&mut self, val: super::vals::Nocomb) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u16) & 0x0f) << 0usize);
    }
}
impl Default for Fctrl20 {
    #[inline(always)]
    fn default() -> Fctrl20 {
        Fctrl20(0u64 as u16)
    }
}
impl core::fmt::Debug for Fctrl20 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fctrl20")
            .field("nocomb", &self.nocomb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fctrl20 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Fctrl20 {
            nocomb: super::vals::Nocomb,
        }
        let proxy = Fctrl20 {
            nocomb: self.nocomb(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fault Filter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ffilt0(pub u16);
impl Ffilt0 {
    #[doc = "Fault Filter Period"]
    #[must_use]
    #[inline(always)]
    pub const fn filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Fault Filter Period"]
    #[inline(always)]
    pub const fn set_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Fault Filter Count"]
    #[must_use]
    #[inline(always)]
    pub const fn filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Fault Filter Count"]
    #[inline(always)]
    pub const fn set_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
    #[doc = "Fault Glitch Stretch Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gstr(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Fault Glitch Stretch Enable"]
    #[inline(always)]
    pub const fn set_gstr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Ffilt0 {
    #[inline(always)]
    fn default() -> Ffilt0 {
        Ffilt0(0u64 as u16)
    }
}
impl core::fmt::Debug for Ffilt0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ffilt0")
            .field("filt_per", &self.filt_per())
            .field("filt_cnt", &self.filt_cnt())
            .field("gstr", &self.gstr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ffilt0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ffilt0 {
            filt_per: u8,
            filt_cnt: u8,
            gstr: bool,
        }
        let proxy = Ffilt0 {
            filt_per: self.filt_per(),
            filt_cnt: self.filt_cnt(),
            gstr: self.gstr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fault Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fsts0(pub u16);
impl Fsts0 {
    #[doc = "Fault Flags"]
    #[must_use]
    #[inline(always)]
    pub const fn fflag(&self) -> super::vals::Fflag {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Fflag::from_bits(val as u8)
    }
    #[doc = "Fault Flags"]
    #[inline(always)]
    pub const fn set_fflag(&mut self, val: super::vals::Fflag) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u16) & 0x0f) << 0usize);
    }
    #[doc = "Full Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn ffull(&self) -> super::vals::Ffull {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::Ffull::from_bits(val as u8)
    }
    #[doc = "Full Cycle"]
    #[inline(always)]
    pub const fn set_ffull(&mut self, val: super::vals::Ffull) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u16) & 0x0f) << 4usize);
    }
    #[doc = "Filtered Fault Pins"]
    #[must_use]
    #[inline(always)]
    pub const fn ffpin(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Filtered Fault Pins"]
    #[inline(always)]
    pub const fn set_ffpin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
    #[doc = "Half Cycle Fault Recovery"]
    #[must_use]
    #[inline(always)]
    pub const fn fhalf(&self) -> super::vals::Fhalf {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Fhalf::from_bits(val as u8)
    }
    #[doc = "Half Cycle Fault Recovery"]
    #[inline(always)]
    pub const fn set_fhalf(&mut self, val: super::vals::Fhalf) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u16) & 0x0f) << 12usize);
    }
}
impl Default for Fsts0 {
    #[inline(always)]
    fn default() -> Fsts0 {
        Fsts0(0u64 as u16)
    }
}
impl core::fmt::Debug for Fsts0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fsts0")
            .field("fflag", &self.fflag())
            .field("ffull", &self.ffull())
            .field("ffpin", &self.ffpin())
            .field("fhalf", &self.fhalf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fsts0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Fsts0 {
            fflag: super::vals::Fflag,
            ffull: super::vals::Ffull,
            ffpin: u8,
            fhalf: super::vals::Fhalf,
        }
        let proxy = Fsts0 {
            fflag: self.fflag(),
            ffull: self.ffull(),
            ffpin: self.ffpin(),
            fhalf: self.fhalf(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fault Test Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ftst0(pub u16);
impl Ftst0 {
    #[doc = "Fault Test"]
    #[must_use]
    #[inline(always)]
    pub const fn ftest(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Fault Test"]
    #[inline(always)]
    pub const fn set_ftest(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
}
impl Default for Ftst0 {
    #[inline(always)]
    fn default() -> Ftst0 {
        Ftst0(0u64 as u16)
    }
}
impl core::fmt::Debug for Ftst0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ftst0")
            .field("ftest", &self.ftest())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ftst0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ftst0 {
            ftest: bool,
        }
        let proxy = Ftst0 {
            ftest: self.ftest(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Mask Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mask(pub u16);
impl Mask {
    #[doc = "PWM_X Masks"]
    #[must_use]
    #[inline(always)]
    pub const fn maskx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_X Masks"]
    #[inline(always)]
    pub const fn set_maskx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "PWM_B Masks"]
    #[must_use]
    #[inline(always)]
    pub const fn maskb(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_B Masks"]
    #[inline(always)]
    pub const fn set_maskb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
    }
    #[doc = "PWM_A Masks"]
    #[must_use]
    #[inline(always)]
    pub const fn maska(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_A Masks"]
    #[inline(always)]
    pub const fn set_maska(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
}
impl Default for Mask {
    #[inline(always)]
    fn default() -> Mask {
        Mask(0u64 as u16)
    }
}
impl core::fmt::Debug for Mask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mask")
            .field("maskx", &self.maskx())
            .field("maskb", &self.maskb())
            .field("maska", &self.maska())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mask {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Mask {
            maskx: u8,
            maskb: u8,
            maska: u8,
        }
        let proxy = Mask {
            maskx: self.maskx(),
            maskb: self.maskb(),
            maska: self.maska(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Master Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mctrl(pub u16);
impl Mctrl {
    #[doc = "Load Okay"]
    #[must_use]
    #[inline(always)]
    pub const fn ldok(&self) -> super::vals::Ldok {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Ldok::from_bits(val as u8)
    }
    #[doc = "Load Okay"]
    #[inline(always)]
    pub const fn set_ldok(&mut self, val: super::vals::Ldok) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u16) & 0x0f) << 0usize);
    }
    #[doc = "Clear Load Okay"]
    #[must_use]
    #[inline(always)]
    pub const fn cldok(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Clear Load Okay"]
    #[inline(always)]
    pub const fn set_cldok(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
    }
    #[doc = "Run"]
    #[must_use]
    #[inline(always)]
    pub const fn run(&self) -> super::vals::Run {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Run::from_bits(val as u8)
    }
    #[doc = "Run"]
    #[inline(always)]
    pub const fn set_run(&mut self, val: super::vals::Run) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u16) & 0x0f) << 8usize);
    }
    #[doc = "Current Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn ipol(&self) -> super::vals::Ipol {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Ipol::from_bits(val as u8)
    }
    #[doc = "Current Polarity"]
    #[inline(always)]
    pub const fn set_ipol(&mut self, val: super::vals::Ipol) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u16) & 0x0f) << 12usize);
    }
}
impl Default for Mctrl {
    #[inline(always)]
    fn default() -> Mctrl {
        Mctrl(0u64 as u16)
    }
}
impl core::fmt::Debug for Mctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mctrl")
            .field("ldok", &self.ldok())
            .field("cldok", &self.cldok())
            .field("run", &self.run())
            .field("ipol", &self.ipol())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mctrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Mctrl {
            ldok: super::vals::Ldok,
            cldok: u8,
            run: super::vals::Run,
            ipol: super::vals::Ipol,
        }
        let proxy = Mctrl {
            ldok: self.ldok(),
            cldok: self.cldok(),
            run: self.run(),
            ipol: self.ipol(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Master Control 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mctrl2(pub u16);
impl Mctrl2 {
    #[doc = "Write protect"]
    #[must_use]
    #[inline(always)]
    pub const fn wrprot(&self) -> super::vals::Wrprot {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Wrprot::from_bits(val as u8)
    }
    #[doc = "Write protect"]
    #[inline(always)]
    pub const fn set_wrprot(&mut self, val: super::vals::Wrprot) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
}
impl Default for Mctrl2 {
    #[inline(always)]
    fn default() -> Mctrl2 {
        Mctrl2(0u64 as u16)
    }
}
impl core::fmt::Debug for Mctrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mctrl2")
            .field("wrprot", &self.wrprot())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mctrl2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Mctrl2 {
            wrprot: super::vals::Wrprot,
        }
        let proxy = Mctrl2 {
            wrprot: self.wrprot(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Output Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Outen(pub u16);
impl Outen {
    #[doc = "PWM_X Output Enables"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmx_en(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_X Output Enables"]
    #[inline(always)]
    pub const fn set_pwmx_en(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "PWM_B Output Enables"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmb_en(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_B Output Enables"]
    #[inline(always)]
    pub const fn set_pwmb_en(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
    }
    #[doc = "PWM_A Output Enables"]
    #[must_use]
    #[inline(always)]
    pub const fn pwma_en(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_A Output Enables"]
    #[inline(always)]
    pub const fn set_pwma_en(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
}
impl Default for Outen {
    #[inline(always)]
    fn default() -> Outen {
        Outen(0u64 as u16)
    }
}
impl core::fmt::Debug for Outen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Outen")
            .field("pwmx_en", &self.pwmx_en())
            .field("pwmb_en", &self.pwmb_en())
            .field("pwma_en", &self.pwma_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Outen {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Outen {
            pwmx_en: u8,
            pwmb_en: u8,
            pwma_en: u8,
        }
        let proxy = Outen {
            pwmx_en: self.pwmx_en(),
            pwmb_en: self.pwmb_en(),
            pwma_en: self.pwma_en(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Compare A Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0captcompa(pub u16);
impl Sm0captcompa {
    #[doc = "Edge Compare A"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcmpa(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Compare A"]
    #[inline(always)]
    pub const fn set_edgcmpa(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Edge Counter A"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcnta(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Counter A"]
    #[inline(always)]
    pub const fn set_edgcnta(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sm0captcompa {
    #[inline(always)]
    fn default() -> Sm0captcompa {
        Sm0captcompa(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm0captcompa {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0captcompa")
            .field("edgcmpa", &self.edgcmpa())
            .field("edgcnta", &self.edgcnta())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0captcompa {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm0captcompa {
            edgcmpa: u8,
            edgcnta: u8,
        }
        let proxy = Sm0captcompa {
            edgcmpa: self.edgcmpa(),
            edgcnta: self.edgcnta(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Compare B Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0captcompb(pub u16);
impl Sm0captcompb {
    #[doc = "Edge Compare B"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcmpb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Compare B"]
    #[inline(always)]
    pub const fn set_edgcmpb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Edge Counter B"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntb(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Counter B"]
    #[inline(always)]
    pub const fn set_edgcntb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sm0captcompb {
    #[inline(always)]
    fn default() -> Sm0captcompb {
        Sm0captcompb(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm0captcompb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0captcompb")
            .field("edgcmpb", &self.edgcmpb())
            .field("edgcntb", &self.edgcntb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0captcompb {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm0captcompb {
            edgcmpb: u8,
            edgcntb: u8,
        }
        let proxy = Sm0captcompb {
            edgcmpb: self.edgcmpb(),
            edgcntb: self.edgcntb(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Compare X Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0captcompx(pub u16);
impl Sm0captcompx {
    #[doc = "Edge Compare X"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcmpx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Compare X"]
    #[inline(always)]
    pub const fn set_edgcmpx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Edge Counter X"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntx(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Counter X"]
    #[inline(always)]
    pub const fn set_edgcntx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sm0captcompx {
    #[inline(always)]
    fn default() -> Sm0captcompx {
        Sm0captcompx(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm0captcompx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0captcompx")
            .field("edgcmpx", &self.edgcmpx())
            .field("edgcntx", &self.edgcntx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0captcompx {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm0captcompx {
            edgcmpx: u8,
            edgcntx: u8,
        }
        let proxy = Sm0captcompx {
            edgcmpx: self.edgcmpx(),
            edgcntx: self.edgcntx(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Control A Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0captctrla(pub u16);
impl Sm0captctrla {
    #[doc = "Arm A"]
    #[must_use]
    #[inline(always)]
    pub const fn arma(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Arm A"]
    #[inline(always)]
    pub const fn set_arma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "One Shot Mode A"]
    #[must_use]
    #[inline(always)]
    pub const fn oneshota(&self) -> super::vals::Sm0captctrlaOneshota {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sm0captctrlaOneshota::from_bits(val as u8)
    }
    #[doc = "One Shot Mode A"]
    #[inline(always)]
    pub const fn set_oneshota(&mut self, val: super::vals::Sm0captctrlaOneshota) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Edge A 0"]
    #[must_use]
    #[inline(always)]
    pub const fn edga0(&self) -> super::vals::Sm0captctrlaEdga0 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Sm0captctrlaEdga0::from_bits(val as u8)
    }
    #[doc = "Edge A 0"]
    #[inline(always)]
    pub const fn set_edga0(&mut self, val: super::vals::Sm0captctrlaEdga0) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Edge A 1"]
    #[must_use]
    #[inline(always)]
    pub const fn edga1(&self) -> super::vals::Sm0captctrlaEdga1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Sm0captctrlaEdga1::from_bits(val as u8)
    }
    #[doc = "Edge A 1"]
    #[inline(always)]
    pub const fn set_edga1(&mut self, val: super::vals::Sm0captctrlaEdga1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Input Select A"]
    #[must_use]
    #[inline(always)]
    pub const fn inp_sela(&self) -> super::vals::Sm0captctrlaInpSela {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Sm0captctrlaInpSela::from_bits(val as u8)
    }
    #[doc = "Input Select A"]
    #[inline(always)]
    pub const fn set_inp_sela(&mut self, val: super::vals::Sm0captctrlaInpSela) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Edge Counter A Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcnta_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Edge Counter A Enable"]
    #[inline(always)]
    pub const fn set_edgcnta_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture A FIFOs Water Mark"]
    #[must_use]
    #[inline(always)]
    pub const fn cfawm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Capture A FIFOs Water Mark"]
    #[inline(always)]
    pub const fn set_cfawm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Capture A0 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn ca0cnt(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Capture A0 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_ca0cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u16) & 0x07) << 10usize);
    }
    #[doc = "Capture A1 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn ca1cnt(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Capture A1 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_ca1cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u16) & 0x07) << 13usize);
    }
}
impl Default for Sm0captctrla {
    #[inline(always)]
    fn default() -> Sm0captctrla {
        Sm0captctrla(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm0captctrla {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0captctrla")
            .field("arma", &self.arma())
            .field("oneshota", &self.oneshota())
            .field("edga0", &self.edga0())
            .field("edga1", &self.edga1())
            .field("inp_sela", &self.inp_sela())
            .field("edgcnta_en", &self.edgcnta_en())
            .field("cfawm", &self.cfawm())
            .field("ca0cnt", &self.ca0cnt())
            .field("ca1cnt", &self.ca1cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0captctrla {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm0captctrla {
            arma: bool,
            oneshota: super::vals::Sm0captctrlaOneshota,
            edga0: super::vals::Sm0captctrlaEdga0,
            edga1: super::vals::Sm0captctrlaEdga1,
            inp_sela: super::vals::Sm0captctrlaInpSela,
            edgcnta_en: bool,
            cfawm: u8,
            ca0cnt: u8,
            ca1cnt: u8,
        }
        let proxy = Sm0captctrla {
            arma: self.arma(),
            oneshota: self.oneshota(),
            edga0: self.edga0(),
            edga1: self.edga1(),
            inp_sela: self.inp_sela(),
            edgcnta_en: self.edgcnta_en(),
            cfawm: self.cfawm(),
            ca0cnt: self.ca0cnt(),
            ca1cnt: self.ca1cnt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Control B Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0captctrlb(pub u16);
impl Sm0captctrlb {
    #[doc = "Arm B"]
    #[must_use]
    #[inline(always)]
    pub const fn armb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Arm B"]
    #[inline(always)]
    pub const fn set_armb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "One Shot Mode B"]
    #[must_use]
    #[inline(always)]
    pub const fn oneshotb(&self) -> super::vals::Sm0captctrlbOneshotb {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sm0captctrlbOneshotb::from_bits(val as u8)
    }
    #[doc = "One Shot Mode B"]
    #[inline(always)]
    pub const fn set_oneshotb(&mut self, val: super::vals::Sm0captctrlbOneshotb) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Edge B 0"]
    #[must_use]
    #[inline(always)]
    pub const fn edgb0(&self) -> super::vals::Sm0captctrlbEdgb0 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Sm0captctrlbEdgb0::from_bits(val as u8)
    }
    #[doc = "Edge B 0"]
    #[inline(always)]
    pub const fn set_edgb0(&mut self, val: super::vals::Sm0captctrlbEdgb0) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Edge B 1"]
    #[must_use]
    #[inline(always)]
    pub const fn edgb1(&self) -> super::vals::Sm0captctrlbEdgb1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Sm0captctrlbEdgb1::from_bits(val as u8)
    }
    #[doc = "Edge B 1"]
    #[inline(always)]
    pub const fn set_edgb1(&mut self, val: super::vals::Sm0captctrlbEdgb1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Input Select B"]
    #[must_use]
    #[inline(always)]
    pub const fn inp_selb(&self) -> super::vals::Sm0captctrlbInpSelb {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Sm0captctrlbInpSelb::from_bits(val as u8)
    }
    #[doc = "Input Select B"]
    #[inline(always)]
    pub const fn set_inp_selb(&mut self, val: super::vals::Sm0captctrlbInpSelb) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Edge Counter B Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntb_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Edge Counter B Enable"]
    #[inline(always)]
    pub const fn set_edgcntb_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture B FIFOs Water Mark"]
    #[must_use]
    #[inline(always)]
    pub const fn cfbwm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Capture B FIFOs Water Mark"]
    #[inline(always)]
    pub const fn set_cfbwm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Capture B0 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn cb0cnt(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Capture B0 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_cb0cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u16) & 0x07) << 10usize);
    }
    #[doc = "Capture B1 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn cb1cnt(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Capture B1 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_cb1cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u16) & 0x07) << 13usize);
    }
}
impl Default for Sm0captctrlb {
    #[inline(always)]
    fn default() -> Sm0captctrlb {
        Sm0captctrlb(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm0captctrlb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0captctrlb")
            .field("armb", &self.armb())
            .field("oneshotb", &self.oneshotb())
            .field("edgb0", &self.edgb0())
            .field("edgb1", &self.edgb1())
            .field("inp_selb", &self.inp_selb())
            .field("edgcntb_en", &self.edgcntb_en())
            .field("cfbwm", &self.cfbwm())
            .field("cb0cnt", &self.cb0cnt())
            .field("cb1cnt", &self.cb1cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0captctrlb {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm0captctrlb {
            armb: bool,
            oneshotb: super::vals::Sm0captctrlbOneshotb,
            edgb0: super::vals::Sm0captctrlbEdgb0,
            edgb1: super::vals::Sm0captctrlbEdgb1,
            inp_selb: super::vals::Sm0captctrlbInpSelb,
            edgcntb_en: bool,
            cfbwm: u8,
            cb0cnt: u8,
            cb1cnt: u8,
        }
        let proxy = Sm0captctrlb {
            armb: self.armb(),
            oneshotb: self.oneshotb(),
            edgb0: self.edgb0(),
            edgb1: self.edgb1(),
            inp_selb: self.inp_selb(),
            edgcntb_en: self.edgcntb_en(),
            cfbwm: self.cfbwm(),
            cb0cnt: self.cb0cnt(),
            cb1cnt: self.cb1cnt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Control X Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0captctrlx(pub u16);
impl Sm0captctrlx {
    #[doc = "Arm X"]
    #[must_use]
    #[inline(always)]
    pub const fn armx(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Arm X"]
    #[inline(always)]
    pub const fn set_armx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "One Shot Mode Aux"]
    #[must_use]
    #[inline(always)]
    pub const fn oneshotx(&self) -> super::vals::Sm0captctrlxOneshotx {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sm0captctrlxOneshotx::from_bits(val as u8)
    }
    #[doc = "One Shot Mode Aux"]
    #[inline(always)]
    pub const fn set_oneshotx(&mut self, val: super::vals::Sm0captctrlxOneshotx) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Edge X 0"]
    #[must_use]
    #[inline(always)]
    pub const fn edgx0(&self) -> super::vals::Sm0captctrlxEdgx0 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Sm0captctrlxEdgx0::from_bits(val as u8)
    }
    #[doc = "Edge X 0"]
    #[inline(always)]
    pub const fn set_edgx0(&mut self, val: super::vals::Sm0captctrlxEdgx0) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Edge X 1"]
    #[must_use]
    #[inline(always)]
    pub const fn edgx1(&self) -> super::vals::Sm0captctrlxEdgx1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Sm0captctrlxEdgx1::from_bits(val as u8)
    }
    #[doc = "Edge X 1"]
    #[inline(always)]
    pub const fn set_edgx1(&mut self, val: super::vals::Sm0captctrlxEdgx1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Input Select X"]
    #[must_use]
    #[inline(always)]
    pub const fn inp_selx(&self) -> super::vals::Sm0captctrlxInpSelx {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Sm0captctrlxInpSelx::from_bits(val as u8)
    }
    #[doc = "Input Select X"]
    #[inline(always)]
    pub const fn set_inp_selx(&mut self, val: super::vals::Sm0captctrlxInpSelx) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Edge Counter X Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntx_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Edge Counter X Enable"]
    #[inline(always)]
    pub const fn set_edgcntx_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture X FIFOs Water Mark"]
    #[must_use]
    #[inline(always)]
    pub const fn cfxwm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Capture X FIFOs Water Mark"]
    #[inline(always)]
    pub const fn set_cfxwm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Capture X0 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn cx0cnt(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Capture X0 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_cx0cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u16) & 0x07) << 10usize);
    }
    #[doc = "Capture X1 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn cx1cnt(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Capture X1 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_cx1cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u16) & 0x07) << 13usize);
    }
}
impl Default for Sm0captctrlx {
    #[inline(always)]
    fn default() -> Sm0captctrlx {
        Sm0captctrlx(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm0captctrlx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0captctrlx")
            .field("armx", &self.armx())
            .field("oneshotx", &self.oneshotx())
            .field("edgx0", &self.edgx0())
            .field("edgx1", &self.edgx1())
            .field("inp_selx", &self.inp_selx())
            .field("edgcntx_en", &self.edgcntx_en())
            .field("cfxwm", &self.cfxwm())
            .field("cx0cnt", &self.cx0cnt())
            .field("cx1cnt", &self.cx1cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0captctrlx {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm0captctrlx {
            armx: bool,
            oneshotx: super::vals::Sm0captctrlxOneshotx,
            edgx0: super::vals::Sm0captctrlxEdgx0,
            edgx1: super::vals::Sm0captctrlxEdgx1,
            inp_selx: super::vals::Sm0captctrlxInpSelx,
            edgcntx_en: bool,
            cfxwm: u8,
            cx0cnt: u8,
            cx1cnt: u8,
        }
        let proxy = Sm0captctrlx {
            armx: self.armx(),
            oneshotx: self.oneshotx(),
            edgx0: self.edgx0(),
            edgx1: self.edgx1(),
            inp_selx: self.inp_selx(),
            edgcntx_en: self.edgcntx_en(),
            cfxwm: self.cfxwm(),
            cx0cnt: self.cx0cnt(),
            cx1cnt: self.cx1cnt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture PWM_A Input Filter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0captfilta(pub u16);
impl Sm0captfilta {
    #[doc = "Input Capture Filter Period"]
    #[must_use]
    #[inline(always)]
    pub const fn capta_filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Capture Filter Period"]
    #[inline(always)]
    pub const fn set_capta_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Input Capture Filter Count"]
    #[must_use]
    #[inline(always)]
    pub const fn capta_filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Capture Filter Count"]
    #[inline(always)]
    pub const fn set_capta_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
}
impl Default for Sm0captfilta {
    #[inline(always)]
    fn default() -> Sm0captfilta {
        Sm0captfilta(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm0captfilta {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0captfilta")
            .field("capta_filt_per", &self.capta_filt_per())
            .field("capta_filt_cnt", &self.capta_filt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0captfilta {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm0captfilta {
            capta_filt_per: u8,
            capta_filt_cnt: u8,
        }
        let proxy = Sm0captfilta {
            capta_filt_per: self.capta_filt_per(),
            capta_filt_cnt: self.capta_filt_cnt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture PWM_B Input Filter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0captfiltb(pub u16);
impl Sm0captfiltb {
    #[doc = "Input Capture Filter Period"]
    #[must_use]
    #[inline(always)]
    pub const fn captb_filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Capture Filter Period"]
    #[inline(always)]
    pub const fn set_captb_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Input Capture Filter Count"]
    #[must_use]
    #[inline(always)]
    pub const fn captb_filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Capture Filter Count"]
    #[inline(always)]
    pub const fn set_captb_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
}
impl Default for Sm0captfiltb {
    #[inline(always)]
    fn default() -> Sm0captfiltb {
        Sm0captfiltb(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm0captfiltb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0captfiltb")
            .field("captb_filt_per", &self.captb_filt_per())
            .field("captb_filt_cnt", &self.captb_filt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0captfiltb {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm0captfiltb {
            captb_filt_per: u8,
            captb_filt_cnt: u8,
        }
        let proxy = Sm0captfiltb {
            captb_filt_per: self.captb_filt_per(),
            captb_filt_cnt: self.captb_filt_cnt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture PWM_X Input Filter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0captfiltx(pub u16);
impl Sm0captfiltx {
    #[doc = "Input Capture Filter Period"]
    #[must_use]
    #[inline(always)]
    pub const fn captx_filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Capture Filter Period"]
    #[inline(always)]
    pub const fn set_captx_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Input Capture Filter Count"]
    #[must_use]
    #[inline(always)]
    pub const fn captx_filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Capture Filter Count"]
    #[inline(always)]
    pub const fn set_captx_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
}
impl Default for Sm0captfiltx {
    #[inline(always)]
    fn default() -> Sm0captfiltx {
        Sm0captfiltx(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm0captfiltx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0captfiltx")
            .field("captx_filt_per", &self.captx_filt_per())
            .field("captx_filt_cnt", &self.captx_filt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0captfiltx {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm0captfiltx {
            captx_filt_per: u8,
            captx_filt_cnt: u8,
        }
        let proxy = Sm0captfiltx {
            captx_filt_per: self.captx_filt_per(),
            captx_filt_cnt: self.captx_filt_cnt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0ctrl(pub u16);
impl Sm0ctrl {
    #[doc = "Double Switching Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dblen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Double Switching Enable"]
    #[inline(always)]
    pub const fn set_dblen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "PWM_X Double Switching Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dblx(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Double Switching Enable"]
    #[inline(always)]
    pub const fn set_dblx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Load Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ldmod(&self) -> super::vals::Sm0ctrlLdmod {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Sm0ctrlLdmod::from_bits(val as u8)
    }
    #[doc = "Load Mode Select"]
    #[inline(always)]
    pub const fn set_ldmod(&mut self, val: super::vals::Sm0ctrlLdmod) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Split the DBLPWM signal to PWM_A and PWM_B"]
    #[must_use]
    #[inline(always)]
    pub const fn split(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Split the DBLPWM signal to PWM_A and PWM_B"]
    #[inline(always)]
    pub const fn set_split(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Prescaler"]
    #[must_use]
    #[inline(always)]
    pub const fn prsc(&self) -> super::vals::Sm0ctrlPrsc {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Sm0ctrlPrsc::from_bits(val as u8)
    }
    #[doc = "Prescaler"]
    #[inline(always)]
    pub const fn set_prsc(&mut self, val: super::vals::Sm0ctrlPrsc) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u16) & 0x07) << 4usize);
    }
    #[doc = "Compare Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn compmode(&self) -> super::vals::Sm0ctrlCompmode {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Sm0ctrlCompmode::from_bits(val as u8)
    }
    #[doc = "Compare Mode"]
    #[inline(always)]
    pub const fn set_compmode(&mut self, val: super::vals::Sm0ctrlCompmode) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "Deadtime"]
    #[must_use]
    #[inline(always)]
    pub const fn dt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Deadtime"]
    #[inline(always)]
    pub const fn set_dt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Full Cycle Reload"]
    #[must_use]
    #[inline(always)]
    pub const fn full(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Full Cycle Reload"]
    #[inline(always)]
    pub const fn set_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Half Cycle Reload"]
    #[must_use]
    #[inline(always)]
    pub const fn half(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Half Cycle Reload"]
    #[inline(always)]
    pub const fn set_half(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Load Frequency"]
    #[must_use]
    #[inline(always)]
    pub const fn ldfq(&self) -> super::vals::Sm0ctrlLdfq {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Sm0ctrlLdfq::from_bits(val as u8)
    }
    #[doc = "Load Frequency"]
    #[inline(always)]
    pub const fn set_ldfq(&mut self, val: super::vals::Sm0ctrlLdfq) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u16) & 0x0f) << 12usize);
    }
}
impl Default for Sm0ctrl {
    #[inline(always)]
    fn default() -> Sm0ctrl {
        Sm0ctrl(1024u64 as u16)
    }
}
impl core::fmt::Debug for Sm0ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0ctrl")
            .field("dblen", &self.dblen())
            .field("dblx", &self.dblx())
            .field("ldmod", &self.ldmod())
            .field("split", &self.split())
            .field("prsc", &self.prsc())
            .field("compmode", &self.compmode())
            .field("dt", &self.dt())
            .field("full", &self.full())
            .field("half", &self.half())
            .field("ldfq", &self.ldfq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0ctrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm0ctrl {
            dblen: bool,
            dblx: bool,
            ldmod: super::vals::Sm0ctrlLdmod,
            split: bool,
            prsc: super::vals::Sm0ctrlPrsc,
            compmode: super::vals::Sm0ctrlCompmode,
            dt: u8,
            full: bool,
            half: bool,
            ldfq: super::vals::Sm0ctrlLdfq,
        }
        let proxy = Sm0ctrl {
            dblen: self.dblen(),
            dblx: self.dblx(),
            ldmod: self.ldmod(),
            split: self.split(),
            prsc: self.prsc(),
            compmode: self.compmode(),
            dt: self.dt(),
            full: self.full(),
            half: self.half(),
            ldfq: self.ldfq(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Control 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0ctrl2(pub u16);
impl Sm0ctrl2 {
    #[doc = "Clock Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn clk_sel(&self) -> super::vals::Sm0ctrl2ClkSel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sm0ctrl2ClkSel::from_bits(val as u8)
    }
    #[doc = "Clock Source Select"]
    #[inline(always)]
    pub const fn set_clk_sel(&mut self, val: super::vals::Sm0ctrl2ClkSel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Reload Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn reload_sel(&self) -> super::vals::Sm0ctrl2ReloadSel {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Sm0ctrl2ReloadSel::from_bits(val as u8)
    }
    #[doc = "Reload Source Select"]
    #[inline(always)]
    pub const fn set_reload_sel(&mut self, val: super::vals::Sm0ctrl2ReloadSel) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Force Select"]
    #[must_use]
    #[inline(always)]
    pub const fn force_sel(&self) -> super::vals::Sm0ctrl2ForceSel {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::Sm0ctrl2ForceSel::from_bits(val as u8)
    }
    #[doc = "Force Select"]
    #[inline(always)]
    pub const fn set_force_sel(&mut self, val: super::vals::Sm0ctrl2ForceSel) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u16) & 0x07) << 3usize);
    }
    #[doc = "Force Initialization"]
    #[must_use]
    #[inline(always)]
    pub const fn force(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Force Initialization"]
    #[inline(always)]
    pub const fn set_force(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Force Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn frcen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Force Enable"]
    #[inline(always)]
    pub const fn set_frcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Initialization Control Select"]
    #[must_use]
    #[inline(always)]
    pub const fn init_sel(&self) -> super::vals::Sm0ctrl2InitSel {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Sm0ctrl2InitSel::from_bits(val as u8)
    }
    #[doc = "Initialization Control Select"]
    #[inline(always)]
    pub const fn set_init_sel(&mut self, val: super::vals::Sm0ctrl2InitSel) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "PWM_X Initial Value"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmx_init(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Initial Value"]
    #[inline(always)]
    pub const fn set_pwmx_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "PWM45 Initial Value"]
    #[must_use]
    #[inline(always)]
    pub const fn pwm45_init(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "PWM45 Initial Value"]
    #[inline(always)]
    pub const fn set_pwm45_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "PWM23 Initial Value"]
    #[must_use]
    #[inline(always)]
    pub const fn pwm23_init(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "PWM23 Initial Value"]
    #[inline(always)]
    pub const fn set_pwm23_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Independent or Complementary Pair Operation"]
    #[must_use]
    #[inline(always)]
    pub const fn indep(&self) -> super::vals::Sm0ctrl2Indep {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Sm0ctrl2Indep::from_bits(val as u8)
    }
    #[doc = "Independent or Complementary Pair Operation"]
    #[inline(always)]
    pub const fn set_indep(&mut self, val: super::vals::Sm0ctrl2Indep) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "Wait Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn waiten(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Wait Enable"]
    #[inline(always)]
    pub const fn set_waiten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "Debug Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dbgen(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Enable"]
    #[inline(always)]
    pub const fn set_dbgen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Sm0ctrl2 {
    #[inline(always)]
    fn default() -> Sm0ctrl2 {
        Sm0ctrl2(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm0ctrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0ctrl2")
            .field("clk_sel", &self.clk_sel())
            .field("reload_sel", &self.reload_sel())
            .field("force_sel", &self.force_sel())
            .field("force", &self.force())
            .field("frcen", &self.frcen())
            .field("init_sel", &self.init_sel())
            .field("pwmx_init", &self.pwmx_init())
            .field("pwm45_init", &self.pwm45_init())
            .field("pwm23_init", &self.pwm23_init())
            .field("indep", &self.indep())
            .field("waiten", &self.waiten())
            .field("dbgen", &self.dbgen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0ctrl2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm0ctrl2 {
            clk_sel: super::vals::Sm0ctrl2ClkSel,
            reload_sel: super::vals::Sm0ctrl2ReloadSel,
            force_sel: super::vals::Sm0ctrl2ForceSel,
            force: bool,
            frcen: bool,
            init_sel: super::vals::Sm0ctrl2InitSel,
            pwmx_init: bool,
            pwm45_init: bool,
            pwm23_init: bool,
            indep: super::vals::Sm0ctrl2Indep,
            waiten: bool,
            dbgen: bool,
        }
        let proxy = Sm0ctrl2 {
            clk_sel: self.clk_sel(),
            reload_sel: self.reload_sel(),
            force_sel: self.force_sel(),
            force: self.force(),
            frcen: self.frcen(),
            init_sel: self.init_sel(),
            pwmx_init: self.pwmx_init(),
            pwm45_init: self.pwm45_init(),
            pwm23_init: self.pwm23_init(),
            indep: self.indep(),
            waiten: self.waiten(),
            dbgen: self.dbgen(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Value 0 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0cval0cyc(pub u16);
impl Sm0cval0cyc {
    #[doc = "Capture Value 0 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval0cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 0 Cycle"]
    #[inline(always)]
    pub const fn set_cval0cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm0cval0cyc {
    #[inline(always)]
    fn default() -> Sm0cval0cyc {
        Sm0cval0cyc(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm0cval0cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0cval0cyc")
            .field("cval0cyc", &self.cval0cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0cval0cyc {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm0cval0cyc {
            cval0cyc: u8,
        }
        let proxy = Sm0cval0cyc {
            cval0cyc: self.cval0cyc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Value 1 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0cval1cyc(pub u16);
impl Sm0cval1cyc {
    #[doc = "Capture Value 1 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval1cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 1 Cycle"]
    #[inline(always)]
    pub const fn set_cval1cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm0cval1cyc {
    #[inline(always)]
    fn default() -> Sm0cval1cyc {
        Sm0cval1cyc(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm0cval1cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0cval1cyc")
            .field("cval1cyc", &self.cval1cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0cval1cyc {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm0cval1cyc {
            cval1cyc: u8,
        }
        let proxy = Sm0cval1cyc {
            cval1cyc: self.cval1cyc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Value 2 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0cval2cyc(pub u16);
impl Sm0cval2cyc {
    #[doc = "Capture Value 2 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval2cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 2 Cycle"]
    #[inline(always)]
    pub const fn set_cval2cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm0cval2cyc {
    #[inline(always)]
    fn default() -> Sm0cval2cyc {
        Sm0cval2cyc(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm0cval2cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0cval2cyc")
            .field("cval2cyc", &self.cval2cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0cval2cyc {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm0cval2cyc {
            cval2cyc: u8,
        }
        let proxy = Sm0cval2cyc {
            cval2cyc: self.cval2cyc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Value 3 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0cval3cyc(pub u16);
impl Sm0cval3cyc {
    #[doc = "Capture Value 3 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval3cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 3 Cycle"]
    #[inline(always)]
    pub const fn set_cval3cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm0cval3cyc {
    #[inline(always)]
    fn default() -> Sm0cval3cyc {
        Sm0cval3cyc(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm0cval3cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0cval3cyc")
            .field("cval3cyc", &self.cval3cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0cval3cyc {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm0cval3cyc {
            cval3cyc: u8,
        }
        let proxy = Sm0cval3cyc {
            cval3cyc: self.cval3cyc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Value 4 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0cval4cyc(pub u16);
impl Sm0cval4cyc {
    #[doc = "Capture Value 4 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval4cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 4 Cycle"]
    #[inline(always)]
    pub const fn set_cval4cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm0cval4cyc {
    #[inline(always)]
    fn default() -> Sm0cval4cyc {
        Sm0cval4cyc(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm0cval4cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0cval4cyc")
            .field("cval4cyc", &self.cval4cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0cval4cyc {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm0cval4cyc {
            cval4cyc: u8,
        }
        let proxy = Sm0cval4cyc {
            cval4cyc: self.cval4cyc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Value 5 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0cval5cyc(pub u16);
impl Sm0cval5cyc {
    #[doc = "Capture Value 5 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval5cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 5 Cycle"]
    #[inline(always)]
    pub const fn set_cval5cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm0cval5cyc {
    #[inline(always)]
    fn default() -> Sm0cval5cyc {
        Sm0cval5cyc(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm0cval5cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0cval5cyc")
            .field("cval5cyc", &self.cval5cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0cval5cyc {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm0cval5cyc {
            cval5cyc: u8,
        }
        let proxy = Sm0cval5cyc {
            cval5cyc: self.cval5cyc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fault Disable Mapping Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0dismap0(pub u16);
impl Sm0dismap0 {
    #[doc = "PWM_A Fault Disable Mask 0"]
    #[must_use]
    #[inline(always)]
    pub const fn dis0a(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_A Fault Disable Mask 0"]
    #[inline(always)]
    pub const fn set_dis0a(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "PWM_B Fault Disable Mask 0"]
    #[must_use]
    #[inline(always)]
    pub const fn dis0b(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_B Fault Disable Mask 0"]
    #[inline(always)]
    pub const fn set_dis0b(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
    }
    #[doc = "PWM_X Fault Disable Mask 0"]
    #[must_use]
    #[inline(always)]
    pub const fn dis0x(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_X Fault Disable Mask 0"]
    #[inline(always)]
    pub const fn set_dis0x(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
}
impl Default for Sm0dismap0 {
    #[inline(always)]
    fn default() -> Sm0dismap0 {
        Sm0dismap0(65535u64 as u16)
    }
}
impl core::fmt::Debug for Sm0dismap0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0dismap0")
            .field("dis0a", &self.dis0a())
            .field("dis0b", &self.dis0b())
            .field("dis0x", &self.dis0x())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0dismap0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm0dismap0 {
            dis0a: u8,
            dis0b: u8,
            dis0x: u8,
        }
        let proxy = Sm0dismap0 {
            dis0a: self.dis0a(),
            dis0b: self.dis0b(),
            dis0x: self.dis0x(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "DMA Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0dmaen(pub u16);
impl Sm0dmaen {
    #[doc = "Capture X0 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cx0de(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X0 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_cx0de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Capture X1 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cx1de(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X1 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_cx1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Capture B0 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cb0de(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Capture B0 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_cb0de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Capture B1 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cb1de(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Capture B1 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_cb1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Capture A0 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ca0de(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Capture A0 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_ca0de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Capture A1 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ca1de(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Capture A1 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_ca1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Capture DMA Enable Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn captde(&self) -> super::vals::Sm0dmaenCaptde {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Sm0dmaenCaptde::from_bits(val as u8)
    }
    #[doc = "Capture DMA Enable Source Select"]
    #[inline(always)]
    pub const fn set_captde(&mut self, val: super::vals::Sm0dmaenCaptde) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "FIFO Watermark AND Control"]
    #[must_use]
    #[inline(always)]
    pub const fn fand(&self) -> super::vals::Sm0dmaenFand {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Sm0dmaenFand::from_bits(val as u8)
    }
    #[doc = "FIFO Watermark AND Control"]
    #[inline(always)]
    pub const fn set_fand(&mut self, val: super::vals::Sm0dmaenFand) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "Value Registers DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn valde(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Value Registers DMA Enable"]
    #[inline(always)]
    pub const fn set_valde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
}
impl Default for Sm0dmaen {
    #[inline(always)]
    fn default() -> Sm0dmaen {
        Sm0dmaen(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm0dmaen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0dmaen")
            .field("cx0de", &self.cx0de())
            .field("cx1de", &self.cx1de())
            .field("cb0de", &self.cb0de())
            .field("cb1de", &self.cb1de())
            .field("ca0de", &self.ca0de())
            .field("ca1de", &self.ca1de())
            .field("captde", &self.captde())
            .field("fand", &self.fand())
            .field("valde", &self.valde())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0dmaen {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm0dmaen {
            cx0de: bool,
            cx1de: bool,
            cb0de: bool,
            cb1de: bool,
            ca0de: bool,
            ca1de: bool,
            captde: super::vals::Sm0dmaenCaptde,
            fand: super::vals::Sm0dmaenFand,
            valde: bool,
        }
        let proxy = Sm0dmaen {
            cx0de: self.cx0de(),
            cx1de: self.cx1de(),
            cb0de: self.cb0de(),
            cb1de: self.cb1de(),
            ca0de: self.ca0de(),
            ca1de: self.ca1de(),
            captde: self.captde(),
            fand: self.fand(),
            valde: self.valde(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fractional Value Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0fracval1(pub u16);
impl Sm0fracval1 {
    #[doc = "Fractional Value 1"]
    #[must_use]
    #[inline(always)]
    pub const fn fracval1(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 1"]
    #[inline(always)]
    pub const fn set_fracval1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Sm0fracval1 {
    #[inline(always)]
    fn default() -> Sm0fracval1 {
        Sm0fracval1(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm0fracval1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0fracval1")
            .field("fracval1", &self.fracval1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0fracval1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm0fracval1 {
            fracval1: u8,
        }
        let proxy = Sm0fracval1 {
            fracval1: self.fracval1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fractional Value Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0fracval2(pub u16);
impl Sm0fracval2 {
    #[doc = "Fractional Value 2"]
    #[must_use]
    #[inline(always)]
    pub const fn fracval2(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 2"]
    #[inline(always)]
    pub const fn set_fracval2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Sm0fracval2 {
    #[inline(always)]
    fn default() -> Sm0fracval2 {
        Sm0fracval2(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm0fracval2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0fracval2")
            .field("fracval2", &self.fracval2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0fracval2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm0fracval2 {
            fracval2: u8,
        }
        let proxy = Sm0fracval2 {
            fracval2: self.fracval2(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fractional Value Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0fracval3(pub u16);
impl Sm0fracval3 {
    #[doc = "Fractional Value 3"]
    #[must_use]
    #[inline(always)]
    pub const fn fracval3(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 3"]
    #[inline(always)]
    pub const fn set_fracval3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Sm0fracval3 {
    #[inline(always)]
    fn default() -> Sm0fracval3 {
        Sm0fracval3(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm0fracval3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0fracval3")
            .field("fracval3", &self.fracval3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0fracval3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm0fracval3 {
            fracval3: u8,
        }
        let proxy = Sm0fracval3 {
            fracval3: self.fracval3(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fractional Value Register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0fracval4(pub u16);
impl Sm0fracval4 {
    #[doc = "Fractional Value 4"]
    #[must_use]
    #[inline(always)]
    pub const fn fracval4(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 4"]
    #[inline(always)]
    pub const fn set_fracval4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Sm0fracval4 {
    #[inline(always)]
    fn default() -> Sm0fracval4 {
        Sm0fracval4(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm0fracval4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0fracval4")
            .field("fracval4", &self.fracval4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0fracval4 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm0fracval4 {
            fracval4: u8,
        }
        let proxy = Sm0fracval4 {
            fracval4: self.fracval4(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fractional Value Register 5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0fracval5(pub u16);
impl Sm0fracval5 {
    #[doc = "Fractional Value 5"]
    #[must_use]
    #[inline(always)]
    pub const fn fracval5(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 5"]
    #[inline(always)]
    pub const fn set_fracval5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Sm0fracval5 {
    #[inline(always)]
    fn default() -> Sm0fracval5 {
        Sm0fracval5(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm0fracval5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0fracval5")
            .field("fracval5", &self.fracval5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0fracval5 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm0fracval5 {
            fracval5: u8,
        }
        let proxy = Sm0fracval5 {
            fracval5: self.fracval5(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fractional Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0frctrl(pub u16);
impl Sm0frctrl {
    #[doc = "Fractional Cycle PWM Period Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn frac1_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Fractional Cycle PWM Period Enable"]
    #[inline(always)]
    pub const fn set_frac1_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Fractional Cycle Placement Enable for PWM_A"]
    #[must_use]
    #[inline(always)]
    pub const fn frac23_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Fractional Cycle Placement Enable for PWM_A"]
    #[inline(always)]
    pub const fn set_frac23_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Fractional Cycle Placement Enable for PWM_B"]
    #[must_use]
    #[inline(always)]
    pub const fn frac45_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Fractional Cycle Placement Enable for PWM_B"]
    #[inline(always)]
    pub const fn set_frac45_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Test Status Bit"]
    #[must_use]
    #[inline(always)]
    pub const fn test(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Test Status Bit"]
    #[inline(always)]
    pub const fn set_test(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Sm0frctrl {
    #[inline(always)]
    fn default() -> Sm0frctrl {
        Sm0frctrl(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm0frctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0frctrl")
            .field("frac1_en", &self.frac1_en())
            .field("frac23_en", &self.frac23_en())
            .field("frac45_en", &self.frac45_en())
            .field("test", &self.test())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0frctrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm0frctrl {
            frac1_en: bool,
            frac23_en: bool,
            frac45_en: bool,
            test: bool,
        }
        let proxy = Sm0frctrl {
            frac1_en: self.frac1_en(),
            frac23_en: self.frac23_en(),
            frac45_en: self.frac45_en(),
            test: self.test(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0inten(pub u16);
impl Sm0inten {
    #[doc = "Compare Interrupt Enables"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpie(&self) -> super::vals::Sm0intenCmpie {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Sm0intenCmpie::from_bits(val as u8)
    }
    #[doc = "Compare Interrupt Enables"]
    #[inline(always)]
    pub const fn set_cmpie(&mut self, val: super::vals::Sm0intenCmpie) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Capture X 0 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cx0ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X 0 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cx0ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Capture X 1 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cx1ie(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cx1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture B 0 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cb0ie(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Capture B 0 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cb0ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "Capture B 1 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cb1ie(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Capture B 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cb1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "Capture A 0 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ca0ie(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Capture A 0 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ca0ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Capture A 1 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ca1ie(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Capture A 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ca1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Reload Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Interrupt Enable"]
    #[inline(always)]
    pub const fn set_rie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Reload Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn reie(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_reie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
}
impl Default for Sm0inten {
    #[inline(always)]
    fn default() -> Sm0inten {
        Sm0inten(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm0inten {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0inten")
            .field("cmpie", &self.cmpie())
            .field("cx0ie", &self.cx0ie())
            .field("cx1ie", &self.cx1ie())
            .field("cb0ie", &self.cb0ie())
            .field("cb1ie", &self.cb1ie())
            .field("ca0ie", &self.ca0ie())
            .field("ca1ie", &self.ca1ie())
            .field("rie", &self.rie())
            .field("reie", &self.reie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0inten {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm0inten {
            cmpie: super::vals::Sm0intenCmpie,
            cx0ie: bool,
            cx1ie: bool,
            cb0ie: bool,
            cb1ie: bool,
            ca0ie: bool,
            ca1ie: bool,
            rie: bool,
            reie: bool,
        }
        let proxy = Sm0inten {
            cmpie: self.cmpie(),
            cx0ie: self.cx0ie(),
            cx1ie: self.cx1ie(),
            cb0ie: self.cb0ie(),
            cb1ie: self.cb1ie(),
            ca0ie: self.ca0ie(),
            ca1ie: self.ca1ie(),
            rie: self.rie(),
            reie: self.reie(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Output Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0octrl(pub u16);
impl Sm0octrl {
    #[doc = "PWM_X Fault State"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmxfs(&self) -> super::vals::Sm0octrlPwmxfs {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sm0octrlPwmxfs::from_bits(val as u8)
    }
    #[doc = "PWM_X Fault State"]
    #[inline(always)]
    pub const fn set_pwmxfs(&mut self, val: super::vals::Sm0octrlPwmxfs) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "PWM_B Fault State"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmbfs(&self) -> super::vals::Sm0octrlPwmbfs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Sm0octrlPwmbfs::from_bits(val as u8)
    }
    #[doc = "PWM_B Fault State"]
    #[inline(always)]
    pub const fn set_pwmbfs(&mut self, val: super::vals::Sm0octrlPwmbfs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "PWM_A Fault State"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmafs(&self) -> super::vals::Sm0octrlPwmafs {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Sm0octrlPwmafs::from_bits(val as u8)
    }
    #[doc = "PWM_A Fault State"]
    #[inline(always)]
    pub const fn set_pwmafs(&mut self, val: super::vals::Sm0octrlPwmafs) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "PWM_X Output Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn polx(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Output Polarity"]
    #[inline(always)]
    pub const fn set_polx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "PWM_B Output Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn polb(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_B Output Polarity"]
    #[inline(always)]
    pub const fn set_polb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "PWM_A Output Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn pola(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_A Output Polarity"]
    #[inline(always)]
    pub const fn set_pola(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "PWM_X Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmx_in(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Input"]
    #[inline(always)]
    pub const fn set_pwmx_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "PWM_B Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmb_in(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_B Input"]
    #[inline(always)]
    pub const fn set_pwmb_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "PWM_A Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pwma_in(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_A Input"]
    #[inline(always)]
    pub const fn set_pwma_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Sm0octrl {
    #[inline(always)]
    fn default() -> Sm0octrl {
        Sm0octrl(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm0octrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0octrl")
            .field("pwmxfs", &self.pwmxfs())
            .field("pwmbfs", &self.pwmbfs())
            .field("pwmafs", &self.pwmafs())
            .field("polx", &self.polx())
            .field("polb", &self.polb())
            .field("pola", &self.pola())
            .field("pwmx_in", &self.pwmx_in())
            .field("pwmb_in", &self.pwmb_in())
            .field("pwma_in", &self.pwma_in())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0octrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm0octrl {
            pwmxfs: super::vals::Sm0octrlPwmxfs,
            pwmbfs: super::vals::Sm0octrlPwmbfs,
            pwmafs: super::vals::Sm0octrlPwmafs,
            polx: bool,
            polb: bool,
            pola: bool,
            pwmx_in: bool,
            pwmb_in: bool,
            pwma_in: bool,
        }
        let proxy = Sm0octrl {
            pwmxfs: self.pwmxfs(),
            pwmbfs: self.pwmbfs(),
            pwmafs: self.pwmafs(),
            polx: self.polx(),
            polb: self.polb(),
            pola: self.pola(),
            pwmx_in: self.pwmx_in(),
            pwmb_in: self.pwmb_in(),
            pwma_in: self.pwma_in(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0sts(pub u16);
impl Sm0sts {
    #[doc = "Compare Flags"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpf(&self) -> super::vals::Sm0stsCmpf {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Sm0stsCmpf::from_bits(val as u8)
    }
    #[doc = "Compare Flags"]
    #[inline(always)]
    pub const fn set_cmpf(&mut self, val: super::vals::Sm0stsCmpf) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Capture Flag X0"]
    #[must_use]
    #[inline(always)]
    pub const fn cfx0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag X0"]
    #[inline(always)]
    pub const fn set_cfx0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Capture Flag X1"]
    #[must_use]
    #[inline(always)]
    pub const fn cfx1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag X1"]
    #[inline(always)]
    pub const fn set_cfx1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture Flag B0"]
    #[must_use]
    #[inline(always)]
    pub const fn cfb0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag B0"]
    #[inline(always)]
    pub const fn set_cfb0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "Capture Flag B1"]
    #[must_use]
    #[inline(always)]
    pub const fn cfb1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag B1"]
    #[inline(always)]
    pub const fn set_cfb1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "Capture Flag A0"]
    #[must_use]
    #[inline(always)]
    pub const fn cfa0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag A0"]
    #[inline(always)]
    pub const fn set_cfa0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Capture Flag A1"]
    #[must_use]
    #[inline(always)]
    pub const fn cfa1(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag A1"]
    #[inline(always)]
    pub const fn set_cfa1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Reload Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rf(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Flag"]
    #[inline(always)]
    pub const fn set_rf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Reload Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ref_(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Error Flag"]
    #[inline(always)]
    pub const fn set_ref_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Registers Updated Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ruf(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Registers Updated Flag"]
    #[inline(always)]
    pub const fn set_ruf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
}
impl Default for Sm0sts {
    #[inline(always)]
    fn default() -> Sm0sts {
        Sm0sts(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm0sts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0sts")
            .field("cmpf", &self.cmpf())
            .field("cfx0", &self.cfx0())
            .field("cfx1", &self.cfx1())
            .field("cfb0", &self.cfb0())
            .field("cfb1", &self.cfb1())
            .field("cfa0", &self.cfa0())
            .field("cfa1", &self.cfa1())
            .field("rf", &self.rf())
            .field("ref_", &self.ref_())
            .field("ruf", &self.ruf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0sts {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm0sts {
            cmpf: super::vals::Sm0stsCmpf,
            cfx0: bool,
            cfx1: bool,
            cfb0: bool,
            cfb1: bool,
            cfa0: bool,
            cfa1: bool,
            rf: bool,
            ref_: bool,
            ruf: bool,
        }
        let proxy = Sm0sts {
            cmpf: self.cmpf(),
            cfx0: self.cfx0(),
            cfx1: self.cfx1(),
            cfb0: self.cfb0(),
            cfb1: self.cfb1(),
            cfa0: self.cfa0(),
            cfa1: self.cfa1(),
            rf: self.rf(),
            ref_: self.ref_(),
            ruf: self.ruf(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Output Trigger Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0tctrl(pub u16);
impl Sm0tctrl {
    #[doc = "Output Trigger Enables"]
    #[must_use]
    #[inline(always)]
    pub const fn out_trig_en(&self) -> super::vals::Sm0tctrlOutTrigEn {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Sm0tctrlOutTrigEn::from_bits(val as u8)
    }
    #[doc = "Output Trigger Enables"]
    #[inline(always)]
    pub const fn set_out_trig_en(&mut self, val: super::vals::Sm0tctrlOutTrigEn) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Trigger Frequency"]
    #[must_use]
    #[inline(always)]
    pub const fn trgfrq(&self) -> super::vals::Sm0tctrlTrgfrq {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Sm0tctrlTrgfrq::from_bits(val as u8)
    }
    #[doc = "Trigger Frequency"]
    #[inline(always)]
    pub const fn set_trgfrq(&mut self, val: super::vals::Sm0tctrlTrgfrq) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
    #[doc = "Mux Output Trigger 1 Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn pwbot1(&self) -> super::vals::Sm0tctrlPwbot1 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Sm0tctrlPwbot1::from_bits(val as u8)
    }
    #[doc = "Mux Output Trigger 1 Source Select"]
    #[inline(always)]
    pub const fn set_pwbot1(&mut self, val: super::vals::Sm0tctrlPwbot1) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "Mux Output Trigger 0 Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn pwaot0(&self) -> super::vals::Sm0tctrlPwaot0 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Sm0tctrlPwaot0::from_bits(val as u8)
    }
    #[doc = "Mux Output Trigger 0 Source Select"]
    #[inline(always)]
    pub const fn set_pwaot0(&mut self, val: super::vals::Sm0tctrlPwaot0) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
    }
}
impl Default for Sm0tctrl {
    #[inline(always)]
    fn default() -> Sm0tctrl {
        Sm0tctrl(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm0tctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0tctrl")
            .field("out_trig_en", &self.out_trig_en())
            .field("trgfrq", &self.trgfrq())
            .field("pwbot1", &self.pwbot1())
            .field("pwaot0", &self.pwaot0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0tctrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm0tctrl {
            out_trig_en: super::vals::Sm0tctrlOutTrigEn,
            trgfrq: super::vals::Sm0tctrlTrgfrq,
            pwbot1: super::vals::Sm0tctrlPwbot1,
            pwaot0: super::vals::Sm0tctrlPwaot0,
        }
        let proxy = Sm0tctrl {
            out_trig_en: self.out_trig_en(),
            trgfrq: self.trgfrq(),
            pwbot1: self.pwbot1(),
            pwaot0: self.pwaot0(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Compare A Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1captcompa(pub u16);
impl Sm1captcompa {
    #[doc = "Edge Compare A"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcmpa(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Compare A"]
    #[inline(always)]
    pub const fn set_edgcmpa(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Edge Counter A"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcnta(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Counter A"]
    #[inline(always)]
    pub const fn set_edgcnta(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sm1captcompa {
    #[inline(always)]
    fn default() -> Sm1captcompa {
        Sm1captcompa(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm1captcompa {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1captcompa")
            .field("edgcmpa", &self.edgcmpa())
            .field("edgcnta", &self.edgcnta())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1captcompa {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm1captcompa {
            edgcmpa: u8,
            edgcnta: u8,
        }
        let proxy = Sm1captcompa {
            edgcmpa: self.edgcmpa(),
            edgcnta: self.edgcnta(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Compare B Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1captcompb(pub u16);
impl Sm1captcompb {
    #[doc = "Edge Compare B"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcmpb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Compare B"]
    #[inline(always)]
    pub const fn set_edgcmpb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Edge Counter B"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntb(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Counter B"]
    #[inline(always)]
    pub const fn set_edgcntb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sm1captcompb {
    #[inline(always)]
    fn default() -> Sm1captcompb {
        Sm1captcompb(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm1captcompb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1captcompb")
            .field("edgcmpb", &self.edgcmpb())
            .field("edgcntb", &self.edgcntb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1captcompb {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm1captcompb {
            edgcmpb: u8,
            edgcntb: u8,
        }
        let proxy = Sm1captcompb {
            edgcmpb: self.edgcmpb(),
            edgcntb: self.edgcntb(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Compare X Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1captcompx(pub u16);
impl Sm1captcompx {
    #[doc = "Edge Compare X"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcmpx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Compare X"]
    #[inline(always)]
    pub const fn set_edgcmpx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Edge Counter X"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntx(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Counter X"]
    #[inline(always)]
    pub const fn set_edgcntx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sm1captcompx {
    #[inline(always)]
    fn default() -> Sm1captcompx {
        Sm1captcompx(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm1captcompx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1captcompx")
            .field("edgcmpx", &self.edgcmpx())
            .field("edgcntx", &self.edgcntx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1captcompx {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm1captcompx {
            edgcmpx: u8,
            edgcntx: u8,
        }
        let proxy = Sm1captcompx {
            edgcmpx: self.edgcmpx(),
            edgcntx: self.edgcntx(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Control A Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1captctrla(pub u16);
impl Sm1captctrla {
    #[doc = "Arm A"]
    #[must_use]
    #[inline(always)]
    pub const fn arma(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Arm A"]
    #[inline(always)]
    pub const fn set_arma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "One Shot Mode A"]
    #[must_use]
    #[inline(always)]
    pub const fn oneshota(&self) -> super::vals::Sm1captctrlaOneshota {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sm1captctrlaOneshota::from_bits(val as u8)
    }
    #[doc = "One Shot Mode A"]
    #[inline(always)]
    pub const fn set_oneshota(&mut self, val: super::vals::Sm1captctrlaOneshota) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Edge A 0"]
    #[must_use]
    #[inline(always)]
    pub const fn edga0(&self) -> super::vals::Sm1captctrlaEdga0 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Sm1captctrlaEdga0::from_bits(val as u8)
    }
    #[doc = "Edge A 0"]
    #[inline(always)]
    pub const fn set_edga0(&mut self, val: super::vals::Sm1captctrlaEdga0) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Edge A 1"]
    #[must_use]
    #[inline(always)]
    pub const fn edga1(&self) -> super::vals::Sm1captctrlaEdga1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Sm1captctrlaEdga1::from_bits(val as u8)
    }
    #[doc = "Edge A 1"]
    #[inline(always)]
    pub const fn set_edga1(&mut self, val: super::vals::Sm1captctrlaEdga1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Input Select A"]
    #[must_use]
    #[inline(always)]
    pub const fn inp_sela(&self) -> super::vals::Sm1captctrlaInpSela {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Sm1captctrlaInpSela::from_bits(val as u8)
    }
    #[doc = "Input Select A"]
    #[inline(always)]
    pub const fn set_inp_sela(&mut self, val: super::vals::Sm1captctrlaInpSela) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Edge Counter A Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcnta_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Edge Counter A Enable"]
    #[inline(always)]
    pub const fn set_edgcnta_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture A FIFOs Water Mark"]
    #[must_use]
    #[inline(always)]
    pub const fn cfawm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Capture A FIFOs Water Mark"]
    #[inline(always)]
    pub const fn set_cfawm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Capture A0 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn ca0cnt(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Capture A0 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_ca0cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u16) & 0x07) << 10usize);
    }
    #[doc = "Capture A1 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn ca1cnt(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Capture A1 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_ca1cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u16) & 0x07) << 13usize);
    }
}
impl Default for Sm1captctrla {
    #[inline(always)]
    fn default() -> Sm1captctrla {
        Sm1captctrla(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm1captctrla {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1captctrla")
            .field("arma", &self.arma())
            .field("oneshota", &self.oneshota())
            .field("edga0", &self.edga0())
            .field("edga1", &self.edga1())
            .field("inp_sela", &self.inp_sela())
            .field("edgcnta_en", &self.edgcnta_en())
            .field("cfawm", &self.cfawm())
            .field("ca0cnt", &self.ca0cnt())
            .field("ca1cnt", &self.ca1cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1captctrla {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm1captctrla {
            arma: bool,
            oneshota: super::vals::Sm1captctrlaOneshota,
            edga0: super::vals::Sm1captctrlaEdga0,
            edga1: super::vals::Sm1captctrlaEdga1,
            inp_sela: super::vals::Sm1captctrlaInpSela,
            edgcnta_en: bool,
            cfawm: u8,
            ca0cnt: u8,
            ca1cnt: u8,
        }
        let proxy = Sm1captctrla {
            arma: self.arma(),
            oneshota: self.oneshota(),
            edga0: self.edga0(),
            edga1: self.edga1(),
            inp_sela: self.inp_sela(),
            edgcnta_en: self.edgcnta_en(),
            cfawm: self.cfawm(),
            ca0cnt: self.ca0cnt(),
            ca1cnt: self.ca1cnt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Control B Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1captctrlb(pub u16);
impl Sm1captctrlb {
    #[doc = "Arm B"]
    #[must_use]
    #[inline(always)]
    pub const fn armb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Arm B"]
    #[inline(always)]
    pub const fn set_armb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "One Shot Mode B"]
    #[must_use]
    #[inline(always)]
    pub const fn oneshotb(&self) -> super::vals::Sm1captctrlbOneshotb {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sm1captctrlbOneshotb::from_bits(val as u8)
    }
    #[doc = "One Shot Mode B"]
    #[inline(always)]
    pub const fn set_oneshotb(&mut self, val: super::vals::Sm1captctrlbOneshotb) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Edge B 0"]
    #[must_use]
    #[inline(always)]
    pub const fn edgb0(&self) -> super::vals::Sm1captctrlbEdgb0 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Sm1captctrlbEdgb0::from_bits(val as u8)
    }
    #[doc = "Edge B 0"]
    #[inline(always)]
    pub const fn set_edgb0(&mut self, val: super::vals::Sm1captctrlbEdgb0) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Edge B 1"]
    #[must_use]
    #[inline(always)]
    pub const fn edgb1(&self) -> super::vals::Sm1captctrlbEdgb1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Sm1captctrlbEdgb1::from_bits(val as u8)
    }
    #[doc = "Edge B 1"]
    #[inline(always)]
    pub const fn set_edgb1(&mut self, val: super::vals::Sm1captctrlbEdgb1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Input Select B"]
    #[must_use]
    #[inline(always)]
    pub const fn inp_selb(&self) -> super::vals::Sm1captctrlbInpSelb {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Sm1captctrlbInpSelb::from_bits(val as u8)
    }
    #[doc = "Input Select B"]
    #[inline(always)]
    pub const fn set_inp_selb(&mut self, val: super::vals::Sm1captctrlbInpSelb) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Edge Counter B Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntb_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Edge Counter B Enable"]
    #[inline(always)]
    pub const fn set_edgcntb_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture B FIFOs Water Mark"]
    #[must_use]
    #[inline(always)]
    pub const fn cfbwm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Capture B FIFOs Water Mark"]
    #[inline(always)]
    pub const fn set_cfbwm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Capture B0 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn cb0cnt(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Capture B0 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_cb0cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u16) & 0x07) << 10usize);
    }
    #[doc = "Capture B1 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn cb1cnt(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Capture B1 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_cb1cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u16) & 0x07) << 13usize);
    }
}
impl Default for Sm1captctrlb {
    #[inline(always)]
    fn default() -> Sm1captctrlb {
        Sm1captctrlb(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm1captctrlb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1captctrlb")
            .field("armb", &self.armb())
            .field("oneshotb", &self.oneshotb())
            .field("edgb0", &self.edgb0())
            .field("edgb1", &self.edgb1())
            .field("inp_selb", &self.inp_selb())
            .field("edgcntb_en", &self.edgcntb_en())
            .field("cfbwm", &self.cfbwm())
            .field("cb0cnt", &self.cb0cnt())
            .field("cb1cnt", &self.cb1cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1captctrlb {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm1captctrlb {
            armb: bool,
            oneshotb: super::vals::Sm1captctrlbOneshotb,
            edgb0: super::vals::Sm1captctrlbEdgb0,
            edgb1: super::vals::Sm1captctrlbEdgb1,
            inp_selb: super::vals::Sm1captctrlbInpSelb,
            edgcntb_en: bool,
            cfbwm: u8,
            cb0cnt: u8,
            cb1cnt: u8,
        }
        let proxy = Sm1captctrlb {
            armb: self.armb(),
            oneshotb: self.oneshotb(),
            edgb0: self.edgb0(),
            edgb1: self.edgb1(),
            inp_selb: self.inp_selb(),
            edgcntb_en: self.edgcntb_en(),
            cfbwm: self.cfbwm(),
            cb0cnt: self.cb0cnt(),
            cb1cnt: self.cb1cnt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Control X Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1captctrlx(pub u16);
impl Sm1captctrlx {
    #[doc = "Arm X"]
    #[must_use]
    #[inline(always)]
    pub const fn armx(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Arm X"]
    #[inline(always)]
    pub const fn set_armx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "One Shot Mode Aux"]
    #[must_use]
    #[inline(always)]
    pub const fn oneshotx(&self) -> super::vals::Sm1captctrlxOneshotx {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sm1captctrlxOneshotx::from_bits(val as u8)
    }
    #[doc = "One Shot Mode Aux"]
    #[inline(always)]
    pub const fn set_oneshotx(&mut self, val: super::vals::Sm1captctrlxOneshotx) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Edge X 0"]
    #[must_use]
    #[inline(always)]
    pub const fn edgx0(&self) -> super::vals::Sm1captctrlxEdgx0 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Sm1captctrlxEdgx0::from_bits(val as u8)
    }
    #[doc = "Edge X 0"]
    #[inline(always)]
    pub const fn set_edgx0(&mut self, val: super::vals::Sm1captctrlxEdgx0) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Edge X 1"]
    #[must_use]
    #[inline(always)]
    pub const fn edgx1(&self) -> super::vals::Sm1captctrlxEdgx1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Sm1captctrlxEdgx1::from_bits(val as u8)
    }
    #[doc = "Edge X 1"]
    #[inline(always)]
    pub const fn set_edgx1(&mut self, val: super::vals::Sm1captctrlxEdgx1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Input Select X"]
    #[must_use]
    #[inline(always)]
    pub const fn inp_selx(&self) -> super::vals::Sm1captctrlxInpSelx {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Sm1captctrlxInpSelx::from_bits(val as u8)
    }
    #[doc = "Input Select X"]
    #[inline(always)]
    pub const fn set_inp_selx(&mut self, val: super::vals::Sm1captctrlxInpSelx) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Edge Counter X Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntx_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Edge Counter X Enable"]
    #[inline(always)]
    pub const fn set_edgcntx_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture X FIFOs Water Mark"]
    #[must_use]
    #[inline(always)]
    pub const fn cfxwm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Capture X FIFOs Water Mark"]
    #[inline(always)]
    pub const fn set_cfxwm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Capture X0 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn cx0cnt(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Capture X0 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_cx0cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u16) & 0x07) << 10usize);
    }
    #[doc = "Capture X1 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn cx1cnt(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Capture X1 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_cx1cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u16) & 0x07) << 13usize);
    }
}
impl Default for Sm1captctrlx {
    #[inline(always)]
    fn default() -> Sm1captctrlx {
        Sm1captctrlx(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm1captctrlx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1captctrlx")
            .field("armx", &self.armx())
            .field("oneshotx", &self.oneshotx())
            .field("edgx0", &self.edgx0())
            .field("edgx1", &self.edgx1())
            .field("inp_selx", &self.inp_selx())
            .field("edgcntx_en", &self.edgcntx_en())
            .field("cfxwm", &self.cfxwm())
            .field("cx0cnt", &self.cx0cnt())
            .field("cx1cnt", &self.cx1cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1captctrlx {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm1captctrlx {
            armx: bool,
            oneshotx: super::vals::Sm1captctrlxOneshotx,
            edgx0: super::vals::Sm1captctrlxEdgx0,
            edgx1: super::vals::Sm1captctrlxEdgx1,
            inp_selx: super::vals::Sm1captctrlxInpSelx,
            edgcntx_en: bool,
            cfxwm: u8,
            cx0cnt: u8,
            cx1cnt: u8,
        }
        let proxy = Sm1captctrlx {
            armx: self.armx(),
            oneshotx: self.oneshotx(),
            edgx0: self.edgx0(),
            edgx1: self.edgx1(),
            inp_selx: self.inp_selx(),
            edgcntx_en: self.edgcntx_en(),
            cfxwm: self.cfxwm(),
            cx0cnt: self.cx0cnt(),
            cx1cnt: self.cx1cnt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture PWM_A Input Filter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1captfilta(pub u16);
impl Sm1captfilta {
    #[doc = "Input Capture Filter Period"]
    #[must_use]
    #[inline(always)]
    pub const fn capta_filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Capture Filter Period"]
    #[inline(always)]
    pub const fn set_capta_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Input Capture Filter Count"]
    #[must_use]
    #[inline(always)]
    pub const fn capta_filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Capture Filter Count"]
    #[inline(always)]
    pub const fn set_capta_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
}
impl Default for Sm1captfilta {
    #[inline(always)]
    fn default() -> Sm1captfilta {
        Sm1captfilta(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm1captfilta {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1captfilta")
            .field("capta_filt_per", &self.capta_filt_per())
            .field("capta_filt_cnt", &self.capta_filt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1captfilta {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm1captfilta {
            capta_filt_per: u8,
            capta_filt_cnt: u8,
        }
        let proxy = Sm1captfilta {
            capta_filt_per: self.capta_filt_per(),
            capta_filt_cnt: self.capta_filt_cnt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture PWM_B Input Filter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1captfiltb(pub u16);
impl Sm1captfiltb {
    #[doc = "Input Capture Filter Period"]
    #[must_use]
    #[inline(always)]
    pub const fn captb_filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Capture Filter Period"]
    #[inline(always)]
    pub const fn set_captb_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Input Capture Filter Count"]
    #[must_use]
    #[inline(always)]
    pub const fn captb_filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Capture Filter Count"]
    #[inline(always)]
    pub const fn set_captb_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
}
impl Default for Sm1captfiltb {
    #[inline(always)]
    fn default() -> Sm1captfiltb {
        Sm1captfiltb(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm1captfiltb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1captfiltb")
            .field("captb_filt_per", &self.captb_filt_per())
            .field("captb_filt_cnt", &self.captb_filt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1captfiltb {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm1captfiltb {
            captb_filt_per: u8,
            captb_filt_cnt: u8,
        }
        let proxy = Sm1captfiltb {
            captb_filt_per: self.captb_filt_per(),
            captb_filt_cnt: self.captb_filt_cnt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture PWM_X Input Filter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1captfiltx(pub u16);
impl Sm1captfiltx {
    #[doc = "Input Capture Filter Period"]
    #[must_use]
    #[inline(always)]
    pub const fn captx_filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Capture Filter Period"]
    #[inline(always)]
    pub const fn set_captx_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Input Capture Filter Count"]
    #[must_use]
    #[inline(always)]
    pub const fn captx_filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Capture Filter Count"]
    #[inline(always)]
    pub const fn set_captx_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
}
impl Default for Sm1captfiltx {
    #[inline(always)]
    fn default() -> Sm1captfiltx {
        Sm1captfiltx(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm1captfiltx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1captfiltx")
            .field("captx_filt_per", &self.captx_filt_per())
            .field("captx_filt_cnt", &self.captx_filt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1captfiltx {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm1captfiltx {
            captx_filt_per: u8,
            captx_filt_cnt: u8,
        }
        let proxy = Sm1captfiltx {
            captx_filt_per: self.captx_filt_per(),
            captx_filt_cnt: self.captx_filt_cnt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1ctrl(pub u16);
impl Sm1ctrl {
    #[doc = "Double Switching Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dblen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Double Switching Enable"]
    #[inline(always)]
    pub const fn set_dblen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "PWM_X Double Switching Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dblx(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Double Switching Enable"]
    #[inline(always)]
    pub const fn set_dblx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Load Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ldmod(&self) -> super::vals::Sm1ctrlLdmod {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Sm1ctrlLdmod::from_bits(val as u8)
    }
    #[doc = "Load Mode Select"]
    #[inline(always)]
    pub const fn set_ldmod(&mut self, val: super::vals::Sm1ctrlLdmod) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Split the DBLPWM signal to PWM_A and PWM_B"]
    #[must_use]
    #[inline(always)]
    pub const fn split(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Split the DBLPWM signal to PWM_A and PWM_B"]
    #[inline(always)]
    pub const fn set_split(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Prescaler"]
    #[must_use]
    #[inline(always)]
    pub const fn prsc(&self) -> super::vals::Sm1ctrlPrsc {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Sm1ctrlPrsc::from_bits(val as u8)
    }
    #[doc = "Prescaler"]
    #[inline(always)]
    pub const fn set_prsc(&mut self, val: super::vals::Sm1ctrlPrsc) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u16) & 0x07) << 4usize);
    }
    #[doc = "Compare Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn compmode(&self) -> super::vals::Sm1ctrlCompmode {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Sm1ctrlCompmode::from_bits(val as u8)
    }
    #[doc = "Compare Mode"]
    #[inline(always)]
    pub const fn set_compmode(&mut self, val: super::vals::Sm1ctrlCompmode) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "Deadtime"]
    #[must_use]
    #[inline(always)]
    pub const fn dt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Deadtime"]
    #[inline(always)]
    pub const fn set_dt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Full Cycle Reload"]
    #[must_use]
    #[inline(always)]
    pub const fn full(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Full Cycle Reload"]
    #[inline(always)]
    pub const fn set_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Half Cycle Reload"]
    #[must_use]
    #[inline(always)]
    pub const fn half(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Half Cycle Reload"]
    #[inline(always)]
    pub const fn set_half(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Load Frequency"]
    #[must_use]
    #[inline(always)]
    pub const fn ldfq(&self) -> super::vals::Sm1ctrlLdfq {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Sm1ctrlLdfq::from_bits(val as u8)
    }
    #[doc = "Load Frequency"]
    #[inline(always)]
    pub const fn set_ldfq(&mut self, val: super::vals::Sm1ctrlLdfq) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u16) & 0x0f) << 12usize);
    }
}
impl Default for Sm1ctrl {
    #[inline(always)]
    fn default() -> Sm1ctrl {
        Sm1ctrl(1024u64 as u16)
    }
}
impl core::fmt::Debug for Sm1ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1ctrl")
            .field("dblen", &self.dblen())
            .field("dblx", &self.dblx())
            .field("ldmod", &self.ldmod())
            .field("split", &self.split())
            .field("prsc", &self.prsc())
            .field("compmode", &self.compmode())
            .field("dt", &self.dt())
            .field("full", &self.full())
            .field("half", &self.half())
            .field("ldfq", &self.ldfq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1ctrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm1ctrl {
            dblen: bool,
            dblx: bool,
            ldmod: super::vals::Sm1ctrlLdmod,
            split: bool,
            prsc: super::vals::Sm1ctrlPrsc,
            compmode: super::vals::Sm1ctrlCompmode,
            dt: u8,
            full: bool,
            half: bool,
            ldfq: super::vals::Sm1ctrlLdfq,
        }
        let proxy = Sm1ctrl {
            dblen: self.dblen(),
            dblx: self.dblx(),
            ldmod: self.ldmod(),
            split: self.split(),
            prsc: self.prsc(),
            compmode: self.compmode(),
            dt: self.dt(),
            full: self.full(),
            half: self.half(),
            ldfq: self.ldfq(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Control 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1ctrl2(pub u16);
impl Sm1ctrl2 {
    #[doc = "Clock Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn clk_sel(&self) -> super::vals::Sm1ctrl2ClkSel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sm1ctrl2ClkSel::from_bits(val as u8)
    }
    #[doc = "Clock Source Select"]
    #[inline(always)]
    pub const fn set_clk_sel(&mut self, val: super::vals::Sm1ctrl2ClkSel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Reload Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn reload_sel(&self) -> super::vals::Sm1ctrl2ReloadSel {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Sm1ctrl2ReloadSel::from_bits(val as u8)
    }
    #[doc = "Reload Source Select"]
    #[inline(always)]
    pub const fn set_reload_sel(&mut self, val: super::vals::Sm1ctrl2ReloadSel) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Force Select"]
    #[must_use]
    #[inline(always)]
    pub const fn force_sel(&self) -> super::vals::Sm1ctrl2ForceSel {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::Sm1ctrl2ForceSel::from_bits(val as u8)
    }
    #[doc = "Force Select"]
    #[inline(always)]
    pub const fn set_force_sel(&mut self, val: super::vals::Sm1ctrl2ForceSel) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u16) & 0x07) << 3usize);
    }
    #[doc = "Force Initialization"]
    #[must_use]
    #[inline(always)]
    pub const fn force(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Force Initialization"]
    #[inline(always)]
    pub const fn set_force(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Force Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn frcen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Force Enable"]
    #[inline(always)]
    pub const fn set_frcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Initialization Control Select"]
    #[must_use]
    #[inline(always)]
    pub const fn init_sel(&self) -> super::vals::Sm1ctrl2InitSel {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Sm1ctrl2InitSel::from_bits(val as u8)
    }
    #[doc = "Initialization Control Select"]
    #[inline(always)]
    pub const fn set_init_sel(&mut self, val: super::vals::Sm1ctrl2InitSel) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "PWM_X Initial Value"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmx_init(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Initial Value"]
    #[inline(always)]
    pub const fn set_pwmx_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "PWM45 Initial Value"]
    #[must_use]
    #[inline(always)]
    pub const fn pwm45_init(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "PWM45 Initial Value"]
    #[inline(always)]
    pub const fn set_pwm45_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "PWM23 Initial Value"]
    #[must_use]
    #[inline(always)]
    pub const fn pwm23_init(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "PWM23 Initial Value"]
    #[inline(always)]
    pub const fn set_pwm23_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Independent or Complementary Pair Operation"]
    #[must_use]
    #[inline(always)]
    pub const fn indep(&self) -> super::vals::Sm1ctrl2Indep {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Sm1ctrl2Indep::from_bits(val as u8)
    }
    #[doc = "Independent or Complementary Pair Operation"]
    #[inline(always)]
    pub const fn set_indep(&mut self, val: super::vals::Sm1ctrl2Indep) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "Wait Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn waiten(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Wait Enable"]
    #[inline(always)]
    pub const fn set_waiten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "Debug Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dbgen(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Enable"]
    #[inline(always)]
    pub const fn set_dbgen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Sm1ctrl2 {
    #[inline(always)]
    fn default() -> Sm1ctrl2 {
        Sm1ctrl2(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm1ctrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1ctrl2")
            .field("clk_sel", &self.clk_sel())
            .field("reload_sel", &self.reload_sel())
            .field("force_sel", &self.force_sel())
            .field("force", &self.force())
            .field("frcen", &self.frcen())
            .field("init_sel", &self.init_sel())
            .field("pwmx_init", &self.pwmx_init())
            .field("pwm45_init", &self.pwm45_init())
            .field("pwm23_init", &self.pwm23_init())
            .field("indep", &self.indep())
            .field("waiten", &self.waiten())
            .field("dbgen", &self.dbgen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1ctrl2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm1ctrl2 {
            clk_sel: super::vals::Sm1ctrl2ClkSel,
            reload_sel: super::vals::Sm1ctrl2ReloadSel,
            force_sel: super::vals::Sm1ctrl2ForceSel,
            force: bool,
            frcen: bool,
            init_sel: super::vals::Sm1ctrl2InitSel,
            pwmx_init: bool,
            pwm45_init: bool,
            pwm23_init: bool,
            indep: super::vals::Sm1ctrl2Indep,
            waiten: bool,
            dbgen: bool,
        }
        let proxy = Sm1ctrl2 {
            clk_sel: self.clk_sel(),
            reload_sel: self.reload_sel(),
            force_sel: self.force_sel(),
            force: self.force(),
            frcen: self.frcen(),
            init_sel: self.init_sel(),
            pwmx_init: self.pwmx_init(),
            pwm45_init: self.pwm45_init(),
            pwm23_init: self.pwm23_init(),
            indep: self.indep(),
            waiten: self.waiten(),
            dbgen: self.dbgen(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Value 0 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1cval0cyc(pub u16);
impl Sm1cval0cyc {
    #[doc = "Capture Value 0 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval0cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 0 Cycle"]
    #[inline(always)]
    pub const fn set_cval0cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm1cval0cyc {
    #[inline(always)]
    fn default() -> Sm1cval0cyc {
        Sm1cval0cyc(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm1cval0cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1cval0cyc")
            .field("cval0cyc", &self.cval0cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1cval0cyc {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm1cval0cyc {
            cval0cyc: u8,
        }
        let proxy = Sm1cval0cyc {
            cval0cyc: self.cval0cyc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Value 1 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1cval1cyc(pub u16);
impl Sm1cval1cyc {
    #[doc = "Capture Value 1 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval1cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 1 Cycle"]
    #[inline(always)]
    pub const fn set_cval1cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm1cval1cyc {
    #[inline(always)]
    fn default() -> Sm1cval1cyc {
        Sm1cval1cyc(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm1cval1cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1cval1cyc")
            .field("cval1cyc", &self.cval1cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1cval1cyc {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm1cval1cyc {
            cval1cyc: u8,
        }
        let proxy = Sm1cval1cyc {
            cval1cyc: self.cval1cyc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Value 2 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1cval2cyc(pub u16);
impl Sm1cval2cyc {
    #[doc = "Capture Value 2 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval2cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 2 Cycle"]
    #[inline(always)]
    pub const fn set_cval2cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm1cval2cyc {
    #[inline(always)]
    fn default() -> Sm1cval2cyc {
        Sm1cval2cyc(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm1cval2cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1cval2cyc")
            .field("cval2cyc", &self.cval2cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1cval2cyc {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm1cval2cyc {
            cval2cyc: u8,
        }
        let proxy = Sm1cval2cyc {
            cval2cyc: self.cval2cyc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Value 3 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1cval3cyc(pub u16);
impl Sm1cval3cyc {
    #[doc = "Capture Value 3 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval3cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 3 Cycle"]
    #[inline(always)]
    pub const fn set_cval3cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm1cval3cyc {
    #[inline(always)]
    fn default() -> Sm1cval3cyc {
        Sm1cval3cyc(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm1cval3cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1cval3cyc")
            .field("cval3cyc", &self.cval3cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1cval3cyc {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm1cval3cyc {
            cval3cyc: u8,
        }
        let proxy = Sm1cval3cyc {
            cval3cyc: self.cval3cyc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Value 4 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1cval4cyc(pub u16);
impl Sm1cval4cyc {
    #[doc = "Capture Value 4 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval4cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 4 Cycle"]
    #[inline(always)]
    pub const fn set_cval4cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm1cval4cyc {
    #[inline(always)]
    fn default() -> Sm1cval4cyc {
        Sm1cval4cyc(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm1cval4cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1cval4cyc")
            .field("cval4cyc", &self.cval4cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1cval4cyc {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm1cval4cyc {
            cval4cyc: u8,
        }
        let proxy = Sm1cval4cyc {
            cval4cyc: self.cval4cyc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Value 5 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1cval5cyc(pub u16);
impl Sm1cval5cyc {
    #[doc = "Capture Value 5 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval5cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 5 Cycle"]
    #[inline(always)]
    pub const fn set_cval5cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm1cval5cyc {
    #[inline(always)]
    fn default() -> Sm1cval5cyc {
        Sm1cval5cyc(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm1cval5cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1cval5cyc")
            .field("cval5cyc", &self.cval5cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1cval5cyc {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm1cval5cyc {
            cval5cyc: u8,
        }
        let proxy = Sm1cval5cyc {
            cval5cyc: self.cval5cyc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fault Disable Mapping Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1dismap0(pub u16);
impl Sm1dismap0 {
    #[doc = "PWM_A Fault Disable Mask 0"]
    #[must_use]
    #[inline(always)]
    pub const fn dis0a(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_A Fault Disable Mask 0"]
    #[inline(always)]
    pub const fn set_dis0a(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "PWM_B Fault Disable Mask 0"]
    #[must_use]
    #[inline(always)]
    pub const fn dis0b(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_B Fault Disable Mask 0"]
    #[inline(always)]
    pub const fn set_dis0b(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
    }
    #[doc = "PWM_X Fault Disable Mask 0"]
    #[must_use]
    #[inline(always)]
    pub const fn dis0x(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_X Fault Disable Mask 0"]
    #[inline(always)]
    pub const fn set_dis0x(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
}
impl Default for Sm1dismap0 {
    #[inline(always)]
    fn default() -> Sm1dismap0 {
        Sm1dismap0(65535u64 as u16)
    }
}
impl core::fmt::Debug for Sm1dismap0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1dismap0")
            .field("dis0a", &self.dis0a())
            .field("dis0b", &self.dis0b())
            .field("dis0x", &self.dis0x())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1dismap0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm1dismap0 {
            dis0a: u8,
            dis0b: u8,
            dis0x: u8,
        }
        let proxy = Sm1dismap0 {
            dis0a: self.dis0a(),
            dis0b: self.dis0b(),
            dis0x: self.dis0x(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "DMA Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1dmaen(pub u16);
impl Sm1dmaen {
    #[doc = "Capture X0 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cx0de(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X0 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_cx0de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Capture X1 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cx1de(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X1 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_cx1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Capture B0 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cb0de(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Capture B0 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_cb0de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Capture B1 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cb1de(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Capture B1 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_cb1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Capture A0 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ca0de(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Capture A0 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_ca0de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Capture A1 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ca1de(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Capture A1 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_ca1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Capture DMA Enable Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn captde(&self) -> super::vals::Sm1dmaenCaptde {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Sm1dmaenCaptde::from_bits(val as u8)
    }
    #[doc = "Capture DMA Enable Source Select"]
    #[inline(always)]
    pub const fn set_captde(&mut self, val: super::vals::Sm1dmaenCaptde) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "FIFO Watermark AND Control"]
    #[must_use]
    #[inline(always)]
    pub const fn fand(&self) -> super::vals::Sm1dmaenFand {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Sm1dmaenFand::from_bits(val as u8)
    }
    #[doc = "FIFO Watermark AND Control"]
    #[inline(always)]
    pub const fn set_fand(&mut self, val: super::vals::Sm1dmaenFand) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "Value Registers DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn valde(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Value Registers DMA Enable"]
    #[inline(always)]
    pub const fn set_valde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
}
impl Default for Sm1dmaen {
    #[inline(always)]
    fn default() -> Sm1dmaen {
        Sm1dmaen(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm1dmaen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1dmaen")
            .field("cx0de", &self.cx0de())
            .field("cx1de", &self.cx1de())
            .field("cb0de", &self.cb0de())
            .field("cb1de", &self.cb1de())
            .field("ca0de", &self.ca0de())
            .field("ca1de", &self.ca1de())
            .field("captde", &self.captde())
            .field("fand", &self.fand())
            .field("valde", &self.valde())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1dmaen {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm1dmaen {
            cx0de: bool,
            cx1de: bool,
            cb0de: bool,
            cb1de: bool,
            ca0de: bool,
            ca1de: bool,
            captde: super::vals::Sm1dmaenCaptde,
            fand: super::vals::Sm1dmaenFand,
            valde: bool,
        }
        let proxy = Sm1dmaen {
            cx0de: self.cx0de(),
            cx1de: self.cx1de(),
            cb0de: self.cb0de(),
            cb1de: self.cb1de(),
            ca0de: self.ca0de(),
            ca1de: self.ca1de(),
            captde: self.captde(),
            fand: self.fand(),
            valde: self.valde(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fractional Value Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1fracval1(pub u16);
impl Sm1fracval1 {
    #[doc = "Fractional Value 1"]
    #[must_use]
    #[inline(always)]
    pub const fn fracval1(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 1"]
    #[inline(always)]
    pub const fn set_fracval1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Sm1fracval1 {
    #[inline(always)]
    fn default() -> Sm1fracval1 {
        Sm1fracval1(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm1fracval1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1fracval1")
            .field("fracval1", &self.fracval1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1fracval1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm1fracval1 {
            fracval1: u8,
        }
        let proxy = Sm1fracval1 {
            fracval1: self.fracval1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fractional Value Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1fracval2(pub u16);
impl Sm1fracval2 {
    #[doc = "Fractional Value 2"]
    #[must_use]
    #[inline(always)]
    pub const fn fracval2(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 2"]
    #[inline(always)]
    pub const fn set_fracval2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Sm1fracval2 {
    #[inline(always)]
    fn default() -> Sm1fracval2 {
        Sm1fracval2(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm1fracval2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1fracval2")
            .field("fracval2", &self.fracval2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1fracval2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm1fracval2 {
            fracval2: u8,
        }
        let proxy = Sm1fracval2 {
            fracval2: self.fracval2(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fractional Value Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1fracval3(pub u16);
impl Sm1fracval3 {
    #[doc = "Fractional Value 3"]
    #[must_use]
    #[inline(always)]
    pub const fn fracval3(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 3"]
    #[inline(always)]
    pub const fn set_fracval3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Sm1fracval3 {
    #[inline(always)]
    fn default() -> Sm1fracval3 {
        Sm1fracval3(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm1fracval3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1fracval3")
            .field("fracval3", &self.fracval3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1fracval3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm1fracval3 {
            fracval3: u8,
        }
        let proxy = Sm1fracval3 {
            fracval3: self.fracval3(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fractional Value Register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1fracval4(pub u16);
impl Sm1fracval4 {
    #[doc = "Fractional Value 4"]
    #[must_use]
    #[inline(always)]
    pub const fn fracval4(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 4"]
    #[inline(always)]
    pub const fn set_fracval4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Sm1fracval4 {
    #[inline(always)]
    fn default() -> Sm1fracval4 {
        Sm1fracval4(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm1fracval4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1fracval4")
            .field("fracval4", &self.fracval4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1fracval4 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm1fracval4 {
            fracval4: u8,
        }
        let proxy = Sm1fracval4 {
            fracval4: self.fracval4(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fractional Value Register 5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1fracval5(pub u16);
impl Sm1fracval5 {
    #[doc = "Fractional Value 5"]
    #[must_use]
    #[inline(always)]
    pub const fn fracval5(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 5"]
    #[inline(always)]
    pub const fn set_fracval5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Sm1fracval5 {
    #[inline(always)]
    fn default() -> Sm1fracval5 {
        Sm1fracval5(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm1fracval5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1fracval5")
            .field("fracval5", &self.fracval5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1fracval5 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm1fracval5 {
            fracval5: u8,
        }
        let proxy = Sm1fracval5 {
            fracval5: self.fracval5(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fractional Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1frctrl(pub u16);
impl Sm1frctrl {
    #[doc = "Fractional Cycle PWM Period Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn frac1_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Fractional Cycle PWM Period Enable"]
    #[inline(always)]
    pub const fn set_frac1_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Fractional Cycle Placement Enable for PWM_A"]
    #[must_use]
    #[inline(always)]
    pub const fn frac23_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Fractional Cycle Placement Enable for PWM_A"]
    #[inline(always)]
    pub const fn set_frac23_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Fractional Cycle Placement Enable for PWM_B"]
    #[must_use]
    #[inline(always)]
    pub const fn frac45_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Fractional Cycle Placement Enable for PWM_B"]
    #[inline(always)]
    pub const fn set_frac45_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Test Status Bit"]
    #[must_use]
    #[inline(always)]
    pub const fn test(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Test Status Bit"]
    #[inline(always)]
    pub const fn set_test(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Sm1frctrl {
    #[inline(always)]
    fn default() -> Sm1frctrl {
        Sm1frctrl(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm1frctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1frctrl")
            .field("frac1_en", &self.frac1_en())
            .field("frac23_en", &self.frac23_en())
            .field("frac45_en", &self.frac45_en())
            .field("test", &self.test())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1frctrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm1frctrl {
            frac1_en: bool,
            frac23_en: bool,
            frac45_en: bool,
            test: bool,
        }
        let proxy = Sm1frctrl {
            frac1_en: self.frac1_en(),
            frac23_en: self.frac23_en(),
            frac45_en: self.frac45_en(),
            test: self.test(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1inten(pub u16);
impl Sm1inten {
    #[doc = "Compare Interrupt Enables"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpie(&self) -> super::vals::Sm1intenCmpie {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Sm1intenCmpie::from_bits(val as u8)
    }
    #[doc = "Compare Interrupt Enables"]
    #[inline(always)]
    pub const fn set_cmpie(&mut self, val: super::vals::Sm1intenCmpie) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Capture X 0 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cx0ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X 0 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cx0ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Capture X 1 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cx1ie(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cx1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture B 0 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cb0ie(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Capture B 0 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cb0ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "Capture B 1 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cb1ie(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Capture B 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cb1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "Capture A 0 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ca0ie(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Capture A 0 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ca0ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Capture A 1 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ca1ie(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Capture A 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ca1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Reload Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Interrupt Enable"]
    #[inline(always)]
    pub const fn set_rie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Reload Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn reie(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_reie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
}
impl Default for Sm1inten {
    #[inline(always)]
    fn default() -> Sm1inten {
        Sm1inten(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm1inten {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1inten")
            .field("cmpie", &self.cmpie())
            .field("cx0ie", &self.cx0ie())
            .field("cx1ie", &self.cx1ie())
            .field("cb0ie", &self.cb0ie())
            .field("cb1ie", &self.cb1ie())
            .field("ca0ie", &self.ca0ie())
            .field("ca1ie", &self.ca1ie())
            .field("rie", &self.rie())
            .field("reie", &self.reie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1inten {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm1inten {
            cmpie: super::vals::Sm1intenCmpie,
            cx0ie: bool,
            cx1ie: bool,
            cb0ie: bool,
            cb1ie: bool,
            ca0ie: bool,
            ca1ie: bool,
            rie: bool,
            reie: bool,
        }
        let proxy = Sm1inten {
            cmpie: self.cmpie(),
            cx0ie: self.cx0ie(),
            cx1ie: self.cx1ie(),
            cb0ie: self.cb0ie(),
            cb1ie: self.cb1ie(),
            ca0ie: self.ca0ie(),
            ca1ie: self.ca1ie(),
            rie: self.rie(),
            reie: self.reie(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Output Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1octrl(pub u16);
impl Sm1octrl {
    #[doc = "PWM_X Fault State"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmxfs(&self) -> super::vals::Sm1octrlPwmxfs {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sm1octrlPwmxfs::from_bits(val as u8)
    }
    #[doc = "PWM_X Fault State"]
    #[inline(always)]
    pub const fn set_pwmxfs(&mut self, val: super::vals::Sm1octrlPwmxfs) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "PWM_B Fault State"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmbfs(&self) -> super::vals::Sm1octrlPwmbfs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Sm1octrlPwmbfs::from_bits(val as u8)
    }
    #[doc = "PWM_B Fault State"]
    #[inline(always)]
    pub const fn set_pwmbfs(&mut self, val: super::vals::Sm1octrlPwmbfs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "PWM_A Fault State"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmafs(&self) -> super::vals::Sm1octrlPwmafs {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Sm1octrlPwmafs::from_bits(val as u8)
    }
    #[doc = "PWM_A Fault State"]
    #[inline(always)]
    pub const fn set_pwmafs(&mut self, val: super::vals::Sm1octrlPwmafs) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "PWM_X Output Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn polx(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Output Polarity"]
    #[inline(always)]
    pub const fn set_polx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "PWM_B Output Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn polb(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_B Output Polarity"]
    #[inline(always)]
    pub const fn set_polb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "PWM_A Output Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn pola(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_A Output Polarity"]
    #[inline(always)]
    pub const fn set_pola(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "PWM_X Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmx_in(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Input"]
    #[inline(always)]
    pub const fn set_pwmx_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "PWM_B Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmb_in(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_B Input"]
    #[inline(always)]
    pub const fn set_pwmb_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "PWM_A Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pwma_in(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_A Input"]
    #[inline(always)]
    pub const fn set_pwma_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Sm1octrl {
    #[inline(always)]
    fn default() -> Sm1octrl {
        Sm1octrl(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm1octrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1octrl")
            .field("pwmxfs", &self.pwmxfs())
            .field("pwmbfs", &self.pwmbfs())
            .field("pwmafs", &self.pwmafs())
            .field("polx", &self.polx())
            .field("polb", &self.polb())
            .field("pola", &self.pola())
            .field("pwmx_in", &self.pwmx_in())
            .field("pwmb_in", &self.pwmb_in())
            .field("pwma_in", &self.pwma_in())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1octrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm1octrl {
            pwmxfs: super::vals::Sm1octrlPwmxfs,
            pwmbfs: super::vals::Sm1octrlPwmbfs,
            pwmafs: super::vals::Sm1octrlPwmafs,
            polx: bool,
            polb: bool,
            pola: bool,
            pwmx_in: bool,
            pwmb_in: bool,
            pwma_in: bool,
        }
        let proxy = Sm1octrl {
            pwmxfs: self.pwmxfs(),
            pwmbfs: self.pwmbfs(),
            pwmafs: self.pwmafs(),
            polx: self.polx(),
            polb: self.polb(),
            pola: self.pola(),
            pwmx_in: self.pwmx_in(),
            pwmb_in: self.pwmb_in(),
            pwma_in: self.pwma_in(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1sts(pub u16);
impl Sm1sts {
    #[doc = "Compare Flags"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpf(&self) -> super::vals::Sm1stsCmpf {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Sm1stsCmpf::from_bits(val as u8)
    }
    #[doc = "Compare Flags"]
    #[inline(always)]
    pub const fn set_cmpf(&mut self, val: super::vals::Sm1stsCmpf) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Capture Flag X0"]
    #[must_use]
    #[inline(always)]
    pub const fn cfx0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag X0"]
    #[inline(always)]
    pub const fn set_cfx0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Capture Flag X1"]
    #[must_use]
    #[inline(always)]
    pub const fn cfx1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag X1"]
    #[inline(always)]
    pub const fn set_cfx1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture Flag B0"]
    #[must_use]
    #[inline(always)]
    pub const fn cfb0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag B0"]
    #[inline(always)]
    pub const fn set_cfb0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "Capture Flag B1"]
    #[must_use]
    #[inline(always)]
    pub const fn cfb1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag B1"]
    #[inline(always)]
    pub const fn set_cfb1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "Capture Flag A0"]
    #[must_use]
    #[inline(always)]
    pub const fn cfa0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag A0"]
    #[inline(always)]
    pub const fn set_cfa0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Capture Flag A1"]
    #[must_use]
    #[inline(always)]
    pub const fn cfa1(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag A1"]
    #[inline(always)]
    pub const fn set_cfa1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Reload Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rf(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Flag"]
    #[inline(always)]
    pub const fn set_rf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Reload Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ref_(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Error Flag"]
    #[inline(always)]
    pub const fn set_ref_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Registers Updated Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ruf(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Registers Updated Flag"]
    #[inline(always)]
    pub const fn set_ruf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
}
impl Default for Sm1sts {
    #[inline(always)]
    fn default() -> Sm1sts {
        Sm1sts(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm1sts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1sts")
            .field("cmpf", &self.cmpf())
            .field("cfx0", &self.cfx0())
            .field("cfx1", &self.cfx1())
            .field("cfb0", &self.cfb0())
            .field("cfb1", &self.cfb1())
            .field("cfa0", &self.cfa0())
            .field("cfa1", &self.cfa1())
            .field("rf", &self.rf())
            .field("ref_", &self.ref_())
            .field("ruf", &self.ruf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1sts {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm1sts {
            cmpf: super::vals::Sm1stsCmpf,
            cfx0: bool,
            cfx1: bool,
            cfb0: bool,
            cfb1: bool,
            cfa0: bool,
            cfa1: bool,
            rf: bool,
            ref_: bool,
            ruf: bool,
        }
        let proxy = Sm1sts {
            cmpf: self.cmpf(),
            cfx0: self.cfx0(),
            cfx1: self.cfx1(),
            cfb0: self.cfb0(),
            cfb1: self.cfb1(),
            cfa0: self.cfa0(),
            cfa1: self.cfa1(),
            rf: self.rf(),
            ref_: self.ref_(),
            ruf: self.ruf(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Output Trigger Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1tctrl(pub u16);
impl Sm1tctrl {
    #[doc = "Output Trigger Enables"]
    #[must_use]
    #[inline(always)]
    pub const fn out_trig_en(&self) -> super::vals::Sm1tctrlOutTrigEn {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Sm1tctrlOutTrigEn::from_bits(val as u8)
    }
    #[doc = "Output Trigger Enables"]
    #[inline(always)]
    pub const fn set_out_trig_en(&mut self, val: super::vals::Sm1tctrlOutTrigEn) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Trigger Frequency"]
    #[must_use]
    #[inline(always)]
    pub const fn trgfrq(&self) -> super::vals::Sm1tctrlTrgfrq {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Sm1tctrlTrgfrq::from_bits(val as u8)
    }
    #[doc = "Trigger Frequency"]
    #[inline(always)]
    pub const fn set_trgfrq(&mut self, val: super::vals::Sm1tctrlTrgfrq) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
    #[doc = "Mux Output Trigger 1 Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn pwbot1(&self) -> super::vals::Sm1tctrlPwbot1 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Sm1tctrlPwbot1::from_bits(val as u8)
    }
    #[doc = "Mux Output Trigger 1 Source Select"]
    #[inline(always)]
    pub const fn set_pwbot1(&mut self, val: super::vals::Sm1tctrlPwbot1) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "Mux Output Trigger 0 Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn pwaot0(&self) -> super::vals::Sm1tctrlPwaot0 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Sm1tctrlPwaot0::from_bits(val as u8)
    }
    #[doc = "Mux Output Trigger 0 Source Select"]
    #[inline(always)]
    pub const fn set_pwaot0(&mut self, val: super::vals::Sm1tctrlPwaot0) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
    }
}
impl Default for Sm1tctrl {
    #[inline(always)]
    fn default() -> Sm1tctrl {
        Sm1tctrl(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm1tctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1tctrl")
            .field("out_trig_en", &self.out_trig_en())
            .field("trgfrq", &self.trgfrq())
            .field("pwbot1", &self.pwbot1())
            .field("pwaot0", &self.pwaot0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1tctrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm1tctrl {
            out_trig_en: super::vals::Sm1tctrlOutTrigEn,
            trgfrq: super::vals::Sm1tctrlTrgfrq,
            pwbot1: super::vals::Sm1tctrlPwbot1,
            pwaot0: super::vals::Sm1tctrlPwaot0,
        }
        let proxy = Sm1tctrl {
            out_trig_en: self.out_trig_en(),
            trgfrq: self.trgfrq(),
            pwbot1: self.pwbot1(),
            pwaot0: self.pwaot0(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Compare A Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2captcompa(pub u16);
impl Sm2captcompa {
    #[doc = "Edge Compare A"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcmpa(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Compare A"]
    #[inline(always)]
    pub const fn set_edgcmpa(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Edge Counter A"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcnta(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Counter A"]
    #[inline(always)]
    pub const fn set_edgcnta(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sm2captcompa {
    #[inline(always)]
    fn default() -> Sm2captcompa {
        Sm2captcompa(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm2captcompa {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2captcompa")
            .field("edgcmpa", &self.edgcmpa())
            .field("edgcnta", &self.edgcnta())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2captcompa {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm2captcompa {
            edgcmpa: u8,
            edgcnta: u8,
        }
        let proxy = Sm2captcompa {
            edgcmpa: self.edgcmpa(),
            edgcnta: self.edgcnta(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Compare B Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2captcompb(pub u16);
impl Sm2captcompb {
    #[doc = "Edge Compare B"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcmpb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Compare B"]
    #[inline(always)]
    pub const fn set_edgcmpb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Edge Counter B"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntb(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Counter B"]
    #[inline(always)]
    pub const fn set_edgcntb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sm2captcompb {
    #[inline(always)]
    fn default() -> Sm2captcompb {
        Sm2captcompb(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm2captcompb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2captcompb")
            .field("edgcmpb", &self.edgcmpb())
            .field("edgcntb", &self.edgcntb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2captcompb {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm2captcompb {
            edgcmpb: u8,
            edgcntb: u8,
        }
        let proxy = Sm2captcompb {
            edgcmpb: self.edgcmpb(),
            edgcntb: self.edgcntb(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Compare X Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2captcompx(pub u16);
impl Sm2captcompx {
    #[doc = "Edge Compare X"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcmpx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Compare X"]
    #[inline(always)]
    pub const fn set_edgcmpx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Edge Counter X"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntx(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Counter X"]
    #[inline(always)]
    pub const fn set_edgcntx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sm2captcompx {
    #[inline(always)]
    fn default() -> Sm2captcompx {
        Sm2captcompx(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm2captcompx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2captcompx")
            .field("edgcmpx", &self.edgcmpx())
            .field("edgcntx", &self.edgcntx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2captcompx {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm2captcompx {
            edgcmpx: u8,
            edgcntx: u8,
        }
        let proxy = Sm2captcompx {
            edgcmpx: self.edgcmpx(),
            edgcntx: self.edgcntx(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Control A Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2captctrla(pub u16);
impl Sm2captctrla {
    #[doc = "Arm A"]
    #[must_use]
    #[inline(always)]
    pub const fn arma(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Arm A"]
    #[inline(always)]
    pub const fn set_arma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "One Shot Mode A"]
    #[must_use]
    #[inline(always)]
    pub const fn oneshota(&self) -> super::vals::Sm2captctrlaOneshota {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sm2captctrlaOneshota::from_bits(val as u8)
    }
    #[doc = "One Shot Mode A"]
    #[inline(always)]
    pub const fn set_oneshota(&mut self, val: super::vals::Sm2captctrlaOneshota) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Edge A 0"]
    #[must_use]
    #[inline(always)]
    pub const fn edga0(&self) -> super::vals::Sm2captctrlaEdga0 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Sm2captctrlaEdga0::from_bits(val as u8)
    }
    #[doc = "Edge A 0"]
    #[inline(always)]
    pub const fn set_edga0(&mut self, val: super::vals::Sm2captctrlaEdga0) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Edge A 1"]
    #[must_use]
    #[inline(always)]
    pub const fn edga1(&self) -> super::vals::Sm2captctrlaEdga1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Sm2captctrlaEdga1::from_bits(val as u8)
    }
    #[doc = "Edge A 1"]
    #[inline(always)]
    pub const fn set_edga1(&mut self, val: super::vals::Sm2captctrlaEdga1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Input Select A"]
    #[must_use]
    #[inline(always)]
    pub const fn inp_sela(&self) -> super::vals::Sm2captctrlaInpSela {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Sm2captctrlaInpSela::from_bits(val as u8)
    }
    #[doc = "Input Select A"]
    #[inline(always)]
    pub const fn set_inp_sela(&mut self, val: super::vals::Sm2captctrlaInpSela) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Edge Counter A Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcnta_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Edge Counter A Enable"]
    #[inline(always)]
    pub const fn set_edgcnta_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture A FIFOs Water Mark"]
    #[must_use]
    #[inline(always)]
    pub const fn cfawm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Capture A FIFOs Water Mark"]
    #[inline(always)]
    pub const fn set_cfawm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Capture A0 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn ca0cnt(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Capture A0 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_ca0cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u16) & 0x07) << 10usize);
    }
    #[doc = "Capture A1 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn ca1cnt(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Capture A1 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_ca1cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u16) & 0x07) << 13usize);
    }
}
impl Default for Sm2captctrla {
    #[inline(always)]
    fn default() -> Sm2captctrla {
        Sm2captctrla(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm2captctrla {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2captctrla")
            .field("arma", &self.arma())
            .field("oneshota", &self.oneshota())
            .field("edga0", &self.edga0())
            .field("edga1", &self.edga1())
            .field("inp_sela", &self.inp_sela())
            .field("edgcnta_en", &self.edgcnta_en())
            .field("cfawm", &self.cfawm())
            .field("ca0cnt", &self.ca0cnt())
            .field("ca1cnt", &self.ca1cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2captctrla {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm2captctrla {
            arma: bool,
            oneshota: super::vals::Sm2captctrlaOneshota,
            edga0: super::vals::Sm2captctrlaEdga0,
            edga1: super::vals::Sm2captctrlaEdga1,
            inp_sela: super::vals::Sm2captctrlaInpSela,
            edgcnta_en: bool,
            cfawm: u8,
            ca0cnt: u8,
            ca1cnt: u8,
        }
        let proxy = Sm2captctrla {
            arma: self.arma(),
            oneshota: self.oneshota(),
            edga0: self.edga0(),
            edga1: self.edga1(),
            inp_sela: self.inp_sela(),
            edgcnta_en: self.edgcnta_en(),
            cfawm: self.cfawm(),
            ca0cnt: self.ca0cnt(),
            ca1cnt: self.ca1cnt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Control B Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2captctrlb(pub u16);
impl Sm2captctrlb {
    #[doc = "Arm B"]
    #[must_use]
    #[inline(always)]
    pub const fn armb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Arm B"]
    #[inline(always)]
    pub const fn set_armb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "One Shot Mode B"]
    #[must_use]
    #[inline(always)]
    pub const fn oneshotb(&self) -> super::vals::Sm2captctrlbOneshotb {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sm2captctrlbOneshotb::from_bits(val as u8)
    }
    #[doc = "One Shot Mode B"]
    #[inline(always)]
    pub const fn set_oneshotb(&mut self, val: super::vals::Sm2captctrlbOneshotb) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Edge B 0"]
    #[must_use]
    #[inline(always)]
    pub const fn edgb0(&self) -> super::vals::Sm2captctrlbEdgb0 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Sm2captctrlbEdgb0::from_bits(val as u8)
    }
    #[doc = "Edge B 0"]
    #[inline(always)]
    pub const fn set_edgb0(&mut self, val: super::vals::Sm2captctrlbEdgb0) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Edge B 1"]
    #[must_use]
    #[inline(always)]
    pub const fn edgb1(&self) -> super::vals::Sm2captctrlbEdgb1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Sm2captctrlbEdgb1::from_bits(val as u8)
    }
    #[doc = "Edge B 1"]
    #[inline(always)]
    pub const fn set_edgb1(&mut self, val: super::vals::Sm2captctrlbEdgb1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Input Select B"]
    #[must_use]
    #[inline(always)]
    pub const fn inp_selb(&self) -> super::vals::Sm2captctrlbInpSelb {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Sm2captctrlbInpSelb::from_bits(val as u8)
    }
    #[doc = "Input Select B"]
    #[inline(always)]
    pub const fn set_inp_selb(&mut self, val: super::vals::Sm2captctrlbInpSelb) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Edge Counter B Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntb_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Edge Counter B Enable"]
    #[inline(always)]
    pub const fn set_edgcntb_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture B FIFOs Water Mark"]
    #[must_use]
    #[inline(always)]
    pub const fn cfbwm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Capture B FIFOs Water Mark"]
    #[inline(always)]
    pub const fn set_cfbwm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Capture B0 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn cb0cnt(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Capture B0 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_cb0cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u16) & 0x07) << 10usize);
    }
    #[doc = "Capture B1 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn cb1cnt(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Capture B1 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_cb1cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u16) & 0x07) << 13usize);
    }
}
impl Default for Sm2captctrlb {
    #[inline(always)]
    fn default() -> Sm2captctrlb {
        Sm2captctrlb(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm2captctrlb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2captctrlb")
            .field("armb", &self.armb())
            .field("oneshotb", &self.oneshotb())
            .field("edgb0", &self.edgb0())
            .field("edgb1", &self.edgb1())
            .field("inp_selb", &self.inp_selb())
            .field("edgcntb_en", &self.edgcntb_en())
            .field("cfbwm", &self.cfbwm())
            .field("cb0cnt", &self.cb0cnt())
            .field("cb1cnt", &self.cb1cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2captctrlb {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm2captctrlb {
            armb: bool,
            oneshotb: super::vals::Sm2captctrlbOneshotb,
            edgb0: super::vals::Sm2captctrlbEdgb0,
            edgb1: super::vals::Sm2captctrlbEdgb1,
            inp_selb: super::vals::Sm2captctrlbInpSelb,
            edgcntb_en: bool,
            cfbwm: u8,
            cb0cnt: u8,
            cb1cnt: u8,
        }
        let proxy = Sm2captctrlb {
            armb: self.armb(),
            oneshotb: self.oneshotb(),
            edgb0: self.edgb0(),
            edgb1: self.edgb1(),
            inp_selb: self.inp_selb(),
            edgcntb_en: self.edgcntb_en(),
            cfbwm: self.cfbwm(),
            cb0cnt: self.cb0cnt(),
            cb1cnt: self.cb1cnt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Control X Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2captctrlx(pub u16);
impl Sm2captctrlx {
    #[doc = "Arm X"]
    #[must_use]
    #[inline(always)]
    pub const fn armx(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Arm X"]
    #[inline(always)]
    pub const fn set_armx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "One Shot Mode Aux"]
    #[must_use]
    #[inline(always)]
    pub const fn oneshotx(&self) -> super::vals::Sm2captctrlxOneshotx {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sm2captctrlxOneshotx::from_bits(val as u8)
    }
    #[doc = "One Shot Mode Aux"]
    #[inline(always)]
    pub const fn set_oneshotx(&mut self, val: super::vals::Sm2captctrlxOneshotx) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Edge X 0"]
    #[must_use]
    #[inline(always)]
    pub const fn edgx0(&self) -> super::vals::Sm2captctrlxEdgx0 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Sm2captctrlxEdgx0::from_bits(val as u8)
    }
    #[doc = "Edge X 0"]
    #[inline(always)]
    pub const fn set_edgx0(&mut self, val: super::vals::Sm2captctrlxEdgx0) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Edge X 1"]
    #[must_use]
    #[inline(always)]
    pub const fn edgx1(&self) -> super::vals::Sm2captctrlxEdgx1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Sm2captctrlxEdgx1::from_bits(val as u8)
    }
    #[doc = "Edge X 1"]
    #[inline(always)]
    pub const fn set_edgx1(&mut self, val: super::vals::Sm2captctrlxEdgx1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Input Select X"]
    #[must_use]
    #[inline(always)]
    pub const fn inp_selx(&self) -> super::vals::Sm2captctrlxInpSelx {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Sm2captctrlxInpSelx::from_bits(val as u8)
    }
    #[doc = "Input Select X"]
    #[inline(always)]
    pub const fn set_inp_selx(&mut self, val: super::vals::Sm2captctrlxInpSelx) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Edge Counter X Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntx_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Edge Counter X Enable"]
    #[inline(always)]
    pub const fn set_edgcntx_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture X FIFOs Water Mark"]
    #[must_use]
    #[inline(always)]
    pub const fn cfxwm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Capture X FIFOs Water Mark"]
    #[inline(always)]
    pub const fn set_cfxwm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Capture X0 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn cx0cnt(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Capture X0 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_cx0cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u16) & 0x07) << 10usize);
    }
    #[doc = "Capture X1 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn cx1cnt(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Capture X1 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_cx1cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u16) & 0x07) << 13usize);
    }
}
impl Default for Sm2captctrlx {
    #[inline(always)]
    fn default() -> Sm2captctrlx {
        Sm2captctrlx(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm2captctrlx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2captctrlx")
            .field("armx", &self.armx())
            .field("oneshotx", &self.oneshotx())
            .field("edgx0", &self.edgx0())
            .field("edgx1", &self.edgx1())
            .field("inp_selx", &self.inp_selx())
            .field("edgcntx_en", &self.edgcntx_en())
            .field("cfxwm", &self.cfxwm())
            .field("cx0cnt", &self.cx0cnt())
            .field("cx1cnt", &self.cx1cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2captctrlx {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm2captctrlx {
            armx: bool,
            oneshotx: super::vals::Sm2captctrlxOneshotx,
            edgx0: super::vals::Sm2captctrlxEdgx0,
            edgx1: super::vals::Sm2captctrlxEdgx1,
            inp_selx: super::vals::Sm2captctrlxInpSelx,
            edgcntx_en: bool,
            cfxwm: u8,
            cx0cnt: u8,
            cx1cnt: u8,
        }
        let proxy = Sm2captctrlx {
            armx: self.armx(),
            oneshotx: self.oneshotx(),
            edgx0: self.edgx0(),
            edgx1: self.edgx1(),
            inp_selx: self.inp_selx(),
            edgcntx_en: self.edgcntx_en(),
            cfxwm: self.cfxwm(),
            cx0cnt: self.cx0cnt(),
            cx1cnt: self.cx1cnt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture PWM_A Input Filter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2captfilta(pub u16);
impl Sm2captfilta {
    #[doc = "Input Capture Filter Period"]
    #[must_use]
    #[inline(always)]
    pub const fn capta_filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Capture Filter Period"]
    #[inline(always)]
    pub const fn set_capta_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Input Capture Filter Count"]
    #[must_use]
    #[inline(always)]
    pub const fn capta_filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Capture Filter Count"]
    #[inline(always)]
    pub const fn set_capta_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
}
impl Default for Sm2captfilta {
    #[inline(always)]
    fn default() -> Sm2captfilta {
        Sm2captfilta(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm2captfilta {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2captfilta")
            .field("capta_filt_per", &self.capta_filt_per())
            .field("capta_filt_cnt", &self.capta_filt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2captfilta {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm2captfilta {
            capta_filt_per: u8,
            capta_filt_cnt: u8,
        }
        let proxy = Sm2captfilta {
            capta_filt_per: self.capta_filt_per(),
            capta_filt_cnt: self.capta_filt_cnt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture PWM_B Input Filter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2captfiltb(pub u16);
impl Sm2captfiltb {
    #[doc = "Input Capture Filter Period"]
    #[must_use]
    #[inline(always)]
    pub const fn captb_filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Capture Filter Period"]
    #[inline(always)]
    pub const fn set_captb_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Input Capture Filter Count"]
    #[must_use]
    #[inline(always)]
    pub const fn captb_filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Capture Filter Count"]
    #[inline(always)]
    pub const fn set_captb_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
}
impl Default for Sm2captfiltb {
    #[inline(always)]
    fn default() -> Sm2captfiltb {
        Sm2captfiltb(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm2captfiltb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2captfiltb")
            .field("captb_filt_per", &self.captb_filt_per())
            .field("captb_filt_cnt", &self.captb_filt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2captfiltb {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm2captfiltb {
            captb_filt_per: u8,
            captb_filt_cnt: u8,
        }
        let proxy = Sm2captfiltb {
            captb_filt_per: self.captb_filt_per(),
            captb_filt_cnt: self.captb_filt_cnt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture PWM_X Input Filter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2captfiltx(pub u16);
impl Sm2captfiltx {
    #[doc = "Input Capture Filter Period"]
    #[must_use]
    #[inline(always)]
    pub const fn captx_filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Capture Filter Period"]
    #[inline(always)]
    pub const fn set_captx_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Input Capture Filter Count"]
    #[must_use]
    #[inline(always)]
    pub const fn captx_filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Capture Filter Count"]
    #[inline(always)]
    pub const fn set_captx_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
}
impl Default for Sm2captfiltx {
    #[inline(always)]
    fn default() -> Sm2captfiltx {
        Sm2captfiltx(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm2captfiltx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2captfiltx")
            .field("captx_filt_per", &self.captx_filt_per())
            .field("captx_filt_cnt", &self.captx_filt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2captfiltx {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm2captfiltx {
            captx_filt_per: u8,
            captx_filt_cnt: u8,
        }
        let proxy = Sm2captfiltx {
            captx_filt_per: self.captx_filt_per(),
            captx_filt_cnt: self.captx_filt_cnt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2ctrl(pub u16);
impl Sm2ctrl {
    #[doc = "Double Switching Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dblen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Double Switching Enable"]
    #[inline(always)]
    pub const fn set_dblen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "PWM_X Double Switching Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dblx(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Double Switching Enable"]
    #[inline(always)]
    pub const fn set_dblx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Load Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ldmod(&self) -> super::vals::Sm2ctrlLdmod {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Sm2ctrlLdmod::from_bits(val as u8)
    }
    #[doc = "Load Mode Select"]
    #[inline(always)]
    pub const fn set_ldmod(&mut self, val: super::vals::Sm2ctrlLdmod) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Split the DBLPWM signal to PWM_A and PWM_B"]
    #[must_use]
    #[inline(always)]
    pub const fn split(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Split the DBLPWM signal to PWM_A and PWM_B"]
    #[inline(always)]
    pub const fn set_split(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Prescaler"]
    #[must_use]
    #[inline(always)]
    pub const fn prsc(&self) -> super::vals::Sm2ctrlPrsc {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Sm2ctrlPrsc::from_bits(val as u8)
    }
    #[doc = "Prescaler"]
    #[inline(always)]
    pub const fn set_prsc(&mut self, val: super::vals::Sm2ctrlPrsc) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u16) & 0x07) << 4usize);
    }
    #[doc = "Compare Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn compmode(&self) -> super::vals::Sm2ctrlCompmode {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Sm2ctrlCompmode::from_bits(val as u8)
    }
    #[doc = "Compare Mode"]
    #[inline(always)]
    pub const fn set_compmode(&mut self, val: super::vals::Sm2ctrlCompmode) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "Deadtime"]
    #[must_use]
    #[inline(always)]
    pub const fn dt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Deadtime"]
    #[inline(always)]
    pub const fn set_dt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Full Cycle Reload"]
    #[must_use]
    #[inline(always)]
    pub const fn full(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Full Cycle Reload"]
    #[inline(always)]
    pub const fn set_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Half Cycle Reload"]
    #[must_use]
    #[inline(always)]
    pub const fn half(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Half Cycle Reload"]
    #[inline(always)]
    pub const fn set_half(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Load Frequency"]
    #[must_use]
    #[inline(always)]
    pub const fn ldfq(&self) -> super::vals::Sm2ctrlLdfq {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Sm2ctrlLdfq::from_bits(val as u8)
    }
    #[doc = "Load Frequency"]
    #[inline(always)]
    pub const fn set_ldfq(&mut self, val: super::vals::Sm2ctrlLdfq) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u16) & 0x0f) << 12usize);
    }
}
impl Default for Sm2ctrl {
    #[inline(always)]
    fn default() -> Sm2ctrl {
        Sm2ctrl(1024u64 as u16)
    }
}
impl core::fmt::Debug for Sm2ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2ctrl")
            .field("dblen", &self.dblen())
            .field("dblx", &self.dblx())
            .field("ldmod", &self.ldmod())
            .field("split", &self.split())
            .field("prsc", &self.prsc())
            .field("compmode", &self.compmode())
            .field("dt", &self.dt())
            .field("full", &self.full())
            .field("half", &self.half())
            .field("ldfq", &self.ldfq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2ctrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm2ctrl {
            dblen: bool,
            dblx: bool,
            ldmod: super::vals::Sm2ctrlLdmod,
            split: bool,
            prsc: super::vals::Sm2ctrlPrsc,
            compmode: super::vals::Sm2ctrlCompmode,
            dt: u8,
            full: bool,
            half: bool,
            ldfq: super::vals::Sm2ctrlLdfq,
        }
        let proxy = Sm2ctrl {
            dblen: self.dblen(),
            dblx: self.dblx(),
            ldmod: self.ldmod(),
            split: self.split(),
            prsc: self.prsc(),
            compmode: self.compmode(),
            dt: self.dt(),
            full: self.full(),
            half: self.half(),
            ldfq: self.ldfq(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Control 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2ctrl2(pub u16);
impl Sm2ctrl2 {
    #[doc = "Clock Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn clk_sel(&self) -> super::vals::Sm2ctrl2ClkSel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sm2ctrl2ClkSel::from_bits(val as u8)
    }
    #[doc = "Clock Source Select"]
    #[inline(always)]
    pub const fn set_clk_sel(&mut self, val: super::vals::Sm2ctrl2ClkSel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Reload Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn reload_sel(&self) -> super::vals::Sm2ctrl2ReloadSel {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Sm2ctrl2ReloadSel::from_bits(val as u8)
    }
    #[doc = "Reload Source Select"]
    #[inline(always)]
    pub const fn set_reload_sel(&mut self, val: super::vals::Sm2ctrl2ReloadSel) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Force Select"]
    #[must_use]
    #[inline(always)]
    pub const fn force_sel(&self) -> super::vals::Sm2ctrl2ForceSel {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::Sm2ctrl2ForceSel::from_bits(val as u8)
    }
    #[doc = "Force Select"]
    #[inline(always)]
    pub const fn set_force_sel(&mut self, val: super::vals::Sm2ctrl2ForceSel) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u16) & 0x07) << 3usize);
    }
    #[doc = "Force Initialization"]
    #[must_use]
    #[inline(always)]
    pub const fn force(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Force Initialization"]
    #[inline(always)]
    pub const fn set_force(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Force Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn frcen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Force Enable"]
    #[inline(always)]
    pub const fn set_frcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Initialization Control Select"]
    #[must_use]
    #[inline(always)]
    pub const fn init_sel(&self) -> super::vals::Sm2ctrl2InitSel {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Sm2ctrl2InitSel::from_bits(val as u8)
    }
    #[doc = "Initialization Control Select"]
    #[inline(always)]
    pub const fn set_init_sel(&mut self, val: super::vals::Sm2ctrl2InitSel) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "PWM_X Initial Value"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmx_init(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Initial Value"]
    #[inline(always)]
    pub const fn set_pwmx_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "PWM45 Initial Value"]
    #[must_use]
    #[inline(always)]
    pub const fn pwm45_init(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "PWM45 Initial Value"]
    #[inline(always)]
    pub const fn set_pwm45_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "PWM23 Initial Value"]
    #[must_use]
    #[inline(always)]
    pub const fn pwm23_init(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "PWM23 Initial Value"]
    #[inline(always)]
    pub const fn set_pwm23_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Independent or Complementary Pair Operation"]
    #[must_use]
    #[inline(always)]
    pub const fn indep(&self) -> super::vals::Sm2ctrl2Indep {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Sm2ctrl2Indep::from_bits(val as u8)
    }
    #[doc = "Independent or Complementary Pair Operation"]
    #[inline(always)]
    pub const fn set_indep(&mut self, val: super::vals::Sm2ctrl2Indep) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "Wait Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn waiten(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Wait Enable"]
    #[inline(always)]
    pub const fn set_waiten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "Debug Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dbgen(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Enable"]
    #[inline(always)]
    pub const fn set_dbgen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Sm2ctrl2 {
    #[inline(always)]
    fn default() -> Sm2ctrl2 {
        Sm2ctrl2(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm2ctrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2ctrl2")
            .field("clk_sel", &self.clk_sel())
            .field("reload_sel", &self.reload_sel())
            .field("force_sel", &self.force_sel())
            .field("force", &self.force())
            .field("frcen", &self.frcen())
            .field("init_sel", &self.init_sel())
            .field("pwmx_init", &self.pwmx_init())
            .field("pwm45_init", &self.pwm45_init())
            .field("pwm23_init", &self.pwm23_init())
            .field("indep", &self.indep())
            .field("waiten", &self.waiten())
            .field("dbgen", &self.dbgen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2ctrl2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm2ctrl2 {
            clk_sel: super::vals::Sm2ctrl2ClkSel,
            reload_sel: super::vals::Sm2ctrl2ReloadSel,
            force_sel: super::vals::Sm2ctrl2ForceSel,
            force: bool,
            frcen: bool,
            init_sel: super::vals::Sm2ctrl2InitSel,
            pwmx_init: bool,
            pwm45_init: bool,
            pwm23_init: bool,
            indep: super::vals::Sm2ctrl2Indep,
            waiten: bool,
            dbgen: bool,
        }
        let proxy = Sm2ctrl2 {
            clk_sel: self.clk_sel(),
            reload_sel: self.reload_sel(),
            force_sel: self.force_sel(),
            force: self.force(),
            frcen: self.frcen(),
            init_sel: self.init_sel(),
            pwmx_init: self.pwmx_init(),
            pwm45_init: self.pwm45_init(),
            pwm23_init: self.pwm23_init(),
            indep: self.indep(),
            waiten: self.waiten(),
            dbgen: self.dbgen(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Value 0 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2cval0cyc(pub u16);
impl Sm2cval0cyc {
    #[doc = "Capture Value 0 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval0cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 0 Cycle"]
    #[inline(always)]
    pub const fn set_cval0cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm2cval0cyc {
    #[inline(always)]
    fn default() -> Sm2cval0cyc {
        Sm2cval0cyc(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm2cval0cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2cval0cyc")
            .field("cval0cyc", &self.cval0cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2cval0cyc {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm2cval0cyc {
            cval0cyc: u8,
        }
        let proxy = Sm2cval0cyc {
            cval0cyc: self.cval0cyc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Value 1 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2cval1cyc(pub u16);
impl Sm2cval1cyc {
    #[doc = "Capture Value 1 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval1cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 1 Cycle"]
    #[inline(always)]
    pub const fn set_cval1cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm2cval1cyc {
    #[inline(always)]
    fn default() -> Sm2cval1cyc {
        Sm2cval1cyc(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm2cval1cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2cval1cyc")
            .field("cval1cyc", &self.cval1cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2cval1cyc {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm2cval1cyc {
            cval1cyc: u8,
        }
        let proxy = Sm2cval1cyc {
            cval1cyc: self.cval1cyc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Value 2 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2cval2cyc(pub u16);
impl Sm2cval2cyc {
    #[doc = "Capture Value 2 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval2cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 2 Cycle"]
    #[inline(always)]
    pub const fn set_cval2cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm2cval2cyc {
    #[inline(always)]
    fn default() -> Sm2cval2cyc {
        Sm2cval2cyc(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm2cval2cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2cval2cyc")
            .field("cval2cyc", &self.cval2cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2cval2cyc {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm2cval2cyc {
            cval2cyc: u8,
        }
        let proxy = Sm2cval2cyc {
            cval2cyc: self.cval2cyc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Value 3 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2cval3cyc(pub u16);
impl Sm2cval3cyc {
    #[doc = "Capture Value 3 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval3cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 3 Cycle"]
    #[inline(always)]
    pub const fn set_cval3cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm2cval3cyc {
    #[inline(always)]
    fn default() -> Sm2cval3cyc {
        Sm2cval3cyc(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm2cval3cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2cval3cyc")
            .field("cval3cyc", &self.cval3cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2cval3cyc {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm2cval3cyc {
            cval3cyc: u8,
        }
        let proxy = Sm2cval3cyc {
            cval3cyc: self.cval3cyc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Value 4 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2cval4cyc(pub u16);
impl Sm2cval4cyc {
    #[doc = "Capture Value 4 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval4cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 4 Cycle"]
    #[inline(always)]
    pub const fn set_cval4cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm2cval4cyc {
    #[inline(always)]
    fn default() -> Sm2cval4cyc {
        Sm2cval4cyc(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm2cval4cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2cval4cyc")
            .field("cval4cyc", &self.cval4cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2cval4cyc {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm2cval4cyc {
            cval4cyc: u8,
        }
        let proxy = Sm2cval4cyc {
            cval4cyc: self.cval4cyc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Value 5 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2cval5cyc(pub u16);
impl Sm2cval5cyc {
    #[doc = "Capture Value 5 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval5cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 5 Cycle"]
    #[inline(always)]
    pub const fn set_cval5cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm2cval5cyc {
    #[inline(always)]
    fn default() -> Sm2cval5cyc {
        Sm2cval5cyc(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm2cval5cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2cval5cyc")
            .field("cval5cyc", &self.cval5cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2cval5cyc {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm2cval5cyc {
            cval5cyc: u8,
        }
        let proxy = Sm2cval5cyc {
            cval5cyc: self.cval5cyc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fault Disable Mapping Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2dismap0(pub u16);
impl Sm2dismap0 {
    #[doc = "PWM_A Fault Disable Mask 0"]
    #[must_use]
    #[inline(always)]
    pub const fn dis0a(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_A Fault Disable Mask 0"]
    #[inline(always)]
    pub const fn set_dis0a(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "PWM_B Fault Disable Mask 0"]
    #[must_use]
    #[inline(always)]
    pub const fn dis0b(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_B Fault Disable Mask 0"]
    #[inline(always)]
    pub const fn set_dis0b(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
    }
    #[doc = "PWM_X Fault Disable Mask 0"]
    #[must_use]
    #[inline(always)]
    pub const fn dis0x(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_X Fault Disable Mask 0"]
    #[inline(always)]
    pub const fn set_dis0x(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
}
impl Default for Sm2dismap0 {
    #[inline(always)]
    fn default() -> Sm2dismap0 {
        Sm2dismap0(65535u64 as u16)
    }
}
impl core::fmt::Debug for Sm2dismap0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2dismap0")
            .field("dis0a", &self.dis0a())
            .field("dis0b", &self.dis0b())
            .field("dis0x", &self.dis0x())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2dismap0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm2dismap0 {
            dis0a: u8,
            dis0b: u8,
            dis0x: u8,
        }
        let proxy = Sm2dismap0 {
            dis0a: self.dis0a(),
            dis0b: self.dis0b(),
            dis0x: self.dis0x(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "DMA Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2dmaen(pub u16);
impl Sm2dmaen {
    #[doc = "Capture X0 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cx0de(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X0 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_cx0de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Capture X1 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cx1de(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X1 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_cx1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Capture B0 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cb0de(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Capture B0 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_cb0de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Capture B1 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cb1de(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Capture B1 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_cb1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Capture A0 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ca0de(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Capture A0 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_ca0de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Capture A1 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ca1de(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Capture A1 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_ca1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Capture DMA Enable Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn captde(&self) -> super::vals::Sm2dmaenCaptde {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Sm2dmaenCaptde::from_bits(val as u8)
    }
    #[doc = "Capture DMA Enable Source Select"]
    #[inline(always)]
    pub const fn set_captde(&mut self, val: super::vals::Sm2dmaenCaptde) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "FIFO Watermark AND Control"]
    #[must_use]
    #[inline(always)]
    pub const fn fand(&self) -> super::vals::Sm2dmaenFand {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Sm2dmaenFand::from_bits(val as u8)
    }
    #[doc = "FIFO Watermark AND Control"]
    #[inline(always)]
    pub const fn set_fand(&mut self, val: super::vals::Sm2dmaenFand) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "Value Registers DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn valde(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Value Registers DMA Enable"]
    #[inline(always)]
    pub const fn set_valde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
}
impl Default for Sm2dmaen {
    #[inline(always)]
    fn default() -> Sm2dmaen {
        Sm2dmaen(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm2dmaen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2dmaen")
            .field("cx0de", &self.cx0de())
            .field("cx1de", &self.cx1de())
            .field("cb0de", &self.cb0de())
            .field("cb1de", &self.cb1de())
            .field("ca0de", &self.ca0de())
            .field("ca1de", &self.ca1de())
            .field("captde", &self.captde())
            .field("fand", &self.fand())
            .field("valde", &self.valde())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2dmaen {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm2dmaen {
            cx0de: bool,
            cx1de: bool,
            cb0de: bool,
            cb1de: bool,
            ca0de: bool,
            ca1de: bool,
            captde: super::vals::Sm2dmaenCaptde,
            fand: super::vals::Sm2dmaenFand,
            valde: bool,
        }
        let proxy = Sm2dmaen {
            cx0de: self.cx0de(),
            cx1de: self.cx1de(),
            cb0de: self.cb0de(),
            cb1de: self.cb1de(),
            ca0de: self.ca0de(),
            ca1de: self.ca1de(),
            captde: self.captde(),
            fand: self.fand(),
            valde: self.valde(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fractional Value Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2fracval1(pub u16);
impl Sm2fracval1 {
    #[doc = "Fractional Value 1"]
    #[must_use]
    #[inline(always)]
    pub const fn fracval1(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 1"]
    #[inline(always)]
    pub const fn set_fracval1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Sm2fracval1 {
    #[inline(always)]
    fn default() -> Sm2fracval1 {
        Sm2fracval1(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm2fracval1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2fracval1")
            .field("fracval1", &self.fracval1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2fracval1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm2fracval1 {
            fracval1: u8,
        }
        let proxy = Sm2fracval1 {
            fracval1: self.fracval1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fractional Value Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2fracval2(pub u16);
impl Sm2fracval2 {
    #[doc = "Fractional Value 2"]
    #[must_use]
    #[inline(always)]
    pub const fn fracval2(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 2"]
    #[inline(always)]
    pub const fn set_fracval2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Sm2fracval2 {
    #[inline(always)]
    fn default() -> Sm2fracval2 {
        Sm2fracval2(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm2fracval2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2fracval2")
            .field("fracval2", &self.fracval2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2fracval2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm2fracval2 {
            fracval2: u8,
        }
        let proxy = Sm2fracval2 {
            fracval2: self.fracval2(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fractional Value Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2fracval3(pub u16);
impl Sm2fracval3 {
    #[doc = "Fractional Value 3"]
    #[must_use]
    #[inline(always)]
    pub const fn fracval3(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 3"]
    #[inline(always)]
    pub const fn set_fracval3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Sm2fracval3 {
    #[inline(always)]
    fn default() -> Sm2fracval3 {
        Sm2fracval3(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm2fracval3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2fracval3")
            .field("fracval3", &self.fracval3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2fracval3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm2fracval3 {
            fracval3: u8,
        }
        let proxy = Sm2fracval3 {
            fracval3: self.fracval3(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fractional Value Register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2fracval4(pub u16);
impl Sm2fracval4 {
    #[doc = "Fractional Value 4"]
    #[must_use]
    #[inline(always)]
    pub const fn fracval4(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 4"]
    #[inline(always)]
    pub const fn set_fracval4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Sm2fracval4 {
    #[inline(always)]
    fn default() -> Sm2fracval4 {
        Sm2fracval4(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm2fracval4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2fracval4")
            .field("fracval4", &self.fracval4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2fracval4 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm2fracval4 {
            fracval4: u8,
        }
        let proxy = Sm2fracval4 {
            fracval4: self.fracval4(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fractional Value Register 5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2fracval5(pub u16);
impl Sm2fracval5 {
    #[doc = "Fractional Value 5"]
    #[must_use]
    #[inline(always)]
    pub const fn fracval5(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 5"]
    #[inline(always)]
    pub const fn set_fracval5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Sm2fracval5 {
    #[inline(always)]
    fn default() -> Sm2fracval5 {
        Sm2fracval5(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm2fracval5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2fracval5")
            .field("fracval5", &self.fracval5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2fracval5 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm2fracval5 {
            fracval5: u8,
        }
        let proxy = Sm2fracval5 {
            fracval5: self.fracval5(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fractional Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2frctrl(pub u16);
impl Sm2frctrl {
    #[doc = "Fractional Cycle PWM Period Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn frac1_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Fractional Cycle PWM Period Enable"]
    #[inline(always)]
    pub const fn set_frac1_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Fractional Cycle Placement Enable for PWM_A"]
    #[must_use]
    #[inline(always)]
    pub const fn frac23_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Fractional Cycle Placement Enable for PWM_A"]
    #[inline(always)]
    pub const fn set_frac23_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Fractional Cycle Placement Enable for PWM_B"]
    #[must_use]
    #[inline(always)]
    pub const fn frac45_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Fractional Cycle Placement Enable for PWM_B"]
    #[inline(always)]
    pub const fn set_frac45_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Test Status Bit"]
    #[must_use]
    #[inline(always)]
    pub const fn test(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Test Status Bit"]
    #[inline(always)]
    pub const fn set_test(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Sm2frctrl {
    #[inline(always)]
    fn default() -> Sm2frctrl {
        Sm2frctrl(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm2frctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2frctrl")
            .field("frac1_en", &self.frac1_en())
            .field("frac23_en", &self.frac23_en())
            .field("frac45_en", &self.frac45_en())
            .field("test", &self.test())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2frctrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm2frctrl {
            frac1_en: bool,
            frac23_en: bool,
            frac45_en: bool,
            test: bool,
        }
        let proxy = Sm2frctrl {
            frac1_en: self.frac1_en(),
            frac23_en: self.frac23_en(),
            frac45_en: self.frac45_en(),
            test: self.test(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2inten(pub u16);
impl Sm2inten {
    #[doc = "Compare Interrupt Enables"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpie(&self) -> super::vals::Sm2intenCmpie {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Sm2intenCmpie::from_bits(val as u8)
    }
    #[doc = "Compare Interrupt Enables"]
    #[inline(always)]
    pub const fn set_cmpie(&mut self, val: super::vals::Sm2intenCmpie) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Capture X 0 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cx0ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X 0 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cx0ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Capture X 1 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cx1ie(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cx1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture B 0 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cb0ie(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Capture B 0 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cb0ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "Capture B 1 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cb1ie(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Capture B 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cb1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "Capture A 0 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ca0ie(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Capture A 0 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ca0ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Capture A 1 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ca1ie(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Capture A 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ca1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Reload Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Interrupt Enable"]
    #[inline(always)]
    pub const fn set_rie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Reload Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn reie(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_reie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
}
impl Default for Sm2inten {
    #[inline(always)]
    fn default() -> Sm2inten {
        Sm2inten(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm2inten {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2inten")
            .field("cmpie", &self.cmpie())
            .field("cx0ie", &self.cx0ie())
            .field("cx1ie", &self.cx1ie())
            .field("cb0ie", &self.cb0ie())
            .field("cb1ie", &self.cb1ie())
            .field("ca0ie", &self.ca0ie())
            .field("ca1ie", &self.ca1ie())
            .field("rie", &self.rie())
            .field("reie", &self.reie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2inten {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm2inten {
            cmpie: super::vals::Sm2intenCmpie,
            cx0ie: bool,
            cx1ie: bool,
            cb0ie: bool,
            cb1ie: bool,
            ca0ie: bool,
            ca1ie: bool,
            rie: bool,
            reie: bool,
        }
        let proxy = Sm2inten {
            cmpie: self.cmpie(),
            cx0ie: self.cx0ie(),
            cx1ie: self.cx1ie(),
            cb0ie: self.cb0ie(),
            cb1ie: self.cb1ie(),
            ca0ie: self.ca0ie(),
            ca1ie: self.ca1ie(),
            rie: self.rie(),
            reie: self.reie(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Output Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2octrl(pub u16);
impl Sm2octrl {
    #[doc = "PWM_X Fault State"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmxfs(&self) -> super::vals::Sm2octrlPwmxfs {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sm2octrlPwmxfs::from_bits(val as u8)
    }
    #[doc = "PWM_X Fault State"]
    #[inline(always)]
    pub const fn set_pwmxfs(&mut self, val: super::vals::Sm2octrlPwmxfs) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "PWM_B Fault State"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmbfs(&self) -> super::vals::Sm2octrlPwmbfs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Sm2octrlPwmbfs::from_bits(val as u8)
    }
    #[doc = "PWM_B Fault State"]
    #[inline(always)]
    pub const fn set_pwmbfs(&mut self, val: super::vals::Sm2octrlPwmbfs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "PWM_A Fault State"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmafs(&self) -> super::vals::Sm2octrlPwmafs {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Sm2octrlPwmafs::from_bits(val as u8)
    }
    #[doc = "PWM_A Fault State"]
    #[inline(always)]
    pub const fn set_pwmafs(&mut self, val: super::vals::Sm2octrlPwmafs) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "PWM_X Output Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn polx(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Output Polarity"]
    #[inline(always)]
    pub const fn set_polx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "PWM_B Output Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn polb(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_B Output Polarity"]
    #[inline(always)]
    pub const fn set_polb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "PWM_A Output Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn pola(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_A Output Polarity"]
    #[inline(always)]
    pub const fn set_pola(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "PWM_X Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmx_in(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Input"]
    #[inline(always)]
    pub const fn set_pwmx_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "PWM_B Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmb_in(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_B Input"]
    #[inline(always)]
    pub const fn set_pwmb_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "PWM_A Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pwma_in(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_A Input"]
    #[inline(always)]
    pub const fn set_pwma_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Sm2octrl {
    #[inline(always)]
    fn default() -> Sm2octrl {
        Sm2octrl(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm2octrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2octrl")
            .field("pwmxfs", &self.pwmxfs())
            .field("pwmbfs", &self.pwmbfs())
            .field("pwmafs", &self.pwmafs())
            .field("polx", &self.polx())
            .field("polb", &self.polb())
            .field("pola", &self.pola())
            .field("pwmx_in", &self.pwmx_in())
            .field("pwmb_in", &self.pwmb_in())
            .field("pwma_in", &self.pwma_in())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2octrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm2octrl {
            pwmxfs: super::vals::Sm2octrlPwmxfs,
            pwmbfs: super::vals::Sm2octrlPwmbfs,
            pwmafs: super::vals::Sm2octrlPwmafs,
            polx: bool,
            polb: bool,
            pola: bool,
            pwmx_in: bool,
            pwmb_in: bool,
            pwma_in: bool,
        }
        let proxy = Sm2octrl {
            pwmxfs: self.pwmxfs(),
            pwmbfs: self.pwmbfs(),
            pwmafs: self.pwmafs(),
            polx: self.polx(),
            polb: self.polb(),
            pola: self.pola(),
            pwmx_in: self.pwmx_in(),
            pwmb_in: self.pwmb_in(),
            pwma_in: self.pwma_in(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2sts(pub u16);
impl Sm2sts {
    #[doc = "Compare Flags"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpf(&self) -> super::vals::Sm2stsCmpf {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Sm2stsCmpf::from_bits(val as u8)
    }
    #[doc = "Compare Flags"]
    #[inline(always)]
    pub const fn set_cmpf(&mut self, val: super::vals::Sm2stsCmpf) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Capture Flag X0"]
    #[must_use]
    #[inline(always)]
    pub const fn cfx0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag X0"]
    #[inline(always)]
    pub const fn set_cfx0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Capture Flag X1"]
    #[must_use]
    #[inline(always)]
    pub const fn cfx1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag X1"]
    #[inline(always)]
    pub const fn set_cfx1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture Flag B0"]
    #[must_use]
    #[inline(always)]
    pub const fn cfb0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag B0"]
    #[inline(always)]
    pub const fn set_cfb0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "Capture Flag B1"]
    #[must_use]
    #[inline(always)]
    pub const fn cfb1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag B1"]
    #[inline(always)]
    pub const fn set_cfb1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "Capture Flag A0"]
    #[must_use]
    #[inline(always)]
    pub const fn cfa0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag A0"]
    #[inline(always)]
    pub const fn set_cfa0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Capture Flag A1"]
    #[must_use]
    #[inline(always)]
    pub const fn cfa1(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag A1"]
    #[inline(always)]
    pub const fn set_cfa1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Reload Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rf(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Flag"]
    #[inline(always)]
    pub const fn set_rf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Reload Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ref_(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Error Flag"]
    #[inline(always)]
    pub const fn set_ref_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Registers Updated Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ruf(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Registers Updated Flag"]
    #[inline(always)]
    pub const fn set_ruf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
}
impl Default for Sm2sts {
    #[inline(always)]
    fn default() -> Sm2sts {
        Sm2sts(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm2sts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2sts")
            .field("cmpf", &self.cmpf())
            .field("cfx0", &self.cfx0())
            .field("cfx1", &self.cfx1())
            .field("cfb0", &self.cfb0())
            .field("cfb1", &self.cfb1())
            .field("cfa0", &self.cfa0())
            .field("cfa1", &self.cfa1())
            .field("rf", &self.rf())
            .field("ref_", &self.ref_())
            .field("ruf", &self.ruf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2sts {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm2sts {
            cmpf: super::vals::Sm2stsCmpf,
            cfx0: bool,
            cfx1: bool,
            cfb0: bool,
            cfb1: bool,
            cfa0: bool,
            cfa1: bool,
            rf: bool,
            ref_: bool,
            ruf: bool,
        }
        let proxy = Sm2sts {
            cmpf: self.cmpf(),
            cfx0: self.cfx0(),
            cfx1: self.cfx1(),
            cfb0: self.cfb0(),
            cfb1: self.cfb1(),
            cfa0: self.cfa0(),
            cfa1: self.cfa1(),
            rf: self.rf(),
            ref_: self.ref_(),
            ruf: self.ruf(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Output Trigger Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2tctrl(pub u16);
impl Sm2tctrl {
    #[doc = "Output Trigger Enables"]
    #[must_use]
    #[inline(always)]
    pub const fn out_trig_en(&self) -> super::vals::Sm2tctrlOutTrigEn {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Sm2tctrlOutTrigEn::from_bits(val as u8)
    }
    #[doc = "Output Trigger Enables"]
    #[inline(always)]
    pub const fn set_out_trig_en(&mut self, val: super::vals::Sm2tctrlOutTrigEn) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Trigger Frequency"]
    #[must_use]
    #[inline(always)]
    pub const fn trgfrq(&self) -> super::vals::Sm2tctrlTrgfrq {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Sm2tctrlTrgfrq::from_bits(val as u8)
    }
    #[doc = "Trigger Frequency"]
    #[inline(always)]
    pub const fn set_trgfrq(&mut self, val: super::vals::Sm2tctrlTrgfrq) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
    #[doc = "Mux Output Trigger 1 Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn pwbot1(&self) -> super::vals::Sm2tctrlPwbot1 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Sm2tctrlPwbot1::from_bits(val as u8)
    }
    #[doc = "Mux Output Trigger 1 Source Select"]
    #[inline(always)]
    pub const fn set_pwbot1(&mut self, val: super::vals::Sm2tctrlPwbot1) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "Mux Output Trigger 0 Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn pwaot0(&self) -> super::vals::Sm2tctrlPwaot0 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Sm2tctrlPwaot0::from_bits(val as u8)
    }
    #[doc = "Mux Output Trigger 0 Source Select"]
    #[inline(always)]
    pub const fn set_pwaot0(&mut self, val: super::vals::Sm2tctrlPwaot0) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
    }
}
impl Default for Sm2tctrl {
    #[inline(always)]
    fn default() -> Sm2tctrl {
        Sm2tctrl(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm2tctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2tctrl")
            .field("out_trig_en", &self.out_trig_en())
            .field("trgfrq", &self.trgfrq())
            .field("pwbot1", &self.pwbot1())
            .field("pwaot0", &self.pwaot0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2tctrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm2tctrl {
            out_trig_en: super::vals::Sm2tctrlOutTrigEn,
            trgfrq: super::vals::Sm2tctrlTrgfrq,
            pwbot1: super::vals::Sm2tctrlPwbot1,
            pwaot0: super::vals::Sm2tctrlPwaot0,
        }
        let proxy = Sm2tctrl {
            out_trig_en: self.out_trig_en(),
            trgfrq: self.trgfrq(),
            pwbot1: self.pwbot1(),
            pwaot0: self.pwaot0(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Compare A Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3captcompa(pub u16);
impl Sm3captcompa {
    #[doc = "Edge Compare A"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcmpa(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Compare A"]
    #[inline(always)]
    pub const fn set_edgcmpa(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Edge Counter A"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcnta(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Counter A"]
    #[inline(always)]
    pub const fn set_edgcnta(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sm3captcompa {
    #[inline(always)]
    fn default() -> Sm3captcompa {
        Sm3captcompa(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm3captcompa {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3captcompa")
            .field("edgcmpa", &self.edgcmpa())
            .field("edgcnta", &self.edgcnta())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3captcompa {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm3captcompa {
            edgcmpa: u8,
            edgcnta: u8,
        }
        let proxy = Sm3captcompa {
            edgcmpa: self.edgcmpa(),
            edgcnta: self.edgcnta(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Compare B Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3captcompb(pub u16);
impl Sm3captcompb {
    #[doc = "Edge Compare B"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcmpb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Compare B"]
    #[inline(always)]
    pub const fn set_edgcmpb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Edge Counter B"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntb(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Counter B"]
    #[inline(always)]
    pub const fn set_edgcntb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sm3captcompb {
    #[inline(always)]
    fn default() -> Sm3captcompb {
        Sm3captcompb(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm3captcompb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3captcompb")
            .field("edgcmpb", &self.edgcmpb())
            .field("edgcntb", &self.edgcntb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3captcompb {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm3captcompb {
            edgcmpb: u8,
            edgcntb: u8,
        }
        let proxy = Sm3captcompb {
            edgcmpb: self.edgcmpb(),
            edgcntb: self.edgcntb(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Compare X Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3captcompx(pub u16);
impl Sm3captcompx {
    #[doc = "Edge Compare X"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcmpx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Compare X"]
    #[inline(always)]
    pub const fn set_edgcmpx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Edge Counter X"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntx(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Counter X"]
    #[inline(always)]
    pub const fn set_edgcntx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sm3captcompx {
    #[inline(always)]
    fn default() -> Sm3captcompx {
        Sm3captcompx(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm3captcompx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3captcompx")
            .field("edgcmpx", &self.edgcmpx())
            .field("edgcntx", &self.edgcntx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3captcompx {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm3captcompx {
            edgcmpx: u8,
            edgcntx: u8,
        }
        let proxy = Sm3captcompx {
            edgcmpx: self.edgcmpx(),
            edgcntx: self.edgcntx(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Control A Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3captctrla(pub u16);
impl Sm3captctrla {
    #[doc = "Arm A"]
    #[must_use]
    #[inline(always)]
    pub const fn arma(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Arm A"]
    #[inline(always)]
    pub const fn set_arma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "One Shot Mode A"]
    #[must_use]
    #[inline(always)]
    pub const fn oneshota(&self) -> super::vals::Sm3captctrlaOneshota {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sm3captctrlaOneshota::from_bits(val as u8)
    }
    #[doc = "One Shot Mode A"]
    #[inline(always)]
    pub const fn set_oneshota(&mut self, val: super::vals::Sm3captctrlaOneshota) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Edge A 0"]
    #[must_use]
    #[inline(always)]
    pub const fn edga0(&self) -> super::vals::Sm3captctrlaEdga0 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Sm3captctrlaEdga0::from_bits(val as u8)
    }
    #[doc = "Edge A 0"]
    #[inline(always)]
    pub const fn set_edga0(&mut self, val: super::vals::Sm3captctrlaEdga0) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Edge A 1"]
    #[must_use]
    #[inline(always)]
    pub const fn edga1(&self) -> super::vals::Sm3captctrlaEdga1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Sm3captctrlaEdga1::from_bits(val as u8)
    }
    #[doc = "Edge A 1"]
    #[inline(always)]
    pub const fn set_edga1(&mut self, val: super::vals::Sm3captctrlaEdga1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Input Select A"]
    #[must_use]
    #[inline(always)]
    pub const fn inp_sela(&self) -> super::vals::Sm3captctrlaInpSela {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Sm3captctrlaInpSela::from_bits(val as u8)
    }
    #[doc = "Input Select A"]
    #[inline(always)]
    pub const fn set_inp_sela(&mut self, val: super::vals::Sm3captctrlaInpSela) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Edge Counter A Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcnta_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Edge Counter A Enable"]
    #[inline(always)]
    pub const fn set_edgcnta_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture A FIFOs Water Mark"]
    #[must_use]
    #[inline(always)]
    pub const fn cfawm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Capture A FIFOs Water Mark"]
    #[inline(always)]
    pub const fn set_cfawm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Capture A0 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn ca0cnt(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Capture A0 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_ca0cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u16) & 0x07) << 10usize);
    }
    #[doc = "Capture A1 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn ca1cnt(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Capture A1 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_ca1cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u16) & 0x07) << 13usize);
    }
}
impl Default for Sm3captctrla {
    #[inline(always)]
    fn default() -> Sm3captctrla {
        Sm3captctrla(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm3captctrla {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3captctrla")
            .field("arma", &self.arma())
            .field("oneshota", &self.oneshota())
            .field("edga0", &self.edga0())
            .field("edga1", &self.edga1())
            .field("inp_sela", &self.inp_sela())
            .field("edgcnta_en", &self.edgcnta_en())
            .field("cfawm", &self.cfawm())
            .field("ca0cnt", &self.ca0cnt())
            .field("ca1cnt", &self.ca1cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3captctrla {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm3captctrla {
            arma: bool,
            oneshota: super::vals::Sm3captctrlaOneshota,
            edga0: super::vals::Sm3captctrlaEdga0,
            edga1: super::vals::Sm3captctrlaEdga1,
            inp_sela: super::vals::Sm3captctrlaInpSela,
            edgcnta_en: bool,
            cfawm: u8,
            ca0cnt: u8,
            ca1cnt: u8,
        }
        let proxy = Sm3captctrla {
            arma: self.arma(),
            oneshota: self.oneshota(),
            edga0: self.edga0(),
            edga1: self.edga1(),
            inp_sela: self.inp_sela(),
            edgcnta_en: self.edgcnta_en(),
            cfawm: self.cfawm(),
            ca0cnt: self.ca0cnt(),
            ca1cnt: self.ca1cnt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Control B Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3captctrlb(pub u16);
impl Sm3captctrlb {
    #[doc = "Arm B"]
    #[must_use]
    #[inline(always)]
    pub const fn armb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Arm B"]
    #[inline(always)]
    pub const fn set_armb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "One Shot Mode B"]
    #[must_use]
    #[inline(always)]
    pub const fn oneshotb(&self) -> super::vals::Sm3captctrlbOneshotb {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sm3captctrlbOneshotb::from_bits(val as u8)
    }
    #[doc = "One Shot Mode B"]
    #[inline(always)]
    pub const fn set_oneshotb(&mut self, val: super::vals::Sm3captctrlbOneshotb) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Edge B 0"]
    #[must_use]
    #[inline(always)]
    pub const fn edgb0(&self) -> super::vals::Sm3captctrlbEdgb0 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Sm3captctrlbEdgb0::from_bits(val as u8)
    }
    #[doc = "Edge B 0"]
    #[inline(always)]
    pub const fn set_edgb0(&mut self, val: super::vals::Sm3captctrlbEdgb0) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Edge B 1"]
    #[must_use]
    #[inline(always)]
    pub const fn edgb1(&self) -> super::vals::Sm3captctrlbEdgb1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Sm3captctrlbEdgb1::from_bits(val as u8)
    }
    #[doc = "Edge B 1"]
    #[inline(always)]
    pub const fn set_edgb1(&mut self, val: super::vals::Sm3captctrlbEdgb1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Input Select B"]
    #[must_use]
    #[inline(always)]
    pub const fn inp_selb(&self) -> super::vals::Sm3captctrlbInpSelb {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Sm3captctrlbInpSelb::from_bits(val as u8)
    }
    #[doc = "Input Select B"]
    #[inline(always)]
    pub const fn set_inp_selb(&mut self, val: super::vals::Sm3captctrlbInpSelb) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Edge Counter B Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntb_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Edge Counter B Enable"]
    #[inline(always)]
    pub const fn set_edgcntb_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture B FIFOs Water Mark"]
    #[must_use]
    #[inline(always)]
    pub const fn cfbwm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Capture B FIFOs Water Mark"]
    #[inline(always)]
    pub const fn set_cfbwm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Capture B0 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn cb0cnt(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Capture B0 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_cb0cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u16) & 0x07) << 10usize);
    }
    #[doc = "Capture B1 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn cb1cnt(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Capture B1 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_cb1cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u16) & 0x07) << 13usize);
    }
}
impl Default for Sm3captctrlb {
    #[inline(always)]
    fn default() -> Sm3captctrlb {
        Sm3captctrlb(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm3captctrlb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3captctrlb")
            .field("armb", &self.armb())
            .field("oneshotb", &self.oneshotb())
            .field("edgb0", &self.edgb0())
            .field("edgb1", &self.edgb1())
            .field("inp_selb", &self.inp_selb())
            .field("edgcntb_en", &self.edgcntb_en())
            .field("cfbwm", &self.cfbwm())
            .field("cb0cnt", &self.cb0cnt())
            .field("cb1cnt", &self.cb1cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3captctrlb {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm3captctrlb {
            armb: bool,
            oneshotb: super::vals::Sm3captctrlbOneshotb,
            edgb0: super::vals::Sm3captctrlbEdgb0,
            edgb1: super::vals::Sm3captctrlbEdgb1,
            inp_selb: super::vals::Sm3captctrlbInpSelb,
            edgcntb_en: bool,
            cfbwm: u8,
            cb0cnt: u8,
            cb1cnt: u8,
        }
        let proxy = Sm3captctrlb {
            armb: self.armb(),
            oneshotb: self.oneshotb(),
            edgb0: self.edgb0(),
            edgb1: self.edgb1(),
            inp_selb: self.inp_selb(),
            edgcntb_en: self.edgcntb_en(),
            cfbwm: self.cfbwm(),
            cb0cnt: self.cb0cnt(),
            cb1cnt: self.cb1cnt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Control X Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3captctrlx(pub u16);
impl Sm3captctrlx {
    #[doc = "Arm X"]
    #[must_use]
    #[inline(always)]
    pub const fn armx(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Arm X"]
    #[inline(always)]
    pub const fn set_armx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "One Shot Mode Aux"]
    #[must_use]
    #[inline(always)]
    pub const fn oneshotx(&self) -> super::vals::Sm3captctrlxOneshotx {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sm3captctrlxOneshotx::from_bits(val as u8)
    }
    #[doc = "One Shot Mode Aux"]
    #[inline(always)]
    pub const fn set_oneshotx(&mut self, val: super::vals::Sm3captctrlxOneshotx) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Edge X 0"]
    #[must_use]
    #[inline(always)]
    pub const fn edgx0(&self) -> super::vals::Sm3captctrlxEdgx0 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Sm3captctrlxEdgx0::from_bits(val as u8)
    }
    #[doc = "Edge X 0"]
    #[inline(always)]
    pub const fn set_edgx0(&mut self, val: super::vals::Sm3captctrlxEdgx0) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Edge X 1"]
    #[must_use]
    #[inline(always)]
    pub const fn edgx1(&self) -> super::vals::Sm3captctrlxEdgx1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Sm3captctrlxEdgx1::from_bits(val as u8)
    }
    #[doc = "Edge X 1"]
    #[inline(always)]
    pub const fn set_edgx1(&mut self, val: super::vals::Sm3captctrlxEdgx1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Input Select X"]
    #[must_use]
    #[inline(always)]
    pub const fn inp_selx(&self) -> super::vals::Sm3captctrlxInpSelx {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Sm3captctrlxInpSelx::from_bits(val as u8)
    }
    #[doc = "Input Select X"]
    #[inline(always)]
    pub const fn set_inp_selx(&mut self, val: super::vals::Sm3captctrlxInpSelx) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Edge Counter X Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntx_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Edge Counter X Enable"]
    #[inline(always)]
    pub const fn set_edgcntx_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture X FIFOs Water Mark"]
    #[must_use]
    #[inline(always)]
    pub const fn cfxwm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Capture X FIFOs Water Mark"]
    #[inline(always)]
    pub const fn set_cfxwm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Capture X0 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn cx0cnt(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Capture X0 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_cx0cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u16) & 0x07) << 10usize);
    }
    #[doc = "Capture X1 FIFO Word Count"]
    #[must_use]
    #[inline(always)]
    pub const fn cx1cnt(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Capture X1 FIFO Word Count"]
    #[inline(always)]
    pub const fn set_cx1cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u16) & 0x07) << 13usize);
    }
}
impl Default for Sm3captctrlx {
    #[inline(always)]
    fn default() -> Sm3captctrlx {
        Sm3captctrlx(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm3captctrlx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3captctrlx")
            .field("armx", &self.armx())
            .field("oneshotx", &self.oneshotx())
            .field("edgx0", &self.edgx0())
            .field("edgx1", &self.edgx1())
            .field("inp_selx", &self.inp_selx())
            .field("edgcntx_en", &self.edgcntx_en())
            .field("cfxwm", &self.cfxwm())
            .field("cx0cnt", &self.cx0cnt())
            .field("cx1cnt", &self.cx1cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3captctrlx {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm3captctrlx {
            armx: bool,
            oneshotx: super::vals::Sm3captctrlxOneshotx,
            edgx0: super::vals::Sm3captctrlxEdgx0,
            edgx1: super::vals::Sm3captctrlxEdgx1,
            inp_selx: super::vals::Sm3captctrlxInpSelx,
            edgcntx_en: bool,
            cfxwm: u8,
            cx0cnt: u8,
            cx1cnt: u8,
        }
        let proxy = Sm3captctrlx {
            armx: self.armx(),
            oneshotx: self.oneshotx(),
            edgx0: self.edgx0(),
            edgx1: self.edgx1(),
            inp_selx: self.inp_selx(),
            edgcntx_en: self.edgcntx_en(),
            cfxwm: self.cfxwm(),
            cx0cnt: self.cx0cnt(),
            cx1cnt: self.cx1cnt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture PWM_A Input Filter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3captfilta(pub u16);
impl Sm3captfilta {
    #[doc = "Input Capture Filter Period"]
    #[must_use]
    #[inline(always)]
    pub const fn capta_filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Capture Filter Period"]
    #[inline(always)]
    pub const fn set_capta_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Input Capture Filter Count"]
    #[must_use]
    #[inline(always)]
    pub const fn capta_filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Capture Filter Count"]
    #[inline(always)]
    pub const fn set_capta_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
}
impl Default for Sm3captfilta {
    #[inline(always)]
    fn default() -> Sm3captfilta {
        Sm3captfilta(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm3captfilta {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3captfilta")
            .field("capta_filt_per", &self.capta_filt_per())
            .field("capta_filt_cnt", &self.capta_filt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3captfilta {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm3captfilta {
            capta_filt_per: u8,
            capta_filt_cnt: u8,
        }
        let proxy = Sm3captfilta {
            capta_filt_per: self.capta_filt_per(),
            capta_filt_cnt: self.capta_filt_cnt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture PWM_B Input Filter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3captfiltb(pub u16);
impl Sm3captfiltb {
    #[doc = "Input Capture Filter Period"]
    #[must_use]
    #[inline(always)]
    pub const fn captb_filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Capture Filter Period"]
    #[inline(always)]
    pub const fn set_captb_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Input Capture Filter Count"]
    #[must_use]
    #[inline(always)]
    pub const fn captb_filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Capture Filter Count"]
    #[inline(always)]
    pub const fn set_captb_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
}
impl Default for Sm3captfiltb {
    #[inline(always)]
    fn default() -> Sm3captfiltb {
        Sm3captfiltb(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm3captfiltb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3captfiltb")
            .field("captb_filt_per", &self.captb_filt_per())
            .field("captb_filt_cnt", &self.captb_filt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3captfiltb {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm3captfiltb {
            captb_filt_per: u8,
            captb_filt_cnt: u8,
        }
        let proxy = Sm3captfiltb {
            captb_filt_per: self.captb_filt_per(),
            captb_filt_cnt: self.captb_filt_cnt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture PWM_X Input Filter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3captfiltx(pub u16);
impl Sm3captfiltx {
    #[doc = "Input Capture Filter Period"]
    #[must_use]
    #[inline(always)]
    pub const fn captx_filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Capture Filter Period"]
    #[inline(always)]
    pub const fn set_captx_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Input Capture Filter Count"]
    #[must_use]
    #[inline(always)]
    pub const fn captx_filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Capture Filter Count"]
    #[inline(always)]
    pub const fn set_captx_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
}
impl Default for Sm3captfiltx {
    #[inline(always)]
    fn default() -> Sm3captfiltx {
        Sm3captfiltx(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm3captfiltx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3captfiltx")
            .field("captx_filt_per", &self.captx_filt_per())
            .field("captx_filt_cnt", &self.captx_filt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3captfiltx {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm3captfiltx {
            captx_filt_per: u8,
            captx_filt_cnt: u8,
        }
        let proxy = Sm3captfiltx {
            captx_filt_per: self.captx_filt_per(),
            captx_filt_cnt: self.captx_filt_cnt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3ctrl(pub u16);
impl Sm3ctrl {
    #[doc = "Double Switching Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dblen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Double Switching Enable"]
    #[inline(always)]
    pub const fn set_dblen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "PWM_X Double Switching Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dblx(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Double Switching Enable"]
    #[inline(always)]
    pub const fn set_dblx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Load Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ldmod(&self) -> super::vals::Sm3ctrlLdmod {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Sm3ctrlLdmod::from_bits(val as u8)
    }
    #[doc = "Load Mode Select"]
    #[inline(always)]
    pub const fn set_ldmod(&mut self, val: super::vals::Sm3ctrlLdmod) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Split the DBLPWM signal to PWM_A and PWM_B"]
    #[must_use]
    #[inline(always)]
    pub const fn split(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Split the DBLPWM signal to PWM_A and PWM_B"]
    #[inline(always)]
    pub const fn set_split(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Prescaler"]
    #[must_use]
    #[inline(always)]
    pub const fn prsc(&self) -> super::vals::Sm3ctrlPrsc {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Sm3ctrlPrsc::from_bits(val as u8)
    }
    #[doc = "Prescaler"]
    #[inline(always)]
    pub const fn set_prsc(&mut self, val: super::vals::Sm3ctrlPrsc) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u16) & 0x07) << 4usize);
    }
    #[doc = "Compare Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn compmode(&self) -> super::vals::Sm3ctrlCompmode {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Sm3ctrlCompmode::from_bits(val as u8)
    }
    #[doc = "Compare Mode"]
    #[inline(always)]
    pub const fn set_compmode(&mut self, val: super::vals::Sm3ctrlCompmode) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "Deadtime"]
    #[must_use]
    #[inline(always)]
    pub const fn dt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Deadtime"]
    #[inline(always)]
    pub const fn set_dt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Full Cycle Reload"]
    #[must_use]
    #[inline(always)]
    pub const fn full(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Full Cycle Reload"]
    #[inline(always)]
    pub const fn set_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Half Cycle Reload"]
    #[must_use]
    #[inline(always)]
    pub const fn half(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Half Cycle Reload"]
    #[inline(always)]
    pub const fn set_half(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Load Frequency"]
    #[must_use]
    #[inline(always)]
    pub const fn ldfq(&self) -> super::vals::Sm3ctrlLdfq {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Sm3ctrlLdfq::from_bits(val as u8)
    }
    #[doc = "Load Frequency"]
    #[inline(always)]
    pub const fn set_ldfq(&mut self, val: super::vals::Sm3ctrlLdfq) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u16) & 0x0f) << 12usize);
    }
}
impl Default for Sm3ctrl {
    #[inline(always)]
    fn default() -> Sm3ctrl {
        Sm3ctrl(1024u64 as u16)
    }
}
impl core::fmt::Debug for Sm3ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3ctrl")
            .field("dblen", &self.dblen())
            .field("dblx", &self.dblx())
            .field("ldmod", &self.ldmod())
            .field("split", &self.split())
            .field("prsc", &self.prsc())
            .field("compmode", &self.compmode())
            .field("dt", &self.dt())
            .field("full", &self.full())
            .field("half", &self.half())
            .field("ldfq", &self.ldfq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3ctrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm3ctrl {
            dblen: bool,
            dblx: bool,
            ldmod: super::vals::Sm3ctrlLdmod,
            split: bool,
            prsc: super::vals::Sm3ctrlPrsc,
            compmode: super::vals::Sm3ctrlCompmode,
            dt: u8,
            full: bool,
            half: bool,
            ldfq: super::vals::Sm3ctrlLdfq,
        }
        let proxy = Sm3ctrl {
            dblen: self.dblen(),
            dblx: self.dblx(),
            ldmod: self.ldmod(),
            split: self.split(),
            prsc: self.prsc(),
            compmode: self.compmode(),
            dt: self.dt(),
            full: self.full(),
            half: self.half(),
            ldfq: self.ldfq(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Control 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3ctrl2(pub u16);
impl Sm3ctrl2 {
    #[doc = "Clock Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn clk_sel(&self) -> super::vals::Sm3ctrl2ClkSel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sm3ctrl2ClkSel::from_bits(val as u8)
    }
    #[doc = "Clock Source Select"]
    #[inline(always)]
    pub const fn set_clk_sel(&mut self, val: super::vals::Sm3ctrl2ClkSel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Reload Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn reload_sel(&self) -> super::vals::Sm3ctrl2ReloadSel {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Sm3ctrl2ReloadSel::from_bits(val as u8)
    }
    #[doc = "Reload Source Select"]
    #[inline(always)]
    pub const fn set_reload_sel(&mut self, val: super::vals::Sm3ctrl2ReloadSel) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Force Select"]
    #[must_use]
    #[inline(always)]
    pub const fn force_sel(&self) -> super::vals::Sm3ctrl2ForceSel {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::Sm3ctrl2ForceSel::from_bits(val as u8)
    }
    #[doc = "Force Select"]
    #[inline(always)]
    pub const fn set_force_sel(&mut self, val: super::vals::Sm3ctrl2ForceSel) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u16) & 0x07) << 3usize);
    }
    #[doc = "Force Initialization"]
    #[must_use]
    #[inline(always)]
    pub const fn force(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Force Initialization"]
    #[inline(always)]
    pub const fn set_force(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Force Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn frcen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Force Enable"]
    #[inline(always)]
    pub const fn set_frcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Initialization Control Select"]
    #[must_use]
    #[inline(always)]
    pub const fn init_sel(&self) -> super::vals::Sm3ctrl2InitSel {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Sm3ctrl2InitSel::from_bits(val as u8)
    }
    #[doc = "Initialization Control Select"]
    #[inline(always)]
    pub const fn set_init_sel(&mut self, val: super::vals::Sm3ctrl2InitSel) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "PWM_X Initial Value"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmx_init(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Initial Value"]
    #[inline(always)]
    pub const fn set_pwmx_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "PWM45 Initial Value"]
    #[must_use]
    #[inline(always)]
    pub const fn pwm45_init(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "PWM45 Initial Value"]
    #[inline(always)]
    pub const fn set_pwm45_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "PWM23 Initial Value"]
    #[must_use]
    #[inline(always)]
    pub const fn pwm23_init(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "PWM23 Initial Value"]
    #[inline(always)]
    pub const fn set_pwm23_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Independent or Complementary Pair Operation"]
    #[must_use]
    #[inline(always)]
    pub const fn indep(&self) -> super::vals::Sm3ctrl2Indep {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Sm3ctrl2Indep::from_bits(val as u8)
    }
    #[doc = "Independent or Complementary Pair Operation"]
    #[inline(always)]
    pub const fn set_indep(&mut self, val: super::vals::Sm3ctrl2Indep) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "Wait Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn waiten(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Wait Enable"]
    #[inline(always)]
    pub const fn set_waiten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "Debug Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dbgen(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Enable"]
    #[inline(always)]
    pub const fn set_dbgen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Sm3ctrl2 {
    #[inline(always)]
    fn default() -> Sm3ctrl2 {
        Sm3ctrl2(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm3ctrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3ctrl2")
            .field("clk_sel", &self.clk_sel())
            .field("reload_sel", &self.reload_sel())
            .field("force_sel", &self.force_sel())
            .field("force", &self.force())
            .field("frcen", &self.frcen())
            .field("init_sel", &self.init_sel())
            .field("pwmx_init", &self.pwmx_init())
            .field("pwm45_init", &self.pwm45_init())
            .field("pwm23_init", &self.pwm23_init())
            .field("indep", &self.indep())
            .field("waiten", &self.waiten())
            .field("dbgen", &self.dbgen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3ctrl2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm3ctrl2 {
            clk_sel: super::vals::Sm3ctrl2ClkSel,
            reload_sel: super::vals::Sm3ctrl2ReloadSel,
            force_sel: super::vals::Sm3ctrl2ForceSel,
            force: bool,
            frcen: bool,
            init_sel: super::vals::Sm3ctrl2InitSel,
            pwmx_init: bool,
            pwm45_init: bool,
            pwm23_init: bool,
            indep: super::vals::Sm3ctrl2Indep,
            waiten: bool,
            dbgen: bool,
        }
        let proxy = Sm3ctrl2 {
            clk_sel: self.clk_sel(),
            reload_sel: self.reload_sel(),
            force_sel: self.force_sel(),
            force: self.force(),
            frcen: self.frcen(),
            init_sel: self.init_sel(),
            pwmx_init: self.pwmx_init(),
            pwm45_init: self.pwm45_init(),
            pwm23_init: self.pwm23_init(),
            indep: self.indep(),
            waiten: self.waiten(),
            dbgen: self.dbgen(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Value 0 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3cval0cyc(pub u16);
impl Sm3cval0cyc {
    #[doc = "Capture Value 0 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval0cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 0 Cycle"]
    #[inline(always)]
    pub const fn set_cval0cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm3cval0cyc {
    #[inline(always)]
    fn default() -> Sm3cval0cyc {
        Sm3cval0cyc(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm3cval0cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3cval0cyc")
            .field("cval0cyc", &self.cval0cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3cval0cyc {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm3cval0cyc {
            cval0cyc: u8,
        }
        let proxy = Sm3cval0cyc {
            cval0cyc: self.cval0cyc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Value 1 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3cval1cyc(pub u16);
impl Sm3cval1cyc {
    #[doc = "Capture Value 1 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval1cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 1 Cycle"]
    #[inline(always)]
    pub const fn set_cval1cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm3cval1cyc {
    #[inline(always)]
    fn default() -> Sm3cval1cyc {
        Sm3cval1cyc(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm3cval1cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3cval1cyc")
            .field("cval1cyc", &self.cval1cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3cval1cyc {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm3cval1cyc {
            cval1cyc: u8,
        }
        let proxy = Sm3cval1cyc {
            cval1cyc: self.cval1cyc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Value 2 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3cval2cyc(pub u16);
impl Sm3cval2cyc {
    #[doc = "Capture Value 2 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval2cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 2 Cycle"]
    #[inline(always)]
    pub const fn set_cval2cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm3cval2cyc {
    #[inline(always)]
    fn default() -> Sm3cval2cyc {
        Sm3cval2cyc(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm3cval2cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3cval2cyc")
            .field("cval2cyc", &self.cval2cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3cval2cyc {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm3cval2cyc {
            cval2cyc: u8,
        }
        let proxy = Sm3cval2cyc {
            cval2cyc: self.cval2cyc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Value 3 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3cval3cyc(pub u16);
impl Sm3cval3cyc {
    #[doc = "Capture Value 3 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval3cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 3 Cycle"]
    #[inline(always)]
    pub const fn set_cval3cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm3cval3cyc {
    #[inline(always)]
    fn default() -> Sm3cval3cyc {
        Sm3cval3cyc(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm3cval3cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3cval3cyc")
            .field("cval3cyc", &self.cval3cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3cval3cyc {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm3cval3cyc {
            cval3cyc: u8,
        }
        let proxy = Sm3cval3cyc {
            cval3cyc: self.cval3cyc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Value 4 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3cval4cyc(pub u16);
impl Sm3cval4cyc {
    #[doc = "Capture Value 4 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval4cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 4 Cycle"]
    #[inline(always)]
    pub const fn set_cval4cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm3cval4cyc {
    #[inline(always)]
    fn default() -> Sm3cval4cyc {
        Sm3cval4cyc(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm3cval4cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3cval4cyc")
            .field("cval4cyc", &self.cval4cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3cval4cyc {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm3cval4cyc {
            cval4cyc: u8,
        }
        let proxy = Sm3cval4cyc {
            cval4cyc: self.cval4cyc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture Value 5 Cycle Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3cval5cyc(pub u16);
impl Sm3cval5cyc {
    #[doc = "Capture Value 5 Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn cval5cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 5 Cycle"]
    #[inline(always)]
    pub const fn set_cval5cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm3cval5cyc {
    #[inline(always)]
    fn default() -> Sm3cval5cyc {
        Sm3cval5cyc(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm3cval5cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3cval5cyc")
            .field("cval5cyc", &self.cval5cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3cval5cyc {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm3cval5cyc {
            cval5cyc: u8,
        }
        let proxy = Sm3cval5cyc {
            cval5cyc: self.cval5cyc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fault Disable Mapping Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3dismap0(pub u16);
impl Sm3dismap0 {
    #[doc = "PWM_A Fault Disable Mask 0"]
    #[must_use]
    #[inline(always)]
    pub const fn dis0a(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_A Fault Disable Mask 0"]
    #[inline(always)]
    pub const fn set_dis0a(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "PWM_B Fault Disable Mask 0"]
    #[must_use]
    #[inline(always)]
    pub const fn dis0b(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_B Fault Disable Mask 0"]
    #[inline(always)]
    pub const fn set_dis0b(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
    }
    #[doc = "PWM_X Fault Disable Mask 0"]
    #[must_use]
    #[inline(always)]
    pub const fn dis0x(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_X Fault Disable Mask 0"]
    #[inline(always)]
    pub const fn set_dis0x(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
}
impl Default for Sm3dismap0 {
    #[inline(always)]
    fn default() -> Sm3dismap0 {
        Sm3dismap0(65535u64 as u16)
    }
}
impl core::fmt::Debug for Sm3dismap0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3dismap0")
            .field("dis0a", &self.dis0a())
            .field("dis0b", &self.dis0b())
            .field("dis0x", &self.dis0x())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3dismap0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm3dismap0 {
            dis0a: u8,
            dis0b: u8,
            dis0x: u8,
        }
        let proxy = Sm3dismap0 {
            dis0a: self.dis0a(),
            dis0b: self.dis0b(),
            dis0x: self.dis0x(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "DMA Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3dmaen(pub u16);
impl Sm3dmaen {
    #[doc = "Capture X0 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cx0de(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X0 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_cx0de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Capture X1 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cx1de(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X1 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_cx1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Capture B0 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cb0de(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Capture B0 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_cb0de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Capture B1 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cb1de(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Capture B1 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_cb1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Capture A0 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ca0de(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Capture A0 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_ca0de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Capture A1 FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ca1de(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Capture A1 FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_ca1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Capture DMA Enable Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn captde(&self) -> super::vals::Sm3dmaenCaptde {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Sm3dmaenCaptde::from_bits(val as u8)
    }
    #[doc = "Capture DMA Enable Source Select"]
    #[inline(always)]
    pub const fn set_captde(&mut self, val: super::vals::Sm3dmaenCaptde) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "FIFO Watermark AND Control"]
    #[must_use]
    #[inline(always)]
    pub const fn fand(&self) -> super::vals::Sm3dmaenFand {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Sm3dmaenFand::from_bits(val as u8)
    }
    #[doc = "FIFO Watermark AND Control"]
    #[inline(always)]
    pub const fn set_fand(&mut self, val: super::vals::Sm3dmaenFand) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "Value Registers DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn valde(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Value Registers DMA Enable"]
    #[inline(always)]
    pub const fn set_valde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
}
impl Default for Sm3dmaen {
    #[inline(always)]
    fn default() -> Sm3dmaen {
        Sm3dmaen(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm3dmaen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3dmaen")
            .field("cx0de", &self.cx0de())
            .field("cx1de", &self.cx1de())
            .field("cb0de", &self.cb0de())
            .field("cb1de", &self.cb1de())
            .field("ca0de", &self.ca0de())
            .field("ca1de", &self.ca1de())
            .field("captde", &self.captde())
            .field("fand", &self.fand())
            .field("valde", &self.valde())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3dmaen {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm3dmaen {
            cx0de: bool,
            cx1de: bool,
            cb0de: bool,
            cb1de: bool,
            ca0de: bool,
            ca1de: bool,
            captde: super::vals::Sm3dmaenCaptde,
            fand: super::vals::Sm3dmaenFand,
            valde: bool,
        }
        let proxy = Sm3dmaen {
            cx0de: self.cx0de(),
            cx1de: self.cx1de(),
            cb0de: self.cb0de(),
            cb1de: self.cb1de(),
            ca0de: self.ca0de(),
            ca1de: self.ca1de(),
            captde: self.captde(),
            fand: self.fand(),
            valde: self.valde(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fractional Value Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3fracval1(pub u16);
impl Sm3fracval1 {
    #[doc = "Fractional Value 1"]
    #[must_use]
    #[inline(always)]
    pub const fn fracval1(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 1"]
    #[inline(always)]
    pub const fn set_fracval1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Sm3fracval1 {
    #[inline(always)]
    fn default() -> Sm3fracval1 {
        Sm3fracval1(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm3fracval1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3fracval1")
            .field("fracval1", &self.fracval1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3fracval1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm3fracval1 {
            fracval1: u8,
        }
        let proxy = Sm3fracval1 {
            fracval1: self.fracval1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fractional Value Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3fracval2(pub u16);
impl Sm3fracval2 {
    #[doc = "Fractional Value 2"]
    #[must_use]
    #[inline(always)]
    pub const fn fracval2(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 2"]
    #[inline(always)]
    pub const fn set_fracval2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Sm3fracval2 {
    #[inline(always)]
    fn default() -> Sm3fracval2 {
        Sm3fracval2(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm3fracval2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3fracval2")
            .field("fracval2", &self.fracval2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3fracval2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm3fracval2 {
            fracval2: u8,
        }
        let proxy = Sm3fracval2 {
            fracval2: self.fracval2(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fractional Value Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3fracval3(pub u16);
impl Sm3fracval3 {
    #[doc = "Fractional Value 3"]
    #[must_use]
    #[inline(always)]
    pub const fn fracval3(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 3"]
    #[inline(always)]
    pub const fn set_fracval3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Sm3fracval3 {
    #[inline(always)]
    fn default() -> Sm3fracval3 {
        Sm3fracval3(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm3fracval3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3fracval3")
            .field("fracval3", &self.fracval3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3fracval3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm3fracval3 {
            fracval3: u8,
        }
        let proxy = Sm3fracval3 {
            fracval3: self.fracval3(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fractional Value Register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3fracval4(pub u16);
impl Sm3fracval4 {
    #[doc = "Fractional Value 4"]
    #[must_use]
    #[inline(always)]
    pub const fn fracval4(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 4"]
    #[inline(always)]
    pub const fn set_fracval4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Sm3fracval4 {
    #[inline(always)]
    fn default() -> Sm3fracval4 {
        Sm3fracval4(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm3fracval4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3fracval4")
            .field("fracval4", &self.fracval4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3fracval4 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm3fracval4 {
            fracval4: u8,
        }
        let proxy = Sm3fracval4 {
            fracval4: self.fracval4(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fractional Value Register 5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3fracval5(pub u16);
impl Sm3fracval5 {
    #[doc = "Fractional Value 5"]
    #[must_use]
    #[inline(always)]
    pub const fn fracval5(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 5"]
    #[inline(always)]
    pub const fn set_fracval5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Sm3fracval5 {
    #[inline(always)]
    fn default() -> Sm3fracval5 {
        Sm3fracval5(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm3fracval5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3fracval5")
            .field("fracval5", &self.fracval5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3fracval5 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm3fracval5 {
            fracval5: u8,
        }
        let proxy = Sm3fracval5 {
            fracval5: self.fracval5(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fractional Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3frctrl(pub u16);
impl Sm3frctrl {
    #[doc = "Fractional Cycle PWM Period Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn frac1_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Fractional Cycle PWM Period Enable"]
    #[inline(always)]
    pub const fn set_frac1_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Fractional Cycle Placement Enable for PWM_A"]
    #[must_use]
    #[inline(always)]
    pub const fn frac23_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Fractional Cycle Placement Enable for PWM_A"]
    #[inline(always)]
    pub const fn set_frac23_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Fractional Cycle Placement Enable for PWM_B"]
    #[must_use]
    #[inline(always)]
    pub const fn frac45_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Fractional Cycle Placement Enable for PWM_B"]
    #[inline(always)]
    pub const fn set_frac45_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Test Status Bit"]
    #[must_use]
    #[inline(always)]
    pub const fn test(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Test Status Bit"]
    #[inline(always)]
    pub const fn set_test(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Sm3frctrl {
    #[inline(always)]
    fn default() -> Sm3frctrl {
        Sm3frctrl(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm3frctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3frctrl")
            .field("frac1_en", &self.frac1_en())
            .field("frac23_en", &self.frac23_en())
            .field("frac45_en", &self.frac45_en())
            .field("test", &self.test())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3frctrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm3frctrl {
            frac1_en: bool,
            frac23_en: bool,
            frac45_en: bool,
            test: bool,
        }
        let proxy = Sm3frctrl {
            frac1_en: self.frac1_en(),
            frac23_en: self.frac23_en(),
            frac45_en: self.frac45_en(),
            test: self.test(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3inten(pub u16);
impl Sm3inten {
    #[doc = "Compare Interrupt Enables"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpie(&self) -> super::vals::Sm3intenCmpie {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Sm3intenCmpie::from_bits(val as u8)
    }
    #[doc = "Compare Interrupt Enables"]
    #[inline(always)]
    pub const fn set_cmpie(&mut self, val: super::vals::Sm3intenCmpie) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Capture X 0 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cx0ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X 0 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cx0ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Capture X 1 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cx1ie(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cx1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture B 0 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cb0ie(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Capture B 0 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cb0ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "Capture B 1 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cb1ie(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Capture B 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cb1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "Capture A 0 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ca0ie(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Capture A 0 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ca0ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Capture A 1 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ca1ie(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Capture A 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ca1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Reload Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Interrupt Enable"]
    #[inline(always)]
    pub const fn set_rie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Reload Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn reie(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_reie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
}
impl Default for Sm3inten {
    #[inline(always)]
    fn default() -> Sm3inten {
        Sm3inten(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm3inten {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3inten")
            .field("cmpie", &self.cmpie())
            .field("cx0ie", &self.cx0ie())
            .field("cx1ie", &self.cx1ie())
            .field("cb0ie", &self.cb0ie())
            .field("cb1ie", &self.cb1ie())
            .field("ca0ie", &self.ca0ie())
            .field("ca1ie", &self.ca1ie())
            .field("rie", &self.rie())
            .field("reie", &self.reie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3inten {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm3inten {
            cmpie: super::vals::Sm3intenCmpie,
            cx0ie: bool,
            cx1ie: bool,
            cb0ie: bool,
            cb1ie: bool,
            ca0ie: bool,
            ca1ie: bool,
            rie: bool,
            reie: bool,
        }
        let proxy = Sm3inten {
            cmpie: self.cmpie(),
            cx0ie: self.cx0ie(),
            cx1ie: self.cx1ie(),
            cb0ie: self.cb0ie(),
            cb1ie: self.cb1ie(),
            ca0ie: self.ca0ie(),
            ca1ie: self.ca1ie(),
            rie: self.rie(),
            reie: self.reie(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Output Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3octrl(pub u16);
impl Sm3octrl {
    #[doc = "PWM_X Fault State"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmxfs(&self) -> super::vals::Sm3octrlPwmxfs {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sm3octrlPwmxfs::from_bits(val as u8)
    }
    #[doc = "PWM_X Fault State"]
    #[inline(always)]
    pub const fn set_pwmxfs(&mut self, val: super::vals::Sm3octrlPwmxfs) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "PWM_B Fault State"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmbfs(&self) -> super::vals::Sm3octrlPwmbfs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Sm3octrlPwmbfs::from_bits(val as u8)
    }
    #[doc = "PWM_B Fault State"]
    #[inline(always)]
    pub const fn set_pwmbfs(&mut self, val: super::vals::Sm3octrlPwmbfs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "PWM_A Fault State"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmafs(&self) -> super::vals::Sm3octrlPwmafs {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Sm3octrlPwmafs::from_bits(val as u8)
    }
    #[doc = "PWM_A Fault State"]
    #[inline(always)]
    pub const fn set_pwmafs(&mut self, val: super::vals::Sm3octrlPwmafs) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "PWM_X Output Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn polx(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Output Polarity"]
    #[inline(always)]
    pub const fn set_polx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "PWM_B Output Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn polb(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_B Output Polarity"]
    #[inline(always)]
    pub const fn set_polb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "PWM_A Output Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn pola(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_A Output Polarity"]
    #[inline(always)]
    pub const fn set_pola(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "PWM_X Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmx_in(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Input"]
    #[inline(always)]
    pub const fn set_pwmx_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "PWM_B Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pwmb_in(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_B Input"]
    #[inline(always)]
    pub const fn set_pwmb_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "PWM_A Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pwma_in(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_A Input"]
    #[inline(always)]
    pub const fn set_pwma_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Sm3octrl {
    #[inline(always)]
    fn default() -> Sm3octrl {
        Sm3octrl(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm3octrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3octrl")
            .field("pwmxfs", &self.pwmxfs())
            .field("pwmbfs", &self.pwmbfs())
            .field("pwmafs", &self.pwmafs())
            .field("polx", &self.polx())
            .field("polb", &self.polb())
            .field("pola", &self.pola())
            .field("pwmx_in", &self.pwmx_in())
            .field("pwmb_in", &self.pwmb_in())
            .field("pwma_in", &self.pwma_in())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3octrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm3octrl {
            pwmxfs: super::vals::Sm3octrlPwmxfs,
            pwmbfs: super::vals::Sm3octrlPwmbfs,
            pwmafs: super::vals::Sm3octrlPwmafs,
            polx: bool,
            polb: bool,
            pola: bool,
            pwmx_in: bool,
            pwmb_in: bool,
            pwma_in: bool,
        }
        let proxy = Sm3octrl {
            pwmxfs: self.pwmxfs(),
            pwmbfs: self.pwmbfs(),
            pwmafs: self.pwmafs(),
            polx: self.polx(),
            polb: self.polb(),
            pola: self.pola(),
            pwmx_in: self.pwmx_in(),
            pwmb_in: self.pwmb_in(),
            pwma_in: self.pwma_in(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3sts(pub u16);
impl Sm3sts {
    #[doc = "Compare Flags"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpf(&self) -> super::vals::Sm3stsCmpf {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Sm3stsCmpf::from_bits(val as u8)
    }
    #[doc = "Compare Flags"]
    #[inline(always)]
    pub const fn set_cmpf(&mut self, val: super::vals::Sm3stsCmpf) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Capture Flag X0"]
    #[must_use]
    #[inline(always)]
    pub const fn cfx0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag X0"]
    #[inline(always)]
    pub const fn set_cfx0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Capture Flag X1"]
    #[must_use]
    #[inline(always)]
    pub const fn cfx1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag X1"]
    #[inline(always)]
    pub const fn set_cfx1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture Flag B0"]
    #[must_use]
    #[inline(always)]
    pub const fn cfb0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag B0"]
    #[inline(always)]
    pub const fn set_cfb0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "Capture Flag B1"]
    #[must_use]
    #[inline(always)]
    pub const fn cfb1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag B1"]
    #[inline(always)]
    pub const fn set_cfb1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "Capture Flag A0"]
    #[must_use]
    #[inline(always)]
    pub const fn cfa0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag A0"]
    #[inline(always)]
    pub const fn set_cfa0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Capture Flag A1"]
    #[must_use]
    #[inline(always)]
    pub const fn cfa1(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag A1"]
    #[inline(always)]
    pub const fn set_cfa1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Reload Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rf(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Flag"]
    #[inline(always)]
    pub const fn set_rf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Reload Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ref_(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Error Flag"]
    #[inline(always)]
    pub const fn set_ref_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Registers Updated Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ruf(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Registers Updated Flag"]
    #[inline(always)]
    pub const fn set_ruf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
}
impl Default for Sm3sts {
    #[inline(always)]
    fn default() -> Sm3sts {
        Sm3sts(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm3sts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3sts")
            .field("cmpf", &self.cmpf())
            .field("cfx0", &self.cfx0())
            .field("cfx1", &self.cfx1())
            .field("cfb0", &self.cfb0())
            .field("cfb1", &self.cfb1())
            .field("cfa0", &self.cfa0())
            .field("cfa1", &self.cfa1())
            .field("rf", &self.rf())
            .field("ref_", &self.ref_())
            .field("ruf", &self.ruf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3sts {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm3sts {
            cmpf: super::vals::Sm3stsCmpf,
            cfx0: bool,
            cfx1: bool,
            cfb0: bool,
            cfb1: bool,
            cfa0: bool,
            cfa1: bool,
            rf: bool,
            ref_: bool,
            ruf: bool,
        }
        let proxy = Sm3sts {
            cmpf: self.cmpf(),
            cfx0: self.cfx0(),
            cfx1: self.cfx1(),
            cfb0: self.cfb0(),
            cfb1: self.cfb1(),
            cfa0: self.cfa0(),
            cfa1: self.cfa1(),
            rf: self.rf(),
            ref_: self.ref_(),
            ruf: self.ruf(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Output Trigger Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3tctrl(pub u16);
impl Sm3tctrl {
    #[doc = "Output Trigger Enables"]
    #[must_use]
    #[inline(always)]
    pub const fn out_trig_en(&self) -> super::vals::Sm3tctrlOutTrigEn {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Sm3tctrlOutTrigEn::from_bits(val as u8)
    }
    #[doc = "Output Trigger Enables"]
    #[inline(always)]
    pub const fn set_out_trig_en(&mut self, val: super::vals::Sm3tctrlOutTrigEn) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Trigger Frequency"]
    #[must_use]
    #[inline(always)]
    pub const fn trgfrq(&self) -> super::vals::Sm3tctrlTrgfrq {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Sm3tctrlTrgfrq::from_bits(val as u8)
    }
    #[doc = "Trigger Frequency"]
    #[inline(always)]
    pub const fn set_trgfrq(&mut self, val: super::vals::Sm3tctrlTrgfrq) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
    #[doc = "Mux Output Trigger 1 Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn pwbot1(&self) -> super::vals::Sm3tctrlPwbot1 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Sm3tctrlPwbot1::from_bits(val as u8)
    }
    #[doc = "Mux Output Trigger 1 Source Select"]
    #[inline(always)]
    pub const fn set_pwbot1(&mut self, val: super::vals::Sm3tctrlPwbot1) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "Mux Output Trigger 0 Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn pwaot0(&self) -> super::vals::Sm3tctrlPwaot0 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Sm3tctrlPwaot0::from_bits(val as u8)
    }
    #[doc = "Mux Output Trigger 0 Source Select"]
    #[inline(always)]
    pub const fn set_pwaot0(&mut self, val: super::vals::Sm3tctrlPwaot0) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
    }
}
impl Default for Sm3tctrl {
    #[inline(always)]
    fn default() -> Sm3tctrl {
        Sm3tctrl(0u64 as u16)
    }
}
impl core::fmt::Debug for Sm3tctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3tctrl")
            .field("out_trig_en", &self.out_trig_en())
            .field("trgfrq", &self.trgfrq())
            .field("pwbot1", &self.pwbot1())
            .field("pwaot0", &self.pwaot0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3tctrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sm3tctrl {
            out_trig_en: super::vals::Sm3tctrlOutTrigEn,
            trgfrq: super::vals::Sm3tctrlTrgfrq,
            pwbot1: super::vals::Sm3tctrlPwbot1,
            pwaot0: super::vals::Sm3tctrlPwaot0,
        }
        let proxy = Sm3tctrl {
            out_trig_en: self.out_trig_en(),
            trgfrq: self.trgfrq(),
            pwbot1: self.pwbot1(),
            pwaot0: self.pwaot0(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Software Controlled Output Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swcout(pub u16);
impl Swcout {
    #[doc = "Submodule 0 Software Controlled Output 45"]
    #[must_use]
    #[inline(always)]
    pub const fn sm0out45(&self) -> super::vals::Sm0out45 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sm0out45::from_bits(val as u8)
    }
    #[doc = "Submodule 0 Software Controlled Output 45"]
    #[inline(always)]
    pub const fn set_sm0out45(&mut self, val: super::vals::Sm0out45) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "Submodule 0 Software Controlled Output 23"]
    #[must_use]
    #[inline(always)]
    pub const fn sm0out23(&self) -> super::vals::Sm0out23 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sm0out23::from_bits(val as u8)
    }
    #[doc = "Submodule 0 Software Controlled Output 23"]
    #[inline(always)]
    pub const fn set_sm0out23(&mut self, val: super::vals::Sm0out23) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Submodule 1 Software Controlled Output 45"]
    #[must_use]
    #[inline(always)]
    pub const fn sm1out45(&self) -> super::vals::Sm1out45 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Sm1out45::from_bits(val as u8)
    }
    #[doc = "Submodule 1 Software Controlled Output 45"]
    #[inline(always)]
    pub const fn set_sm1out45(&mut self, val: super::vals::Sm1out45) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Submodule 1 Software Controlled Output 23"]
    #[must_use]
    #[inline(always)]
    pub const fn sm1out23(&self) -> super::vals::Sm1out23 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Sm1out23::from_bits(val as u8)
    }
    #[doc = "Submodule 1 Software Controlled Output 23"]
    #[inline(always)]
    pub const fn set_sm1out23(&mut self, val: super::vals::Sm1out23) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
    #[doc = "Submodule 2 Software Controlled Output 45"]
    #[must_use]
    #[inline(always)]
    pub const fn sm2out45(&self) -> super::vals::Sm2out45 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Sm2out45::from_bits(val as u8)
    }
    #[doc = "Submodule 2 Software Controlled Output 45"]
    #[inline(always)]
    pub const fn set_sm2out45(&mut self, val: super::vals::Sm2out45) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Submodule 2 Software Controlled Output 23"]
    #[must_use]
    #[inline(always)]
    pub const fn sm2out23(&self) -> super::vals::Sm2out23 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Sm2out23::from_bits(val as u8)
    }
    #[doc = "Submodule 2 Software Controlled Output 23"]
    #[inline(always)]
    pub const fn set_sm2out23(&mut self, val: super::vals::Sm2out23) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
    }
    #[doc = "Submodule 3 Software Controlled Output 45"]
    #[must_use]
    #[inline(always)]
    pub const fn sm3out45(&self) -> super::vals::Sm3out45 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Sm3out45::from_bits(val as u8)
    }
    #[doc = "Submodule 3 Software Controlled Output 45"]
    #[inline(always)]
    pub const fn set_sm3out45(&mut self, val: super::vals::Sm3out45) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Submodule 3 Software Controlled Output 23"]
    #[must_use]
    #[inline(always)]
    pub const fn sm3out23(&self) -> super::vals::Sm3out23 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Sm3out23::from_bits(val as u8)
    }
    #[doc = "Submodule 3 Software Controlled Output 23"]
    #[inline(always)]
    pub const fn set_sm3out23(&mut self, val: super::vals::Sm3out23) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
}
impl Default for Swcout {
    #[inline(always)]
    fn default() -> Swcout {
        Swcout(0u64 as u16)
    }
}
impl core::fmt::Debug for Swcout {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Swcout")
            .field("sm0out45", &self.sm0out45())
            .field("sm0out23", &self.sm0out23())
            .field("sm1out45", &self.sm1out45())
            .field("sm1out23", &self.sm1out23())
            .field("sm2out45", &self.sm2out45())
            .field("sm2out23", &self.sm2out23())
            .field("sm3out45", &self.sm3out45())
            .field("sm3out23", &self.sm3out23())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Swcout {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Swcout {
            sm0out45: super::vals::Sm0out45,
            sm0out23: super::vals::Sm0out23,
            sm1out45: super::vals::Sm1out45,
            sm1out23: super::vals::Sm1out23,
            sm2out45: super::vals::Sm2out45,
            sm2out23: super::vals::Sm2out23,
            sm3out45: super::vals::Sm3out45,
            sm3out23: super::vals::Sm3out23,
        }
        let proxy = Swcout {
            sm0out45: self.sm0out45(),
            sm0out23: self.sm0out23(),
            sm1out45: self.sm1out45(),
            sm1out23: self.sm1out23(),
            sm2out45: self.sm2out45(),
            sm2out23: self.sm2out23(),
            sm3out45: self.sm3out45(),
            sm3out23: self.sm3out23(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
