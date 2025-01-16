#[doc = "Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc = "GPT Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "GPT Enable"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "GPT Enable Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn enmod(&self) -> super::vals::Enmod {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Enmod::from_bits(val as u8)
    }
    #[doc = "GPT Enable Mode"]
    #[inline(always)]
    pub const fn set_enmod(&mut self, val: super::vals::Enmod) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "GPT Debug Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dbgen(&self) -> super::vals::Dbgen {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Dbgen::from_bits(val as u8)
    }
    #[doc = "GPT Debug Mode Enable"]
    #[inline(always)]
    pub const fn set_dbgen(&mut self, val: super::vals::Dbgen) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "GPT Wait Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn waiten(&self) -> super::vals::Waiten {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Waiten::from_bits(val as u8)
    }
    #[doc = "GPT Wait Mode Enable"]
    #[inline(always)]
    pub const fn set_waiten(&mut self, val: super::vals::Waiten) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "GPT Doze Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dozeen(&self) -> super::vals::Dozeen {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Dozeen::from_bits(val as u8)
    }
    #[doc = "GPT Doze Mode Enable"]
    #[inline(always)]
    pub const fn set_dozeen(&mut self, val: super::vals::Dozeen) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "GPT Stop Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn stopen(&self) -> super::vals::Stopen {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Stopen::from_bits(val as u8)
    }
    #[doc = "GPT Stop Mode Enable"]
    #[inline(always)]
    pub const fn set_stopen(&mut self, val: super::vals::Stopen) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Clock Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn clksrc(&self) -> super::vals::Clksrc {
        let val = (self.0 >> 6usize) & 0x07;
        super::vals::Clksrc::from_bits(val as u8)
    }
    #[doc = "Clock Source Select"]
    #[inline(always)]
    pub const fn set_clksrc(&mut self, val: super::vals::Clksrc) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val.to_bits() as u32) & 0x07) << 6usize);
    }
    #[doc = "Free-Run or Restart Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn frr(&self) -> super::vals::Frr {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Frr::from_bits(val as u8)
    }
    #[doc = "Free-Run or Restart Mode"]
    #[inline(always)]
    pub const fn set_frr(&mut self, val: super::vals::Frr) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Enable Oscillator Clock Input"]
    #[must_use]
    #[inline(always)]
    pub const fn en_24m(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Oscillator Clock Input"]
    #[inline(always)]
    pub const fn set_en_24m(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Software Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn swr(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn set_swr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Input Capture Operating Mode for Channel 1"]
    #[must_use]
    #[inline(always)]
    pub const fn im1(&self) -> super::vals::Im1 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Im1::from_bits(val as u8)
    }
    #[doc = "Input Capture Operating Mode for Channel 1"]
    #[inline(always)]
    pub const fn set_im1(&mut self, val: super::vals::Im1) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Input Capture Operating Mode for Channel 2"]
    #[must_use]
    #[inline(always)]
    pub const fn im2(&self) -> super::vals::Im2 {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::Im2::from_bits(val as u8)
    }
    #[doc = "Input Capture Operating Mode for Channel 2"]
    #[inline(always)]
    pub const fn set_im2(&mut self, val: super::vals::Im2) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Output Compare Operating Mode for Channel 1"]
    #[must_use]
    #[inline(always)]
    pub const fn om1(&self) -> super::vals::Om1 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Om1::from_bits(val as u8)
    }
    #[doc = "Output Compare Operating Mode for Channel 1"]
    #[inline(always)]
    pub const fn set_om1(&mut self, val: super::vals::Om1) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "Output Compare Operating Mode for Channel 2"]
    #[must_use]
    #[inline(always)]
    pub const fn om2(&self) -> super::vals::Om2 {
        let val = (self.0 >> 23usize) & 0x07;
        super::vals::Om2::from_bits(val as u8)
    }
    #[doc = "Output Compare Operating Mode for Channel 2"]
    #[inline(always)]
    pub const fn set_om2(&mut self, val: super::vals::Om2) {
        self.0 = (self.0 & !(0x07 << 23usize)) | (((val.to_bits() as u32) & 0x07) << 23usize);
    }
    #[doc = "Output Compare Operating Mode for Channel 3"]
    #[must_use]
    #[inline(always)]
    pub const fn om3(&self) -> super::vals::Om3 {
        let val = (self.0 >> 26usize) & 0x07;
        super::vals::Om3::from_bits(val as u8)
    }
    #[doc = "Output Compare Operating Mode for Channel 3"]
    #[inline(always)]
    pub const fn set_om3(&mut self, val: super::vals::Om3) {
        self.0 = (self.0 & !(0x07 << 26usize)) | (((val.to_bits() as u32) & 0x07) << 26usize);
    }
    #[doc = "Force Output Compare for Channel 1"]
    #[must_use]
    #[inline(always)]
    pub const fn fo1(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Force Output Compare for Channel 1"]
    #[inline(always)]
    pub const fn set_fo1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Force Output Compare for Channel 2"]
    #[must_use]
    #[inline(always)]
    pub const fn fo2(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Force Output Compare for Channel 2"]
    #[inline(always)]
    pub const fn set_fo2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Force Output Compare for Channel 3"]
    #[must_use]
    #[inline(always)]
    pub const fn fo3(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Force Output Compare for Channel 3"]
    #[inline(always)]
    pub const fn set_fo3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Cr {
    #[inline(always)]
    fn default() -> Cr {
        Cr(0u64 as u32)
    }
}
impl core::fmt::Debug for Cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr")
            .field("en", &self.en())
            .field("enmod", &self.enmod())
            .field("dbgen", &self.dbgen())
            .field("waiten", &self.waiten())
            .field("dozeen", &self.dozeen())
            .field("stopen", &self.stopen())
            .field("clksrc", &self.clksrc())
            .field("frr", &self.frr())
            .field("en_24m", &self.en_24m())
            .field("swr", &self.swr())
            .field("im1", &self.im1())
            .field("im2", &self.im2())
            .field("om1", &self.om1())
            .field("om2", &self.om2())
            .field("om3", &self.om3())
            .field("fo1", &self.fo1())
            .field("fo2", &self.fo2())
            .field("fo3", &self.fo3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cr {
            en: bool,
            enmod: super::vals::Enmod,
            dbgen: super::vals::Dbgen,
            waiten: super::vals::Waiten,
            dozeen: super::vals::Dozeen,
            stopen: super::vals::Stopen,
            clksrc: super::vals::Clksrc,
            frr: super::vals::Frr,
            en_24m: bool,
            swr: bool,
            im1: super::vals::Im1,
            im2: super::vals::Im2,
            om1: super::vals::Om1,
            om2: super::vals::Om2,
            om3: super::vals::Om3,
            fo1: bool,
            fo2: bool,
            fo3: bool,
        }
        let proxy = Cr {
            en: self.en(),
            enmod: self.enmod(),
            dbgen: self.dbgen(),
            waiten: self.waiten(),
            dozeen: self.dozeen(),
            stopen: self.stopen(),
            clksrc: self.clksrc(),
            frr: self.frr(),
            en_24m: self.en_24m(),
            swr: self.swr(),
            im1: self.im1(),
            im2: self.im2(),
            om1: self.om1(),
            om2: self.om2(),
            om3: self.om3(),
            fo1: self.fo1(),
            fo2: self.fo2(),
            fo3: self.fo3(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ir(pub u32);
impl Ir {
    #[doc = "Output Compare Flag for Channel 1 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn of1ie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Output Compare Flag for Channel 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_of1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Output Compare Flag for Channel 2 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn of2ie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Output Compare Flag for Channel 2 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_of2ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Output Compare Flag for Channel 3 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn of3ie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Output Compare Flag for Channel 3 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_of3ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Input Capture Flag for Channel 1 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn if1ie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Input Capture Flag for Channel 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_if1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Input Capture Flag for Channel 2 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn if2ie(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Input Capture Flag for Channel 2 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_if2ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Rollover Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rovie(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Rollover Interrupt Enable"]
    #[inline(always)]
    pub const fn set_rovie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Ir {
    #[inline(always)]
    fn default() -> Ir {
        Ir(0u64 as u32)
    }
}
impl core::fmt::Debug for Ir {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ir")
            .field("of1ie", &self.of1ie())
            .field("of2ie", &self.of2ie())
            .field("of3ie", &self.of3ie())
            .field("if1ie", &self.if1ie())
            .field("if2ie", &self.if2ie())
            .field("rovie", &self.rovie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ir {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ir {
            of1ie: bool,
            of2ie: bool,
            of3ie: bool,
            if1ie: bool,
            if2ie: bool,
            rovie: bool,
        }
        let proxy = Ir {
            of1ie: self.of1ie(),
            of2ie: self.of2ie(),
            of3ie: self.of3ie(),
            if1ie: self.if1ie(),
            if2ie: self.if2ie(),
            rovie: self.rovie(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Prescaler"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pr(pub u32);
impl Pr {
    #[doc = "Prescaler Divide Value"]
    #[must_use]
    #[inline(always)]
    pub const fn prescaler(&self) -> super::vals::Prescaler {
        let val = (self.0 >> 0usize) & 0x0fff;
        super::vals::Prescaler::from_bits(val as u16)
    }
    #[doc = "Prescaler Divide Value"]
    #[inline(always)]
    pub const fn set_prescaler(&mut self, val: super::vals::Prescaler) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val.to_bits() as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Prescaler Divide Value for the Oscillator Clock"]
    #[must_use]
    #[inline(always)]
    pub const fn prescaler24m(&self) -> super::vals::Prescaler24m {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Prescaler24m::from_bits(val as u8)
    }
    #[doc = "Prescaler Divide Value for the Oscillator Clock"]
    #[inline(always)]
    pub const fn set_prescaler24m(&mut self, val: super::vals::Prescaler24m) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
}
impl Default for Pr {
    #[inline(always)]
    fn default() -> Pr {
        Pr(0u64 as u32)
    }
}
impl core::fmt::Debug for Pr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pr")
            .field("prescaler", &self.prescaler())
            .field("prescaler24m", &self.prescaler24m())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pr {
            prescaler: super::vals::Prescaler,
            prescaler24m: super::vals::Prescaler24m,
        }
        let proxy = Pr {
            prescaler: self.prescaler(),
            prescaler24m: self.prescaler24m(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "Output Compare Flag for Channel 1"]
    #[must_use]
    #[inline(always)]
    pub const fn of1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Output Compare Flag for Channel 1"]
    #[inline(always)]
    pub const fn set_of1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Output Compare Flag for Channel 2"]
    #[must_use]
    #[inline(always)]
    pub const fn of2(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Output Compare Flag for Channel 2"]
    #[inline(always)]
    pub const fn set_of2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Output Compare Flag for Channel 3"]
    #[must_use]
    #[inline(always)]
    pub const fn of3(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Output Compare Flag for Channel 3"]
    #[inline(always)]
    pub const fn set_of3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Input Capture Flag for Channel 1"]
    #[must_use]
    #[inline(always)]
    pub const fn if1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Input Capture Flag for Channel 1"]
    #[inline(always)]
    pub const fn set_if1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Input Capture Flag for Channel 2"]
    #[must_use]
    #[inline(always)]
    pub const fn if2(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Input Capture Flag for Channel 2"]
    #[inline(always)]
    pub const fn set_if2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Rollover Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rov(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Rollover Flag"]
    #[inline(always)]
    pub const fn set_rov(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Sr {
    #[inline(always)]
    fn default() -> Sr {
        Sr(0u64 as u32)
    }
}
impl core::fmt::Debug for Sr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sr")
            .field("of1", &self.of1())
            .field("of2", &self.of2())
            .field("of3", &self.of3())
            .field("if1", &self.if1())
            .field("if2", &self.if2())
            .field("rov", &self.rov())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sr {
            of1: bool,
            of2: bool,
            of3: bool,
            if1: bool,
            if2: bool,
            rov: bool,
        }
        let proxy = Sr {
            of1: self.of1(),
            of2: self.of2(),
            of3: self.of3(),
            if1: self.if1(),
            if2: self.if2(),
            rov: self.rov(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
