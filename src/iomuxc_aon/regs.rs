#[doc = "I3C1_PIN_SCL_IN_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3c1PinSclInSelectInput(pub u32);
impl I3c1PinSclInSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::I3c1PinSclInSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::I3c1PinSclInSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::I3c1PinSclInSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for I3c1PinSclInSelectInput {
    #[inline(always)]
    fn default() -> I3c1PinSclInSelectInput {
        I3c1PinSclInSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for I3c1PinSclInSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3c1PinSclInSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3c1PinSclInSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct I3c1PinSclInSelectInput {
            daisy: super::vals::I3c1PinSclInSelectInputDaisy,
        }
        let proxy = I3c1PinSclInSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "I3C1_PIN_SDA_IN_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3c1PinSdaInSelectInput(pub u32);
impl I3c1PinSdaInSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::I3c1PinSdaInSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::I3c1PinSdaInSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::I3c1PinSdaInSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for I3c1PinSdaInSelectInput {
    #[inline(always)]
    fn default() -> I3c1PinSdaInSelectInput {
        I3c1PinSdaInSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for I3c1PinSdaInSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3c1PinSdaInSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3c1PinSdaInSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct I3c1PinSdaInSelectInput {
            daisy: super::vals::I3c1PinSdaInSelectInputDaisy,
        }
        let proxy = I3c1PinSdaInSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPI2C1_IPP_IND_LPI2C_SCL_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpi2c1IppIndLpi2cSclSelectInput(pub u32);
impl Lpi2c1IppIndLpi2cSclSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpi2c1IppIndLpi2cSclSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpi2c1IppIndLpi2cSclSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpi2c1IppIndLpi2cSclSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpi2c1IppIndLpi2cSclSelectInput {
    #[inline(always)]
    fn default() -> Lpi2c1IppIndLpi2cSclSelectInput {
        Lpi2c1IppIndLpi2cSclSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpi2c1IppIndLpi2cSclSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpi2c1IppIndLpi2cSclSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpi2c1IppIndLpi2cSclSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpi2c1IppIndLpi2cSclSelectInput {
            daisy: super::vals::Lpi2c1IppIndLpi2cSclSelectInputDaisy,
        }
        let proxy = Lpi2c1IppIndLpi2cSclSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPI2C1_IPP_IND_LPI2C_SDA_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpi2c1IppIndLpi2cSdaSelectInput(pub u32);
impl Lpi2c1IppIndLpi2cSdaSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpi2c1IppIndLpi2cSdaSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpi2c1IppIndLpi2cSdaSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpi2c1IppIndLpi2cSdaSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpi2c1IppIndLpi2cSdaSelectInput {
    #[inline(always)]
    fn default() -> Lpi2c1IppIndLpi2cSdaSelectInput {
        Lpi2c1IppIndLpi2cSdaSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpi2c1IppIndLpi2cSdaSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpi2c1IppIndLpi2cSdaSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpi2c1IppIndLpi2cSdaSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpi2c1IppIndLpi2cSdaSelectInput {
            daisy: super::vals::Lpi2c1IppIndLpi2cSdaSelectInputDaisy,
        }
        let proxy = Lpi2c1IppIndLpi2cSdaSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPI2C2_IPP_IND_LPI2C_SCL_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpi2c2IppIndLpi2cSclSelectInput(pub u32);
impl Lpi2c2IppIndLpi2cSclSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpi2c2IppIndLpi2cSclSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpi2c2IppIndLpi2cSclSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpi2c2IppIndLpi2cSclSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpi2c2IppIndLpi2cSclSelectInput {
    #[inline(always)]
    fn default() -> Lpi2c2IppIndLpi2cSclSelectInput {
        Lpi2c2IppIndLpi2cSclSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpi2c2IppIndLpi2cSclSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpi2c2IppIndLpi2cSclSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpi2c2IppIndLpi2cSclSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpi2c2IppIndLpi2cSclSelectInput {
            daisy: super::vals::Lpi2c2IppIndLpi2cSclSelectInputDaisy,
        }
        let proxy = Lpi2c2IppIndLpi2cSclSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPI2C2_IPP_IND_LPI2C_SDA_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpi2c2IppIndLpi2cSdaSelectInput(pub u32);
impl Lpi2c2IppIndLpi2cSdaSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpi2c2IppIndLpi2cSdaSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpi2c2IppIndLpi2cSdaSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpi2c2IppIndLpi2cSdaSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpi2c2IppIndLpi2cSdaSelectInput {
    #[inline(always)]
    fn default() -> Lpi2c2IppIndLpi2cSdaSelectInput {
        Lpi2c2IppIndLpi2cSdaSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpi2c2IppIndLpi2cSdaSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpi2c2IppIndLpi2cSdaSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpi2c2IppIndLpi2cSdaSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpi2c2IppIndLpi2cSdaSelectInput {
            daisy: super::vals::Lpi2c2IppIndLpi2cSdaSelectInputDaisy,
        }
        let proxy = Lpi2c2IppIndLpi2cSdaSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPSPI1_IPP_IND_LPSPI_PCS_SELECT_INPUT_0 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi1IppIndLpspiPcsSelectInput0(pub u32);
impl Lpspi1IppIndLpspiPcsSelectInput0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi1IppIndLpspiPcsSelectInput0Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpspi1IppIndLpspiPcsSelectInput0Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi1IppIndLpspiPcsSelectInput0Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpspi1IppIndLpspiPcsSelectInput0 {
    #[inline(always)]
    fn default() -> Lpspi1IppIndLpspiPcsSelectInput0 {
        Lpspi1IppIndLpspiPcsSelectInput0(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpspi1IppIndLpspiPcsSelectInput0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi1IppIndLpspiPcsSelectInput0")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi1IppIndLpspiPcsSelectInput0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpspi1IppIndLpspiPcsSelectInput0 {
            daisy: super::vals::Lpspi1IppIndLpspiPcsSelectInput0Daisy,
        }
        let proxy = Lpspi1IppIndLpspiPcsSelectInput0 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPSPI1_IPP_IND_LPSPI_PCS_SELECT_INPUT_1 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi1IppIndLpspiPcsSelectInput1(pub u32);
impl Lpspi1IppIndLpspiPcsSelectInput1 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi1IppIndLpspiPcsSelectInput1Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpspi1IppIndLpspiPcsSelectInput1Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi1IppIndLpspiPcsSelectInput1Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpspi1IppIndLpspiPcsSelectInput1 {
    #[inline(always)]
    fn default() -> Lpspi1IppIndLpspiPcsSelectInput1 {
        Lpspi1IppIndLpspiPcsSelectInput1(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpspi1IppIndLpspiPcsSelectInput1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi1IppIndLpspiPcsSelectInput1")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi1IppIndLpspiPcsSelectInput1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpspi1IppIndLpspiPcsSelectInput1 {
            daisy: super::vals::Lpspi1IppIndLpspiPcsSelectInput1Daisy,
        }
        let proxy = Lpspi1IppIndLpspiPcsSelectInput1 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPSPI1_IPP_IND_LPSPI_SCK_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi1IppIndLpspiSckSelectInput(pub u32);
impl Lpspi1IppIndLpspiSckSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi1IppIndLpspiSckSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpspi1IppIndLpspiSckSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi1IppIndLpspiSckSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpspi1IppIndLpspiSckSelectInput {
    #[inline(always)]
    fn default() -> Lpspi1IppIndLpspiSckSelectInput {
        Lpspi1IppIndLpspiSckSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpspi1IppIndLpspiSckSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi1IppIndLpspiSckSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi1IppIndLpspiSckSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpspi1IppIndLpspiSckSelectInput {
            daisy: super::vals::Lpspi1IppIndLpspiSckSelectInputDaisy,
        }
        let proxy = Lpspi1IppIndLpspiSckSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPSPI1_IPP_IND_LPSPI_SDI_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi1IppIndLpspiSdiSelectInput(pub u32);
impl Lpspi1IppIndLpspiSdiSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi1IppIndLpspiSdiSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpspi1IppIndLpspiSdiSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi1IppIndLpspiSdiSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpspi1IppIndLpspiSdiSelectInput {
    #[inline(always)]
    fn default() -> Lpspi1IppIndLpspiSdiSelectInput {
        Lpspi1IppIndLpspiSdiSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpspi1IppIndLpspiSdiSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi1IppIndLpspiSdiSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi1IppIndLpspiSdiSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpspi1IppIndLpspiSdiSelectInput {
            daisy: super::vals::Lpspi1IppIndLpspiSdiSelectInputDaisy,
        }
        let proxy = Lpspi1IppIndLpspiSdiSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPSPI1_IPP_IND_LPSPI_SDO_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi1IppIndLpspiSdoSelectInput(pub u32);
impl Lpspi1IppIndLpspiSdoSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi1IppIndLpspiSdoSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpspi1IppIndLpspiSdoSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi1IppIndLpspiSdoSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpspi1IppIndLpspiSdoSelectInput {
    #[inline(always)]
    fn default() -> Lpspi1IppIndLpspiSdoSelectInput {
        Lpspi1IppIndLpspiSdoSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpspi1IppIndLpspiSdoSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi1IppIndLpspiSdoSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi1IppIndLpspiSdoSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpspi1IppIndLpspiSdoSelectInput {
            daisy: super::vals::Lpspi1IppIndLpspiSdoSelectInputDaisy,
        }
        let proxy = Lpspi1IppIndLpspiSdoSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPSPI2_IPP_IND_LPSPI_PCS_SELECT_INPUT_0 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi2IppIndLpspiPcsSelectInput0(pub u32);
impl Lpspi2IppIndLpspiPcsSelectInput0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi2IppIndLpspiPcsSelectInput0Daisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpspi2IppIndLpspiPcsSelectInput0Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi2IppIndLpspiPcsSelectInput0Daisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpspi2IppIndLpspiPcsSelectInput0 {
    #[inline(always)]
    fn default() -> Lpspi2IppIndLpspiPcsSelectInput0 {
        Lpspi2IppIndLpspiPcsSelectInput0(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpspi2IppIndLpspiPcsSelectInput0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi2IppIndLpspiPcsSelectInput0")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi2IppIndLpspiPcsSelectInput0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpspi2IppIndLpspiPcsSelectInput0 {
            daisy: super::vals::Lpspi2IppIndLpspiPcsSelectInput0Daisy,
        }
        let proxy = Lpspi2IppIndLpspiPcsSelectInput0 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPSPI2_IPP_IND_LPSPI_PCS_SELECT_INPUT_1 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi2IppIndLpspiPcsSelectInput1(pub u32);
impl Lpspi2IppIndLpspiPcsSelectInput1 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi2IppIndLpspiPcsSelectInput1Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpspi2IppIndLpspiPcsSelectInput1Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi2IppIndLpspiPcsSelectInput1Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpspi2IppIndLpspiPcsSelectInput1 {
    #[inline(always)]
    fn default() -> Lpspi2IppIndLpspiPcsSelectInput1 {
        Lpspi2IppIndLpspiPcsSelectInput1(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpspi2IppIndLpspiPcsSelectInput1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi2IppIndLpspiPcsSelectInput1")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi2IppIndLpspiPcsSelectInput1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpspi2IppIndLpspiPcsSelectInput1 {
            daisy: super::vals::Lpspi2IppIndLpspiPcsSelectInput1Daisy,
        }
        let proxy = Lpspi2IppIndLpspiPcsSelectInput1 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPSPI2_IPP_IND_LPSPI_PCS_SELECT_INPUT_3 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi2IppIndLpspiPcsSelectInput3(pub u32);
impl Lpspi2IppIndLpspiPcsSelectInput3 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi2IppIndLpspiPcsSelectInput3Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpspi2IppIndLpspiPcsSelectInput3Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi2IppIndLpspiPcsSelectInput3Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpspi2IppIndLpspiPcsSelectInput3 {
    #[inline(always)]
    fn default() -> Lpspi2IppIndLpspiPcsSelectInput3 {
        Lpspi2IppIndLpspiPcsSelectInput3(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpspi2IppIndLpspiPcsSelectInput3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi2IppIndLpspiPcsSelectInput3")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi2IppIndLpspiPcsSelectInput3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpspi2IppIndLpspiPcsSelectInput3 {
            daisy: super::vals::Lpspi2IppIndLpspiPcsSelectInput3Daisy,
        }
        let proxy = Lpspi2IppIndLpspiPcsSelectInput3 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPSPI2_IPP_IND_LPSPI_SCK_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi2IppIndLpspiSckSelectInput(pub u32);
impl Lpspi2IppIndLpspiSckSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi2IppIndLpspiSckSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpspi2IppIndLpspiSckSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi2IppIndLpspiSckSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpspi2IppIndLpspiSckSelectInput {
    #[inline(always)]
    fn default() -> Lpspi2IppIndLpspiSckSelectInput {
        Lpspi2IppIndLpspiSckSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpspi2IppIndLpspiSckSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi2IppIndLpspiSckSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi2IppIndLpspiSckSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpspi2IppIndLpspiSckSelectInput {
            daisy: super::vals::Lpspi2IppIndLpspiSckSelectInputDaisy,
        }
        let proxy = Lpspi2IppIndLpspiSckSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPSPI2_IPP_IND_LPSPI_SDI_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi2IppIndLpspiSdiSelectInput(pub u32);
impl Lpspi2IppIndLpspiSdiSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi2IppIndLpspiSdiSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpspi2IppIndLpspiSdiSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi2IppIndLpspiSdiSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpspi2IppIndLpspiSdiSelectInput {
    #[inline(always)]
    fn default() -> Lpspi2IppIndLpspiSdiSelectInput {
        Lpspi2IppIndLpspiSdiSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpspi2IppIndLpspiSdiSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi2IppIndLpspiSdiSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi2IppIndLpspiSdiSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpspi2IppIndLpspiSdiSelectInput {
            daisy: super::vals::Lpspi2IppIndLpspiSdiSelectInputDaisy,
        }
        let proxy = Lpspi2IppIndLpspiSdiSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPSPI2_IPP_IND_LPSPI_SDO_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi2IppIndLpspiSdoSelectInput(pub u32);
impl Lpspi2IppIndLpspiSdoSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi2IppIndLpspiSdoSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpspi2IppIndLpspiSdoSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi2IppIndLpspiSdoSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpspi2IppIndLpspiSdoSelectInput {
    #[inline(always)]
    fn default() -> Lpspi2IppIndLpspiSdoSelectInput {
        Lpspi2IppIndLpspiSdoSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpspi2IppIndLpspiSdoSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi2IppIndLpspiSdoSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi2IppIndLpspiSdoSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpspi2IppIndLpspiSdoSelectInput {
            daisy: super::vals::Lpspi2IppIndLpspiSdoSelectInputDaisy,
        }
        let proxy = Lpspi2IppIndLpspiSdoSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPTMR1_IPP_IND_LPTIMER_SELECT_INPUT_1 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lptmr1IppIndLptimerSelectInput1(pub u32);
impl Lptmr1IppIndLptimerSelectInput1 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lptmr1IppIndLptimerSelectInput1Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lptmr1IppIndLptimerSelectInput1Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lptmr1IppIndLptimerSelectInput1Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lptmr1IppIndLptimerSelectInput1 {
    #[inline(always)]
    fn default() -> Lptmr1IppIndLptimerSelectInput1 {
        Lptmr1IppIndLptimerSelectInput1(0u64 as u32)
    }
}
impl core::fmt::Debug for Lptmr1IppIndLptimerSelectInput1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lptmr1IppIndLptimerSelectInput1")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lptmr1IppIndLptimerSelectInput1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lptmr1IppIndLptimerSelectInput1 {
            daisy: super::vals::Lptmr1IppIndLptimerSelectInput1Daisy,
        }
        let proxy = Lptmr1IppIndLptimerSelectInput1 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPTMR1_IPP_IND_LPTIMER_SELECT_INPUT_2 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lptmr1IppIndLptimerSelectInput2(pub u32);
impl Lptmr1IppIndLptimerSelectInput2 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lptmr1IppIndLptimerSelectInput2Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lptmr1IppIndLptimerSelectInput2Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lptmr1IppIndLptimerSelectInput2Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lptmr1IppIndLptimerSelectInput2 {
    #[inline(always)]
    fn default() -> Lptmr1IppIndLptimerSelectInput2 {
        Lptmr1IppIndLptimerSelectInput2(0u64 as u32)
    }
}
impl core::fmt::Debug for Lptmr1IppIndLptimerSelectInput2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lptmr1IppIndLptimerSelectInput2")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lptmr1IppIndLptimerSelectInput2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lptmr1IppIndLptimerSelectInput2 {
            daisy: super::vals::Lptmr1IppIndLptimerSelectInput2Daisy,
        }
        let proxy = Lptmr1IppIndLptimerSelectInput2 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPTMR1_IPP_IND_LPTIMER_SELECT_INPUT_3 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lptmr1IppIndLptimerSelectInput3(pub u32);
impl Lptmr1IppIndLptimerSelectInput3 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lptmr1IppIndLptimerSelectInput3Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lptmr1IppIndLptimerSelectInput3Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lptmr1IppIndLptimerSelectInput3Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lptmr1IppIndLptimerSelectInput3 {
    #[inline(always)]
    fn default() -> Lptmr1IppIndLptimerSelectInput3 {
        Lptmr1IppIndLptimerSelectInput3(0u64 as u32)
    }
}
impl core::fmt::Debug for Lptmr1IppIndLptimerSelectInput3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lptmr1IppIndLptimerSelectInput3")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lptmr1IppIndLptimerSelectInput3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lptmr1IppIndLptimerSelectInput3 {
            daisy: super::vals::Lptmr1IppIndLptimerSelectInput3Daisy,
        }
        let proxy = Lptmr1IppIndLptimerSelectInput3 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPUART12_IPP_IND_LPUART_CTS_N_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart12IppIndLpuartCtsNSelectInput(pub u32);
impl Lpuart12IppIndLpuartCtsNSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart12IppIndLpuartCtsNSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpuart12IppIndLpuartCtsNSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart12IppIndLpuartCtsNSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart12IppIndLpuartCtsNSelectInput {
    #[inline(always)]
    fn default() -> Lpuart12IppIndLpuartCtsNSelectInput {
        Lpuart12IppIndLpuartCtsNSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpuart12IppIndLpuartCtsNSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart12IppIndLpuartCtsNSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart12IppIndLpuartCtsNSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpuart12IppIndLpuartCtsNSelectInput {
            daisy: super::vals::Lpuart12IppIndLpuartCtsNSelectInputDaisy,
        }
        let proxy = Lpuart12IppIndLpuartCtsNSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPUART12_IPP_IND_LPUART_RXD_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart12IppIndLpuartRxdSelectInput(pub u32);
impl Lpuart12IppIndLpuartRxdSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart12IppIndLpuartRxdSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpuart12IppIndLpuartRxdSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart12IppIndLpuartRxdSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart12IppIndLpuartRxdSelectInput {
    #[inline(always)]
    fn default() -> Lpuart12IppIndLpuartRxdSelectInput {
        Lpuart12IppIndLpuartRxdSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpuart12IppIndLpuartRxdSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart12IppIndLpuartRxdSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart12IppIndLpuartRxdSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpuart12IppIndLpuartRxdSelectInput {
            daisy: super::vals::Lpuart12IppIndLpuartRxdSelectInputDaisy,
        }
        let proxy = Lpuart12IppIndLpuartRxdSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPUART12_IPP_IND_LPUART_TXD_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart12IppIndLpuartTxdSelectInput(pub u32);
impl Lpuart12IppIndLpuartTxdSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart12IppIndLpuartTxdSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpuart12IppIndLpuartTxdSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart12IppIndLpuartTxdSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart12IppIndLpuartTxdSelectInput {
    #[inline(always)]
    fn default() -> Lpuart12IppIndLpuartTxdSelectInput {
        Lpuart12IppIndLpuartTxdSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpuart12IppIndLpuartTxdSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart12IppIndLpuartTxdSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart12IppIndLpuartTxdSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpuart12IppIndLpuartTxdSelectInput {
            daisy: super::vals::Lpuart12IppIndLpuartTxdSelectInputDaisy,
        }
        let proxy = Lpuart12IppIndLpuartTxdSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPUART1_IPP_IND_LPUART_CTS_N_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart1IppIndLpuartCtsNSelectInput(pub u32);
impl Lpuart1IppIndLpuartCtsNSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart1IppIndLpuartCtsNSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpuart1IppIndLpuartCtsNSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart1IppIndLpuartCtsNSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart1IppIndLpuartCtsNSelectInput {
    #[inline(always)]
    fn default() -> Lpuart1IppIndLpuartCtsNSelectInput {
        Lpuart1IppIndLpuartCtsNSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpuart1IppIndLpuartCtsNSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart1IppIndLpuartCtsNSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart1IppIndLpuartCtsNSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpuart1IppIndLpuartCtsNSelectInput {
            daisy: super::vals::Lpuart1IppIndLpuartCtsNSelectInputDaisy,
        }
        let proxy = Lpuart1IppIndLpuartCtsNSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPUART1_IPP_IND_LPUART_DCD_N_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart1IppIndLpuartDcdNSelectInput(pub u32);
impl Lpuart1IppIndLpuartDcdNSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart1IppIndLpuartDcdNSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpuart1IppIndLpuartDcdNSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart1IppIndLpuartDcdNSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart1IppIndLpuartDcdNSelectInput {
    #[inline(always)]
    fn default() -> Lpuart1IppIndLpuartDcdNSelectInput {
        Lpuart1IppIndLpuartDcdNSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpuart1IppIndLpuartDcdNSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart1IppIndLpuartDcdNSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart1IppIndLpuartDcdNSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpuart1IppIndLpuartDcdNSelectInput {
            daisy: super::vals::Lpuart1IppIndLpuartDcdNSelectInputDaisy,
        }
        let proxy = Lpuart1IppIndLpuartDcdNSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPUART1_IPP_IND_LPUART_DSR_N_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart1IppIndLpuartDsrNSelectInput(pub u32);
impl Lpuart1IppIndLpuartDsrNSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart1IppIndLpuartDsrNSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpuart1IppIndLpuartDsrNSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart1IppIndLpuartDsrNSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart1IppIndLpuartDsrNSelectInput {
    #[inline(always)]
    fn default() -> Lpuart1IppIndLpuartDsrNSelectInput {
        Lpuart1IppIndLpuartDsrNSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpuart1IppIndLpuartDsrNSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart1IppIndLpuartDsrNSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart1IppIndLpuartDsrNSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpuart1IppIndLpuartDsrNSelectInput {
            daisy: super::vals::Lpuart1IppIndLpuartDsrNSelectInputDaisy,
        }
        let proxy = Lpuart1IppIndLpuartDsrNSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPUART2_IPP_IND_LPUART_CTS_N_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart2IppIndLpuartCtsNSelectInput(pub u32);
impl Lpuart2IppIndLpuartCtsNSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart2IppIndLpuartCtsNSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpuart2IppIndLpuartCtsNSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart2IppIndLpuartCtsNSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart2IppIndLpuartCtsNSelectInput {
    #[inline(always)]
    fn default() -> Lpuart2IppIndLpuartCtsNSelectInput {
        Lpuart2IppIndLpuartCtsNSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpuart2IppIndLpuartCtsNSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart2IppIndLpuartCtsNSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart2IppIndLpuartCtsNSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpuart2IppIndLpuartCtsNSelectInput {
            daisy: super::vals::Lpuart2IppIndLpuartCtsNSelectInputDaisy,
        }
        let proxy = Lpuart2IppIndLpuartCtsNSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPUART2_IPP_IND_LPUART_RXD_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart2IppIndLpuartRxdSelectInput(pub u32);
impl Lpuart2IppIndLpuartRxdSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart2IppIndLpuartRxdSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpuart2IppIndLpuartRxdSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart2IppIndLpuartRxdSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart2IppIndLpuartRxdSelectInput {
    #[inline(always)]
    fn default() -> Lpuart2IppIndLpuartRxdSelectInput {
        Lpuart2IppIndLpuartRxdSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpuart2IppIndLpuartRxdSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart2IppIndLpuartRxdSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart2IppIndLpuartRxdSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpuart2IppIndLpuartRxdSelectInput {
            daisy: super::vals::Lpuart2IppIndLpuartRxdSelectInputDaisy,
        }
        let proxy = Lpuart2IppIndLpuartRxdSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPUART2_IPP_IND_LPUART_TXD_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart2IppIndLpuartTxdSelectInput(pub u32);
impl Lpuart2IppIndLpuartTxdSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart2IppIndLpuartTxdSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpuart2IppIndLpuartTxdSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart2IppIndLpuartTxdSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart2IppIndLpuartTxdSelectInput {
    #[inline(always)]
    fn default() -> Lpuart2IppIndLpuartTxdSelectInput {
        Lpuart2IppIndLpuartTxdSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpuart2IppIndLpuartTxdSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart2IppIndLpuartTxdSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart2IppIndLpuartTxdSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpuart2IppIndLpuartTxdSelectInput {
            daisy: super::vals::Lpuart2IppIndLpuartTxdSelectInputDaisy,
        }
        let proxy = Lpuart2IppIndLpuartTxdSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPUART7_IPP_IND_LPUART_CTS_N_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart7IppIndLpuartCtsNSelectInput(pub u32);
impl Lpuart7IppIndLpuartCtsNSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart7IppIndLpuartCtsNSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpuart7IppIndLpuartCtsNSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart7IppIndLpuartCtsNSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpuart7IppIndLpuartCtsNSelectInput {
    #[inline(always)]
    fn default() -> Lpuart7IppIndLpuartCtsNSelectInput {
        Lpuart7IppIndLpuartCtsNSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpuart7IppIndLpuartCtsNSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart7IppIndLpuartCtsNSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart7IppIndLpuartCtsNSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpuart7IppIndLpuartCtsNSelectInput {
            daisy: super::vals::Lpuart7IppIndLpuartCtsNSelectInputDaisy,
        }
        let proxy = Lpuart7IppIndLpuartCtsNSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPUART7_IPP_IND_LPUART_RXD_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart7IppIndLpuartRxdSelectInput(pub u32);
impl Lpuart7IppIndLpuartRxdSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart7IppIndLpuartRxdSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpuart7IppIndLpuartRxdSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart7IppIndLpuartRxdSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart7IppIndLpuartRxdSelectInput {
    #[inline(always)]
    fn default() -> Lpuart7IppIndLpuartRxdSelectInput {
        Lpuart7IppIndLpuartRxdSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpuart7IppIndLpuartRxdSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart7IppIndLpuartRxdSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart7IppIndLpuartRxdSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpuart7IppIndLpuartRxdSelectInput {
            daisy: super::vals::Lpuart7IppIndLpuartRxdSelectInputDaisy,
        }
        let proxy = Lpuart7IppIndLpuartRxdSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPUART7_IPP_IND_LPUART_TXD_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart7IppIndLpuartTxdSelectInput(pub u32);
impl Lpuart7IppIndLpuartTxdSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart7IppIndLpuartTxdSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpuart7IppIndLpuartTxdSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart7IppIndLpuartTxdSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart7IppIndLpuartTxdSelectInput {
    #[inline(always)]
    fn default() -> Lpuart7IppIndLpuartTxdSelectInput {
        Lpuart7IppIndLpuartTxdSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpuart7IppIndLpuartTxdSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart7IppIndLpuartTxdSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart7IppIndLpuartTxdSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpuart7IppIndLpuartTxdSelectInput {
            daisy: super::vals::Lpuart7IppIndLpuartTxdSelectInputDaisy,
        }
        let proxy = Lpuart7IppIndLpuartTxdSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SAI1_IPG_CLK_SAI_MCLK_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai1IpgClkSaiMclkSelectInput(pub u32);
impl Sai1IpgClkSaiMclkSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sai1IpgClkSaiMclkSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sai1IpgClkSaiMclkSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sai1IpgClkSaiMclkSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Sai1IpgClkSaiMclkSelectInput {
    #[inline(always)]
    fn default() -> Sai1IpgClkSaiMclkSelectInput {
        Sai1IpgClkSaiMclkSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Sai1IpgClkSaiMclkSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai1IpgClkSaiMclkSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai1IpgClkSaiMclkSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sai1IpgClkSaiMclkSelectInput {
            daisy: super::vals::Sai1IpgClkSaiMclkSelectInputDaisy,
        }
        let proxy = Sai1IpgClkSaiMclkSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SAI1_IPP_IND_SAI_RXBCLK_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai1IppIndSaiRxbclkSelectInput(pub u32);
impl Sai1IppIndSaiRxbclkSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sai1IppIndSaiRxbclkSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sai1IppIndSaiRxbclkSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sai1IppIndSaiRxbclkSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Sai1IppIndSaiRxbclkSelectInput {
    #[inline(always)]
    fn default() -> Sai1IppIndSaiRxbclkSelectInput {
        Sai1IppIndSaiRxbclkSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Sai1IppIndSaiRxbclkSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai1IppIndSaiRxbclkSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai1IppIndSaiRxbclkSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sai1IppIndSaiRxbclkSelectInput {
            daisy: super::vals::Sai1IppIndSaiRxbclkSelectInputDaisy,
        }
        let proxy = Sai1IppIndSaiRxbclkSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SAI1_IPP_IND_SAI_RXDATA_SELECT_INPUT_0 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai1IppIndSaiRxdataSelectInput0(pub u32);
impl Sai1IppIndSaiRxdataSelectInput0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sai1IppIndSaiRxdataSelectInput0Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sai1IppIndSaiRxdataSelectInput0Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sai1IppIndSaiRxdataSelectInput0Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Sai1IppIndSaiRxdataSelectInput0 {
    #[inline(always)]
    fn default() -> Sai1IppIndSaiRxdataSelectInput0 {
        Sai1IppIndSaiRxdataSelectInput0(0u64 as u32)
    }
}
impl core::fmt::Debug for Sai1IppIndSaiRxdataSelectInput0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai1IppIndSaiRxdataSelectInput0")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai1IppIndSaiRxdataSelectInput0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sai1IppIndSaiRxdataSelectInput0 {
            daisy: super::vals::Sai1IppIndSaiRxdataSelectInput0Daisy,
        }
        let proxy = Sai1IppIndSaiRxdataSelectInput0 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SAI1_IPP_IND_SAI_RXDATA_SELECT_INPUT_1 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai1IppIndSaiRxdataSelectInput1(pub u32);
impl Sai1IppIndSaiRxdataSelectInput1 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sai1IppIndSaiRxdataSelectInput1Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sai1IppIndSaiRxdataSelectInput1Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sai1IppIndSaiRxdataSelectInput1Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Sai1IppIndSaiRxdataSelectInput1 {
    #[inline(always)]
    fn default() -> Sai1IppIndSaiRxdataSelectInput1 {
        Sai1IppIndSaiRxdataSelectInput1(0u64 as u32)
    }
}
impl core::fmt::Debug for Sai1IppIndSaiRxdataSelectInput1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai1IppIndSaiRxdataSelectInput1")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai1IppIndSaiRxdataSelectInput1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sai1IppIndSaiRxdataSelectInput1 {
            daisy: super::vals::Sai1IppIndSaiRxdataSelectInput1Daisy,
        }
        let proxy = Sai1IppIndSaiRxdataSelectInput1 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SAI1_IPP_IND_SAI_RXSYNC_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai1IppIndSaiRxsyncSelectInput(pub u32);
impl Sai1IppIndSaiRxsyncSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sai1IppIndSaiRxsyncSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sai1IppIndSaiRxsyncSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sai1IppIndSaiRxsyncSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Sai1IppIndSaiRxsyncSelectInput {
    #[inline(always)]
    fn default() -> Sai1IppIndSaiRxsyncSelectInput {
        Sai1IppIndSaiRxsyncSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Sai1IppIndSaiRxsyncSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai1IppIndSaiRxsyncSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai1IppIndSaiRxsyncSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sai1IppIndSaiRxsyncSelectInput {
            daisy: super::vals::Sai1IppIndSaiRxsyncSelectInputDaisy,
        }
        let proxy = Sai1IppIndSaiRxsyncSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SAI1_IPP_IND_SAI_TXBCLK_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai1IppIndSaiTxbclkSelectInput(pub u32);
impl Sai1IppIndSaiTxbclkSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sai1IppIndSaiTxbclkSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sai1IppIndSaiTxbclkSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sai1IppIndSaiTxbclkSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Sai1IppIndSaiTxbclkSelectInput {
    #[inline(always)]
    fn default() -> Sai1IppIndSaiTxbclkSelectInput {
        Sai1IppIndSaiTxbclkSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Sai1IppIndSaiTxbclkSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai1IppIndSaiTxbclkSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai1IppIndSaiTxbclkSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sai1IppIndSaiTxbclkSelectInput {
            daisy: super::vals::Sai1IppIndSaiTxbclkSelectInputDaisy,
        }
        let proxy = Sai1IppIndSaiTxbclkSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SAI1_IPP_IND_SAI_TXSYNC_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai1IppIndSaiTxsyncSelectInput(pub u32);
impl Sai1IppIndSaiTxsyncSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sai1IppIndSaiTxsyncSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sai1IppIndSaiTxsyncSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sai1IppIndSaiTxsyncSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Sai1IppIndSaiTxsyncSelectInput {
    #[inline(always)]
    fn default() -> Sai1IppIndSaiTxsyncSelectInput {
        Sai1IppIndSaiTxsyncSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Sai1IppIndSaiTxsyncSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai1IppIndSaiTxsyncSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai1IppIndSaiTxsyncSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sai1IppIndSaiTxsyncSelectInput {
            daisy: super::vals::Sai1IppIndSaiTxsyncSelectInputDaisy,
        }
        let proxy = Sai1IppIndSaiTxsyncSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AON_00 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAon00(pub u32);
impl SwMuxCtlPadGpioAon00 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAon00MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAon00MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAon00MuxMode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[must_use]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for SwMuxCtlPadGpioAon00 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAon00 {
        SwMuxCtlPadGpioAon00(0u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAon00 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAon00")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAon00 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAon00 {
            mux_mode: super::vals::SwMuxCtlPadGpioAon00MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAon00 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AON_01 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAon01(pub u32);
impl SwMuxCtlPadGpioAon01 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAon01MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAon01MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAon01MuxMode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[must_use]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for SwMuxCtlPadGpioAon01 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAon01 {
        SwMuxCtlPadGpioAon01(0u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAon01 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAon01")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAon01 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAon01 {
            mux_mode: super::vals::SwMuxCtlPadGpioAon01MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAon01 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AON_02 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAon02(pub u32);
impl SwMuxCtlPadGpioAon02 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAon02MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAon02MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAon02MuxMode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[must_use]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for SwMuxCtlPadGpioAon02 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAon02 {
        SwMuxCtlPadGpioAon02(0u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAon02 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAon02")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAon02 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAon02 {
            mux_mode: super::vals::SwMuxCtlPadGpioAon02MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAon02 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AON_03 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAon03(pub u32);
impl SwMuxCtlPadGpioAon03 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAon03MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAon03MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAon03MuxMode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[must_use]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for SwMuxCtlPadGpioAon03 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAon03 {
        SwMuxCtlPadGpioAon03(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAon03 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAon03")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAon03 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAon03 {
            mux_mode: super::vals::SwMuxCtlPadGpioAon03MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAon03 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AON_04 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAon04(pub u32);
impl SwMuxCtlPadGpioAon04 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAon04MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAon04MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAon04MuxMode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[must_use]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for SwMuxCtlPadGpioAon04 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAon04 {
        SwMuxCtlPadGpioAon04(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAon04 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAon04")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAon04 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAon04 {
            mux_mode: super::vals::SwMuxCtlPadGpioAon04MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAon04 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AON_05 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAon05(pub u32);
impl SwMuxCtlPadGpioAon05 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAon05MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAon05MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAon05MuxMode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[must_use]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for SwMuxCtlPadGpioAon05 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAon05 {
        SwMuxCtlPadGpioAon05(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAon05 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAon05")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAon05 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAon05 {
            mux_mode: super::vals::SwMuxCtlPadGpioAon05MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAon05 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AON_06 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAon06(pub u32);
impl SwMuxCtlPadGpioAon06 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAon06MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAon06MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAon06MuxMode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[must_use]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for SwMuxCtlPadGpioAon06 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAon06 {
        SwMuxCtlPadGpioAon06(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAon06 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAon06")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAon06 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAon06 {
            mux_mode: super::vals::SwMuxCtlPadGpioAon06MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAon06 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AON_07 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAon07(pub u32);
impl SwMuxCtlPadGpioAon07 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAon07MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAon07MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAon07MuxMode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[must_use]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for SwMuxCtlPadGpioAon07 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAon07 {
        SwMuxCtlPadGpioAon07(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAon07 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAon07")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAon07 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAon07 {
            mux_mode: super::vals::SwMuxCtlPadGpioAon07MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAon07 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AON_08 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAon08(pub u32);
impl SwMuxCtlPadGpioAon08 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAon08MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAon08MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAon08MuxMode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[must_use]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for SwMuxCtlPadGpioAon08 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAon08 {
        SwMuxCtlPadGpioAon08(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAon08 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAon08")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAon08 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAon08 {
            mux_mode: super::vals::SwMuxCtlPadGpioAon08MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAon08 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AON_09 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAon09(pub u32);
impl SwMuxCtlPadGpioAon09 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAon09MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAon09MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAon09MuxMode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[must_use]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for SwMuxCtlPadGpioAon09 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAon09 {
        SwMuxCtlPadGpioAon09(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAon09 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAon09")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAon09 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAon09 {
            mux_mode: super::vals::SwMuxCtlPadGpioAon09MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAon09 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AON_10 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAon10(pub u32);
impl SwMuxCtlPadGpioAon10 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAon10MuxMode {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::SwMuxCtlPadGpioAon10MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAon10MuxMode) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[must_use]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for SwMuxCtlPadGpioAon10 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAon10 {
        SwMuxCtlPadGpioAon10(0u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAon10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAon10")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAon10 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAon10 {
            mux_mode: super::vals::SwMuxCtlPadGpioAon10MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAon10 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AON_11 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAon11(pub u32);
impl SwMuxCtlPadGpioAon11 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAon11MuxMode {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::SwMuxCtlPadGpioAon11MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAon11MuxMode) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[must_use]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for SwMuxCtlPadGpioAon11 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAon11 {
        SwMuxCtlPadGpioAon11(0u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAon11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAon11")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAon11 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAon11 {
            mux_mode: super::vals::SwMuxCtlPadGpioAon11MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAon11 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AON_12 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAon12(pub u32);
impl SwMuxCtlPadGpioAon12 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAon12MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAon12MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAon12MuxMode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[must_use]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for SwMuxCtlPadGpioAon12 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAon12 {
        SwMuxCtlPadGpioAon12(0u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAon12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAon12")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAon12 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAon12 {
            mux_mode: super::vals::SwMuxCtlPadGpioAon12MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAon12 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AON_13 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAon13(pub u32);
impl SwMuxCtlPadGpioAon13 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAon13MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAon13MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAon13MuxMode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[must_use]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for SwMuxCtlPadGpioAon13 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAon13 {
        SwMuxCtlPadGpioAon13(0u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAon13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAon13")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAon13 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAon13 {
            mux_mode: super::vals::SwMuxCtlPadGpioAon13MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAon13 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AON_14 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAon14(pub u32);
impl SwMuxCtlPadGpioAon14 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAon14MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAon14MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAon14MuxMode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[must_use]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for SwMuxCtlPadGpioAon14 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAon14 {
        SwMuxCtlPadGpioAon14(0u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAon14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAon14")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAon14 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAon14 {
            mux_mode: super::vals::SwMuxCtlPadGpioAon14MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAon14 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AON_15 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAon15(pub u32);
impl SwMuxCtlPadGpioAon15 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAon15MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAon15MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAon15MuxMode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[must_use]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for SwMuxCtlPadGpioAon15 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAon15 {
        SwMuxCtlPadGpioAon15(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAon15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAon15")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAon15 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAon15 {
            mux_mode: super::vals::SwMuxCtlPadGpioAon15MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAon15 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AON_16 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAon16(pub u32);
impl SwMuxCtlPadGpioAon16 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAon16MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAon16MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAon16MuxMode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[must_use]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for SwMuxCtlPadGpioAon16 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAon16 {
        SwMuxCtlPadGpioAon16(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAon16 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAon16")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAon16 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAon16 {
            mux_mode: super::vals::SwMuxCtlPadGpioAon16MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAon16 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AON_17 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAon17(pub u32);
impl SwMuxCtlPadGpioAon17 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAon17MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAon17MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAon17MuxMode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[must_use]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for SwMuxCtlPadGpioAon17 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAon17 {
        SwMuxCtlPadGpioAon17(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAon17 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAon17")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAon17 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAon17 {
            mux_mode: super::vals::SwMuxCtlPadGpioAon17MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAon17 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AON_18 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAon18(pub u32);
impl SwMuxCtlPadGpioAon18 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAon18MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAon18MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAon18MuxMode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[must_use]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for SwMuxCtlPadGpioAon18 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAon18 {
        SwMuxCtlPadGpioAon18(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAon18 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAon18")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAon18 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAon18 {
            mux_mode: super::vals::SwMuxCtlPadGpioAon18MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAon18 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AON_19 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAon19(pub u32);
impl SwMuxCtlPadGpioAon19 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAon19MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAon19MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAon19MuxMode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[must_use]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for SwMuxCtlPadGpioAon19 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAon19 {
        SwMuxCtlPadGpioAon19(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAon19 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAon19")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAon19 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAon19 {
            mux_mode: super::vals::SwMuxCtlPadGpioAon19MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAon19 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AON_20 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAon20(pub u32);
impl SwMuxCtlPadGpioAon20 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAon20MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAon20MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAon20MuxMode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[must_use]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for SwMuxCtlPadGpioAon20 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAon20 {
        SwMuxCtlPadGpioAon20(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAon20 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAon20")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAon20 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAon20 {
            mux_mode: super::vals::SwMuxCtlPadGpioAon20MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAon20 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AON_21 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAon21(pub u32);
impl SwMuxCtlPadGpioAon21 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAon21MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAon21MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAon21MuxMode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[must_use]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for SwMuxCtlPadGpioAon21 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAon21 {
        SwMuxCtlPadGpioAon21(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAon21 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAon21")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAon21 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAon21 {
            mux_mode: super::vals::SwMuxCtlPadGpioAon21MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAon21 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AON_22 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAon22(pub u32);
impl SwMuxCtlPadGpioAon22 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAon22MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAon22MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAon22MuxMode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[must_use]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for SwMuxCtlPadGpioAon22 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAon22 {
        SwMuxCtlPadGpioAon22(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAon22 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAon22")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAon22 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAon22 {
            mux_mode: super::vals::SwMuxCtlPadGpioAon22MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAon22 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AON_23 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAon23(pub u32);
impl SwMuxCtlPadGpioAon23 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAon23MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAon23MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAon23MuxMode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[must_use]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for SwMuxCtlPadGpioAon23 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAon23 {
        SwMuxCtlPadGpioAon23(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAon23 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAon23")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAon23 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAon23 {
            mux_mode: super::vals::SwMuxCtlPadGpioAon23MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAon23 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AON_24 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAon24(pub u32);
impl SwMuxCtlPadGpioAon24 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAon24MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAon24MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAon24MuxMode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[must_use]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for SwMuxCtlPadGpioAon24 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAon24 {
        SwMuxCtlPadGpioAon24(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAon24 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAon24")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAon24 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAon24 {
            mux_mode: super::vals::SwMuxCtlPadGpioAon24MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAon24 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AON_25 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAon25(pub u32);
impl SwMuxCtlPadGpioAon25 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAon25MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAon25MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAon25MuxMode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[must_use]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for SwMuxCtlPadGpioAon25 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAon25 {
        SwMuxCtlPadGpioAon25(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAon25 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAon25")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAon25 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAon25 {
            mux_mode: super::vals::SwMuxCtlPadGpioAon25MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAon25 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AON_26 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAon26(pub u32);
impl SwMuxCtlPadGpioAon26 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAon26MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAon26MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAon26MuxMode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[must_use]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for SwMuxCtlPadGpioAon26 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAon26 {
        SwMuxCtlPadGpioAon26(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAon26 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAon26")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAon26 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAon26 {
            mux_mode: super::vals::SwMuxCtlPadGpioAon26MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAon26 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AON_27 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAon27(pub u32);
impl SwMuxCtlPadGpioAon27 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAon27MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAon27MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAon27MuxMode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[must_use]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for SwMuxCtlPadGpioAon27 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAon27 {
        SwMuxCtlPadGpioAon27(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAon27 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAon27")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAon27 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAon27 {
            mux_mode: super::vals::SwMuxCtlPadGpioAon27MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAon27 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AON_28_DUMMY SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAon28(pub u32);
impl SwMuxCtlPadGpioAon28 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAon28MuxMode {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::SwMuxCtlPadGpioAon28MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAon28MuxMode) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[must_use]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for SwMuxCtlPadGpioAon28 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAon28 {
        SwMuxCtlPadGpioAon28(0u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAon28 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAon28")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAon28 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAon28 {
            mux_mode: super::vals::SwMuxCtlPadGpioAon28MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAon28 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AON_00 SW PAD Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwPadCtlPadGpioAon00(pub u32);
impl SwPadCtlPadGpioAon00 {
    #[doc = "Slew Rate Field"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::SwPadCtlPadGpioAon00Sre {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon00Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Field"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::SwPadCtlPadGpioAon00Sre) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Drive Strength Field"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::SwPadCtlPadGpioAon00Dse {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon00Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Field"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::SwPadCtlPadGpioAon00Dse) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Pull / Keep Select Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pue(&self) -> super::vals::SwPadCtlPadGpioAon00Pue {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon00Pue::from_bits(val as u8)
    }
    #[doc = "Pull / Keep Select Field"]
    #[inline(always)]
    pub const fn set_pue(&mut self, val: super::vals::SwPadCtlPadGpioAon00Pue) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pus(&self) -> super::vals::SwPadCtlPadGpioAon00Pus {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon00Pus::from_bits(val as u8)
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[inline(always)]
    pub const fn set_pus(&mut self, val: super::vals::SwPadCtlPadGpioAon00Pus) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Field"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::SwPadCtlPadGpioAon00Ode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon00Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Field"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::SwPadCtlPadGpioAon00Ode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Domain write protection"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection"]
    #[inline(always)]
    pub const fn set_dwp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Domain write protection lock"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp_lock(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection lock"]
    #[inline(always)]
    pub const fn set_dwp_lock(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for SwPadCtlPadGpioAon00 {
    #[inline(always)]
    fn default() -> SwPadCtlPadGpioAon00 {
        SwPadCtlPadGpioAon00(6u64 as u32)
    }
}
impl core::fmt::Debug for SwPadCtlPadGpioAon00 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwPadCtlPadGpioAon00")
            .field("sre", &self.sre())
            .field("dse", &self.dse())
            .field("pue", &self.pue())
            .field("pus", &self.pus())
            .field("ode", &self.ode())
            .field("dwp", &self.dwp())
            .field("dwp_lock", &self.dwp_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwPadCtlPadGpioAon00 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwPadCtlPadGpioAon00 {
            sre: super::vals::SwPadCtlPadGpioAon00Sre,
            dse: super::vals::SwPadCtlPadGpioAon00Dse,
            pue: super::vals::SwPadCtlPadGpioAon00Pue,
            pus: super::vals::SwPadCtlPadGpioAon00Pus,
            ode: super::vals::SwPadCtlPadGpioAon00Ode,
            dwp: u8,
            dwp_lock: u8,
        }
        let proxy = SwPadCtlPadGpioAon00 {
            sre: self.sre(),
            dse: self.dse(),
            pue: self.pue(),
            pus: self.pus(),
            ode: self.ode(),
            dwp: self.dwp(),
            dwp_lock: self.dwp_lock(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AON_01 SW PAD Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwPadCtlPadGpioAon01(pub u32);
impl SwPadCtlPadGpioAon01 {
    #[doc = "Slew Rate Field"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::SwPadCtlPadGpioAon01Sre {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon01Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Field"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::SwPadCtlPadGpioAon01Sre) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Drive Strength Field"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::SwPadCtlPadGpioAon01Dse {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon01Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Field"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::SwPadCtlPadGpioAon01Dse) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Pull / Keep Select Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pue(&self) -> super::vals::SwPadCtlPadGpioAon01Pue {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon01Pue::from_bits(val as u8)
    }
    #[doc = "Pull / Keep Select Field"]
    #[inline(always)]
    pub const fn set_pue(&mut self, val: super::vals::SwPadCtlPadGpioAon01Pue) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pus(&self) -> super::vals::SwPadCtlPadGpioAon01Pus {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon01Pus::from_bits(val as u8)
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[inline(always)]
    pub const fn set_pus(&mut self, val: super::vals::SwPadCtlPadGpioAon01Pus) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Field"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::SwPadCtlPadGpioAon01Ode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon01Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Field"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::SwPadCtlPadGpioAon01Ode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Domain write protection"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection"]
    #[inline(always)]
    pub const fn set_dwp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Domain write protection lock"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp_lock(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection lock"]
    #[inline(always)]
    pub const fn set_dwp_lock(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for SwPadCtlPadGpioAon01 {
    #[inline(always)]
    fn default() -> SwPadCtlPadGpioAon01 {
        SwPadCtlPadGpioAon01(6u64 as u32)
    }
}
impl core::fmt::Debug for SwPadCtlPadGpioAon01 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwPadCtlPadGpioAon01")
            .field("sre", &self.sre())
            .field("dse", &self.dse())
            .field("pue", &self.pue())
            .field("pus", &self.pus())
            .field("ode", &self.ode())
            .field("dwp", &self.dwp())
            .field("dwp_lock", &self.dwp_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwPadCtlPadGpioAon01 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwPadCtlPadGpioAon01 {
            sre: super::vals::SwPadCtlPadGpioAon01Sre,
            dse: super::vals::SwPadCtlPadGpioAon01Dse,
            pue: super::vals::SwPadCtlPadGpioAon01Pue,
            pus: super::vals::SwPadCtlPadGpioAon01Pus,
            ode: super::vals::SwPadCtlPadGpioAon01Ode,
            dwp: u8,
            dwp_lock: u8,
        }
        let proxy = SwPadCtlPadGpioAon01 {
            sre: self.sre(),
            dse: self.dse(),
            pue: self.pue(),
            pus: self.pus(),
            ode: self.ode(),
            dwp: self.dwp(),
            dwp_lock: self.dwp_lock(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AON_02 SW PAD Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwPadCtlPadGpioAon02(pub u32);
impl SwPadCtlPadGpioAon02 {
    #[doc = "Slew Rate Field"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::SwPadCtlPadGpioAon02Sre {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon02Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Field"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::SwPadCtlPadGpioAon02Sre) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Drive Strength Field"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::SwPadCtlPadGpioAon02Dse {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon02Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Field"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::SwPadCtlPadGpioAon02Dse) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Pull / Keep Select Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pue(&self) -> super::vals::SwPadCtlPadGpioAon02Pue {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon02Pue::from_bits(val as u8)
    }
    #[doc = "Pull / Keep Select Field"]
    #[inline(always)]
    pub const fn set_pue(&mut self, val: super::vals::SwPadCtlPadGpioAon02Pue) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pus(&self) -> super::vals::SwPadCtlPadGpioAon02Pus {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon02Pus::from_bits(val as u8)
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[inline(always)]
    pub const fn set_pus(&mut self, val: super::vals::SwPadCtlPadGpioAon02Pus) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Field"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::SwPadCtlPadGpioAon02Ode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon02Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Field"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::SwPadCtlPadGpioAon02Ode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Domain write protection"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection"]
    #[inline(always)]
    pub const fn set_dwp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Domain write protection lock"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp_lock(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection lock"]
    #[inline(always)]
    pub const fn set_dwp_lock(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for SwPadCtlPadGpioAon02 {
    #[inline(always)]
    fn default() -> SwPadCtlPadGpioAon02 {
        SwPadCtlPadGpioAon02(6u64 as u32)
    }
}
impl core::fmt::Debug for SwPadCtlPadGpioAon02 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwPadCtlPadGpioAon02")
            .field("sre", &self.sre())
            .field("dse", &self.dse())
            .field("pue", &self.pue())
            .field("pus", &self.pus())
            .field("ode", &self.ode())
            .field("dwp", &self.dwp())
            .field("dwp_lock", &self.dwp_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwPadCtlPadGpioAon02 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwPadCtlPadGpioAon02 {
            sre: super::vals::SwPadCtlPadGpioAon02Sre,
            dse: super::vals::SwPadCtlPadGpioAon02Dse,
            pue: super::vals::SwPadCtlPadGpioAon02Pue,
            pus: super::vals::SwPadCtlPadGpioAon02Pus,
            ode: super::vals::SwPadCtlPadGpioAon02Ode,
            dwp: u8,
            dwp_lock: u8,
        }
        let proxy = SwPadCtlPadGpioAon02 {
            sre: self.sre(),
            dse: self.dse(),
            pue: self.pue(),
            pus: self.pus(),
            ode: self.ode(),
            dwp: self.dwp(),
            dwp_lock: self.dwp_lock(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AON_03 SW PAD Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwPadCtlPadGpioAon03(pub u32);
impl SwPadCtlPadGpioAon03 {
    #[doc = "Slew Rate Field"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::SwPadCtlPadGpioAon03Sre {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon03Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Field"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::SwPadCtlPadGpioAon03Sre) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Drive Strength Field"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::SwPadCtlPadGpioAon03Dse {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon03Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Field"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::SwPadCtlPadGpioAon03Dse) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Pull / Keep Select Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pue(&self) -> super::vals::SwPadCtlPadGpioAon03Pue {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon03Pue::from_bits(val as u8)
    }
    #[doc = "Pull / Keep Select Field"]
    #[inline(always)]
    pub const fn set_pue(&mut self, val: super::vals::SwPadCtlPadGpioAon03Pue) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pus(&self) -> super::vals::SwPadCtlPadGpioAon03Pus {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon03Pus::from_bits(val as u8)
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[inline(always)]
    pub const fn set_pus(&mut self, val: super::vals::SwPadCtlPadGpioAon03Pus) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Field"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::SwPadCtlPadGpioAon03Ode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon03Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Field"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::SwPadCtlPadGpioAon03Ode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Domain write protection"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection"]
    #[inline(always)]
    pub const fn set_dwp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Domain write protection lock"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp_lock(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection lock"]
    #[inline(always)]
    pub const fn set_dwp_lock(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for SwPadCtlPadGpioAon03 {
    #[inline(always)]
    fn default() -> SwPadCtlPadGpioAon03 {
        SwPadCtlPadGpioAon03(6u64 as u32)
    }
}
impl core::fmt::Debug for SwPadCtlPadGpioAon03 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwPadCtlPadGpioAon03")
            .field("sre", &self.sre())
            .field("dse", &self.dse())
            .field("pue", &self.pue())
            .field("pus", &self.pus())
            .field("ode", &self.ode())
            .field("dwp", &self.dwp())
            .field("dwp_lock", &self.dwp_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwPadCtlPadGpioAon03 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwPadCtlPadGpioAon03 {
            sre: super::vals::SwPadCtlPadGpioAon03Sre,
            dse: super::vals::SwPadCtlPadGpioAon03Dse,
            pue: super::vals::SwPadCtlPadGpioAon03Pue,
            pus: super::vals::SwPadCtlPadGpioAon03Pus,
            ode: super::vals::SwPadCtlPadGpioAon03Ode,
            dwp: u8,
            dwp_lock: u8,
        }
        let proxy = SwPadCtlPadGpioAon03 {
            sre: self.sre(),
            dse: self.dse(),
            pue: self.pue(),
            pus: self.pus(),
            ode: self.ode(),
            dwp: self.dwp(),
            dwp_lock: self.dwp_lock(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AON_04 SW PAD Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwPadCtlPadGpioAon04(pub u32);
impl SwPadCtlPadGpioAon04 {
    #[doc = "Slew Rate Field"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::SwPadCtlPadGpioAon04Sre {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon04Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Field"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::SwPadCtlPadGpioAon04Sre) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Drive Strength Field"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::SwPadCtlPadGpioAon04Dse {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon04Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Field"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::SwPadCtlPadGpioAon04Dse) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Pull / Keep Select Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pue(&self) -> super::vals::SwPadCtlPadGpioAon04Pue {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon04Pue::from_bits(val as u8)
    }
    #[doc = "Pull / Keep Select Field"]
    #[inline(always)]
    pub const fn set_pue(&mut self, val: super::vals::SwPadCtlPadGpioAon04Pue) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pus(&self) -> super::vals::SwPadCtlPadGpioAon04Pus {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon04Pus::from_bits(val as u8)
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[inline(always)]
    pub const fn set_pus(&mut self, val: super::vals::SwPadCtlPadGpioAon04Pus) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Field"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::SwPadCtlPadGpioAon04Ode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon04Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Field"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::SwPadCtlPadGpioAon04Ode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Domain write protection"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection"]
    #[inline(always)]
    pub const fn set_dwp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Domain write protection lock"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp_lock(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection lock"]
    #[inline(always)]
    pub const fn set_dwp_lock(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for SwPadCtlPadGpioAon04 {
    #[inline(always)]
    fn default() -> SwPadCtlPadGpioAon04 {
        SwPadCtlPadGpioAon04(6u64 as u32)
    }
}
impl core::fmt::Debug for SwPadCtlPadGpioAon04 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwPadCtlPadGpioAon04")
            .field("sre", &self.sre())
            .field("dse", &self.dse())
            .field("pue", &self.pue())
            .field("pus", &self.pus())
            .field("ode", &self.ode())
            .field("dwp", &self.dwp())
            .field("dwp_lock", &self.dwp_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwPadCtlPadGpioAon04 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwPadCtlPadGpioAon04 {
            sre: super::vals::SwPadCtlPadGpioAon04Sre,
            dse: super::vals::SwPadCtlPadGpioAon04Dse,
            pue: super::vals::SwPadCtlPadGpioAon04Pue,
            pus: super::vals::SwPadCtlPadGpioAon04Pus,
            ode: super::vals::SwPadCtlPadGpioAon04Ode,
            dwp: u8,
            dwp_lock: u8,
        }
        let proxy = SwPadCtlPadGpioAon04 {
            sre: self.sre(),
            dse: self.dse(),
            pue: self.pue(),
            pus: self.pus(),
            ode: self.ode(),
            dwp: self.dwp(),
            dwp_lock: self.dwp_lock(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AON_05 SW PAD Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwPadCtlPadGpioAon05(pub u32);
impl SwPadCtlPadGpioAon05 {
    #[doc = "Slew Rate Field"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::SwPadCtlPadGpioAon05Sre {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon05Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Field"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::SwPadCtlPadGpioAon05Sre) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Drive Strength Field"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::SwPadCtlPadGpioAon05Dse {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon05Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Field"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::SwPadCtlPadGpioAon05Dse) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Pull / Keep Select Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pue(&self) -> super::vals::SwPadCtlPadGpioAon05Pue {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon05Pue::from_bits(val as u8)
    }
    #[doc = "Pull / Keep Select Field"]
    #[inline(always)]
    pub const fn set_pue(&mut self, val: super::vals::SwPadCtlPadGpioAon05Pue) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pus(&self) -> super::vals::SwPadCtlPadGpioAon05Pus {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon05Pus::from_bits(val as u8)
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[inline(always)]
    pub const fn set_pus(&mut self, val: super::vals::SwPadCtlPadGpioAon05Pus) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Field"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::SwPadCtlPadGpioAon05Ode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon05Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Field"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::SwPadCtlPadGpioAon05Ode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Domain write protection"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection"]
    #[inline(always)]
    pub const fn set_dwp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Domain write protection lock"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp_lock(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection lock"]
    #[inline(always)]
    pub const fn set_dwp_lock(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for SwPadCtlPadGpioAon05 {
    #[inline(always)]
    fn default() -> SwPadCtlPadGpioAon05 {
        SwPadCtlPadGpioAon05(6u64 as u32)
    }
}
impl core::fmt::Debug for SwPadCtlPadGpioAon05 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwPadCtlPadGpioAon05")
            .field("sre", &self.sre())
            .field("dse", &self.dse())
            .field("pue", &self.pue())
            .field("pus", &self.pus())
            .field("ode", &self.ode())
            .field("dwp", &self.dwp())
            .field("dwp_lock", &self.dwp_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwPadCtlPadGpioAon05 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwPadCtlPadGpioAon05 {
            sre: super::vals::SwPadCtlPadGpioAon05Sre,
            dse: super::vals::SwPadCtlPadGpioAon05Dse,
            pue: super::vals::SwPadCtlPadGpioAon05Pue,
            pus: super::vals::SwPadCtlPadGpioAon05Pus,
            ode: super::vals::SwPadCtlPadGpioAon05Ode,
            dwp: u8,
            dwp_lock: u8,
        }
        let proxy = SwPadCtlPadGpioAon05 {
            sre: self.sre(),
            dse: self.dse(),
            pue: self.pue(),
            pus: self.pus(),
            ode: self.ode(),
            dwp: self.dwp(),
            dwp_lock: self.dwp_lock(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AON_06 SW PAD Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwPadCtlPadGpioAon06(pub u32);
impl SwPadCtlPadGpioAon06 {
    #[doc = "Slew Rate Field"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::SwPadCtlPadGpioAon06Sre {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon06Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Field"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::SwPadCtlPadGpioAon06Sre) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Drive Strength Field"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::SwPadCtlPadGpioAon06Dse {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon06Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Field"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::SwPadCtlPadGpioAon06Dse) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Pull / Keep Select Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pue(&self) -> super::vals::SwPadCtlPadGpioAon06Pue {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon06Pue::from_bits(val as u8)
    }
    #[doc = "Pull / Keep Select Field"]
    #[inline(always)]
    pub const fn set_pue(&mut self, val: super::vals::SwPadCtlPadGpioAon06Pue) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pus(&self) -> super::vals::SwPadCtlPadGpioAon06Pus {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon06Pus::from_bits(val as u8)
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[inline(always)]
    pub const fn set_pus(&mut self, val: super::vals::SwPadCtlPadGpioAon06Pus) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Field"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::SwPadCtlPadGpioAon06Ode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon06Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Field"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::SwPadCtlPadGpioAon06Ode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Domain write protection"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection"]
    #[inline(always)]
    pub const fn set_dwp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Domain write protection lock"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp_lock(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection lock"]
    #[inline(always)]
    pub const fn set_dwp_lock(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for SwPadCtlPadGpioAon06 {
    #[inline(always)]
    fn default() -> SwPadCtlPadGpioAon06 {
        SwPadCtlPadGpioAon06(6u64 as u32)
    }
}
impl core::fmt::Debug for SwPadCtlPadGpioAon06 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwPadCtlPadGpioAon06")
            .field("sre", &self.sre())
            .field("dse", &self.dse())
            .field("pue", &self.pue())
            .field("pus", &self.pus())
            .field("ode", &self.ode())
            .field("dwp", &self.dwp())
            .field("dwp_lock", &self.dwp_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwPadCtlPadGpioAon06 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwPadCtlPadGpioAon06 {
            sre: super::vals::SwPadCtlPadGpioAon06Sre,
            dse: super::vals::SwPadCtlPadGpioAon06Dse,
            pue: super::vals::SwPadCtlPadGpioAon06Pue,
            pus: super::vals::SwPadCtlPadGpioAon06Pus,
            ode: super::vals::SwPadCtlPadGpioAon06Ode,
            dwp: u8,
            dwp_lock: u8,
        }
        let proxy = SwPadCtlPadGpioAon06 {
            sre: self.sre(),
            dse: self.dse(),
            pue: self.pue(),
            pus: self.pus(),
            ode: self.ode(),
            dwp: self.dwp(),
            dwp_lock: self.dwp_lock(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AON_07 SW PAD Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwPadCtlPadGpioAon07(pub u32);
impl SwPadCtlPadGpioAon07 {
    #[doc = "Slew Rate Field"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::SwPadCtlPadGpioAon07Sre {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon07Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Field"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::SwPadCtlPadGpioAon07Sre) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Drive Strength Field"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::SwPadCtlPadGpioAon07Dse {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon07Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Field"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::SwPadCtlPadGpioAon07Dse) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Pull / Keep Select Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pue(&self) -> super::vals::SwPadCtlPadGpioAon07Pue {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon07Pue::from_bits(val as u8)
    }
    #[doc = "Pull / Keep Select Field"]
    #[inline(always)]
    pub const fn set_pue(&mut self, val: super::vals::SwPadCtlPadGpioAon07Pue) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pus(&self) -> super::vals::SwPadCtlPadGpioAon07Pus {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon07Pus::from_bits(val as u8)
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[inline(always)]
    pub const fn set_pus(&mut self, val: super::vals::SwPadCtlPadGpioAon07Pus) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Field"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::SwPadCtlPadGpioAon07Ode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon07Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Field"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::SwPadCtlPadGpioAon07Ode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Domain write protection"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection"]
    #[inline(always)]
    pub const fn set_dwp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Domain write protection lock"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp_lock(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection lock"]
    #[inline(always)]
    pub const fn set_dwp_lock(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for SwPadCtlPadGpioAon07 {
    #[inline(always)]
    fn default() -> SwPadCtlPadGpioAon07 {
        SwPadCtlPadGpioAon07(6u64 as u32)
    }
}
impl core::fmt::Debug for SwPadCtlPadGpioAon07 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwPadCtlPadGpioAon07")
            .field("sre", &self.sre())
            .field("dse", &self.dse())
            .field("pue", &self.pue())
            .field("pus", &self.pus())
            .field("ode", &self.ode())
            .field("dwp", &self.dwp())
            .field("dwp_lock", &self.dwp_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwPadCtlPadGpioAon07 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwPadCtlPadGpioAon07 {
            sre: super::vals::SwPadCtlPadGpioAon07Sre,
            dse: super::vals::SwPadCtlPadGpioAon07Dse,
            pue: super::vals::SwPadCtlPadGpioAon07Pue,
            pus: super::vals::SwPadCtlPadGpioAon07Pus,
            ode: super::vals::SwPadCtlPadGpioAon07Ode,
            dwp: u8,
            dwp_lock: u8,
        }
        let proxy = SwPadCtlPadGpioAon07 {
            sre: self.sre(),
            dse: self.dse(),
            pue: self.pue(),
            pus: self.pus(),
            ode: self.ode(),
            dwp: self.dwp(),
            dwp_lock: self.dwp_lock(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AON_08 SW PAD Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwPadCtlPadGpioAon08(pub u32);
impl SwPadCtlPadGpioAon08 {
    #[doc = "Slew Rate Field"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::SwPadCtlPadGpioAon08Sre {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon08Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Field"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::SwPadCtlPadGpioAon08Sre) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Drive Strength Field"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::SwPadCtlPadGpioAon08Dse {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon08Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Field"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::SwPadCtlPadGpioAon08Dse) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Pull / Keep Select Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pue(&self) -> super::vals::SwPadCtlPadGpioAon08Pue {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon08Pue::from_bits(val as u8)
    }
    #[doc = "Pull / Keep Select Field"]
    #[inline(always)]
    pub const fn set_pue(&mut self, val: super::vals::SwPadCtlPadGpioAon08Pue) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pus(&self) -> super::vals::SwPadCtlPadGpioAon08Pus {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon08Pus::from_bits(val as u8)
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[inline(always)]
    pub const fn set_pus(&mut self, val: super::vals::SwPadCtlPadGpioAon08Pus) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Field"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::SwPadCtlPadGpioAon08Ode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon08Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Field"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::SwPadCtlPadGpioAon08Ode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Domain write protection"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection"]
    #[inline(always)]
    pub const fn set_dwp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Domain write protection lock"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp_lock(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection lock"]
    #[inline(always)]
    pub const fn set_dwp_lock(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for SwPadCtlPadGpioAon08 {
    #[inline(always)]
    fn default() -> SwPadCtlPadGpioAon08 {
        SwPadCtlPadGpioAon08(6u64 as u32)
    }
}
impl core::fmt::Debug for SwPadCtlPadGpioAon08 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwPadCtlPadGpioAon08")
            .field("sre", &self.sre())
            .field("dse", &self.dse())
            .field("pue", &self.pue())
            .field("pus", &self.pus())
            .field("ode", &self.ode())
            .field("dwp", &self.dwp())
            .field("dwp_lock", &self.dwp_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwPadCtlPadGpioAon08 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwPadCtlPadGpioAon08 {
            sre: super::vals::SwPadCtlPadGpioAon08Sre,
            dse: super::vals::SwPadCtlPadGpioAon08Dse,
            pue: super::vals::SwPadCtlPadGpioAon08Pue,
            pus: super::vals::SwPadCtlPadGpioAon08Pus,
            ode: super::vals::SwPadCtlPadGpioAon08Ode,
            dwp: u8,
            dwp_lock: u8,
        }
        let proxy = SwPadCtlPadGpioAon08 {
            sre: self.sre(),
            dse: self.dse(),
            pue: self.pue(),
            pus: self.pus(),
            ode: self.ode(),
            dwp: self.dwp(),
            dwp_lock: self.dwp_lock(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AON_09 SW PAD Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwPadCtlPadGpioAon09(pub u32);
impl SwPadCtlPadGpioAon09 {
    #[doc = "Slew Rate Field"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::SwPadCtlPadGpioAon09Sre {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon09Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Field"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::SwPadCtlPadGpioAon09Sre) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Drive Strength Field"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::SwPadCtlPadGpioAon09Dse {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon09Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Field"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::SwPadCtlPadGpioAon09Dse) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Pull / Keep Select Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pue(&self) -> super::vals::SwPadCtlPadGpioAon09Pue {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon09Pue::from_bits(val as u8)
    }
    #[doc = "Pull / Keep Select Field"]
    #[inline(always)]
    pub const fn set_pue(&mut self, val: super::vals::SwPadCtlPadGpioAon09Pue) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pus(&self) -> super::vals::SwPadCtlPadGpioAon09Pus {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon09Pus::from_bits(val as u8)
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[inline(always)]
    pub const fn set_pus(&mut self, val: super::vals::SwPadCtlPadGpioAon09Pus) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Field"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::SwPadCtlPadGpioAon09Ode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon09Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Field"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::SwPadCtlPadGpioAon09Ode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Domain write protection"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection"]
    #[inline(always)]
    pub const fn set_dwp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Domain write protection lock"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp_lock(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection lock"]
    #[inline(always)]
    pub const fn set_dwp_lock(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for SwPadCtlPadGpioAon09 {
    #[inline(always)]
    fn default() -> SwPadCtlPadGpioAon09 {
        SwPadCtlPadGpioAon09(6u64 as u32)
    }
}
impl core::fmt::Debug for SwPadCtlPadGpioAon09 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwPadCtlPadGpioAon09")
            .field("sre", &self.sre())
            .field("dse", &self.dse())
            .field("pue", &self.pue())
            .field("pus", &self.pus())
            .field("ode", &self.ode())
            .field("dwp", &self.dwp())
            .field("dwp_lock", &self.dwp_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwPadCtlPadGpioAon09 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwPadCtlPadGpioAon09 {
            sre: super::vals::SwPadCtlPadGpioAon09Sre,
            dse: super::vals::SwPadCtlPadGpioAon09Dse,
            pue: super::vals::SwPadCtlPadGpioAon09Pue,
            pus: super::vals::SwPadCtlPadGpioAon09Pus,
            ode: super::vals::SwPadCtlPadGpioAon09Ode,
            dwp: u8,
            dwp_lock: u8,
        }
        let proxy = SwPadCtlPadGpioAon09 {
            sre: self.sre(),
            dse: self.dse(),
            pue: self.pue(),
            pus: self.pus(),
            ode: self.ode(),
            dwp: self.dwp(),
            dwp_lock: self.dwp_lock(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AON_10 SW PAD Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwPadCtlPadGpioAon10(pub u32);
impl SwPadCtlPadGpioAon10 {
    #[doc = "Slew Rate Field"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::SwPadCtlPadGpioAon10Sre {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon10Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Field"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::SwPadCtlPadGpioAon10Sre) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Drive Strength Field"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::SwPadCtlPadGpioAon10Dse {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon10Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Field"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::SwPadCtlPadGpioAon10Dse) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Pull / Keep Select Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pue(&self) -> super::vals::SwPadCtlPadGpioAon10Pue {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon10Pue::from_bits(val as u8)
    }
    #[doc = "Pull / Keep Select Field"]
    #[inline(always)]
    pub const fn set_pue(&mut self, val: super::vals::SwPadCtlPadGpioAon10Pue) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pus(&self) -> super::vals::SwPadCtlPadGpioAon10Pus {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon10Pus::from_bits(val as u8)
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[inline(always)]
    pub const fn set_pus(&mut self, val: super::vals::SwPadCtlPadGpioAon10Pus) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Field"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::SwPadCtlPadGpioAon10Ode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon10Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Field"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::SwPadCtlPadGpioAon10Ode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Domain write protection"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection"]
    #[inline(always)]
    pub const fn set_dwp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Domain write protection lock"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp_lock(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection lock"]
    #[inline(always)]
    pub const fn set_dwp_lock(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for SwPadCtlPadGpioAon10 {
    #[inline(always)]
    fn default() -> SwPadCtlPadGpioAon10 {
        SwPadCtlPadGpioAon10(14u64 as u32)
    }
}
impl core::fmt::Debug for SwPadCtlPadGpioAon10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwPadCtlPadGpioAon10")
            .field("sre", &self.sre())
            .field("dse", &self.dse())
            .field("pue", &self.pue())
            .field("pus", &self.pus())
            .field("ode", &self.ode())
            .field("dwp", &self.dwp())
            .field("dwp_lock", &self.dwp_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwPadCtlPadGpioAon10 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwPadCtlPadGpioAon10 {
            sre: super::vals::SwPadCtlPadGpioAon10Sre,
            dse: super::vals::SwPadCtlPadGpioAon10Dse,
            pue: super::vals::SwPadCtlPadGpioAon10Pue,
            pus: super::vals::SwPadCtlPadGpioAon10Pus,
            ode: super::vals::SwPadCtlPadGpioAon10Ode,
            dwp: u8,
            dwp_lock: u8,
        }
        let proxy = SwPadCtlPadGpioAon10 {
            sre: self.sre(),
            dse: self.dse(),
            pue: self.pue(),
            pus: self.pus(),
            ode: self.ode(),
            dwp: self.dwp(),
            dwp_lock: self.dwp_lock(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AON_11 SW PAD Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwPadCtlPadGpioAon11(pub u32);
impl SwPadCtlPadGpioAon11 {
    #[doc = "Slew Rate Field"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::SwPadCtlPadGpioAon11Sre {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon11Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Field"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::SwPadCtlPadGpioAon11Sre) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Drive Strength Field"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::SwPadCtlPadGpioAon11Dse {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon11Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Field"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::SwPadCtlPadGpioAon11Dse) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Pull / Keep Select Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pue(&self) -> super::vals::SwPadCtlPadGpioAon11Pue {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon11Pue::from_bits(val as u8)
    }
    #[doc = "Pull / Keep Select Field"]
    #[inline(always)]
    pub const fn set_pue(&mut self, val: super::vals::SwPadCtlPadGpioAon11Pue) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pus(&self) -> super::vals::SwPadCtlPadGpioAon11Pus {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon11Pus::from_bits(val as u8)
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[inline(always)]
    pub const fn set_pus(&mut self, val: super::vals::SwPadCtlPadGpioAon11Pus) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Field"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::SwPadCtlPadGpioAon11Ode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon11Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Field"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::SwPadCtlPadGpioAon11Ode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Domain write protection"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection"]
    #[inline(always)]
    pub const fn set_dwp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Domain write protection lock"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp_lock(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection lock"]
    #[inline(always)]
    pub const fn set_dwp_lock(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for SwPadCtlPadGpioAon11 {
    #[inline(always)]
    fn default() -> SwPadCtlPadGpioAon11 {
        SwPadCtlPadGpioAon11(10u64 as u32)
    }
}
impl core::fmt::Debug for SwPadCtlPadGpioAon11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwPadCtlPadGpioAon11")
            .field("sre", &self.sre())
            .field("dse", &self.dse())
            .field("pue", &self.pue())
            .field("pus", &self.pus())
            .field("ode", &self.ode())
            .field("dwp", &self.dwp())
            .field("dwp_lock", &self.dwp_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwPadCtlPadGpioAon11 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwPadCtlPadGpioAon11 {
            sre: super::vals::SwPadCtlPadGpioAon11Sre,
            dse: super::vals::SwPadCtlPadGpioAon11Dse,
            pue: super::vals::SwPadCtlPadGpioAon11Pue,
            pus: super::vals::SwPadCtlPadGpioAon11Pus,
            ode: super::vals::SwPadCtlPadGpioAon11Ode,
            dwp: u8,
            dwp_lock: u8,
        }
        let proxy = SwPadCtlPadGpioAon11 {
            sre: self.sre(),
            dse: self.dse(),
            pue: self.pue(),
            pus: self.pus(),
            ode: self.ode(),
            dwp: self.dwp(),
            dwp_lock: self.dwp_lock(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AON_12 SW PAD Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwPadCtlPadGpioAon12(pub u32);
impl SwPadCtlPadGpioAon12 {
    #[doc = "Slew Rate Field"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::SwPadCtlPadGpioAon12Sre {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon12Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Field"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::SwPadCtlPadGpioAon12Sre) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Drive Strength Field"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::SwPadCtlPadGpioAon12Dse {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon12Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Field"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::SwPadCtlPadGpioAon12Dse) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Pull / Keep Select Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pue(&self) -> super::vals::SwPadCtlPadGpioAon12Pue {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon12Pue::from_bits(val as u8)
    }
    #[doc = "Pull / Keep Select Field"]
    #[inline(always)]
    pub const fn set_pue(&mut self, val: super::vals::SwPadCtlPadGpioAon12Pue) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pus(&self) -> super::vals::SwPadCtlPadGpioAon12Pus {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon12Pus::from_bits(val as u8)
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[inline(always)]
    pub const fn set_pus(&mut self, val: super::vals::SwPadCtlPadGpioAon12Pus) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Field"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::SwPadCtlPadGpioAon12Ode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon12Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Field"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::SwPadCtlPadGpioAon12Ode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Domain write protection"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection"]
    #[inline(always)]
    pub const fn set_dwp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Domain write protection lock"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp_lock(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection lock"]
    #[inline(always)]
    pub const fn set_dwp_lock(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for SwPadCtlPadGpioAon12 {
    #[inline(always)]
    fn default() -> SwPadCtlPadGpioAon12 {
        SwPadCtlPadGpioAon12(14u64 as u32)
    }
}
impl core::fmt::Debug for SwPadCtlPadGpioAon12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwPadCtlPadGpioAon12")
            .field("sre", &self.sre())
            .field("dse", &self.dse())
            .field("pue", &self.pue())
            .field("pus", &self.pus())
            .field("ode", &self.ode())
            .field("dwp", &self.dwp())
            .field("dwp_lock", &self.dwp_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwPadCtlPadGpioAon12 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwPadCtlPadGpioAon12 {
            sre: super::vals::SwPadCtlPadGpioAon12Sre,
            dse: super::vals::SwPadCtlPadGpioAon12Dse,
            pue: super::vals::SwPadCtlPadGpioAon12Pue,
            pus: super::vals::SwPadCtlPadGpioAon12Pus,
            ode: super::vals::SwPadCtlPadGpioAon12Ode,
            dwp: u8,
            dwp_lock: u8,
        }
        let proxy = SwPadCtlPadGpioAon12 {
            sre: self.sre(),
            dse: self.dse(),
            pue: self.pue(),
            pus: self.pus(),
            ode: self.ode(),
            dwp: self.dwp(),
            dwp_lock: self.dwp_lock(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AON_13 SW PAD Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwPadCtlPadGpioAon13(pub u32);
impl SwPadCtlPadGpioAon13 {
    #[doc = "Slew Rate Field"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::SwPadCtlPadGpioAon13Sre {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon13Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Field"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::SwPadCtlPadGpioAon13Sre) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Drive Strength Field"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::SwPadCtlPadGpioAon13Dse {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon13Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Field"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::SwPadCtlPadGpioAon13Dse) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Pull / Keep Select Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pue(&self) -> super::vals::SwPadCtlPadGpioAon13Pue {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon13Pue::from_bits(val as u8)
    }
    #[doc = "Pull / Keep Select Field"]
    #[inline(always)]
    pub const fn set_pue(&mut self, val: super::vals::SwPadCtlPadGpioAon13Pue) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pus(&self) -> super::vals::SwPadCtlPadGpioAon13Pus {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon13Pus::from_bits(val as u8)
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[inline(always)]
    pub const fn set_pus(&mut self, val: super::vals::SwPadCtlPadGpioAon13Pus) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Field"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::SwPadCtlPadGpioAon13Ode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon13Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Field"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::SwPadCtlPadGpioAon13Ode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Domain write protection"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection"]
    #[inline(always)]
    pub const fn set_dwp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Domain write protection lock"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp_lock(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection lock"]
    #[inline(always)]
    pub const fn set_dwp_lock(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for SwPadCtlPadGpioAon13 {
    #[inline(always)]
    fn default() -> SwPadCtlPadGpioAon13 {
        SwPadCtlPadGpioAon13(6u64 as u32)
    }
}
impl core::fmt::Debug for SwPadCtlPadGpioAon13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwPadCtlPadGpioAon13")
            .field("sre", &self.sre())
            .field("dse", &self.dse())
            .field("pue", &self.pue())
            .field("pus", &self.pus())
            .field("ode", &self.ode())
            .field("dwp", &self.dwp())
            .field("dwp_lock", &self.dwp_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwPadCtlPadGpioAon13 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwPadCtlPadGpioAon13 {
            sre: super::vals::SwPadCtlPadGpioAon13Sre,
            dse: super::vals::SwPadCtlPadGpioAon13Dse,
            pue: super::vals::SwPadCtlPadGpioAon13Pue,
            pus: super::vals::SwPadCtlPadGpioAon13Pus,
            ode: super::vals::SwPadCtlPadGpioAon13Ode,
            dwp: u8,
            dwp_lock: u8,
        }
        let proxy = SwPadCtlPadGpioAon13 {
            sre: self.sre(),
            dse: self.dse(),
            pue: self.pue(),
            pus: self.pus(),
            ode: self.ode(),
            dwp: self.dwp(),
            dwp_lock: self.dwp_lock(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AON_14 SW PAD Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwPadCtlPadGpioAon14(pub u32);
impl SwPadCtlPadGpioAon14 {
    #[doc = "Slew Rate Field"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::SwPadCtlPadGpioAon14Sre {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon14Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Field"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::SwPadCtlPadGpioAon14Sre) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Drive Strength Field"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::SwPadCtlPadGpioAon14Dse {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon14Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Field"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::SwPadCtlPadGpioAon14Dse) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Pull / Keep Select Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pue(&self) -> super::vals::SwPadCtlPadGpioAon14Pue {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon14Pue::from_bits(val as u8)
    }
    #[doc = "Pull / Keep Select Field"]
    #[inline(always)]
    pub const fn set_pue(&mut self, val: super::vals::SwPadCtlPadGpioAon14Pue) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pus(&self) -> super::vals::SwPadCtlPadGpioAon14Pus {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon14Pus::from_bits(val as u8)
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[inline(always)]
    pub const fn set_pus(&mut self, val: super::vals::SwPadCtlPadGpioAon14Pus) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Field"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::SwPadCtlPadGpioAon14Ode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon14Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Field"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::SwPadCtlPadGpioAon14Ode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Domain write protection"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection"]
    #[inline(always)]
    pub const fn set_dwp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Domain write protection lock"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp_lock(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection lock"]
    #[inline(always)]
    pub const fn set_dwp_lock(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for SwPadCtlPadGpioAon14 {
    #[inline(always)]
    fn default() -> SwPadCtlPadGpioAon14 {
        SwPadCtlPadGpioAon14(14u64 as u32)
    }
}
impl core::fmt::Debug for SwPadCtlPadGpioAon14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwPadCtlPadGpioAon14")
            .field("sre", &self.sre())
            .field("dse", &self.dse())
            .field("pue", &self.pue())
            .field("pus", &self.pus())
            .field("ode", &self.ode())
            .field("dwp", &self.dwp())
            .field("dwp_lock", &self.dwp_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwPadCtlPadGpioAon14 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwPadCtlPadGpioAon14 {
            sre: super::vals::SwPadCtlPadGpioAon14Sre,
            dse: super::vals::SwPadCtlPadGpioAon14Dse,
            pue: super::vals::SwPadCtlPadGpioAon14Pue,
            pus: super::vals::SwPadCtlPadGpioAon14Pus,
            ode: super::vals::SwPadCtlPadGpioAon14Ode,
            dwp: u8,
            dwp_lock: u8,
        }
        let proxy = SwPadCtlPadGpioAon14 {
            sre: self.sre(),
            dse: self.dse(),
            pue: self.pue(),
            pus: self.pus(),
            ode: self.ode(),
            dwp: self.dwp(),
            dwp_lock: self.dwp_lock(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AON_15 SW PAD Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwPadCtlPadGpioAon15(pub u32);
impl SwPadCtlPadGpioAon15 {
    #[doc = "Slew Rate Field"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::SwPadCtlPadGpioAon15Sre {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon15Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Field"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::SwPadCtlPadGpioAon15Sre) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Drive Strength Field"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::SwPadCtlPadGpioAon15Dse {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon15Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Field"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::SwPadCtlPadGpioAon15Dse) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Pull / Keep Select Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pue(&self) -> super::vals::SwPadCtlPadGpioAon15Pue {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon15Pue::from_bits(val as u8)
    }
    #[doc = "Pull / Keep Select Field"]
    #[inline(always)]
    pub const fn set_pue(&mut self, val: super::vals::SwPadCtlPadGpioAon15Pue) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pus(&self) -> super::vals::SwPadCtlPadGpioAon15Pus {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon15Pus::from_bits(val as u8)
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[inline(always)]
    pub const fn set_pus(&mut self, val: super::vals::SwPadCtlPadGpioAon15Pus) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Field"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::SwPadCtlPadGpioAon15Ode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon15Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Field"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::SwPadCtlPadGpioAon15Ode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Domain write protection"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection"]
    #[inline(always)]
    pub const fn set_dwp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Domain write protection lock"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp_lock(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection lock"]
    #[inline(always)]
    pub const fn set_dwp_lock(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for SwPadCtlPadGpioAon15 {
    #[inline(always)]
    fn default() -> SwPadCtlPadGpioAon15 {
        SwPadCtlPadGpioAon15(6u64 as u32)
    }
}
impl core::fmt::Debug for SwPadCtlPadGpioAon15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwPadCtlPadGpioAon15")
            .field("sre", &self.sre())
            .field("dse", &self.dse())
            .field("pue", &self.pue())
            .field("pus", &self.pus())
            .field("ode", &self.ode())
            .field("dwp", &self.dwp())
            .field("dwp_lock", &self.dwp_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwPadCtlPadGpioAon15 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwPadCtlPadGpioAon15 {
            sre: super::vals::SwPadCtlPadGpioAon15Sre,
            dse: super::vals::SwPadCtlPadGpioAon15Dse,
            pue: super::vals::SwPadCtlPadGpioAon15Pue,
            pus: super::vals::SwPadCtlPadGpioAon15Pus,
            ode: super::vals::SwPadCtlPadGpioAon15Ode,
            dwp: u8,
            dwp_lock: u8,
        }
        let proxy = SwPadCtlPadGpioAon15 {
            sre: self.sre(),
            dse: self.dse(),
            pue: self.pue(),
            pus: self.pus(),
            ode: self.ode(),
            dwp: self.dwp(),
            dwp_lock: self.dwp_lock(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AON_16 SW PAD Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwPadCtlPadGpioAon16(pub u32);
impl SwPadCtlPadGpioAon16 {
    #[doc = "Slew Rate Field"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::SwPadCtlPadGpioAon16Sre {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon16Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Field"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::SwPadCtlPadGpioAon16Sre) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Drive Strength Field"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::SwPadCtlPadGpioAon16Dse {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon16Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Field"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::SwPadCtlPadGpioAon16Dse) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Pull / Keep Select Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pue(&self) -> super::vals::SwPadCtlPadGpioAon16Pue {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon16Pue::from_bits(val as u8)
    }
    #[doc = "Pull / Keep Select Field"]
    #[inline(always)]
    pub const fn set_pue(&mut self, val: super::vals::SwPadCtlPadGpioAon16Pue) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pus(&self) -> super::vals::SwPadCtlPadGpioAon16Pus {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon16Pus::from_bits(val as u8)
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[inline(always)]
    pub const fn set_pus(&mut self, val: super::vals::SwPadCtlPadGpioAon16Pus) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Field"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::SwPadCtlPadGpioAon16Ode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon16Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Field"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::SwPadCtlPadGpioAon16Ode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Domain write protection"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection"]
    #[inline(always)]
    pub const fn set_dwp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Domain write protection lock"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp_lock(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection lock"]
    #[inline(always)]
    pub const fn set_dwp_lock(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for SwPadCtlPadGpioAon16 {
    #[inline(always)]
    fn default() -> SwPadCtlPadGpioAon16 {
        SwPadCtlPadGpioAon16(6u64 as u32)
    }
}
impl core::fmt::Debug for SwPadCtlPadGpioAon16 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwPadCtlPadGpioAon16")
            .field("sre", &self.sre())
            .field("dse", &self.dse())
            .field("pue", &self.pue())
            .field("pus", &self.pus())
            .field("ode", &self.ode())
            .field("dwp", &self.dwp())
            .field("dwp_lock", &self.dwp_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwPadCtlPadGpioAon16 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwPadCtlPadGpioAon16 {
            sre: super::vals::SwPadCtlPadGpioAon16Sre,
            dse: super::vals::SwPadCtlPadGpioAon16Dse,
            pue: super::vals::SwPadCtlPadGpioAon16Pue,
            pus: super::vals::SwPadCtlPadGpioAon16Pus,
            ode: super::vals::SwPadCtlPadGpioAon16Ode,
            dwp: u8,
            dwp_lock: u8,
        }
        let proxy = SwPadCtlPadGpioAon16 {
            sre: self.sre(),
            dse: self.dse(),
            pue: self.pue(),
            pus: self.pus(),
            ode: self.ode(),
            dwp: self.dwp(),
            dwp_lock: self.dwp_lock(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AON_17 SW PAD Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwPadCtlPadGpioAon17(pub u32);
impl SwPadCtlPadGpioAon17 {
    #[doc = "Slew Rate Field"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::SwPadCtlPadGpioAon17Sre {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon17Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Field"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::SwPadCtlPadGpioAon17Sre) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Drive Strength Field"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::SwPadCtlPadGpioAon17Dse {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon17Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Field"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::SwPadCtlPadGpioAon17Dse) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Pull / Keep Select Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pue(&self) -> super::vals::SwPadCtlPadGpioAon17Pue {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon17Pue::from_bits(val as u8)
    }
    #[doc = "Pull / Keep Select Field"]
    #[inline(always)]
    pub const fn set_pue(&mut self, val: super::vals::SwPadCtlPadGpioAon17Pue) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pus(&self) -> super::vals::SwPadCtlPadGpioAon17Pus {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon17Pus::from_bits(val as u8)
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[inline(always)]
    pub const fn set_pus(&mut self, val: super::vals::SwPadCtlPadGpioAon17Pus) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Field"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::SwPadCtlPadGpioAon17Ode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon17Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Field"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::SwPadCtlPadGpioAon17Ode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Domain write protection"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection"]
    #[inline(always)]
    pub const fn set_dwp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Domain write protection lock"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp_lock(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection lock"]
    #[inline(always)]
    pub const fn set_dwp_lock(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for SwPadCtlPadGpioAon17 {
    #[inline(always)]
    fn default() -> SwPadCtlPadGpioAon17 {
        SwPadCtlPadGpioAon17(6u64 as u32)
    }
}
impl core::fmt::Debug for SwPadCtlPadGpioAon17 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwPadCtlPadGpioAon17")
            .field("sre", &self.sre())
            .field("dse", &self.dse())
            .field("pue", &self.pue())
            .field("pus", &self.pus())
            .field("ode", &self.ode())
            .field("dwp", &self.dwp())
            .field("dwp_lock", &self.dwp_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwPadCtlPadGpioAon17 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwPadCtlPadGpioAon17 {
            sre: super::vals::SwPadCtlPadGpioAon17Sre,
            dse: super::vals::SwPadCtlPadGpioAon17Dse,
            pue: super::vals::SwPadCtlPadGpioAon17Pue,
            pus: super::vals::SwPadCtlPadGpioAon17Pus,
            ode: super::vals::SwPadCtlPadGpioAon17Ode,
            dwp: u8,
            dwp_lock: u8,
        }
        let proxy = SwPadCtlPadGpioAon17 {
            sre: self.sre(),
            dse: self.dse(),
            pue: self.pue(),
            pus: self.pus(),
            ode: self.ode(),
            dwp: self.dwp(),
            dwp_lock: self.dwp_lock(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AON_18 SW PAD Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwPadCtlPadGpioAon18(pub u32);
impl SwPadCtlPadGpioAon18 {
    #[doc = "Slew Rate Field"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::SwPadCtlPadGpioAon18Sre {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon18Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Field"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::SwPadCtlPadGpioAon18Sre) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Drive Strength Field"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::SwPadCtlPadGpioAon18Dse {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon18Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Field"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::SwPadCtlPadGpioAon18Dse) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Pull / Keep Select Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pue(&self) -> super::vals::SwPadCtlPadGpioAon18Pue {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon18Pue::from_bits(val as u8)
    }
    #[doc = "Pull / Keep Select Field"]
    #[inline(always)]
    pub const fn set_pue(&mut self, val: super::vals::SwPadCtlPadGpioAon18Pue) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pus(&self) -> super::vals::SwPadCtlPadGpioAon18Pus {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon18Pus::from_bits(val as u8)
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[inline(always)]
    pub const fn set_pus(&mut self, val: super::vals::SwPadCtlPadGpioAon18Pus) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Field"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::SwPadCtlPadGpioAon18Ode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon18Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Field"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::SwPadCtlPadGpioAon18Ode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Domain write protection"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection"]
    #[inline(always)]
    pub const fn set_dwp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Domain write protection lock"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp_lock(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection lock"]
    #[inline(always)]
    pub const fn set_dwp_lock(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for SwPadCtlPadGpioAon18 {
    #[inline(always)]
    fn default() -> SwPadCtlPadGpioAon18 {
        SwPadCtlPadGpioAon18(6u64 as u32)
    }
}
impl core::fmt::Debug for SwPadCtlPadGpioAon18 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwPadCtlPadGpioAon18")
            .field("sre", &self.sre())
            .field("dse", &self.dse())
            .field("pue", &self.pue())
            .field("pus", &self.pus())
            .field("ode", &self.ode())
            .field("dwp", &self.dwp())
            .field("dwp_lock", &self.dwp_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwPadCtlPadGpioAon18 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwPadCtlPadGpioAon18 {
            sre: super::vals::SwPadCtlPadGpioAon18Sre,
            dse: super::vals::SwPadCtlPadGpioAon18Dse,
            pue: super::vals::SwPadCtlPadGpioAon18Pue,
            pus: super::vals::SwPadCtlPadGpioAon18Pus,
            ode: super::vals::SwPadCtlPadGpioAon18Ode,
            dwp: u8,
            dwp_lock: u8,
        }
        let proxy = SwPadCtlPadGpioAon18 {
            sre: self.sre(),
            dse: self.dse(),
            pue: self.pue(),
            pus: self.pus(),
            ode: self.ode(),
            dwp: self.dwp(),
            dwp_lock: self.dwp_lock(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AON_19 SW PAD Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwPadCtlPadGpioAon19(pub u32);
impl SwPadCtlPadGpioAon19 {
    #[doc = "Slew Rate Field"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::SwPadCtlPadGpioAon19Sre {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon19Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Field"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::SwPadCtlPadGpioAon19Sre) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Drive Strength Field"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::SwPadCtlPadGpioAon19Dse {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon19Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Field"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::SwPadCtlPadGpioAon19Dse) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Pull / Keep Select Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pue(&self) -> super::vals::SwPadCtlPadGpioAon19Pue {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon19Pue::from_bits(val as u8)
    }
    #[doc = "Pull / Keep Select Field"]
    #[inline(always)]
    pub const fn set_pue(&mut self, val: super::vals::SwPadCtlPadGpioAon19Pue) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pus(&self) -> super::vals::SwPadCtlPadGpioAon19Pus {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon19Pus::from_bits(val as u8)
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[inline(always)]
    pub const fn set_pus(&mut self, val: super::vals::SwPadCtlPadGpioAon19Pus) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Field"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::SwPadCtlPadGpioAon19Ode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon19Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Field"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::SwPadCtlPadGpioAon19Ode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Domain write protection"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection"]
    #[inline(always)]
    pub const fn set_dwp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Domain write protection lock"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp_lock(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection lock"]
    #[inline(always)]
    pub const fn set_dwp_lock(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for SwPadCtlPadGpioAon19 {
    #[inline(always)]
    fn default() -> SwPadCtlPadGpioAon19 {
        SwPadCtlPadGpioAon19(14u64 as u32)
    }
}
impl core::fmt::Debug for SwPadCtlPadGpioAon19 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwPadCtlPadGpioAon19")
            .field("sre", &self.sre())
            .field("dse", &self.dse())
            .field("pue", &self.pue())
            .field("pus", &self.pus())
            .field("ode", &self.ode())
            .field("dwp", &self.dwp())
            .field("dwp_lock", &self.dwp_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwPadCtlPadGpioAon19 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwPadCtlPadGpioAon19 {
            sre: super::vals::SwPadCtlPadGpioAon19Sre,
            dse: super::vals::SwPadCtlPadGpioAon19Dse,
            pue: super::vals::SwPadCtlPadGpioAon19Pue,
            pus: super::vals::SwPadCtlPadGpioAon19Pus,
            ode: super::vals::SwPadCtlPadGpioAon19Ode,
            dwp: u8,
            dwp_lock: u8,
        }
        let proxy = SwPadCtlPadGpioAon19 {
            sre: self.sre(),
            dse: self.dse(),
            pue: self.pue(),
            pus: self.pus(),
            ode: self.ode(),
            dwp: self.dwp(),
            dwp_lock: self.dwp_lock(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AON_20 SW PAD Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwPadCtlPadGpioAon20(pub u32);
impl SwPadCtlPadGpioAon20 {
    #[doc = "Slew Rate Field"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::SwPadCtlPadGpioAon20Sre {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon20Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Field"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::SwPadCtlPadGpioAon20Sre) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Drive Strength Field"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::SwPadCtlPadGpioAon20Dse {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon20Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Field"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::SwPadCtlPadGpioAon20Dse) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Pull / Keep Select Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pue(&self) -> super::vals::SwPadCtlPadGpioAon20Pue {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon20Pue::from_bits(val as u8)
    }
    #[doc = "Pull / Keep Select Field"]
    #[inline(always)]
    pub const fn set_pue(&mut self, val: super::vals::SwPadCtlPadGpioAon20Pue) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pus(&self) -> super::vals::SwPadCtlPadGpioAon20Pus {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon20Pus::from_bits(val as u8)
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[inline(always)]
    pub const fn set_pus(&mut self, val: super::vals::SwPadCtlPadGpioAon20Pus) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Field"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::SwPadCtlPadGpioAon20Ode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon20Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Field"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::SwPadCtlPadGpioAon20Ode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Domain write protection"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection"]
    #[inline(always)]
    pub const fn set_dwp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Domain write protection lock"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp_lock(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection lock"]
    #[inline(always)]
    pub const fn set_dwp_lock(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for SwPadCtlPadGpioAon20 {
    #[inline(always)]
    fn default() -> SwPadCtlPadGpioAon20 {
        SwPadCtlPadGpioAon20(14u64 as u32)
    }
}
impl core::fmt::Debug for SwPadCtlPadGpioAon20 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwPadCtlPadGpioAon20")
            .field("sre", &self.sre())
            .field("dse", &self.dse())
            .field("pue", &self.pue())
            .field("pus", &self.pus())
            .field("ode", &self.ode())
            .field("dwp", &self.dwp())
            .field("dwp_lock", &self.dwp_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwPadCtlPadGpioAon20 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwPadCtlPadGpioAon20 {
            sre: super::vals::SwPadCtlPadGpioAon20Sre,
            dse: super::vals::SwPadCtlPadGpioAon20Dse,
            pue: super::vals::SwPadCtlPadGpioAon20Pue,
            pus: super::vals::SwPadCtlPadGpioAon20Pus,
            ode: super::vals::SwPadCtlPadGpioAon20Ode,
            dwp: u8,
            dwp_lock: u8,
        }
        let proxy = SwPadCtlPadGpioAon20 {
            sre: self.sre(),
            dse: self.dse(),
            pue: self.pue(),
            pus: self.pus(),
            ode: self.ode(),
            dwp: self.dwp(),
            dwp_lock: self.dwp_lock(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AON_21 SW PAD Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwPadCtlPadGpioAon21(pub u32);
impl SwPadCtlPadGpioAon21 {
    #[doc = "Slew Rate Field"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::SwPadCtlPadGpioAon21Sre {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon21Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Field"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::SwPadCtlPadGpioAon21Sre) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Drive Strength Field"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::SwPadCtlPadGpioAon21Dse {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon21Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Field"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::SwPadCtlPadGpioAon21Dse) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Pull / Keep Select Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pue(&self) -> super::vals::SwPadCtlPadGpioAon21Pue {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon21Pue::from_bits(val as u8)
    }
    #[doc = "Pull / Keep Select Field"]
    #[inline(always)]
    pub const fn set_pue(&mut self, val: super::vals::SwPadCtlPadGpioAon21Pue) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pus(&self) -> super::vals::SwPadCtlPadGpioAon21Pus {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon21Pus::from_bits(val as u8)
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[inline(always)]
    pub const fn set_pus(&mut self, val: super::vals::SwPadCtlPadGpioAon21Pus) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Field"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::SwPadCtlPadGpioAon21Ode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon21Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Field"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::SwPadCtlPadGpioAon21Ode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Domain write protection"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection"]
    #[inline(always)]
    pub const fn set_dwp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Domain write protection lock"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp_lock(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection lock"]
    #[inline(always)]
    pub const fn set_dwp_lock(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for SwPadCtlPadGpioAon21 {
    #[inline(always)]
    fn default() -> SwPadCtlPadGpioAon21 {
        SwPadCtlPadGpioAon21(14u64 as u32)
    }
}
impl core::fmt::Debug for SwPadCtlPadGpioAon21 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwPadCtlPadGpioAon21")
            .field("sre", &self.sre())
            .field("dse", &self.dse())
            .field("pue", &self.pue())
            .field("pus", &self.pus())
            .field("ode", &self.ode())
            .field("dwp", &self.dwp())
            .field("dwp_lock", &self.dwp_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwPadCtlPadGpioAon21 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwPadCtlPadGpioAon21 {
            sre: super::vals::SwPadCtlPadGpioAon21Sre,
            dse: super::vals::SwPadCtlPadGpioAon21Dse,
            pue: super::vals::SwPadCtlPadGpioAon21Pue,
            pus: super::vals::SwPadCtlPadGpioAon21Pus,
            ode: super::vals::SwPadCtlPadGpioAon21Ode,
            dwp: u8,
            dwp_lock: u8,
        }
        let proxy = SwPadCtlPadGpioAon21 {
            sre: self.sre(),
            dse: self.dse(),
            pue: self.pue(),
            pus: self.pus(),
            ode: self.ode(),
            dwp: self.dwp(),
            dwp_lock: self.dwp_lock(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AON_22 SW PAD Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwPadCtlPadGpioAon22(pub u32);
impl SwPadCtlPadGpioAon22 {
    #[doc = "Slew Rate Field"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::SwPadCtlPadGpioAon22Sre {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon22Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Field"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::SwPadCtlPadGpioAon22Sre) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Drive Strength Field"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::SwPadCtlPadGpioAon22Dse {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon22Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Field"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::SwPadCtlPadGpioAon22Dse) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Pull / Keep Select Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pue(&self) -> super::vals::SwPadCtlPadGpioAon22Pue {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon22Pue::from_bits(val as u8)
    }
    #[doc = "Pull / Keep Select Field"]
    #[inline(always)]
    pub const fn set_pue(&mut self, val: super::vals::SwPadCtlPadGpioAon22Pue) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pus(&self) -> super::vals::SwPadCtlPadGpioAon22Pus {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon22Pus::from_bits(val as u8)
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[inline(always)]
    pub const fn set_pus(&mut self, val: super::vals::SwPadCtlPadGpioAon22Pus) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Field"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::SwPadCtlPadGpioAon22Ode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon22Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Field"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::SwPadCtlPadGpioAon22Ode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Domain write protection"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection"]
    #[inline(always)]
    pub const fn set_dwp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Domain write protection lock"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp_lock(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection lock"]
    #[inline(always)]
    pub const fn set_dwp_lock(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for SwPadCtlPadGpioAon22 {
    #[inline(always)]
    fn default() -> SwPadCtlPadGpioAon22 {
        SwPadCtlPadGpioAon22(14u64 as u32)
    }
}
impl core::fmt::Debug for SwPadCtlPadGpioAon22 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwPadCtlPadGpioAon22")
            .field("sre", &self.sre())
            .field("dse", &self.dse())
            .field("pue", &self.pue())
            .field("pus", &self.pus())
            .field("ode", &self.ode())
            .field("dwp", &self.dwp())
            .field("dwp_lock", &self.dwp_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwPadCtlPadGpioAon22 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwPadCtlPadGpioAon22 {
            sre: super::vals::SwPadCtlPadGpioAon22Sre,
            dse: super::vals::SwPadCtlPadGpioAon22Dse,
            pue: super::vals::SwPadCtlPadGpioAon22Pue,
            pus: super::vals::SwPadCtlPadGpioAon22Pus,
            ode: super::vals::SwPadCtlPadGpioAon22Ode,
            dwp: u8,
            dwp_lock: u8,
        }
        let proxy = SwPadCtlPadGpioAon22 {
            sre: self.sre(),
            dse: self.dse(),
            pue: self.pue(),
            pus: self.pus(),
            ode: self.ode(),
            dwp: self.dwp(),
            dwp_lock: self.dwp_lock(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AON_23 SW PAD Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwPadCtlPadGpioAon23(pub u32);
impl SwPadCtlPadGpioAon23 {
    #[doc = "Slew Rate Field"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::SwPadCtlPadGpioAon23Sre {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon23Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Field"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::SwPadCtlPadGpioAon23Sre) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Drive Strength Field"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::SwPadCtlPadGpioAon23Dse {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon23Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Field"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::SwPadCtlPadGpioAon23Dse) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Pull / Keep Select Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pue(&self) -> super::vals::SwPadCtlPadGpioAon23Pue {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon23Pue::from_bits(val as u8)
    }
    #[doc = "Pull / Keep Select Field"]
    #[inline(always)]
    pub const fn set_pue(&mut self, val: super::vals::SwPadCtlPadGpioAon23Pue) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pus(&self) -> super::vals::SwPadCtlPadGpioAon23Pus {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon23Pus::from_bits(val as u8)
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[inline(always)]
    pub const fn set_pus(&mut self, val: super::vals::SwPadCtlPadGpioAon23Pus) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Field"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::SwPadCtlPadGpioAon23Ode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon23Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Field"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::SwPadCtlPadGpioAon23Ode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Domain write protection"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection"]
    #[inline(always)]
    pub const fn set_dwp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Domain write protection lock"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp_lock(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection lock"]
    #[inline(always)]
    pub const fn set_dwp_lock(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for SwPadCtlPadGpioAon23 {
    #[inline(always)]
    fn default() -> SwPadCtlPadGpioAon23 {
        SwPadCtlPadGpioAon23(6u64 as u32)
    }
}
impl core::fmt::Debug for SwPadCtlPadGpioAon23 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwPadCtlPadGpioAon23")
            .field("sre", &self.sre())
            .field("dse", &self.dse())
            .field("pue", &self.pue())
            .field("pus", &self.pus())
            .field("ode", &self.ode())
            .field("dwp", &self.dwp())
            .field("dwp_lock", &self.dwp_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwPadCtlPadGpioAon23 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwPadCtlPadGpioAon23 {
            sre: super::vals::SwPadCtlPadGpioAon23Sre,
            dse: super::vals::SwPadCtlPadGpioAon23Dse,
            pue: super::vals::SwPadCtlPadGpioAon23Pue,
            pus: super::vals::SwPadCtlPadGpioAon23Pus,
            ode: super::vals::SwPadCtlPadGpioAon23Ode,
            dwp: u8,
            dwp_lock: u8,
        }
        let proxy = SwPadCtlPadGpioAon23 {
            sre: self.sre(),
            dse: self.dse(),
            pue: self.pue(),
            pus: self.pus(),
            ode: self.ode(),
            dwp: self.dwp(),
            dwp_lock: self.dwp_lock(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AON_24 SW PAD Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwPadCtlPadGpioAon24(pub u32);
impl SwPadCtlPadGpioAon24 {
    #[doc = "Slew Rate Field"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::SwPadCtlPadGpioAon24Sre {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon24Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Field"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::SwPadCtlPadGpioAon24Sre) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Drive Strength Field"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::SwPadCtlPadGpioAon24Dse {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon24Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Field"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::SwPadCtlPadGpioAon24Dse) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Pull / Keep Select Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pue(&self) -> super::vals::SwPadCtlPadGpioAon24Pue {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon24Pue::from_bits(val as u8)
    }
    #[doc = "Pull / Keep Select Field"]
    #[inline(always)]
    pub const fn set_pue(&mut self, val: super::vals::SwPadCtlPadGpioAon24Pue) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pus(&self) -> super::vals::SwPadCtlPadGpioAon24Pus {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon24Pus::from_bits(val as u8)
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[inline(always)]
    pub const fn set_pus(&mut self, val: super::vals::SwPadCtlPadGpioAon24Pus) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Field"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::SwPadCtlPadGpioAon24Ode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon24Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Field"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::SwPadCtlPadGpioAon24Ode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Domain write protection"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection"]
    #[inline(always)]
    pub const fn set_dwp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Domain write protection lock"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp_lock(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection lock"]
    #[inline(always)]
    pub const fn set_dwp_lock(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for SwPadCtlPadGpioAon24 {
    #[inline(always)]
    fn default() -> SwPadCtlPadGpioAon24 {
        SwPadCtlPadGpioAon24(6u64 as u32)
    }
}
impl core::fmt::Debug for SwPadCtlPadGpioAon24 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwPadCtlPadGpioAon24")
            .field("sre", &self.sre())
            .field("dse", &self.dse())
            .field("pue", &self.pue())
            .field("pus", &self.pus())
            .field("ode", &self.ode())
            .field("dwp", &self.dwp())
            .field("dwp_lock", &self.dwp_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwPadCtlPadGpioAon24 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwPadCtlPadGpioAon24 {
            sre: super::vals::SwPadCtlPadGpioAon24Sre,
            dse: super::vals::SwPadCtlPadGpioAon24Dse,
            pue: super::vals::SwPadCtlPadGpioAon24Pue,
            pus: super::vals::SwPadCtlPadGpioAon24Pus,
            ode: super::vals::SwPadCtlPadGpioAon24Ode,
            dwp: u8,
            dwp_lock: u8,
        }
        let proxy = SwPadCtlPadGpioAon24 {
            sre: self.sre(),
            dse: self.dse(),
            pue: self.pue(),
            pus: self.pus(),
            ode: self.ode(),
            dwp: self.dwp(),
            dwp_lock: self.dwp_lock(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AON_25 SW PAD Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwPadCtlPadGpioAon25(pub u32);
impl SwPadCtlPadGpioAon25 {
    #[doc = "Slew Rate Field"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::SwPadCtlPadGpioAon25Sre {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon25Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Field"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::SwPadCtlPadGpioAon25Sre) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Drive Strength Field"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::SwPadCtlPadGpioAon25Dse {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon25Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Field"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::SwPadCtlPadGpioAon25Dse) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Pull / Keep Select Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pue(&self) -> super::vals::SwPadCtlPadGpioAon25Pue {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon25Pue::from_bits(val as u8)
    }
    #[doc = "Pull / Keep Select Field"]
    #[inline(always)]
    pub const fn set_pue(&mut self, val: super::vals::SwPadCtlPadGpioAon25Pue) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pus(&self) -> super::vals::SwPadCtlPadGpioAon25Pus {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon25Pus::from_bits(val as u8)
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[inline(always)]
    pub const fn set_pus(&mut self, val: super::vals::SwPadCtlPadGpioAon25Pus) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Field"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::SwPadCtlPadGpioAon25Ode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon25Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Field"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::SwPadCtlPadGpioAon25Ode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Domain write protection"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection"]
    #[inline(always)]
    pub const fn set_dwp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Domain write protection lock"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp_lock(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection lock"]
    #[inline(always)]
    pub const fn set_dwp_lock(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for SwPadCtlPadGpioAon25 {
    #[inline(always)]
    fn default() -> SwPadCtlPadGpioAon25 {
        SwPadCtlPadGpioAon25(6u64 as u32)
    }
}
impl core::fmt::Debug for SwPadCtlPadGpioAon25 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwPadCtlPadGpioAon25")
            .field("sre", &self.sre())
            .field("dse", &self.dse())
            .field("pue", &self.pue())
            .field("pus", &self.pus())
            .field("ode", &self.ode())
            .field("dwp", &self.dwp())
            .field("dwp_lock", &self.dwp_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwPadCtlPadGpioAon25 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwPadCtlPadGpioAon25 {
            sre: super::vals::SwPadCtlPadGpioAon25Sre,
            dse: super::vals::SwPadCtlPadGpioAon25Dse,
            pue: super::vals::SwPadCtlPadGpioAon25Pue,
            pus: super::vals::SwPadCtlPadGpioAon25Pus,
            ode: super::vals::SwPadCtlPadGpioAon25Ode,
            dwp: u8,
            dwp_lock: u8,
        }
        let proxy = SwPadCtlPadGpioAon25 {
            sre: self.sre(),
            dse: self.dse(),
            pue: self.pue(),
            pus: self.pus(),
            ode: self.ode(),
            dwp: self.dwp(),
            dwp_lock: self.dwp_lock(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AON_26 SW PAD Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwPadCtlPadGpioAon26(pub u32);
impl SwPadCtlPadGpioAon26 {
    #[doc = "Slew Rate Field"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::SwPadCtlPadGpioAon26Sre {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon26Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Field"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::SwPadCtlPadGpioAon26Sre) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Drive Strength Field"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::SwPadCtlPadGpioAon26Dse {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon26Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Field"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::SwPadCtlPadGpioAon26Dse) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Pull / Keep Select Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pue(&self) -> super::vals::SwPadCtlPadGpioAon26Pue {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon26Pue::from_bits(val as u8)
    }
    #[doc = "Pull / Keep Select Field"]
    #[inline(always)]
    pub const fn set_pue(&mut self, val: super::vals::SwPadCtlPadGpioAon26Pue) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pus(&self) -> super::vals::SwPadCtlPadGpioAon26Pus {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon26Pus::from_bits(val as u8)
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[inline(always)]
    pub const fn set_pus(&mut self, val: super::vals::SwPadCtlPadGpioAon26Pus) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Field"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::SwPadCtlPadGpioAon26Ode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon26Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Field"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::SwPadCtlPadGpioAon26Ode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Domain write protection"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection"]
    #[inline(always)]
    pub const fn set_dwp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Domain write protection lock"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp_lock(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection lock"]
    #[inline(always)]
    pub const fn set_dwp_lock(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for SwPadCtlPadGpioAon26 {
    #[inline(always)]
    fn default() -> SwPadCtlPadGpioAon26 {
        SwPadCtlPadGpioAon26(6u64 as u32)
    }
}
impl core::fmt::Debug for SwPadCtlPadGpioAon26 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwPadCtlPadGpioAon26")
            .field("sre", &self.sre())
            .field("dse", &self.dse())
            .field("pue", &self.pue())
            .field("pus", &self.pus())
            .field("ode", &self.ode())
            .field("dwp", &self.dwp())
            .field("dwp_lock", &self.dwp_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwPadCtlPadGpioAon26 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwPadCtlPadGpioAon26 {
            sre: super::vals::SwPadCtlPadGpioAon26Sre,
            dse: super::vals::SwPadCtlPadGpioAon26Dse,
            pue: super::vals::SwPadCtlPadGpioAon26Pue,
            pus: super::vals::SwPadCtlPadGpioAon26Pus,
            ode: super::vals::SwPadCtlPadGpioAon26Ode,
            dwp: u8,
            dwp_lock: u8,
        }
        let proxy = SwPadCtlPadGpioAon26 {
            sre: self.sre(),
            dse: self.dse(),
            pue: self.pue(),
            pus: self.pus(),
            ode: self.ode(),
            dwp: self.dwp(),
            dwp_lock: self.dwp_lock(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AON_27 SW PAD Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwPadCtlPadGpioAon27(pub u32);
impl SwPadCtlPadGpioAon27 {
    #[doc = "Slew Rate Field"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::SwPadCtlPadGpioAon27Sre {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon27Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Field"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::SwPadCtlPadGpioAon27Sre) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Drive Strength Field"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::SwPadCtlPadGpioAon27Dse {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon27Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Field"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::SwPadCtlPadGpioAon27Dse) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Pull / Keep Select Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pue(&self) -> super::vals::SwPadCtlPadGpioAon27Pue {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon27Pue::from_bits(val as u8)
    }
    #[doc = "Pull / Keep Select Field"]
    #[inline(always)]
    pub const fn set_pue(&mut self, val: super::vals::SwPadCtlPadGpioAon27Pue) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pus(&self) -> super::vals::SwPadCtlPadGpioAon27Pus {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon27Pus::from_bits(val as u8)
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[inline(always)]
    pub const fn set_pus(&mut self, val: super::vals::SwPadCtlPadGpioAon27Pus) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Field"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::SwPadCtlPadGpioAon27Ode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon27Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Field"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::SwPadCtlPadGpioAon27Ode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Domain write protection"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection"]
    #[inline(always)]
    pub const fn set_dwp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Domain write protection lock"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp_lock(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection lock"]
    #[inline(always)]
    pub const fn set_dwp_lock(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for SwPadCtlPadGpioAon27 {
    #[inline(always)]
    fn default() -> SwPadCtlPadGpioAon27 {
        SwPadCtlPadGpioAon27(14u64 as u32)
    }
}
impl core::fmt::Debug for SwPadCtlPadGpioAon27 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwPadCtlPadGpioAon27")
            .field("sre", &self.sre())
            .field("dse", &self.dse())
            .field("pue", &self.pue())
            .field("pus", &self.pus())
            .field("ode", &self.ode())
            .field("dwp", &self.dwp())
            .field("dwp_lock", &self.dwp_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwPadCtlPadGpioAon27 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwPadCtlPadGpioAon27 {
            sre: super::vals::SwPadCtlPadGpioAon27Sre,
            dse: super::vals::SwPadCtlPadGpioAon27Dse,
            pue: super::vals::SwPadCtlPadGpioAon27Pue,
            pus: super::vals::SwPadCtlPadGpioAon27Pus,
            ode: super::vals::SwPadCtlPadGpioAon27Ode,
            dwp: u8,
            dwp_lock: u8,
        }
        let proxy = SwPadCtlPadGpioAon27 {
            sre: self.sre(),
            dse: self.dse(),
            pue: self.pue(),
            pus: self.pus(),
            ode: self.ode(),
            dwp: self.dwp(),
            dwp_lock: self.dwp_lock(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AON_28_DUMMY SW PAD Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwPadCtlPadGpioAon28(pub u32);
impl SwPadCtlPadGpioAon28 {
    #[doc = "Slew Rate Field"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::SwPadCtlPadGpioAon28Sre {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon28Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Field"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::SwPadCtlPadGpioAon28Sre) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Drive Strength Field"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::SwPadCtlPadGpioAon28Dse {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon28Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Field"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::SwPadCtlPadGpioAon28Dse) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Pull / Keep Select Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pue(&self) -> super::vals::SwPadCtlPadGpioAon28Pue {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon28Pue::from_bits(val as u8)
    }
    #[doc = "Pull / Keep Select Field"]
    #[inline(always)]
    pub const fn set_pue(&mut self, val: super::vals::SwPadCtlPadGpioAon28Pue) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pus(&self) -> super::vals::SwPadCtlPadGpioAon28Pus {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon28Pus::from_bits(val as u8)
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[inline(always)]
    pub const fn set_pus(&mut self, val: super::vals::SwPadCtlPadGpioAon28Pus) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Field"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::SwPadCtlPadGpioAon28Ode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::SwPadCtlPadGpioAon28Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Field"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::SwPadCtlPadGpioAon28Ode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Domain write protection"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection"]
    #[inline(always)]
    pub const fn set_dwp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Domain write protection lock"]
    #[must_use]
    #[inline(always)]
    pub const fn dwp_lock(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain write protection lock"]
    #[inline(always)]
    pub const fn set_dwp_lock(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for SwPadCtlPadGpioAon28 {
    #[inline(always)]
    fn default() -> SwPadCtlPadGpioAon28 {
        SwPadCtlPadGpioAon28(6u64 as u32)
    }
}
impl core::fmt::Debug for SwPadCtlPadGpioAon28 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwPadCtlPadGpioAon28")
            .field("sre", &self.sre())
            .field("dse", &self.dse())
            .field("pue", &self.pue())
            .field("pus", &self.pus())
            .field("ode", &self.ode())
            .field("dwp", &self.dwp())
            .field("dwp_lock", &self.dwp_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwPadCtlPadGpioAon28 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwPadCtlPadGpioAon28 {
            sre: super::vals::SwPadCtlPadGpioAon28Sre,
            dse: super::vals::SwPadCtlPadGpioAon28Dse,
            pue: super::vals::SwPadCtlPadGpioAon28Pue,
            pus: super::vals::SwPadCtlPadGpioAon28Pus,
            ode: super::vals::SwPadCtlPadGpioAon28Ode,
            dwp: u8,
            dwp_lock: u8,
        }
        let proxy = SwPadCtlPadGpioAon28 {
            sre: self.sre(),
            dse: self.dse(),
            pue: self.pue(),
            pus: self.pus(),
            ode: self.ode(),
            dwp: self.dwp(),
            dwp_lock: self.dwp_lock(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
