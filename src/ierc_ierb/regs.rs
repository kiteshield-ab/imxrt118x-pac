#[doc = "Function 0 EC config header device ID and vendor ID register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct F0EcCfhDidvid(pub u32);
impl F0EcCfhDidvid {
    #[doc = "Vendor ID This field identifies the manufacturer of the device as shown in the PCIe Vendor ID Register (00h)"]
    #[must_use]
    #[inline(always)]
    pub const fn vendor_id(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Vendor ID This field identifies the manufacturer of the device as shown in the PCIe Vendor ID Register (00h)"]
    #[inline(always)]
    pub const fn set_vendor_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Device ID This field identifies the device ID of the device shown in the PCIe Device ID Register (02h)"]
    #[must_use]
    #[inline(always)]
    pub const fn device_id(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Device ID This field identifies the device ID of the device shown in the PCIe Device ID Register (02h)"]
    #[inline(always)]
    pub const fn set_device_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for F0EcCfhDidvid {
    #[inline(always)]
    fn default() -> F0EcCfhDidvid {
        F0EcCfhDidvid(3758168407u64 as u32)
    }
}
impl core::fmt::Debug for F0EcCfhDidvid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("F0EcCfhDidvid")
            .field("vendor_id", &self.vendor_id())
            .field("device_id", &self.device_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for F0EcCfhDidvid {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct F0EcCfhDidvid {
            vendor_id: u16,
            device_id: u16,
        }
        let proxy = F0EcCfhDidvid {
            vendor_id: self.vendor_id(),
            device_id: self.device_id(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Function 0 EC config header subsystem ID and subsystem vendor ID register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct F0EcCfhSidsvid(pub u32);
impl F0EcCfhSidsvid {
    #[doc = "Subsystem Vendor ID This field identifies the manufacturer of the subsystem as shown in the PCIe Subsystem Vendor ID Register (2Ch)"]
    #[must_use]
    #[inline(always)]
    pub const fn subsystem_vendor_id(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Subsystem Vendor ID This field identifies the manufacturer of the subsystem as shown in the PCIe Subsystem Vendor ID Register (2Ch)"]
    #[inline(always)]
    pub const fn set_subsystem_vendor_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Subsystem Device ID This field identifies the particular subsystem as shown in the PCIe Subsystem ID Register (2Eh)"]
    #[must_use]
    #[inline(always)]
    pub const fn subsystem_device_id(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Subsystem Device ID This field identifies the particular subsystem as shown in the PCIe Subsystem ID Register (2Eh)"]
    #[inline(always)]
    pub const fn set_subsystem_device_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for F0EcCfhSidsvid {
    #[inline(always)]
    fn default() -> F0EcCfhSidsvid {
        F0EcCfhSidsvid(3758168407u64 as u32)
    }
}
impl core::fmt::Debug for F0EcCfhSidsvid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("F0EcCfhSidsvid")
            .field("subsystem_vendor_id", &self.subsystem_vendor_id())
            .field("subsystem_device_id", &self.subsystem_device_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for F0EcCfhSidsvid {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct F0EcCfhSidsvid {
            subsystem_vendor_id: u16,
            subsystem_device_id: u16,
        }
        let proxy = F0EcCfhSidsvid {
            subsystem_vendor_id: self.subsystem_vendor_id(),
            subsystem_device_id: self.subsystem_device_id(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
