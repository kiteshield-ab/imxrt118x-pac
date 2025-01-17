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
