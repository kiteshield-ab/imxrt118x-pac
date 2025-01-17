#[doc = "CAN1_IPP_IND_CANRX_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Can1IppIndCanrxSelectInput(pub u32);
impl Can1IppIndCanrxSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Can1IppIndCanrxSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Can1IppIndCanrxSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Can1IppIndCanrxSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Can1IppIndCanrxSelectInput {
    #[inline(always)]
    fn default() -> Can1IppIndCanrxSelectInput {
        Can1IppIndCanrxSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Can1IppIndCanrxSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Can1IppIndCanrxSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Can1IppIndCanrxSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Can1IppIndCanrxSelectInput {
            daisy: super::vals::Can1IppIndCanrxSelectInputDaisy,
        }
        let proxy = Can1IppIndCanrxSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "CAN2_IPP_IND_CANRX_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Can2IppIndCanrxSelectInput(pub u32);
impl Can2IppIndCanrxSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Can2IppIndCanrxSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Can2IppIndCanrxSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Can2IppIndCanrxSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Can2IppIndCanrxSelectInput {
    #[inline(always)]
    fn default() -> Can2IppIndCanrxSelectInput {
        Can2IppIndCanrxSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Can2IppIndCanrxSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Can2IppIndCanrxSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Can2IppIndCanrxSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Can2IppIndCanrxSelectInput {
            daisy: super::vals::Can2IppIndCanrxSelectInputDaisy,
        }
        let proxy = Can2IppIndCanrxSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "CAN3_IPP_IND_CANRX_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Can3IppIndCanrxSelectInput(pub u32);
impl Can3IppIndCanrxSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Can3IppIndCanrxSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Can3IppIndCanrxSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Can3IppIndCanrxSelectInputDaisy) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Can3IppIndCanrxSelectInput {
    #[inline(always)]
    fn default() -> Can3IppIndCanrxSelectInput {
        Can3IppIndCanrxSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Can3IppIndCanrxSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Can3IppIndCanrxSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Can3IppIndCanrxSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Can3IppIndCanrxSelectInput {
            daisy: super::vals::Can3IppIndCanrxSelectInputDaisy,
        }
        let proxy = Can3IppIndCanrxSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ECAT_ECAT_RX_CLK_0_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EcatEcatRxClk0SelectInput(pub u32);
impl EcatEcatRxClk0SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::EcatEcatRxClk0SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::EcatEcatRxClk0SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::EcatEcatRxClk0SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for EcatEcatRxClk0SelectInput {
    #[inline(always)]
    fn default() -> EcatEcatRxClk0SelectInput {
        EcatEcatRxClk0SelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for EcatEcatRxClk0SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EcatEcatRxClk0SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EcatEcatRxClk0SelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EcatEcatRxClk0SelectInput {
            daisy: super::vals::EcatEcatRxClk0SelectInputDaisy,
        }
        let proxy = EcatEcatRxClk0SelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ECAT_ECAT_RX_CLK_1_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EcatEcatRxClk1SelectInput(pub u32);
impl EcatEcatRxClk1SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::EcatEcatRxClk1SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::EcatEcatRxClk1SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::EcatEcatRxClk1SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for EcatEcatRxClk1SelectInput {
    #[inline(always)]
    fn default() -> EcatEcatRxClk1SelectInput {
        EcatEcatRxClk1SelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for EcatEcatRxClk1SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EcatEcatRxClk1SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EcatEcatRxClk1SelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EcatEcatRxClk1SelectInput {
            daisy: super::vals::EcatEcatRxClk1SelectInputDaisy,
        }
        let proxy = EcatEcatRxClk1SelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ECAT_ECAT_RX_DATA0_0_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EcatEcatRxData00SelectInput(pub u32);
impl EcatEcatRxData00SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::EcatEcatRxData00SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::EcatEcatRxData00SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::EcatEcatRxData00SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for EcatEcatRxData00SelectInput {
    #[inline(always)]
    fn default() -> EcatEcatRxData00SelectInput {
        EcatEcatRxData00SelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for EcatEcatRxData00SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EcatEcatRxData00SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EcatEcatRxData00SelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EcatEcatRxData00SelectInput {
            daisy: super::vals::EcatEcatRxData00SelectInputDaisy,
        }
        let proxy = EcatEcatRxData00SelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ECAT_ECAT_RX_DATA0_1_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EcatEcatRxData01SelectInput(pub u32);
impl EcatEcatRxData01SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::EcatEcatRxData01SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::EcatEcatRxData01SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::EcatEcatRxData01SelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for EcatEcatRxData01SelectInput {
    #[inline(always)]
    fn default() -> EcatEcatRxData01SelectInput {
        EcatEcatRxData01SelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for EcatEcatRxData01SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EcatEcatRxData01SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EcatEcatRxData01SelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EcatEcatRxData01SelectInput {
            daisy: super::vals::EcatEcatRxData01SelectInputDaisy,
        }
        let proxy = EcatEcatRxData01SelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ECAT_ECAT_RX_DATA1_0_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EcatEcatRxData10SelectInput(pub u32);
impl EcatEcatRxData10SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::EcatEcatRxData10SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::EcatEcatRxData10SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::EcatEcatRxData10SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for EcatEcatRxData10SelectInput {
    #[inline(always)]
    fn default() -> EcatEcatRxData10SelectInput {
        EcatEcatRxData10SelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for EcatEcatRxData10SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EcatEcatRxData10SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EcatEcatRxData10SelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EcatEcatRxData10SelectInput {
            daisy: super::vals::EcatEcatRxData10SelectInputDaisy,
        }
        let proxy = EcatEcatRxData10SelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ECAT_ECAT_RX_DATA1_1_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EcatEcatRxData11SelectInput(pub u32);
impl EcatEcatRxData11SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::EcatEcatRxData11SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::EcatEcatRxData11SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::EcatEcatRxData11SelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for EcatEcatRxData11SelectInput {
    #[inline(always)]
    fn default() -> EcatEcatRxData11SelectInput {
        EcatEcatRxData11SelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for EcatEcatRxData11SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EcatEcatRxData11SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EcatEcatRxData11SelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EcatEcatRxData11SelectInput {
            daisy: super::vals::EcatEcatRxData11SelectInputDaisy,
        }
        let proxy = EcatEcatRxData11SelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ECAT_ECAT_RX_DATA2_0_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EcatEcatRxData20SelectInput(pub u32);
impl EcatEcatRxData20SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::EcatEcatRxData20SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::EcatEcatRxData20SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::EcatEcatRxData20SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for EcatEcatRxData20SelectInput {
    #[inline(always)]
    fn default() -> EcatEcatRxData20SelectInput {
        EcatEcatRxData20SelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for EcatEcatRxData20SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EcatEcatRxData20SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EcatEcatRxData20SelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EcatEcatRxData20SelectInput {
            daisy: super::vals::EcatEcatRxData20SelectInputDaisy,
        }
        let proxy = EcatEcatRxData20SelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ECAT_ECAT_RX_DATA2_1_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EcatEcatRxData21SelectInput(pub u32);
impl EcatEcatRxData21SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::EcatEcatRxData21SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::EcatEcatRxData21SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::EcatEcatRxData21SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for EcatEcatRxData21SelectInput {
    #[inline(always)]
    fn default() -> EcatEcatRxData21SelectInput {
        EcatEcatRxData21SelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for EcatEcatRxData21SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EcatEcatRxData21SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EcatEcatRxData21SelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EcatEcatRxData21SelectInput {
            daisy: super::vals::EcatEcatRxData21SelectInputDaisy,
        }
        let proxy = EcatEcatRxData21SelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ECAT_ECAT_RX_DATA3_0_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EcatEcatRxData30SelectInput(pub u32);
impl EcatEcatRxData30SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::EcatEcatRxData30SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::EcatEcatRxData30SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::EcatEcatRxData30SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for EcatEcatRxData30SelectInput {
    #[inline(always)]
    fn default() -> EcatEcatRxData30SelectInput {
        EcatEcatRxData30SelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for EcatEcatRxData30SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EcatEcatRxData30SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EcatEcatRxData30SelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EcatEcatRxData30SelectInput {
            daisy: super::vals::EcatEcatRxData30SelectInputDaisy,
        }
        let proxy = EcatEcatRxData30SelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ECAT_ECAT_RX_DATA3_1_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EcatEcatRxData31SelectInput(pub u32);
impl EcatEcatRxData31SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::EcatEcatRxData31SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::EcatEcatRxData31SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::EcatEcatRxData31SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for EcatEcatRxData31SelectInput {
    #[inline(always)]
    fn default() -> EcatEcatRxData31SelectInput {
        EcatEcatRxData31SelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for EcatEcatRxData31SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EcatEcatRxData31SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EcatEcatRxData31SelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EcatEcatRxData31SelectInput {
            daisy: super::vals::EcatEcatRxData31SelectInputDaisy,
        }
        let proxy = EcatEcatRxData31SelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ECAT_ECAT_RX_DV_0_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EcatEcatRxDv0SelectInput(pub u32);
impl EcatEcatRxDv0SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::EcatEcatRxDv0SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::EcatEcatRxDv0SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::EcatEcatRxDv0SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for EcatEcatRxDv0SelectInput {
    #[inline(always)]
    fn default() -> EcatEcatRxDv0SelectInput {
        EcatEcatRxDv0SelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for EcatEcatRxDv0SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EcatEcatRxDv0SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EcatEcatRxDv0SelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EcatEcatRxDv0SelectInput {
            daisy: super::vals::EcatEcatRxDv0SelectInputDaisy,
        }
        let proxy = EcatEcatRxDv0SelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ECAT_ECAT_RX_DV_1_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EcatEcatRxDv1SelectInput(pub u32);
impl EcatEcatRxDv1SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::EcatEcatRxDv1SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::EcatEcatRxDv1SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::EcatEcatRxDv1SelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for EcatEcatRxDv1SelectInput {
    #[inline(always)]
    fn default() -> EcatEcatRxDv1SelectInput {
        EcatEcatRxDv1SelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for EcatEcatRxDv1SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EcatEcatRxDv1SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EcatEcatRxDv1SelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EcatEcatRxDv1SelectInput {
            daisy: super::vals::EcatEcatRxDv1SelectInputDaisy,
        }
        let proxy = EcatEcatRxDv1SelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ECAT_ECAT_RX_ER_0_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EcatEcatRxEr0SelectInput(pub u32);
impl EcatEcatRxEr0SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::EcatEcatRxEr0SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::EcatEcatRxEr0SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::EcatEcatRxEr0SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for EcatEcatRxEr0SelectInput {
    #[inline(always)]
    fn default() -> EcatEcatRxEr0SelectInput {
        EcatEcatRxEr0SelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for EcatEcatRxEr0SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EcatEcatRxEr0SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EcatEcatRxEr0SelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EcatEcatRxEr0SelectInput {
            daisy: super::vals::EcatEcatRxEr0SelectInputDaisy,
        }
        let proxy = EcatEcatRxEr0SelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ECAT_ECAT_RX_ER_1_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EcatEcatRxEr1SelectInput(pub u32);
impl EcatEcatRxEr1SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::EcatEcatRxEr1SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::EcatEcatRxEr1SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::EcatEcatRxEr1SelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for EcatEcatRxEr1SelectInput {
    #[inline(always)]
    fn default() -> EcatEcatRxEr1SelectInput {
        EcatEcatRxEr1SelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for EcatEcatRxEr1SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EcatEcatRxEr1SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EcatEcatRxEr1SelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EcatEcatRxEr1SelectInput {
            daisy: super::vals::EcatEcatRxEr1SelectInputDaisy,
        }
        let proxy = EcatEcatRxEr1SelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ECAT_ECAT_TX_CLK_0_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EcatEcatTxClk0SelectInput(pub u32);
impl EcatEcatTxClk0SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::EcatEcatTxClk0SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::EcatEcatTxClk0SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::EcatEcatTxClk0SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for EcatEcatTxClk0SelectInput {
    #[inline(always)]
    fn default() -> EcatEcatTxClk0SelectInput {
        EcatEcatTxClk0SelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for EcatEcatTxClk0SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EcatEcatTxClk0SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EcatEcatTxClk0SelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EcatEcatTxClk0SelectInput {
            daisy: super::vals::EcatEcatTxClk0SelectInputDaisy,
        }
        let proxy = EcatEcatTxClk0SelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ECAT_ECAT_TX_CLK_1_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EcatEcatTxClk1SelectInput(pub u32);
impl EcatEcatTxClk1SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::EcatEcatTxClk1SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::EcatEcatTxClk1SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::EcatEcatTxClk1SelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for EcatEcatTxClk1SelectInput {
    #[inline(always)]
    fn default() -> EcatEcatTxClk1SelectInput {
        EcatEcatTxClk1SelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for EcatEcatTxClk1SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EcatEcatTxClk1SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EcatEcatTxClk1SelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EcatEcatTxClk1SelectInput {
            daisy: super::vals::EcatEcatTxClk1SelectInputDaisy,
        }
        let proxy = EcatEcatTxClk1SelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ECAT_MDIO_DATA_IN_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EcatMdioDataInSelectInput(pub u32);
impl EcatMdioDataInSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::EcatMdioDataInSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::EcatMdioDataInSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::EcatMdioDataInSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for EcatMdioDataInSelectInput {
    #[inline(always)]
    fn default() -> EcatMdioDataInSelectInput {
        EcatMdioDataInSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for EcatMdioDataInSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EcatMdioDataInSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EcatMdioDataInSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EcatMdioDataInSelectInput {
            daisy: super::vals::EcatMdioDataInSelectInputDaisy,
        }
        let proxy = EcatMdioDataInSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ECAT_PROM_DATA_IN_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EcatPromDataInSelectInput(pub u32);
impl EcatPromDataInSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::EcatPromDataInSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::EcatPromDataInSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::EcatPromDataInSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for EcatPromDataInSelectInput {
    #[inline(always)]
    fn default() -> EcatPromDataInSelectInput {
        EcatPromDataInSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for EcatPromDataInSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EcatPromDataInSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EcatPromDataInSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EcatPromDataInSelectInput {
            daisy: super::vals::EcatPromDataInSelectInputDaisy,
        }
        let proxy = EcatPromDataInSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_0 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm1IppIndPwmaSelectInput0(pub u32);
impl Flexpwm1IppIndPwmaSelectInput0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm1IppIndPwmaSelectInput0Daisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Flexpwm1IppIndPwmaSelectInput0Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm1IppIndPwmaSelectInput0Daisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Flexpwm1IppIndPwmaSelectInput0 {
    #[inline(always)]
    fn default() -> Flexpwm1IppIndPwmaSelectInput0 {
        Flexpwm1IppIndPwmaSelectInput0(0u64 as u32)
    }
}
impl core::fmt::Debug for Flexpwm1IppIndPwmaSelectInput0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm1IppIndPwmaSelectInput0")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm1IppIndPwmaSelectInput0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flexpwm1IppIndPwmaSelectInput0 {
            daisy: super::vals::Flexpwm1IppIndPwmaSelectInput0Daisy,
        }
        let proxy = Flexpwm1IppIndPwmaSelectInput0 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_1 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm1IppIndPwmaSelectInput1(pub u32);
impl Flexpwm1IppIndPwmaSelectInput1 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm1IppIndPwmaSelectInput1Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexpwm1IppIndPwmaSelectInput1Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm1IppIndPwmaSelectInput1Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm1IppIndPwmaSelectInput1 {
    #[inline(always)]
    fn default() -> Flexpwm1IppIndPwmaSelectInput1 {
        Flexpwm1IppIndPwmaSelectInput1(0u64 as u32)
    }
}
impl core::fmt::Debug for Flexpwm1IppIndPwmaSelectInput1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm1IppIndPwmaSelectInput1")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm1IppIndPwmaSelectInput1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flexpwm1IppIndPwmaSelectInput1 {
            daisy: super::vals::Flexpwm1IppIndPwmaSelectInput1Daisy,
        }
        let proxy = Flexpwm1IppIndPwmaSelectInput1 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_2 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm1IppIndPwmaSelectInput2(pub u32);
impl Flexpwm1IppIndPwmaSelectInput2 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm1IppIndPwmaSelectInput2Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexpwm1IppIndPwmaSelectInput2Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm1IppIndPwmaSelectInput2Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm1IppIndPwmaSelectInput2 {
    #[inline(always)]
    fn default() -> Flexpwm1IppIndPwmaSelectInput2 {
        Flexpwm1IppIndPwmaSelectInput2(0u64 as u32)
    }
}
impl core::fmt::Debug for Flexpwm1IppIndPwmaSelectInput2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm1IppIndPwmaSelectInput2")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm1IppIndPwmaSelectInput2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flexpwm1IppIndPwmaSelectInput2 {
            daisy: super::vals::Flexpwm1IppIndPwmaSelectInput2Daisy,
        }
        let proxy = Flexpwm1IppIndPwmaSelectInput2 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_0 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm1IppIndPwmbSelectInput0(pub u32);
impl Flexpwm1IppIndPwmbSelectInput0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm1IppIndPwmbSelectInput0Daisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Flexpwm1IppIndPwmbSelectInput0Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm1IppIndPwmbSelectInput0Daisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Flexpwm1IppIndPwmbSelectInput0 {
    #[inline(always)]
    fn default() -> Flexpwm1IppIndPwmbSelectInput0 {
        Flexpwm1IppIndPwmbSelectInput0(0u64 as u32)
    }
}
impl core::fmt::Debug for Flexpwm1IppIndPwmbSelectInput0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm1IppIndPwmbSelectInput0")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm1IppIndPwmbSelectInput0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flexpwm1IppIndPwmbSelectInput0 {
            daisy: super::vals::Flexpwm1IppIndPwmbSelectInput0Daisy,
        }
        let proxy = Flexpwm1IppIndPwmbSelectInput0 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_1 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm1IppIndPwmbSelectInput1(pub u32);
impl Flexpwm1IppIndPwmbSelectInput1 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm1IppIndPwmbSelectInput1Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexpwm1IppIndPwmbSelectInput1Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm1IppIndPwmbSelectInput1Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm1IppIndPwmbSelectInput1 {
    #[inline(always)]
    fn default() -> Flexpwm1IppIndPwmbSelectInput1 {
        Flexpwm1IppIndPwmbSelectInput1(0u64 as u32)
    }
}
impl core::fmt::Debug for Flexpwm1IppIndPwmbSelectInput1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm1IppIndPwmbSelectInput1")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm1IppIndPwmbSelectInput1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flexpwm1IppIndPwmbSelectInput1 {
            daisy: super::vals::Flexpwm1IppIndPwmbSelectInput1Daisy,
        }
        let proxy = Flexpwm1IppIndPwmbSelectInput1 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_2 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm1IppIndPwmbSelectInput2(pub u32);
impl Flexpwm1IppIndPwmbSelectInput2 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm1IppIndPwmbSelectInput2Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexpwm1IppIndPwmbSelectInput2Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm1IppIndPwmbSelectInput2Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm1IppIndPwmbSelectInput2 {
    #[inline(always)]
    fn default() -> Flexpwm1IppIndPwmbSelectInput2 {
        Flexpwm1IppIndPwmbSelectInput2(0u64 as u32)
    }
}
impl core::fmt::Debug for Flexpwm1IppIndPwmbSelectInput2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm1IppIndPwmbSelectInput2")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm1IppIndPwmbSelectInput2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flexpwm1IppIndPwmbSelectInput2 {
            daisy: super::vals::Flexpwm1IppIndPwmbSelectInput2Daisy,
        }
        let proxy = Flexpwm1IppIndPwmbSelectInput2 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FLEXPWM2_IPP_IND_PWMA_SELECT_INPUT_0 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm2IppIndPwmaSelectInput0(pub u32);
impl Flexpwm2IppIndPwmaSelectInput0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm2IppIndPwmaSelectInput0Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexpwm2IppIndPwmaSelectInput0Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm2IppIndPwmaSelectInput0Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm2IppIndPwmaSelectInput0 {
    #[inline(always)]
    fn default() -> Flexpwm2IppIndPwmaSelectInput0 {
        Flexpwm2IppIndPwmaSelectInput0(0u64 as u32)
    }
}
impl core::fmt::Debug for Flexpwm2IppIndPwmaSelectInput0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm2IppIndPwmaSelectInput0")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm2IppIndPwmaSelectInput0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flexpwm2IppIndPwmaSelectInput0 {
            daisy: super::vals::Flexpwm2IppIndPwmaSelectInput0Daisy,
        }
        let proxy = Flexpwm2IppIndPwmaSelectInput0 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FLEXPWM2_IPP_IND_PWMA_SELECT_INPUT_1 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm2IppIndPwmaSelectInput1(pub u32);
impl Flexpwm2IppIndPwmaSelectInput1 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm2IppIndPwmaSelectInput1Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexpwm2IppIndPwmaSelectInput1Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm2IppIndPwmaSelectInput1Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm2IppIndPwmaSelectInput1 {
    #[inline(always)]
    fn default() -> Flexpwm2IppIndPwmaSelectInput1 {
        Flexpwm2IppIndPwmaSelectInput1(0u64 as u32)
    }
}
impl core::fmt::Debug for Flexpwm2IppIndPwmaSelectInput1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm2IppIndPwmaSelectInput1")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm2IppIndPwmaSelectInput1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flexpwm2IppIndPwmaSelectInput1 {
            daisy: super::vals::Flexpwm2IppIndPwmaSelectInput1Daisy,
        }
        let proxy = Flexpwm2IppIndPwmaSelectInput1 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FLEXPWM2_IPP_IND_PWMA_SELECT_INPUT_2 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm2IppIndPwmaSelectInput2(pub u32);
impl Flexpwm2IppIndPwmaSelectInput2 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm2IppIndPwmaSelectInput2Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexpwm2IppIndPwmaSelectInput2Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm2IppIndPwmaSelectInput2Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm2IppIndPwmaSelectInput2 {
    #[inline(always)]
    fn default() -> Flexpwm2IppIndPwmaSelectInput2 {
        Flexpwm2IppIndPwmaSelectInput2(0u64 as u32)
    }
}
impl core::fmt::Debug for Flexpwm2IppIndPwmaSelectInput2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm2IppIndPwmaSelectInput2")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm2IppIndPwmaSelectInput2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flexpwm2IppIndPwmaSelectInput2 {
            daisy: super::vals::Flexpwm2IppIndPwmaSelectInput2Daisy,
        }
        let proxy = Flexpwm2IppIndPwmaSelectInput2 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FLEXPWM2_IPP_IND_PWMB_SELECT_INPUT_0 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm2IppIndPwmbSelectInput0(pub u32);
impl Flexpwm2IppIndPwmbSelectInput0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm2IppIndPwmbSelectInput0Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexpwm2IppIndPwmbSelectInput0Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm2IppIndPwmbSelectInput0Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm2IppIndPwmbSelectInput0 {
    #[inline(always)]
    fn default() -> Flexpwm2IppIndPwmbSelectInput0 {
        Flexpwm2IppIndPwmbSelectInput0(0u64 as u32)
    }
}
impl core::fmt::Debug for Flexpwm2IppIndPwmbSelectInput0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm2IppIndPwmbSelectInput0")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm2IppIndPwmbSelectInput0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flexpwm2IppIndPwmbSelectInput0 {
            daisy: super::vals::Flexpwm2IppIndPwmbSelectInput0Daisy,
        }
        let proxy = Flexpwm2IppIndPwmbSelectInput0 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FLEXPWM2_IPP_IND_PWMB_SELECT_INPUT_1 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm2IppIndPwmbSelectInput1(pub u32);
impl Flexpwm2IppIndPwmbSelectInput1 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm2IppIndPwmbSelectInput1Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexpwm2IppIndPwmbSelectInput1Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm2IppIndPwmbSelectInput1Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm2IppIndPwmbSelectInput1 {
    #[inline(always)]
    fn default() -> Flexpwm2IppIndPwmbSelectInput1 {
        Flexpwm2IppIndPwmbSelectInput1(0u64 as u32)
    }
}
impl core::fmt::Debug for Flexpwm2IppIndPwmbSelectInput1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm2IppIndPwmbSelectInput1")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm2IppIndPwmbSelectInput1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flexpwm2IppIndPwmbSelectInput1 {
            daisy: super::vals::Flexpwm2IppIndPwmbSelectInput1Daisy,
        }
        let proxy = Flexpwm2IppIndPwmbSelectInput1 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FLEXPWM2_IPP_IND_PWMB_SELECT_INPUT_2 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm2IppIndPwmbSelectInput2(pub u32);
impl Flexpwm2IppIndPwmbSelectInput2 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm2IppIndPwmbSelectInput2Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexpwm2IppIndPwmbSelectInput2Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm2IppIndPwmbSelectInput2Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm2IppIndPwmbSelectInput2 {
    #[inline(always)]
    fn default() -> Flexpwm2IppIndPwmbSelectInput2 {
        Flexpwm2IppIndPwmbSelectInput2(0u64 as u32)
    }
}
impl core::fmt::Debug for Flexpwm2IppIndPwmbSelectInput2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm2IppIndPwmbSelectInput2")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm2IppIndPwmbSelectInput2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flexpwm2IppIndPwmbSelectInput2 {
            daisy: super::vals::Flexpwm2IppIndPwmbSelectInput2Daisy,
        }
        let proxy = Flexpwm2IppIndPwmbSelectInput2 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FLEXPWM3_IPP_IND_PWMA_SELECT_INPUT_0 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm3IppIndPwmaSelectInput0(pub u32);
impl Flexpwm3IppIndPwmaSelectInput0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm3IppIndPwmaSelectInput0Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexpwm3IppIndPwmaSelectInput0Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm3IppIndPwmaSelectInput0Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm3IppIndPwmaSelectInput0 {
    #[inline(always)]
    fn default() -> Flexpwm3IppIndPwmaSelectInput0 {
        Flexpwm3IppIndPwmaSelectInput0(0u64 as u32)
    }
}
impl core::fmt::Debug for Flexpwm3IppIndPwmaSelectInput0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm3IppIndPwmaSelectInput0")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm3IppIndPwmaSelectInput0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flexpwm3IppIndPwmaSelectInput0 {
            daisy: super::vals::Flexpwm3IppIndPwmaSelectInput0Daisy,
        }
        let proxy = Flexpwm3IppIndPwmaSelectInput0 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FLEXPWM3_IPP_IND_PWMA_SELECT_INPUT_1 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm3IppIndPwmaSelectInput1(pub u32);
impl Flexpwm3IppIndPwmaSelectInput1 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm3IppIndPwmaSelectInput1Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexpwm3IppIndPwmaSelectInput1Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm3IppIndPwmaSelectInput1Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm3IppIndPwmaSelectInput1 {
    #[inline(always)]
    fn default() -> Flexpwm3IppIndPwmaSelectInput1 {
        Flexpwm3IppIndPwmaSelectInput1(0u64 as u32)
    }
}
impl core::fmt::Debug for Flexpwm3IppIndPwmaSelectInput1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm3IppIndPwmaSelectInput1")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm3IppIndPwmaSelectInput1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flexpwm3IppIndPwmaSelectInput1 {
            daisy: super::vals::Flexpwm3IppIndPwmaSelectInput1Daisy,
        }
        let proxy = Flexpwm3IppIndPwmaSelectInput1 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FLEXPWM3_IPP_IND_PWMA_SELECT_INPUT_2 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm3IppIndPwmaSelectInput2(pub u32);
impl Flexpwm3IppIndPwmaSelectInput2 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm3IppIndPwmaSelectInput2Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexpwm3IppIndPwmaSelectInput2Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm3IppIndPwmaSelectInput2Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm3IppIndPwmaSelectInput2 {
    #[inline(always)]
    fn default() -> Flexpwm3IppIndPwmaSelectInput2 {
        Flexpwm3IppIndPwmaSelectInput2(0u64 as u32)
    }
}
impl core::fmt::Debug for Flexpwm3IppIndPwmaSelectInput2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm3IppIndPwmaSelectInput2")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm3IppIndPwmaSelectInput2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flexpwm3IppIndPwmaSelectInput2 {
            daisy: super::vals::Flexpwm3IppIndPwmaSelectInput2Daisy,
        }
        let proxy = Flexpwm3IppIndPwmaSelectInput2 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FLEXPWM3_IPP_IND_PWMA_SELECT_INPUT_3 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm3IppIndPwmaSelectInput3(pub u32);
impl Flexpwm3IppIndPwmaSelectInput3 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm3IppIndPwmaSelectInput3Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexpwm3IppIndPwmaSelectInput3Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm3IppIndPwmaSelectInput3Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm3IppIndPwmaSelectInput3 {
    #[inline(always)]
    fn default() -> Flexpwm3IppIndPwmaSelectInput3 {
        Flexpwm3IppIndPwmaSelectInput3(0u64 as u32)
    }
}
impl core::fmt::Debug for Flexpwm3IppIndPwmaSelectInput3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm3IppIndPwmaSelectInput3")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm3IppIndPwmaSelectInput3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flexpwm3IppIndPwmaSelectInput3 {
            daisy: super::vals::Flexpwm3IppIndPwmaSelectInput3Daisy,
        }
        let proxy = Flexpwm3IppIndPwmaSelectInput3 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FLEXPWM3_IPP_IND_PWMB_SELECT_INPUT_0 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm3IppIndPwmbSelectInput0(pub u32);
impl Flexpwm3IppIndPwmbSelectInput0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm3IppIndPwmbSelectInput0Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexpwm3IppIndPwmbSelectInput0Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm3IppIndPwmbSelectInput0Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm3IppIndPwmbSelectInput0 {
    #[inline(always)]
    fn default() -> Flexpwm3IppIndPwmbSelectInput0 {
        Flexpwm3IppIndPwmbSelectInput0(0u64 as u32)
    }
}
impl core::fmt::Debug for Flexpwm3IppIndPwmbSelectInput0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm3IppIndPwmbSelectInput0")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm3IppIndPwmbSelectInput0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flexpwm3IppIndPwmbSelectInput0 {
            daisy: super::vals::Flexpwm3IppIndPwmbSelectInput0Daisy,
        }
        let proxy = Flexpwm3IppIndPwmbSelectInput0 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FLEXPWM3_IPP_IND_PWMB_SELECT_INPUT_1 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm3IppIndPwmbSelectInput1(pub u32);
impl Flexpwm3IppIndPwmbSelectInput1 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm3IppIndPwmbSelectInput1Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexpwm3IppIndPwmbSelectInput1Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm3IppIndPwmbSelectInput1Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm3IppIndPwmbSelectInput1 {
    #[inline(always)]
    fn default() -> Flexpwm3IppIndPwmbSelectInput1 {
        Flexpwm3IppIndPwmbSelectInput1(0u64 as u32)
    }
}
impl core::fmt::Debug for Flexpwm3IppIndPwmbSelectInput1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm3IppIndPwmbSelectInput1")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm3IppIndPwmbSelectInput1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flexpwm3IppIndPwmbSelectInput1 {
            daisy: super::vals::Flexpwm3IppIndPwmbSelectInput1Daisy,
        }
        let proxy = Flexpwm3IppIndPwmbSelectInput1 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FLEXPWM3_IPP_IND_PWMB_SELECT_INPUT_2 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm3IppIndPwmbSelectInput2(pub u32);
impl Flexpwm3IppIndPwmbSelectInput2 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm3IppIndPwmbSelectInput2Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexpwm3IppIndPwmbSelectInput2Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm3IppIndPwmbSelectInput2Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm3IppIndPwmbSelectInput2 {
    #[inline(always)]
    fn default() -> Flexpwm3IppIndPwmbSelectInput2 {
        Flexpwm3IppIndPwmbSelectInput2(0u64 as u32)
    }
}
impl core::fmt::Debug for Flexpwm3IppIndPwmbSelectInput2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm3IppIndPwmbSelectInput2")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm3IppIndPwmbSelectInput2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flexpwm3IppIndPwmbSelectInput2 {
            daisy: super::vals::Flexpwm3IppIndPwmbSelectInput2Daisy,
        }
        let proxy = Flexpwm3IppIndPwmbSelectInput2 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FLEXPWM3_IPP_IND_PWMB_SELECT_INPUT_3 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm3IppIndPwmbSelectInput3(pub u32);
impl Flexpwm3IppIndPwmbSelectInput3 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm3IppIndPwmbSelectInput3Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexpwm3IppIndPwmbSelectInput3Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm3IppIndPwmbSelectInput3Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm3IppIndPwmbSelectInput3 {
    #[inline(always)]
    fn default() -> Flexpwm3IppIndPwmbSelectInput3 {
        Flexpwm3IppIndPwmbSelectInput3(0u64 as u32)
    }
}
impl core::fmt::Debug for Flexpwm3IppIndPwmbSelectInput3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm3IppIndPwmbSelectInput3")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm3IppIndPwmbSelectInput3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flexpwm3IppIndPwmbSelectInput3 {
            daisy: super::vals::Flexpwm3IppIndPwmbSelectInput3Daisy,
        }
        let proxy = Flexpwm3IppIndPwmbSelectInput3 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FLEXSPI1_BUS2BIT_IPP_IND_DQS_FA_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspi1Bus2bitIppIndDqsFaSelectInput(pub u32);
impl Flexspi1Bus2bitIppIndDqsFaSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexspi1Bus2bitIppIndDqsFaSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexspi1Bus2bitIppIndDqsFaSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(
        &mut self,
        val: super::vals::Flexspi1Bus2bitIppIndDqsFaSelectInputDaisy,
    ) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexspi1Bus2bitIppIndDqsFaSelectInput {
    #[inline(always)]
    fn default() -> Flexspi1Bus2bitIppIndDqsFaSelectInput {
        Flexspi1Bus2bitIppIndDqsFaSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Flexspi1Bus2bitIppIndDqsFaSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexspi1Bus2bitIppIndDqsFaSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexspi1Bus2bitIppIndDqsFaSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flexspi1Bus2bitIppIndDqsFaSelectInput {
            daisy: super::vals::Flexspi1Bus2bitIppIndDqsFaSelectInputDaisy,
        }
        let proxy = Flexspi1Bus2bitIppIndDqsFaSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FLEXSPI1_BUS2BIT_IPP_IND_DQS_FB_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspi1Bus2bitIppIndDqsFbSelectInput(pub u32);
impl Flexspi1Bus2bitIppIndDqsFbSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexspi1Bus2bitIppIndDqsFbSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Flexspi1Bus2bitIppIndDqsFbSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(
        &mut self,
        val: super::vals::Flexspi1Bus2bitIppIndDqsFbSelectInputDaisy,
    ) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Flexspi1Bus2bitIppIndDqsFbSelectInput {
    #[inline(always)]
    fn default() -> Flexspi1Bus2bitIppIndDqsFbSelectInput {
        Flexspi1Bus2bitIppIndDqsFbSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Flexspi1Bus2bitIppIndDqsFbSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexspi1Bus2bitIppIndDqsFbSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexspi1Bus2bitIppIndDqsFbSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flexspi1Bus2bitIppIndDqsFbSelectInput {
            daisy: super::vals::Flexspi1Bus2bitIppIndDqsFbSelectInputDaisy,
        }
        let proxy = Flexspi1Bus2bitIppIndDqsFbSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FLEXSPI1_BUS2BIT_IPP_IND_IO_FB_BIT0_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspi1Bus2bitIppIndIoFbBit0SelectInput(pub u32);
impl Flexspi1Bus2bitIppIndIoFbBit0SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexspi1Bus2bitIppIndIoFbBit0SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexspi1Bus2bitIppIndIoFbBit0SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(
        &mut self,
        val: super::vals::Flexspi1Bus2bitIppIndIoFbBit0SelectInputDaisy,
    ) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexspi1Bus2bitIppIndIoFbBit0SelectInput {
    #[inline(always)]
    fn default() -> Flexspi1Bus2bitIppIndIoFbBit0SelectInput {
        Flexspi1Bus2bitIppIndIoFbBit0SelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Flexspi1Bus2bitIppIndIoFbBit0SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexspi1Bus2bitIppIndIoFbBit0SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexspi1Bus2bitIppIndIoFbBit0SelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flexspi1Bus2bitIppIndIoFbBit0SelectInput {
            daisy: super::vals::Flexspi1Bus2bitIppIndIoFbBit0SelectInputDaisy,
        }
        let proxy = Flexspi1Bus2bitIppIndIoFbBit0SelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FLEXSPI1_BUS2BIT_IPP_IND_IO_FB_BIT1_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspi1Bus2bitIppIndIoFbBit1SelectInput(pub u32);
impl Flexspi1Bus2bitIppIndIoFbBit1SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexspi1Bus2bitIppIndIoFbBit1SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexspi1Bus2bitIppIndIoFbBit1SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(
        &mut self,
        val: super::vals::Flexspi1Bus2bitIppIndIoFbBit1SelectInputDaisy,
    ) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexspi1Bus2bitIppIndIoFbBit1SelectInput {
    #[inline(always)]
    fn default() -> Flexspi1Bus2bitIppIndIoFbBit1SelectInput {
        Flexspi1Bus2bitIppIndIoFbBit1SelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Flexspi1Bus2bitIppIndIoFbBit1SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexspi1Bus2bitIppIndIoFbBit1SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexspi1Bus2bitIppIndIoFbBit1SelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flexspi1Bus2bitIppIndIoFbBit1SelectInput {
            daisy: super::vals::Flexspi1Bus2bitIppIndIoFbBit1SelectInputDaisy,
        }
        let proxy = Flexspi1Bus2bitIppIndIoFbBit1SelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FLEXSPI1_BUS2BIT_IPP_IND_IO_FB_BIT2_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspi1Bus2bitIppIndIoFbBit2SelectInput(pub u32);
impl Flexspi1Bus2bitIppIndIoFbBit2SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexspi1Bus2bitIppIndIoFbBit2SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexspi1Bus2bitIppIndIoFbBit2SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(
        &mut self,
        val: super::vals::Flexspi1Bus2bitIppIndIoFbBit2SelectInputDaisy,
    ) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexspi1Bus2bitIppIndIoFbBit2SelectInput {
    #[inline(always)]
    fn default() -> Flexspi1Bus2bitIppIndIoFbBit2SelectInput {
        Flexspi1Bus2bitIppIndIoFbBit2SelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Flexspi1Bus2bitIppIndIoFbBit2SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexspi1Bus2bitIppIndIoFbBit2SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexspi1Bus2bitIppIndIoFbBit2SelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flexspi1Bus2bitIppIndIoFbBit2SelectInput {
            daisy: super::vals::Flexspi1Bus2bitIppIndIoFbBit2SelectInputDaisy,
        }
        let proxy = Flexspi1Bus2bitIppIndIoFbBit2SelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FLEXSPI1_BUS2BIT_IPP_IND_IO_FB_BIT3_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspi1Bus2bitIppIndIoFbBit3SelectInput(pub u32);
impl Flexspi1Bus2bitIppIndIoFbBit3SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexspi1Bus2bitIppIndIoFbBit3SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexspi1Bus2bitIppIndIoFbBit3SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(
        &mut self,
        val: super::vals::Flexspi1Bus2bitIppIndIoFbBit3SelectInputDaisy,
    ) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexspi1Bus2bitIppIndIoFbBit3SelectInput {
    #[inline(always)]
    fn default() -> Flexspi1Bus2bitIppIndIoFbBit3SelectInput {
        Flexspi1Bus2bitIppIndIoFbBit3SelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Flexspi1Bus2bitIppIndIoFbBit3SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexspi1Bus2bitIppIndIoFbBit3SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexspi1Bus2bitIppIndIoFbBit3SelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flexspi1Bus2bitIppIndIoFbBit3SelectInput {
            daisy: super::vals::Flexspi1Bus2bitIppIndIoFbBit3SelectInputDaisy,
        }
        let proxy = Flexspi1Bus2bitIppIndIoFbBit3SelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FLEXSPI1_BUS2BIT_IPP_IND_IO_FB_BIT4_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspi1Bus2bitIppIndIoFbBit4SelectInput(pub u32);
impl Flexspi1Bus2bitIppIndIoFbBit4SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexspi1Bus2bitIppIndIoFbBit4SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexspi1Bus2bitIppIndIoFbBit4SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(
        &mut self,
        val: super::vals::Flexspi1Bus2bitIppIndIoFbBit4SelectInputDaisy,
    ) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexspi1Bus2bitIppIndIoFbBit4SelectInput {
    #[inline(always)]
    fn default() -> Flexspi1Bus2bitIppIndIoFbBit4SelectInput {
        Flexspi1Bus2bitIppIndIoFbBit4SelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Flexspi1Bus2bitIppIndIoFbBit4SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexspi1Bus2bitIppIndIoFbBit4SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexspi1Bus2bitIppIndIoFbBit4SelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flexspi1Bus2bitIppIndIoFbBit4SelectInput {
            daisy: super::vals::Flexspi1Bus2bitIppIndIoFbBit4SelectInputDaisy,
        }
        let proxy = Flexspi1Bus2bitIppIndIoFbBit4SelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FLEXSPI1_BUS2BIT_IPP_IND_IO_FB_BIT5_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspi1Bus2bitIppIndIoFbBit5SelectInput(pub u32);
impl Flexspi1Bus2bitIppIndIoFbBit5SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexspi1Bus2bitIppIndIoFbBit5SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexspi1Bus2bitIppIndIoFbBit5SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(
        &mut self,
        val: super::vals::Flexspi1Bus2bitIppIndIoFbBit5SelectInputDaisy,
    ) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexspi1Bus2bitIppIndIoFbBit5SelectInput {
    #[inline(always)]
    fn default() -> Flexspi1Bus2bitIppIndIoFbBit5SelectInput {
        Flexspi1Bus2bitIppIndIoFbBit5SelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Flexspi1Bus2bitIppIndIoFbBit5SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexspi1Bus2bitIppIndIoFbBit5SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexspi1Bus2bitIppIndIoFbBit5SelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flexspi1Bus2bitIppIndIoFbBit5SelectInput {
            daisy: super::vals::Flexspi1Bus2bitIppIndIoFbBit5SelectInputDaisy,
        }
        let proxy = Flexspi1Bus2bitIppIndIoFbBit5SelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FLEXSPI1_BUS2BIT_IPP_IND_IO_FB_BIT6_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspi1Bus2bitIppIndIoFbBit6SelectInput(pub u32);
impl Flexspi1Bus2bitIppIndIoFbBit6SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexspi1Bus2bitIppIndIoFbBit6SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexspi1Bus2bitIppIndIoFbBit6SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(
        &mut self,
        val: super::vals::Flexspi1Bus2bitIppIndIoFbBit6SelectInputDaisy,
    ) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexspi1Bus2bitIppIndIoFbBit6SelectInput {
    #[inline(always)]
    fn default() -> Flexspi1Bus2bitIppIndIoFbBit6SelectInput {
        Flexspi1Bus2bitIppIndIoFbBit6SelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Flexspi1Bus2bitIppIndIoFbBit6SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexspi1Bus2bitIppIndIoFbBit6SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexspi1Bus2bitIppIndIoFbBit6SelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flexspi1Bus2bitIppIndIoFbBit6SelectInput {
            daisy: super::vals::Flexspi1Bus2bitIppIndIoFbBit6SelectInputDaisy,
        }
        let proxy = Flexspi1Bus2bitIppIndIoFbBit6SelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FLEXSPI1_BUS2BIT_IPP_IND_IO_FB_BIT7_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspi1Bus2bitIppIndIoFbBit7SelectInput(pub u32);
impl Flexspi1Bus2bitIppIndIoFbBit7SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexspi1Bus2bitIppIndIoFbBit7SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexspi1Bus2bitIppIndIoFbBit7SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(
        &mut self,
        val: super::vals::Flexspi1Bus2bitIppIndIoFbBit7SelectInputDaisy,
    ) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexspi1Bus2bitIppIndIoFbBit7SelectInput {
    #[inline(always)]
    fn default() -> Flexspi1Bus2bitIppIndIoFbBit7SelectInput {
        Flexspi1Bus2bitIppIndIoFbBit7SelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Flexspi1Bus2bitIppIndIoFbBit7SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexspi1Bus2bitIppIndIoFbBit7SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexspi1Bus2bitIppIndIoFbBit7SelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flexspi1Bus2bitIppIndIoFbBit7SelectInput {
            daisy: super::vals::Flexspi1Bus2bitIppIndIoFbBit7SelectInputDaisy,
        }
        let proxy = Flexspi1Bus2bitIppIndIoFbBit7SelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FLEXSPI1_BUS2BIT_IPP_IND_SCK_FB_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspi1Bus2bitIppIndSckFbSelectInput(pub u32);
impl Flexspi1Bus2bitIppIndSckFbSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexspi1Bus2bitIppIndSckFbSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Flexspi1Bus2bitIppIndSckFbSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(
        &mut self,
        val: super::vals::Flexspi1Bus2bitIppIndSckFbSelectInputDaisy,
    ) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Flexspi1Bus2bitIppIndSckFbSelectInput {
    #[inline(always)]
    fn default() -> Flexspi1Bus2bitIppIndSckFbSelectInput {
        Flexspi1Bus2bitIppIndSckFbSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Flexspi1Bus2bitIppIndSckFbSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexspi1Bus2bitIppIndSckFbSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexspi1Bus2bitIppIndSckFbSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flexspi1Bus2bitIppIndSckFbSelectInput {
            daisy: super::vals::Flexspi1Bus2bitIppIndSckFbSelectInputDaisy,
        }
        let proxy = Flexspi1Bus2bitIppIndSckFbSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FLEXSPI2_BUS2BIT_IPP_IND_DQS_FA_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspi2Bus2bitIppIndDqsFaSelectInput(pub u32);
impl Flexspi2Bus2bitIppIndDqsFaSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexspi2Bus2bitIppIndDqsFaSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Flexspi2Bus2bitIppIndDqsFaSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(
        &mut self,
        val: super::vals::Flexspi2Bus2bitIppIndDqsFaSelectInputDaisy,
    ) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Flexspi2Bus2bitIppIndDqsFaSelectInput {
    #[inline(always)]
    fn default() -> Flexspi2Bus2bitIppIndDqsFaSelectInput {
        Flexspi2Bus2bitIppIndDqsFaSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Flexspi2Bus2bitIppIndDqsFaSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexspi2Bus2bitIppIndDqsFaSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexspi2Bus2bitIppIndDqsFaSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flexspi2Bus2bitIppIndDqsFaSelectInput {
            daisy: super::vals::Flexspi2Bus2bitIppIndDqsFaSelectInputDaisy,
        }
        let proxy = Flexspi2Bus2bitIppIndDqsFaSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FLEXSPI2_BUS2BIT_IPP_IND_DQS_FB_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspi2Bus2bitIppIndDqsFbSelectInput(pub u32);
impl Flexspi2Bus2bitIppIndDqsFbSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexspi2Bus2bitIppIndDqsFbSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Flexspi2Bus2bitIppIndDqsFbSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(
        &mut self,
        val: super::vals::Flexspi2Bus2bitIppIndDqsFbSelectInputDaisy,
    ) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Flexspi2Bus2bitIppIndDqsFbSelectInput {
    #[inline(always)]
    fn default() -> Flexspi2Bus2bitIppIndDqsFbSelectInput {
        Flexspi2Bus2bitIppIndDqsFbSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Flexspi2Bus2bitIppIndDqsFbSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexspi2Bus2bitIppIndDqsFbSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexspi2Bus2bitIppIndDqsFbSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flexspi2Bus2bitIppIndDqsFbSelectInput {
            daisy: super::vals::Flexspi2Bus2bitIppIndDqsFbSelectInputDaisy,
        }
        let proxy = Flexspi2Bus2bitIppIndDqsFbSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FLEXSPI2_BUS2BIT_IPP_IND_IO_FA_BIT0_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspi2Bus2bitIppIndIoFaBit0SelectInput(pub u32);
impl Flexspi2Bus2bitIppIndIoFaBit0SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexspi2Bus2bitIppIndIoFaBit0SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexspi2Bus2bitIppIndIoFaBit0SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(
        &mut self,
        val: super::vals::Flexspi2Bus2bitIppIndIoFaBit0SelectInputDaisy,
    ) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexspi2Bus2bitIppIndIoFaBit0SelectInput {
    #[inline(always)]
    fn default() -> Flexspi2Bus2bitIppIndIoFaBit0SelectInput {
        Flexspi2Bus2bitIppIndIoFaBit0SelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Flexspi2Bus2bitIppIndIoFaBit0SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexspi2Bus2bitIppIndIoFaBit0SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexspi2Bus2bitIppIndIoFaBit0SelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flexspi2Bus2bitIppIndIoFaBit0SelectInput {
            daisy: super::vals::Flexspi2Bus2bitIppIndIoFaBit0SelectInputDaisy,
        }
        let proxy = Flexspi2Bus2bitIppIndIoFaBit0SelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FLEXSPI2_BUS2BIT_IPP_IND_IO_FA_BIT1_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspi2Bus2bitIppIndIoFaBit1SelectInput(pub u32);
impl Flexspi2Bus2bitIppIndIoFaBit1SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexspi2Bus2bitIppIndIoFaBit1SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexspi2Bus2bitIppIndIoFaBit1SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(
        &mut self,
        val: super::vals::Flexspi2Bus2bitIppIndIoFaBit1SelectInputDaisy,
    ) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexspi2Bus2bitIppIndIoFaBit1SelectInput {
    #[inline(always)]
    fn default() -> Flexspi2Bus2bitIppIndIoFaBit1SelectInput {
        Flexspi2Bus2bitIppIndIoFaBit1SelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Flexspi2Bus2bitIppIndIoFaBit1SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexspi2Bus2bitIppIndIoFaBit1SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexspi2Bus2bitIppIndIoFaBit1SelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flexspi2Bus2bitIppIndIoFaBit1SelectInput {
            daisy: super::vals::Flexspi2Bus2bitIppIndIoFaBit1SelectInputDaisy,
        }
        let proxy = Flexspi2Bus2bitIppIndIoFaBit1SelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FLEXSPI2_BUS2BIT_IPP_IND_IO_FA_BIT2_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspi2Bus2bitIppIndIoFaBit2SelectInput(pub u32);
impl Flexspi2Bus2bitIppIndIoFaBit2SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexspi2Bus2bitIppIndIoFaBit2SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexspi2Bus2bitIppIndIoFaBit2SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(
        &mut self,
        val: super::vals::Flexspi2Bus2bitIppIndIoFaBit2SelectInputDaisy,
    ) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexspi2Bus2bitIppIndIoFaBit2SelectInput {
    #[inline(always)]
    fn default() -> Flexspi2Bus2bitIppIndIoFaBit2SelectInput {
        Flexspi2Bus2bitIppIndIoFaBit2SelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Flexspi2Bus2bitIppIndIoFaBit2SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexspi2Bus2bitIppIndIoFaBit2SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexspi2Bus2bitIppIndIoFaBit2SelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flexspi2Bus2bitIppIndIoFaBit2SelectInput {
            daisy: super::vals::Flexspi2Bus2bitIppIndIoFaBit2SelectInputDaisy,
        }
        let proxy = Flexspi2Bus2bitIppIndIoFaBit2SelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FLEXSPI2_BUS2BIT_IPP_IND_IO_FA_BIT3_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspi2Bus2bitIppIndIoFaBit3SelectInput(pub u32);
impl Flexspi2Bus2bitIppIndIoFaBit3SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexspi2Bus2bitIppIndIoFaBit3SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexspi2Bus2bitIppIndIoFaBit3SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(
        &mut self,
        val: super::vals::Flexspi2Bus2bitIppIndIoFaBit3SelectInputDaisy,
    ) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexspi2Bus2bitIppIndIoFaBit3SelectInput {
    #[inline(always)]
    fn default() -> Flexspi2Bus2bitIppIndIoFaBit3SelectInput {
        Flexspi2Bus2bitIppIndIoFaBit3SelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Flexspi2Bus2bitIppIndIoFaBit3SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexspi2Bus2bitIppIndIoFaBit3SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexspi2Bus2bitIppIndIoFaBit3SelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flexspi2Bus2bitIppIndIoFaBit3SelectInput {
            daisy: super::vals::Flexspi2Bus2bitIppIndIoFaBit3SelectInputDaisy,
        }
        let proxy = Flexspi2Bus2bitIppIndIoFaBit3SelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FLEXSPI2_BUS2BIT_IPP_IND_IO_FB_BIT0_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspi2Bus2bitIppIndIoFbBit0SelectInput(pub u32);
impl Flexspi2Bus2bitIppIndIoFbBit0SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexspi2Bus2bitIppIndIoFbBit0SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Flexspi2Bus2bitIppIndIoFbBit0SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(
        &mut self,
        val: super::vals::Flexspi2Bus2bitIppIndIoFbBit0SelectInputDaisy,
    ) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Flexspi2Bus2bitIppIndIoFbBit0SelectInput {
    #[inline(always)]
    fn default() -> Flexspi2Bus2bitIppIndIoFbBit0SelectInput {
        Flexspi2Bus2bitIppIndIoFbBit0SelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Flexspi2Bus2bitIppIndIoFbBit0SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexspi2Bus2bitIppIndIoFbBit0SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexspi2Bus2bitIppIndIoFbBit0SelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flexspi2Bus2bitIppIndIoFbBit0SelectInput {
            daisy: super::vals::Flexspi2Bus2bitIppIndIoFbBit0SelectInputDaisy,
        }
        let proxy = Flexspi2Bus2bitIppIndIoFbBit0SelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FLEXSPI2_BUS2BIT_IPP_IND_IO_FB_BIT1_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspi2Bus2bitIppIndIoFbBit1SelectInput(pub u32);
impl Flexspi2Bus2bitIppIndIoFbBit1SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexspi2Bus2bitIppIndIoFbBit1SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Flexspi2Bus2bitIppIndIoFbBit1SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(
        &mut self,
        val: super::vals::Flexspi2Bus2bitIppIndIoFbBit1SelectInputDaisy,
    ) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Flexspi2Bus2bitIppIndIoFbBit1SelectInput {
    #[inline(always)]
    fn default() -> Flexspi2Bus2bitIppIndIoFbBit1SelectInput {
        Flexspi2Bus2bitIppIndIoFbBit1SelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Flexspi2Bus2bitIppIndIoFbBit1SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexspi2Bus2bitIppIndIoFbBit1SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexspi2Bus2bitIppIndIoFbBit1SelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flexspi2Bus2bitIppIndIoFbBit1SelectInput {
            daisy: super::vals::Flexspi2Bus2bitIppIndIoFbBit1SelectInputDaisy,
        }
        let proxy = Flexspi2Bus2bitIppIndIoFbBit1SelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FLEXSPI2_BUS2BIT_IPP_IND_IO_FB_BIT2_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspi2Bus2bitIppIndIoFbBit2SelectInput(pub u32);
impl Flexspi2Bus2bitIppIndIoFbBit2SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexspi2Bus2bitIppIndIoFbBit2SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Flexspi2Bus2bitIppIndIoFbBit2SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(
        &mut self,
        val: super::vals::Flexspi2Bus2bitIppIndIoFbBit2SelectInputDaisy,
    ) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Flexspi2Bus2bitIppIndIoFbBit2SelectInput {
    #[inline(always)]
    fn default() -> Flexspi2Bus2bitIppIndIoFbBit2SelectInput {
        Flexspi2Bus2bitIppIndIoFbBit2SelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Flexspi2Bus2bitIppIndIoFbBit2SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexspi2Bus2bitIppIndIoFbBit2SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexspi2Bus2bitIppIndIoFbBit2SelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flexspi2Bus2bitIppIndIoFbBit2SelectInput {
            daisy: super::vals::Flexspi2Bus2bitIppIndIoFbBit2SelectInputDaisy,
        }
        let proxy = Flexspi2Bus2bitIppIndIoFbBit2SelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FLEXSPI2_BUS2BIT_IPP_IND_IO_FB_BIT3_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspi2Bus2bitIppIndIoFbBit3SelectInput(pub u32);
impl Flexspi2Bus2bitIppIndIoFbBit3SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexspi2Bus2bitIppIndIoFbBit3SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Flexspi2Bus2bitIppIndIoFbBit3SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(
        &mut self,
        val: super::vals::Flexspi2Bus2bitIppIndIoFbBit3SelectInputDaisy,
    ) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Flexspi2Bus2bitIppIndIoFbBit3SelectInput {
    #[inline(always)]
    fn default() -> Flexspi2Bus2bitIppIndIoFbBit3SelectInput {
        Flexspi2Bus2bitIppIndIoFbBit3SelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Flexspi2Bus2bitIppIndIoFbBit3SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexspi2Bus2bitIppIndIoFbBit3SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexspi2Bus2bitIppIndIoFbBit3SelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flexspi2Bus2bitIppIndIoFbBit3SelectInput {
            daisy: super::vals::Flexspi2Bus2bitIppIndIoFbBit3SelectInputDaisy,
        }
        let proxy = Flexspi2Bus2bitIppIndIoFbBit3SelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FLEXSPI2_BUS2BIT_IPP_IND_SCK_FA_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspi2Bus2bitIppIndSckFaSelectInput(pub u32);
impl Flexspi2Bus2bitIppIndSckFaSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexspi2Bus2bitIppIndSckFaSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexspi2Bus2bitIppIndSckFaSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(
        &mut self,
        val: super::vals::Flexspi2Bus2bitIppIndSckFaSelectInputDaisy,
    ) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexspi2Bus2bitIppIndSckFaSelectInput {
    #[inline(always)]
    fn default() -> Flexspi2Bus2bitIppIndSckFaSelectInput {
        Flexspi2Bus2bitIppIndSckFaSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Flexspi2Bus2bitIppIndSckFaSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexspi2Bus2bitIppIndSckFaSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexspi2Bus2bitIppIndSckFaSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flexspi2Bus2bitIppIndSckFaSelectInput {
            daisy: super::vals::Flexspi2Bus2bitIppIndSckFaSelectInputDaisy,
        }
        let proxy = Flexspi2Bus2bitIppIndSckFaSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FLEXSPI2_BUS2BIT_IPP_IND_SCK_FB_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspi2Bus2bitIppIndSckFbSelectInput(pub u32);
impl Flexspi2Bus2bitIppIndSckFbSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexspi2Bus2bitIppIndSckFbSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexspi2Bus2bitIppIndSckFbSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(
        &mut self,
        val: super::vals::Flexspi2Bus2bitIppIndSckFbSelectInputDaisy,
    ) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexspi2Bus2bitIppIndSckFbSelectInput {
    #[inline(always)]
    fn default() -> Flexspi2Bus2bitIppIndSckFbSelectInput {
        Flexspi2Bus2bitIppIndSckFbSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Flexspi2Bus2bitIppIndSckFbSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexspi2Bus2bitIppIndSckFbSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexspi2Bus2bitIppIndSckFbSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flexspi2Bus2bitIppIndSckFbSelectInput {
            daisy: super::vals::Flexspi2Bus2bitIppIndSckFbSelectInputDaisy,
        }
        let proxy = Flexspi2Bus2bitIppIndSckFbSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "I3C2_PIN_SCL_IN_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3c2PinSclInSelectInput(pub u32);
impl I3c2PinSclInSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::I3c2PinSclInSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::I3c2PinSclInSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::I3c2PinSclInSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for I3c2PinSclInSelectInput {
    #[inline(always)]
    fn default() -> I3c2PinSclInSelectInput {
        I3c2PinSclInSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for I3c2PinSclInSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3c2PinSclInSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3c2PinSclInSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct I3c2PinSclInSelectInput {
            daisy: super::vals::I3c2PinSclInSelectInputDaisy,
        }
        let proxy = I3c2PinSclInSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "I3C2_PIN_SDA_IN_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3c2PinSdaInSelectInput(pub u32);
impl I3c2PinSdaInSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::I3c2PinSdaInSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::I3c2PinSdaInSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::I3c2PinSdaInSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for I3c2PinSdaInSelectInput {
    #[inline(always)]
    fn default() -> I3c2PinSdaInSelectInput {
        I3c2PinSdaInSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for I3c2PinSdaInSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3c2PinSdaInSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3c2PinSdaInSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct I3c2PinSdaInSelectInput {
            daisy: super::vals::I3c2PinSdaInSelectInputDaisy,
        }
        let proxy = I3c2PinSdaInSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "KPP_IPP_IND_COL_SELECT_INPUT_0 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KppIppIndColSelectInput0(pub u32);
impl KppIppIndColSelectInput0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::KppIppIndColSelectInput0Daisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::KppIppIndColSelectInput0Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::KppIppIndColSelectInput0Daisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for KppIppIndColSelectInput0 {
    #[inline(always)]
    fn default() -> KppIppIndColSelectInput0 {
        KppIppIndColSelectInput0(0u64 as u32)
    }
}
impl core::fmt::Debug for KppIppIndColSelectInput0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KppIppIndColSelectInput0")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KppIppIndColSelectInput0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct KppIppIndColSelectInput0 {
            daisy: super::vals::KppIppIndColSelectInput0Daisy,
        }
        let proxy = KppIppIndColSelectInput0 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "KPP_IPP_IND_COL_SELECT_INPUT_1 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KppIppIndColSelectInput1(pub u32);
impl KppIppIndColSelectInput1 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::KppIppIndColSelectInput1Daisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::KppIppIndColSelectInput1Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::KppIppIndColSelectInput1Daisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for KppIppIndColSelectInput1 {
    #[inline(always)]
    fn default() -> KppIppIndColSelectInput1 {
        KppIppIndColSelectInput1(0u64 as u32)
    }
}
impl core::fmt::Debug for KppIppIndColSelectInput1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KppIppIndColSelectInput1")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KppIppIndColSelectInput1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct KppIppIndColSelectInput1 {
            daisy: super::vals::KppIppIndColSelectInput1Daisy,
        }
        let proxy = KppIppIndColSelectInput1 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "KPP_IPP_IND_COL_SELECT_INPUT_2 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KppIppIndColSelectInput2(pub u32);
impl KppIppIndColSelectInput2 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::KppIppIndColSelectInput2Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::KppIppIndColSelectInput2Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::KppIppIndColSelectInput2Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for KppIppIndColSelectInput2 {
    #[inline(always)]
    fn default() -> KppIppIndColSelectInput2 {
        KppIppIndColSelectInput2(0u64 as u32)
    }
}
impl core::fmt::Debug for KppIppIndColSelectInput2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KppIppIndColSelectInput2")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KppIppIndColSelectInput2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct KppIppIndColSelectInput2 {
            daisy: super::vals::KppIppIndColSelectInput2Daisy,
        }
        let proxy = KppIppIndColSelectInput2 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "KPP_IPP_IND_COL_SELECT_INPUT_3 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KppIppIndColSelectInput3(pub u32);
impl KppIppIndColSelectInput3 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::KppIppIndColSelectInput3Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::KppIppIndColSelectInput3Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::KppIppIndColSelectInput3Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for KppIppIndColSelectInput3 {
    #[inline(always)]
    fn default() -> KppIppIndColSelectInput3 {
        KppIppIndColSelectInput3(0u64 as u32)
    }
}
impl core::fmt::Debug for KppIppIndColSelectInput3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KppIppIndColSelectInput3")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KppIppIndColSelectInput3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct KppIppIndColSelectInput3 {
            daisy: super::vals::KppIppIndColSelectInput3Daisy,
        }
        let proxy = KppIppIndColSelectInput3 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "KPP_IPP_IND_COL_SELECT_INPUT_4 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KppIppIndColSelectInput4(pub u32);
impl KppIppIndColSelectInput4 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::KppIppIndColSelectInput4Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::KppIppIndColSelectInput4Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::KppIppIndColSelectInput4Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for KppIppIndColSelectInput4 {
    #[inline(always)]
    fn default() -> KppIppIndColSelectInput4 {
        KppIppIndColSelectInput4(0u64 as u32)
    }
}
impl core::fmt::Debug for KppIppIndColSelectInput4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KppIppIndColSelectInput4")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KppIppIndColSelectInput4 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct KppIppIndColSelectInput4 {
            daisy: super::vals::KppIppIndColSelectInput4Daisy,
        }
        let proxy = KppIppIndColSelectInput4 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "KPP_IPP_IND_COL_SELECT_INPUT_5 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KppIppIndColSelectInput5(pub u32);
impl KppIppIndColSelectInput5 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::KppIppIndColSelectInput5Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::KppIppIndColSelectInput5Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::KppIppIndColSelectInput5Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for KppIppIndColSelectInput5 {
    #[inline(always)]
    fn default() -> KppIppIndColSelectInput5 {
        KppIppIndColSelectInput5(0u64 as u32)
    }
}
impl core::fmt::Debug for KppIppIndColSelectInput5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KppIppIndColSelectInput5")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KppIppIndColSelectInput5 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct KppIppIndColSelectInput5 {
            daisy: super::vals::KppIppIndColSelectInput5Daisy,
        }
        let proxy = KppIppIndColSelectInput5 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "KPP_IPP_IND_COL_SELECT_INPUT_6 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KppIppIndColSelectInput6(pub u32);
impl KppIppIndColSelectInput6 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::KppIppIndColSelectInput6Daisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::KppIppIndColSelectInput6Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::KppIppIndColSelectInput6Daisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for KppIppIndColSelectInput6 {
    #[inline(always)]
    fn default() -> KppIppIndColSelectInput6 {
        KppIppIndColSelectInput6(0u64 as u32)
    }
}
impl core::fmt::Debug for KppIppIndColSelectInput6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KppIppIndColSelectInput6")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KppIppIndColSelectInput6 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct KppIppIndColSelectInput6 {
            daisy: super::vals::KppIppIndColSelectInput6Daisy,
        }
        let proxy = KppIppIndColSelectInput6 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "KPP_IPP_IND_COL_SELECT_INPUT_7 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KppIppIndColSelectInput7(pub u32);
impl KppIppIndColSelectInput7 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::KppIppIndColSelectInput7Daisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::KppIppIndColSelectInput7Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::KppIppIndColSelectInput7Daisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for KppIppIndColSelectInput7 {
    #[inline(always)]
    fn default() -> KppIppIndColSelectInput7 {
        KppIppIndColSelectInput7(0u64 as u32)
    }
}
impl core::fmt::Debug for KppIppIndColSelectInput7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KppIppIndColSelectInput7")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KppIppIndColSelectInput7 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct KppIppIndColSelectInput7 {
            daisy: super::vals::KppIppIndColSelectInput7Daisy,
        }
        let proxy = KppIppIndColSelectInput7 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "KPP_IPP_IND_ROW_SELECT_INPUT_0 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KppIppIndRowSelectInput0(pub u32);
impl KppIppIndRowSelectInput0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::KppIppIndRowSelectInput0Daisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::KppIppIndRowSelectInput0Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::KppIppIndRowSelectInput0Daisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for KppIppIndRowSelectInput0 {
    #[inline(always)]
    fn default() -> KppIppIndRowSelectInput0 {
        KppIppIndRowSelectInput0(0u64 as u32)
    }
}
impl core::fmt::Debug for KppIppIndRowSelectInput0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KppIppIndRowSelectInput0")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KppIppIndRowSelectInput0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct KppIppIndRowSelectInput0 {
            daisy: super::vals::KppIppIndRowSelectInput0Daisy,
        }
        let proxy = KppIppIndRowSelectInput0 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "KPP_IPP_IND_ROW_SELECT_INPUT_1 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KppIppIndRowSelectInput1(pub u32);
impl KppIppIndRowSelectInput1 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::KppIppIndRowSelectInput1Daisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::KppIppIndRowSelectInput1Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::KppIppIndRowSelectInput1Daisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for KppIppIndRowSelectInput1 {
    #[inline(always)]
    fn default() -> KppIppIndRowSelectInput1 {
        KppIppIndRowSelectInput1(0u64 as u32)
    }
}
impl core::fmt::Debug for KppIppIndRowSelectInput1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KppIppIndRowSelectInput1")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KppIppIndRowSelectInput1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct KppIppIndRowSelectInput1 {
            daisy: super::vals::KppIppIndRowSelectInput1Daisy,
        }
        let proxy = KppIppIndRowSelectInput1 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "KPP_IPP_IND_ROW_SELECT_INPUT_2 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KppIppIndRowSelectInput2(pub u32);
impl KppIppIndRowSelectInput2 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::KppIppIndRowSelectInput2Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::KppIppIndRowSelectInput2Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::KppIppIndRowSelectInput2Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for KppIppIndRowSelectInput2 {
    #[inline(always)]
    fn default() -> KppIppIndRowSelectInput2 {
        KppIppIndRowSelectInput2(0u64 as u32)
    }
}
impl core::fmt::Debug for KppIppIndRowSelectInput2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KppIppIndRowSelectInput2")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KppIppIndRowSelectInput2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct KppIppIndRowSelectInput2 {
            daisy: super::vals::KppIppIndRowSelectInput2Daisy,
        }
        let proxy = KppIppIndRowSelectInput2 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "KPP_IPP_IND_ROW_SELECT_INPUT_3 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KppIppIndRowSelectInput3(pub u32);
impl KppIppIndRowSelectInput3 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::KppIppIndRowSelectInput3Daisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::KppIppIndRowSelectInput3Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::KppIppIndRowSelectInput3Daisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for KppIppIndRowSelectInput3 {
    #[inline(always)]
    fn default() -> KppIppIndRowSelectInput3 {
        KppIppIndRowSelectInput3(0u64 as u32)
    }
}
impl core::fmt::Debug for KppIppIndRowSelectInput3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KppIppIndRowSelectInput3")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KppIppIndRowSelectInput3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct KppIppIndRowSelectInput3 {
            daisy: super::vals::KppIppIndRowSelectInput3Daisy,
        }
        let proxy = KppIppIndRowSelectInput3 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "KPP_IPP_IND_ROW_SELECT_INPUT_4 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KppIppIndRowSelectInput4(pub u32);
impl KppIppIndRowSelectInput4 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::KppIppIndRowSelectInput4Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::KppIppIndRowSelectInput4Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::KppIppIndRowSelectInput4Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for KppIppIndRowSelectInput4 {
    #[inline(always)]
    fn default() -> KppIppIndRowSelectInput4 {
        KppIppIndRowSelectInput4(0u64 as u32)
    }
}
impl core::fmt::Debug for KppIppIndRowSelectInput4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KppIppIndRowSelectInput4")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KppIppIndRowSelectInput4 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct KppIppIndRowSelectInput4 {
            daisy: super::vals::KppIppIndRowSelectInput4Daisy,
        }
        let proxy = KppIppIndRowSelectInput4 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "KPP_IPP_IND_ROW_SELECT_INPUT_5 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KppIppIndRowSelectInput5(pub u32);
impl KppIppIndRowSelectInput5 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::KppIppIndRowSelectInput5Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::KppIppIndRowSelectInput5Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::KppIppIndRowSelectInput5Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for KppIppIndRowSelectInput5 {
    #[inline(always)]
    fn default() -> KppIppIndRowSelectInput5 {
        KppIppIndRowSelectInput5(0u64 as u32)
    }
}
impl core::fmt::Debug for KppIppIndRowSelectInput5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KppIppIndRowSelectInput5")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KppIppIndRowSelectInput5 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct KppIppIndRowSelectInput5 {
            daisy: super::vals::KppIppIndRowSelectInput5Daisy,
        }
        let proxy = KppIppIndRowSelectInput5 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "KPP_IPP_IND_ROW_SELECT_INPUT_6 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KppIppIndRowSelectInput6(pub u32);
impl KppIppIndRowSelectInput6 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::KppIppIndRowSelectInput6Daisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::KppIppIndRowSelectInput6Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::KppIppIndRowSelectInput6Daisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for KppIppIndRowSelectInput6 {
    #[inline(always)]
    fn default() -> KppIppIndRowSelectInput6 {
        KppIppIndRowSelectInput6(0u64 as u32)
    }
}
impl core::fmt::Debug for KppIppIndRowSelectInput6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KppIppIndRowSelectInput6")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KppIppIndRowSelectInput6 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct KppIppIndRowSelectInput6 {
            daisy: super::vals::KppIppIndRowSelectInput6Daisy,
        }
        let proxy = KppIppIndRowSelectInput6 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "KPP_IPP_IND_ROW_SELECT_INPUT_7 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KppIppIndRowSelectInput7(pub u32);
impl KppIppIndRowSelectInput7 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::KppIppIndRowSelectInput7Daisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::KppIppIndRowSelectInput7Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::KppIppIndRowSelectInput7Daisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for KppIppIndRowSelectInput7 {
    #[inline(always)]
    fn default() -> KppIppIndRowSelectInput7 {
        KppIppIndRowSelectInput7(0u64 as u32)
    }
}
impl core::fmt::Debug for KppIppIndRowSelectInput7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KppIppIndRowSelectInput7")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KppIppIndRowSelectInput7 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct KppIppIndRowSelectInput7 {
            daisy: super::vals::KppIppIndRowSelectInput7Daisy,
        }
        let proxy = KppIppIndRowSelectInput7 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPI2C3_IPP_IND_LPI2C_SCL_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpi2c3IppIndLpi2cSclSelectInput(pub u32);
impl Lpi2c3IppIndLpi2cSclSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpi2c3IppIndLpi2cSclSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpi2c3IppIndLpi2cSclSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpi2c3IppIndLpi2cSclSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpi2c3IppIndLpi2cSclSelectInput {
    #[inline(always)]
    fn default() -> Lpi2c3IppIndLpi2cSclSelectInput {
        Lpi2c3IppIndLpi2cSclSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpi2c3IppIndLpi2cSclSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpi2c3IppIndLpi2cSclSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpi2c3IppIndLpi2cSclSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpi2c3IppIndLpi2cSclSelectInput {
            daisy: super::vals::Lpi2c3IppIndLpi2cSclSelectInputDaisy,
        }
        let proxy = Lpi2c3IppIndLpi2cSclSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPI2C3_IPP_IND_LPI2C_SDA_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpi2c3IppIndLpi2cSdaSelectInput(pub u32);
impl Lpi2c3IppIndLpi2cSdaSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpi2c3IppIndLpi2cSdaSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpi2c3IppIndLpi2cSdaSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpi2c3IppIndLpi2cSdaSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpi2c3IppIndLpi2cSdaSelectInput {
    #[inline(always)]
    fn default() -> Lpi2c3IppIndLpi2cSdaSelectInput {
        Lpi2c3IppIndLpi2cSdaSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpi2c3IppIndLpi2cSdaSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpi2c3IppIndLpi2cSdaSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpi2c3IppIndLpi2cSdaSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpi2c3IppIndLpi2cSdaSelectInput {
            daisy: super::vals::Lpi2c3IppIndLpi2cSdaSelectInputDaisy,
        }
        let proxy = Lpi2c3IppIndLpi2cSdaSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPI2C4_IPP_IND_LPI2C_SCL_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpi2c4IppIndLpi2cSclSelectInput(pub u32);
impl Lpi2c4IppIndLpi2cSclSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpi2c4IppIndLpi2cSclSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpi2c4IppIndLpi2cSclSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpi2c4IppIndLpi2cSclSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpi2c4IppIndLpi2cSclSelectInput {
    #[inline(always)]
    fn default() -> Lpi2c4IppIndLpi2cSclSelectInput {
        Lpi2c4IppIndLpi2cSclSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpi2c4IppIndLpi2cSclSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpi2c4IppIndLpi2cSclSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpi2c4IppIndLpi2cSclSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpi2c4IppIndLpi2cSclSelectInput {
            daisy: super::vals::Lpi2c4IppIndLpi2cSclSelectInputDaisy,
        }
        let proxy = Lpi2c4IppIndLpi2cSclSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPI2C4_IPP_IND_LPI2C_SDA_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpi2c4IppIndLpi2cSdaSelectInput(pub u32);
impl Lpi2c4IppIndLpi2cSdaSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpi2c4IppIndLpi2cSdaSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpi2c4IppIndLpi2cSdaSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpi2c4IppIndLpi2cSdaSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpi2c4IppIndLpi2cSdaSelectInput {
    #[inline(always)]
    fn default() -> Lpi2c4IppIndLpi2cSdaSelectInput {
        Lpi2c4IppIndLpi2cSdaSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpi2c4IppIndLpi2cSdaSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpi2c4IppIndLpi2cSdaSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpi2c4IppIndLpi2cSdaSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpi2c4IppIndLpi2cSdaSelectInput {
            daisy: super::vals::Lpi2c4IppIndLpi2cSdaSelectInputDaisy,
        }
        let proxy = Lpi2c4IppIndLpi2cSdaSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPI2C5_IPP_IND_LPI2C_SCL_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpi2c5IppIndLpi2cSclSelectInput(pub u32);
impl Lpi2c5IppIndLpi2cSclSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpi2c5IppIndLpi2cSclSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpi2c5IppIndLpi2cSclSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpi2c5IppIndLpi2cSclSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpi2c5IppIndLpi2cSclSelectInput {
    #[inline(always)]
    fn default() -> Lpi2c5IppIndLpi2cSclSelectInput {
        Lpi2c5IppIndLpi2cSclSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpi2c5IppIndLpi2cSclSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpi2c5IppIndLpi2cSclSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpi2c5IppIndLpi2cSclSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpi2c5IppIndLpi2cSclSelectInput {
            daisy: super::vals::Lpi2c5IppIndLpi2cSclSelectInputDaisy,
        }
        let proxy = Lpi2c5IppIndLpi2cSclSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPI2C5_IPP_IND_LPI2C_SDA_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpi2c5IppIndLpi2cSdaSelectInput(pub u32);
impl Lpi2c5IppIndLpi2cSdaSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpi2c5IppIndLpi2cSdaSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpi2c5IppIndLpi2cSdaSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpi2c5IppIndLpi2cSdaSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpi2c5IppIndLpi2cSdaSelectInput {
    #[inline(always)]
    fn default() -> Lpi2c5IppIndLpi2cSdaSelectInput {
        Lpi2c5IppIndLpi2cSdaSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpi2c5IppIndLpi2cSdaSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpi2c5IppIndLpi2cSdaSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpi2c5IppIndLpi2cSdaSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpi2c5IppIndLpi2cSdaSelectInput {
            daisy: super::vals::Lpi2c5IppIndLpi2cSdaSelectInputDaisy,
        }
        let proxy = Lpi2c5IppIndLpi2cSdaSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPI2C6_IPP_IND_LPI2C_SCL_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpi2c6IppIndLpi2cSclSelectInput(pub u32);
impl Lpi2c6IppIndLpi2cSclSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpi2c6IppIndLpi2cSclSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpi2c6IppIndLpi2cSclSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpi2c6IppIndLpi2cSclSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpi2c6IppIndLpi2cSclSelectInput {
    #[inline(always)]
    fn default() -> Lpi2c6IppIndLpi2cSclSelectInput {
        Lpi2c6IppIndLpi2cSclSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpi2c6IppIndLpi2cSclSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpi2c6IppIndLpi2cSclSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpi2c6IppIndLpi2cSclSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpi2c6IppIndLpi2cSclSelectInput {
            daisy: super::vals::Lpi2c6IppIndLpi2cSclSelectInputDaisy,
        }
        let proxy = Lpi2c6IppIndLpi2cSclSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPI2C6_IPP_IND_LPI2C_SDA_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpi2c6IppIndLpi2cSdaSelectInput(pub u32);
impl Lpi2c6IppIndLpi2cSdaSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpi2c6IppIndLpi2cSdaSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpi2c6IppIndLpi2cSdaSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpi2c6IppIndLpi2cSdaSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpi2c6IppIndLpi2cSdaSelectInput {
    #[inline(always)]
    fn default() -> Lpi2c6IppIndLpi2cSdaSelectInput {
        Lpi2c6IppIndLpi2cSdaSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpi2c6IppIndLpi2cSdaSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpi2c6IppIndLpi2cSdaSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpi2c6IppIndLpi2cSdaSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpi2c6IppIndLpi2cSdaSelectInput {
            daisy: super::vals::Lpi2c6IppIndLpi2cSdaSelectInputDaisy,
        }
        let proxy = Lpi2c6IppIndLpi2cSdaSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPSPI3_IPP_IND_LPSPI_PCS_SELECT_INPUT_0 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi3IppIndLpspiPcsSelectInput0(pub u32);
impl Lpspi3IppIndLpspiPcsSelectInput0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi3IppIndLpspiPcsSelectInput0Daisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpspi3IppIndLpspiPcsSelectInput0Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi3IppIndLpspiPcsSelectInput0Daisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpspi3IppIndLpspiPcsSelectInput0 {
    #[inline(always)]
    fn default() -> Lpspi3IppIndLpspiPcsSelectInput0 {
        Lpspi3IppIndLpspiPcsSelectInput0(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpspi3IppIndLpspiPcsSelectInput0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi3IppIndLpspiPcsSelectInput0")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi3IppIndLpspiPcsSelectInput0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpspi3IppIndLpspiPcsSelectInput0 {
            daisy: super::vals::Lpspi3IppIndLpspiPcsSelectInput0Daisy,
        }
        let proxy = Lpspi3IppIndLpspiPcsSelectInput0 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPSPI3_IPP_IND_LPSPI_PCS_SELECT_INPUT_1 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi3IppIndLpspiPcsSelectInput1(pub u32);
impl Lpspi3IppIndLpspiPcsSelectInput1 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi3IppIndLpspiPcsSelectInput1Daisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpspi3IppIndLpspiPcsSelectInput1Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi3IppIndLpspiPcsSelectInput1Daisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpspi3IppIndLpspiPcsSelectInput1 {
    #[inline(always)]
    fn default() -> Lpspi3IppIndLpspiPcsSelectInput1 {
        Lpspi3IppIndLpspiPcsSelectInput1(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpspi3IppIndLpspiPcsSelectInput1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi3IppIndLpspiPcsSelectInput1")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi3IppIndLpspiPcsSelectInput1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpspi3IppIndLpspiPcsSelectInput1 {
            daisy: super::vals::Lpspi3IppIndLpspiPcsSelectInput1Daisy,
        }
        let proxy = Lpspi3IppIndLpspiPcsSelectInput1 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPSPI3_IPP_IND_LPSPI_PCS_SELECT_INPUT_2 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi3IppIndLpspiPcsSelectInput2(pub u32);
impl Lpspi3IppIndLpspiPcsSelectInput2 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi3IppIndLpspiPcsSelectInput2Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpspi3IppIndLpspiPcsSelectInput2Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi3IppIndLpspiPcsSelectInput2Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpspi3IppIndLpspiPcsSelectInput2 {
    #[inline(always)]
    fn default() -> Lpspi3IppIndLpspiPcsSelectInput2 {
        Lpspi3IppIndLpspiPcsSelectInput2(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpspi3IppIndLpspiPcsSelectInput2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi3IppIndLpspiPcsSelectInput2")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi3IppIndLpspiPcsSelectInput2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpspi3IppIndLpspiPcsSelectInput2 {
            daisy: super::vals::Lpspi3IppIndLpspiPcsSelectInput2Daisy,
        }
        let proxy = Lpspi3IppIndLpspiPcsSelectInput2 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPSPI3_IPP_IND_LPSPI_PCS_SELECT_INPUT_3 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi3IppIndLpspiPcsSelectInput3(pub u32);
impl Lpspi3IppIndLpspiPcsSelectInput3 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi3IppIndLpspiPcsSelectInput3Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpspi3IppIndLpspiPcsSelectInput3Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi3IppIndLpspiPcsSelectInput3Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpspi3IppIndLpspiPcsSelectInput3 {
    #[inline(always)]
    fn default() -> Lpspi3IppIndLpspiPcsSelectInput3 {
        Lpspi3IppIndLpspiPcsSelectInput3(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpspi3IppIndLpspiPcsSelectInput3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi3IppIndLpspiPcsSelectInput3")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi3IppIndLpspiPcsSelectInput3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpspi3IppIndLpspiPcsSelectInput3 {
            daisy: super::vals::Lpspi3IppIndLpspiPcsSelectInput3Daisy,
        }
        let proxy = Lpspi3IppIndLpspiPcsSelectInput3 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPSPI3_IPP_IND_LPSPI_SCK_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi3IppIndLpspiSckSelectInput(pub u32);
impl Lpspi3IppIndLpspiSckSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi3IppIndLpspiSckSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpspi3IppIndLpspiSckSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi3IppIndLpspiSckSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpspi3IppIndLpspiSckSelectInput {
    #[inline(always)]
    fn default() -> Lpspi3IppIndLpspiSckSelectInput {
        Lpspi3IppIndLpspiSckSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpspi3IppIndLpspiSckSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi3IppIndLpspiSckSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi3IppIndLpspiSckSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpspi3IppIndLpspiSckSelectInput {
            daisy: super::vals::Lpspi3IppIndLpspiSckSelectInputDaisy,
        }
        let proxy = Lpspi3IppIndLpspiSckSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPSPI3_IPP_IND_LPSPI_SDI_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi3IppIndLpspiSdiSelectInput(pub u32);
impl Lpspi3IppIndLpspiSdiSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi3IppIndLpspiSdiSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpspi3IppIndLpspiSdiSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi3IppIndLpspiSdiSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpspi3IppIndLpspiSdiSelectInput {
    #[inline(always)]
    fn default() -> Lpspi3IppIndLpspiSdiSelectInput {
        Lpspi3IppIndLpspiSdiSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpspi3IppIndLpspiSdiSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi3IppIndLpspiSdiSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi3IppIndLpspiSdiSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpspi3IppIndLpspiSdiSelectInput {
            daisy: super::vals::Lpspi3IppIndLpspiSdiSelectInputDaisy,
        }
        let proxy = Lpspi3IppIndLpspiSdiSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPSPI3_IPP_IND_LPSPI_SDO_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi3IppIndLpspiSdoSelectInput(pub u32);
impl Lpspi3IppIndLpspiSdoSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi3IppIndLpspiSdoSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpspi3IppIndLpspiSdoSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi3IppIndLpspiSdoSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpspi3IppIndLpspiSdoSelectInput {
    #[inline(always)]
    fn default() -> Lpspi3IppIndLpspiSdoSelectInput {
        Lpspi3IppIndLpspiSdoSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpspi3IppIndLpspiSdoSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi3IppIndLpspiSdoSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi3IppIndLpspiSdoSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpspi3IppIndLpspiSdoSelectInput {
            daisy: super::vals::Lpspi3IppIndLpspiSdoSelectInputDaisy,
        }
        let proxy = Lpspi3IppIndLpspiSdoSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPSPI4_IPP_IND_LPSPI_PCS_SELECT_INPUT_0 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi4IppIndLpspiPcsSelectInput0(pub u32);
impl Lpspi4IppIndLpspiPcsSelectInput0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi4IppIndLpspiPcsSelectInput0Daisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpspi4IppIndLpspiPcsSelectInput0Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi4IppIndLpspiPcsSelectInput0Daisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpspi4IppIndLpspiPcsSelectInput0 {
    #[inline(always)]
    fn default() -> Lpspi4IppIndLpspiPcsSelectInput0 {
        Lpspi4IppIndLpspiPcsSelectInput0(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpspi4IppIndLpspiPcsSelectInput0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi4IppIndLpspiPcsSelectInput0")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi4IppIndLpspiPcsSelectInput0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpspi4IppIndLpspiPcsSelectInput0 {
            daisy: super::vals::Lpspi4IppIndLpspiPcsSelectInput0Daisy,
        }
        let proxy = Lpspi4IppIndLpspiPcsSelectInput0 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPSPI4_IPP_IND_LPSPI_SCK_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi4IppIndLpspiSckSelectInput(pub u32);
impl Lpspi4IppIndLpspiSckSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi4IppIndLpspiSckSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpspi4IppIndLpspiSckSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi4IppIndLpspiSckSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpspi4IppIndLpspiSckSelectInput {
    #[inline(always)]
    fn default() -> Lpspi4IppIndLpspiSckSelectInput {
        Lpspi4IppIndLpspiSckSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpspi4IppIndLpspiSckSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi4IppIndLpspiSckSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi4IppIndLpspiSckSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpspi4IppIndLpspiSckSelectInput {
            daisy: super::vals::Lpspi4IppIndLpspiSckSelectInputDaisy,
        }
        let proxy = Lpspi4IppIndLpspiSckSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPSPI4_IPP_IND_LPSPI_SDI_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi4IppIndLpspiSdiSelectInput(pub u32);
impl Lpspi4IppIndLpspiSdiSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi4IppIndLpspiSdiSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpspi4IppIndLpspiSdiSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi4IppIndLpspiSdiSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpspi4IppIndLpspiSdiSelectInput {
    #[inline(always)]
    fn default() -> Lpspi4IppIndLpspiSdiSelectInput {
        Lpspi4IppIndLpspiSdiSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpspi4IppIndLpspiSdiSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi4IppIndLpspiSdiSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi4IppIndLpspiSdiSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpspi4IppIndLpspiSdiSelectInput {
            daisy: super::vals::Lpspi4IppIndLpspiSdiSelectInputDaisy,
        }
        let proxy = Lpspi4IppIndLpspiSdiSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPSPI4_IPP_IND_LPSPI_SDO_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi4IppIndLpspiSdoSelectInput(pub u32);
impl Lpspi4IppIndLpspiSdoSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi4IppIndLpspiSdoSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpspi4IppIndLpspiSdoSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi4IppIndLpspiSdoSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpspi4IppIndLpspiSdoSelectInput {
    #[inline(always)]
    fn default() -> Lpspi4IppIndLpspiSdoSelectInput {
        Lpspi4IppIndLpspiSdoSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpspi4IppIndLpspiSdoSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi4IppIndLpspiSdoSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi4IppIndLpspiSdoSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpspi4IppIndLpspiSdoSelectInput {
            daisy: super::vals::Lpspi4IppIndLpspiSdoSelectInputDaisy,
        }
        let proxy = Lpspi4IppIndLpspiSdoSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPSPI5_IPP_IND_LPSPI_PCS_SELECT_INPUT_0 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi5IppIndLpspiPcsSelectInput0(pub u32);
impl Lpspi5IppIndLpspiPcsSelectInput0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi5IppIndLpspiPcsSelectInput0Daisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpspi5IppIndLpspiPcsSelectInput0Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi5IppIndLpspiPcsSelectInput0Daisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpspi5IppIndLpspiPcsSelectInput0 {
    #[inline(always)]
    fn default() -> Lpspi5IppIndLpspiPcsSelectInput0 {
        Lpspi5IppIndLpspiPcsSelectInput0(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpspi5IppIndLpspiPcsSelectInput0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi5IppIndLpspiPcsSelectInput0")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi5IppIndLpspiPcsSelectInput0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpspi5IppIndLpspiPcsSelectInput0 {
            daisy: super::vals::Lpspi5IppIndLpspiPcsSelectInput0Daisy,
        }
        let proxy = Lpspi5IppIndLpspiPcsSelectInput0 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPSPI5_IPP_IND_LPSPI_PCS_SELECT_INPUT_1 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi5IppIndLpspiPcsSelectInput1(pub u32);
impl Lpspi5IppIndLpspiPcsSelectInput1 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi5IppIndLpspiPcsSelectInput1Daisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpspi5IppIndLpspiPcsSelectInput1Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi5IppIndLpspiPcsSelectInput1Daisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpspi5IppIndLpspiPcsSelectInput1 {
    #[inline(always)]
    fn default() -> Lpspi5IppIndLpspiPcsSelectInput1 {
        Lpspi5IppIndLpspiPcsSelectInput1(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpspi5IppIndLpspiPcsSelectInput1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi5IppIndLpspiPcsSelectInput1")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi5IppIndLpspiPcsSelectInput1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpspi5IppIndLpspiPcsSelectInput1 {
            daisy: super::vals::Lpspi5IppIndLpspiPcsSelectInput1Daisy,
        }
        let proxy = Lpspi5IppIndLpspiPcsSelectInput1 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPSPI5_IPP_IND_LPSPI_PCS_SELECT_INPUT_2 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi5IppIndLpspiPcsSelectInput2(pub u32);
impl Lpspi5IppIndLpspiPcsSelectInput2 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi5IppIndLpspiPcsSelectInput2Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpspi5IppIndLpspiPcsSelectInput2Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi5IppIndLpspiPcsSelectInput2Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpspi5IppIndLpspiPcsSelectInput2 {
    #[inline(always)]
    fn default() -> Lpspi5IppIndLpspiPcsSelectInput2 {
        Lpspi5IppIndLpspiPcsSelectInput2(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpspi5IppIndLpspiPcsSelectInput2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi5IppIndLpspiPcsSelectInput2")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi5IppIndLpspiPcsSelectInput2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpspi5IppIndLpspiPcsSelectInput2 {
            daisy: super::vals::Lpspi5IppIndLpspiPcsSelectInput2Daisy,
        }
        let proxy = Lpspi5IppIndLpspiPcsSelectInput2 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPSPI5_IPP_IND_LPSPI_PCS_SELECT_INPUT_3 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi5IppIndLpspiPcsSelectInput3(pub u32);
impl Lpspi5IppIndLpspiPcsSelectInput3 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi5IppIndLpspiPcsSelectInput3Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpspi5IppIndLpspiPcsSelectInput3Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi5IppIndLpspiPcsSelectInput3Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpspi5IppIndLpspiPcsSelectInput3 {
    #[inline(always)]
    fn default() -> Lpspi5IppIndLpspiPcsSelectInput3 {
        Lpspi5IppIndLpspiPcsSelectInput3(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpspi5IppIndLpspiPcsSelectInput3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi5IppIndLpspiPcsSelectInput3")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi5IppIndLpspiPcsSelectInput3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpspi5IppIndLpspiPcsSelectInput3 {
            daisy: super::vals::Lpspi5IppIndLpspiPcsSelectInput3Daisy,
        }
        let proxy = Lpspi5IppIndLpspiPcsSelectInput3 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPSPI5_IPP_IND_LPSPI_SCK_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi5IppIndLpspiSckSelectInput(pub u32);
impl Lpspi5IppIndLpspiSckSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi5IppIndLpspiSckSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpspi5IppIndLpspiSckSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi5IppIndLpspiSckSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpspi5IppIndLpspiSckSelectInput {
    #[inline(always)]
    fn default() -> Lpspi5IppIndLpspiSckSelectInput {
        Lpspi5IppIndLpspiSckSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpspi5IppIndLpspiSckSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi5IppIndLpspiSckSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi5IppIndLpspiSckSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpspi5IppIndLpspiSckSelectInput {
            daisy: super::vals::Lpspi5IppIndLpspiSckSelectInputDaisy,
        }
        let proxy = Lpspi5IppIndLpspiSckSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPSPI5_IPP_IND_LPSPI_SDI_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi5IppIndLpspiSdiSelectInput(pub u32);
impl Lpspi5IppIndLpspiSdiSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi5IppIndLpspiSdiSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpspi5IppIndLpspiSdiSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi5IppIndLpspiSdiSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpspi5IppIndLpspiSdiSelectInput {
    #[inline(always)]
    fn default() -> Lpspi5IppIndLpspiSdiSelectInput {
        Lpspi5IppIndLpspiSdiSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpspi5IppIndLpspiSdiSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi5IppIndLpspiSdiSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi5IppIndLpspiSdiSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpspi5IppIndLpspiSdiSelectInput {
            daisy: super::vals::Lpspi5IppIndLpspiSdiSelectInputDaisy,
        }
        let proxy = Lpspi5IppIndLpspiSdiSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPSPI5_IPP_IND_LPSPI_SDO_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi5IppIndLpspiSdoSelectInput(pub u32);
impl Lpspi5IppIndLpspiSdoSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi5IppIndLpspiSdoSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpspi5IppIndLpspiSdoSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi5IppIndLpspiSdoSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpspi5IppIndLpspiSdoSelectInput {
    #[inline(always)]
    fn default() -> Lpspi5IppIndLpspiSdoSelectInput {
        Lpspi5IppIndLpspiSdoSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpspi5IppIndLpspiSdoSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi5IppIndLpspiSdoSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi5IppIndLpspiSdoSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpspi5IppIndLpspiSdoSelectInput {
            daisy: super::vals::Lpspi5IppIndLpspiSdoSelectInputDaisy,
        }
        let proxy = Lpspi5IppIndLpspiSdoSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPSPI6_IPP_IND_LPSPI_PCS_SELECT_INPUT_0 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi6IppIndLpspiPcsSelectInput0(pub u32);
impl Lpspi6IppIndLpspiPcsSelectInput0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi6IppIndLpspiPcsSelectInput0Daisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpspi6IppIndLpspiPcsSelectInput0Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi6IppIndLpspiPcsSelectInput0Daisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpspi6IppIndLpspiPcsSelectInput0 {
    #[inline(always)]
    fn default() -> Lpspi6IppIndLpspiPcsSelectInput0 {
        Lpspi6IppIndLpspiPcsSelectInput0(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpspi6IppIndLpspiPcsSelectInput0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi6IppIndLpspiPcsSelectInput0")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi6IppIndLpspiPcsSelectInput0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpspi6IppIndLpspiPcsSelectInput0 {
            daisy: super::vals::Lpspi6IppIndLpspiPcsSelectInput0Daisy,
        }
        let proxy = Lpspi6IppIndLpspiPcsSelectInput0 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPSPI6_IPP_IND_LPSPI_PCS_SELECT_INPUT_1 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi6IppIndLpspiPcsSelectInput1(pub u32);
impl Lpspi6IppIndLpspiPcsSelectInput1 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi6IppIndLpspiPcsSelectInput1Daisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpspi6IppIndLpspiPcsSelectInput1Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi6IppIndLpspiPcsSelectInput1Daisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpspi6IppIndLpspiPcsSelectInput1 {
    #[inline(always)]
    fn default() -> Lpspi6IppIndLpspiPcsSelectInput1 {
        Lpspi6IppIndLpspiPcsSelectInput1(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpspi6IppIndLpspiPcsSelectInput1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi6IppIndLpspiPcsSelectInput1")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi6IppIndLpspiPcsSelectInput1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpspi6IppIndLpspiPcsSelectInput1 {
            daisy: super::vals::Lpspi6IppIndLpspiPcsSelectInput1Daisy,
        }
        let proxy = Lpspi6IppIndLpspiPcsSelectInput1 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPSPI6_IPP_IND_LPSPI_PCS_SELECT_INPUT_2 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi6IppIndLpspiPcsSelectInput2(pub u32);
impl Lpspi6IppIndLpspiPcsSelectInput2 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi6IppIndLpspiPcsSelectInput2Daisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpspi6IppIndLpspiPcsSelectInput2Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi6IppIndLpspiPcsSelectInput2Daisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpspi6IppIndLpspiPcsSelectInput2 {
    #[inline(always)]
    fn default() -> Lpspi6IppIndLpspiPcsSelectInput2 {
        Lpspi6IppIndLpspiPcsSelectInput2(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpspi6IppIndLpspiPcsSelectInput2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi6IppIndLpspiPcsSelectInput2")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi6IppIndLpspiPcsSelectInput2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpspi6IppIndLpspiPcsSelectInput2 {
            daisy: super::vals::Lpspi6IppIndLpspiPcsSelectInput2Daisy,
        }
        let proxy = Lpspi6IppIndLpspiPcsSelectInput2 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPSPI6_IPP_IND_LPSPI_PCS_SELECT_INPUT_3 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi6IppIndLpspiPcsSelectInput3(pub u32);
impl Lpspi6IppIndLpspiPcsSelectInput3 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi6IppIndLpspiPcsSelectInput3Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpspi6IppIndLpspiPcsSelectInput3Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi6IppIndLpspiPcsSelectInput3Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpspi6IppIndLpspiPcsSelectInput3 {
    #[inline(always)]
    fn default() -> Lpspi6IppIndLpspiPcsSelectInput3 {
        Lpspi6IppIndLpspiPcsSelectInput3(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpspi6IppIndLpspiPcsSelectInput3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi6IppIndLpspiPcsSelectInput3")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi6IppIndLpspiPcsSelectInput3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpspi6IppIndLpspiPcsSelectInput3 {
            daisy: super::vals::Lpspi6IppIndLpspiPcsSelectInput3Daisy,
        }
        let proxy = Lpspi6IppIndLpspiPcsSelectInput3 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPSPI6_IPP_IND_LPSPI_SCK_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi6IppIndLpspiSckSelectInput(pub u32);
impl Lpspi6IppIndLpspiSckSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi6IppIndLpspiSckSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpspi6IppIndLpspiSckSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi6IppIndLpspiSckSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpspi6IppIndLpspiSckSelectInput {
    #[inline(always)]
    fn default() -> Lpspi6IppIndLpspiSckSelectInput {
        Lpspi6IppIndLpspiSckSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpspi6IppIndLpspiSckSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi6IppIndLpspiSckSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi6IppIndLpspiSckSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpspi6IppIndLpspiSckSelectInput {
            daisy: super::vals::Lpspi6IppIndLpspiSckSelectInputDaisy,
        }
        let proxy = Lpspi6IppIndLpspiSckSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPSPI6_IPP_IND_LPSPI_SDI_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi6IppIndLpspiSdiSelectInput(pub u32);
impl Lpspi6IppIndLpspiSdiSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi6IppIndLpspiSdiSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpspi6IppIndLpspiSdiSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi6IppIndLpspiSdiSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpspi6IppIndLpspiSdiSelectInput {
    #[inline(always)]
    fn default() -> Lpspi6IppIndLpspiSdiSelectInput {
        Lpspi6IppIndLpspiSdiSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpspi6IppIndLpspiSdiSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi6IppIndLpspiSdiSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi6IppIndLpspiSdiSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpspi6IppIndLpspiSdiSelectInput {
            daisy: super::vals::Lpspi6IppIndLpspiSdiSelectInputDaisy,
        }
        let proxy = Lpspi6IppIndLpspiSdiSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPSPI6_IPP_IND_LPSPI_SDO_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi6IppIndLpspiSdoSelectInput(pub u32);
impl Lpspi6IppIndLpspiSdoSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi6IppIndLpspiSdoSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpspi6IppIndLpspiSdoSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi6IppIndLpspiSdoSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpspi6IppIndLpspiSdoSelectInput {
    #[inline(always)]
    fn default() -> Lpspi6IppIndLpspiSdoSelectInput {
        Lpspi6IppIndLpspiSdoSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpspi6IppIndLpspiSdoSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi6IppIndLpspiSdoSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi6IppIndLpspiSdoSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpspi6IppIndLpspiSdoSelectInput {
            daisy: super::vals::Lpspi6IppIndLpspiSdoSelectInputDaisy,
        }
        let proxy = Lpspi6IppIndLpspiSdoSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPUART10_IPP_IND_LPUART_RXD_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart10IppIndLpuartRxdSelectInput(pub u32);
impl Lpuart10IppIndLpuartRxdSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart10IppIndLpuartRxdSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpuart10IppIndLpuartRxdSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart10IppIndLpuartRxdSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart10IppIndLpuartRxdSelectInput {
    #[inline(always)]
    fn default() -> Lpuart10IppIndLpuartRxdSelectInput {
        Lpuart10IppIndLpuartRxdSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpuart10IppIndLpuartRxdSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart10IppIndLpuartRxdSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart10IppIndLpuartRxdSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpuart10IppIndLpuartRxdSelectInput {
            daisy: super::vals::Lpuart10IppIndLpuartRxdSelectInputDaisy,
        }
        let proxy = Lpuart10IppIndLpuartRxdSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPUART10_IPP_IND_LPUART_TXD_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart10IppIndLpuartTxdSelectInput(pub u32);
impl Lpuart10IppIndLpuartTxdSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart10IppIndLpuartTxdSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpuart10IppIndLpuartTxdSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart10IppIndLpuartTxdSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart10IppIndLpuartTxdSelectInput {
    #[inline(always)]
    fn default() -> Lpuart10IppIndLpuartTxdSelectInput {
        Lpuart10IppIndLpuartTxdSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpuart10IppIndLpuartTxdSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart10IppIndLpuartTxdSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart10IppIndLpuartTxdSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpuart10IppIndLpuartTxdSelectInput {
            daisy: super::vals::Lpuart10IppIndLpuartTxdSelectInputDaisy,
        }
        let proxy = Lpuart10IppIndLpuartTxdSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPUART11_IPP_IND_LPUART_RXD_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart11IppIndLpuartRxdSelectInput(pub u32);
impl Lpuart11IppIndLpuartRxdSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart11IppIndLpuartRxdSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpuart11IppIndLpuartRxdSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart11IppIndLpuartRxdSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpuart11IppIndLpuartRxdSelectInput {
    #[inline(always)]
    fn default() -> Lpuart11IppIndLpuartRxdSelectInput {
        Lpuart11IppIndLpuartRxdSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpuart11IppIndLpuartRxdSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart11IppIndLpuartRxdSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart11IppIndLpuartRxdSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpuart11IppIndLpuartRxdSelectInput {
            daisy: super::vals::Lpuart11IppIndLpuartRxdSelectInputDaisy,
        }
        let proxy = Lpuart11IppIndLpuartRxdSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPUART11_IPP_IND_LPUART_TXD_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart11IppIndLpuartTxdSelectInput(pub u32);
impl Lpuart11IppIndLpuartTxdSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart11IppIndLpuartTxdSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpuart11IppIndLpuartTxdSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart11IppIndLpuartTxdSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpuart11IppIndLpuartTxdSelectInput {
    #[inline(always)]
    fn default() -> Lpuart11IppIndLpuartTxdSelectInput {
        Lpuart11IppIndLpuartTxdSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpuart11IppIndLpuartTxdSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart11IppIndLpuartTxdSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart11IppIndLpuartTxdSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpuart11IppIndLpuartTxdSelectInput {
            daisy: super::vals::Lpuart11IppIndLpuartTxdSelectInputDaisy,
        }
        let proxy = Lpuart11IppIndLpuartTxdSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPUART3_IPP_IND_LPUART_CTS_N_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart3IppIndLpuartCtsNSelectInput(pub u32);
impl Lpuart3IppIndLpuartCtsNSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart3IppIndLpuartCtsNSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpuart3IppIndLpuartCtsNSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart3IppIndLpuartCtsNSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart3IppIndLpuartCtsNSelectInput {
    #[inline(always)]
    fn default() -> Lpuart3IppIndLpuartCtsNSelectInput {
        Lpuart3IppIndLpuartCtsNSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpuart3IppIndLpuartCtsNSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart3IppIndLpuartCtsNSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart3IppIndLpuartCtsNSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpuart3IppIndLpuartCtsNSelectInput {
            daisy: super::vals::Lpuart3IppIndLpuartCtsNSelectInputDaisy,
        }
        let proxy = Lpuart3IppIndLpuartCtsNSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPUART3_IPP_IND_LPUART_RXD_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart3IppIndLpuartRxdSelectInput(pub u32);
impl Lpuart3IppIndLpuartRxdSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart3IppIndLpuartRxdSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpuart3IppIndLpuartRxdSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart3IppIndLpuartRxdSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart3IppIndLpuartRxdSelectInput {
    #[inline(always)]
    fn default() -> Lpuart3IppIndLpuartRxdSelectInput {
        Lpuart3IppIndLpuartRxdSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpuart3IppIndLpuartRxdSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart3IppIndLpuartRxdSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart3IppIndLpuartRxdSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpuart3IppIndLpuartRxdSelectInput {
            daisy: super::vals::Lpuart3IppIndLpuartRxdSelectInputDaisy,
        }
        let proxy = Lpuart3IppIndLpuartRxdSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPUART3_IPP_IND_LPUART_TXD_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart3IppIndLpuartTxdSelectInput(pub u32);
impl Lpuart3IppIndLpuartTxdSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart3IppIndLpuartTxdSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpuart3IppIndLpuartTxdSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart3IppIndLpuartTxdSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart3IppIndLpuartTxdSelectInput {
    #[inline(always)]
    fn default() -> Lpuart3IppIndLpuartTxdSelectInput {
        Lpuart3IppIndLpuartTxdSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpuart3IppIndLpuartTxdSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart3IppIndLpuartTxdSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart3IppIndLpuartTxdSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpuart3IppIndLpuartTxdSelectInput {
            daisy: super::vals::Lpuart3IppIndLpuartTxdSelectInputDaisy,
        }
        let proxy = Lpuart3IppIndLpuartTxdSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPUART4_IPP_IND_LPUART_CTS_N_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart4IppIndLpuartCtsNSelectInput(pub u32);
impl Lpuart4IppIndLpuartCtsNSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart4IppIndLpuartCtsNSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpuart4IppIndLpuartCtsNSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart4IppIndLpuartCtsNSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart4IppIndLpuartCtsNSelectInput {
    #[inline(always)]
    fn default() -> Lpuart4IppIndLpuartCtsNSelectInput {
        Lpuart4IppIndLpuartCtsNSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpuart4IppIndLpuartCtsNSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart4IppIndLpuartCtsNSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart4IppIndLpuartCtsNSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpuart4IppIndLpuartCtsNSelectInput {
            daisy: super::vals::Lpuart4IppIndLpuartCtsNSelectInputDaisy,
        }
        let proxy = Lpuart4IppIndLpuartCtsNSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPUART5_IPP_IND_LPUART_CTS_N_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart5IppIndLpuartCtsNSelectInput(pub u32);
impl Lpuart5IppIndLpuartCtsNSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart5IppIndLpuartCtsNSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpuart5IppIndLpuartCtsNSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart5IppIndLpuartCtsNSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpuart5IppIndLpuartCtsNSelectInput {
    #[inline(always)]
    fn default() -> Lpuart5IppIndLpuartCtsNSelectInput {
        Lpuart5IppIndLpuartCtsNSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpuart5IppIndLpuartCtsNSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart5IppIndLpuartCtsNSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart5IppIndLpuartCtsNSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpuart5IppIndLpuartCtsNSelectInput {
            daisy: super::vals::Lpuart5IppIndLpuartCtsNSelectInputDaisy,
        }
        let proxy = Lpuart5IppIndLpuartCtsNSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPUART5_IPP_IND_LPUART_DCD_N_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart5IppIndLpuartDcdNSelectInput(pub u32);
impl Lpuart5IppIndLpuartDcdNSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart5IppIndLpuartDcdNSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpuart5IppIndLpuartDcdNSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart5IppIndLpuartDcdNSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart5IppIndLpuartDcdNSelectInput {
    #[inline(always)]
    fn default() -> Lpuart5IppIndLpuartDcdNSelectInput {
        Lpuart5IppIndLpuartDcdNSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpuart5IppIndLpuartDcdNSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart5IppIndLpuartDcdNSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart5IppIndLpuartDcdNSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpuart5IppIndLpuartDcdNSelectInput {
            daisy: super::vals::Lpuart5IppIndLpuartDcdNSelectInputDaisy,
        }
        let proxy = Lpuart5IppIndLpuartDcdNSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPUART5_IPP_IND_LPUART_DSR_N_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart5IppIndLpuartDsrNSelectInput(pub u32);
impl Lpuart5IppIndLpuartDsrNSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart5IppIndLpuartDsrNSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpuart5IppIndLpuartDsrNSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart5IppIndLpuartDsrNSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart5IppIndLpuartDsrNSelectInput {
    #[inline(always)]
    fn default() -> Lpuart5IppIndLpuartDsrNSelectInput {
        Lpuart5IppIndLpuartDsrNSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpuart5IppIndLpuartDsrNSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart5IppIndLpuartDsrNSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart5IppIndLpuartDsrNSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpuart5IppIndLpuartDsrNSelectInput {
            daisy: super::vals::Lpuart5IppIndLpuartDsrNSelectInputDaisy,
        }
        let proxy = Lpuart5IppIndLpuartDsrNSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPUART5_IPP_IND_LPUART_RI_N_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart5IppIndLpuartRiNSelectInput(pub u32);
impl Lpuart5IppIndLpuartRiNSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart5IppIndLpuartRiNSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpuart5IppIndLpuartRiNSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart5IppIndLpuartRiNSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart5IppIndLpuartRiNSelectInput {
    #[inline(always)]
    fn default() -> Lpuart5IppIndLpuartRiNSelectInput {
        Lpuart5IppIndLpuartRiNSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpuart5IppIndLpuartRiNSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart5IppIndLpuartRiNSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart5IppIndLpuartRiNSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpuart5IppIndLpuartRiNSelectInput {
            daisy: super::vals::Lpuart5IppIndLpuartRiNSelectInputDaisy,
        }
        let proxy = Lpuart5IppIndLpuartRiNSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPUART5_IPP_IND_LPUART_RXD_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart5IppIndLpuartRxdSelectInput(pub u32);
impl Lpuart5IppIndLpuartRxdSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart5IppIndLpuartRxdSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Lpuart5IppIndLpuartRxdSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart5IppIndLpuartRxdSelectInputDaisy) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Lpuart5IppIndLpuartRxdSelectInput {
    #[inline(always)]
    fn default() -> Lpuart5IppIndLpuartRxdSelectInput {
        Lpuart5IppIndLpuartRxdSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpuart5IppIndLpuartRxdSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart5IppIndLpuartRxdSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart5IppIndLpuartRxdSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpuart5IppIndLpuartRxdSelectInput {
            daisy: super::vals::Lpuart5IppIndLpuartRxdSelectInputDaisy,
        }
        let proxy = Lpuart5IppIndLpuartRxdSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPUART5_IPP_IND_LPUART_TXD_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart5IppIndLpuartTxdSelectInput(pub u32);
impl Lpuart5IppIndLpuartTxdSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart5IppIndLpuartTxdSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Lpuart5IppIndLpuartTxdSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart5IppIndLpuartTxdSelectInputDaisy) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Lpuart5IppIndLpuartTxdSelectInput {
    #[inline(always)]
    fn default() -> Lpuart5IppIndLpuartTxdSelectInput {
        Lpuart5IppIndLpuartTxdSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpuart5IppIndLpuartTxdSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart5IppIndLpuartTxdSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart5IppIndLpuartTxdSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpuart5IppIndLpuartTxdSelectInput {
            daisy: super::vals::Lpuart5IppIndLpuartTxdSelectInputDaisy,
        }
        let proxy = Lpuart5IppIndLpuartTxdSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPUART6_IPP_IND_LPUART_CTS_N_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart6IppIndLpuartCtsNSelectInput(pub u32);
impl Lpuart6IppIndLpuartCtsNSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart6IppIndLpuartCtsNSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpuart6IppIndLpuartCtsNSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart6IppIndLpuartCtsNSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpuart6IppIndLpuartCtsNSelectInput {
    #[inline(always)]
    fn default() -> Lpuart6IppIndLpuartCtsNSelectInput {
        Lpuart6IppIndLpuartCtsNSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpuart6IppIndLpuartCtsNSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart6IppIndLpuartCtsNSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart6IppIndLpuartCtsNSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpuart6IppIndLpuartCtsNSelectInput {
            daisy: super::vals::Lpuart6IppIndLpuartCtsNSelectInputDaisy,
        }
        let proxy = Lpuart6IppIndLpuartCtsNSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPUART6_IPP_IND_LPUART_DCD_N_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart6IppIndLpuartDcdNSelectInput(pub u32);
impl Lpuart6IppIndLpuartDcdNSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart6IppIndLpuartDcdNSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpuart6IppIndLpuartDcdNSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart6IppIndLpuartDcdNSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart6IppIndLpuartDcdNSelectInput {
    #[inline(always)]
    fn default() -> Lpuart6IppIndLpuartDcdNSelectInput {
        Lpuart6IppIndLpuartDcdNSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpuart6IppIndLpuartDcdNSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart6IppIndLpuartDcdNSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart6IppIndLpuartDcdNSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpuart6IppIndLpuartDcdNSelectInput {
            daisy: super::vals::Lpuart6IppIndLpuartDcdNSelectInputDaisy,
        }
        let proxy = Lpuart6IppIndLpuartDcdNSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPUART6_IPP_IND_LPUART_DSR_N_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart6IppIndLpuartDsrNSelectInput(pub u32);
impl Lpuart6IppIndLpuartDsrNSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart6IppIndLpuartDsrNSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpuart6IppIndLpuartDsrNSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart6IppIndLpuartDsrNSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart6IppIndLpuartDsrNSelectInput {
    #[inline(always)]
    fn default() -> Lpuart6IppIndLpuartDsrNSelectInput {
        Lpuart6IppIndLpuartDsrNSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpuart6IppIndLpuartDsrNSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart6IppIndLpuartDsrNSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart6IppIndLpuartDsrNSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpuart6IppIndLpuartDsrNSelectInput {
            daisy: super::vals::Lpuart6IppIndLpuartDsrNSelectInputDaisy,
        }
        let proxy = Lpuart6IppIndLpuartDsrNSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPUART6_IPP_IND_LPUART_RI_N_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart6IppIndLpuartRiNSelectInput(pub u32);
impl Lpuart6IppIndLpuartRiNSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart6IppIndLpuartRiNSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpuart6IppIndLpuartRiNSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart6IppIndLpuartRiNSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart6IppIndLpuartRiNSelectInput {
    #[inline(always)]
    fn default() -> Lpuart6IppIndLpuartRiNSelectInput {
        Lpuart6IppIndLpuartRiNSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpuart6IppIndLpuartRiNSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart6IppIndLpuartRiNSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart6IppIndLpuartRiNSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpuart6IppIndLpuartRiNSelectInput {
            daisy: super::vals::Lpuart6IppIndLpuartRiNSelectInputDaisy,
        }
        let proxy = Lpuart6IppIndLpuartRiNSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPUART6_IPP_IND_LPUART_RXD_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart6IppIndLpuartRxdSelectInput(pub u32);
impl Lpuart6IppIndLpuartRxdSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart6IppIndLpuartRxdSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpuart6IppIndLpuartRxdSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart6IppIndLpuartRxdSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpuart6IppIndLpuartRxdSelectInput {
    #[inline(always)]
    fn default() -> Lpuart6IppIndLpuartRxdSelectInput {
        Lpuart6IppIndLpuartRxdSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpuart6IppIndLpuartRxdSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart6IppIndLpuartRxdSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart6IppIndLpuartRxdSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpuart6IppIndLpuartRxdSelectInput {
            daisy: super::vals::Lpuart6IppIndLpuartRxdSelectInputDaisy,
        }
        let proxy = Lpuart6IppIndLpuartRxdSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPUART6_IPP_IND_LPUART_TXD_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart6IppIndLpuartTxdSelectInput(pub u32);
impl Lpuart6IppIndLpuartTxdSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart6IppIndLpuartTxdSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpuart6IppIndLpuartTxdSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart6IppIndLpuartTxdSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpuart6IppIndLpuartTxdSelectInput {
    #[inline(always)]
    fn default() -> Lpuart6IppIndLpuartTxdSelectInput {
        Lpuart6IppIndLpuartTxdSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpuart6IppIndLpuartTxdSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart6IppIndLpuartTxdSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart6IppIndLpuartTxdSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpuart6IppIndLpuartTxdSelectInput {
            daisy: super::vals::Lpuart6IppIndLpuartTxdSelectInputDaisy,
        }
        let proxy = Lpuart6IppIndLpuartTxdSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPUART8_IPP_IND_LPUART_CTS_N_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart8IppIndLpuartCtsNSelectInput(pub u32);
impl Lpuart8IppIndLpuartCtsNSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart8IppIndLpuartCtsNSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpuart8IppIndLpuartCtsNSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart8IppIndLpuartCtsNSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart8IppIndLpuartCtsNSelectInput {
    #[inline(always)]
    fn default() -> Lpuart8IppIndLpuartCtsNSelectInput {
        Lpuart8IppIndLpuartCtsNSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpuart8IppIndLpuartCtsNSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart8IppIndLpuartCtsNSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart8IppIndLpuartCtsNSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpuart8IppIndLpuartCtsNSelectInput {
            daisy: super::vals::Lpuart8IppIndLpuartCtsNSelectInputDaisy,
        }
        let proxy = Lpuart8IppIndLpuartCtsNSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPUART8_IPP_IND_LPUART_RXD_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart8IppIndLpuartRxdSelectInput(pub u32);
impl Lpuart8IppIndLpuartRxdSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart8IppIndLpuartRxdSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpuart8IppIndLpuartRxdSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart8IppIndLpuartRxdSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpuart8IppIndLpuartRxdSelectInput {
    #[inline(always)]
    fn default() -> Lpuart8IppIndLpuartRxdSelectInput {
        Lpuart8IppIndLpuartRxdSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpuart8IppIndLpuartRxdSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart8IppIndLpuartRxdSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart8IppIndLpuartRxdSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpuart8IppIndLpuartRxdSelectInput {
            daisy: super::vals::Lpuart8IppIndLpuartRxdSelectInputDaisy,
        }
        let proxy = Lpuart8IppIndLpuartRxdSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPUART8_IPP_IND_LPUART_TXD_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart8IppIndLpuartTxdSelectInput(pub u32);
impl Lpuart8IppIndLpuartTxdSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart8IppIndLpuartTxdSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpuart8IppIndLpuartTxdSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart8IppIndLpuartTxdSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpuart8IppIndLpuartTxdSelectInput {
    #[inline(always)]
    fn default() -> Lpuart8IppIndLpuartTxdSelectInput {
        Lpuart8IppIndLpuartTxdSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpuart8IppIndLpuartTxdSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart8IppIndLpuartTxdSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart8IppIndLpuartTxdSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpuart8IppIndLpuartTxdSelectInput {
            daisy: super::vals::Lpuart8IppIndLpuartTxdSelectInputDaisy,
        }
        let proxy = Lpuart8IppIndLpuartTxdSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPUART9_IPP_IND_LPUART_RXD_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart9IppIndLpuartRxdSelectInput(pub u32);
impl Lpuart9IppIndLpuartRxdSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart9IppIndLpuartRxdSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpuart9IppIndLpuartRxdSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart9IppIndLpuartRxdSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart9IppIndLpuartRxdSelectInput {
    #[inline(always)]
    fn default() -> Lpuart9IppIndLpuartRxdSelectInput {
        Lpuart9IppIndLpuartRxdSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpuart9IppIndLpuartRxdSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart9IppIndLpuartRxdSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart9IppIndLpuartRxdSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpuart9IppIndLpuartRxdSelectInput {
            daisy: super::vals::Lpuart9IppIndLpuartRxdSelectInputDaisy,
        }
        let proxy = Lpuart9IppIndLpuartRxdSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPUART9_IPP_IND_LPUART_TXD_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart9IppIndLpuartTxdSelectInput(pub u32);
impl Lpuart9IppIndLpuartTxdSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart9IppIndLpuartTxdSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpuart9IppIndLpuartTxdSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart9IppIndLpuartTxdSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart9IppIndLpuartTxdSelectInput {
    #[inline(always)]
    fn default() -> Lpuart9IppIndLpuartTxdSelectInput {
        Lpuart9IppIndLpuartTxdSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Lpuart9IppIndLpuartTxdSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart9IppIndLpuartTxdSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart9IppIndLpuartTxdSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpuart9IppIndLpuartTxdSelectInput {
            daisy: super::vals::Lpuart9IppIndLpuartTxdSelectInputDaisy,
        }
        let proxy = Lpuart9IppIndLpuartTxdSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "MIC_IPP_IND_MIC_PDM_BITSTREAM_SELECT_INPUT_0 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MicIppIndMicPdmBitstreamSelectInput0(pub u32);
impl MicIppIndMicPdmBitstreamSelectInput0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::MicIppIndMicPdmBitstreamSelectInput0Daisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::MicIppIndMicPdmBitstreamSelectInput0Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::MicIppIndMicPdmBitstreamSelectInput0Daisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for MicIppIndMicPdmBitstreamSelectInput0 {
    #[inline(always)]
    fn default() -> MicIppIndMicPdmBitstreamSelectInput0 {
        MicIppIndMicPdmBitstreamSelectInput0(0u64 as u32)
    }
}
impl core::fmt::Debug for MicIppIndMicPdmBitstreamSelectInput0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MicIppIndMicPdmBitstreamSelectInput0")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MicIppIndMicPdmBitstreamSelectInput0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MicIppIndMicPdmBitstreamSelectInput0 {
            daisy: super::vals::MicIppIndMicPdmBitstreamSelectInput0Daisy,
        }
        let proxy = MicIppIndMicPdmBitstreamSelectInput0 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "MIC_IPP_IND_MIC_PDM_BITSTREAM_SELECT_INPUT_1 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MicIppIndMicPdmBitstreamSelectInput1(pub u32);
impl MicIppIndMicPdmBitstreamSelectInput1 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::MicIppIndMicPdmBitstreamSelectInput1Daisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::MicIppIndMicPdmBitstreamSelectInput1Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::MicIppIndMicPdmBitstreamSelectInput1Daisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for MicIppIndMicPdmBitstreamSelectInput1 {
    #[inline(always)]
    fn default() -> MicIppIndMicPdmBitstreamSelectInput1 {
        MicIppIndMicPdmBitstreamSelectInput1(0u64 as u32)
    }
}
impl core::fmt::Debug for MicIppIndMicPdmBitstreamSelectInput1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MicIppIndMicPdmBitstreamSelectInput1")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MicIppIndMicPdmBitstreamSelectInput1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MicIppIndMicPdmBitstreamSelectInput1 {
            daisy: super::vals::MicIppIndMicPdmBitstreamSelectInput1Daisy,
        }
        let proxy = MicIppIndMicPdmBitstreamSelectInput1 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "MIC_IPP_IND_MIC_PDM_BITSTREAM_SELECT_INPUT_2 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MicIppIndMicPdmBitstreamSelectInput2(pub u32);
impl MicIppIndMicPdmBitstreamSelectInput2 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::MicIppIndMicPdmBitstreamSelectInput2Daisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::MicIppIndMicPdmBitstreamSelectInput2Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::MicIppIndMicPdmBitstreamSelectInput2Daisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for MicIppIndMicPdmBitstreamSelectInput2 {
    #[inline(always)]
    fn default() -> MicIppIndMicPdmBitstreamSelectInput2 {
        MicIppIndMicPdmBitstreamSelectInput2(0u64 as u32)
    }
}
impl core::fmt::Debug for MicIppIndMicPdmBitstreamSelectInput2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MicIppIndMicPdmBitstreamSelectInput2")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MicIppIndMicPdmBitstreamSelectInput2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MicIppIndMicPdmBitstreamSelectInput2 {
            daisy: super::vals::MicIppIndMicPdmBitstreamSelectInput2Daisy,
        }
        let proxy = MicIppIndMicPdmBitstreamSelectInput2 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "MIC_IPP_IND_MIC_PDM_BITSTREAM_SELECT_INPUT_3 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MicIppIndMicPdmBitstreamSelectInput3(pub u32);
impl MicIppIndMicPdmBitstreamSelectInput3 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::MicIppIndMicPdmBitstreamSelectInput3Daisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::MicIppIndMicPdmBitstreamSelectInput3Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::MicIppIndMicPdmBitstreamSelectInput3Daisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for MicIppIndMicPdmBitstreamSelectInput3 {
    #[inline(always)]
    fn default() -> MicIppIndMicPdmBitstreamSelectInput3 {
        MicIppIndMicPdmBitstreamSelectInput3(0u64 as u32)
    }
}
impl core::fmt::Debug for MicIppIndMicPdmBitstreamSelectInput3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MicIppIndMicPdmBitstreamSelectInput3")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MicIppIndMicPdmBitstreamSelectInput3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MicIppIndMicPdmBitstreamSelectInput3 {
            daisy: super::vals::MicIppIndMicPdmBitstreamSelectInput3Daisy,
        }
        let proxy = MicIppIndMicPdmBitstreamSelectInput3 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_CLKGEN_IPP_TMR_CLK_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcClkgenIppTmrClkSelectInput(pub u32);
impl NetcClkgenIppTmrClkSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcClkgenIppTmrClkSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::NetcClkgenIppTmrClkSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcClkgenIppTmrClkSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for NetcClkgenIppTmrClkSelectInput {
    #[inline(always)]
    fn default() -> NetcClkgenIppTmrClkSelectInput {
        NetcClkgenIppTmrClkSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcClkgenIppTmrClkSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcClkgenIppTmrClkSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcClkgenIppTmrClkSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcClkgenIppTmrClkSelectInput {
            daisy: super::vals::NetcClkgenIppTmrClkSelectInputDaisy,
        }
        let proxy = NetcClkgenIppTmrClkSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_EMDIO_IN_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcEmdioInSelectInput(pub u32);
impl NetcEmdioInSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcEmdioInSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::NetcEmdioInSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcEmdioInSelectInputDaisy) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for NetcEmdioInSelectInput {
    #[inline(always)]
    fn default() -> NetcEmdioInSelectInput {
        NetcEmdioInSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcEmdioInSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcEmdioInSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcEmdioInSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcEmdioInSelectInput {
            daisy: super::vals::NetcEmdioInSelectInputDaisy,
        }
        let proxy = NetcEmdioInSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_ETH2_COL_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcEth2ColSelectInput(pub u32);
impl NetcEth2ColSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcEth2ColSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::NetcEth2ColSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcEth2ColSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for NetcEth2ColSelectInput {
    #[inline(always)]
    fn default() -> NetcEth2ColSelectInput {
        NetcEth2ColSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcEth2ColSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcEth2ColSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcEth2ColSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcEth2ColSelectInput {
            daisy: super::vals::NetcEth2ColSelectInputDaisy,
        }
        let proxy = NetcEth2ColSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_ETH2_CRS_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcEth2CrsSelectInput(pub u32);
impl NetcEth2CrsSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcEth2CrsSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::NetcEth2CrsSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcEth2CrsSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for NetcEth2CrsSelectInput {
    #[inline(always)]
    fn default() -> NetcEth2CrsSelectInput {
        NetcEth2CrsSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcEth2CrsSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcEth2CrsSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcEth2CrsSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcEth2CrsSelectInput {
            daisy: super::vals::NetcEth2CrsSelectInputDaisy,
        }
        let proxy = NetcEth2CrsSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_ETH2_SLV_MDC_IN_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcEth2SlvMdcInSelectInput(pub u32);
impl NetcEth2SlvMdcInSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcEth2SlvMdcInSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::NetcEth2SlvMdcInSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcEth2SlvMdcInSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for NetcEth2SlvMdcInSelectInput {
    #[inline(always)]
    fn default() -> NetcEth2SlvMdcInSelectInput {
        NetcEth2SlvMdcInSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcEth2SlvMdcInSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcEth2SlvMdcInSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcEth2SlvMdcInSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcEth2SlvMdcInSelectInput {
            daisy: super::vals::NetcEth2SlvMdcInSelectInputDaisy,
        }
        let proxy = NetcEth2SlvMdcInSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_ETH2_SLV_MDIO_IN_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcEth2SlvMdioInSelectInput(pub u32);
impl NetcEth2SlvMdioInSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcEth2SlvMdioInSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::NetcEth2SlvMdioInSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcEth2SlvMdioInSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for NetcEth2SlvMdioInSelectInput {
    #[inline(always)]
    fn default() -> NetcEth2SlvMdioInSelectInput {
        NetcEth2SlvMdioInSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcEth2SlvMdioInSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcEth2SlvMdioInSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcEth2SlvMdioInSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcEth2SlvMdioInSelectInput {
            daisy: super::vals::NetcEth2SlvMdioInSelectInputDaisy,
        }
        let proxy = NetcEth2SlvMdioInSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_ETH3_COL_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcEth3ColSelectInput(pub u32);
impl NetcEth3ColSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcEth3ColSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::NetcEth3ColSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcEth3ColSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for NetcEth3ColSelectInput {
    #[inline(always)]
    fn default() -> NetcEth3ColSelectInput {
        NetcEth3ColSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcEth3ColSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcEth3ColSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcEth3ColSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcEth3ColSelectInput {
            daisy: super::vals::NetcEth3ColSelectInputDaisy,
        }
        let proxy = NetcEth3ColSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_ETH3_CRS_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcEth3CrsSelectInput(pub u32);
impl NetcEth3CrsSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcEth3CrsSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::NetcEth3CrsSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcEth3CrsSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for NetcEth3CrsSelectInput {
    #[inline(always)]
    fn default() -> NetcEth3CrsSelectInput {
        NetcEth3CrsSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcEth3CrsSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcEth3CrsSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcEth3CrsSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcEth3CrsSelectInput {
            daisy: super::vals::NetcEth3CrsSelectInputDaisy,
        }
        let proxy = NetcEth3CrsSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_ETH3_SLV_MDC_IN_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcEth3SlvMdcInSelectInput(pub u32);
impl NetcEth3SlvMdcInSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcEth3SlvMdcInSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::NetcEth3SlvMdcInSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcEth3SlvMdcInSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for NetcEth3SlvMdcInSelectInput {
    #[inline(always)]
    fn default() -> NetcEth3SlvMdcInSelectInput {
        NetcEth3SlvMdcInSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcEth3SlvMdcInSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcEth3SlvMdcInSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcEth3SlvMdcInSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcEth3SlvMdcInSelectInput {
            daisy: super::vals::NetcEth3SlvMdcInSelectInputDaisy,
        }
        let proxy = NetcEth3SlvMdcInSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_ETH3_SLV_MDIO_IN_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcEth3SlvMdioInSelectInput(pub u32);
impl NetcEth3SlvMdioInSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcEth3SlvMdioInSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::NetcEth3SlvMdioInSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcEth3SlvMdioInSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for NetcEth3SlvMdioInSelectInput {
    #[inline(always)]
    fn default() -> NetcEth3SlvMdioInSelectInput {
        NetcEth3SlvMdioInSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcEth3SlvMdioInSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcEth3SlvMdioInSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcEth3SlvMdioInSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcEth3SlvMdioInSelectInput {
            daisy: super::vals::NetcEth3SlvMdioInSelectInputDaisy,
        }
        let proxy = NetcEth3SlvMdioInSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_ETH4_COL_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcEth4ColSelectInput(pub u32);
impl NetcEth4ColSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcEth4ColSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::NetcEth4ColSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcEth4ColSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for NetcEth4ColSelectInput {
    #[inline(always)]
    fn default() -> NetcEth4ColSelectInput {
        NetcEth4ColSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcEth4ColSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcEth4ColSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcEth4ColSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcEth4ColSelectInput {
            daisy: super::vals::NetcEth4ColSelectInputDaisy,
        }
        let proxy = NetcEth4ColSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_ETH4_CRS_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcEth4CrsSelectInput(pub u32);
impl NetcEth4CrsSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcEth4CrsSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::NetcEth4CrsSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcEth4CrsSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for NetcEth4CrsSelectInput {
    #[inline(always)]
    fn default() -> NetcEth4CrsSelectInput {
        NetcEth4CrsSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcEth4CrsSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcEth4CrsSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcEth4CrsSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcEth4CrsSelectInput {
            daisy: super::vals::NetcEth4CrsSelectInputDaisy,
        }
        let proxy = NetcEth4CrsSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_ETH4_SLV_MDC_IN_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcEth4SlvMdcInSelectInput(pub u32);
impl NetcEth4SlvMdcInSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcEth4SlvMdcInSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::NetcEth4SlvMdcInSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcEth4SlvMdcInSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for NetcEth4SlvMdcInSelectInput {
    #[inline(always)]
    fn default() -> NetcEth4SlvMdcInSelectInput {
        NetcEth4SlvMdcInSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcEth4SlvMdcInSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcEth4SlvMdcInSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcEth4SlvMdcInSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcEth4SlvMdcInSelectInput {
            daisy: super::vals::NetcEth4SlvMdcInSelectInputDaisy,
        }
        let proxy = NetcEth4SlvMdcInSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_ETH4_SLV_MDIO_IN_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcEth4SlvMdioInSelectInput(pub u32);
impl NetcEth4SlvMdioInSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcEth4SlvMdioInSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::NetcEth4SlvMdioInSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcEth4SlvMdioInSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for NetcEth4SlvMdioInSelectInput {
    #[inline(always)]
    fn default() -> NetcEth4SlvMdioInSelectInput {
        NetcEth4SlvMdioInSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcEth4SlvMdioInSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcEth4SlvMdioInSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcEth4SlvMdioInSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcEth4SlvMdioInSelectInput {
            daisy: super::vals::NetcEth4SlvMdioInSelectInputDaisy,
        }
        let proxy = NetcEth4SlvMdioInSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_PINMUX_IPP_IND_ETH0_RX_CLK_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcPinmuxIppIndEth0RxClkSelectInput(pub u32);
impl NetcPinmuxIppIndEth0RxClkSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcPinmuxIppIndEth0RxClkSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::NetcPinmuxIppIndEth0RxClkSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcPinmuxIppIndEth0RxClkSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for NetcPinmuxIppIndEth0RxClkSelectInput {
    #[inline(always)]
    fn default() -> NetcPinmuxIppIndEth0RxClkSelectInput {
        NetcPinmuxIppIndEth0RxClkSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcPinmuxIppIndEth0RxClkSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcPinmuxIppIndEth0RxClkSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcPinmuxIppIndEth0RxClkSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcPinmuxIppIndEth0RxClkSelectInput {
            daisy: super::vals::NetcPinmuxIppIndEth0RxClkSelectInputDaisy,
        }
        let proxy = NetcPinmuxIppIndEth0RxClkSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_PINMUX_IPP_IND_ETH0_RX_DV_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcPinmuxIppIndEth0RxDvSelectInput(pub u32);
impl NetcPinmuxIppIndEth0RxDvSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcPinmuxIppIndEth0RxDvSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::NetcPinmuxIppIndEth0RxDvSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcPinmuxIppIndEth0RxDvSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for NetcPinmuxIppIndEth0RxDvSelectInput {
    #[inline(always)]
    fn default() -> NetcPinmuxIppIndEth0RxDvSelectInput {
        NetcPinmuxIppIndEth0RxDvSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcPinmuxIppIndEth0RxDvSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcPinmuxIppIndEth0RxDvSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcPinmuxIppIndEth0RxDvSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcPinmuxIppIndEth0RxDvSelectInput {
            daisy: super::vals::NetcPinmuxIppIndEth0RxDvSelectInputDaisy,
        }
        let proxy = NetcPinmuxIppIndEth0RxDvSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_PINMUX_IPP_IND_ETH0_RX_ER_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcPinmuxIppIndEth0RxErSelectInput(pub u32);
impl NetcPinmuxIppIndEth0RxErSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcPinmuxIppIndEth0RxErSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::NetcPinmuxIppIndEth0RxErSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcPinmuxIppIndEth0RxErSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for NetcPinmuxIppIndEth0RxErSelectInput {
    #[inline(always)]
    fn default() -> NetcPinmuxIppIndEth0RxErSelectInput {
        NetcPinmuxIppIndEth0RxErSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcPinmuxIppIndEth0RxErSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcPinmuxIppIndEth0RxErSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcPinmuxIppIndEth0RxErSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcPinmuxIppIndEth0RxErSelectInput {
            daisy: super::vals::NetcPinmuxIppIndEth0RxErSelectInputDaisy,
        }
        let proxy = NetcPinmuxIppIndEth0RxErSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_PINMUX_IPP_IND_ETH0_RXD_SELECT_INPUT_0 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcPinmuxIppIndEth0RxdSelectInput0(pub u32);
impl NetcPinmuxIppIndEth0RxdSelectInput0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcPinmuxIppIndEth0RxdSelectInput0Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::NetcPinmuxIppIndEth0RxdSelectInput0Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcPinmuxIppIndEth0RxdSelectInput0Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for NetcPinmuxIppIndEth0RxdSelectInput0 {
    #[inline(always)]
    fn default() -> NetcPinmuxIppIndEth0RxdSelectInput0 {
        NetcPinmuxIppIndEth0RxdSelectInput0(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcPinmuxIppIndEth0RxdSelectInput0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcPinmuxIppIndEth0RxdSelectInput0")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcPinmuxIppIndEth0RxdSelectInput0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcPinmuxIppIndEth0RxdSelectInput0 {
            daisy: super::vals::NetcPinmuxIppIndEth0RxdSelectInput0Daisy,
        }
        let proxy = NetcPinmuxIppIndEth0RxdSelectInput0 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_PINMUX_IPP_IND_ETH0_RXD_SELECT_INPUT_1 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcPinmuxIppIndEth0RxdSelectInput1(pub u32);
impl NetcPinmuxIppIndEth0RxdSelectInput1 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcPinmuxIppIndEth0RxdSelectInput1Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::NetcPinmuxIppIndEth0RxdSelectInput1Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcPinmuxIppIndEth0RxdSelectInput1Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for NetcPinmuxIppIndEth0RxdSelectInput1 {
    #[inline(always)]
    fn default() -> NetcPinmuxIppIndEth0RxdSelectInput1 {
        NetcPinmuxIppIndEth0RxdSelectInput1(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcPinmuxIppIndEth0RxdSelectInput1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcPinmuxIppIndEth0RxdSelectInput1")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcPinmuxIppIndEth0RxdSelectInput1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcPinmuxIppIndEth0RxdSelectInput1 {
            daisy: super::vals::NetcPinmuxIppIndEth0RxdSelectInput1Daisy,
        }
        let proxy = NetcPinmuxIppIndEth0RxdSelectInput1 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_PINMUX_IPP_IND_ETH0_RXD_SELECT_INPUT_2 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcPinmuxIppIndEth0RxdSelectInput2(pub u32);
impl NetcPinmuxIppIndEth0RxdSelectInput2 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcPinmuxIppIndEth0RxdSelectInput2Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::NetcPinmuxIppIndEth0RxdSelectInput2Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcPinmuxIppIndEth0RxdSelectInput2Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for NetcPinmuxIppIndEth0RxdSelectInput2 {
    #[inline(always)]
    fn default() -> NetcPinmuxIppIndEth0RxdSelectInput2 {
        NetcPinmuxIppIndEth0RxdSelectInput2(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcPinmuxIppIndEth0RxdSelectInput2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcPinmuxIppIndEth0RxdSelectInput2")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcPinmuxIppIndEth0RxdSelectInput2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcPinmuxIppIndEth0RxdSelectInput2 {
            daisy: super::vals::NetcPinmuxIppIndEth0RxdSelectInput2Daisy,
        }
        let proxy = NetcPinmuxIppIndEth0RxdSelectInput2 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_PINMUX_IPP_IND_ETH0_RXD_SELECT_INPUT_3 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcPinmuxIppIndEth0RxdSelectInput3(pub u32);
impl NetcPinmuxIppIndEth0RxdSelectInput3 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcPinmuxIppIndEth0RxdSelectInput3Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::NetcPinmuxIppIndEth0RxdSelectInput3Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcPinmuxIppIndEth0RxdSelectInput3Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for NetcPinmuxIppIndEth0RxdSelectInput3 {
    #[inline(always)]
    fn default() -> NetcPinmuxIppIndEth0RxdSelectInput3 {
        NetcPinmuxIppIndEth0RxdSelectInput3(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcPinmuxIppIndEth0RxdSelectInput3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcPinmuxIppIndEth0RxdSelectInput3")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcPinmuxIppIndEth0RxdSelectInput3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcPinmuxIppIndEth0RxdSelectInput3 {
            daisy: super::vals::NetcPinmuxIppIndEth0RxdSelectInput3Daisy,
        }
        let proxy = NetcPinmuxIppIndEth0RxdSelectInput3 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_PINMUX_IPP_IND_ETH0_TX_CLK_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcPinmuxIppIndEth0TxClkSelectInput(pub u32);
impl NetcPinmuxIppIndEth0TxClkSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcPinmuxIppIndEth0TxClkSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::NetcPinmuxIppIndEth0TxClkSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcPinmuxIppIndEth0TxClkSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for NetcPinmuxIppIndEth0TxClkSelectInput {
    #[inline(always)]
    fn default() -> NetcPinmuxIppIndEth0TxClkSelectInput {
        NetcPinmuxIppIndEth0TxClkSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcPinmuxIppIndEth0TxClkSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcPinmuxIppIndEth0TxClkSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcPinmuxIppIndEth0TxClkSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcPinmuxIppIndEth0TxClkSelectInput {
            daisy: super::vals::NetcPinmuxIppIndEth0TxClkSelectInputDaisy,
        }
        let proxy = NetcPinmuxIppIndEth0TxClkSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_PINMUX_IPP_IND_ETH2_RX_CLK_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcPinmuxIppIndEth2RxClkSelectInput(pub u32);
impl NetcPinmuxIppIndEth2RxClkSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcPinmuxIppIndEth2RxClkSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::NetcPinmuxIppIndEth2RxClkSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcPinmuxIppIndEth2RxClkSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for NetcPinmuxIppIndEth2RxClkSelectInput {
    #[inline(always)]
    fn default() -> NetcPinmuxIppIndEth2RxClkSelectInput {
        NetcPinmuxIppIndEth2RxClkSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcPinmuxIppIndEth2RxClkSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcPinmuxIppIndEth2RxClkSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcPinmuxIppIndEth2RxClkSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcPinmuxIppIndEth2RxClkSelectInput {
            daisy: super::vals::NetcPinmuxIppIndEth2RxClkSelectInputDaisy,
        }
        let proxy = NetcPinmuxIppIndEth2RxClkSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_PINMUX_IPP_IND_ETH2_RX_DV_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcPinmuxIppIndEth2RxDvSelectInput(pub u32);
impl NetcPinmuxIppIndEth2RxDvSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcPinmuxIppIndEth2RxDvSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::NetcPinmuxIppIndEth2RxDvSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcPinmuxIppIndEth2RxDvSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for NetcPinmuxIppIndEth2RxDvSelectInput {
    #[inline(always)]
    fn default() -> NetcPinmuxIppIndEth2RxDvSelectInput {
        NetcPinmuxIppIndEth2RxDvSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcPinmuxIppIndEth2RxDvSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcPinmuxIppIndEth2RxDvSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcPinmuxIppIndEth2RxDvSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcPinmuxIppIndEth2RxDvSelectInput {
            daisy: super::vals::NetcPinmuxIppIndEth2RxDvSelectInputDaisy,
        }
        let proxy = NetcPinmuxIppIndEth2RxDvSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_PINMUX_IPP_IND_ETH2_RX_ER_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcPinmuxIppIndEth2RxErSelectInput(pub u32);
impl NetcPinmuxIppIndEth2RxErSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcPinmuxIppIndEth2RxErSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::NetcPinmuxIppIndEth2RxErSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcPinmuxIppIndEth2RxErSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for NetcPinmuxIppIndEth2RxErSelectInput {
    #[inline(always)]
    fn default() -> NetcPinmuxIppIndEth2RxErSelectInput {
        NetcPinmuxIppIndEth2RxErSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcPinmuxIppIndEth2RxErSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcPinmuxIppIndEth2RxErSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcPinmuxIppIndEth2RxErSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcPinmuxIppIndEth2RxErSelectInput {
            daisy: super::vals::NetcPinmuxIppIndEth2RxErSelectInputDaisy,
        }
        let proxy = NetcPinmuxIppIndEth2RxErSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_PINMUX_IPP_IND_ETH2_RXD_SELECT_INPUT_0 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcPinmuxIppIndEth2RxdSelectInput0(pub u32);
impl NetcPinmuxIppIndEth2RxdSelectInput0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcPinmuxIppIndEth2RxdSelectInput0Daisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::NetcPinmuxIppIndEth2RxdSelectInput0Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcPinmuxIppIndEth2RxdSelectInput0Daisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for NetcPinmuxIppIndEth2RxdSelectInput0 {
    #[inline(always)]
    fn default() -> NetcPinmuxIppIndEth2RxdSelectInput0 {
        NetcPinmuxIppIndEth2RxdSelectInput0(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcPinmuxIppIndEth2RxdSelectInput0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcPinmuxIppIndEth2RxdSelectInput0")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcPinmuxIppIndEth2RxdSelectInput0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcPinmuxIppIndEth2RxdSelectInput0 {
            daisy: super::vals::NetcPinmuxIppIndEth2RxdSelectInput0Daisy,
        }
        let proxy = NetcPinmuxIppIndEth2RxdSelectInput0 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_PINMUX_IPP_IND_ETH2_RXD_SELECT_INPUT_1 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcPinmuxIppIndEth2RxdSelectInput1(pub u32);
impl NetcPinmuxIppIndEth2RxdSelectInput1 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcPinmuxIppIndEth2RxdSelectInput1Daisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::NetcPinmuxIppIndEth2RxdSelectInput1Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcPinmuxIppIndEth2RxdSelectInput1Daisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for NetcPinmuxIppIndEth2RxdSelectInput1 {
    #[inline(always)]
    fn default() -> NetcPinmuxIppIndEth2RxdSelectInput1 {
        NetcPinmuxIppIndEth2RxdSelectInput1(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcPinmuxIppIndEth2RxdSelectInput1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcPinmuxIppIndEth2RxdSelectInput1")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcPinmuxIppIndEth2RxdSelectInput1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcPinmuxIppIndEth2RxdSelectInput1 {
            daisy: super::vals::NetcPinmuxIppIndEth2RxdSelectInput1Daisy,
        }
        let proxy = NetcPinmuxIppIndEth2RxdSelectInput1 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_PINMUX_IPP_IND_ETH2_RXD_SELECT_INPUT_2 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcPinmuxIppIndEth2RxdSelectInput2(pub u32);
impl NetcPinmuxIppIndEth2RxdSelectInput2 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcPinmuxIppIndEth2RxdSelectInput2Daisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::NetcPinmuxIppIndEth2RxdSelectInput2Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcPinmuxIppIndEth2RxdSelectInput2Daisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for NetcPinmuxIppIndEth2RxdSelectInput2 {
    #[inline(always)]
    fn default() -> NetcPinmuxIppIndEth2RxdSelectInput2 {
        NetcPinmuxIppIndEth2RxdSelectInput2(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcPinmuxIppIndEth2RxdSelectInput2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcPinmuxIppIndEth2RxdSelectInput2")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcPinmuxIppIndEth2RxdSelectInput2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcPinmuxIppIndEth2RxdSelectInput2 {
            daisy: super::vals::NetcPinmuxIppIndEth2RxdSelectInput2Daisy,
        }
        let proxy = NetcPinmuxIppIndEth2RxdSelectInput2 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_PINMUX_IPP_IND_ETH2_RXD_SELECT_INPUT_3 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcPinmuxIppIndEth2RxdSelectInput3(pub u32);
impl NetcPinmuxIppIndEth2RxdSelectInput3 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcPinmuxIppIndEth2RxdSelectInput3Daisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::NetcPinmuxIppIndEth2RxdSelectInput3Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcPinmuxIppIndEth2RxdSelectInput3Daisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for NetcPinmuxIppIndEth2RxdSelectInput3 {
    #[inline(always)]
    fn default() -> NetcPinmuxIppIndEth2RxdSelectInput3 {
        NetcPinmuxIppIndEth2RxdSelectInput3(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcPinmuxIppIndEth2RxdSelectInput3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcPinmuxIppIndEth2RxdSelectInput3")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcPinmuxIppIndEth2RxdSelectInput3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcPinmuxIppIndEth2RxdSelectInput3 {
            daisy: super::vals::NetcPinmuxIppIndEth2RxdSelectInput3Daisy,
        }
        let proxy = NetcPinmuxIppIndEth2RxdSelectInput3 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_PINMUX_IPP_IND_ETH2_TX_CLK_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcPinmuxIppIndEth2TxClkSelectInput(pub u32);
impl NetcPinmuxIppIndEth2TxClkSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcPinmuxIppIndEth2TxClkSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::NetcPinmuxIppIndEth2TxClkSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcPinmuxIppIndEth2TxClkSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for NetcPinmuxIppIndEth2TxClkSelectInput {
    #[inline(always)]
    fn default() -> NetcPinmuxIppIndEth2TxClkSelectInput {
        NetcPinmuxIppIndEth2TxClkSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcPinmuxIppIndEth2TxClkSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcPinmuxIppIndEth2TxClkSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcPinmuxIppIndEth2TxClkSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcPinmuxIppIndEth2TxClkSelectInput {
            daisy: super::vals::NetcPinmuxIppIndEth2TxClkSelectInputDaisy,
        }
        let proxy = NetcPinmuxIppIndEth2TxClkSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_PINMUX_IPP_IND_ETH3_RX_CLK_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcPinmuxIppIndEth3RxClkSelectInput(pub u32);
impl NetcPinmuxIppIndEth3RxClkSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcPinmuxIppIndEth3RxClkSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::NetcPinmuxIppIndEth3RxClkSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcPinmuxIppIndEth3RxClkSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for NetcPinmuxIppIndEth3RxClkSelectInput {
    #[inline(always)]
    fn default() -> NetcPinmuxIppIndEth3RxClkSelectInput {
        NetcPinmuxIppIndEth3RxClkSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcPinmuxIppIndEth3RxClkSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcPinmuxIppIndEth3RxClkSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcPinmuxIppIndEth3RxClkSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcPinmuxIppIndEth3RxClkSelectInput {
            daisy: super::vals::NetcPinmuxIppIndEth3RxClkSelectInputDaisy,
        }
        let proxy = NetcPinmuxIppIndEth3RxClkSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_PINMUX_IPP_IND_ETH3_RX_DV_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcPinmuxIppIndEth3RxDvSelectInput(pub u32);
impl NetcPinmuxIppIndEth3RxDvSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcPinmuxIppIndEth3RxDvSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::NetcPinmuxIppIndEth3RxDvSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcPinmuxIppIndEth3RxDvSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for NetcPinmuxIppIndEth3RxDvSelectInput {
    #[inline(always)]
    fn default() -> NetcPinmuxIppIndEth3RxDvSelectInput {
        NetcPinmuxIppIndEth3RxDvSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcPinmuxIppIndEth3RxDvSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcPinmuxIppIndEth3RxDvSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcPinmuxIppIndEth3RxDvSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcPinmuxIppIndEth3RxDvSelectInput {
            daisy: super::vals::NetcPinmuxIppIndEth3RxDvSelectInputDaisy,
        }
        let proxy = NetcPinmuxIppIndEth3RxDvSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_PINMUX_IPP_IND_ETH3_RX_ER_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcPinmuxIppIndEth3RxErSelectInput(pub u32);
impl NetcPinmuxIppIndEth3RxErSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcPinmuxIppIndEth3RxErSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::NetcPinmuxIppIndEth3RxErSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcPinmuxIppIndEth3RxErSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for NetcPinmuxIppIndEth3RxErSelectInput {
    #[inline(always)]
    fn default() -> NetcPinmuxIppIndEth3RxErSelectInput {
        NetcPinmuxIppIndEth3RxErSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcPinmuxIppIndEth3RxErSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcPinmuxIppIndEth3RxErSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcPinmuxIppIndEth3RxErSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcPinmuxIppIndEth3RxErSelectInput {
            daisy: super::vals::NetcPinmuxIppIndEth3RxErSelectInputDaisy,
        }
        let proxy = NetcPinmuxIppIndEth3RxErSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_PINMUX_IPP_IND_ETH3_RXD_SELECT_INPUT_0 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcPinmuxIppIndEth3RxdSelectInput0(pub u32);
impl NetcPinmuxIppIndEth3RxdSelectInput0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcPinmuxIppIndEth3RxdSelectInput0Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::NetcPinmuxIppIndEth3RxdSelectInput0Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcPinmuxIppIndEth3RxdSelectInput0Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for NetcPinmuxIppIndEth3RxdSelectInput0 {
    #[inline(always)]
    fn default() -> NetcPinmuxIppIndEth3RxdSelectInput0 {
        NetcPinmuxIppIndEth3RxdSelectInput0(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcPinmuxIppIndEth3RxdSelectInput0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcPinmuxIppIndEth3RxdSelectInput0")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcPinmuxIppIndEth3RxdSelectInput0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcPinmuxIppIndEth3RxdSelectInput0 {
            daisy: super::vals::NetcPinmuxIppIndEth3RxdSelectInput0Daisy,
        }
        let proxy = NetcPinmuxIppIndEth3RxdSelectInput0 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_PINMUX_IPP_IND_ETH3_RXD_SELECT_INPUT_1 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcPinmuxIppIndEth3RxdSelectInput1(pub u32);
impl NetcPinmuxIppIndEth3RxdSelectInput1 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcPinmuxIppIndEth3RxdSelectInput1Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::NetcPinmuxIppIndEth3RxdSelectInput1Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcPinmuxIppIndEth3RxdSelectInput1Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for NetcPinmuxIppIndEth3RxdSelectInput1 {
    #[inline(always)]
    fn default() -> NetcPinmuxIppIndEth3RxdSelectInput1 {
        NetcPinmuxIppIndEth3RxdSelectInput1(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcPinmuxIppIndEth3RxdSelectInput1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcPinmuxIppIndEth3RxdSelectInput1")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcPinmuxIppIndEth3RxdSelectInput1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcPinmuxIppIndEth3RxdSelectInput1 {
            daisy: super::vals::NetcPinmuxIppIndEth3RxdSelectInput1Daisy,
        }
        let proxy = NetcPinmuxIppIndEth3RxdSelectInput1 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_PINMUX_IPP_IND_ETH3_RXD_SELECT_INPUT_2 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcPinmuxIppIndEth3RxdSelectInput2(pub u32);
impl NetcPinmuxIppIndEth3RxdSelectInput2 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcPinmuxIppIndEth3RxdSelectInput2Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::NetcPinmuxIppIndEth3RxdSelectInput2Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcPinmuxIppIndEth3RxdSelectInput2Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for NetcPinmuxIppIndEth3RxdSelectInput2 {
    #[inline(always)]
    fn default() -> NetcPinmuxIppIndEth3RxdSelectInput2 {
        NetcPinmuxIppIndEth3RxdSelectInput2(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcPinmuxIppIndEth3RxdSelectInput2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcPinmuxIppIndEth3RxdSelectInput2")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcPinmuxIppIndEth3RxdSelectInput2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcPinmuxIppIndEth3RxdSelectInput2 {
            daisy: super::vals::NetcPinmuxIppIndEth3RxdSelectInput2Daisy,
        }
        let proxy = NetcPinmuxIppIndEth3RxdSelectInput2 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_PINMUX_IPP_IND_ETH3_RXD_SELECT_INPUT_3 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcPinmuxIppIndEth3RxdSelectInput3(pub u32);
impl NetcPinmuxIppIndEth3RxdSelectInput3 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcPinmuxIppIndEth3RxdSelectInput3Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::NetcPinmuxIppIndEth3RxdSelectInput3Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcPinmuxIppIndEth3RxdSelectInput3Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for NetcPinmuxIppIndEth3RxdSelectInput3 {
    #[inline(always)]
    fn default() -> NetcPinmuxIppIndEth3RxdSelectInput3 {
        NetcPinmuxIppIndEth3RxdSelectInput3(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcPinmuxIppIndEth3RxdSelectInput3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcPinmuxIppIndEth3RxdSelectInput3")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcPinmuxIppIndEth3RxdSelectInput3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcPinmuxIppIndEth3RxdSelectInput3 {
            daisy: super::vals::NetcPinmuxIppIndEth3RxdSelectInput3Daisy,
        }
        let proxy = NetcPinmuxIppIndEth3RxdSelectInput3 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_PINMUX_IPP_IND_ETH3_TX_CLK_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcPinmuxIppIndEth3TxClkSelectInput(pub u32);
impl NetcPinmuxIppIndEth3TxClkSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcPinmuxIppIndEth3TxClkSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::NetcPinmuxIppIndEth3TxClkSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcPinmuxIppIndEth3TxClkSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for NetcPinmuxIppIndEth3TxClkSelectInput {
    #[inline(always)]
    fn default() -> NetcPinmuxIppIndEth3TxClkSelectInput {
        NetcPinmuxIppIndEth3TxClkSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcPinmuxIppIndEth3TxClkSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcPinmuxIppIndEth3TxClkSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcPinmuxIppIndEth3TxClkSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcPinmuxIppIndEth3TxClkSelectInput {
            daisy: super::vals::NetcPinmuxIppIndEth3TxClkSelectInputDaisy,
        }
        let proxy = NetcPinmuxIppIndEth3TxClkSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_PINMUX_IPP_IND_ETH4_RX_CLK_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcPinmuxIppIndEth4RxClkSelectInput(pub u32);
impl NetcPinmuxIppIndEth4RxClkSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcPinmuxIppIndEth4RxClkSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::NetcPinmuxIppIndEth4RxClkSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcPinmuxIppIndEth4RxClkSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for NetcPinmuxIppIndEth4RxClkSelectInput {
    #[inline(always)]
    fn default() -> NetcPinmuxIppIndEth4RxClkSelectInput {
        NetcPinmuxIppIndEth4RxClkSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcPinmuxIppIndEth4RxClkSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcPinmuxIppIndEth4RxClkSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcPinmuxIppIndEth4RxClkSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcPinmuxIppIndEth4RxClkSelectInput {
            daisy: super::vals::NetcPinmuxIppIndEth4RxClkSelectInputDaisy,
        }
        let proxy = NetcPinmuxIppIndEth4RxClkSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_PINMUX_IPP_IND_ETH4_RX_DV_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcPinmuxIppIndEth4RxDvSelectInput(pub u32);
impl NetcPinmuxIppIndEth4RxDvSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcPinmuxIppIndEth4RxDvSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::NetcPinmuxIppIndEth4RxDvSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcPinmuxIppIndEth4RxDvSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for NetcPinmuxIppIndEth4RxDvSelectInput {
    #[inline(always)]
    fn default() -> NetcPinmuxIppIndEth4RxDvSelectInput {
        NetcPinmuxIppIndEth4RxDvSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcPinmuxIppIndEth4RxDvSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcPinmuxIppIndEth4RxDvSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcPinmuxIppIndEth4RxDvSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcPinmuxIppIndEth4RxDvSelectInput {
            daisy: super::vals::NetcPinmuxIppIndEth4RxDvSelectInputDaisy,
        }
        let proxy = NetcPinmuxIppIndEth4RxDvSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_PINMUX_IPP_IND_ETH4_RX_ER_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcPinmuxIppIndEth4RxErSelectInput(pub u32);
impl NetcPinmuxIppIndEth4RxErSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcPinmuxIppIndEth4RxErSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::NetcPinmuxIppIndEth4RxErSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcPinmuxIppIndEth4RxErSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for NetcPinmuxIppIndEth4RxErSelectInput {
    #[inline(always)]
    fn default() -> NetcPinmuxIppIndEth4RxErSelectInput {
        NetcPinmuxIppIndEth4RxErSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcPinmuxIppIndEth4RxErSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcPinmuxIppIndEth4RxErSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcPinmuxIppIndEth4RxErSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcPinmuxIppIndEth4RxErSelectInput {
            daisy: super::vals::NetcPinmuxIppIndEth4RxErSelectInputDaisy,
        }
        let proxy = NetcPinmuxIppIndEth4RxErSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_PINMUX_IPP_IND_ETH4_RXD_SELECT_INPUT_0 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcPinmuxIppIndEth4RxdSelectInput0(pub u32);
impl NetcPinmuxIppIndEth4RxdSelectInput0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcPinmuxIppIndEth4RxdSelectInput0Daisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::NetcPinmuxIppIndEth4RxdSelectInput0Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcPinmuxIppIndEth4RxdSelectInput0Daisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for NetcPinmuxIppIndEth4RxdSelectInput0 {
    #[inline(always)]
    fn default() -> NetcPinmuxIppIndEth4RxdSelectInput0 {
        NetcPinmuxIppIndEth4RxdSelectInput0(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcPinmuxIppIndEth4RxdSelectInput0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcPinmuxIppIndEth4RxdSelectInput0")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcPinmuxIppIndEth4RxdSelectInput0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcPinmuxIppIndEth4RxdSelectInput0 {
            daisy: super::vals::NetcPinmuxIppIndEth4RxdSelectInput0Daisy,
        }
        let proxy = NetcPinmuxIppIndEth4RxdSelectInput0 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_PINMUX_IPP_IND_ETH4_RXD_SELECT_INPUT_1 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcPinmuxIppIndEth4RxdSelectInput1(pub u32);
impl NetcPinmuxIppIndEth4RxdSelectInput1 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcPinmuxIppIndEth4RxdSelectInput1Daisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::NetcPinmuxIppIndEth4RxdSelectInput1Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcPinmuxIppIndEth4RxdSelectInput1Daisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for NetcPinmuxIppIndEth4RxdSelectInput1 {
    #[inline(always)]
    fn default() -> NetcPinmuxIppIndEth4RxdSelectInput1 {
        NetcPinmuxIppIndEth4RxdSelectInput1(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcPinmuxIppIndEth4RxdSelectInput1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcPinmuxIppIndEth4RxdSelectInput1")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcPinmuxIppIndEth4RxdSelectInput1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcPinmuxIppIndEth4RxdSelectInput1 {
            daisy: super::vals::NetcPinmuxIppIndEth4RxdSelectInput1Daisy,
        }
        let proxy = NetcPinmuxIppIndEth4RxdSelectInput1 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_PINMUX_IPP_IND_ETH4_RXD_SELECT_INPUT_2 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcPinmuxIppIndEth4RxdSelectInput2(pub u32);
impl NetcPinmuxIppIndEth4RxdSelectInput2 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcPinmuxIppIndEth4RxdSelectInput2Daisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::NetcPinmuxIppIndEth4RxdSelectInput2Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcPinmuxIppIndEth4RxdSelectInput2Daisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for NetcPinmuxIppIndEth4RxdSelectInput2 {
    #[inline(always)]
    fn default() -> NetcPinmuxIppIndEth4RxdSelectInput2 {
        NetcPinmuxIppIndEth4RxdSelectInput2(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcPinmuxIppIndEth4RxdSelectInput2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcPinmuxIppIndEth4RxdSelectInput2")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcPinmuxIppIndEth4RxdSelectInput2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcPinmuxIppIndEth4RxdSelectInput2 {
            daisy: super::vals::NetcPinmuxIppIndEth4RxdSelectInput2Daisy,
        }
        let proxy = NetcPinmuxIppIndEth4RxdSelectInput2 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_PINMUX_IPP_IND_ETH4_RXD_SELECT_INPUT_3 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcPinmuxIppIndEth4RxdSelectInput3(pub u32);
impl NetcPinmuxIppIndEth4RxdSelectInput3 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcPinmuxIppIndEth4RxdSelectInput3Daisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::NetcPinmuxIppIndEth4RxdSelectInput3Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcPinmuxIppIndEth4RxdSelectInput3Daisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for NetcPinmuxIppIndEth4RxdSelectInput3 {
    #[inline(always)]
    fn default() -> NetcPinmuxIppIndEth4RxdSelectInput3 {
        NetcPinmuxIppIndEth4RxdSelectInput3(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcPinmuxIppIndEth4RxdSelectInput3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcPinmuxIppIndEth4RxdSelectInput3")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcPinmuxIppIndEth4RxdSelectInput3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcPinmuxIppIndEth4RxdSelectInput3 {
            daisy: super::vals::NetcPinmuxIppIndEth4RxdSelectInput3Daisy,
        }
        let proxy = NetcPinmuxIppIndEth4RxdSelectInput3 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_PINMUX_IPP_IND_ETH4_TX_CLK_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcPinmuxIppIndEth4TxClkSelectInput(pub u32);
impl NetcPinmuxIppIndEth4TxClkSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcPinmuxIppIndEth4TxClkSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::NetcPinmuxIppIndEth4TxClkSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcPinmuxIppIndEth4TxClkSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for NetcPinmuxIppIndEth4TxClkSelectInput {
    #[inline(always)]
    fn default() -> NetcPinmuxIppIndEth4TxClkSelectInput {
        NetcPinmuxIppIndEth4TxClkSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcPinmuxIppIndEth4TxClkSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcPinmuxIppIndEth4TxClkSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcPinmuxIppIndEth4TxClkSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcPinmuxIppIndEth4TxClkSelectInput {
            daisy: super::vals::NetcPinmuxIppIndEth4TxClkSelectInputDaisy,
        }
        let proxy = NetcPinmuxIppIndEth4TxClkSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_TMR_TRIG1_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcTmrTrig1SelectInput(pub u32);
impl NetcTmrTrig1SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcTmrTrig1SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::NetcTmrTrig1SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcTmrTrig1SelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for NetcTmrTrig1SelectInput {
    #[inline(always)]
    fn default() -> NetcTmrTrig1SelectInput {
        NetcTmrTrig1SelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcTmrTrig1SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcTmrTrig1SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcTmrTrig1SelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcTmrTrig1SelectInput {
            daisy: super::vals::NetcTmrTrig1SelectInputDaisy,
        }
        let proxy = NetcTmrTrig1SelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC_TMR_TRIG2_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcTmrTrig2SelectInput(pub u32);
impl NetcTmrTrig2SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NetcTmrTrig2SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::NetcTmrTrig2SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NetcTmrTrig2SelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for NetcTmrTrig2SelectInput {
    #[inline(always)]
    fn default() -> NetcTmrTrig2SelectInput {
        NetcTmrTrig2SelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcTmrTrig2SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcTmrTrig2SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcTmrTrig2SelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcTmrTrig2SelectInput {
            daisy: super::vals::NetcTmrTrig2SelectInputDaisy,
        }
        let proxy = NetcTmrTrig2SelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "QTIMER1_TMR0_INPUT_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qtimer1Tmr0InputSelectInput(pub u32);
impl Qtimer1Tmr0InputSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Qtimer1Tmr0InputSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Qtimer1Tmr0InputSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Qtimer1Tmr0InputSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Qtimer1Tmr0InputSelectInput {
    #[inline(always)]
    fn default() -> Qtimer1Tmr0InputSelectInput {
        Qtimer1Tmr0InputSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Qtimer1Tmr0InputSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qtimer1Tmr0InputSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qtimer1Tmr0InputSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Qtimer1Tmr0InputSelectInput {
            daisy: super::vals::Qtimer1Tmr0InputSelectInputDaisy,
        }
        let proxy = Qtimer1Tmr0InputSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "QTIMER1_TMR1_INPUT_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qtimer1Tmr1InputSelectInput(pub u32);
impl Qtimer1Tmr1InputSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Qtimer1Tmr1InputSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Qtimer1Tmr1InputSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Qtimer1Tmr1InputSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Qtimer1Tmr1InputSelectInput {
    #[inline(always)]
    fn default() -> Qtimer1Tmr1InputSelectInput {
        Qtimer1Tmr1InputSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Qtimer1Tmr1InputSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qtimer1Tmr1InputSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qtimer1Tmr1InputSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Qtimer1Tmr1InputSelectInput {
            daisy: super::vals::Qtimer1Tmr1InputSelectInputDaisy,
        }
        let proxy = Qtimer1Tmr1InputSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "QTIMER1_TMR2_INPUT_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qtimer1Tmr2InputSelectInput(pub u32);
impl Qtimer1Tmr2InputSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Qtimer1Tmr2InputSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Qtimer1Tmr2InputSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Qtimer1Tmr2InputSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Qtimer1Tmr2InputSelectInput {
    #[inline(always)]
    fn default() -> Qtimer1Tmr2InputSelectInput {
        Qtimer1Tmr2InputSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Qtimer1Tmr2InputSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qtimer1Tmr2InputSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qtimer1Tmr2InputSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Qtimer1Tmr2InputSelectInput {
            daisy: super::vals::Qtimer1Tmr2InputSelectInputDaisy,
        }
        let proxy = Qtimer1Tmr2InputSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "QTIMER2_TMR0_INPUT_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qtimer2Tmr0InputSelectInput(pub u32);
impl Qtimer2Tmr0InputSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Qtimer2Tmr0InputSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Qtimer2Tmr0InputSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Qtimer2Tmr0InputSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Qtimer2Tmr0InputSelectInput {
    #[inline(always)]
    fn default() -> Qtimer2Tmr0InputSelectInput {
        Qtimer2Tmr0InputSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Qtimer2Tmr0InputSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qtimer2Tmr0InputSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qtimer2Tmr0InputSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Qtimer2Tmr0InputSelectInput {
            daisy: super::vals::Qtimer2Tmr0InputSelectInputDaisy,
        }
        let proxy = Qtimer2Tmr0InputSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "QTIMER2_TMR1_INPUT_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qtimer2Tmr1InputSelectInput(pub u32);
impl Qtimer2Tmr1InputSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Qtimer2Tmr1InputSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Qtimer2Tmr1InputSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Qtimer2Tmr1InputSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Qtimer2Tmr1InputSelectInput {
    #[inline(always)]
    fn default() -> Qtimer2Tmr1InputSelectInput {
        Qtimer2Tmr1InputSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Qtimer2Tmr1InputSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qtimer2Tmr1InputSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qtimer2Tmr1InputSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Qtimer2Tmr1InputSelectInput {
            daisy: super::vals::Qtimer2Tmr1InputSelectInputDaisy,
        }
        let proxy = Qtimer2Tmr1InputSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "QTIMER2_TMR2_INPUT_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qtimer2Tmr2InputSelectInput(pub u32);
impl Qtimer2Tmr2InputSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Qtimer2Tmr2InputSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Qtimer2Tmr2InputSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Qtimer2Tmr2InputSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Qtimer2Tmr2InputSelectInput {
    #[inline(always)]
    fn default() -> Qtimer2Tmr2InputSelectInput {
        Qtimer2Tmr2InputSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Qtimer2Tmr2InputSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qtimer2Tmr2InputSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qtimer2Tmr2InputSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Qtimer2Tmr2InputSelectInput {
            daisy: super::vals::Qtimer2Tmr2InputSelectInputDaisy,
        }
        let proxy = Qtimer2Tmr2InputSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "QTIMER3_TMR0_INPUT_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qtimer3Tmr0InputSelectInput(pub u32);
impl Qtimer3Tmr0InputSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Qtimer3Tmr0InputSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Qtimer3Tmr0InputSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Qtimer3Tmr0InputSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Qtimer3Tmr0InputSelectInput {
    #[inline(always)]
    fn default() -> Qtimer3Tmr0InputSelectInput {
        Qtimer3Tmr0InputSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Qtimer3Tmr0InputSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qtimer3Tmr0InputSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qtimer3Tmr0InputSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Qtimer3Tmr0InputSelectInput {
            daisy: super::vals::Qtimer3Tmr0InputSelectInputDaisy,
        }
        let proxy = Qtimer3Tmr0InputSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "QTIMER3_TMR1_INPUT_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qtimer3Tmr1InputSelectInput(pub u32);
impl Qtimer3Tmr1InputSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Qtimer3Tmr1InputSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Qtimer3Tmr1InputSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Qtimer3Tmr1InputSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Qtimer3Tmr1InputSelectInput {
    #[inline(always)]
    fn default() -> Qtimer3Tmr1InputSelectInput {
        Qtimer3Tmr1InputSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Qtimer3Tmr1InputSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qtimer3Tmr1InputSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qtimer3Tmr1InputSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Qtimer3Tmr1InputSelectInput {
            daisy: super::vals::Qtimer3Tmr1InputSelectInputDaisy,
        }
        let proxy = Qtimer3Tmr1InputSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "QTIMER3_TMR2_INPUT_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qtimer3Tmr2InputSelectInput(pub u32);
impl Qtimer3Tmr2InputSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Qtimer3Tmr2InputSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Qtimer3Tmr2InputSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Qtimer3Tmr2InputSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Qtimer3Tmr2InputSelectInput {
    #[inline(always)]
    fn default() -> Qtimer3Tmr2InputSelectInput {
        Qtimer3Tmr2InputSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Qtimer3Tmr2InputSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qtimer3Tmr2InputSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qtimer3Tmr2InputSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Qtimer3Tmr2InputSelectInput {
            daisy: super::vals::Qtimer3Tmr2InputSelectInputDaisy,
        }
        let proxy = Qtimer3Tmr2InputSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "QTIMER4_TMR0_INPUT_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qtimer4Tmr0InputSelectInput(pub u32);
impl Qtimer4Tmr0InputSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Qtimer4Tmr0InputSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Qtimer4Tmr0InputSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Qtimer4Tmr0InputSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Qtimer4Tmr0InputSelectInput {
    #[inline(always)]
    fn default() -> Qtimer4Tmr0InputSelectInput {
        Qtimer4Tmr0InputSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Qtimer4Tmr0InputSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qtimer4Tmr0InputSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qtimer4Tmr0InputSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Qtimer4Tmr0InputSelectInput {
            daisy: super::vals::Qtimer4Tmr0InputSelectInputDaisy,
        }
        let proxy = Qtimer4Tmr0InputSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "QTIMER4_TMR1_INPUT_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qtimer4Tmr1InputSelectInput(pub u32);
impl Qtimer4Tmr1InputSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Qtimer4Tmr1InputSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Qtimer4Tmr1InputSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Qtimer4Tmr1InputSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Qtimer4Tmr1InputSelectInput {
    #[inline(always)]
    fn default() -> Qtimer4Tmr1InputSelectInput {
        Qtimer4Tmr1InputSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Qtimer4Tmr1InputSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qtimer4Tmr1InputSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qtimer4Tmr1InputSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Qtimer4Tmr1InputSelectInput {
            daisy: super::vals::Qtimer4Tmr1InputSelectInputDaisy,
        }
        let proxy = Qtimer4Tmr1InputSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "QTIMER4_TMR2_INPUT_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qtimer4Tmr2InputSelectInput(pub u32);
impl Qtimer4Tmr2InputSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Qtimer4Tmr2InputSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Qtimer4Tmr2InputSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Qtimer4Tmr2InputSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Qtimer4Tmr2InputSelectInput {
    #[inline(always)]
    fn default() -> Qtimer4Tmr2InputSelectInput {
        Qtimer4Tmr2InputSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Qtimer4Tmr2InputSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qtimer4Tmr2InputSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qtimer4Tmr2InputSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Qtimer4Tmr2InputSelectInput {
            daisy: super::vals::Qtimer4Tmr2InputSelectInputDaisy,
        }
        let proxy = Qtimer4Tmr2InputSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "QTIMER5_TMR0_INPUT_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qtimer5Tmr0InputSelectInput(pub u32);
impl Qtimer5Tmr0InputSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Qtimer5Tmr0InputSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Qtimer5Tmr0InputSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Qtimer5Tmr0InputSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Qtimer5Tmr0InputSelectInput {
    #[inline(always)]
    fn default() -> Qtimer5Tmr0InputSelectInput {
        Qtimer5Tmr0InputSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Qtimer5Tmr0InputSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qtimer5Tmr0InputSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qtimer5Tmr0InputSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Qtimer5Tmr0InputSelectInput {
            daisy: super::vals::Qtimer5Tmr0InputSelectInputDaisy,
        }
        let proxy = Qtimer5Tmr0InputSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "QTIMER5_TMR1_INPUT_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qtimer5Tmr1InputSelectInput(pub u32);
impl Qtimer5Tmr1InputSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Qtimer5Tmr1InputSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Qtimer5Tmr1InputSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Qtimer5Tmr1InputSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Qtimer5Tmr1InputSelectInput {
    #[inline(always)]
    fn default() -> Qtimer5Tmr1InputSelectInput {
        Qtimer5Tmr1InputSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Qtimer5Tmr1InputSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qtimer5Tmr1InputSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qtimer5Tmr1InputSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Qtimer5Tmr1InputSelectInput {
            daisy: super::vals::Qtimer5Tmr1InputSelectInputDaisy,
        }
        let proxy = Qtimer5Tmr1InputSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "QTIMER5_TMR2_INPUT_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qtimer5Tmr2InputSelectInput(pub u32);
impl Qtimer5Tmr2InputSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Qtimer5Tmr2InputSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Qtimer5Tmr2InputSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Qtimer5Tmr2InputSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Qtimer5Tmr2InputSelectInput {
    #[inline(always)]
    fn default() -> Qtimer5Tmr2InputSelectInput {
        Qtimer5Tmr2InputSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Qtimer5Tmr2InputSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qtimer5Tmr2InputSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qtimer5Tmr2InputSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Qtimer5Tmr2InputSelectInput {
            daisy: super::vals::Qtimer5Tmr2InputSelectInputDaisy,
        }
        let proxy = Qtimer5Tmr2InputSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "QTIMER6_TMR0_INPUT_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qtimer6Tmr0InputSelectInput(pub u32);
impl Qtimer6Tmr0InputSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Qtimer6Tmr0InputSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Qtimer6Tmr0InputSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Qtimer6Tmr0InputSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Qtimer6Tmr0InputSelectInput {
    #[inline(always)]
    fn default() -> Qtimer6Tmr0InputSelectInput {
        Qtimer6Tmr0InputSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Qtimer6Tmr0InputSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qtimer6Tmr0InputSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qtimer6Tmr0InputSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Qtimer6Tmr0InputSelectInput {
            daisy: super::vals::Qtimer6Tmr0InputSelectInputDaisy,
        }
        let proxy = Qtimer6Tmr0InputSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "QTIMER6_TMR1_INPUT_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qtimer6Tmr1InputSelectInput(pub u32);
impl Qtimer6Tmr1InputSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Qtimer6Tmr1InputSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Qtimer6Tmr1InputSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Qtimer6Tmr1InputSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Qtimer6Tmr1InputSelectInput {
    #[inline(always)]
    fn default() -> Qtimer6Tmr1InputSelectInput {
        Qtimer6Tmr1InputSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Qtimer6Tmr1InputSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qtimer6Tmr1InputSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qtimer6Tmr1InputSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Qtimer6Tmr1InputSelectInput {
            daisy: super::vals::Qtimer6Tmr1InputSelectInputDaisy,
        }
        let proxy = Qtimer6Tmr1InputSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "QTIMER6_TMR2_INPUT_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qtimer6Tmr2InputSelectInput(pub u32);
impl Qtimer6Tmr2InputSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Qtimer6Tmr2InputSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Qtimer6Tmr2InputSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Qtimer6Tmr2InputSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Qtimer6Tmr2InputSelectInput {
    #[inline(always)]
    fn default() -> Qtimer6Tmr2InputSelectInput {
        Qtimer6Tmr2InputSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Qtimer6Tmr2InputSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qtimer6Tmr2InputSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qtimer6Tmr2InputSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Qtimer6Tmr2InputSelectInput {
            daisy: super::vals::Qtimer6Tmr2InputSelectInputDaisy,
        }
        let proxy = Qtimer6Tmr2InputSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "QTIMER7_TMR0_INPUT_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qtimer7Tmr0InputSelectInput(pub u32);
impl Qtimer7Tmr0InputSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Qtimer7Tmr0InputSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Qtimer7Tmr0InputSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Qtimer7Tmr0InputSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Qtimer7Tmr0InputSelectInput {
    #[inline(always)]
    fn default() -> Qtimer7Tmr0InputSelectInput {
        Qtimer7Tmr0InputSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Qtimer7Tmr0InputSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qtimer7Tmr0InputSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qtimer7Tmr0InputSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Qtimer7Tmr0InputSelectInput {
            daisy: super::vals::Qtimer7Tmr0InputSelectInputDaisy,
        }
        let proxy = Qtimer7Tmr0InputSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "QTIMER7_TMR1_INPUT_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qtimer7Tmr1InputSelectInput(pub u32);
impl Qtimer7Tmr1InputSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Qtimer7Tmr1InputSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Qtimer7Tmr1InputSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Qtimer7Tmr1InputSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Qtimer7Tmr1InputSelectInput {
    #[inline(always)]
    fn default() -> Qtimer7Tmr1InputSelectInput {
        Qtimer7Tmr1InputSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Qtimer7Tmr1InputSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qtimer7Tmr1InputSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qtimer7Tmr1InputSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Qtimer7Tmr1InputSelectInput {
            daisy: super::vals::Qtimer7Tmr1InputSelectInputDaisy,
        }
        let proxy = Qtimer7Tmr1InputSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "QTIMER8_TMR0_INPUT_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qtimer8Tmr0InputSelectInput(pub u32);
impl Qtimer8Tmr0InputSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Qtimer8Tmr0InputSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Qtimer8Tmr0InputSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Qtimer8Tmr0InputSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Qtimer8Tmr0InputSelectInput {
    #[inline(always)]
    fn default() -> Qtimer8Tmr0InputSelectInput {
        Qtimer8Tmr0InputSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Qtimer8Tmr0InputSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qtimer8Tmr0InputSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qtimer8Tmr0InputSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Qtimer8Tmr0InputSelectInput {
            daisy: super::vals::Qtimer8Tmr0InputSelectInputDaisy,
        }
        let proxy = Qtimer8Tmr0InputSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "QTIMER8_TMR1_INPUT_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qtimer8Tmr1InputSelectInput(pub u32);
impl Qtimer8Tmr1InputSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Qtimer8Tmr1InputSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Qtimer8Tmr1InputSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Qtimer8Tmr1InputSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Qtimer8Tmr1InputSelectInput {
    #[inline(always)]
    fn default() -> Qtimer8Tmr1InputSelectInput {
        Qtimer8Tmr1InputSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Qtimer8Tmr1InputSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qtimer8Tmr1InputSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qtimer8Tmr1InputSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Qtimer8Tmr1InputSelectInput {
            daisy: super::vals::Qtimer8Tmr1InputSelectInputDaisy,
        }
        let proxy = Qtimer8Tmr1InputSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SAI4_IPG_CLK_SAI_MCLK_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai4IpgClkSaiMclkSelectInput(pub u32);
impl Sai4IpgClkSaiMclkSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sai4IpgClkSaiMclkSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sai4IpgClkSaiMclkSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sai4IpgClkSaiMclkSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Sai4IpgClkSaiMclkSelectInput {
    #[inline(always)]
    fn default() -> Sai4IpgClkSaiMclkSelectInput {
        Sai4IpgClkSaiMclkSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Sai4IpgClkSaiMclkSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai4IpgClkSaiMclkSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai4IpgClkSaiMclkSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sai4IpgClkSaiMclkSelectInput {
            daisy: super::vals::Sai4IpgClkSaiMclkSelectInputDaisy,
        }
        let proxy = Sai4IpgClkSaiMclkSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SAI4_IPP_IND_SAI_RXBCLK_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai4IppIndSaiRxbclkSelectInput(pub u32);
impl Sai4IppIndSaiRxbclkSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sai4IppIndSaiRxbclkSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sai4IppIndSaiRxbclkSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sai4IppIndSaiRxbclkSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Sai4IppIndSaiRxbclkSelectInput {
    #[inline(always)]
    fn default() -> Sai4IppIndSaiRxbclkSelectInput {
        Sai4IppIndSaiRxbclkSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Sai4IppIndSaiRxbclkSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai4IppIndSaiRxbclkSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai4IppIndSaiRxbclkSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sai4IppIndSaiRxbclkSelectInput {
            daisy: super::vals::Sai4IppIndSaiRxbclkSelectInputDaisy,
        }
        let proxy = Sai4IppIndSaiRxbclkSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SAI4_IPP_IND_SAI_RXDATA_SELECT_INPUT_0 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai4IppIndSaiRxdataSelectInput0(pub u32);
impl Sai4IppIndSaiRxdataSelectInput0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sai4IppIndSaiRxdataSelectInput0Daisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sai4IppIndSaiRxdataSelectInput0Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sai4IppIndSaiRxdataSelectInput0Daisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Sai4IppIndSaiRxdataSelectInput0 {
    #[inline(always)]
    fn default() -> Sai4IppIndSaiRxdataSelectInput0 {
        Sai4IppIndSaiRxdataSelectInput0(0u64 as u32)
    }
}
impl core::fmt::Debug for Sai4IppIndSaiRxdataSelectInput0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai4IppIndSaiRxdataSelectInput0")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai4IppIndSaiRxdataSelectInput0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sai4IppIndSaiRxdataSelectInput0 {
            daisy: super::vals::Sai4IppIndSaiRxdataSelectInput0Daisy,
        }
        let proxy = Sai4IppIndSaiRxdataSelectInput0 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SAI4_IPP_IND_SAI_RXDATA_SELECT_INPUT_1 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai4IppIndSaiRxdataSelectInput1(pub u32);
impl Sai4IppIndSaiRxdataSelectInput1 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sai4IppIndSaiRxdataSelectInput1Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sai4IppIndSaiRxdataSelectInput1Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sai4IppIndSaiRxdataSelectInput1Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Sai4IppIndSaiRxdataSelectInput1 {
    #[inline(always)]
    fn default() -> Sai4IppIndSaiRxdataSelectInput1 {
        Sai4IppIndSaiRxdataSelectInput1(0u64 as u32)
    }
}
impl core::fmt::Debug for Sai4IppIndSaiRxdataSelectInput1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai4IppIndSaiRxdataSelectInput1")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai4IppIndSaiRxdataSelectInput1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sai4IppIndSaiRxdataSelectInput1 {
            daisy: super::vals::Sai4IppIndSaiRxdataSelectInput1Daisy,
        }
        let proxy = Sai4IppIndSaiRxdataSelectInput1 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SAI4_IPP_IND_SAI_RXSYNC_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai4IppIndSaiRxsyncSelectInput(pub u32);
impl Sai4IppIndSaiRxsyncSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sai4IppIndSaiRxsyncSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sai4IppIndSaiRxsyncSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sai4IppIndSaiRxsyncSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Sai4IppIndSaiRxsyncSelectInput {
    #[inline(always)]
    fn default() -> Sai4IppIndSaiRxsyncSelectInput {
        Sai4IppIndSaiRxsyncSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Sai4IppIndSaiRxsyncSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai4IppIndSaiRxsyncSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai4IppIndSaiRxsyncSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sai4IppIndSaiRxsyncSelectInput {
            daisy: super::vals::Sai4IppIndSaiRxsyncSelectInputDaisy,
        }
        let proxy = Sai4IppIndSaiRxsyncSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SAI4_IPP_IND_SAI_TXBCLK_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai4IppIndSaiTxbclkSelectInput(pub u32);
impl Sai4IppIndSaiTxbclkSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sai4IppIndSaiTxbclkSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sai4IppIndSaiTxbclkSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sai4IppIndSaiTxbclkSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Sai4IppIndSaiTxbclkSelectInput {
    #[inline(always)]
    fn default() -> Sai4IppIndSaiTxbclkSelectInput {
        Sai4IppIndSaiTxbclkSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Sai4IppIndSaiTxbclkSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai4IppIndSaiTxbclkSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai4IppIndSaiTxbclkSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sai4IppIndSaiTxbclkSelectInput {
            daisy: super::vals::Sai4IppIndSaiTxbclkSelectInputDaisy,
        }
        let proxy = Sai4IppIndSaiTxbclkSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SAI4_IPP_IND_SAI_TXSYNC_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai4IppIndSaiTxsyncSelectInput(pub u32);
impl Sai4IppIndSaiTxsyncSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sai4IppIndSaiTxsyncSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sai4IppIndSaiTxsyncSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sai4IppIndSaiTxsyncSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Sai4IppIndSaiTxsyncSelectInput {
    #[inline(always)]
    fn default() -> Sai4IppIndSaiTxsyncSelectInput {
        Sai4IppIndSaiTxsyncSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Sai4IppIndSaiTxsyncSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai4IppIndSaiTxsyncSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai4IppIndSaiTxsyncSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sai4IppIndSaiTxsyncSelectInput {
            daisy: super::vals::Sai4IppIndSaiTxsyncSelectInputDaisy,
        }
        let proxy = Sai4IppIndSaiTxsyncSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SINC1_IPP_IND_EMBIT_SELECT_INPUT_0 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sinc1IppIndEmbitSelectInput0(pub u32);
impl Sinc1IppIndEmbitSelectInput0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sinc1IppIndEmbitSelectInput0Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sinc1IppIndEmbitSelectInput0Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sinc1IppIndEmbitSelectInput0Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Sinc1IppIndEmbitSelectInput0 {
    #[inline(always)]
    fn default() -> Sinc1IppIndEmbitSelectInput0 {
        Sinc1IppIndEmbitSelectInput0(0u64 as u32)
    }
}
impl core::fmt::Debug for Sinc1IppIndEmbitSelectInput0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sinc1IppIndEmbitSelectInput0")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sinc1IppIndEmbitSelectInput0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sinc1IppIndEmbitSelectInput0 {
            daisy: super::vals::Sinc1IppIndEmbitSelectInput0Daisy,
        }
        let proxy = Sinc1IppIndEmbitSelectInput0 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SINC1_IPP_IND_EMBIT_SELECT_INPUT_1 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sinc1IppIndEmbitSelectInput1(pub u32);
impl Sinc1IppIndEmbitSelectInput1 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sinc1IppIndEmbitSelectInput1Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sinc1IppIndEmbitSelectInput1Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sinc1IppIndEmbitSelectInput1Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Sinc1IppIndEmbitSelectInput1 {
    #[inline(always)]
    fn default() -> Sinc1IppIndEmbitSelectInput1 {
        Sinc1IppIndEmbitSelectInput1(0u64 as u32)
    }
}
impl core::fmt::Debug for Sinc1IppIndEmbitSelectInput1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sinc1IppIndEmbitSelectInput1")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sinc1IppIndEmbitSelectInput1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sinc1IppIndEmbitSelectInput1 {
            daisy: super::vals::Sinc1IppIndEmbitSelectInput1Daisy,
        }
        let proxy = Sinc1IppIndEmbitSelectInput1 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SINC1_IPP_IND_EMBIT_SELECT_INPUT_2 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sinc1IppIndEmbitSelectInput2(pub u32);
impl Sinc1IppIndEmbitSelectInput2 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sinc1IppIndEmbitSelectInput2Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sinc1IppIndEmbitSelectInput2Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sinc1IppIndEmbitSelectInput2Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Sinc1IppIndEmbitSelectInput2 {
    #[inline(always)]
    fn default() -> Sinc1IppIndEmbitSelectInput2 {
        Sinc1IppIndEmbitSelectInput2(0u64 as u32)
    }
}
impl core::fmt::Debug for Sinc1IppIndEmbitSelectInput2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sinc1IppIndEmbitSelectInput2")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sinc1IppIndEmbitSelectInput2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sinc1IppIndEmbitSelectInput2 {
            daisy: super::vals::Sinc1IppIndEmbitSelectInput2Daisy,
        }
        let proxy = Sinc1IppIndEmbitSelectInput2 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SINC1_IPP_IND_EMBIT_SELECT_INPUT_3 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sinc1IppIndEmbitSelectInput3(pub u32);
impl Sinc1IppIndEmbitSelectInput3 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sinc1IppIndEmbitSelectInput3Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sinc1IppIndEmbitSelectInput3Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sinc1IppIndEmbitSelectInput3Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Sinc1IppIndEmbitSelectInput3 {
    #[inline(always)]
    fn default() -> Sinc1IppIndEmbitSelectInput3 {
        Sinc1IppIndEmbitSelectInput3(0u64 as u32)
    }
}
impl core::fmt::Debug for Sinc1IppIndEmbitSelectInput3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sinc1IppIndEmbitSelectInput3")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sinc1IppIndEmbitSelectInput3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sinc1IppIndEmbitSelectInput3 {
            daisy: super::vals::Sinc1IppIndEmbitSelectInput3Daisy,
        }
        let proxy = Sinc1IppIndEmbitSelectInput3 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SINC1_IPP_IND_EMCLK_SELECT_INPUT_0 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sinc1IppIndEmclkSelectInput0(pub u32);
impl Sinc1IppIndEmclkSelectInput0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sinc1IppIndEmclkSelectInput0Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sinc1IppIndEmclkSelectInput0Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sinc1IppIndEmclkSelectInput0Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Sinc1IppIndEmclkSelectInput0 {
    #[inline(always)]
    fn default() -> Sinc1IppIndEmclkSelectInput0 {
        Sinc1IppIndEmclkSelectInput0(0u64 as u32)
    }
}
impl core::fmt::Debug for Sinc1IppIndEmclkSelectInput0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sinc1IppIndEmclkSelectInput0")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sinc1IppIndEmclkSelectInput0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sinc1IppIndEmclkSelectInput0 {
            daisy: super::vals::Sinc1IppIndEmclkSelectInput0Daisy,
        }
        let proxy = Sinc1IppIndEmclkSelectInput0 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SINC1_IPP_IND_EMCLK_SELECT_INPUT_1 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sinc1IppIndEmclkSelectInput1(pub u32);
impl Sinc1IppIndEmclkSelectInput1 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sinc1IppIndEmclkSelectInput1Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sinc1IppIndEmclkSelectInput1Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sinc1IppIndEmclkSelectInput1Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Sinc1IppIndEmclkSelectInput1 {
    #[inline(always)]
    fn default() -> Sinc1IppIndEmclkSelectInput1 {
        Sinc1IppIndEmclkSelectInput1(0u64 as u32)
    }
}
impl core::fmt::Debug for Sinc1IppIndEmclkSelectInput1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sinc1IppIndEmclkSelectInput1")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sinc1IppIndEmclkSelectInput1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sinc1IppIndEmclkSelectInput1 {
            daisy: super::vals::Sinc1IppIndEmclkSelectInput1Daisy,
        }
        let proxy = Sinc1IppIndEmclkSelectInput1 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SINC1_IPP_IND_EMCLK_SELECT_INPUT_2 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sinc1IppIndEmclkSelectInput2(pub u32);
impl Sinc1IppIndEmclkSelectInput2 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sinc1IppIndEmclkSelectInput2Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sinc1IppIndEmclkSelectInput2Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sinc1IppIndEmclkSelectInput2Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Sinc1IppIndEmclkSelectInput2 {
    #[inline(always)]
    fn default() -> Sinc1IppIndEmclkSelectInput2 {
        Sinc1IppIndEmclkSelectInput2(0u64 as u32)
    }
}
impl core::fmt::Debug for Sinc1IppIndEmclkSelectInput2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sinc1IppIndEmclkSelectInput2")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sinc1IppIndEmclkSelectInput2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sinc1IppIndEmclkSelectInput2 {
            daisy: super::vals::Sinc1IppIndEmclkSelectInput2Daisy,
        }
        let proxy = Sinc1IppIndEmclkSelectInput2 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SINC1_IPP_IND_EMCLK_SELECT_INPUT_3 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sinc1IppIndEmclkSelectInput3(pub u32);
impl Sinc1IppIndEmclkSelectInput3 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sinc1IppIndEmclkSelectInput3Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sinc1IppIndEmclkSelectInput3Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sinc1IppIndEmclkSelectInput3Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Sinc1IppIndEmclkSelectInput3 {
    #[inline(always)]
    fn default() -> Sinc1IppIndEmclkSelectInput3 {
        Sinc1IppIndEmclkSelectInput3(0u64 as u32)
    }
}
impl core::fmt::Debug for Sinc1IppIndEmclkSelectInput3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sinc1IppIndEmclkSelectInput3")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sinc1IppIndEmclkSelectInput3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sinc1IppIndEmclkSelectInput3 {
            daisy: super::vals::Sinc1IppIndEmclkSelectInput3Daisy,
        }
        let proxy = Sinc1IppIndEmclkSelectInput3 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SINC2_IPP_IND_EMBIT_SELECT_INPUT_2 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sinc2IppIndEmbitSelectInput2(pub u32);
impl Sinc2IppIndEmbitSelectInput2 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sinc2IppIndEmbitSelectInput2Daisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sinc2IppIndEmbitSelectInput2Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sinc2IppIndEmbitSelectInput2Daisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Sinc2IppIndEmbitSelectInput2 {
    #[inline(always)]
    fn default() -> Sinc2IppIndEmbitSelectInput2 {
        Sinc2IppIndEmbitSelectInput2(0u64 as u32)
    }
}
impl core::fmt::Debug for Sinc2IppIndEmbitSelectInput2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sinc2IppIndEmbitSelectInput2")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sinc2IppIndEmbitSelectInput2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sinc2IppIndEmbitSelectInput2 {
            daisy: super::vals::Sinc2IppIndEmbitSelectInput2Daisy,
        }
        let proxy = Sinc2IppIndEmbitSelectInput2 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SINC2_IPP_IND_EMBIT_SELECT_INPUT_3 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sinc2IppIndEmbitSelectInput3(pub u32);
impl Sinc2IppIndEmbitSelectInput3 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sinc2IppIndEmbitSelectInput3Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sinc2IppIndEmbitSelectInput3Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sinc2IppIndEmbitSelectInput3Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Sinc2IppIndEmbitSelectInput3 {
    #[inline(always)]
    fn default() -> Sinc2IppIndEmbitSelectInput3 {
        Sinc2IppIndEmbitSelectInput3(0u64 as u32)
    }
}
impl core::fmt::Debug for Sinc2IppIndEmbitSelectInput3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sinc2IppIndEmbitSelectInput3")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sinc2IppIndEmbitSelectInput3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sinc2IppIndEmbitSelectInput3 {
            daisy: super::vals::Sinc2IppIndEmbitSelectInput3Daisy,
        }
        let proxy = Sinc2IppIndEmbitSelectInput3 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SINC2_IPP_IND_EMCLK_SELECT_INPUT_0 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sinc2IppIndEmclkSelectInput0(pub u32);
impl Sinc2IppIndEmclkSelectInput0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sinc2IppIndEmclkSelectInput0Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sinc2IppIndEmclkSelectInput0Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sinc2IppIndEmclkSelectInput0Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Sinc2IppIndEmclkSelectInput0 {
    #[inline(always)]
    fn default() -> Sinc2IppIndEmclkSelectInput0 {
        Sinc2IppIndEmclkSelectInput0(0u64 as u32)
    }
}
impl core::fmt::Debug for Sinc2IppIndEmclkSelectInput0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sinc2IppIndEmclkSelectInput0")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sinc2IppIndEmclkSelectInput0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sinc2IppIndEmclkSelectInput0 {
            daisy: super::vals::Sinc2IppIndEmclkSelectInput0Daisy,
        }
        let proxy = Sinc2IppIndEmclkSelectInput0 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SINC2_IPP_IND_EMCLK_SELECT_INPUT_2 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sinc2IppIndEmclkSelectInput2(pub u32);
impl Sinc2IppIndEmclkSelectInput2 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sinc2IppIndEmclkSelectInput2Daisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sinc2IppIndEmclkSelectInput2Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sinc2IppIndEmclkSelectInput2Daisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Sinc2IppIndEmclkSelectInput2 {
    #[inline(always)]
    fn default() -> Sinc2IppIndEmclkSelectInput2 {
        Sinc2IppIndEmclkSelectInput2(0u64 as u32)
    }
}
impl core::fmt::Debug for Sinc2IppIndEmclkSelectInput2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sinc2IppIndEmclkSelectInput2")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sinc2IppIndEmclkSelectInput2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sinc2IppIndEmclkSelectInput2 {
            daisy: super::vals::Sinc2IppIndEmclkSelectInput2Daisy,
        }
        let proxy = Sinc2IppIndEmclkSelectInput2 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SINC2_IPP_IND_EMCLK_SELECT_INPUT_3 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sinc2IppIndEmclkSelectInput3(pub u32);
impl Sinc2IppIndEmclkSelectInput3 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sinc2IppIndEmclkSelectInput3Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sinc2IppIndEmclkSelectInput3Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sinc2IppIndEmclkSelectInput3Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Sinc2IppIndEmclkSelectInput3 {
    #[inline(always)]
    fn default() -> Sinc2IppIndEmclkSelectInput3 {
        Sinc2IppIndEmclkSelectInput3(0u64 as u32)
    }
}
impl core::fmt::Debug for Sinc2IppIndEmclkSelectInput3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sinc2IppIndEmclkSelectInput3")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sinc2IppIndEmclkSelectInput3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sinc2IppIndEmclkSelectInput3 {
            daisy: super::vals::Sinc2IppIndEmclkSelectInput3Daisy,
        }
        let proxy = Sinc2IppIndEmclkSelectInput3 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SPDIF_SPDIF_IN1_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpdifSpdifIn1SelectInput(pub u32);
impl SpdifSpdifIn1SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::SpdifSpdifIn1SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SpdifSpdifIn1SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::SpdifSpdifIn1SelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for SpdifSpdifIn1SelectInput {
    #[inline(always)]
    fn default() -> SpdifSpdifIn1SelectInput {
        SpdifSpdifIn1SelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for SpdifSpdifIn1SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SpdifSpdifIn1SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SpdifSpdifIn1SelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SpdifSpdifIn1SelectInput {
            daisy: super::vals::SpdifSpdifIn1SelectInputDaisy,
        }
        let proxy = SpdifSpdifIn1SelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_00 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAd00(pub u32);
impl SwMuxCtlPadGpioAd00 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAd00MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAd00MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAd00MuxMode) {
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
impl Default for SwMuxCtlPadGpioAd00 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAd00 {
        SwMuxCtlPadGpioAd00(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAd00 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAd00")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAd00 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAd00 {
            mux_mode: super::vals::SwMuxCtlPadGpioAd00MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAd00 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_01 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAd01(pub u32);
impl SwMuxCtlPadGpioAd01 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAd01MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAd01MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAd01MuxMode) {
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
impl Default for SwMuxCtlPadGpioAd01 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAd01 {
        SwMuxCtlPadGpioAd01(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAd01 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAd01")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAd01 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAd01 {
            mux_mode: super::vals::SwMuxCtlPadGpioAd01MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAd01 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_02 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAd02(pub u32);
impl SwMuxCtlPadGpioAd02 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAd02MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAd02MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAd02MuxMode) {
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
impl Default for SwMuxCtlPadGpioAd02 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAd02 {
        SwMuxCtlPadGpioAd02(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAd02 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAd02")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAd02 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAd02 {
            mux_mode: super::vals::SwMuxCtlPadGpioAd02MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAd02 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_03 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAd03(pub u32);
impl SwMuxCtlPadGpioAd03 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAd03MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAd03MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAd03MuxMode) {
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
impl Default for SwMuxCtlPadGpioAd03 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAd03 {
        SwMuxCtlPadGpioAd03(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAd03 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAd03")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAd03 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAd03 {
            mux_mode: super::vals::SwMuxCtlPadGpioAd03MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAd03 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_04 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAd04(pub u32);
impl SwMuxCtlPadGpioAd04 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAd04MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAd04MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAd04MuxMode) {
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
impl Default for SwMuxCtlPadGpioAd04 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAd04 {
        SwMuxCtlPadGpioAd04(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAd04 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAd04")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAd04 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAd04 {
            mux_mode: super::vals::SwMuxCtlPadGpioAd04MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAd04 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_05 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAd05(pub u32);
impl SwMuxCtlPadGpioAd05 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAd05MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAd05MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAd05MuxMode) {
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
impl Default for SwMuxCtlPadGpioAd05 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAd05 {
        SwMuxCtlPadGpioAd05(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAd05 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAd05")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAd05 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAd05 {
            mux_mode: super::vals::SwMuxCtlPadGpioAd05MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAd05 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_06 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAd06(pub u32);
impl SwMuxCtlPadGpioAd06 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAd06MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAd06MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAd06MuxMode) {
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
impl Default for SwMuxCtlPadGpioAd06 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAd06 {
        SwMuxCtlPadGpioAd06(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAd06 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAd06")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAd06 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAd06 {
            mux_mode: super::vals::SwMuxCtlPadGpioAd06MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAd06 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_07 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAd07(pub u32);
impl SwMuxCtlPadGpioAd07 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAd07MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAd07MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAd07MuxMode) {
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
impl Default for SwMuxCtlPadGpioAd07 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAd07 {
        SwMuxCtlPadGpioAd07(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAd07 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAd07")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAd07 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAd07 {
            mux_mode: super::vals::SwMuxCtlPadGpioAd07MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAd07 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_08 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAd08(pub u32);
impl SwMuxCtlPadGpioAd08 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAd08MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAd08MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAd08MuxMode) {
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
impl Default for SwMuxCtlPadGpioAd08 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAd08 {
        SwMuxCtlPadGpioAd08(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAd08 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAd08")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAd08 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAd08 {
            mux_mode: super::vals::SwMuxCtlPadGpioAd08MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAd08 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_09 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAd09(pub u32);
impl SwMuxCtlPadGpioAd09 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAd09MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAd09MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAd09MuxMode) {
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
impl Default for SwMuxCtlPadGpioAd09 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAd09 {
        SwMuxCtlPadGpioAd09(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAd09 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAd09")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAd09 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAd09 {
            mux_mode: super::vals::SwMuxCtlPadGpioAd09MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAd09 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_10 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAd10(pub u32);
impl SwMuxCtlPadGpioAd10 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAd10MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAd10MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAd10MuxMode) {
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
impl Default for SwMuxCtlPadGpioAd10 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAd10 {
        SwMuxCtlPadGpioAd10(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAd10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAd10")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAd10 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAd10 {
            mux_mode: super::vals::SwMuxCtlPadGpioAd10MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAd10 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_11 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAd11(pub u32);
impl SwMuxCtlPadGpioAd11 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAd11MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAd11MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAd11MuxMode) {
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
impl Default for SwMuxCtlPadGpioAd11 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAd11 {
        SwMuxCtlPadGpioAd11(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAd11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAd11")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAd11 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAd11 {
            mux_mode: super::vals::SwMuxCtlPadGpioAd11MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAd11 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_12 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAd12(pub u32);
impl SwMuxCtlPadGpioAd12 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAd12MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAd12MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAd12MuxMode) {
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
impl Default for SwMuxCtlPadGpioAd12 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAd12 {
        SwMuxCtlPadGpioAd12(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAd12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAd12")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAd12 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAd12 {
            mux_mode: super::vals::SwMuxCtlPadGpioAd12MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAd12 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_13 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAd13(pub u32);
impl SwMuxCtlPadGpioAd13 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAd13MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAd13MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAd13MuxMode) {
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
impl Default for SwMuxCtlPadGpioAd13 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAd13 {
        SwMuxCtlPadGpioAd13(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAd13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAd13")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAd13 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAd13 {
            mux_mode: super::vals::SwMuxCtlPadGpioAd13MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAd13 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_14 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAd14(pub u32);
impl SwMuxCtlPadGpioAd14 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAd14MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAd14MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAd14MuxMode) {
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
impl Default for SwMuxCtlPadGpioAd14 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAd14 {
        SwMuxCtlPadGpioAd14(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAd14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAd14")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAd14 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAd14 {
            mux_mode: super::vals::SwMuxCtlPadGpioAd14MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAd14 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_15 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAd15(pub u32);
impl SwMuxCtlPadGpioAd15 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAd15MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAd15MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAd15MuxMode) {
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
impl Default for SwMuxCtlPadGpioAd15 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAd15 {
        SwMuxCtlPadGpioAd15(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAd15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAd15")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAd15 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAd15 {
            mux_mode: super::vals::SwMuxCtlPadGpioAd15MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAd15 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_16 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAd16(pub u32);
impl SwMuxCtlPadGpioAd16 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAd16MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAd16MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAd16MuxMode) {
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
impl Default for SwMuxCtlPadGpioAd16 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAd16 {
        SwMuxCtlPadGpioAd16(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAd16 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAd16")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAd16 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAd16 {
            mux_mode: super::vals::SwMuxCtlPadGpioAd16MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAd16 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_17 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAd17(pub u32);
impl SwMuxCtlPadGpioAd17 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAd17MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAd17MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAd17MuxMode) {
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
impl Default for SwMuxCtlPadGpioAd17 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAd17 {
        SwMuxCtlPadGpioAd17(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAd17 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAd17")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAd17 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAd17 {
            mux_mode: super::vals::SwMuxCtlPadGpioAd17MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAd17 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_18 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAd18(pub u32);
impl SwMuxCtlPadGpioAd18 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAd18MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAd18MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAd18MuxMode) {
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
impl Default for SwMuxCtlPadGpioAd18 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAd18 {
        SwMuxCtlPadGpioAd18(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAd18 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAd18")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAd18 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAd18 {
            mux_mode: super::vals::SwMuxCtlPadGpioAd18MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAd18 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_19 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAd19(pub u32);
impl SwMuxCtlPadGpioAd19 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAd19MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAd19MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAd19MuxMode) {
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
impl Default for SwMuxCtlPadGpioAd19 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAd19 {
        SwMuxCtlPadGpioAd19(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAd19 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAd19")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAd19 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAd19 {
            mux_mode: super::vals::SwMuxCtlPadGpioAd19MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAd19 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_20 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAd20(pub u32);
impl SwMuxCtlPadGpioAd20 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAd20MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAd20MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAd20MuxMode) {
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
impl Default for SwMuxCtlPadGpioAd20 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAd20 {
        SwMuxCtlPadGpioAd20(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAd20 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAd20")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAd20 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAd20 {
            mux_mode: super::vals::SwMuxCtlPadGpioAd20MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAd20 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_21 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAd21(pub u32);
impl SwMuxCtlPadGpioAd21 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAd21MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAd21MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAd21MuxMode) {
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
impl Default for SwMuxCtlPadGpioAd21 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAd21 {
        SwMuxCtlPadGpioAd21(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAd21 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAd21")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAd21 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAd21 {
            mux_mode: super::vals::SwMuxCtlPadGpioAd21MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAd21 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_22 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAd22(pub u32);
impl SwMuxCtlPadGpioAd22 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAd22MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAd22MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAd22MuxMode) {
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
impl Default for SwMuxCtlPadGpioAd22 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAd22 {
        SwMuxCtlPadGpioAd22(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAd22 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAd22")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAd22 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAd22 {
            mux_mode: super::vals::SwMuxCtlPadGpioAd22MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAd22 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_23 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAd23(pub u32);
impl SwMuxCtlPadGpioAd23 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAd23MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAd23MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAd23MuxMode) {
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
impl Default for SwMuxCtlPadGpioAd23 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAd23 {
        SwMuxCtlPadGpioAd23(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAd23 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAd23")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAd23 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAd23 {
            mux_mode: super::vals::SwMuxCtlPadGpioAd23MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAd23 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_24 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAd24(pub u32);
impl SwMuxCtlPadGpioAd24 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAd24MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAd24MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAd24MuxMode) {
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
impl Default for SwMuxCtlPadGpioAd24 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAd24 {
        SwMuxCtlPadGpioAd24(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAd24 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAd24")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAd24 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAd24 {
            mux_mode: super::vals::SwMuxCtlPadGpioAd24MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAd24 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_25 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAd25(pub u32);
impl SwMuxCtlPadGpioAd25 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAd25MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAd25MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAd25MuxMode) {
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
impl Default for SwMuxCtlPadGpioAd25 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAd25 {
        SwMuxCtlPadGpioAd25(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAd25 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAd25")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAd25 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAd25 {
            mux_mode: super::vals::SwMuxCtlPadGpioAd25MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAd25 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_26 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAd26(pub u32);
impl SwMuxCtlPadGpioAd26 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAd26MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAd26MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAd26MuxMode) {
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
impl Default for SwMuxCtlPadGpioAd26 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAd26 {
        SwMuxCtlPadGpioAd26(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAd26 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAd26")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAd26 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAd26 {
            mux_mode: super::vals::SwMuxCtlPadGpioAd26MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAd26 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_27 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAd27(pub u32);
impl SwMuxCtlPadGpioAd27 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAd27MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAd27MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAd27MuxMode) {
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
impl Default for SwMuxCtlPadGpioAd27 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAd27 {
        SwMuxCtlPadGpioAd27(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAd27 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAd27")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAd27 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAd27 {
            mux_mode: super::vals::SwMuxCtlPadGpioAd27MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAd27 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_28 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAd28(pub u32);
impl SwMuxCtlPadGpioAd28 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAd28MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAd28MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAd28MuxMode) {
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
impl Default for SwMuxCtlPadGpioAd28 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAd28 {
        SwMuxCtlPadGpioAd28(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAd28 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAd28")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAd28 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAd28 {
            mux_mode: super::vals::SwMuxCtlPadGpioAd28MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAd28 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_29 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAd29(pub u32);
impl SwMuxCtlPadGpioAd29 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAd29MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAd29MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAd29MuxMode) {
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
impl Default for SwMuxCtlPadGpioAd29 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAd29 {
        SwMuxCtlPadGpioAd29(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAd29 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAd29")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAd29 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAd29 {
            mux_mode: super::vals::SwMuxCtlPadGpioAd29MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAd29 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_30 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAd30(pub u32);
impl SwMuxCtlPadGpioAd30 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAd30MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAd30MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAd30MuxMode) {
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
impl Default for SwMuxCtlPadGpioAd30 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAd30 {
        SwMuxCtlPadGpioAd30(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAd30 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAd30")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAd30 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAd30 {
            mux_mode: super::vals::SwMuxCtlPadGpioAd30MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAd30 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_31 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAd31(pub u32);
impl SwMuxCtlPadGpioAd31 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAd31MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAd31MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAd31MuxMode) {
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
impl Default for SwMuxCtlPadGpioAd31 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAd31 {
        SwMuxCtlPadGpioAd31(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAd31 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAd31")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAd31 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAd31 {
            mux_mode: super::vals::SwMuxCtlPadGpioAd31MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAd31 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_32 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAd32(pub u32);
impl SwMuxCtlPadGpioAd32 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAd32MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAd32MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAd32MuxMode) {
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
impl Default for SwMuxCtlPadGpioAd32 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAd32 {
        SwMuxCtlPadGpioAd32(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAd32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAd32")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAd32 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAd32 {
            mux_mode: super::vals::SwMuxCtlPadGpioAd32MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAd32 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_33 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAd33(pub u32);
impl SwMuxCtlPadGpioAd33 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAd33MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAd33MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAd33MuxMode) {
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
impl Default for SwMuxCtlPadGpioAd33 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAd33 {
        SwMuxCtlPadGpioAd33(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAd33 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAd33")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAd33 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAd33 {
            mux_mode: super::vals::SwMuxCtlPadGpioAd33MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAd33 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_34 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAd34(pub u32);
impl SwMuxCtlPadGpioAd34 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAd34MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAd34MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAd34MuxMode) {
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
impl Default for SwMuxCtlPadGpioAd34 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAd34 {
        SwMuxCtlPadGpioAd34(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAd34 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAd34")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAd34 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAd34 {
            mux_mode: super::vals::SwMuxCtlPadGpioAd34MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAd34 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_35 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioAd35(pub u32);
impl SwMuxCtlPadGpioAd35 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioAd35MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioAd35MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioAd35MuxMode) {
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
impl Default for SwMuxCtlPadGpioAd35 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioAd35 {
        SwMuxCtlPadGpioAd35(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioAd35 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioAd35")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioAd35 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioAd35 {
            mux_mode: super::vals::SwMuxCtlPadGpioAd35MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioAd35 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_00 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioB100(pub u32);
impl SwMuxCtlPadGpioB100 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioB100MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioB100MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioB100MuxMode) {
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
impl Default for SwMuxCtlPadGpioB100 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioB100 {
        SwMuxCtlPadGpioB100(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioB100 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioB100")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioB100 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioB100 {
            mux_mode: super::vals::SwMuxCtlPadGpioB100MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioB100 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_01 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioB101(pub u32);
impl SwMuxCtlPadGpioB101 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioB101MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioB101MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioB101MuxMode) {
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
impl Default for SwMuxCtlPadGpioB101 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioB101 {
        SwMuxCtlPadGpioB101(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioB101 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioB101")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioB101 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioB101 {
            mux_mode: super::vals::SwMuxCtlPadGpioB101MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioB101 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_02 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioB102(pub u32);
impl SwMuxCtlPadGpioB102 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioB102MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioB102MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioB102MuxMode) {
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
impl Default for SwMuxCtlPadGpioB102 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioB102 {
        SwMuxCtlPadGpioB102(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioB102 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioB102")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioB102 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioB102 {
            mux_mode: super::vals::SwMuxCtlPadGpioB102MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioB102 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_03 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioB103(pub u32);
impl SwMuxCtlPadGpioB103 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioB103MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioB103MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioB103MuxMode) {
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
impl Default for SwMuxCtlPadGpioB103 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioB103 {
        SwMuxCtlPadGpioB103(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioB103 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioB103")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioB103 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioB103 {
            mux_mode: super::vals::SwMuxCtlPadGpioB103MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioB103 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_04 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioB104(pub u32);
impl SwMuxCtlPadGpioB104 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioB104MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioB104MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioB104MuxMode) {
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
impl Default for SwMuxCtlPadGpioB104 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioB104 {
        SwMuxCtlPadGpioB104(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioB104 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioB104")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioB104 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioB104 {
            mux_mode: super::vals::SwMuxCtlPadGpioB104MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioB104 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_05 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioB105(pub u32);
impl SwMuxCtlPadGpioB105 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioB105MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioB105MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioB105MuxMode) {
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
impl Default for SwMuxCtlPadGpioB105 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioB105 {
        SwMuxCtlPadGpioB105(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioB105 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioB105")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioB105 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioB105 {
            mux_mode: super::vals::SwMuxCtlPadGpioB105MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioB105 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_06 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioB106(pub u32);
impl SwMuxCtlPadGpioB106 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioB106MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioB106MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioB106MuxMode) {
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
impl Default for SwMuxCtlPadGpioB106 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioB106 {
        SwMuxCtlPadGpioB106(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioB106 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioB106")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioB106 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioB106 {
            mux_mode: super::vals::SwMuxCtlPadGpioB106MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioB106 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_07 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioB107(pub u32);
impl SwMuxCtlPadGpioB107 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioB107MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioB107MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioB107MuxMode) {
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
impl Default for SwMuxCtlPadGpioB107 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioB107 {
        SwMuxCtlPadGpioB107(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioB107 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioB107")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioB107 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioB107 {
            mux_mode: super::vals::SwMuxCtlPadGpioB107MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioB107 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_08 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioB108(pub u32);
impl SwMuxCtlPadGpioB108 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioB108MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioB108MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioB108MuxMode) {
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
impl Default for SwMuxCtlPadGpioB108 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioB108 {
        SwMuxCtlPadGpioB108(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioB108 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioB108")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioB108 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioB108 {
            mux_mode: super::vals::SwMuxCtlPadGpioB108MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioB108 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_09 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioB109(pub u32);
impl SwMuxCtlPadGpioB109 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioB109MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioB109MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioB109MuxMode) {
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
impl Default for SwMuxCtlPadGpioB109 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioB109 {
        SwMuxCtlPadGpioB109(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioB109 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioB109")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioB109 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioB109 {
            mux_mode: super::vals::SwMuxCtlPadGpioB109MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioB109 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_10 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioB110(pub u32);
impl SwMuxCtlPadGpioB110 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioB110MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioB110MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioB110MuxMode) {
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
impl Default for SwMuxCtlPadGpioB110 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioB110 {
        SwMuxCtlPadGpioB110(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioB110 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioB110")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioB110 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioB110 {
            mux_mode: super::vals::SwMuxCtlPadGpioB110MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioB110 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_11 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioB111(pub u32);
impl SwMuxCtlPadGpioB111 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioB111MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioB111MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioB111MuxMode) {
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
impl Default for SwMuxCtlPadGpioB111 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioB111 {
        SwMuxCtlPadGpioB111(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioB111 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioB111")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioB111 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioB111 {
            mux_mode: super::vals::SwMuxCtlPadGpioB111MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioB111 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_12 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioB112(pub u32);
impl SwMuxCtlPadGpioB112 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioB112MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioB112MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioB112MuxMode) {
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
impl Default for SwMuxCtlPadGpioB112 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioB112 {
        SwMuxCtlPadGpioB112(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioB112 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioB112")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioB112 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioB112 {
            mux_mode: super::vals::SwMuxCtlPadGpioB112MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioB112 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_13 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioB113(pub u32);
impl SwMuxCtlPadGpioB113 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioB113MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioB113MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioB113MuxMode) {
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
impl Default for SwMuxCtlPadGpioB113 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioB113 {
        SwMuxCtlPadGpioB113(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioB113 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioB113")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioB113 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioB113 {
            mux_mode: super::vals::SwMuxCtlPadGpioB113MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioB113 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_B2_00 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioB200(pub u32);
impl SwMuxCtlPadGpioB200 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioB200MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioB200MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioB200MuxMode) {
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
impl Default for SwMuxCtlPadGpioB200 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioB200 {
        SwMuxCtlPadGpioB200(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioB200 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioB200")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioB200 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioB200 {
            mux_mode: super::vals::SwMuxCtlPadGpioB200MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioB200 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_B2_01 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioB201(pub u32);
impl SwMuxCtlPadGpioB201 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioB201MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioB201MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioB201MuxMode) {
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
impl Default for SwMuxCtlPadGpioB201 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioB201 {
        SwMuxCtlPadGpioB201(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioB201 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioB201")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioB201 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioB201 {
            mux_mode: super::vals::SwMuxCtlPadGpioB201MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioB201 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_B2_02 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioB202(pub u32);
impl SwMuxCtlPadGpioB202 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioB202MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioB202MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioB202MuxMode) {
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
impl Default for SwMuxCtlPadGpioB202 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioB202 {
        SwMuxCtlPadGpioB202(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioB202 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioB202")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioB202 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioB202 {
            mux_mode: super::vals::SwMuxCtlPadGpioB202MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioB202 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_B2_03 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioB203(pub u32);
impl SwMuxCtlPadGpioB203 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioB203MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioB203MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioB203MuxMode) {
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
impl Default for SwMuxCtlPadGpioB203 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioB203 {
        SwMuxCtlPadGpioB203(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioB203 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioB203")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioB203 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioB203 {
            mux_mode: super::vals::SwMuxCtlPadGpioB203MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioB203 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_B2_04 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioB204(pub u32);
impl SwMuxCtlPadGpioB204 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioB204MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioB204MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioB204MuxMode) {
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
impl Default for SwMuxCtlPadGpioB204 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioB204 {
        SwMuxCtlPadGpioB204(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioB204 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioB204")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioB204 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioB204 {
            mux_mode: super::vals::SwMuxCtlPadGpioB204MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioB204 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_B2_05 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioB205(pub u32);
impl SwMuxCtlPadGpioB205 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioB205MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioB205MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioB205MuxMode) {
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
impl Default for SwMuxCtlPadGpioB205 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioB205 {
        SwMuxCtlPadGpioB205(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioB205 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioB205")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioB205 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioB205 {
            mux_mode: super::vals::SwMuxCtlPadGpioB205MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioB205 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_B2_06 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioB206(pub u32);
impl SwMuxCtlPadGpioB206 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioB206MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioB206MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioB206MuxMode) {
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
impl Default for SwMuxCtlPadGpioB206 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioB206 {
        SwMuxCtlPadGpioB206(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioB206 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioB206")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioB206 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioB206 {
            mux_mode: super::vals::SwMuxCtlPadGpioB206MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioB206 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_B2_07 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioB207(pub u32);
impl SwMuxCtlPadGpioB207 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioB207MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioB207MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioB207MuxMode) {
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
impl Default for SwMuxCtlPadGpioB207 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioB207 {
        SwMuxCtlPadGpioB207(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioB207 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioB207")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioB207 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioB207 {
            mux_mode: super::vals::SwMuxCtlPadGpioB207MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioB207 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_B2_08 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioB208(pub u32);
impl SwMuxCtlPadGpioB208 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioB208MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioB208MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioB208MuxMode) {
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
impl Default for SwMuxCtlPadGpioB208 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioB208 {
        SwMuxCtlPadGpioB208(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioB208 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioB208")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioB208 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioB208 {
            mux_mode: super::vals::SwMuxCtlPadGpioB208MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioB208 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_B2_09 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioB209(pub u32);
impl SwMuxCtlPadGpioB209 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioB209MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioB209MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioB209MuxMode) {
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
impl Default for SwMuxCtlPadGpioB209 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioB209 {
        SwMuxCtlPadGpioB209(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioB209 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioB209")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioB209 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioB209 {
            mux_mode: super::vals::SwMuxCtlPadGpioB209MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioB209 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_B2_10 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioB210(pub u32);
impl SwMuxCtlPadGpioB210 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioB210MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioB210MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioB210MuxMode) {
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
impl Default for SwMuxCtlPadGpioB210 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioB210 {
        SwMuxCtlPadGpioB210(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioB210 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioB210")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioB210 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioB210 {
            mux_mode: super::vals::SwMuxCtlPadGpioB210MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioB210 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_B2_11 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioB211(pub u32);
impl SwMuxCtlPadGpioB211 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioB211MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioB211MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioB211MuxMode) {
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
impl Default for SwMuxCtlPadGpioB211 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioB211 {
        SwMuxCtlPadGpioB211(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioB211 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioB211")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioB211 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioB211 {
            mux_mode: super::vals::SwMuxCtlPadGpioB211MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioB211 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_B2_12 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioB212(pub u32);
impl SwMuxCtlPadGpioB212 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioB212MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioB212MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioB212MuxMode) {
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
impl Default for SwMuxCtlPadGpioB212 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioB212 {
        SwMuxCtlPadGpioB212(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioB212 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioB212")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioB212 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioB212 {
            mux_mode: super::vals::SwMuxCtlPadGpioB212MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioB212 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_B2_13 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioB213(pub u32);
impl SwMuxCtlPadGpioB213 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioB213MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioB213MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioB213MuxMode) {
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
impl Default for SwMuxCtlPadGpioB213 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioB213 {
        SwMuxCtlPadGpioB213(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioB213 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioB213")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioB213 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioB213 {
            mux_mode: super::vals::SwMuxCtlPadGpioB213MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioB213 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_00 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB100(pub u32);
impl SwMuxCtlPadGpioEmcB100 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB100MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB100MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB100MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB100 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB100 {
        SwMuxCtlPadGpioEmcB100(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB100 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB100")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB100 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB100 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB100MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB100 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_01 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB101(pub u32);
impl SwMuxCtlPadGpioEmcB101 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB101MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB101MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB101MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB101 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB101 {
        SwMuxCtlPadGpioEmcB101(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB101 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB101")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB101 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB101 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB101MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB101 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_02 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB102(pub u32);
impl SwMuxCtlPadGpioEmcB102 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB102MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB102MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB102MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB102 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB102 {
        SwMuxCtlPadGpioEmcB102(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB102 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB102")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB102 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB102 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB102MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB102 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_03 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB103(pub u32);
impl SwMuxCtlPadGpioEmcB103 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB103MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB103MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB103MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB103 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB103 {
        SwMuxCtlPadGpioEmcB103(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB103 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB103")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB103 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB103 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB103MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB103 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_04 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB104(pub u32);
impl SwMuxCtlPadGpioEmcB104 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB104MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB104MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB104MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB104 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB104 {
        SwMuxCtlPadGpioEmcB104(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB104 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB104")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB104 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB104 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB104MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB104 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_05 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB105(pub u32);
impl SwMuxCtlPadGpioEmcB105 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB105MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB105MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB105MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB105 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB105 {
        SwMuxCtlPadGpioEmcB105(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB105 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB105")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB105 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB105 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB105MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB105 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_06 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB106(pub u32);
impl SwMuxCtlPadGpioEmcB106 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB106MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB106MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB106MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB106 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB106 {
        SwMuxCtlPadGpioEmcB106(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB106 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB106")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB106 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB106 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB106MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB106 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_07 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB107(pub u32);
impl SwMuxCtlPadGpioEmcB107 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB107MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB107MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB107MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB107 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB107 {
        SwMuxCtlPadGpioEmcB107(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB107 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB107")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB107 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB107 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB107MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB107 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_08 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB108(pub u32);
impl SwMuxCtlPadGpioEmcB108 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB108MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB108MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB108MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB108 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB108 {
        SwMuxCtlPadGpioEmcB108(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB108 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB108")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB108 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB108 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB108MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB108 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_09 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB109(pub u32);
impl SwMuxCtlPadGpioEmcB109 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB109MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB109MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB109MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB109 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB109 {
        SwMuxCtlPadGpioEmcB109(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB109 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB109")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB109 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB109 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB109MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB109 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_10 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB110(pub u32);
impl SwMuxCtlPadGpioEmcB110 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB110MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB110MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB110MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB110 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB110 {
        SwMuxCtlPadGpioEmcB110(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB110 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB110")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB110 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB110 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB110MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB110 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_11 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB111(pub u32);
impl SwMuxCtlPadGpioEmcB111 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB111MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB111MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB111MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB111 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB111 {
        SwMuxCtlPadGpioEmcB111(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB111 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB111")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB111 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB111 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB111MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB111 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_12 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB112(pub u32);
impl SwMuxCtlPadGpioEmcB112 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB112MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB112MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB112MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB112 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB112 {
        SwMuxCtlPadGpioEmcB112(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB112 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB112")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB112 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB112 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB112MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB112 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_13 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB113(pub u32);
impl SwMuxCtlPadGpioEmcB113 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB113MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB113MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB113MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB113 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB113 {
        SwMuxCtlPadGpioEmcB113(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB113 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB113")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB113 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB113 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB113MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB113 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_14 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB114(pub u32);
impl SwMuxCtlPadGpioEmcB114 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB114MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB114MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB114MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB114 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB114 {
        SwMuxCtlPadGpioEmcB114(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB114 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB114")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB114 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB114 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB114MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB114 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_15 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB115(pub u32);
impl SwMuxCtlPadGpioEmcB115 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB115MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB115MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB115MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB115 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB115 {
        SwMuxCtlPadGpioEmcB115(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB115 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB115")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB115 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB115 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB115MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB115 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_16 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB116(pub u32);
impl SwMuxCtlPadGpioEmcB116 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB116MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB116MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB116MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB116 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB116 {
        SwMuxCtlPadGpioEmcB116(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB116 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB116")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB116 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB116 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB116MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB116 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_17 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB117(pub u32);
impl SwMuxCtlPadGpioEmcB117 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB117MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB117MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB117MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB117 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB117 {
        SwMuxCtlPadGpioEmcB117(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB117 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB117")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB117 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB117 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB117MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB117 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_18 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB118(pub u32);
impl SwMuxCtlPadGpioEmcB118 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB118MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB118MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB118MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB118 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB118 {
        SwMuxCtlPadGpioEmcB118(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB118 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB118")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB118 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB118 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB118MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB118 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_19 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB119(pub u32);
impl SwMuxCtlPadGpioEmcB119 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB119MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB119MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB119MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB119 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB119 {
        SwMuxCtlPadGpioEmcB119(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB119 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB119")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB119 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB119 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB119MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB119 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_20 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB120(pub u32);
impl SwMuxCtlPadGpioEmcB120 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB120MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB120MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB120MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB120 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB120 {
        SwMuxCtlPadGpioEmcB120(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB120 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB120")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB120 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB120 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB120MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB120 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_21 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB121(pub u32);
impl SwMuxCtlPadGpioEmcB121 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB121MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB121MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB121MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB121 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB121 {
        SwMuxCtlPadGpioEmcB121(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB121 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB121")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB121 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB121 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB121MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB121 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_22 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB122(pub u32);
impl SwMuxCtlPadGpioEmcB122 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB122MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB122MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB122MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB122 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB122 {
        SwMuxCtlPadGpioEmcB122(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB122 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB122")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB122 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB122 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB122MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB122 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_23 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB123(pub u32);
impl SwMuxCtlPadGpioEmcB123 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB123MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB123MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB123MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB123 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB123 {
        SwMuxCtlPadGpioEmcB123(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB123 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB123")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB123 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB123 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB123MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB123 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_24 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB124(pub u32);
impl SwMuxCtlPadGpioEmcB124 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB124MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB124MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB124MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB124 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB124 {
        SwMuxCtlPadGpioEmcB124(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB124 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB124")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB124 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB124 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB124MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB124 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_25 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB125(pub u32);
impl SwMuxCtlPadGpioEmcB125 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB125MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB125MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB125MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB125 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB125 {
        SwMuxCtlPadGpioEmcB125(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB125 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB125")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB125 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB125 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB125MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB125 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_26 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB126(pub u32);
impl SwMuxCtlPadGpioEmcB126 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB126MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB126MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB126MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB126 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB126 {
        SwMuxCtlPadGpioEmcB126(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB126 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB126")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB126 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB126 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB126MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB126 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_27 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB127(pub u32);
impl SwMuxCtlPadGpioEmcB127 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB127MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB127MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB127MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB127 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB127 {
        SwMuxCtlPadGpioEmcB127(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB127 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB127")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB127 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB127 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB127MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB127 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_28 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB128(pub u32);
impl SwMuxCtlPadGpioEmcB128 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB128MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB128MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB128MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB128 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB128 {
        SwMuxCtlPadGpioEmcB128(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB128 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB128")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB128 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB128 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB128MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB128 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_29 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB129(pub u32);
impl SwMuxCtlPadGpioEmcB129 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB129MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB129MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB129MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB129 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB129 {
        SwMuxCtlPadGpioEmcB129(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB129 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB129")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB129 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB129 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB129MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB129 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_30 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB130(pub u32);
impl SwMuxCtlPadGpioEmcB130 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB130MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB130MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB130MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB130 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB130 {
        SwMuxCtlPadGpioEmcB130(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB130 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB130")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB130 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB130 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB130MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB130 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_31 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB131(pub u32);
impl SwMuxCtlPadGpioEmcB131 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB131MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB131MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB131MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB131 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB131 {
        SwMuxCtlPadGpioEmcB131(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB131 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB131")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB131 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB131 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB131MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB131 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_32 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB132(pub u32);
impl SwMuxCtlPadGpioEmcB132 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB132MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB132MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB132MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB132 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB132 {
        SwMuxCtlPadGpioEmcB132(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB132 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB132")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB132 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB132 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB132MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB132 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_33 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB133(pub u32);
impl SwMuxCtlPadGpioEmcB133 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB133MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB133MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB133MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB133 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB133 {
        SwMuxCtlPadGpioEmcB133(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB133 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB133")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB133 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB133 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB133MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB133 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_34 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB134(pub u32);
impl SwMuxCtlPadGpioEmcB134 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB134MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB134MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB134MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB134 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB134 {
        SwMuxCtlPadGpioEmcB134(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB134 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB134")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB134 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB134 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB134MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB134 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_35 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB135(pub u32);
impl SwMuxCtlPadGpioEmcB135 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB135MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB135MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB135MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB135 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB135 {
        SwMuxCtlPadGpioEmcB135(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB135 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB135")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB135 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB135 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB135MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB135 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_36 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB136(pub u32);
impl SwMuxCtlPadGpioEmcB136 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB136MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB136MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB136MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB136 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB136 {
        SwMuxCtlPadGpioEmcB136(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB136 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB136")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB136 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB136 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB136MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB136 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_37 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB137(pub u32);
impl SwMuxCtlPadGpioEmcB137 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB137MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB137MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB137MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB137 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB137 {
        SwMuxCtlPadGpioEmcB137(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB137 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB137")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB137 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB137 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB137MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB137 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_38 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB138(pub u32);
impl SwMuxCtlPadGpioEmcB138 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB138MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB138MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB138MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB138 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB138 {
        SwMuxCtlPadGpioEmcB138(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB138 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB138")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB138 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB138 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB138MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB138 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_39 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB139(pub u32);
impl SwMuxCtlPadGpioEmcB139 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB139MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB139MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB139MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB139 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB139 {
        SwMuxCtlPadGpioEmcB139(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB139 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB139")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB139 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB139 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB139MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB139 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_40 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB140(pub u32);
impl SwMuxCtlPadGpioEmcB140 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB140MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB140MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB140MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB140 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB140 {
        SwMuxCtlPadGpioEmcB140(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB140 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB140")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB140 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB140 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB140MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB140 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_41 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB141(pub u32);
impl SwMuxCtlPadGpioEmcB141 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB141MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB141MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB141MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB141 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB141 {
        SwMuxCtlPadGpioEmcB141(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB141 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB141")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB141 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB141 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB141MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB141 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B2_00 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB200(pub u32);
impl SwMuxCtlPadGpioEmcB200 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB200MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB200MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB200MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB200 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB200 {
        SwMuxCtlPadGpioEmcB200(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB200 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB200")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB200 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB200 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB200MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB200 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B2_01 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB201(pub u32);
impl SwMuxCtlPadGpioEmcB201 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB201MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB201MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB201MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB201 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB201 {
        SwMuxCtlPadGpioEmcB201(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB201 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB201")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB201 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB201 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB201MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB201 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B2_02 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB202(pub u32);
impl SwMuxCtlPadGpioEmcB202 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB202MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB202MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB202MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB202 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB202 {
        SwMuxCtlPadGpioEmcB202(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB202 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB202")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB202 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB202 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB202MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB202 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B2_03 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB203(pub u32);
impl SwMuxCtlPadGpioEmcB203 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB203MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB203MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB203MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB203 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB203 {
        SwMuxCtlPadGpioEmcB203(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB203 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB203")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB203 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB203 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB203MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB203 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B2_04 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB204(pub u32);
impl SwMuxCtlPadGpioEmcB204 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB204MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB204MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB204MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB204 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB204 {
        SwMuxCtlPadGpioEmcB204(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB204 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB204")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB204 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB204 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB204MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB204 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B2_05 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB205(pub u32);
impl SwMuxCtlPadGpioEmcB205 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB205MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB205MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB205MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB205 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB205 {
        SwMuxCtlPadGpioEmcB205(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB205 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB205")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB205 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB205 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB205MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB205 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B2_06 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB206(pub u32);
impl SwMuxCtlPadGpioEmcB206 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB206MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB206MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB206MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB206 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB206 {
        SwMuxCtlPadGpioEmcB206(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB206 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB206")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB206 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB206 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB206MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB206 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B2_07 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB207(pub u32);
impl SwMuxCtlPadGpioEmcB207 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB207MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB207MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB207MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB207 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB207 {
        SwMuxCtlPadGpioEmcB207(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB207 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB207")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB207 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB207 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB207MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB207 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B2_08 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB208(pub u32);
impl SwMuxCtlPadGpioEmcB208 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB208MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB208MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB208MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB208 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB208 {
        SwMuxCtlPadGpioEmcB208(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB208 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB208")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB208 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB208 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB208MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB208 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B2_09 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB209(pub u32);
impl SwMuxCtlPadGpioEmcB209 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB209MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB209MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB209MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB209 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB209 {
        SwMuxCtlPadGpioEmcB209(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB209 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB209")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB209 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB209 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB209MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB209 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B2_10 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB210(pub u32);
impl SwMuxCtlPadGpioEmcB210 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB210MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB210MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB210MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB210 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB210 {
        SwMuxCtlPadGpioEmcB210(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB210 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB210")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB210 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB210 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB210MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB210 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B2_11 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB211(pub u32);
impl SwMuxCtlPadGpioEmcB211 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB211MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB211MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB211MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB211 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB211 {
        SwMuxCtlPadGpioEmcB211(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB211 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB211")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB211 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB211 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB211MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB211 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B2_12 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB212(pub u32);
impl SwMuxCtlPadGpioEmcB212 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB212MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB212MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB212MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB212 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB212 {
        SwMuxCtlPadGpioEmcB212(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB212 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB212")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB212 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB212 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB212MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB212 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B2_13 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB213(pub u32);
impl SwMuxCtlPadGpioEmcB213 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB213MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB213MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB213MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB213 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB213 {
        SwMuxCtlPadGpioEmcB213(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB213 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB213")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB213 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB213 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB213MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB213 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B2_14 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB214(pub u32);
impl SwMuxCtlPadGpioEmcB214 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB214MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB214MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB214MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB214 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB214 {
        SwMuxCtlPadGpioEmcB214(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB214 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB214")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB214 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB214 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB214MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB214 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B2_15 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB215(pub u32);
impl SwMuxCtlPadGpioEmcB215 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB215MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB215MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB215MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB215 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB215 {
        SwMuxCtlPadGpioEmcB215(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB215 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB215")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB215 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB215 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB215MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB215 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B2_16 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB216(pub u32);
impl SwMuxCtlPadGpioEmcB216 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB216MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB216MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB216MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB216 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB216 {
        SwMuxCtlPadGpioEmcB216(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB216 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB216")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB216 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB216 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB216MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB216 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B2_17 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB217(pub u32);
impl SwMuxCtlPadGpioEmcB217 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB217MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB217MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB217MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB217 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB217 {
        SwMuxCtlPadGpioEmcB217(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB217 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB217")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB217 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB217 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB217MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB217 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B2_18 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB218(pub u32);
impl SwMuxCtlPadGpioEmcB218 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB218MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB218MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB218MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB218 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB218 {
        SwMuxCtlPadGpioEmcB218(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB218 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB218")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB218 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB218 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB218MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB218 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B2_19 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB219(pub u32);
impl SwMuxCtlPadGpioEmcB219 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB219MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB219MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB219MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB219 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB219 {
        SwMuxCtlPadGpioEmcB219(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB219 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB219")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB219 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB219 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB219MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB219 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B2_20 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioEmcB220(pub u32);
impl SwMuxCtlPadGpioEmcB220 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioEmcB220MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioEmcB220MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioEmcB220MuxMode) {
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
impl Default for SwMuxCtlPadGpioEmcB220 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioEmcB220 {
        SwMuxCtlPadGpioEmcB220(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioEmcB220 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioEmcB220")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioEmcB220 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioEmcB220 {
            mux_mode: super::vals::SwMuxCtlPadGpioEmcB220MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioEmcB220 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_00 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioSdB100(pub u32);
impl SwMuxCtlPadGpioSdB100 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioSdB100MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioSdB100MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioSdB100MuxMode) {
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
impl Default for SwMuxCtlPadGpioSdB100 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioSdB100 {
        SwMuxCtlPadGpioSdB100(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioSdB100 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioSdB100")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioSdB100 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioSdB100 {
            mux_mode: super::vals::SwMuxCtlPadGpioSdB100MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioSdB100 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_01 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioSdB101(pub u32);
impl SwMuxCtlPadGpioSdB101 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioSdB101MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioSdB101MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioSdB101MuxMode) {
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
impl Default for SwMuxCtlPadGpioSdB101 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioSdB101 {
        SwMuxCtlPadGpioSdB101(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioSdB101 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioSdB101")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioSdB101 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioSdB101 {
            mux_mode: super::vals::SwMuxCtlPadGpioSdB101MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioSdB101 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_02 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioSdB102(pub u32);
impl SwMuxCtlPadGpioSdB102 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioSdB102MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioSdB102MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioSdB102MuxMode) {
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
impl Default for SwMuxCtlPadGpioSdB102 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioSdB102 {
        SwMuxCtlPadGpioSdB102(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioSdB102 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioSdB102")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioSdB102 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioSdB102 {
            mux_mode: super::vals::SwMuxCtlPadGpioSdB102MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioSdB102 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_03 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioSdB103(pub u32);
impl SwMuxCtlPadGpioSdB103 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioSdB103MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioSdB103MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioSdB103MuxMode) {
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
impl Default for SwMuxCtlPadGpioSdB103 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioSdB103 {
        SwMuxCtlPadGpioSdB103(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioSdB103 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioSdB103")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioSdB103 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioSdB103 {
            mux_mode: super::vals::SwMuxCtlPadGpioSdB103MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioSdB103 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_04 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioSdB104(pub u32);
impl SwMuxCtlPadGpioSdB104 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioSdB104MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioSdB104MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioSdB104MuxMode) {
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
impl Default for SwMuxCtlPadGpioSdB104 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioSdB104 {
        SwMuxCtlPadGpioSdB104(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioSdB104 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioSdB104")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioSdB104 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioSdB104 {
            mux_mode: super::vals::SwMuxCtlPadGpioSdB104MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioSdB104 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_05 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioSdB105(pub u32);
impl SwMuxCtlPadGpioSdB105 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioSdB105MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioSdB105MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioSdB105MuxMode) {
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
impl Default for SwMuxCtlPadGpioSdB105 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioSdB105 {
        SwMuxCtlPadGpioSdB105(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioSdB105 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioSdB105")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioSdB105 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioSdB105 {
            mux_mode: super::vals::SwMuxCtlPadGpioSdB105MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioSdB105 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B2_00 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioSdB200(pub u32);
impl SwMuxCtlPadGpioSdB200 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioSdB200MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioSdB200MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioSdB200MuxMode) {
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
impl Default for SwMuxCtlPadGpioSdB200 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioSdB200 {
        SwMuxCtlPadGpioSdB200(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioSdB200 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioSdB200")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioSdB200 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioSdB200 {
            mux_mode: super::vals::SwMuxCtlPadGpioSdB200MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioSdB200 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B2_01 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioSdB201(pub u32);
impl SwMuxCtlPadGpioSdB201 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioSdB201MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioSdB201MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioSdB201MuxMode) {
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
impl Default for SwMuxCtlPadGpioSdB201 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioSdB201 {
        SwMuxCtlPadGpioSdB201(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioSdB201 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioSdB201")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioSdB201 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioSdB201 {
            mux_mode: super::vals::SwMuxCtlPadGpioSdB201MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioSdB201 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B2_02 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioSdB202(pub u32);
impl SwMuxCtlPadGpioSdB202 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioSdB202MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioSdB202MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioSdB202MuxMode) {
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
impl Default for SwMuxCtlPadGpioSdB202 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioSdB202 {
        SwMuxCtlPadGpioSdB202(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioSdB202 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioSdB202")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioSdB202 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioSdB202 {
            mux_mode: super::vals::SwMuxCtlPadGpioSdB202MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioSdB202 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B2_03 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioSdB203(pub u32);
impl SwMuxCtlPadGpioSdB203 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioSdB203MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioSdB203MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioSdB203MuxMode) {
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
impl Default for SwMuxCtlPadGpioSdB203 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioSdB203 {
        SwMuxCtlPadGpioSdB203(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioSdB203 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioSdB203")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioSdB203 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioSdB203 {
            mux_mode: super::vals::SwMuxCtlPadGpioSdB203MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioSdB203 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B2_04 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioSdB204(pub u32);
impl SwMuxCtlPadGpioSdB204 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioSdB204MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioSdB204MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioSdB204MuxMode) {
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
impl Default for SwMuxCtlPadGpioSdB204 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioSdB204 {
        SwMuxCtlPadGpioSdB204(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioSdB204 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioSdB204")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioSdB204 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioSdB204 {
            mux_mode: super::vals::SwMuxCtlPadGpioSdB204MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioSdB204 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B2_05 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioSdB205(pub u32);
impl SwMuxCtlPadGpioSdB205 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioSdB205MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioSdB205MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioSdB205MuxMode) {
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
impl Default for SwMuxCtlPadGpioSdB205 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioSdB205 {
        SwMuxCtlPadGpioSdB205(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioSdB205 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioSdB205")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioSdB205 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioSdB205 {
            mux_mode: super::vals::SwMuxCtlPadGpioSdB205MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioSdB205 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B2_06 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioSdB206(pub u32);
impl SwMuxCtlPadGpioSdB206 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioSdB206MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioSdB206MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioSdB206MuxMode) {
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
impl Default for SwMuxCtlPadGpioSdB206 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioSdB206 {
        SwMuxCtlPadGpioSdB206(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioSdB206 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioSdB206")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioSdB206 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioSdB206 {
            mux_mode: super::vals::SwMuxCtlPadGpioSdB206MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioSdB206 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B2_07 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioSdB207(pub u32);
impl SwMuxCtlPadGpioSdB207 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioSdB207MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioSdB207MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioSdB207MuxMode) {
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
impl Default for SwMuxCtlPadGpioSdB207 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioSdB207 {
        SwMuxCtlPadGpioSdB207(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioSdB207 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioSdB207")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioSdB207 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioSdB207 {
            mux_mode: super::vals::SwMuxCtlPadGpioSdB207MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioSdB207 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B2_08 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioSdB208(pub u32);
impl SwMuxCtlPadGpioSdB208 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioSdB208MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioSdB208MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioSdB208MuxMode) {
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
impl Default for SwMuxCtlPadGpioSdB208 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioSdB208 {
        SwMuxCtlPadGpioSdB208(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioSdB208 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioSdB208")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioSdB208 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioSdB208 {
            mux_mode: super::vals::SwMuxCtlPadGpioSdB208MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioSdB208 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B2_09 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioSdB209(pub u32);
impl SwMuxCtlPadGpioSdB209 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioSdB209MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioSdB209MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioSdB209MuxMode) {
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
impl Default for SwMuxCtlPadGpioSdB209 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioSdB209 {
        SwMuxCtlPadGpioSdB209(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioSdB209 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioSdB209")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioSdB209 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioSdB209 {
            mux_mode: super::vals::SwMuxCtlPadGpioSdB209MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioSdB209 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B2_10 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioSdB210(pub u32);
impl SwMuxCtlPadGpioSdB210 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioSdB210MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioSdB210MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioSdB210MuxMode) {
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
impl Default for SwMuxCtlPadGpioSdB210 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioSdB210 {
        SwMuxCtlPadGpioSdB210(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioSdB210 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioSdB210")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioSdB210 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioSdB210 {
            mux_mode: super::vals::SwMuxCtlPadGpioSdB210MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioSdB210 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B2_11 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioSdB211(pub u32);
impl SwMuxCtlPadGpioSdB211 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioSdB211MuxMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SwMuxCtlPadGpioSdB211MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioSdB211MuxMode) {
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
impl Default for SwMuxCtlPadGpioSdB211 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioSdB211 {
        SwMuxCtlPadGpioSdB211(5u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioSdB211 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioSdB211")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioSdB211 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioSdB211 {
            mux_mode: super::vals::SwMuxCtlPadGpioSdB211MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioSdB211 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B2_12_DUMMY SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwMuxCtlPadGpioSdB212(pub u32);
impl SwMuxCtlPadGpioSdB212 {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> super::vals::SwMuxCtlPadGpioSdB212MuxMode {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::SwMuxCtlPadGpioSdB212MuxMode::from_bits(val as u8)
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: super::vals::SwMuxCtlPadGpioSdB212MuxMode) {
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
impl Default for SwMuxCtlPadGpioSdB212 {
    #[inline(always)]
    fn default() -> SwMuxCtlPadGpioSdB212 {
        SwMuxCtlPadGpioSdB212(0u64 as u32)
    }
}
impl core::fmt::Debug for SwMuxCtlPadGpioSdB212 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwMuxCtlPadGpioSdB212")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwMuxCtlPadGpioSdB212 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwMuxCtlPadGpioSdB212 {
            mux_mode: super::vals::SwMuxCtlPadGpioSdB212MuxMode,
            sion: bool,
        }
        let proxy = SwMuxCtlPadGpioSdB212 {
            mux_mode: self.mux_mode(),
            sion: self.sion(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_B1_00 SW PAD Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwPadCtlPadGpio(pub u32);
impl SwPadCtlPadGpio {
    #[doc = "PDRV Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pdrv(&self) -> super::vals::Pdrv {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pdrv::from_bits(val as u8)
    }
    #[doc = "PDRV Field"]
    #[inline(always)]
    pub const fn set_pdrv(&mut self, val: super::vals::Pdrv) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Pull Down Pull Up Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pull(&self) -> super::vals::Pull {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Pull::from_bits(val as u8)
    }
    #[doc = "Pull Down Pull Up Field"]
    #[inline(always)]
    pub const fn set_pull(&mut self, val: super::vals::Pull) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Open Drain Field"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::Ode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Field"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::Ode) {
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
impl Default for SwPadCtlPadGpio {
    #[inline(always)]
    fn default() -> SwPadCtlPadGpio {
        SwPadCtlPadGpio(4u64 as u32)
    }
}
impl core::fmt::Debug for SwPadCtlPadGpio {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwPadCtlPadGpio")
            .field("pdrv", &self.pdrv())
            .field("pull", &self.pull())
            .field("ode", &self.ode())
            .field("dwp", &self.dwp())
            .field("dwp_lock", &self.dwp_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwPadCtlPadGpio {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwPadCtlPadGpio {
            pdrv: super::vals::Pdrv,
            pull: super::vals::Pull,
            ode: super::vals::Ode,
            dwp: u8,
            dwp_lock: u8,
        }
        let proxy = SwPadCtlPadGpio {
            pdrv: self.pdrv(),
            pull: self.pull(),
            ode: self.ode(),
            dwp: self.dwp(),
            dwp_lock: self.dwp_lock(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_00 SW PAD Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwPadCtlPadGpioAd(pub u32);
impl SwPadCtlPadGpioAd {
    #[doc = "Slew Rate Field"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::Sre {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Field"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::Sre) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Drive Strength Field"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::Dse {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Field"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::Dse) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Pull / Keep Select Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pue(&self) -> super::vals::Pue {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pue::from_bits(val as u8)
    }
    #[doc = "Pull / Keep Select Field"]
    #[inline(always)]
    pub const fn set_pue(&mut self, val: super::vals::Pue) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pus(&self) -> super::vals::Pus {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pus::from_bits(val as u8)
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[inline(always)]
    pub const fn set_pus(&mut self, val: super::vals::Pus) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Field"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::Ode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Field"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::Ode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Force ibe off Field"]
    #[must_use]
    #[inline(always)]
    pub const fn ibe_off(&self) -> super::vals::IbeOff {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::IbeOff::from_bits(val as u8)
    }
    #[doc = "Force ibe off Field"]
    #[inline(always)]
    pub const fn set_ibe_off(&mut self, val: super::vals::IbeOff) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
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
impl Default for SwPadCtlPadGpioAd {
    #[inline(always)]
    fn default() -> SwPadCtlPadGpioAd {
        SwPadCtlPadGpioAd(6u64 as u32)
    }
}
impl core::fmt::Debug for SwPadCtlPadGpioAd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwPadCtlPadGpioAd")
            .field("sre", &self.sre())
            .field("dse", &self.dse())
            .field("pue", &self.pue())
            .field("pus", &self.pus())
            .field("ode", &self.ode())
            .field("ibe_off", &self.ibe_off())
            .field("dwp", &self.dwp())
            .field("dwp_lock", &self.dwp_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwPadCtlPadGpioAd {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwPadCtlPadGpioAd {
            sre: super::vals::Sre,
            dse: super::vals::Dse,
            pue: super::vals::Pue,
            pus: super::vals::Pus,
            ode: super::vals::Ode,
            ibe_off: super::vals::IbeOff,
            dwp: u8,
            dwp_lock: u8,
        }
        let proxy = SwPadCtlPadGpioAd {
            sre: self.sre(),
            dse: self.dse(),
            pue: self.pue(),
            pus: self.pus(),
            ode: self.ode(),
            ibe_off: self.ibe_off(),
            dwp: self.dwp(),
            dwp_lock: self.dwp_lock(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AON_00 SW PAD Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwPadCtlPadGpioAon(pub u32);
impl SwPadCtlPadGpioAon {
    #[doc = "Slew Rate Field"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::Sre {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Field"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::Sre) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Drive Strength Field"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::Dse {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Field"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::Dse) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Pull / Keep Select Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pue(&self) -> super::vals::Pue {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pue::from_bits(val as u8)
    }
    #[doc = "Pull / Keep Select Field"]
    #[inline(always)]
    pub const fn set_pue(&mut self, val: super::vals::Pue) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pus(&self) -> super::vals::Pus {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pus::from_bits(val as u8)
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[inline(always)]
    pub const fn set_pus(&mut self, val: super::vals::Pus) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Field"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::Ode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Field"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::Ode) {
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
impl Default for SwPadCtlPadGpioAon {
    #[inline(always)]
    fn default() -> SwPadCtlPadGpioAon {
        SwPadCtlPadGpioAon(6u64 as u32)
    }
}
impl core::fmt::Debug for SwPadCtlPadGpioAon {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwPadCtlPadGpioAon")
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
impl defmt::Format for SwPadCtlPadGpioAon {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwPadCtlPadGpioAon {
            sre: super::vals::Sre,
            dse: super::vals::Dse,
            pue: super::vals::Pue,
            pus: super::vals::Pus,
            ode: super::vals::Ode,
            dwp: u8,
            dwp_lock: u8,
        }
        let proxy = SwPadCtlPadGpioAon {
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
#[doc = "USB_IPP_IND_OTG2_OC_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbIppIndOtg2OcSelectInput(pub u32);
impl UsbIppIndOtg2OcSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::UsbIppIndOtg2OcSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::UsbIppIndOtg2OcSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::UsbIppIndOtg2OcSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for UsbIppIndOtg2OcSelectInput {
    #[inline(always)]
    fn default() -> UsbIppIndOtg2OcSelectInput {
        UsbIppIndOtg2OcSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for UsbIppIndOtg2OcSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UsbIppIndOtg2OcSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UsbIppIndOtg2OcSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct UsbIppIndOtg2OcSelectInput {
            daisy: super::vals::UsbIppIndOtg2OcSelectInputDaisy,
        }
        let proxy = UsbIppIndOtg2OcSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "USB_IPP_IND_OTG_OC_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbIppIndOtgOcSelectInput(pub u32);
impl UsbIppIndOtgOcSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::UsbIppIndOtgOcSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::UsbIppIndOtgOcSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::UsbIppIndOtgOcSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for UsbIppIndOtgOcSelectInput {
    #[inline(always)]
    fn default() -> UsbIppIndOtgOcSelectInput {
        UsbIppIndOtgOcSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for UsbIppIndOtgOcSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UsbIppIndOtgOcSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UsbIppIndOtgOcSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct UsbIppIndOtgOcSelectInput {
            daisy: super::vals::UsbIppIndOtgOcSelectInputDaisy,
        }
        let proxy = UsbIppIndOtgOcSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "USBPHY1_USB_ID_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbphy1UsbIdSelectInput(pub u32);
impl Usbphy1UsbIdSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Usbphy1UsbIdSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Usbphy1UsbIdSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Usbphy1UsbIdSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Usbphy1UsbIdSelectInput {
    #[inline(always)]
    fn default() -> Usbphy1UsbIdSelectInput {
        Usbphy1UsbIdSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Usbphy1UsbIdSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usbphy1UsbIdSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usbphy1UsbIdSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Usbphy1UsbIdSelectInput {
            daisy: super::vals::Usbphy1UsbIdSelectInputDaisy,
        }
        let proxy = Usbphy1UsbIdSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "USBPHY2_USB_ID_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbphy2UsbIdSelectInput(pub u32);
impl Usbphy2UsbIdSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Usbphy2UsbIdSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Usbphy2UsbIdSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Usbphy2UsbIdSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Usbphy2UsbIdSelectInput {
    #[inline(always)]
    fn default() -> Usbphy2UsbIdSelectInput {
        Usbphy2UsbIdSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Usbphy2UsbIdSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usbphy2UsbIdSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usbphy2UsbIdSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Usbphy2UsbIdSelectInput {
            daisy: super::vals::Usbphy2UsbIdSelectInputDaisy,
        }
        let proxy = Usbphy2UsbIdSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "USDHC1_IPP_CARD_DET_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usdhc1IppCardDetSelectInput(pub u32);
impl Usdhc1IppCardDetSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Usdhc1IppCardDetSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Usdhc1IppCardDetSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Usdhc1IppCardDetSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Usdhc1IppCardDetSelectInput {
    #[inline(always)]
    fn default() -> Usdhc1IppCardDetSelectInput {
        Usdhc1IppCardDetSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Usdhc1IppCardDetSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usdhc1IppCardDetSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usdhc1IppCardDetSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Usdhc1IppCardDetSelectInput {
            daisy: super::vals::Usdhc1IppCardDetSelectInputDaisy,
        }
        let proxy = Usdhc1IppCardDetSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "USDHC1_IPP_WP_ON_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usdhc1IppWpOnSelectInput(pub u32);
impl Usdhc1IppWpOnSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Usdhc1IppWpOnSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Usdhc1IppWpOnSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Usdhc1IppWpOnSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Usdhc1IppWpOnSelectInput {
    #[inline(always)]
    fn default() -> Usdhc1IppWpOnSelectInput {
        Usdhc1IppWpOnSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Usdhc1IppWpOnSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usdhc1IppWpOnSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usdhc1IppWpOnSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Usdhc1IppWpOnSelectInput {
            daisy: super::vals::Usdhc1IppWpOnSelectInputDaisy,
        }
        let proxy = Usdhc1IppWpOnSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "USDHC2_IPP_CARD_DET_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usdhc2IppCardDetSelectInput(pub u32);
impl Usdhc2IppCardDetSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Usdhc2IppCardDetSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Usdhc2IppCardDetSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Usdhc2IppCardDetSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Usdhc2IppCardDetSelectInput {
    #[inline(always)]
    fn default() -> Usdhc2IppCardDetSelectInput {
        Usdhc2IppCardDetSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Usdhc2IppCardDetSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usdhc2IppCardDetSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usdhc2IppCardDetSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Usdhc2IppCardDetSelectInput {
            daisy: super::vals::Usdhc2IppCardDetSelectInputDaisy,
        }
        let proxy = Usdhc2IppCardDetSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "USDHC2_IPP_WP_ON_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usdhc2IppWpOnSelectInput(pub u32);
impl Usdhc2IppWpOnSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Usdhc2IppWpOnSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Usdhc2IppWpOnSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Usdhc2IppWpOnSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Usdhc2IppWpOnSelectInput {
    #[inline(always)]
    fn default() -> Usdhc2IppWpOnSelectInput {
        Usdhc2IppWpOnSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for Usdhc2IppWpOnSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usdhc2IppWpOnSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usdhc2IppWpOnSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Usdhc2IppWpOnSelectInput {
            daisy: super::vals::Usdhc2IppWpOnSelectInputDaisy,
        }
        let proxy = Usdhc2IppWpOnSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "XBAR1_XBAR_IN_SELECT_INPUT_14 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbar1XbarInSelectInput14(pub u32);
impl Xbar1XbarInSelectInput14 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Xbar1XbarInSelectInput14Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Xbar1XbarInSelectInput14Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Xbar1XbarInSelectInput14Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Xbar1XbarInSelectInput14 {
    #[inline(always)]
    fn default() -> Xbar1XbarInSelectInput14 {
        Xbar1XbarInSelectInput14(0u64 as u32)
    }
}
impl core::fmt::Debug for Xbar1XbarInSelectInput14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xbar1XbarInSelectInput14")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xbar1XbarInSelectInput14 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Xbar1XbarInSelectInput14 {
            daisy: super::vals::Xbar1XbarInSelectInput14Daisy,
        }
        let proxy = Xbar1XbarInSelectInput14 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "XBAR1_XBAR_IN_SELECT_INPUT_15 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbar1XbarInSelectInput15(pub u32);
impl Xbar1XbarInSelectInput15 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Xbar1XbarInSelectInput15Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Xbar1XbarInSelectInput15Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Xbar1XbarInSelectInput15Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Xbar1XbarInSelectInput15 {
    #[inline(always)]
    fn default() -> Xbar1XbarInSelectInput15 {
        Xbar1XbarInSelectInput15(0u64 as u32)
    }
}
impl core::fmt::Debug for Xbar1XbarInSelectInput15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xbar1XbarInSelectInput15")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xbar1XbarInSelectInput15 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Xbar1XbarInSelectInput15 {
            daisy: super::vals::Xbar1XbarInSelectInput15Daisy,
        }
        let proxy = Xbar1XbarInSelectInput15 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "XBAR1_XBAR_IN_SELECT_INPUT_17 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbar1XbarInSelectInput17(pub u32);
impl Xbar1XbarInSelectInput17 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Xbar1XbarInSelectInput17Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Xbar1XbarInSelectInput17Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Xbar1XbarInSelectInput17Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Xbar1XbarInSelectInput17 {
    #[inline(always)]
    fn default() -> Xbar1XbarInSelectInput17 {
        Xbar1XbarInSelectInput17(0u64 as u32)
    }
}
impl core::fmt::Debug for Xbar1XbarInSelectInput17 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xbar1XbarInSelectInput17")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xbar1XbarInSelectInput17 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Xbar1XbarInSelectInput17 {
            daisy: super::vals::Xbar1XbarInSelectInput17Daisy,
        }
        let proxy = Xbar1XbarInSelectInput17 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "XBAR1_XBAR_IN_SELECT_INPUT_18 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbar1XbarInSelectInput18(pub u32);
impl Xbar1XbarInSelectInput18 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Xbar1XbarInSelectInput18Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Xbar1XbarInSelectInput18Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Xbar1XbarInSelectInput18Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Xbar1XbarInSelectInput18 {
    #[inline(always)]
    fn default() -> Xbar1XbarInSelectInput18 {
        Xbar1XbarInSelectInput18(0u64 as u32)
    }
}
impl core::fmt::Debug for Xbar1XbarInSelectInput18 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xbar1XbarInSelectInput18")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xbar1XbarInSelectInput18 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Xbar1XbarInSelectInput18 {
            daisy: super::vals::Xbar1XbarInSelectInput18Daisy,
        }
        let proxy = Xbar1XbarInSelectInput18 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "XBAR1_XBAR_IN_SELECT_INPUT_19 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbar1XbarInSelectInput19(pub u32);
impl Xbar1XbarInSelectInput19 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Xbar1XbarInSelectInput19Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Xbar1XbarInSelectInput19Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Xbar1XbarInSelectInput19Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Xbar1XbarInSelectInput19 {
    #[inline(always)]
    fn default() -> Xbar1XbarInSelectInput19 {
        Xbar1XbarInSelectInput19(0u64 as u32)
    }
}
impl core::fmt::Debug for Xbar1XbarInSelectInput19 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xbar1XbarInSelectInput19")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xbar1XbarInSelectInput19 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Xbar1XbarInSelectInput19 {
            daisy: super::vals::Xbar1XbarInSelectInput19Daisy,
        }
        let proxy = Xbar1XbarInSelectInput19 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "XBAR1_XBAR_IN_SELECT_INPUT_20 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbar1XbarInSelectInput20(pub u32);
impl Xbar1XbarInSelectInput20 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Xbar1XbarInSelectInput20Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Xbar1XbarInSelectInput20Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Xbar1XbarInSelectInput20Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Xbar1XbarInSelectInput20 {
    #[inline(always)]
    fn default() -> Xbar1XbarInSelectInput20 {
        Xbar1XbarInSelectInput20(0u64 as u32)
    }
}
impl core::fmt::Debug for Xbar1XbarInSelectInput20 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xbar1XbarInSelectInput20")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xbar1XbarInSelectInput20 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Xbar1XbarInSelectInput20 {
            daisy: super::vals::Xbar1XbarInSelectInput20Daisy,
        }
        let proxy = Xbar1XbarInSelectInput20 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "XBAR1_XBAR_IN_SELECT_INPUT_21 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbar1XbarInSelectInput21(pub u32);
impl Xbar1XbarInSelectInput21 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Xbar1XbarInSelectInput21Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Xbar1XbarInSelectInput21Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Xbar1XbarInSelectInput21Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Xbar1XbarInSelectInput21 {
    #[inline(always)]
    fn default() -> Xbar1XbarInSelectInput21 {
        Xbar1XbarInSelectInput21(0u64 as u32)
    }
}
impl core::fmt::Debug for Xbar1XbarInSelectInput21 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xbar1XbarInSelectInput21")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xbar1XbarInSelectInput21 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Xbar1XbarInSelectInput21 {
            daisy: super::vals::Xbar1XbarInSelectInput21Daisy,
        }
        let proxy = Xbar1XbarInSelectInput21 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "XBAR1_XBAR_IN_SELECT_INPUT_22 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbar1XbarInSelectInput22(pub u32);
impl Xbar1XbarInSelectInput22 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Xbar1XbarInSelectInput22Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Xbar1XbarInSelectInput22Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Xbar1XbarInSelectInput22Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Xbar1XbarInSelectInput22 {
    #[inline(always)]
    fn default() -> Xbar1XbarInSelectInput22 {
        Xbar1XbarInSelectInput22(0u64 as u32)
    }
}
impl core::fmt::Debug for Xbar1XbarInSelectInput22 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xbar1XbarInSelectInput22")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xbar1XbarInSelectInput22 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Xbar1XbarInSelectInput22 {
            daisy: super::vals::Xbar1XbarInSelectInput22Daisy,
        }
        let proxy = Xbar1XbarInSelectInput22 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "XBAR1_XBAR_IN_SELECT_INPUT_23 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbar1XbarInSelectInput23(pub u32);
impl Xbar1XbarInSelectInput23 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Xbar1XbarInSelectInput23Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Xbar1XbarInSelectInput23Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Xbar1XbarInSelectInput23Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Xbar1XbarInSelectInput23 {
    #[inline(always)]
    fn default() -> Xbar1XbarInSelectInput23 {
        Xbar1XbarInSelectInput23(0u64 as u32)
    }
}
impl core::fmt::Debug for Xbar1XbarInSelectInput23 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xbar1XbarInSelectInput23")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xbar1XbarInSelectInput23 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Xbar1XbarInSelectInput23 {
            daisy: super::vals::Xbar1XbarInSelectInput23Daisy,
        }
        let proxy = Xbar1XbarInSelectInput23 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "XBAR1_XBAR_IN_SELECT_INPUT_24 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbar1XbarInSelectInput24(pub u32);
impl Xbar1XbarInSelectInput24 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Xbar1XbarInSelectInput24Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Xbar1XbarInSelectInput24Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Xbar1XbarInSelectInput24Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Xbar1XbarInSelectInput24 {
    #[inline(always)]
    fn default() -> Xbar1XbarInSelectInput24 {
        Xbar1XbarInSelectInput24(0u64 as u32)
    }
}
impl core::fmt::Debug for Xbar1XbarInSelectInput24 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xbar1XbarInSelectInput24")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xbar1XbarInSelectInput24 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Xbar1XbarInSelectInput24 {
            daisy: super::vals::Xbar1XbarInSelectInput24Daisy,
        }
        let proxy = Xbar1XbarInSelectInput24 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "XBAR1_XBAR_IN_SELECT_INPUT_25 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbar1XbarInSelectInput25(pub u32);
impl Xbar1XbarInSelectInput25 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Xbar1XbarInSelectInput25Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Xbar1XbarInSelectInput25Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Xbar1XbarInSelectInput25Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Xbar1XbarInSelectInput25 {
    #[inline(always)]
    fn default() -> Xbar1XbarInSelectInput25 {
        Xbar1XbarInSelectInput25(0u64 as u32)
    }
}
impl core::fmt::Debug for Xbar1XbarInSelectInput25 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xbar1XbarInSelectInput25")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xbar1XbarInSelectInput25 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Xbar1XbarInSelectInput25 {
            daisy: super::vals::Xbar1XbarInSelectInput25Daisy,
        }
        let proxy = Xbar1XbarInSelectInput25 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "XBAR1_XBAR_IN_SELECT_INPUT_26 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbar1XbarInSelectInput26(pub u32);
impl Xbar1XbarInSelectInput26 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Xbar1XbarInSelectInput26Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Xbar1XbarInSelectInput26Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Xbar1XbarInSelectInput26Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Xbar1XbarInSelectInput26 {
    #[inline(always)]
    fn default() -> Xbar1XbarInSelectInput26 {
        Xbar1XbarInSelectInput26(0u64 as u32)
    }
}
impl core::fmt::Debug for Xbar1XbarInSelectInput26 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xbar1XbarInSelectInput26")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xbar1XbarInSelectInput26 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Xbar1XbarInSelectInput26 {
            daisy: super::vals::Xbar1XbarInSelectInput26Daisy,
        }
        let proxy = Xbar1XbarInSelectInput26 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "XBAR1_XBAR_IN_SELECT_INPUT_27 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbar1XbarInSelectInput27(pub u32);
impl Xbar1XbarInSelectInput27 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Xbar1XbarInSelectInput27Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Xbar1XbarInSelectInput27Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Xbar1XbarInSelectInput27Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Xbar1XbarInSelectInput27 {
    #[inline(always)]
    fn default() -> Xbar1XbarInSelectInput27 {
        Xbar1XbarInSelectInput27(0u64 as u32)
    }
}
impl core::fmt::Debug for Xbar1XbarInSelectInput27 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xbar1XbarInSelectInput27")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xbar1XbarInSelectInput27 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Xbar1XbarInSelectInput27 {
            daisy: super::vals::Xbar1XbarInSelectInput27Daisy,
        }
        let proxy = Xbar1XbarInSelectInput27 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "XBAR1_XBAR_IN_SELECT_INPUT_28 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbar1XbarInSelectInput28(pub u32);
impl Xbar1XbarInSelectInput28 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Xbar1XbarInSelectInput28Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Xbar1XbarInSelectInput28Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Xbar1XbarInSelectInput28Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Xbar1XbarInSelectInput28 {
    #[inline(always)]
    fn default() -> Xbar1XbarInSelectInput28 {
        Xbar1XbarInSelectInput28(0u64 as u32)
    }
}
impl core::fmt::Debug for Xbar1XbarInSelectInput28 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xbar1XbarInSelectInput28")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xbar1XbarInSelectInput28 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Xbar1XbarInSelectInput28 {
            daisy: super::vals::Xbar1XbarInSelectInput28Daisy,
        }
        let proxy = Xbar1XbarInSelectInput28 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "XBAR1_XBAR_IN_SELECT_INPUT_29 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbar1XbarInSelectInput29(pub u32);
impl Xbar1XbarInSelectInput29 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Xbar1XbarInSelectInput29Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Xbar1XbarInSelectInput29Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Xbar1XbarInSelectInput29Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Xbar1XbarInSelectInput29 {
    #[inline(always)]
    fn default() -> Xbar1XbarInSelectInput29 {
        Xbar1XbarInSelectInput29(0u64 as u32)
    }
}
impl core::fmt::Debug for Xbar1XbarInSelectInput29 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xbar1XbarInSelectInput29")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xbar1XbarInSelectInput29 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Xbar1XbarInSelectInput29 {
            daisy: super::vals::Xbar1XbarInSelectInput29Daisy,
        }
        let proxy = Xbar1XbarInSelectInput29 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "XBAR1_XBAR_IN_SELECT_INPUT_30 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbar1XbarInSelectInput30(pub u32);
impl Xbar1XbarInSelectInput30 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Xbar1XbarInSelectInput30Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Xbar1XbarInSelectInput30Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Xbar1XbarInSelectInput30Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Xbar1XbarInSelectInput30 {
    #[inline(always)]
    fn default() -> Xbar1XbarInSelectInput30 {
        Xbar1XbarInSelectInput30(0u64 as u32)
    }
}
impl core::fmt::Debug for Xbar1XbarInSelectInput30 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xbar1XbarInSelectInput30")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xbar1XbarInSelectInput30 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Xbar1XbarInSelectInput30 {
            daisy: super::vals::Xbar1XbarInSelectInput30Daisy,
        }
        let proxy = Xbar1XbarInSelectInput30 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "XBAR1_XBAR_IN_SELECT_INPUT_31 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbar1XbarInSelectInput31(pub u32);
impl Xbar1XbarInSelectInput31 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Xbar1XbarInSelectInput31Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Xbar1XbarInSelectInput31Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Xbar1XbarInSelectInput31Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Xbar1XbarInSelectInput31 {
    #[inline(always)]
    fn default() -> Xbar1XbarInSelectInput31 {
        Xbar1XbarInSelectInput31(0u64 as u32)
    }
}
impl core::fmt::Debug for Xbar1XbarInSelectInput31 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xbar1XbarInSelectInput31")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xbar1XbarInSelectInput31 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Xbar1XbarInSelectInput31 {
            daisy: super::vals::Xbar1XbarInSelectInput31Daisy,
        }
        let proxy = Xbar1XbarInSelectInput31 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "XBAR1_XBAR_IN_SELECT_INPUT_32 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbar1XbarInSelectInput32(pub u32);
impl Xbar1XbarInSelectInput32 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Xbar1XbarInSelectInput32Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Xbar1XbarInSelectInput32Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Xbar1XbarInSelectInput32Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Xbar1XbarInSelectInput32 {
    #[inline(always)]
    fn default() -> Xbar1XbarInSelectInput32 {
        Xbar1XbarInSelectInput32(0u64 as u32)
    }
}
impl core::fmt::Debug for Xbar1XbarInSelectInput32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xbar1XbarInSelectInput32")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xbar1XbarInSelectInput32 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Xbar1XbarInSelectInput32 {
            daisy: super::vals::Xbar1XbarInSelectInput32Daisy,
        }
        let proxy = Xbar1XbarInSelectInput32 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "XBAR1_XBAR_IN_SELECT_INPUT_33 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbar1XbarInSelectInput33(pub u32);
impl Xbar1XbarInSelectInput33 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Xbar1XbarInSelectInput33Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Xbar1XbarInSelectInput33Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Xbar1XbarInSelectInput33Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Xbar1XbarInSelectInput33 {
    #[inline(always)]
    fn default() -> Xbar1XbarInSelectInput33 {
        Xbar1XbarInSelectInput33(0u64 as u32)
    }
}
impl core::fmt::Debug for Xbar1XbarInSelectInput33 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xbar1XbarInSelectInput33")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xbar1XbarInSelectInput33 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Xbar1XbarInSelectInput33 {
            daisy: super::vals::Xbar1XbarInSelectInput33Daisy,
        }
        let proxy = Xbar1XbarInSelectInput33 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "XBAR1_XBAR_IN_SELECT_INPUT_34 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbar1XbarInSelectInput34(pub u32);
impl Xbar1XbarInSelectInput34 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Xbar1XbarInSelectInput34Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Xbar1XbarInSelectInput34Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Xbar1XbarInSelectInput34Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Xbar1XbarInSelectInput34 {
    #[inline(always)]
    fn default() -> Xbar1XbarInSelectInput34 {
        Xbar1XbarInSelectInput34(0u64 as u32)
    }
}
impl core::fmt::Debug for Xbar1XbarInSelectInput34 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xbar1XbarInSelectInput34")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xbar1XbarInSelectInput34 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Xbar1XbarInSelectInput34 {
            daisy: super::vals::Xbar1XbarInSelectInput34Daisy,
        }
        let proxy = Xbar1XbarInSelectInput34 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "XBAR1_XBAR_IN_SELECT_INPUT_35 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbar1XbarInSelectInput35(pub u32);
impl Xbar1XbarInSelectInput35 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Xbar1XbarInSelectInput35Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Xbar1XbarInSelectInput35Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Xbar1XbarInSelectInput35Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Xbar1XbarInSelectInput35 {
    #[inline(always)]
    fn default() -> Xbar1XbarInSelectInput35 {
        Xbar1XbarInSelectInput35(0u64 as u32)
    }
}
impl core::fmt::Debug for Xbar1XbarInSelectInput35 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xbar1XbarInSelectInput35")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xbar1XbarInSelectInput35 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Xbar1XbarInSelectInput35 {
            daisy: super::vals::Xbar1XbarInSelectInput35Daisy,
        }
        let proxy = Xbar1XbarInSelectInput35 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "XBAR1_XBAR_IN_SELECT_INPUT_36 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbar1XbarInSelectInput36(pub u32);
impl Xbar1XbarInSelectInput36 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Xbar1XbarInSelectInput36Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Xbar1XbarInSelectInput36Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Xbar1XbarInSelectInput36Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Xbar1XbarInSelectInput36 {
    #[inline(always)]
    fn default() -> Xbar1XbarInSelectInput36 {
        Xbar1XbarInSelectInput36(0u64 as u32)
    }
}
impl core::fmt::Debug for Xbar1XbarInSelectInput36 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xbar1XbarInSelectInput36")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xbar1XbarInSelectInput36 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Xbar1XbarInSelectInput36 {
            daisy: super::vals::Xbar1XbarInSelectInput36Daisy,
        }
        let proxy = Xbar1XbarInSelectInput36 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "XBAR1_XBAR_IN_SELECT_INPUT_37 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbar1XbarInSelectInput37(pub u32);
impl Xbar1XbarInSelectInput37 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Xbar1XbarInSelectInput37Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Xbar1XbarInSelectInput37Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Xbar1XbarInSelectInput37Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Xbar1XbarInSelectInput37 {
    #[inline(always)]
    fn default() -> Xbar1XbarInSelectInput37 {
        Xbar1XbarInSelectInput37(0u64 as u32)
    }
}
impl core::fmt::Debug for Xbar1XbarInSelectInput37 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xbar1XbarInSelectInput37")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xbar1XbarInSelectInput37 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Xbar1XbarInSelectInput37 {
            daisy: super::vals::Xbar1XbarInSelectInput37Daisy,
        }
        let proxy = Xbar1XbarInSelectInput37 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "XSPI_SLV_IPP_IND_CS_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XspiSlvIppIndCsSelectInput(pub u32);
impl XspiSlvIppIndCsSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::XspiSlvIppIndCsSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::XspiSlvIppIndCsSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::XspiSlvIppIndCsSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for XspiSlvIppIndCsSelectInput {
    #[inline(always)]
    fn default() -> XspiSlvIppIndCsSelectInput {
        XspiSlvIppIndCsSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for XspiSlvIppIndCsSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XspiSlvIppIndCsSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for XspiSlvIppIndCsSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct XspiSlvIppIndCsSelectInput {
            daisy: super::vals::XspiSlvIppIndCsSelectInputDaisy,
        }
        let proxy = XspiSlvIppIndCsSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "XSPI_SLV_IPP_IND_DQS_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XspiSlvIppIndDqsSelectInput(pub u32);
impl XspiSlvIppIndDqsSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::XspiSlvIppIndDqsSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::XspiSlvIppIndDqsSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::XspiSlvIppIndDqsSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for XspiSlvIppIndDqsSelectInput {
    #[inline(always)]
    fn default() -> XspiSlvIppIndDqsSelectInput {
        XspiSlvIppIndDqsSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for XspiSlvIppIndDqsSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XspiSlvIppIndDqsSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for XspiSlvIppIndDqsSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct XspiSlvIppIndDqsSelectInput {
            daisy: super::vals::XspiSlvIppIndDqsSelectInputDaisy,
        }
        let proxy = XspiSlvIppIndDqsSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "XSPI_SLV_IPP_IND_IO_SELECT_INPUT_0 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XspiSlvIppIndIoSelectInput0(pub u32);
impl XspiSlvIppIndIoSelectInput0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::XspiSlvIppIndIoSelectInput0Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::XspiSlvIppIndIoSelectInput0Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::XspiSlvIppIndIoSelectInput0Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for XspiSlvIppIndIoSelectInput0 {
    #[inline(always)]
    fn default() -> XspiSlvIppIndIoSelectInput0 {
        XspiSlvIppIndIoSelectInput0(0u64 as u32)
    }
}
impl core::fmt::Debug for XspiSlvIppIndIoSelectInput0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XspiSlvIppIndIoSelectInput0")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for XspiSlvIppIndIoSelectInput0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct XspiSlvIppIndIoSelectInput0 {
            daisy: super::vals::XspiSlvIppIndIoSelectInput0Daisy,
        }
        let proxy = XspiSlvIppIndIoSelectInput0 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "XSPI_SLV_IPP_IND_IO_SELECT_INPUT_1 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XspiSlvIppIndIoSelectInput1(pub u32);
impl XspiSlvIppIndIoSelectInput1 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::XspiSlvIppIndIoSelectInput1Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::XspiSlvIppIndIoSelectInput1Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::XspiSlvIppIndIoSelectInput1Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for XspiSlvIppIndIoSelectInput1 {
    #[inline(always)]
    fn default() -> XspiSlvIppIndIoSelectInput1 {
        XspiSlvIppIndIoSelectInput1(0u64 as u32)
    }
}
impl core::fmt::Debug for XspiSlvIppIndIoSelectInput1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XspiSlvIppIndIoSelectInput1")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for XspiSlvIppIndIoSelectInput1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct XspiSlvIppIndIoSelectInput1 {
            daisy: super::vals::XspiSlvIppIndIoSelectInput1Daisy,
        }
        let proxy = XspiSlvIppIndIoSelectInput1 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "XSPI_SLV_IPP_IND_IO_SELECT_INPUT_2 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XspiSlvIppIndIoSelectInput2(pub u32);
impl XspiSlvIppIndIoSelectInput2 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::XspiSlvIppIndIoSelectInput2Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::XspiSlvIppIndIoSelectInput2Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::XspiSlvIppIndIoSelectInput2Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for XspiSlvIppIndIoSelectInput2 {
    #[inline(always)]
    fn default() -> XspiSlvIppIndIoSelectInput2 {
        XspiSlvIppIndIoSelectInput2(0u64 as u32)
    }
}
impl core::fmt::Debug for XspiSlvIppIndIoSelectInput2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XspiSlvIppIndIoSelectInput2")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for XspiSlvIppIndIoSelectInput2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct XspiSlvIppIndIoSelectInput2 {
            daisy: super::vals::XspiSlvIppIndIoSelectInput2Daisy,
        }
        let proxy = XspiSlvIppIndIoSelectInput2 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "XSPI_SLV_IPP_IND_IO_SELECT_INPUT_3 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XspiSlvIppIndIoSelectInput3(pub u32);
impl XspiSlvIppIndIoSelectInput3 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::XspiSlvIppIndIoSelectInput3Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::XspiSlvIppIndIoSelectInput3Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::XspiSlvIppIndIoSelectInput3Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for XspiSlvIppIndIoSelectInput3 {
    #[inline(always)]
    fn default() -> XspiSlvIppIndIoSelectInput3 {
        XspiSlvIppIndIoSelectInput3(0u64 as u32)
    }
}
impl core::fmt::Debug for XspiSlvIppIndIoSelectInput3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XspiSlvIppIndIoSelectInput3")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for XspiSlvIppIndIoSelectInput3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct XspiSlvIppIndIoSelectInput3 {
            daisy: super::vals::XspiSlvIppIndIoSelectInput3Daisy,
        }
        let proxy = XspiSlvIppIndIoSelectInput3 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "XSPI_SLV_IPP_IND_IO_SELECT_INPUT_4 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XspiSlvIppIndIoSelectInput4(pub u32);
impl XspiSlvIppIndIoSelectInput4 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::XspiSlvIppIndIoSelectInput4Daisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::XspiSlvIppIndIoSelectInput4Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::XspiSlvIppIndIoSelectInput4Daisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for XspiSlvIppIndIoSelectInput4 {
    #[inline(always)]
    fn default() -> XspiSlvIppIndIoSelectInput4 {
        XspiSlvIppIndIoSelectInput4(0u64 as u32)
    }
}
impl core::fmt::Debug for XspiSlvIppIndIoSelectInput4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XspiSlvIppIndIoSelectInput4")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for XspiSlvIppIndIoSelectInput4 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct XspiSlvIppIndIoSelectInput4 {
            daisy: super::vals::XspiSlvIppIndIoSelectInput4Daisy,
        }
        let proxy = XspiSlvIppIndIoSelectInput4 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "XSPI_SLV_IPP_IND_IO_SELECT_INPUT_5 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XspiSlvIppIndIoSelectInput5(pub u32);
impl XspiSlvIppIndIoSelectInput5 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::XspiSlvIppIndIoSelectInput5Daisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::XspiSlvIppIndIoSelectInput5Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::XspiSlvIppIndIoSelectInput5Daisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for XspiSlvIppIndIoSelectInput5 {
    #[inline(always)]
    fn default() -> XspiSlvIppIndIoSelectInput5 {
        XspiSlvIppIndIoSelectInput5(0u64 as u32)
    }
}
impl core::fmt::Debug for XspiSlvIppIndIoSelectInput5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XspiSlvIppIndIoSelectInput5")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for XspiSlvIppIndIoSelectInput5 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct XspiSlvIppIndIoSelectInput5 {
            daisy: super::vals::XspiSlvIppIndIoSelectInput5Daisy,
        }
        let proxy = XspiSlvIppIndIoSelectInput5 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "XSPI_SLV_IPP_IND_IO_SELECT_INPUT_6 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XspiSlvIppIndIoSelectInput6(pub u32);
impl XspiSlvIppIndIoSelectInput6 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::XspiSlvIppIndIoSelectInput6Daisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::XspiSlvIppIndIoSelectInput6Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::XspiSlvIppIndIoSelectInput6Daisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for XspiSlvIppIndIoSelectInput6 {
    #[inline(always)]
    fn default() -> XspiSlvIppIndIoSelectInput6 {
        XspiSlvIppIndIoSelectInput6(0u64 as u32)
    }
}
impl core::fmt::Debug for XspiSlvIppIndIoSelectInput6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XspiSlvIppIndIoSelectInput6")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for XspiSlvIppIndIoSelectInput6 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct XspiSlvIppIndIoSelectInput6 {
            daisy: super::vals::XspiSlvIppIndIoSelectInput6Daisy,
        }
        let proxy = XspiSlvIppIndIoSelectInput6 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "XSPI_SLV_IPP_IND_IO_SELECT_INPUT_7 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XspiSlvIppIndIoSelectInput7(pub u32);
impl XspiSlvIppIndIoSelectInput7 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::XspiSlvIppIndIoSelectInput7Daisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::XspiSlvIppIndIoSelectInput7Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::XspiSlvIppIndIoSelectInput7Daisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for XspiSlvIppIndIoSelectInput7 {
    #[inline(always)]
    fn default() -> XspiSlvIppIndIoSelectInput7 {
        XspiSlvIppIndIoSelectInput7(0u64 as u32)
    }
}
impl core::fmt::Debug for XspiSlvIppIndIoSelectInput7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XspiSlvIppIndIoSelectInput7")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for XspiSlvIppIndIoSelectInput7 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct XspiSlvIppIndIoSelectInput7 {
            daisy: super::vals::XspiSlvIppIndIoSelectInput7Daisy,
        }
        let proxy = XspiSlvIppIndIoSelectInput7 {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "XSPI_SLV_IPP_IND_SCK_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XspiSlvIppIndSckSelectInput(pub u32);
impl XspiSlvIppIndSckSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::XspiSlvIppIndSckSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::XspiSlvIppIndSckSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::XspiSlvIppIndSckSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for XspiSlvIppIndSckSelectInput {
    #[inline(always)]
    fn default() -> XspiSlvIppIndSckSelectInput {
        XspiSlvIppIndSckSelectInput(0u64 as u32)
    }
}
impl core::fmt::Debug for XspiSlvIppIndSckSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XspiSlvIppIndSckSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for XspiSlvIppIndSckSelectInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct XspiSlvIppIndSckSelectInput {
            daisy: super::vals::XspiSlvIppIndSckSelectInputDaisy,
        }
        let proxy = XspiSlvIppIndSckSelectInput {
            daisy: self.daisy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
