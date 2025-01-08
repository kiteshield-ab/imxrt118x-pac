#[doc = "Control 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl1(pub u32);
impl Ctrl1 {
    #[doc = "Temperature Measurement Frequency"]
    #[must_use]
    #[inline(always)]
    pub const fn freq(&self) -> super::vals::Freq {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Freq::from_bits(val as u16)
    }
    #[doc = "Temperature Measurement Frequency"]
    #[inline(always)]
    pub const fn set_freq(&mut self, val: super::vals::Freq) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
    #[doc = "Measurement Finished Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn finish_ie(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Measurement Finished Interrupt Enable"]
    #[inline(always)]
    pub const fn set_finish_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Low Temperature Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn low_temp_ie(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Low Temperature Interrupt Enable"]
    #[inline(always)]
    pub const fn set_low_temp_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "High Temperature Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn high_temp_ie(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "High Temperature Interrupt Enable"]
    #[inline(always)]
    pub const fn set_high_temp_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Panic Temperature Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn panic_temp_ie(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Panic Temperature Interrupt Enable"]
    #[inline(always)]
    pub const fn set_panic_temp_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Start Temperature Measurement"]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> super::vals::Start {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Start::from_bits(val as u8)
    }
    #[doc = "Start Temperature Measurement"]
    #[inline(always)]
    pub const fn set_start(&mut self, val: super::vals::Start) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Power Down Except Bias Current"]
    #[must_use]
    #[inline(always)]
    pub const fn pwd(&self) -> super::vals::Pwd {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Pwd::from_bits(val as u8)
    }
    #[doc = "Power Down Except Bias Current"]
    #[inline(always)]
    pub const fn set_pwd(&mut self, val: super::vals::Pwd) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Power Down"]
    #[must_use]
    #[inline(always)]
    pub const fn pwd_full(&self) -> super::vals::PwdFull {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PwdFull::from_bits(val as u8)
    }
    #[doc = "Power Down"]
    #[inline(always)]
    pub const fn set_pwd_full(&mut self, val: super::vals::PwdFull) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctrl1 {
    #[inline(always)]
    fn default() -> Ctrl1 {
        Ctrl1(2155872256u64 as u32)
    }
}
impl core::fmt::Debug for Ctrl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl1")
            .field("freq", &self.freq())
            .field("finish_ie", &self.finish_ie())
            .field("low_temp_ie", &self.low_temp_ie())
            .field("high_temp_ie", &self.high_temp_ie())
            .field("panic_temp_ie", &self.panic_temp_ie())
            .field("start", &self.start())
            .field("pwd", &self.pwd())
            .field("pwd_full", &self.pwd_full())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ctrl1 {
            freq: super::vals::Freq,
            finish_ie: bool,
            low_temp_ie: bool,
            high_temp_ie: bool,
            panic_temp_ie: bool,
            start: super::vals::Start,
            pwd: super::vals::Pwd,
            pwd_full: super::vals::PwdFull,
        }
        let proxy = Ctrl1 {
            freq: self.freq(),
            finish_ie: self.finish_ie(),
            low_temp_ie: self.low_temp_ie(),
            high_temp_ie: self.high_temp_ie(),
            panic_temp_ie: self.panic_temp_ie(),
            start: self.start(),
            pwd: self.pwd(),
            pwd_full: self.pwd_full(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Control 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl1Clr(pub u32);
impl Ctrl1Clr {
    #[doc = "Temperature Measurement Frequency"]
    #[must_use]
    #[inline(always)]
    pub const fn freq(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Temperature Measurement Frequency"]
    #[inline(always)]
    pub const fn set_freq(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Measurement Finished Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn finish_ie(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Measurement Finished Interrupt Enable"]
    #[inline(always)]
    pub const fn set_finish_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Low Temperature Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn low_temp_ie(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Low Temperature Interrupt Enable"]
    #[inline(always)]
    pub const fn set_low_temp_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "High Temperature Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn high_temp_ie(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "High Temperature Interrupt Enable"]
    #[inline(always)]
    pub const fn set_high_temp_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Panic Temperature Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn panic_temp_ie(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Panic Temperature Interrupt Enable"]
    #[inline(always)]
    pub const fn set_panic_temp_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Start Temperature Measurement"]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Start Temperature Measurement"]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Power Down Except Bias Current"]
    #[must_use]
    #[inline(always)]
    pub const fn pwd(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down Except Bias Current"]
    #[inline(always)]
    pub const fn set_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Power Down"]
    #[must_use]
    #[inline(always)]
    pub const fn pwd_full(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down"]
    #[inline(always)]
    pub const fn set_pwd_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctrl1Clr {
    #[inline(always)]
    fn default() -> Ctrl1Clr {
        Ctrl1Clr(2155872256u64 as u32)
    }
}
impl core::fmt::Debug for Ctrl1Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl1Clr")
            .field("freq", &self.freq())
            .field("finish_ie", &self.finish_ie())
            .field("low_temp_ie", &self.low_temp_ie())
            .field("high_temp_ie", &self.high_temp_ie())
            .field("panic_temp_ie", &self.panic_temp_ie())
            .field("start", &self.start())
            .field("pwd", &self.pwd())
            .field("pwd_full", &self.pwd_full())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl1Clr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ctrl1Clr {
            freq: u16,
            finish_ie: bool,
            low_temp_ie: bool,
            high_temp_ie: bool,
            panic_temp_ie: bool,
            start: bool,
            pwd: bool,
            pwd_full: bool,
        }
        let proxy = Ctrl1Clr {
            freq: self.freq(),
            finish_ie: self.finish_ie(),
            low_temp_ie: self.low_temp_ie(),
            high_temp_ie: self.high_temp_ie(),
            panic_temp_ie: self.panic_temp_ie(),
            start: self.start(),
            pwd: self.pwd(),
            pwd_full: self.pwd_full(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Control 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl1Set(pub u32);
impl Ctrl1Set {
    #[doc = "Temperature Measurement Frequency"]
    #[must_use]
    #[inline(always)]
    pub const fn freq(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Temperature Measurement Frequency"]
    #[inline(always)]
    pub const fn set_freq(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Measurement Finished Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn finish_ie(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Measurement Finished Interrupt Enable"]
    #[inline(always)]
    pub const fn set_finish_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Low Temperature Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn low_temp_ie(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Low Temperature Interrupt Enable"]
    #[inline(always)]
    pub const fn set_low_temp_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "High Temperature Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn high_temp_ie(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "High Temperature Interrupt Enable"]
    #[inline(always)]
    pub const fn set_high_temp_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Panic Temperature Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn panic_temp_ie(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Panic Temperature Interrupt Enable"]
    #[inline(always)]
    pub const fn set_panic_temp_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Start Temperature Measurement"]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Start Temperature Measurement"]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Power Down Except Bias Current"]
    #[must_use]
    #[inline(always)]
    pub const fn pwd(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down Except Bias Current"]
    #[inline(always)]
    pub const fn set_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Power Down"]
    #[must_use]
    #[inline(always)]
    pub const fn pwd_full(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down"]
    #[inline(always)]
    pub const fn set_pwd_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctrl1Set {
    #[inline(always)]
    fn default() -> Ctrl1Set {
        Ctrl1Set(2155872256u64 as u32)
    }
}
impl core::fmt::Debug for Ctrl1Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl1Set")
            .field("freq", &self.freq())
            .field("finish_ie", &self.finish_ie())
            .field("low_temp_ie", &self.low_temp_ie())
            .field("high_temp_ie", &self.high_temp_ie())
            .field("panic_temp_ie", &self.panic_temp_ie())
            .field("start", &self.start())
            .field("pwd", &self.pwd())
            .field("pwd_full", &self.pwd_full())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl1Set {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ctrl1Set {
            freq: u16,
            finish_ie: bool,
            low_temp_ie: bool,
            high_temp_ie: bool,
            panic_temp_ie: bool,
            start: bool,
            pwd: bool,
            pwd_full: bool,
        }
        let proxy = Ctrl1Set {
            freq: self.freq(),
            finish_ie: self.finish_ie(),
            low_temp_ie: self.low_temp_ie(),
            high_temp_ie: self.high_temp_ie(),
            panic_temp_ie: self.panic_temp_ie(),
            start: self.start(),
            pwd: self.pwd(),
            pwd_full: self.pwd_full(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Control 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl1Tog(pub u32);
impl Ctrl1Tog {
    #[doc = "Temperature Measurement Frequency"]
    #[must_use]
    #[inline(always)]
    pub const fn freq(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Temperature Measurement Frequency"]
    #[inline(always)]
    pub const fn set_freq(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Measurement Finished Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn finish_ie(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Measurement Finished Interrupt Enable"]
    #[inline(always)]
    pub const fn set_finish_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Low Temperature Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn low_temp_ie(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Low Temperature Interrupt Enable"]
    #[inline(always)]
    pub const fn set_low_temp_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "High Temperature Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn high_temp_ie(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "High Temperature Interrupt Enable"]
    #[inline(always)]
    pub const fn set_high_temp_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Panic Temperature Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn panic_temp_ie(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Panic Temperature Interrupt Enable"]
    #[inline(always)]
    pub const fn set_panic_temp_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Start Temperature Measurement"]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Start Temperature Measurement"]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Power Down Except Bias Current"]
    #[must_use]
    #[inline(always)]
    pub const fn pwd(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down Except Bias Current"]
    #[inline(always)]
    pub const fn set_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Power Down"]
    #[must_use]
    #[inline(always)]
    pub const fn pwd_full(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down"]
    #[inline(always)]
    pub const fn set_pwd_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctrl1Tog {
    #[inline(always)]
    fn default() -> Ctrl1Tog {
        Ctrl1Tog(2155872256u64 as u32)
    }
}
impl core::fmt::Debug for Ctrl1Tog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl1Tog")
            .field("freq", &self.freq())
            .field("finish_ie", &self.finish_ie())
            .field("low_temp_ie", &self.low_temp_ie())
            .field("high_temp_ie", &self.high_temp_ie())
            .field("panic_temp_ie", &self.panic_temp_ie())
            .field("start", &self.start())
            .field("pwd", &self.pwd())
            .field("pwd_full", &self.pwd_full())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl1Tog {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ctrl1Tog {
            freq: u16,
            finish_ie: bool,
            low_temp_ie: bool,
            high_temp_ie: bool,
            panic_temp_ie: bool,
            start: bool,
            pwd: bool,
            pwd_full: bool,
        }
        let proxy = Ctrl1Tog {
            freq: self.freq(),
            finish_ie: self.finish_ie(),
            low_temp_ie: self.low_temp_ie(),
            high_temp_ie: self.high_temp_ie(),
            panic_temp_ie: self.panic_temp_ie(),
            start: self.start(),
            pwd: self.pwd(),
            pwd_full: self.pwd_full(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Range 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Range0(pub u32);
impl Range0 {
    #[doc = "Low Temperature Threshold Value"]
    #[must_use]
    #[inline(always)]
    pub const fn low_temp_val(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Low Temperature Threshold Value"]
    #[inline(always)]
    pub const fn set_low_temp_val(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "High Temperature Threshold Value"]
    #[must_use]
    #[inline(always)]
    pub const fn high_temp_val(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "High Temperature Threshold Value"]
    #[inline(always)]
    pub const fn set_high_temp_val(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Range0 {
    #[inline(always)]
    fn default() -> Range0 {
        Range0(0u64 as u32)
    }
}
impl core::fmt::Debug for Range0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Range0")
            .field("low_temp_val", &self.low_temp_val())
            .field("high_temp_val", &self.high_temp_val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Range0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Range0 {
            low_temp_val: u16,
            high_temp_val: u16,
        }
        let proxy = Range0 {
            low_temp_val: self.low_temp_val(),
            high_temp_val: self.high_temp_val(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Range 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Range0Clr(pub u32);
impl Range0Clr {
    #[doc = "Low Temperature Threshold Value"]
    #[must_use]
    #[inline(always)]
    pub const fn low_temp_val(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Low Temperature Threshold Value"]
    #[inline(always)]
    pub const fn set_low_temp_val(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "High Temperature Threshold Value"]
    #[must_use]
    #[inline(always)]
    pub const fn high_temp_val(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "High Temperature Threshold Value"]
    #[inline(always)]
    pub const fn set_high_temp_val(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Range0Clr {
    #[inline(always)]
    fn default() -> Range0Clr {
        Range0Clr(0u64 as u32)
    }
}
impl core::fmt::Debug for Range0Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Range0Clr")
            .field("low_temp_val", &self.low_temp_val())
            .field("high_temp_val", &self.high_temp_val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Range0Clr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Range0Clr {
            low_temp_val: u16,
            high_temp_val: u16,
        }
        let proxy = Range0Clr {
            low_temp_val: self.low_temp_val(),
            high_temp_val: self.high_temp_val(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Range 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Range0Set(pub u32);
impl Range0Set {
    #[doc = "Low Temperature Threshold Value"]
    #[must_use]
    #[inline(always)]
    pub const fn low_temp_val(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Low Temperature Threshold Value"]
    #[inline(always)]
    pub const fn set_low_temp_val(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "High Temperature Threshold Value"]
    #[must_use]
    #[inline(always)]
    pub const fn high_temp_val(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "High Temperature Threshold Value"]
    #[inline(always)]
    pub const fn set_high_temp_val(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Range0Set {
    #[inline(always)]
    fn default() -> Range0Set {
        Range0Set(0u64 as u32)
    }
}
impl core::fmt::Debug for Range0Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Range0Set")
            .field("low_temp_val", &self.low_temp_val())
            .field("high_temp_val", &self.high_temp_val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Range0Set {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Range0Set {
            low_temp_val: u16,
            high_temp_val: u16,
        }
        let proxy = Range0Set {
            low_temp_val: self.low_temp_val(),
            high_temp_val: self.high_temp_val(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Range 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Range0Tog(pub u32);
impl Range0Tog {
    #[doc = "Low Temperature Threshold Value"]
    #[must_use]
    #[inline(always)]
    pub const fn low_temp_val(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Low Temperature Threshold Value"]
    #[inline(always)]
    pub const fn set_low_temp_val(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "High Temperature Threshold Value"]
    #[must_use]
    #[inline(always)]
    pub const fn high_temp_val(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "High Temperature Threshold Value"]
    #[inline(always)]
    pub const fn set_high_temp_val(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Range0Tog {
    #[inline(always)]
    fn default() -> Range0Tog {
        Range0Tog(0u64 as u32)
    }
}
impl core::fmt::Debug for Range0Tog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Range0Tog")
            .field("low_temp_val", &self.low_temp_val())
            .field("high_temp_val", &self.high_temp_val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Range0Tog {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Range0Tog {
            low_temp_val: u16,
            high_temp_val: u16,
        }
        let proxy = Range0Tog {
            low_temp_val: self.low_temp_val(),
            high_temp_val: self.high_temp_val(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Range 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Range1(pub u32);
impl Range1 {
    #[doc = "Panic Temperature Threshold Value"]
    #[must_use]
    #[inline(always)]
    pub const fn panic_temp_val(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Panic Temperature Threshold Value"]
    #[inline(always)]
    pub const fn set_panic_temp_val(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for Range1 {
    #[inline(always)]
    fn default() -> Range1 {
        Range1(0u64 as u32)
    }
}
impl core::fmt::Debug for Range1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Range1")
            .field("panic_temp_val", &self.panic_temp_val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Range1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Range1 {
            panic_temp_val: u16,
        }
        let proxy = Range1 {
            panic_temp_val: self.panic_temp_val(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Range 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Range1Clr(pub u32);
impl Range1Clr {
    #[doc = "Panic Temperature Threshold Value"]
    #[must_use]
    #[inline(always)]
    pub const fn panic_temp_val(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Panic Temperature Threshold Value"]
    #[inline(always)]
    pub const fn set_panic_temp_val(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for Range1Clr {
    #[inline(always)]
    fn default() -> Range1Clr {
        Range1Clr(0u64 as u32)
    }
}
impl core::fmt::Debug for Range1Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Range1Clr")
            .field("panic_temp_val", &self.panic_temp_val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Range1Clr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Range1Clr {
            panic_temp_val: u16,
        }
        let proxy = Range1Clr {
            panic_temp_val: self.panic_temp_val(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Range 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Range1Set(pub u32);
impl Range1Set {
    #[doc = "Panic Temperature Threshold Value"]
    #[must_use]
    #[inline(always)]
    pub const fn panic_temp_val(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Panic Temperature Threshold Value"]
    #[inline(always)]
    pub const fn set_panic_temp_val(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for Range1Set {
    #[inline(always)]
    fn default() -> Range1Set {
        Range1Set(0u64 as u32)
    }
}
impl core::fmt::Debug for Range1Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Range1Set")
            .field("panic_temp_val", &self.panic_temp_val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Range1Set {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Range1Set {
            panic_temp_val: u16,
        }
        let proxy = Range1Set {
            panic_temp_val: self.panic_temp_val(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Range 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Range1Tog(pub u32);
impl Range1Tog {
    #[doc = "Panic Temperature Threshold Value"]
    #[must_use]
    #[inline(always)]
    pub const fn panic_temp_val(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Panic Temperature Threshold Value"]
    #[inline(always)]
    pub const fn set_panic_temp_val(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for Range1Tog {
    #[inline(always)]
    fn default() -> Range1Tog {
        Range1Tog(0u64 as u32)
    }
}
impl core::fmt::Debug for Range1Tog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Range1Tog")
            .field("panic_temp_val", &self.panic_temp_val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Range1Tog {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Range1Tog {
            panic_temp_val: u16,
        }
        let proxy = Range1Tog {
            panic_temp_val: self.panic_temp_val(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Status 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status0(pub u32);
impl Status0 {
    #[doc = "Measured Temperature Value"]
    #[must_use]
    #[inline(always)]
    pub const fn temp_val(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Measured Temperature Value"]
    #[inline(always)]
    pub const fn set_temp_val(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Temperature Measurement Complete"]
    #[must_use]
    #[inline(always)]
    pub const fn finish(&self) -> super::vals::Finish {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Finish::from_bits(val as u8)
    }
    #[doc = "Temperature Measurement Complete"]
    #[inline(always)]
    pub const fn set_finish(&mut self, val: super::vals::Finish) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Low Temperature Alarm"]
    #[must_use]
    #[inline(always)]
    pub const fn low_temp(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Low Temperature Alarm"]
    #[inline(always)]
    pub const fn set_low_temp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "High Temperature Alarm"]
    #[must_use]
    #[inline(always)]
    pub const fn high_temp(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "High Temperature Alarm"]
    #[inline(always)]
    pub const fn set_high_temp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Panic Temperature Alarm"]
    #[must_use]
    #[inline(always)]
    pub const fn panic_temp(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Panic Temperature Alarm"]
    #[inline(always)]
    pub const fn set_panic_temp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for Status0 {
    #[inline(always)]
    fn default() -> Status0 {
        Status0(0u64 as u32)
    }
}
impl core::fmt::Debug for Status0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Status0")
            .field("temp_val", &self.temp_val())
            .field("finish", &self.finish())
            .field("low_temp", &self.low_temp())
            .field("high_temp", &self.high_temp())
            .field("panic_temp", &self.panic_temp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Status0 {
            temp_val: u16,
            finish: super::vals::Finish,
            low_temp: bool,
            high_temp: bool,
            panic_temp: bool,
        }
        let proxy = Status0 {
            temp_val: self.temp_val(),
            finish: self.finish(),
            low_temp: self.low_temp(),
            high_temp: self.high_temp(),
            panic_temp: self.panic_temp(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
