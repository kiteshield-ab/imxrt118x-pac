#[doc = "AL Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AlControl(pub u16);
impl AlControl {
    #[doc = "Initiate State Transition of the Device State Machine:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Initiate State Transition of the Device State Machine:"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "Error Ind Ack"]
    #[must_use]
    #[inline(always)]
    pub const fn bf4(&self) -> super::vals::AlControlBf4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::AlControlBf4::from_bits(val as u8)
    }
    #[doc = "Error Ind Ack"]
    #[inline(always)]
    pub const fn set_bf4(&mut self, val: super::vals::AlControlBf4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Device Identification:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf5(&self) -> super::vals::AlControlBf5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::AlControlBf5::from_bits(val as u8)
    }
    #[doc = "Device Identification:"]
    #[inline(always)]
    pub const fn set_bf5(&mut self, val: super::vals::AlControlBf5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf6(&self) -> u16 {
        let val = (self.0 >> 6usize) & 0x03ff;
        val as u16
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf6(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 6usize)) | (((val as u16) & 0x03ff) << 6usize);
    }
}
impl Default for AlControl {
    #[inline(always)]
    fn default() -> AlControl {
        AlControl(1u64 as u16)
    }
}
impl core::fmt::Debug for AlControl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AlControl")
            .field("bf0", &self.bf0())
            .field("bf4", &self.bf4())
            .field("bf5", &self.bf5())
            .field("bf6", &self.bf6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AlControl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct AlControl {
            bf0: u8,
            bf4: super::vals::AlControlBf4,
            bf5: super::vals::AlControlBf5,
            bf6: u16,
        }
        let proxy = AlControl {
            bf0: self.bf0(),
            bf4: self.bf4(),
            bf5: self.bf5(),
            bf6: self.bf6(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "AL Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AlControlPdi(pub u16);
impl AlControlPdi {
    #[doc = "Initiate State Transition of the Device State Machine:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Initiate State Transition of the Device State Machine:"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "Error Ind Ack"]
    #[must_use]
    #[inline(always)]
    pub const fn bf4(&self) -> super::vals::AlControlPdiBf4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::AlControlPdiBf4::from_bits(val as u8)
    }
    #[doc = "Error Ind Ack"]
    #[inline(always)]
    pub const fn set_bf4(&mut self, val: super::vals::AlControlPdiBf4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Device Identification:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf5(&self) -> super::vals::AlControlPdiBf5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::AlControlPdiBf5::from_bits(val as u8)
    }
    #[doc = "Device Identification:"]
    #[inline(always)]
    pub const fn set_bf5(&mut self, val: super::vals::AlControlPdiBf5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf6(&self) -> u16 {
        let val = (self.0 >> 6usize) & 0x03ff;
        val as u16
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf6(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 6usize)) | (((val as u16) & 0x03ff) << 6usize);
    }
}
impl Default for AlControlPdi {
    #[inline(always)]
    fn default() -> AlControlPdi {
        AlControlPdi(1u64 as u16)
    }
}
impl core::fmt::Debug for AlControlPdi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AlControlPdi")
            .field("bf0", &self.bf0())
            .field("bf4", &self.bf4())
            .field("bf5", &self.bf5())
            .field("bf6", &self.bf6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AlControlPdi {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct AlControlPdi {
            bf0: u8,
            bf4: super::vals::AlControlPdiBf4,
            bf5: super::vals::AlControlPdiBf5,
            bf6: u16,
        }
        let proxy = AlControlPdi {
            bf0: self.bf0(),
            bf4: self.bf4(),
            bf5: self.bf5(),
            bf6: self.bf6(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "AL Event request"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AlEventRequest(pub u32);
impl AlEventRequest {
    #[doc = "AL Control event:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::AlEventRequestBf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::AlEventRequestBf0::from_bits(val as u8)
    }
    #[doc = "AL Control event:"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::AlEventRequestBf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "DC Latch event:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> super::vals::AlEventRequestBf1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::AlEventRequestBf1::from_bits(val as u8)
    }
    #[doc = "DC Latch event:"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: super::vals::AlEventRequestBf1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "State of DC SYNC0 (if register 0x0151\\[3\\]=1):"]
    #[must_use]
    #[inline(always)]
    pub const fn bf2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "State of DC SYNC0 (if register 0x0151\\[3\\]=1):"]
    #[inline(always)]
    pub const fn set_bf2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "State of DC SYNC1 (if register 0x0151\\[7\\]=1):"]
    #[must_use]
    #[inline(always)]
    pub const fn bf3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "State of DC SYNC1 (if register 0x0151\\[7\\]=1):"]
    #[inline(always)]
    pub const fn set_bf3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "SyncManager activation register (SyncManager register offset 0x6) changed:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf4(&self) -> super::vals::AlEventRequestBf4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::AlEventRequestBf4::from_bits(val as u8)
    }
    #[doc = "SyncManager activation register (SyncManager register offset 0x6) changed:"]
    #[inline(always)]
    pub const fn set_bf4(&mut self, val: super::vals::AlEventRequestBf4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "EEPROM Emulation:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf5(&self) -> super::vals::AlEventRequestBf5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::AlEventRequestBf5::from_bits(val as u8)
    }
    #[doc = "EEPROM Emulation:"]
    #[inline(always)]
    pub const fn set_bf5(&mut self, val: super::vals::AlEventRequestBf5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Watchdog Process Data:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf6(&self) -> super::vals::AlEventRequestBf6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::AlEventRequestBf6::from_bits(val as u8)
    }
    #[doc = "Watchdog Process Data:"]
    #[inline(always)]
    pub const fn set_bf6(&mut self, val: super::vals::AlEventRequestBf6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn bf7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_bf7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "SyncManager interrupts (SyncManager register offset 0x5, bit \\[0\\] or \\[1\\]):"]
    #[must_use]
    #[inline(always)]
    pub const fn bf8(&self) -> super::vals::AlEventRequestBf8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::AlEventRequestBf8::from_bits(val as u8)
    }
    #[doc = "SyncManager interrupts (SyncManager register offset 0x5, bit \\[0\\] or \\[1\\]):"]
    #[inline(always)]
    pub const fn set_bf8(&mut self, val: super::vals::AlEventRequestBf8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Bit field access for ECAT: r/-"]
    #[must_use]
    #[inline(always)]
    pub const fn bf9(&self) -> super::vals::AlEventRequestBf9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::AlEventRequestBf9::from_bits(val as u8)
    }
    #[doc = "Bit field access for ECAT: r/-"]
    #[inline(always)]
    pub const fn set_bf9(&mut self, val: super::vals::AlEventRequestBf9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Bit field access for ECAT: r/-"]
    #[must_use]
    #[inline(always)]
    pub const fn bf10(&self) -> super::vals::AlEventRequestBf10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::AlEventRequestBf10::from_bits(val as u8)
    }
    #[doc = "Bit field access for ECAT: r/-"]
    #[inline(always)]
    pub const fn set_bf10(&mut self, val: super::vals::AlEventRequestBf10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Bit field access for ECAT: r/-"]
    #[must_use]
    #[inline(always)]
    pub const fn bf11(&self) -> super::vals::AlEventRequestBf11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::AlEventRequestBf11::from_bits(val as u8)
    }
    #[doc = "Bit field access for ECAT: r/-"]
    #[inline(always)]
    pub const fn set_bf11(&mut self, val: super::vals::AlEventRequestBf11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Bit field access for ECAT: r/-"]
    #[must_use]
    #[inline(always)]
    pub const fn bf12(&self) -> super::vals::AlEventRequestBf12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::AlEventRequestBf12::from_bits(val as u8)
    }
    #[doc = "Bit field access for ECAT: r/-"]
    #[inline(always)]
    pub const fn set_bf12(&mut self, val: super::vals::AlEventRequestBf12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Bit field access for ECAT: r/-"]
    #[must_use]
    #[inline(always)]
    pub const fn bf13(&self) -> super::vals::AlEventRequestBf13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::AlEventRequestBf13::from_bits(val as u8)
    }
    #[doc = "Bit field access for ECAT: r/-"]
    #[inline(always)]
    pub const fn set_bf13(&mut self, val: super::vals::AlEventRequestBf13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Bit field access for ECAT: r/-"]
    #[must_use]
    #[inline(always)]
    pub const fn bf14(&self) -> super::vals::AlEventRequestBf14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::AlEventRequestBf14::from_bits(val as u8)
    }
    #[doc = "Bit field access for ECAT: r/-"]
    #[inline(always)]
    pub const fn set_bf14(&mut self, val: super::vals::AlEventRequestBf14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Bit field access for ECAT: r/-"]
    #[must_use]
    #[inline(always)]
    pub const fn bf15(&self) -> super::vals::AlEventRequestBf15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::AlEventRequestBf15::from_bits(val as u8)
    }
    #[doc = "Bit field access for ECAT: r/-"]
    #[inline(always)]
    pub const fn set_bf15(&mut self, val: super::vals::AlEventRequestBf15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn bf16(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_bf16(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for AlEventRequest {
    #[inline(always)]
    fn default() -> AlEventRequest {
        AlEventRequest(32u64 as u32)
    }
}
impl core::fmt::Debug for AlEventRequest {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AlEventRequest")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .field("bf2", &self.bf2())
            .field("bf3", &self.bf3())
            .field("bf4", &self.bf4())
            .field("bf5", &self.bf5())
            .field("bf6", &self.bf6())
            .field("bf7", &self.bf7())
            .field("bf8", &self.bf8())
            .field("bf9", &self.bf9())
            .field("bf10", &self.bf10())
            .field("bf11", &self.bf11())
            .field("bf12", &self.bf12())
            .field("bf13", &self.bf13())
            .field("bf14", &self.bf14())
            .field("bf15", &self.bf15())
            .field("bf16", &self.bf16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AlEventRequest {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct AlEventRequest {
            bf0: super::vals::AlEventRequestBf0,
            bf1: super::vals::AlEventRequestBf1,
            bf2: bool,
            bf3: bool,
            bf4: super::vals::AlEventRequestBf4,
            bf5: super::vals::AlEventRequestBf5,
            bf6: super::vals::AlEventRequestBf6,
            bf7: bool,
            bf8: super::vals::AlEventRequestBf8,
            bf9: super::vals::AlEventRequestBf9,
            bf10: super::vals::AlEventRequestBf10,
            bf11: super::vals::AlEventRequestBf11,
            bf12: super::vals::AlEventRequestBf12,
            bf13: super::vals::AlEventRequestBf13,
            bf14: super::vals::AlEventRequestBf14,
            bf15: super::vals::AlEventRequestBf15,
            bf16: u16,
        }
        let proxy = AlEventRequest {
            bf0: self.bf0(),
            bf1: self.bf1(),
            bf2: self.bf2(),
            bf3: self.bf3(),
            bf4: self.bf4(),
            bf5: self.bf5(),
            bf6: self.bf6(),
            bf7: self.bf7(),
            bf8: self.bf8(),
            bf9: self.bf9(),
            bf10: self.bf10(),
            bf11: self.bf11(),
            bf12: self.bf12(),
            bf13: self.bf13(),
            bf14: self.bf14(),
            bf15: self.bf15(),
            bf16: self.bf16(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "AL Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AlStatus(pub u16);
impl AlStatus {
    #[doc = "Actual State of the Device State Machine:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Actual State of the Device State Machine:"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "Error Ind:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf4(&self) -> super::vals::AlStatusBf4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::AlStatusBf4::from_bits(val as u8)
    }
    #[doc = "Error Ind:"]
    #[inline(always)]
    pub const fn set_bf4(&mut self, val: super::vals::AlStatusBf4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Device Identification:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf5(&self) -> super::vals::AlStatusBf5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::AlStatusBf5::from_bits(val as u8)
    }
    #[doc = "Device Identification:"]
    #[inline(always)]
    pub const fn set_bf5(&mut self, val: super::vals::AlStatusBf5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf6(&self) -> u16 {
        let val = (self.0 >> 6usize) & 0x03ff;
        val as u16
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf6(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 6usize)) | (((val as u16) & 0x03ff) << 6usize);
    }
}
impl Default for AlStatus {
    #[inline(always)]
    fn default() -> AlStatus {
        AlStatus(1u64 as u16)
    }
}
impl core::fmt::Debug for AlStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AlStatus")
            .field("bf0", &self.bf0())
            .field("bf4", &self.bf4())
            .field("bf5", &self.bf5())
            .field("bf6", &self.bf6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AlStatus {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct AlStatus {
            bf0: u8,
            bf4: super::vals::AlStatusBf4,
            bf5: super::vals::AlStatusBf5,
            bf6: u16,
        }
        let proxy = AlStatus {
            bf0: self.bf0(),
            bf4: self.bf4(),
            bf5: self.bf5(),
            bf6: self.bf6(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "AL Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AlStatusPdi(pub u16);
impl AlStatusPdi {
    #[doc = "Actual State of the Device State Machine:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Actual State of the Device State Machine:"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "Error Ind:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf4(&self) -> super::vals::AlStatusPdiBf4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::AlStatusPdiBf4::from_bits(val as u8)
    }
    #[doc = "Error Ind:"]
    #[inline(always)]
    pub const fn set_bf4(&mut self, val: super::vals::AlStatusPdiBf4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Device Identification:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf5(&self) -> super::vals::AlStatusPdiBf5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::AlStatusPdiBf5::from_bits(val as u8)
    }
    #[doc = "Device Identification:"]
    #[inline(always)]
    pub const fn set_bf5(&mut self, val: super::vals::AlStatusPdiBf5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf6(&self) -> u16 {
        let val = (self.0 >> 6usize) & 0x03ff;
        val as u16
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf6(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 6usize)) | (((val as u16) & 0x03ff) << 6usize);
    }
}
impl Default for AlStatusPdi {
    #[inline(always)]
    fn default() -> AlStatusPdi {
        AlStatusPdi(1u64 as u16)
    }
}
impl core::fmt::Debug for AlStatusPdi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AlStatusPdi")
            .field("bf0", &self.bf0())
            .field("bf4", &self.bf4())
            .field("bf5", &self.bf5())
            .field("bf6", &self.bf6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AlStatusPdi {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct AlStatusPdi {
            bf0: u8,
            bf4: super::vals::AlStatusPdiBf4,
            bf5: super::vals::AlStatusPdiBf5,
            bf6: u16,
        }
        let proxy = AlStatusPdi {
            bf0: self.bf0(),
            bf4: self.bf4(),
            bf5: self.bf5(),
            bf6: self.bf6(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ASYNCHRONOUS_SYNCHRONOUS_MICROCONTROLLER_PDI_ERROR_CODE."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AsynchronousSynchronousMicrocontroller(pub u8);
impl AsynchronousSynchronousMicrocontroller {
    #[doc = "Busy violation during read access"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::AsynchronousSynchronousMicrocontrollerBf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::AsynchronousSynchronousMicrocontrollerBf0::from_bits(val as u8)
    }
    #[doc = "Busy violation during read access"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::AsynchronousSynchronousMicrocontrollerBf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Busy violation during write access"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> super::vals::AsynchronousSynchronousMicrocontrollerBf1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::AsynchronousSynchronousMicrocontrollerBf1::from_bits(val as u8)
    }
    #[doc = "Busy violation during write access"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: super::vals::AsynchronousSynchronousMicrocontrollerBf1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "Addressing error for a read access (odd address without BHE)"]
    #[must_use]
    #[inline(always)]
    pub const fn bf2(&self) -> super::vals::AsynchronousSynchronousMicrocontrollerBf2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::AsynchronousSynchronousMicrocontrollerBf2::from_bits(val as u8)
    }
    #[doc = "Addressing error for a read access (odd address without BHE)"]
    #[inline(always)]
    pub const fn set_bf2(&mut self, val: super::vals::AsynchronousSynchronousMicrocontrollerBf2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u8) & 0x01) << 2usize);
    }
    #[doc = "Addressing error for a write access (odd address without BHE)"]
    #[must_use]
    #[inline(always)]
    pub const fn bf3(&self) -> super::vals::AsynchronousSynchronousMicrocontrollerBf3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::AsynchronousSynchronousMicrocontrollerBf3::from_bits(val as u8)
    }
    #[doc = "Addressing error for a write access (odd address without BHE)"]
    #[inline(always)]
    pub const fn set_bf3(&mut self, val: super::vals::AsynchronousSynchronousMicrocontrollerBf3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn bf4(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub const fn set_bf4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for AsynchronousSynchronousMicrocontroller {
    #[inline(always)]
    fn default() -> AsynchronousSynchronousMicrocontroller {
        AsynchronousSynchronousMicrocontroller(0u64 as u8)
    }
}
impl core::fmt::Debug for AsynchronousSynchronousMicrocontroller {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AsynchronousSynchronousMicrocontroller")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .field("bf2", &self.bf2())
            .field("bf3", &self.bf3())
            .field("bf4", &self.bf4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AsynchronousSynchronousMicrocontroller {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct AsynchronousSynchronousMicrocontroller {
            bf0: super::vals::AsynchronousSynchronousMicrocontrollerBf0,
            bf1: super::vals::AsynchronousSynchronousMicrocontrollerBf1,
            bf2: super::vals::AsynchronousSynchronousMicrocontrollerBf2,
            bf3: super::vals::AsynchronousSynchronousMicrocontrollerBf3,
            bf4: u8,
        }
        let proxy = AsynchronousSynchronousMicrocontroller {
            bf0: self.bf0(),
            bf1: self.bf1(),
            bf2: self.bf2(),
            bf3: self.bf3(),
            bf4: self.bf4(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Register Cyclic Unit Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CyclicUnitControl(pub u8);
impl CyclicUnitControl {
    #[doc = "SYNC out unit control:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::CyclicUnitControlBf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::CyclicUnitControlBf0::from_bits(val as u8)
    }
    #[doc = "SYNC out unit control:"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::CyclicUnitControlBf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u8) & 0x07) << 1usize);
    }
    #[doc = "Latch In unit 0:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf4(&self) -> super::vals::CyclicUnitControlBf4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::CyclicUnitControlBf4::from_bits(val as u8)
    }
    #[doc = "Latch In unit 0:"]
    #[inline(always)]
    pub const fn set_bf4(&mut self, val: super::vals::CyclicUnitControlBf4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
    #[doc = "Latch In unit 1:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf5(&self) -> super::vals::CyclicUnitControlBf5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::CyclicUnitControlBf5::from_bits(val as u8)
    }
    #[doc = "Latch In unit 1:"]
    #[inline(always)]
    pub const fn set_bf5(&mut self, val: super::vals::CyclicUnitControlBf5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf6(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
    }
}
impl Default for CyclicUnitControl {
    #[inline(always)]
    fn default() -> CyclicUnitControl {
        CyclicUnitControl(16u64 as u8)
    }
}
impl core::fmt::Debug for CyclicUnitControl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CyclicUnitControl")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .field("bf4", &self.bf4())
            .field("bf5", &self.bf5())
            .field("bf6", &self.bf6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CyclicUnitControl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct CyclicUnitControl {
            bf0: super::vals::CyclicUnitControlBf0,
            bf1: u8,
            bf4: super::vals::CyclicUnitControlBf4,
            bf5: super::vals::CyclicUnitControlBf5,
            bf6: u8,
        }
        let proxy = CyclicUnitControl {
            bf0: self.bf0(),
            bf1: self.bf1(),
            bf4: self.bf4(),
            bf5: self.bf5(),
            bf6: self.bf6(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Register Cyclic Unit Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CyclicUnitControlPdi(pub u8);
impl CyclicUnitControlPdi {
    #[doc = "SYNC out unit control:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::CyclicUnitControlPdiBf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::CyclicUnitControlPdiBf0::from_bits(val as u8)
    }
    #[doc = "SYNC out unit control:"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::CyclicUnitControlPdiBf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u8) & 0x07) << 1usize);
    }
    #[doc = "Latch In unit 0:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf4(&self) -> super::vals::CyclicUnitControlPdiBf4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::CyclicUnitControlPdiBf4::from_bits(val as u8)
    }
    #[doc = "Latch In unit 0:"]
    #[inline(always)]
    pub const fn set_bf4(&mut self, val: super::vals::CyclicUnitControlPdiBf4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
    #[doc = "Latch In unit 1:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf5(&self) -> super::vals::CyclicUnitControlPdiBf5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::CyclicUnitControlPdiBf5::from_bits(val as u8)
    }
    #[doc = "Latch In unit 1:"]
    #[inline(always)]
    pub const fn set_bf5(&mut self, val: super::vals::CyclicUnitControlPdiBf5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf6(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
    }
}
impl Default for CyclicUnitControlPdi {
    #[inline(always)]
    fn default() -> CyclicUnitControlPdi {
        CyclicUnitControlPdi(16u64 as u8)
    }
}
impl core::fmt::Debug for CyclicUnitControlPdi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CyclicUnitControlPdi")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .field("bf4", &self.bf4())
            .field("bf5", &self.bf5())
            .field("bf6", &self.bf6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CyclicUnitControlPdi {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct CyclicUnitControlPdi {
            bf0: super::vals::CyclicUnitControlPdiBf0,
            bf1: u8,
            bf4: super::vals::CyclicUnitControlPdiBf4,
            bf5: super::vals::CyclicUnitControlPdiBf5,
            bf6: u8,
        }
        let proxy = CyclicUnitControlPdi {
            bf0: self.bf0(),
            bf1: self.bf1(),
            bf4: self.bf4(),
            bf5: self.bf5(),
            bf6: self.bf6(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ECAT Event Mask"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EcatEventMask(pub u16);
impl EcatEventMask {
    #[doc = "ECAT Event masking of the ECAT Event Request Events for mapping into ECAT event field of EtherCAT frames:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::EcatEventMaskBf0 {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::EcatEventMaskBf0::from_bits(val as u16)
    }
    #[doc = "ECAT Event masking of the ECAT Event Request Events for mapping into ECAT event field of EtherCAT frames:"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::EcatEventMaskBf0) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u16) & 0xffff) << 0usize);
    }
}
impl Default for EcatEventMask {
    #[inline(always)]
    fn default() -> EcatEventMask {
        EcatEventMask(0u64 as u16)
    }
}
impl core::fmt::Debug for EcatEventMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EcatEventMask")
            .field("bf0", &self.bf0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EcatEventMask {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EcatEventMask {
            bf0: super::vals::EcatEventMaskBf0,
        }
        let proxy = EcatEventMask { bf0: self.bf0() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ECAT Event Mask"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EcatEventMaskPdi(pub u16);
impl EcatEventMaskPdi {
    #[doc = "ECAT Event masking of the ECAT Event Request Events for mapping into ECAT event field of EtherCAT frames:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::EcatEventMaskPdiBf0 {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::EcatEventMaskPdiBf0::from_bits(val as u16)
    }
    #[doc = "ECAT Event masking of the ECAT Event Request Events for mapping into ECAT event field of EtherCAT frames:"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::EcatEventMaskPdiBf0) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u16) & 0xffff) << 0usize);
    }
}
impl Default for EcatEventMaskPdi {
    #[inline(always)]
    fn default() -> EcatEventMaskPdi {
        EcatEventMaskPdi(0u64 as u16)
    }
}
impl core::fmt::Debug for EcatEventMaskPdi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EcatEventMaskPdi")
            .field("bf0", &self.bf0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EcatEventMaskPdi {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EcatEventMaskPdi {
            bf0: super::vals::EcatEventMaskPdiBf0,
        }
        let proxy = EcatEventMaskPdi { bf0: self.bf0() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ECAT Event Request"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EcatEventRequest(pub u16);
impl EcatEventRequest {
    #[doc = "DC Latch event:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::EcatEventRequestBf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::EcatEventRequestBf0::from_bits(val as u8)
    }
    #[doc = "DC Latch event:"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::EcatEventRequestBf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "DL Status event:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf2(&self) -> super::vals::EcatEventRequestBf2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::EcatEventRequestBf2::from_bits(val as u8)
    }
    #[doc = "DL Status event:"]
    #[inline(always)]
    pub const fn set_bf2(&mut self, val: super::vals::EcatEventRequestBf2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "AL Status event:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf3(&self) -> super::vals::EcatEventRequestBf3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::EcatEventRequestBf3::from_bits(val as u8)
    }
    #[doc = "AL Status event:"]
    #[inline(always)]
    pub const fn set_bf3(&mut self, val: super::vals::EcatEventRequestBf3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
    #[doc = "Mirrors values of each SyncManager Status:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf4(&self) -> super::vals::EcatEventRequestBf4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::EcatEventRequestBf4::from_bits(val as u8)
    }
    #[doc = "Mirrors values of each SyncManager Status:"]
    #[inline(always)]
    pub const fn set_bf4(&mut self, val: super::vals::EcatEventRequestBf4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Mirrors values of each SyncManager Status:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf5(&self) -> super::vals::EcatEventRequestBf5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::EcatEventRequestBf5::from_bits(val as u8)
    }
    #[doc = "Mirrors values of each SyncManager Status:"]
    #[inline(always)]
    pub const fn set_bf5(&mut self, val: super::vals::EcatEventRequestBf5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
    }
    #[doc = "Mirrors values of each SyncManager Status:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf6(&self) -> super::vals::EcatEventRequestBf6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::EcatEventRequestBf6::from_bits(val as u8)
    }
    #[doc = "Mirrors values of each SyncManager Status:"]
    #[inline(always)]
    pub const fn set_bf6(&mut self, val: super::vals::EcatEventRequestBf6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Mirrors values of each SyncManager Status:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf7(&self) -> super::vals::EcatEventRequestBf7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::EcatEventRequestBf7::from_bits(val as u8)
    }
    #[doc = "Mirrors values of each SyncManager Status:"]
    #[inline(always)]
    pub const fn set_bf7(&mut self, val: super::vals::EcatEventRequestBf7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "Mirrors values of each SyncManager Status:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf8(&self) -> super::vals::EcatEventRequestBf8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::EcatEventRequestBf8::from_bits(val as u8)
    }
    #[doc = "Mirrors values of each SyncManager Status:"]
    #[inline(always)]
    pub const fn set_bf8(&mut self, val: super::vals::EcatEventRequestBf8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "Mirrors values of each SyncManager Status:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf9(&self) -> super::vals::EcatEventRequestBf9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::EcatEventRequestBf9::from_bits(val as u8)
    }
    #[doc = "Mirrors values of each SyncManager Status:"]
    #[inline(always)]
    pub const fn set_bf9(&mut self, val: super::vals::EcatEventRequestBf9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
    }
    #[doc = "Mirrors values of each SyncManager Status:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf10(&self) -> super::vals::EcatEventRequestBf10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::EcatEventRequestBf10::from_bits(val as u8)
    }
    #[doc = "Mirrors values of each SyncManager Status:"]
    #[inline(always)]
    pub const fn set_bf10(&mut self, val: super::vals::EcatEventRequestBf10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u16) & 0x01) << 10usize);
    }
    #[doc = "Mirrors values of each SyncManager Status:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf11(&self) -> super::vals::EcatEventRequestBf11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::EcatEventRequestBf11::from_bits(val as u8)
    }
    #[doc = "Mirrors values of each SyncManager Status:"]
    #[inline(always)]
    pub const fn set_bf11(&mut self, val: super::vals::EcatEventRequestBf11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u16) & 0x01) << 11usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn bf12(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_bf12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u16) & 0x0f) << 12usize);
    }
}
impl Default for EcatEventRequest {
    #[inline(always)]
    fn default() -> EcatEventRequest {
        EcatEventRequest(4u64 as u16)
    }
}
impl core::fmt::Debug for EcatEventRequest {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EcatEventRequest")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .field("bf2", &self.bf2())
            .field("bf3", &self.bf3())
            .field("bf4", &self.bf4())
            .field("bf5", &self.bf5())
            .field("bf6", &self.bf6())
            .field("bf7", &self.bf7())
            .field("bf8", &self.bf8())
            .field("bf9", &self.bf9())
            .field("bf10", &self.bf10())
            .field("bf11", &self.bf11())
            .field("bf12", &self.bf12())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EcatEventRequest {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EcatEventRequest {
            bf0: super::vals::EcatEventRequestBf0,
            bf1: bool,
            bf2: super::vals::EcatEventRequestBf2,
            bf3: super::vals::EcatEventRequestBf3,
            bf4: super::vals::EcatEventRequestBf4,
            bf5: super::vals::EcatEventRequestBf5,
            bf6: super::vals::EcatEventRequestBf6,
            bf7: super::vals::EcatEventRequestBf7,
            bf8: super::vals::EcatEventRequestBf8,
            bf9: super::vals::EcatEventRequestBf9,
            bf10: super::vals::EcatEventRequestBf10,
            bf11: super::vals::EcatEventRequestBf11,
            bf12: u8,
        }
        let proxy = EcatEventRequest {
            bf0: self.bf0(),
            bf1: self.bf1(),
            bf2: self.bf2(),
            bf3: self.bf3(),
            bf4: self.bf4(),
            bf5: self.bf5(),
            bf6: self.bf6(),
            bf7: self.bf7(),
            bf8: self.bf8(),
            bf9: self.bf9(),
            bf10: self.bf10(),
            bf11: self.bf11(),
            bf12: self.bf12(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "EEPROM Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EepromConfiguration(pub u8);
impl EepromConfiguration {
    #[doc = "EEPROM control is offered to PDI"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::EepromConfigurationBf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::EepromConfigurationBf0::from_bits(val as u8)
    }
    #[doc = "EEPROM control is offered to PDI"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::EepromConfigurationBf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Force ECAT access"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> super::vals::EepromConfigurationBf1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::EepromConfigurationBf1::from_bits(val as u8)
    }
    #[doc = "Force ECAT access"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: super::vals::EepromConfigurationBf1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf2(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u8) & 0x3f) << 2usize);
    }
}
impl Default for EepromConfiguration {
    #[inline(always)]
    fn default() -> EepromConfiguration {
        EepromConfiguration(0u64 as u8)
    }
}
impl core::fmt::Debug for EepromConfiguration {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EepromConfiguration")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .field("bf2", &self.bf2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EepromConfiguration {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EepromConfiguration {
            bf0: super::vals::EepromConfigurationBf0,
            bf1: super::vals::EepromConfigurationBf1,
            bf2: u8,
        }
        let proxy = EepromConfiguration {
            bf0: self.bf0(),
            bf1: self.bf1(),
            bf2: self.bf2(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "EEPROM Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EepromConfigurationPdi(pub u8);
impl EepromConfigurationPdi {
    #[doc = "EEPROM control is offered to PDI"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::EepromConfigurationPdiBf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::EepromConfigurationPdiBf0::from_bits(val as u8)
    }
    #[doc = "EEPROM control is offered to PDI"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::EepromConfigurationPdiBf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Force ECAT access"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> super::vals::EepromConfigurationPdiBf1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::EepromConfigurationPdiBf1::from_bits(val as u8)
    }
    #[doc = "Force ECAT access"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: super::vals::EepromConfigurationPdiBf1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf2(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u8) & 0x3f) << 2usize);
    }
}
impl Default for EepromConfigurationPdi {
    #[inline(always)]
    fn default() -> EepromConfigurationPdi {
        EepromConfigurationPdi(0u64 as u8)
    }
}
impl core::fmt::Debug for EepromConfigurationPdi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EepromConfigurationPdi")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .field("bf2", &self.bf2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EepromConfigurationPdi {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EepromConfigurationPdi {
            bf0: super::vals::EepromConfigurationPdiBf0,
            bf1: super::vals::EepromConfigurationPdiBf1,
            bf2: u8,
        }
        let proxy = EepromConfigurationPdi {
            bf0: self.bf0(),
            bf1: self.bf1(),
            bf2: self.bf2(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Register EEPROM Control/Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EepromControlStatus(pub u16);
impl EepromControlStatus {
    #[doc = "ECAT write enable2"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::EepromControlStatusBf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::EepromControlStatusBf0::from_bits(val as u8)
    }
    #[doc = "ECAT write enable2"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::EepromControlStatusBf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x0f;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 1usize)) | (((val as u16) & 0x0f) << 1usize);
    }
    #[doc = "EEPROM emulation:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf5(&self) -> super::vals::EepromControlStatusBf5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::EepromControlStatusBf5::from_bits(val as u8)
    }
    #[doc = "EEPROM emulation:"]
    #[inline(always)]
    pub const fn set_bf5(&mut self, val: super::vals::EepromControlStatusBf5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
    }
    #[doc = "Supported number of EEPROM read bytes:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf6(&self) -> super::vals::EepromControlStatusBf6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::EepromControlStatusBf6::from_bits(val as u8)
    }
    #[doc = "Supported number of EEPROM read bytes:"]
    #[inline(always)]
    pub const fn set_bf6(&mut self, val: super::vals::EepromControlStatusBf6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Selected EEPROM Algorithm:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf7(&self) -> super::vals::EepromControlStatusBf7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::EepromControlStatusBf7::from_bits(val as u8)
    }
    #[doc = "Selected EEPROM Algorithm:"]
    #[inline(always)]
    pub const fn set_bf7(&mut self, val: super::vals::EepromControlStatusBf7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "Command register"]
    #[must_use]
    #[inline(always)]
    pub const fn bf8(&self) -> super::vals::EepromControlStatusBf8 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::EepromControlStatusBf8::from_bits(val as u8)
    }
    #[doc = "Command register"]
    #[inline(always)]
    pub const fn set_bf8(&mut self, val: super::vals::EepromControlStatusBf8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u16) & 0x07) << 8usize);
    }
    #[doc = "Checksum Error in ESC Configuration Area:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf11(&self) -> super::vals::EepromControlStatusBf11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::EepromControlStatusBf11::from_bits(val as u8)
    }
    #[doc = "Checksum Error in ESC Configuration Area:"]
    #[inline(always)]
    pub const fn set_bf11(&mut self, val: super::vals::EepromControlStatusBf11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u16) & 0x01) << 11usize);
    }
    #[doc = "EEPROM loading status:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf12(&self) -> super::vals::EepromControlStatusBf12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::EepromControlStatusBf12::from_bits(val as u8)
    }
    #[doc = "EEPROM loading status:"]
    #[inline(always)]
    pub const fn set_bf12(&mut self, val: super::vals::EepromControlStatusBf12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
    #[doc = "Error Acknowledge/Command3:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf13(&self) -> super::vals::EepromControlStatusBf13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::EepromControlStatusBf13::from_bits(val as u8)
    }
    #[doc = "Error Acknowledge/Command3:"]
    #[inline(always)]
    pub const fn set_bf13(&mut self, val: super::vals::EepromControlStatusBf13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "Error Write Enable3:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf14(&self) -> super::vals::EepromControlStatusBf14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::EepromControlStatusBf14::from_bits(val as u8)
    }
    #[doc = "Error Write Enable3:"]
    #[inline(always)]
    pub const fn set_bf14(&mut self, val: super::vals::EepromControlStatusBf14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "Busy:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf15(&self) -> super::vals::EepromControlStatusBf15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::EepromControlStatusBf15::from_bits(val as u8)
    }
    #[doc = "Busy:"]
    #[inline(always)]
    pub const fn set_bf15(&mut self, val: super::vals::EepromControlStatusBf15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
    }
}
impl Default for EepromControlStatus {
    #[inline(always)]
    fn default() -> EepromControlStatus {
        EepromControlStatus(37952u64 as u16)
    }
}
impl core::fmt::Debug for EepromControlStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EepromControlStatus")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .field("bf5", &self.bf5())
            .field("bf6", &self.bf6())
            .field("bf7", &self.bf7())
            .field("bf8", &self.bf8())
            .field("bf11", &self.bf11())
            .field("bf12", &self.bf12())
            .field("bf13", &self.bf13())
            .field("bf14", &self.bf14())
            .field("bf15", &self.bf15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EepromControlStatus {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EepromControlStatus {
            bf0: super::vals::EepromControlStatusBf0,
            bf1: u8,
            bf5: super::vals::EepromControlStatusBf5,
            bf6: super::vals::EepromControlStatusBf6,
            bf7: super::vals::EepromControlStatusBf7,
            bf8: super::vals::EepromControlStatusBf8,
            bf11: super::vals::EepromControlStatusBf11,
            bf12: super::vals::EepromControlStatusBf12,
            bf13: super::vals::EepromControlStatusBf13,
            bf14: super::vals::EepromControlStatusBf14,
            bf15: super::vals::EepromControlStatusBf15,
        }
        let proxy = EepromControlStatus {
            bf0: self.bf0(),
            bf1: self.bf1(),
            bf5: self.bf5(),
            bf6: self.bf6(),
            bf7: self.bf7(),
            bf8: self.bf8(),
            bf11: self.bf11(),
            bf12: self.bf12(),
            bf13: self.bf13(),
            bf14: self.bf14(),
            bf15: self.bf15(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Register EEPROM Control/Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EepromControlStatusPdi(pub u16);
impl EepromControlStatusPdi {
    #[doc = "ECAT write enable2"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::EepromControlStatusPdiBf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::EepromControlStatusPdiBf0::from_bits(val as u8)
    }
    #[doc = "ECAT write enable2"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::EepromControlStatusPdiBf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x0f;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 1usize)) | (((val as u16) & 0x0f) << 1usize);
    }
    #[doc = "EEPROM emulation:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf5(&self) -> super::vals::EepromControlStatusPdiBf5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::EepromControlStatusPdiBf5::from_bits(val as u8)
    }
    #[doc = "EEPROM emulation:"]
    #[inline(always)]
    pub const fn set_bf5(&mut self, val: super::vals::EepromControlStatusPdiBf5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
    }
    #[doc = "Supported number of EEPROM read bytes:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf6(&self) -> super::vals::EepromControlStatusPdiBf6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::EepromControlStatusPdiBf6::from_bits(val as u8)
    }
    #[doc = "Supported number of EEPROM read bytes:"]
    #[inline(always)]
    pub const fn set_bf6(&mut self, val: super::vals::EepromControlStatusPdiBf6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Selected EEPROM Algorithm:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf7(&self) -> super::vals::EepromControlStatusPdiBf7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::EepromControlStatusPdiBf7::from_bits(val as u8)
    }
    #[doc = "Selected EEPROM Algorithm:"]
    #[inline(always)]
    pub const fn set_bf7(&mut self, val: super::vals::EepromControlStatusPdiBf7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "Command register"]
    #[must_use]
    #[inline(always)]
    pub const fn bf8(&self) -> super::vals::EepromControlStatusPdiBf8 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::EepromControlStatusPdiBf8::from_bits(val as u8)
    }
    #[doc = "Command register"]
    #[inline(always)]
    pub const fn set_bf8(&mut self, val: super::vals::EepromControlStatusPdiBf8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u16) & 0x07) << 8usize);
    }
    #[doc = "Checksum Error in ESC Configuration Area:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf11(&self) -> super::vals::EepromControlStatusPdiBf11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::EepromControlStatusPdiBf11::from_bits(val as u8)
    }
    #[doc = "Checksum Error in ESC Configuration Area:"]
    #[inline(always)]
    pub const fn set_bf11(&mut self, val: super::vals::EepromControlStatusPdiBf11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u16) & 0x01) << 11usize);
    }
    #[doc = "EEPROM loading status:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf12(&self) -> super::vals::EepromControlStatusPdiBf12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::EepromControlStatusPdiBf12::from_bits(val as u8)
    }
    #[doc = "EEPROM loading status:"]
    #[inline(always)]
    pub const fn set_bf12(&mut self, val: super::vals::EepromControlStatusPdiBf12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
    #[doc = "Error Acknowledge/Command3:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf13(&self) -> super::vals::EepromControlStatusPdiBf13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::EepromControlStatusPdiBf13::from_bits(val as u8)
    }
    #[doc = "Error Acknowledge/Command3:"]
    #[inline(always)]
    pub const fn set_bf13(&mut self, val: super::vals::EepromControlStatusPdiBf13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "Error Write Enable3:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf14(&self) -> super::vals::EepromControlStatusPdiBf14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::EepromControlStatusPdiBf14::from_bits(val as u8)
    }
    #[doc = "Error Write Enable3:"]
    #[inline(always)]
    pub const fn set_bf14(&mut self, val: super::vals::EepromControlStatusPdiBf14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "Busy:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf15(&self) -> super::vals::EepromControlStatusPdiBf15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::EepromControlStatusPdiBf15::from_bits(val as u8)
    }
    #[doc = "Busy:"]
    #[inline(always)]
    pub const fn set_bf15(&mut self, val: super::vals::EepromControlStatusPdiBf15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
    }
}
impl Default for EepromControlStatusPdi {
    #[inline(always)]
    fn default() -> EepromControlStatusPdi {
        EepromControlStatusPdi(37952u64 as u16)
    }
}
impl core::fmt::Debug for EepromControlStatusPdi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EepromControlStatusPdi")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .field("bf5", &self.bf5())
            .field("bf6", &self.bf6())
            .field("bf7", &self.bf7())
            .field("bf8", &self.bf8())
            .field("bf11", &self.bf11())
            .field("bf12", &self.bf12())
            .field("bf13", &self.bf13())
            .field("bf14", &self.bf14())
            .field("bf15", &self.bf15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EepromControlStatusPdi {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EepromControlStatusPdi {
            bf0: super::vals::EepromControlStatusPdiBf0,
            bf1: u8,
            bf5: super::vals::EepromControlStatusPdiBf5,
            bf6: super::vals::EepromControlStatusPdiBf6,
            bf7: super::vals::EepromControlStatusPdiBf7,
            bf8: super::vals::EepromControlStatusPdiBf8,
            bf11: super::vals::EepromControlStatusPdiBf11,
            bf12: super::vals::EepromControlStatusPdiBf12,
            bf13: super::vals::EepromControlStatusPdiBf13,
            bf14: super::vals::EepromControlStatusPdiBf14,
            bf15: super::vals::EepromControlStatusPdiBf15,
        }
        let proxy = EepromControlStatusPdi {
            bf0: self.bf0(),
            bf1: self.bf1(),
            bf5: self.bf5(),
            bf6: self.bf6(),
            bf7: self.bf7(),
            bf8: self.bf8(),
            bf11: self.bf11(),
            bf12: self.bf12(),
            bf13: self.bf13(),
            bf14: self.bf14(),
            bf15: self.bf15(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "EEPROM Data"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EepromData(pub u64);
impl EepromData {
    #[doc = "EEPROM Write data (data to be written to EEPROM) or EEPROM Read data (data read from EEPROM,. lower bytes)"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "EEPROM Write data (data to be written to EEPROM) or EEPROM Read data (data read from EEPROM,. lower bytes)"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u64) & 0xffff) << 0usize);
    }
    #[doc = "EEPROM Read data (data read from EEPROM, higher bytes)"]
    #[must_use]
    #[inline(always)]
    pub const fn bf16(&self) -> u64 {
        let val = (self.0 >> 16usize) & 0xffff_ffff_ffff;
        val as u64
    }
    #[doc = "EEPROM Read data (data read from EEPROM, higher bytes)"]
    #[inline(always)]
    pub const fn set_bf16(&mut self, val: u64) {
        self.0 = (self.0 & !(0xffff_ffff_ffff << 16usize))
            | (((val as u64) & 0xffff_ffff_ffff) << 16usize);
    }
}
impl Default for EepromData {
    #[inline(always)]
    fn default() -> EepromData {
        EepromData(0u64 as u64)
    }
}
impl core::fmt::Debug for EepromData {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EepromData")
            .field("bf0", &self.bf0())
            .field("bf16", &self.bf16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EepromData {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EepromData {
            bf0: u16,
            bf16: u64,
        }
        let proxy = EepromData {
            bf0: self.bf0(),
            bf16: self.bf16(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ERR LED Override"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ErrLedOverride(pub u8);
impl ErrLedOverride {
    #[doc = "LED code"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "LED code"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Enable Override:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf4(&self) -> super::vals::ErrLedOverrideBf4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::ErrLedOverrideBf4::from_bits(val as u8)
    }
    #[doc = "Enable Override:"]
    #[inline(always)]
    pub const fn set_bf4(&mut self, val: super::vals::ErrLedOverrideBf4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf5(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u8) & 0x07) << 5usize);
    }
}
impl Default for ErrLedOverride {
    #[inline(always)]
    fn default() -> ErrLedOverride {
        ErrLedOverride(0u64 as u8)
    }
}
impl core::fmt::Debug for ErrLedOverride {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ErrLedOverride")
            .field("bf0", &self.bf0())
            .field("bf4", &self.bf4())
            .field("bf5", &self.bf5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ErrLedOverride {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ErrLedOverride {
            bf0: u8,
            bf4: super::vals::ErrLedOverrideBf4,
            bf5: u8,
        }
        let proxy = ErrLedOverride {
            bf0: self.bf0(),
            bf4: self.bf4(),
            bf5: self.bf5(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ESC Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EscConfiguration(pub u8);
impl EscConfiguration {
    #[doc = "Device emulation (control of AL status):"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::EscConfigurationBf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::EscConfigurationBf0::from_bits(val as u8)
    }
    #[doc = "Device emulation (control of AL status):"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::EscConfigurationBf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Enhanced Link detection all ports:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> super::vals::EscConfigurationBf1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::EscConfigurationBf1::from_bits(val as u8)
    }
    #[doc = "Enhanced Link detection all ports:"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: super::vals::EscConfigurationBf1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "Distributed Clocks SYNC Out Unit:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf2(&self) -> super::vals::EscConfigurationBf2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::EscConfigurationBf2::from_bits(val as u8)
    }
    #[doc = "Distributed Clocks SYNC Out Unit:"]
    #[inline(always)]
    pub const fn set_bf2(&mut self, val: super::vals::EscConfigurationBf2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u8) & 0x01) << 2usize);
    }
    #[doc = "Distributed Clocks Latch In Unit:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf3(&self) -> super::vals::EscConfigurationBf3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::EscConfigurationBf3::from_bits(val as u8)
    }
    #[doc = "Distributed Clocks Latch In Unit:"]
    #[inline(always)]
    pub const fn set_bf3(&mut self, val: super::vals::EscConfigurationBf3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "Enhanced Link port 0:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf4(&self) -> super::vals::EscConfigurationBf4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::EscConfigurationBf4::from_bits(val as u8)
    }
    #[doc = "Enhanced Link port 0:"]
    #[inline(always)]
    pub const fn set_bf4(&mut self, val: super::vals::EscConfigurationBf4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
    #[doc = "Enhanced Link port 1:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf5(&self) -> super::vals::EscConfigurationBf5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::EscConfigurationBf5::from_bits(val as u8)
    }
    #[doc = "Enhanced Link port 1:"]
    #[inline(always)]
    pub const fn set_bf5(&mut self, val: super::vals::EscConfigurationBf5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn bf6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_bf6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn bf7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_bf7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for EscConfiguration {
    #[inline(always)]
    fn default() -> EscConfiguration {
        EscConfiguration(254u64 as u8)
    }
}
impl core::fmt::Debug for EscConfiguration {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EscConfiguration")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .field("bf2", &self.bf2())
            .field("bf3", &self.bf3())
            .field("bf4", &self.bf4())
            .field("bf5", &self.bf5())
            .field("bf6", &self.bf6())
            .field("bf7", &self.bf7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EscConfiguration {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EscConfiguration {
            bf0: super::vals::EscConfigurationBf0,
            bf1: super::vals::EscConfigurationBf1,
            bf2: super::vals::EscConfigurationBf2,
            bf3: super::vals::EscConfigurationBf3,
            bf4: super::vals::EscConfigurationBf4,
            bf5: super::vals::EscConfigurationBf5,
            bf6: bool,
            bf7: bool,
        }
        let proxy = EscConfiguration {
            bf0: self.bf0(),
            bf1: self.bf1(),
            bf2: self.bf2(),
            bf3: self.bf3(),
            bf4: self.bf4(),
            bf5: self.bf5(),
            bf6: self.bf6(),
            bf7: self.bf7(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ESC DL Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EscDlControl(pub u32);
impl EscDlControl {
    #[doc = "Forwarding Rule"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::EscDlControlBf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::EscDlControlBf0::from_bits(val as u8)
    }
    #[doc = "Forwarding Rule"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::EscDlControlBf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Temporary use of settings in 0x0100:0x0103\\[8:15\\]:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> super::vals::EscDlControlBf1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::EscDlControlBf1::from_bits(val as u8)
    }
    #[doc = "Temporary use of settings in 0x0100:0x0103\\[8:15\\]:"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: super::vals::EscDlControlBf1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Reserved, Write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf2(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[doc = "Reserved, Write 0"]
    #[inline(always)]
    pub const fn set_bf2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u32) & 0x3f) << 2usize);
    }
    #[doc = "Loop Port 0:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf8(&self) -> super::vals::EscDlControlBf8 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::EscDlControlBf8::from_bits(val as u8)
    }
    #[doc = "Loop Port 0:"]
    #[inline(always)]
    pub const fn set_bf8(&mut self, val: super::vals::EscDlControlBf8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Loop Port 1:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf10(&self) -> super::vals::EscDlControlBf10 {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::EscDlControlBf10::from_bits(val as u8)
    }
    #[doc = "Loop Port 1:"]
    #[inline(always)]
    pub const fn set_bf10(&mut self, val: super::vals::EscDlControlBf10) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn bf12(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_bf12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn bf14(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_bf14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "RX FIFO Size (ESC delays start of forwarding until FIFO is at least half full)."]
    #[must_use]
    #[inline(always)]
    pub const fn bf16(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "RX FIFO Size (ESC delays start of forwarding until FIFO is at least half full)."]
    #[inline(always)]
    pub const fn set_bf16(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf20(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf20(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Station alias:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf24(&self) -> super::vals::EscDlControlBf24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::EscDlControlBf24::from_bits(val as u8)
    }
    #[doc = "Station alias:"]
    #[inline(always)]
    pub const fn set_bf24(&mut self, val: super::vals::EscDlControlBf24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf25(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x7f;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf25(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
    }
}
impl Default for EscDlControl {
    #[inline(always)]
    fn default() -> EscDlControl {
        EscDlControl(507905u64 as u32)
    }
}
impl core::fmt::Debug for EscDlControl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EscDlControl")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .field("bf2", &self.bf2())
            .field("bf8", &self.bf8())
            .field("bf10", &self.bf10())
            .field("bf12", &self.bf12())
            .field("bf14", &self.bf14())
            .field("bf16", &self.bf16())
            .field("bf20", &self.bf20())
            .field("bf23", &self.bf23())
            .field("bf24", &self.bf24())
            .field("bf25", &self.bf25())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EscDlControl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EscDlControl {
            bf0: super::vals::EscDlControlBf0,
            bf1: super::vals::EscDlControlBf1,
            bf2: u8,
            bf8: super::vals::EscDlControlBf8,
            bf10: super::vals::EscDlControlBf10,
            bf12: u8,
            bf14: u8,
            bf16: u8,
            bf20: u8,
            bf23: bool,
            bf24: super::vals::EscDlControlBf24,
            bf25: u8,
        }
        let proxy = EscDlControl {
            bf0: self.bf0(),
            bf1: self.bf1(),
            bf2: self.bf2(),
            bf8: self.bf8(),
            bf10: self.bf10(),
            bf12: self.bf12(),
            bf14: self.bf14(),
            bf16: self.bf16(),
            bf20: self.bf20(),
            bf23: self.bf23(),
            bf24: self.bf24(),
            bf25: self.bf25(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ESC DL Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EscDlControlPdi(pub u32);
impl EscDlControlPdi {
    #[doc = "Forwarding Rule"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::EscDlControlPdiBf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::EscDlControlPdiBf0::from_bits(val as u8)
    }
    #[doc = "Forwarding Rule"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::EscDlControlPdiBf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Temporary use of settings in 0x0100:0x0103\\[8:15\\]:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> super::vals::EscDlControlPdiBf1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::EscDlControlPdiBf1::from_bits(val as u8)
    }
    #[doc = "Temporary use of settings in 0x0100:0x0103\\[8:15\\]:"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: super::vals::EscDlControlPdiBf1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Reserved, Write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf2(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[doc = "Reserved, Write 0"]
    #[inline(always)]
    pub const fn set_bf2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u32) & 0x3f) << 2usize);
    }
    #[doc = "Loop Port 0:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf8(&self) -> super::vals::EscDlControlPdiBf8 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::EscDlControlPdiBf8::from_bits(val as u8)
    }
    #[doc = "Loop Port 0:"]
    #[inline(always)]
    pub const fn set_bf8(&mut self, val: super::vals::EscDlControlPdiBf8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Loop Port 1:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf10(&self) -> super::vals::EscDlControlPdiBf10 {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::EscDlControlPdiBf10::from_bits(val as u8)
    }
    #[doc = "Loop Port 1:"]
    #[inline(always)]
    pub const fn set_bf10(&mut self, val: super::vals::EscDlControlPdiBf10) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn bf12(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_bf12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn bf14(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_bf14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "RX FIFO Size (ESC delays start of forwarding until FIFO is at least half full)."]
    #[must_use]
    #[inline(always)]
    pub const fn bf16(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "RX FIFO Size (ESC delays start of forwarding until FIFO is at least half full)."]
    #[inline(always)]
    pub const fn set_bf16(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf20(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf20(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Station alias:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf24(&self) -> super::vals::EscDlControlPdiBf24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::EscDlControlPdiBf24::from_bits(val as u8)
    }
    #[doc = "Station alias:"]
    #[inline(always)]
    pub const fn set_bf24(&mut self, val: super::vals::EscDlControlPdiBf24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf25(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x7f;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf25(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
    }
}
impl Default for EscDlControlPdi {
    #[inline(always)]
    fn default() -> EscDlControlPdi {
        EscDlControlPdi(507905u64 as u32)
    }
}
impl core::fmt::Debug for EscDlControlPdi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EscDlControlPdi")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .field("bf2", &self.bf2())
            .field("bf8", &self.bf8())
            .field("bf10", &self.bf10())
            .field("bf12", &self.bf12())
            .field("bf14", &self.bf14())
            .field("bf16", &self.bf16())
            .field("bf20", &self.bf20())
            .field("bf23", &self.bf23())
            .field("bf24", &self.bf24())
            .field("bf25", &self.bf25())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EscDlControlPdi {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EscDlControlPdi {
            bf0: super::vals::EscDlControlPdiBf0,
            bf1: super::vals::EscDlControlPdiBf1,
            bf2: u8,
            bf8: super::vals::EscDlControlPdiBf8,
            bf10: super::vals::EscDlControlPdiBf10,
            bf12: u8,
            bf14: u8,
            bf16: u8,
            bf20: u8,
            bf23: bool,
            bf24: super::vals::EscDlControlPdiBf24,
            bf25: u8,
        }
        let proxy = EscDlControlPdi {
            bf0: self.bf0(),
            bf1: self.bf1(),
            bf2: self.bf2(),
            bf8: self.bf8(),
            bf10: self.bf10(),
            bf12: self.bf12(),
            bf14: self.bf14(),
            bf16: self.bf16(),
            bf20: self.bf20(),
            bf23: self.bf23(),
            bf24: self.bf24(),
            bf25: self.bf25(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ESC DL Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EscDlStatus(pub u16);
impl EscDlStatus {
    #[doc = "Register ESC DL Status"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::EscDlStatusBf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::EscDlStatusBf0::from_bits(val as u8)
    }
    #[doc = "Register ESC DL Status"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::EscDlStatusBf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "PDI Watchdog Status:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> super::vals::EscDlStatusBf1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::EscDlStatusBf1::from_bits(val as u8)
    }
    #[doc = "PDI Watchdog Status:"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: super::vals::EscDlStatusBf1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Enhanced Link detection:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf2(&self) -> super::vals::EscDlStatusBf2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::EscDlStatusBf2::from_bits(val as u8)
    }
    #[doc = "Enhanced Link detection:"]
    #[inline(always)]
    pub const fn set_bf2(&mut self, val: super::vals::EscDlStatusBf2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn bf3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_bf3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Physical link on Port 0:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf4(&self) -> super::vals::EscDlStatusBf4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::EscDlStatusBf4::from_bits(val as u8)
    }
    #[doc = "Physical link on Port 0:"]
    #[inline(always)]
    pub const fn set_bf4(&mut self, val: super::vals::EscDlStatusBf4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Physical link on Port 1:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf5(&self) -> super::vals::EscDlStatusBf5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::EscDlStatusBf5::from_bits(val as u8)
    }
    #[doc = "Physical link on Port 1:"]
    #[inline(always)]
    pub const fn set_bf5(&mut self, val: super::vals::EscDlStatusBf5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn bf6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_bf6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn bf7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_bf7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Loop Port 0:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf8(&self) -> super::vals::EscDlStatusBf8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::EscDlStatusBf8::from_bits(val as u8)
    }
    #[doc = "Loop Port 0:"]
    #[inline(always)]
    pub const fn set_bf8(&mut self, val: super::vals::EscDlStatusBf8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "Communication on Port 0:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf9(&self) -> super::vals::EscDlStatusBf9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::EscDlStatusBf9::from_bits(val as u8)
    }
    #[doc = "Communication on Port 0:"]
    #[inline(always)]
    pub const fn set_bf9(&mut self, val: super::vals::EscDlStatusBf9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
    }
    #[doc = "Loop Port 1"]
    #[must_use]
    #[inline(always)]
    pub const fn bf10(&self) -> super::vals::EscDlStatusBf10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::EscDlStatusBf10::from_bits(val as u8)
    }
    #[doc = "Loop Port 1"]
    #[inline(always)]
    pub const fn set_bf10(&mut self, val: super::vals::EscDlStatusBf10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u16) & 0x01) << 10usize);
    }
    #[doc = "Communication on Port 1:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf11(&self) -> super::vals::EscDlStatusBf11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::EscDlStatusBf11::from_bits(val as u8)
    }
    #[doc = "Communication on Port 1:"]
    #[inline(always)]
    pub const fn set_bf11(&mut self, val: super::vals::EscDlStatusBf11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u16) & 0x01) << 11usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn bf12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_bf12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn bf13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_bf13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn bf14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_bf14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn bf15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_bf15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for EscDlStatus {
    #[inline(always)]
    fn default() -> EscDlStatus {
        EscDlStatus(23094u64 as u16)
    }
}
impl core::fmt::Debug for EscDlStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EscDlStatus")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .field("bf2", &self.bf2())
            .field("bf3", &self.bf3())
            .field("bf4", &self.bf4())
            .field("bf5", &self.bf5())
            .field("bf6", &self.bf6())
            .field("bf7", &self.bf7())
            .field("bf8", &self.bf8())
            .field("bf9", &self.bf9())
            .field("bf10", &self.bf10())
            .field("bf11", &self.bf11())
            .field("bf12", &self.bf12())
            .field("bf13", &self.bf13())
            .field("bf14", &self.bf14())
            .field("bf15", &self.bf15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EscDlStatus {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EscDlStatus {
            bf0: super::vals::EscDlStatusBf0,
            bf1: super::vals::EscDlStatusBf1,
            bf2: super::vals::EscDlStatusBf2,
            bf3: bool,
            bf4: super::vals::EscDlStatusBf4,
            bf5: super::vals::EscDlStatusBf5,
            bf6: bool,
            bf7: bool,
            bf8: super::vals::EscDlStatusBf8,
            bf9: super::vals::EscDlStatusBf9,
            bf10: super::vals::EscDlStatusBf10,
            bf11: super::vals::EscDlStatusBf11,
            bf12: bool,
            bf13: bool,
            bf14: bool,
            bf15: bool,
        }
        let proxy = EscDlStatus {
            bf0: self.bf0(),
            bf1: self.bf1(),
            bf2: self.bf2(),
            bf3: self.bf3(),
            bf4: self.bf4(),
            bf5: self.bf5(),
            bf6: self.bf6(),
            bf7: self.bf7(),
            bf8: self.bf8(),
            bf9: self.bf9(),
            bf10: self.bf10(),
            bf11: self.bf11(),
            bf12: self.bf12(),
            bf13: self.bf13(),
            bf14: self.bf14(),
            bf15: self.bf15(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Register ESC Features supported"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EscFeaturesSupported(pub u16);
impl EscFeaturesSupported {
    #[doc = "FMMU Operation:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::EscFeaturesSupportedBf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::EscFeaturesSupportedBf0::from_bits(val as u8)
    }
    #[doc = "FMMU Operation:"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::EscFeaturesSupportedBf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "Unused register access:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> super::vals::EscFeaturesSupportedBf1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::EscFeaturesSupportedBf1::from_bits(val as u8)
    }
    #[doc = "Unused register access:"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: super::vals::EscFeaturesSupportedBf1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Distributed Clocks:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf2(&self) -> super::vals::EscFeaturesSupportedBf2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::EscFeaturesSupportedBf2::from_bits(val as u8)
    }
    #[doc = "Distributed Clocks:"]
    #[inline(always)]
    pub const fn set_bf2(&mut self, val: super::vals::EscFeaturesSupportedBf2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Distributed Clocks (width):"]
    #[must_use]
    #[inline(always)]
    pub const fn bf3(&self) -> super::vals::EscFeaturesSupportedBf3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::EscFeaturesSupportedBf3::from_bits(val as u8)
    }
    #[doc = "Distributed Clocks (width):"]
    #[inline(always)]
    pub const fn set_bf3(&mut self, val: super::vals::EscFeaturesSupportedBf3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
    #[doc = "Enhanced Link Detection MII:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf6(&self) -> super::vals::EscFeaturesSupportedBf6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::EscFeaturesSupportedBf6::from_bits(val as u8)
    }
    #[doc = "Enhanced Link Detection MII:"]
    #[inline(always)]
    pub const fn set_bf6(&mut self, val: super::vals::EscFeaturesSupportedBf6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Separate Handling of FCS Errors:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf7(&self) -> super::vals::EscFeaturesSupportedBf7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::EscFeaturesSupportedBf7::from_bits(val as u8)
    }
    #[doc = "Separate Handling of FCS Errors:"]
    #[inline(always)]
    pub const fn set_bf7(&mut self, val: super::vals::EscFeaturesSupportedBf7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "Enhanced DC SYNC Activation"]
    #[must_use]
    #[inline(always)]
    pub const fn bf8(&self) -> super::vals::EscFeaturesSupportedBf8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::EscFeaturesSupportedBf8::from_bits(val as u8)
    }
    #[doc = "Enhanced DC SYNC Activation"]
    #[inline(always)]
    pub const fn set_bf8(&mut self, val: super::vals::EscFeaturesSupportedBf8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "EtherCAT LRW command support:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf9(&self) -> super::vals::EscFeaturesSupportedBf9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::EscFeaturesSupportedBf9::from_bits(val as u8)
    }
    #[doc = "EtherCAT LRW command support:"]
    #[inline(always)]
    pub const fn set_bf9(&mut self, val: super::vals::EscFeaturesSupportedBf9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
    }
    #[doc = "EtherCAT read/write command support (BRW, APRW, FPRW):"]
    #[must_use]
    #[inline(always)]
    pub const fn bf10(&self) -> super::vals::EscFeaturesSupportedBf10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::EscFeaturesSupportedBf10::from_bits(val as u8)
    }
    #[doc = "EtherCAT read/write command support (BRW, APRW, FPRW):"]
    #[inline(always)]
    pub const fn set_bf10(&mut self, val: super::vals::EscFeaturesSupportedBf10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u16) & 0x01) << 10usize);
    }
    #[doc = "Fixed FMMU/SyncManager configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn bf11(&self) -> super::vals::EscFeaturesSupportedBf11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::EscFeaturesSupportedBf11::from_bits(val as u8)
    }
    #[doc = "Fixed FMMU/SyncManager configuration"]
    #[inline(always)]
    pub const fn set_bf11(&mut self, val: super::vals::EscFeaturesSupportedBf11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u16) & 0x01) << 11usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn bf12(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_bf12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u16) & 0x0f) << 12usize);
    }
}
impl Default for EscFeaturesSupported {
    #[inline(always)]
    fn default() -> EscFeaturesSupported {
        EscFeaturesSupported(460u64 as u16)
    }
}
impl core::fmt::Debug for EscFeaturesSupported {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EscFeaturesSupported")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .field("bf2", &self.bf2())
            .field("bf3", &self.bf3())
            .field("bf6", &self.bf6())
            .field("bf7", &self.bf7())
            .field("bf8", &self.bf8())
            .field("bf9", &self.bf9())
            .field("bf10", &self.bf10())
            .field("bf11", &self.bf11())
            .field("bf12", &self.bf12())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EscFeaturesSupported {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EscFeaturesSupported {
            bf0: super::vals::EscFeaturesSupportedBf0,
            bf1: super::vals::EscFeaturesSupportedBf1,
            bf2: super::vals::EscFeaturesSupportedBf2,
            bf3: super::vals::EscFeaturesSupportedBf3,
            bf6: super::vals::EscFeaturesSupportedBf6,
            bf7: super::vals::EscFeaturesSupportedBf7,
            bf8: super::vals::EscFeaturesSupportedBf8,
            bf9: super::vals::EscFeaturesSupportedBf9,
            bf10: super::vals::EscFeaturesSupportedBf10,
            bf11: super::vals::EscFeaturesSupportedBf11,
            bf12: u8,
        }
        let proxy = EscFeaturesSupported {
            bf0: self.bf0(),
            bf1: self.bf1(),
            bf2: self.bf2(),
            bf3: self.bf3(),
            bf6: self.bf6(),
            bf7: self.bf7(),
            bf8: self.bf8(),
            bf9: self.bf9(),
            bf10: self.bf10(),
            bf11: self.bf11(),
            bf12: self.bf12(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ESC Write Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EscWriteEnable(pub u8);
impl EscWriteEnable {
    #[doc = "ESC Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ESC Write Enable"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u8) & 0x7f) << 1usize);
    }
}
impl Default for EscWriteEnable {
    #[inline(always)]
    fn default() -> EscWriteEnable {
        EscWriteEnable(0u64 as u8)
    }
}
impl core::fmt::Debug for EscWriteEnable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EscWriteEnable")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EscWriteEnable {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EscWriteEnable {
            bf0: bool,
            bf1: u8,
        }
        let proxy = EscWriteEnable {
            bf0: self.bf0(),
            bf1: self.bf1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ESC Write Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EscWriteEnablePdi(pub u8);
impl EscWriteEnablePdi {
    #[doc = "ESC Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ESC Write Enable"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u8) & 0x7f) << 1usize);
    }
}
impl Default for EscWriteEnablePdi {
    #[inline(always)]
    fn default() -> EscWriteEnablePdi {
        EscWriteEnablePdi(0u64 as u8)
    }
}
impl core::fmt::Debug for EscWriteEnablePdi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EscWriteEnablePdi")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EscWriteEnablePdi {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EscWriteEnablePdi {
            bf0: bool,
            bf1: u8,
        }
        let proxy = EscWriteEnablePdi {
            bf0: self.bf0(),
            bf1: self.bf1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ESC Write Protection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EscWriteProtection(pub u8);
impl EscWriteProtection {
    #[doc = "Write protect:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::EscWriteProtectionBf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::EscWriteProtectionBf0::from_bits(val as u8)
    }
    #[doc = "Write protect:"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::EscWriteProtectionBf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u8) & 0x7f) << 1usize);
    }
}
impl Default for EscWriteProtection {
    #[inline(always)]
    fn default() -> EscWriteProtection {
        EscWriteProtection(0u64 as u8)
    }
}
impl core::fmt::Debug for EscWriteProtection {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EscWriteProtection")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EscWriteProtection {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EscWriteProtection {
            bf0: super::vals::EscWriteProtectionBf0,
            bf1: u8,
        }
        let proxy = EscWriteProtection {
            bf0: self.bf0(),
            bf1: self.bf1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ESC Write Protection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EscWriteProtectionPdi(pub u8);
impl EscWriteProtectionPdi {
    #[doc = "Write protect:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::EscWriteProtectionPdiBf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::EscWriteProtectionPdiBf0::from_bits(val as u8)
    }
    #[doc = "Write protect:"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::EscWriteProtectionPdiBf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u8) & 0x7f) << 1usize);
    }
}
impl Default for EscWriteProtectionPdi {
    #[inline(always)]
    fn default() -> EscWriteProtectionPdi {
        EscWriteProtectionPdi(0u64 as u8)
    }
}
impl core::fmt::Debug for EscWriteProtectionPdi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EscWriteProtectionPdi")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EscWriteProtectionPdi {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EscWriteProtectionPdi {
            bf0: super::vals::EscWriteProtectionPdiBf0,
            bf1: u8,
        }
        let proxy = EscWriteProtectionPdi {
            bf0: self.bf0(),
            bf1: self.bf1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Register Activate FMMU"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FmmuActivate(pub u8);
impl FmmuActivate {
    #[doc = "Bit field access for ECAT: r/w"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::FmmuActivateBf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::FmmuActivateBf0::from_bits(val as u8)
    }
    #[doc = "Bit field access for ECAT: r/w"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::FmmuActivateBf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u8) & 0x7f) << 1usize);
    }
}
impl Default for FmmuActivate {
    #[inline(always)]
    fn default() -> FmmuActivate {
        FmmuActivate(0u64 as u8)
    }
}
impl core::fmt::Debug for FmmuActivate {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FmmuActivate")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FmmuActivate {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct FmmuActivate {
            bf0: super::vals::FmmuActivateBf0,
            bf1: u8,
        }
        let proxy = FmmuActivate {
            bf0: self.bf0(),
            bf1: self.bf1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Register Activate FMMU"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FmmuActivatePdi(pub u8);
impl FmmuActivatePdi {
    #[doc = "Bit field access for PDI: r/-"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::FmmuActivatePdiBf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::FmmuActivatePdiBf0::from_bits(val as u8)
    }
    #[doc = "Bit field access for PDI: r/-"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::FmmuActivatePdiBf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u8) & 0x7f) << 1usize);
    }
}
impl Default for FmmuActivatePdi {
    #[inline(always)]
    fn default() -> FmmuActivatePdi {
        FmmuActivatePdi(0u64 as u8)
    }
}
impl core::fmt::Debug for FmmuActivatePdi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FmmuActivatePdi")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FmmuActivatePdi {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct FmmuActivatePdi {
            bf0: super::vals::FmmuActivatePdiBf0,
            bf1: u8,
        }
        let proxy = FmmuActivatePdi {
            bf0: self.bf0(),
            bf1: self.bf1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Register Start bit FMMU y in logical address space"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FmmuLogicalStartBit(pub u8);
impl FmmuLogicalStartBit {
    #[doc = "Logical starting bit that shall be mapped (bits are counted from least significant bit 0 to most significant bit 7)"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Logical starting bit that shall be mapped (bits are counted from least significant bit 0 to most significant bit 7)"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u8) & 0x07) << 0usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf3(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u8) & 0x1f) << 3usize);
    }
}
impl Default for FmmuLogicalStartBit {
    #[inline(always)]
    fn default() -> FmmuLogicalStartBit {
        FmmuLogicalStartBit(0u64 as u8)
    }
}
impl core::fmt::Debug for FmmuLogicalStartBit {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FmmuLogicalStartBit")
            .field("bf0", &self.bf0())
            .field("bf3", &self.bf3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FmmuLogicalStartBit {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct FmmuLogicalStartBit {
            bf0: u8,
            bf3: u8,
        }
        let proxy = FmmuLogicalStartBit {
            bf0: self.bf0(),
            bf3: self.bf3(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Register Start bit FMMU y in logical address space"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FmmuLogicalStartBitPdi(pub u8);
impl FmmuLogicalStartBitPdi {
    #[doc = "Logical starting bit that shall be mapped (bits are counted from least significant bit 0 to most significant bit 7)"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Logical starting bit that shall be mapped (bits are counted from least significant bit 0 to most significant bit 7)"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u8) & 0x07) << 0usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf3(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u8) & 0x1f) << 3usize);
    }
}
impl Default for FmmuLogicalStartBitPdi {
    #[inline(always)]
    fn default() -> FmmuLogicalStartBitPdi {
        FmmuLogicalStartBitPdi(0u64 as u8)
    }
}
impl core::fmt::Debug for FmmuLogicalStartBitPdi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FmmuLogicalStartBitPdi")
            .field("bf0", &self.bf0())
            .field("bf3", &self.bf3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FmmuLogicalStartBitPdi {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct FmmuLogicalStartBitPdi {
            bf0: u8,
            bf3: u8,
        }
        let proxy = FmmuLogicalStartBitPdi {
            bf0: self.bf0(),
            bf3: self.bf3(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Register Stop bit FMMU y in logical address space"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FmmuLogicalStopBit(pub u8);
impl FmmuLogicalStopBit {
    #[doc = "Last logical bit that shall be mapped (bits are counted from least significant bit 0 to most significant bit 7)"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Last logical bit that shall be mapped (bits are counted from least significant bit 0 to most significant bit 7)"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u8) & 0x07) << 0usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf3(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u8) & 0x1f) << 3usize);
    }
}
impl Default for FmmuLogicalStopBit {
    #[inline(always)]
    fn default() -> FmmuLogicalStopBit {
        FmmuLogicalStopBit(0u64 as u8)
    }
}
impl core::fmt::Debug for FmmuLogicalStopBit {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FmmuLogicalStopBit")
            .field("bf0", &self.bf0())
            .field("bf3", &self.bf3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FmmuLogicalStopBit {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct FmmuLogicalStopBit {
            bf0: u8,
            bf3: u8,
        }
        let proxy = FmmuLogicalStopBit {
            bf0: self.bf0(),
            bf3: self.bf3(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Register Stop bit FMMU y in logical address space"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FmmuLogicalStopBitPdi(pub u8);
impl FmmuLogicalStopBitPdi {
    #[doc = "Last logical bit that shall be mapped (bits are counted from least significant bit 0 to most significant bit 7)"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Last logical bit that shall be mapped (bits are counted from least significant bit 0 to most significant bit 7)"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u8) & 0x07) << 0usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf3(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u8) & 0x1f) << 3usize);
    }
}
impl Default for FmmuLogicalStopBitPdi {
    #[inline(always)]
    fn default() -> FmmuLogicalStopBitPdi {
        FmmuLogicalStopBitPdi(0u64 as u8)
    }
}
impl core::fmt::Debug for FmmuLogicalStopBitPdi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FmmuLogicalStopBitPdi")
            .field("bf0", &self.bf0())
            .field("bf3", &self.bf3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FmmuLogicalStopBitPdi {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct FmmuLogicalStopBitPdi {
            bf0: u8,
            bf3: u8,
        }
        let proxy = FmmuLogicalStopBitPdi {
            bf0: self.bf0(),
            bf3: self.bf3(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Register Physical Start bit FMMU"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FmmuPhysicalStartBit(pub u8);
impl FmmuPhysicalStartBit {
    #[doc = "Physical starting bit as target of logical start bit mapping (bits are counted from least significant bit 0 to most significant bit 7)"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Physical starting bit as target of logical start bit mapping (bits are counted from least significant bit 0 to most significant bit 7)"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u8) & 0x07) << 0usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf3(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u8) & 0x1f) << 3usize);
    }
}
impl Default for FmmuPhysicalStartBit {
    #[inline(always)]
    fn default() -> FmmuPhysicalStartBit {
        FmmuPhysicalStartBit(0u64 as u8)
    }
}
impl core::fmt::Debug for FmmuPhysicalStartBit {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FmmuPhysicalStartBit")
            .field("bf0", &self.bf0())
            .field("bf3", &self.bf3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FmmuPhysicalStartBit {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct FmmuPhysicalStartBit {
            bf0: u8,
            bf3: u8,
        }
        let proxy = FmmuPhysicalStartBit {
            bf0: self.bf0(),
            bf3: self.bf3(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Register Physical Start bit FMMU"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FmmuPhysicalStartBitPdi(pub u8);
impl FmmuPhysicalStartBitPdi {
    #[doc = "Physical starting bit as target of logical start bit mapping (bits are counted from least significant bit 0 to most significant bit 7)"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Physical starting bit as target of logical start bit mapping (bits are counted from least significant bit 0 to most significant bit 7)"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u8) & 0x07) << 0usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf3(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u8) & 0x1f) << 3usize);
    }
}
impl Default for FmmuPhysicalStartBitPdi {
    #[inline(always)]
    fn default() -> FmmuPhysicalStartBitPdi {
        FmmuPhysicalStartBitPdi(0u64 as u8)
    }
}
impl core::fmt::Debug for FmmuPhysicalStartBitPdi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FmmuPhysicalStartBitPdi")
            .field("bf0", &self.bf0())
            .field("bf3", &self.bf3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FmmuPhysicalStartBitPdi {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct FmmuPhysicalStartBitPdi {
            bf0: u8,
            bf3: u8,
        }
        let proxy = FmmuPhysicalStartBitPdi {
            bf0: self.bf0(),
            bf3: self.bf3(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Register Type FMMU y"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FmmuType(pub u8);
impl FmmuType {
    #[doc = "Bit field access for ECAT: r/w"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::FmmuTypeBf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::FmmuTypeBf0::from_bits(val as u8)
    }
    #[doc = "Bit field access for ECAT: r/w"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::FmmuTypeBf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Bit field access for ECAT: r/w"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> super::vals::FmmuTypeBf1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::FmmuTypeBf1::from_bits(val as u8)
    }
    #[doc = "Bit field access for ECAT: r/w"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: super::vals::FmmuTypeBf1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf2(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u8) & 0x3f) << 2usize);
    }
}
impl Default for FmmuType {
    #[inline(always)]
    fn default() -> FmmuType {
        FmmuType(0u64 as u8)
    }
}
impl core::fmt::Debug for FmmuType {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FmmuType")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .field("bf2", &self.bf2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FmmuType {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct FmmuType {
            bf0: super::vals::FmmuTypeBf0,
            bf1: super::vals::FmmuTypeBf1,
            bf2: u8,
        }
        let proxy = FmmuType {
            bf0: self.bf0(),
            bf1: self.bf1(),
            bf2: self.bf2(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Register Type FMMU y"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FmmuTypePdi(pub u8);
impl FmmuTypePdi {
    #[doc = "Bit field access for PDI: r/-"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::FmmuTypePdiBf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::FmmuTypePdiBf0::from_bits(val as u8)
    }
    #[doc = "Bit field access for PDI: r/-"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::FmmuTypePdiBf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Bit field access for PDI: r/-"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> super::vals::FmmuTypePdiBf1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::FmmuTypePdiBf1::from_bits(val as u8)
    }
    #[doc = "Bit field access for PDI: r/-"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: super::vals::FmmuTypePdiBf1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf2(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u8) & 0x3f) << 2usize);
    }
}
impl Default for FmmuTypePdi {
    #[inline(always)]
    fn default() -> FmmuTypePdi {
        FmmuTypePdi(0u64 as u8)
    }
}
impl core::fmt::Debug for FmmuTypePdi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FmmuTypePdi")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .field("bf2", &self.bf2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FmmuTypePdi {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct FmmuTypePdi {
            bf0: super::vals::FmmuTypePdiBf0,
            bf1: super::vals::FmmuTypePdiBf1,
            bf2: u8,
        }
        let proxy = FmmuTypePdi {
            bf0: self.bf0(),
            bf1: self.bf1(),
            bf2: self.bf2(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Register Latch0 Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Latch0Control(pub u8);
impl Latch0Control {
    #[doc = "Latch0 positive edge:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::Latch0ControlBf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Latch0ControlBf0::from_bits(val as u8)
    }
    #[doc = "Latch0 positive edge:"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::Latch0ControlBf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Latch0 negative edge:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> super::vals::Latch0ControlBf1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Latch0ControlBf1::from_bits(val as u8)
    }
    #[doc = "Latch0 negative edge:"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: super::vals::Latch0ControlBf1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf2(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u8) & 0x3f) << 2usize);
    }
}
impl Default for Latch0Control {
    #[inline(always)]
    fn default() -> Latch0Control {
        Latch0Control(0u64 as u8)
    }
}
impl core::fmt::Debug for Latch0Control {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Latch0Control")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .field("bf2", &self.bf2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Latch0Control {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Latch0Control {
            bf0: super::vals::Latch0ControlBf0,
            bf1: super::vals::Latch0ControlBf1,
            bf2: u8,
        }
        let proxy = Latch0Control {
            bf0: self.bf0(),
            bf1: self.bf1(),
            bf2: self.bf2(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Register Latch0 Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Latch0Status(pub u8);
impl Latch0Status {
    #[doc = "Event Latch0 positive edge."]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::Latch0StatusBf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Latch0StatusBf0::from_bits(val as u8)
    }
    #[doc = "Event Latch0 positive edge."]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::Latch0StatusBf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Event Latch0 negative edge."]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> super::vals::Latch0StatusBf1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Latch0StatusBf1::from_bits(val as u8)
    }
    #[doc = "Event Latch0 negative edge."]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: super::vals::Latch0StatusBf1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "Latch0 pin state"]
    #[must_use]
    #[inline(always)]
    pub const fn bf2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Latch0 pin state"]
    #[inline(always)]
    pub const fn set_bf2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn bf3(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_bf3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u8) & 0x1f) << 3usize);
    }
}
impl Default for Latch0Status {
    #[inline(always)]
    fn default() -> Latch0Status {
        Latch0Status(0u64 as u8)
    }
}
impl core::fmt::Debug for Latch0Status {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Latch0Status")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .field("bf2", &self.bf2())
            .field("bf3", &self.bf3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Latch0Status {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Latch0Status {
            bf0: super::vals::Latch0StatusBf0,
            bf1: super::vals::Latch0StatusBf1,
            bf2: bool,
            bf3: u8,
        }
        let proxy = Latch0Status {
            bf0: self.bf0(),
            bf1: self.bf1(),
            bf2: self.bf2(),
            bf3: self.bf3(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Register Latch1 Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Latch1Control(pub u8);
impl Latch1Control {
    #[doc = "Latch1 positive edge:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::Latch1ControlBf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Latch1ControlBf0::from_bits(val as u8)
    }
    #[doc = "Latch1 positive edge:"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::Latch1ControlBf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Latch1 negative edge:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> super::vals::Latch1ControlBf1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Latch1ControlBf1::from_bits(val as u8)
    }
    #[doc = "Latch1 negative edge:"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: super::vals::Latch1ControlBf1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf2(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u8) & 0x3f) << 2usize);
    }
}
impl Default for Latch1Control {
    #[inline(always)]
    fn default() -> Latch1Control {
        Latch1Control(0u64 as u8)
    }
}
impl core::fmt::Debug for Latch1Control {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Latch1Control")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .field("bf2", &self.bf2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Latch1Control {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Latch1Control {
            bf0: super::vals::Latch1ControlBf0,
            bf1: super::vals::Latch1ControlBf1,
            bf2: u8,
        }
        let proxy = Latch1Control {
            bf0: self.bf0(),
            bf1: self.bf1(),
            bf2: self.bf2(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Register Latch1 Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Latch1Status(pub u8);
impl Latch1Status {
    #[doc = "Event Latch1 positive edge."]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::Latch1StatusBf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Latch1StatusBf0::from_bits(val as u8)
    }
    #[doc = "Event Latch1 positive edge."]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::Latch1StatusBf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Event Latch1 negative edge."]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> super::vals::Latch1StatusBf1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Latch1StatusBf1::from_bits(val as u8)
    }
    #[doc = "Event Latch1 negative edge."]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: super::vals::Latch1StatusBf1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "Latch1 pin state"]
    #[must_use]
    #[inline(always)]
    pub const fn bf2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Latch1 pin state"]
    #[inline(always)]
    pub const fn set_bf2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn bf3(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_bf3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u8) & 0x1f) << 3usize);
    }
}
impl Default for Latch1Status {
    #[inline(always)]
    fn default() -> Latch1Status {
        Latch1Status(0u64 as u8)
    }
}
impl core::fmt::Debug for Latch1Status {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Latch1Status")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .field("bf2", &self.bf2())
            .field("bf3", &self.bf3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Latch1Status {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Latch1Status {
            bf0: super::vals::Latch1StatusBf0,
            bf1: super::vals::Latch1StatusBf1,
            bf2: bool,
            bf3: u8,
        }
        let proxy = Latch1Status {
            bf0: self.bf0(),
            bf1: self.bf1(),
            bf2: self.bf2(),
            bf3: self.bf3(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "MII Management Control/Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MiiManagementControlOrStatus(pub u16);
impl MiiManagementControlOrStatus {
    #[doc = "Write enable*:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::MiiManagementControlOrStatusBf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::MiiManagementControlOrStatusBf0::from_bits(val as u8)
    }
    #[doc = "Write enable*:"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::MiiManagementControlOrStatusBf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "Management Interface can be controlled by PDI (registers 0x0516-0x0517):"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> super::vals::MiiManagementControlOrStatusBf1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::MiiManagementControlOrStatusBf1::from_bits(val as u8)
    }
    #[doc = "Management Interface can be controlled by PDI (registers 0x0516-0x0517):"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: super::vals::MiiManagementControlOrStatusBf1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "MI link detection and configuration:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf2(&self) -> super::vals::MiiManagementControlOrStatusBf2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::MiiManagementControlOrStatusBf2::from_bits(val as u8)
    }
    #[doc = "MI link detection and configuration:"]
    #[inline(always)]
    pub const fn set_bf2(&mut self, val: super::vals::MiiManagementControlOrStatusBf2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "PHY address of port 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf3(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "PHY address of port 0"]
    #[inline(always)]
    pub const fn set_bf3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u16) & 0x1f) << 3usize);
    }
    #[doc = "Command register*:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf8(&self) -> super::vals::MiiManagementControlOrStatusBf8 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::MiiManagementControlOrStatusBf8::from_bits(val as u8)
    }
    #[doc = "Command register*:"]
    #[inline(always)]
    pub const fn set_bf8(&mut self, val: super::vals::MiiManagementControlOrStatusBf8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf10(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u16) & 0x07) << 10usize);
    }
    #[doc = "Read error:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf13(&self) -> super::vals::MiiManagementControlOrStatusBf13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::MiiManagementControlOrStatusBf13::from_bits(val as u8)
    }
    #[doc = "Read error:"]
    #[inline(always)]
    pub const fn set_bf13(&mut self, val: super::vals::MiiManagementControlOrStatusBf13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "Command error:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf14(&self) -> super::vals::MiiManagementControlOrStatusBf14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::MiiManagementControlOrStatusBf14::from_bits(val as u8)
    }
    #[doc = "Command error:"]
    #[inline(always)]
    pub const fn set_bf14(&mut self, val: super::vals::MiiManagementControlOrStatusBf14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "Busy:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf15(&self) -> super::vals::MiiManagementControlOrStatusBf15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::MiiManagementControlOrStatusBf15::from_bits(val as u8)
    }
    #[doc = "Busy:"]
    #[inline(always)]
    pub const fn set_bf15(&mut self, val: super::vals::MiiManagementControlOrStatusBf15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
    }
}
impl Default for MiiManagementControlOrStatus {
    #[inline(always)]
    fn default() -> MiiManagementControlOrStatus {
        MiiManagementControlOrStatus(2u64 as u16)
    }
}
impl core::fmt::Debug for MiiManagementControlOrStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MiiManagementControlOrStatus")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .field("bf2", &self.bf2())
            .field("bf3", &self.bf3())
            .field("bf8", &self.bf8())
            .field("bf10", &self.bf10())
            .field("bf13", &self.bf13())
            .field("bf14", &self.bf14())
            .field("bf15", &self.bf15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MiiManagementControlOrStatus {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MiiManagementControlOrStatus {
            bf0: super::vals::MiiManagementControlOrStatusBf0,
            bf1: super::vals::MiiManagementControlOrStatusBf1,
            bf2: super::vals::MiiManagementControlOrStatusBf2,
            bf3: u8,
            bf8: super::vals::MiiManagementControlOrStatusBf8,
            bf10: u8,
            bf13: super::vals::MiiManagementControlOrStatusBf13,
            bf14: super::vals::MiiManagementControlOrStatusBf14,
            bf15: super::vals::MiiManagementControlOrStatusBf15,
        }
        let proxy = MiiManagementControlOrStatus {
            bf0: self.bf0(),
            bf1: self.bf1(),
            bf2: self.bf2(),
            bf3: self.bf3(),
            bf8: self.bf8(),
            bf10: self.bf10(),
            bf13: self.bf13(),
            bf14: self.bf14(),
            bf15: self.bf15(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "MII Management Control/Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MiiManagementControlOrStatusPdi(pub u16);
impl MiiManagementControlOrStatusPdi {
    #[doc = "Write enable*:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::MiiManagementControlOrStatusPdiBf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::MiiManagementControlOrStatusPdiBf0::from_bits(val as u8)
    }
    #[doc = "Write enable*:"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::MiiManagementControlOrStatusPdiBf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "Management Interface can be controlled by PDI (registers 0x0516-0x0517):"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> super::vals::MiiManagementControlOrStatusPdiBf1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::MiiManagementControlOrStatusPdiBf1::from_bits(val as u8)
    }
    #[doc = "Management Interface can be controlled by PDI (registers 0x0516-0x0517):"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: super::vals::MiiManagementControlOrStatusPdiBf1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "MI link detection and configuration:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf2(&self) -> super::vals::MiiManagementControlOrStatusPdiBf2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::MiiManagementControlOrStatusPdiBf2::from_bits(val as u8)
    }
    #[doc = "MI link detection and configuration:"]
    #[inline(always)]
    pub const fn set_bf2(&mut self, val: super::vals::MiiManagementControlOrStatusPdiBf2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "PHY address of port 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf3(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "PHY address of port 0"]
    #[inline(always)]
    pub const fn set_bf3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u16) & 0x1f) << 3usize);
    }
    #[doc = "Command register*:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf8(&self) -> super::vals::MiiManagementControlOrStatusPdiBf8 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::MiiManagementControlOrStatusPdiBf8::from_bits(val as u8)
    }
    #[doc = "Command register*:"]
    #[inline(always)]
    pub const fn set_bf8(&mut self, val: super::vals::MiiManagementControlOrStatusPdiBf8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf10(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u16) & 0x07) << 10usize);
    }
    #[doc = "Read error:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf13(&self) -> super::vals::MiiManagementControlOrStatusPdiBf13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::MiiManagementControlOrStatusPdiBf13::from_bits(val as u8)
    }
    #[doc = "Read error:"]
    #[inline(always)]
    pub const fn set_bf13(&mut self, val: super::vals::MiiManagementControlOrStatusPdiBf13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "Command error:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf14(&self) -> super::vals::MiiManagementControlOrStatusPdiBf14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::MiiManagementControlOrStatusPdiBf14::from_bits(val as u8)
    }
    #[doc = "Command error:"]
    #[inline(always)]
    pub const fn set_bf14(&mut self, val: super::vals::MiiManagementControlOrStatusPdiBf14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "Busy:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf15(&self) -> super::vals::MiiManagementControlOrStatusPdiBf15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::MiiManagementControlOrStatusPdiBf15::from_bits(val as u8)
    }
    #[doc = "Busy:"]
    #[inline(always)]
    pub const fn set_bf15(&mut self, val: super::vals::MiiManagementControlOrStatusPdiBf15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
    }
}
impl Default for MiiManagementControlOrStatusPdi {
    #[inline(always)]
    fn default() -> MiiManagementControlOrStatusPdi {
        MiiManagementControlOrStatusPdi(2u64 as u16)
    }
}
impl core::fmt::Debug for MiiManagementControlOrStatusPdi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MiiManagementControlOrStatusPdi")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .field("bf2", &self.bf2())
            .field("bf3", &self.bf3())
            .field("bf8", &self.bf8())
            .field("bf10", &self.bf10())
            .field("bf13", &self.bf13())
            .field("bf14", &self.bf14())
            .field("bf15", &self.bf15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MiiManagementControlOrStatusPdi {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MiiManagementControlOrStatusPdi {
            bf0: super::vals::MiiManagementControlOrStatusPdiBf0,
            bf1: super::vals::MiiManagementControlOrStatusPdiBf1,
            bf2: super::vals::MiiManagementControlOrStatusPdiBf2,
            bf3: u8,
            bf8: super::vals::MiiManagementControlOrStatusPdiBf8,
            bf10: u8,
            bf13: super::vals::MiiManagementControlOrStatusPdiBf13,
            bf14: super::vals::MiiManagementControlOrStatusPdiBf14,
            bf15: super::vals::MiiManagementControlOrStatusPdiBf15,
        }
        let proxy = MiiManagementControlOrStatusPdi {
            bf0: self.bf0(),
            bf1: self.bf1(),
            bf2: self.bf2(),
            bf3: self.bf3(),
            bf8: self.bf8(),
            bf10: self.bf10(),
            bf13: self.bf13(),
            bf14: self.bf14(),
            bf15: self.bf15(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "MII Management ECAT Access State"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MiiManagementEcatAccessState(pub u8);
impl MiiManagementEcatAccessState {
    #[doc = "Access to MII management:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::MiiManagementEcatAccessStateBf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::MiiManagementEcatAccessStateBf0::from_bits(val as u8)
    }
    #[doc = "Access to MII management:"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::MiiManagementEcatAccessStateBf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u8) & 0x7f) << 1usize);
    }
}
impl Default for MiiManagementEcatAccessState {
    #[inline(always)]
    fn default() -> MiiManagementEcatAccessState {
        MiiManagementEcatAccessState(0u64 as u8)
    }
}
impl core::fmt::Debug for MiiManagementEcatAccessState {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MiiManagementEcatAccessState")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MiiManagementEcatAccessState {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MiiManagementEcatAccessState {
            bf0: super::vals::MiiManagementEcatAccessStateBf0,
            bf1: u8,
        }
        let proxy = MiiManagementEcatAccessState {
            bf0: self.bf0(),
            bf1: self.bf1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "MII Management ECAT Access State"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MiiManagementEcatAccessStatePdi(pub u8);
impl MiiManagementEcatAccessStatePdi {
    #[doc = "Access to MII management:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::MiiManagementEcatAccessStatePdiBf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::MiiManagementEcatAccessStatePdiBf0::from_bits(val as u8)
    }
    #[doc = "Access to MII management:"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::MiiManagementEcatAccessStatePdiBf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u8) & 0x7f) << 1usize);
    }
}
impl Default for MiiManagementEcatAccessStatePdi {
    #[inline(always)]
    fn default() -> MiiManagementEcatAccessStatePdi {
        MiiManagementEcatAccessStatePdi(0u64 as u8)
    }
}
impl core::fmt::Debug for MiiManagementEcatAccessStatePdi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MiiManagementEcatAccessStatePdi")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MiiManagementEcatAccessStatePdi {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MiiManagementEcatAccessStatePdi {
            bf0: super::vals::MiiManagementEcatAccessStatePdiBf0,
            bf1: u8,
        }
        let proxy = MiiManagementEcatAccessStatePdi {
            bf0: self.bf0(),
            bf1: self.bf1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "MII Management PDI Access State"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MiiManagementPdiAccessState(pub u8);
impl MiiManagementPdiAccessState {
    #[doc = "Access to MII management:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::MiiManagementPdiAccessStateBf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::MiiManagementPdiAccessStateBf0::from_bits(val as u8)
    }
    #[doc = "Access to MII management:"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::MiiManagementPdiAccessStateBf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Force PDI Access State:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> super::vals::MiiManagementPdiAccessStateBf1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::MiiManagementPdiAccessStateBf1::from_bits(val as u8)
    }
    #[doc = "Force PDI Access State:"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: super::vals::MiiManagementPdiAccessStateBf1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf2(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u8) & 0x3f) << 2usize);
    }
}
impl Default for MiiManagementPdiAccessState {
    #[inline(always)]
    fn default() -> MiiManagementPdiAccessState {
        MiiManagementPdiAccessState(0u64 as u8)
    }
}
impl core::fmt::Debug for MiiManagementPdiAccessState {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MiiManagementPdiAccessState")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .field("bf2", &self.bf2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MiiManagementPdiAccessState {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MiiManagementPdiAccessState {
            bf0: super::vals::MiiManagementPdiAccessStateBf0,
            bf1: super::vals::MiiManagementPdiAccessStateBf1,
            bf2: u8,
        }
        let proxy = MiiManagementPdiAccessState {
            bf0: self.bf0(),
            bf1: self.bf1(),
            bf2: self.bf2(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "MII Management PDI Access State"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MiiManagementPdiAccessStatePdi(pub u8);
impl MiiManagementPdiAccessStatePdi {
    #[doc = "Access to MII management:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::MiiManagementPdiAccessStatePdiBf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::MiiManagementPdiAccessStatePdiBf0::from_bits(val as u8)
    }
    #[doc = "Access to MII management:"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::MiiManagementPdiAccessStatePdiBf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Force PDI Access State:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> super::vals::MiiManagementPdiAccessStatePdiBf1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::MiiManagementPdiAccessStatePdiBf1::from_bits(val as u8)
    }
    #[doc = "Force PDI Access State:"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: super::vals::MiiManagementPdiAccessStatePdiBf1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf2(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u8) & 0x3f) << 2usize);
    }
}
impl Default for MiiManagementPdiAccessStatePdi {
    #[inline(always)]
    fn default() -> MiiManagementPdiAccessStatePdi {
        MiiManagementPdiAccessStatePdi(0u64 as u8)
    }
}
impl core::fmt::Debug for MiiManagementPdiAccessStatePdi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MiiManagementPdiAccessStatePdi")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .field("bf2", &self.bf2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MiiManagementPdiAccessStatePdi {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MiiManagementPdiAccessStatePdi {
            bf0: super::vals::MiiManagementPdiAccessStatePdiBf0,
            bf1: super::vals::MiiManagementPdiAccessStatePdiBf1,
            bf2: u8,
        }
        let proxy = MiiManagementPdiAccessStatePdi {
            bf0: self.bf0(),
            bf1: self.bf1(),
            bf2: self.bf2(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PDI AL Event Mask"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PdiAlEventMask(pub u32);
impl PdiAlEventMask {
    #[doc = "AL Event masking of the AL Event Request register Events for mapping to PDI IRQ signal:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::PdiAlEventMaskBf0 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::PdiAlEventMaskBf0::from_bits(val as u32)
    }
    #[doc = "AL Event masking of the AL Event Request register Events for mapping to PDI IRQ signal:"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::PdiAlEventMaskBf0) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PdiAlEventMask {
    #[inline(always)]
    fn default() -> PdiAlEventMask {
        PdiAlEventMask(16776975u64 as u32)
    }
}
impl core::fmt::Debug for PdiAlEventMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PdiAlEventMask")
            .field("bf0", &self.bf0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PdiAlEventMask {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PdiAlEventMask {
            bf0: super::vals::PdiAlEventMaskBf0,
        }
        let proxy = PdiAlEventMask { bf0: self.bf0() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PDI AL Event Mask"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PdiAlEventMaskPdi(pub u32);
impl PdiAlEventMaskPdi {
    #[doc = "AL Event masking of the AL Event Request register Events for mapping to PDI IRQ signal:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::PdiAlEventMaskPdiBf0 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::PdiAlEventMaskPdiBf0::from_bits(val as u32)
    }
    #[doc = "AL Event masking of the AL Event Request register Events for mapping to PDI IRQ signal:"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::PdiAlEventMaskPdiBf0) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PdiAlEventMaskPdi {
    #[inline(always)]
    fn default() -> PdiAlEventMaskPdi {
        PdiAlEventMaskPdi(16776975u64 as u32)
    }
}
impl core::fmt::Debug for PdiAlEventMaskPdi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PdiAlEventMaskPdi")
            .field("bf0", &self.bf0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PdiAlEventMaskPdi {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PdiAlEventMaskPdi {
            bf0: super::vals::PdiAlEventMaskPdiBf0,
        }
        let proxy = PdiAlEventMaskPdi { bf0: self.bf0() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PDI Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PdiControl(pub u8);
impl PdiControl {
    #[doc = "Process data interface:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::PdiControlBf0 {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::PdiControlBf0::from_bits(val as u8)
    }
    #[doc = "Process data interface:"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::PdiControlBf0) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u8) & 0xff) << 0usize);
    }
}
impl Default for PdiControl {
    #[inline(always)]
    fn default() -> PdiControl {
        PdiControl(0u64 as u8)
    }
}
impl core::fmt::Debug for PdiControl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PdiControl")
            .field("bf0", &self.bf0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PdiControl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PdiControl {
            bf0: super::vals::PdiControlBf0,
        }
        let proxy = PdiControl { bf0: self.bf0() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PDI Information"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PdiInformation(pub u16);
impl PdiInformation {
    #[doc = "PDI function"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::PdiInformationBf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::PdiInformationBf0::from_bits(val as u8)
    }
    #[doc = "PDI function"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::PdiInformationBf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "ESC configuration area loaded from EEPROM:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> super::vals::PdiInformationBf1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::PdiInformationBf1::from_bits(val as u8)
    }
    #[doc = "ESC configuration area loaded from EEPROM:"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: super::vals::PdiInformationBf1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "PDI active:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf2(&self) -> super::vals::PdiInformationBf2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::PdiInformationBf2::from_bits(val as u8)
    }
    #[doc = "PDI active:"]
    #[inline(always)]
    pub const fn set_bf2(&mut self, val: super::vals::PdiInformationBf2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "PDI configuration invalid:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf3(&self) -> super::vals::PdiInformationBf3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::PdiInformationBf3::from_bits(val as u8)
    }
    #[doc = "PDI configuration invalid:"]
    #[inline(always)]
    pub const fn set_bf3(&mut self, val: super::vals::PdiInformationBf3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn bf4(&self) -> u16 {
        let val = (self.0 >> 4usize) & 0x0fff;
        val as u16
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_bf4(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u16) & 0x0fff) << 4usize);
    }
}
impl Default for PdiInformation {
    #[inline(always)]
    fn default() -> PdiInformation {
        PdiInformation(0u64 as u16)
    }
}
impl core::fmt::Debug for PdiInformation {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PdiInformation")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .field("bf2", &self.bf2())
            .field("bf3", &self.bf3())
            .field("bf4", &self.bf4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PdiInformation {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PdiInformation {
            bf0: super::vals::PdiInformationBf0,
            bf1: super::vals::PdiInformationBf1,
            bf2: super::vals::PdiInformationBf2,
            bf3: super::vals::PdiInformationBf3,
            bf4: u16,
        }
        let proxy = PdiInformation {
            bf0: self.bf0(),
            bf1: self.bf1(),
            bf2: self.bf2(),
            bf3: self.bf3(),
            bf4: self.bf4(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Register PDI On-chip bus configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PdiOnChipBusConfiguration(pub u8);
impl PdiOnChipBusConfiguration {
    #[doc = "On-chip bus clock:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::PdiOnChipBusConfigurationBf0 {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::PdiOnChipBusConfigurationBf0::from_bits(val as u8)
    }
    #[doc = "On-chip bus clock:"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::PdiOnChipBusConfigurationBf0) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u8) & 0x1f) << 0usize);
    }
    #[doc = "On-chip bus"]
    #[must_use]
    #[inline(always)]
    pub const fn bf5(&self) -> super::vals::PdiOnChipBusConfigurationBf5 {
        let val = (self.0 >> 5usize) & 0x07;
        super::vals::PdiOnChipBusConfigurationBf5::from_bits(val as u8)
    }
    #[doc = "On-chip bus"]
    #[inline(always)]
    pub const fn set_bf5(&mut self, val: super::vals::PdiOnChipBusConfigurationBf5) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val.to_bits() as u8) & 0x07) << 5usize);
    }
}
impl Default for PdiOnChipBusConfiguration {
    #[inline(always)]
    fn default() -> PdiOnChipBusConfiguration {
        PdiOnChipBusConfiguration(132u64 as u8)
    }
}
impl core::fmt::Debug for PdiOnChipBusConfiguration {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PdiOnChipBusConfiguration")
            .field("bf0", &self.bf0())
            .field("bf5", &self.bf5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PdiOnChipBusConfiguration {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PdiOnChipBusConfiguration {
            bf0: super::vals::PdiOnChipBusConfigurationBf0,
            bf5: super::vals::PdiOnChipBusConfigurationBf5,
        }
        let proxy = PdiOnChipBusConfiguration {
            bf0: self.bf0(),
            bf5: self.bf5(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Register PDI On-chip bus extended configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PdiOnChipBusExtendedConfiguration(pub u16);
impl PdiOnChipBusExtendedConfiguration {
    #[doc = "Read prefetch size (in cycles of PDI width):"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::PdiOnChipBusExtendedConfigurationBf0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::PdiOnChipBusExtendedConfigurationBf0::from_bits(val as u8)
    }
    #[doc = "Read prefetch size (in cycles of PDI width):"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::PdiOnChipBusExtendedConfigurationBf0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn bf2(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_bf2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u16) & 0x3f) << 2usize);
    }
    #[doc = "On-chip bus sub-type for AXI:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf8(&self) -> super::vals::PdiOnChipBusExtendedConfigurationBf8 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::PdiOnChipBusExtendedConfigurationBf8::from_bits(val as u8)
    }
    #[doc = "On-chip bus sub-type for AXI:"]
    #[inline(always)]
    pub const fn set_bf8(&mut self, val: super::vals::PdiOnChipBusExtendedConfigurationBf8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u16) & 0x07) << 8usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn bf11(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_bf11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for PdiOnChipBusExtendedConfiguration {
    #[inline(always)]
    fn default() -> PdiOnChipBusExtendedConfiguration {
        PdiOnChipBusExtendedConfiguration(0u64 as u16)
    }
}
impl core::fmt::Debug for PdiOnChipBusExtendedConfiguration {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PdiOnChipBusExtendedConfiguration")
            .field("bf0", &self.bf0())
            .field("bf2", &self.bf2())
            .field("bf8", &self.bf8())
            .field("bf11", &self.bf11())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PdiOnChipBusExtendedConfiguration {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PdiOnChipBusExtendedConfiguration {
            bf0: super::vals::PdiOnChipBusExtendedConfigurationBf0,
            bf2: u8,
            bf8: super::vals::PdiOnChipBusExtendedConfigurationBf8,
            bf11: u8,
        }
        let proxy = PdiOnChipBusExtendedConfiguration {
            bf0: self.bf0(),
            bf2: self.bf2(),
            bf8: self.bf8(),
            bf11: self.bf11(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PHY Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PhyAddress(pub u8);
impl PhyAddress {
    #[doc = "PHY Address"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "PHY Address"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf5(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u8) & 0x03) << 5usize);
    }
    #[doc = "Show configured PHY address of port 0-3 in register 0x0510\\[7:3\\]. This is used if the PHY addresses are not consecutive."]
    #[must_use]
    #[inline(always)]
    pub const fn bf7(&self) -> super::vals::PhyAddressBf7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::PhyAddressBf7::from_bits(val as u8)
    }
    #[doc = "Show configured PHY address of port 0-3 in register 0x0510\\[7:3\\]. This is used if the PHY addresses are not consecutive."]
    #[inline(always)]
    pub const fn set_bf7(&mut self, val: super::vals::PhyAddressBf7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for PhyAddress {
    #[inline(always)]
    fn default() -> PhyAddress {
        PhyAddress(0u64 as u8)
    }
}
impl core::fmt::Debug for PhyAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PhyAddress")
            .field("bf0", &self.bf0())
            .field("bf5", &self.bf5())
            .field("bf7", &self.bf7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PhyAddress {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PhyAddress {
            bf0: u8,
            bf5: u8,
            bf7: super::vals::PhyAddressBf7,
        }
        let proxy = PhyAddress {
            bf0: self.bf0(),
            bf5: self.bf5(),
            bf7: self.bf7(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PHY Port"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PhyPortStatus(pub u8);
impl PhyPortStatus {
    #[doc = "Physical link status (PHY status register 1.2):"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::PhyPortStatusBf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::PhyPortStatusBf0::from_bits(val as u8)
    }
    #[doc = "Physical link status (PHY status register 1.2):"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::PhyPortStatusBf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Link status (100 Mbit/s, Full Duplex, Auto negotiation):"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> super::vals::PhyPortStatusBf1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::PhyPortStatusBf1::from_bits(val as u8)
    }
    #[doc = "Link status (100 Mbit/s, Full Duplex, Auto negotiation):"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: super::vals::PhyPortStatusBf1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "Link status error:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf2(&self) -> super::vals::PhyPortStatusBf2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::PhyPortStatusBf2::from_bits(val as u8)
    }
    #[doc = "Link status error:"]
    #[inline(always)]
    pub const fn set_bf2(&mut self, val: super::vals::PhyPortStatusBf2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u8) & 0x01) << 2usize);
    }
    #[doc = "Read error:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf3(&self) -> super::vals::PhyPortStatusBf3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::PhyPortStatusBf3::from_bits(val as u8)
    }
    #[doc = "Read error:"]
    #[inline(always)]
    pub const fn set_bf3(&mut self, val: super::vals::PhyPortStatusBf3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "Link partner error:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf4(&self) -> super::vals::PhyPortStatusBf4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::PhyPortStatusBf4::from_bits(val as u8)
    }
    #[doc = "Link partner error:"]
    #[inline(always)]
    pub const fn set_bf4(&mut self, val: super::vals::PhyPortStatusBf4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
    #[doc = "PHY configuration updated"]
    #[must_use]
    #[inline(always)]
    pub const fn bf5(&self) -> super::vals::PhyPortStatusBf5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::PhyPortStatusBf5::from_bits(val as u8)
    }
    #[doc = "PHY configuration updated"]
    #[inline(always)]
    pub const fn set_bf5(&mut self, val: super::vals::PhyPortStatusBf5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn bf6(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_bf6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
    }
}
impl Default for PhyPortStatus {
    #[inline(always)]
    fn default() -> PhyPortStatus {
        PhyPortStatus(0u64 as u8)
    }
}
impl core::fmt::Debug for PhyPortStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PhyPortStatus")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .field("bf2", &self.bf2())
            .field("bf3", &self.bf3())
            .field("bf4", &self.bf4())
            .field("bf5", &self.bf5())
            .field("bf6", &self.bf6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PhyPortStatus {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PhyPortStatus {
            bf0: super::vals::PhyPortStatusBf0,
            bf1: super::vals::PhyPortStatusBf1,
            bf2: super::vals::PhyPortStatusBf2,
            bf3: super::vals::PhyPortStatusBf3,
            bf4: super::vals::PhyPortStatusBf4,
            bf5: super::vals::PhyPortStatusBf5,
            bf6: u8,
        }
        let proxy = PhyPortStatus {
            bf0: self.bf0(),
            bf1: self.bf1(),
            bf2: self.bf2(),
            bf3: self.bf3(),
            bf4: self.bf4(),
            bf5: self.bf5(),
            bf6: self.bf6(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PHY Register Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PhyRegisterAddress(pub u8);
impl PhyRegisterAddress {
    #[doc = "Address of PHY Register that shall be read/written"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Address of PHY Register that shall be read/written"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf5(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u8) & 0x07) << 5usize);
    }
}
impl Default for PhyRegisterAddress {
    #[inline(always)]
    fn default() -> PhyRegisterAddress {
        PhyRegisterAddress(0u64 as u8)
    }
}
impl core::fmt::Debug for PhyRegisterAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PhyRegisterAddress")
            .field("bf0", &self.bf0())
            .field("bf5", &self.bf5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PhyRegisterAddress {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PhyRegisterAddress {
            bf0: u8,
            bf5: u8,
        }
        let proxy = PhyRegisterAddress {
            bf0: self.bf0(),
            bf5: self.bf5(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PortDescriptor(pub u8);
impl PortDescriptor {
    #[doc = "Port 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Port 0"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
    }
    #[doc = "Port 1"]
    #[must_use]
    #[inline(always)]
    pub const fn bf2(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Port 1"]
    #[inline(always)]
    pub const fn set_bf2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn bf4(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_bf4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn bf6(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_bf6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
    }
}
impl Default for PortDescriptor {
    #[inline(always)]
    fn default() -> PortDescriptor {
        PortDescriptor(15u64 as u8)
    }
}
impl core::fmt::Debug for PortDescriptor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PortDescriptor")
            .field("bf0", &self.bf0())
            .field("bf2", &self.bf2())
            .field("bf4", &self.bf4())
            .field("bf6", &self.bf6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PortDescriptor {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PortDescriptor {
            bf0: u8,
            bf2: u8,
            bf4: u8,
            bf6: u8,
        }
        let proxy = PortDescriptor {
            bf0: self.bf0(),
            bf2: self.bf2(),
            bf4: self.bf4(),
            bf6: self.bf6(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Distributed Clocks Receive Times"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ReceiveTimes(pub u32);
impl ReceiveTimes {
    #[doc = "Write"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Write"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Local time at the beginning of the last receive frame containing a write access to register 0x0900."]
    #[must_use]
    #[inline(always)]
    pub const fn bf8(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Local time at the beginning of the last receive frame containing a write access to register 0x0900."]
    #[inline(always)]
    pub const fn set_bf8(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for ReceiveTimes {
    #[inline(always)]
    fn default() -> ReceiveTimes {
        ReceiveTimes(1701344325u64 as u32)
    }
}
impl core::fmt::Debug for ReceiveTimes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ReceiveTimes")
            .field("bf0", &self.bf0())
            .field("bf8", &self.bf8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ReceiveTimes {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ReceiveTimes {
            bf0: u8,
            bf8: u32,
        }
        let proxy = ReceiveTimes {
            bf0: self.bf0(),
            bf8: self.bf8(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Distributed Clocks Receive Times"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ReceiveTimesPdi(pub u32);
impl ReceiveTimesPdi {
    #[doc = "Write"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Write"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Local time at the beginning of the last receive frame containing a write access to register 0x0900."]
    #[must_use]
    #[inline(always)]
    pub const fn bf8(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Local time at the beginning of the last receive frame containing a write access to register 0x0900."]
    #[inline(always)]
    pub const fn set_bf8(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for ReceiveTimesPdi {
    #[inline(always)]
    fn default() -> ReceiveTimesPdi {
        ReceiveTimesPdi(1701344325u64 as u32)
    }
}
impl core::fmt::Debug for ReceiveTimesPdi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ReceiveTimesPdi")
            .field("bf0", &self.bf0())
            .field("bf8", &self.bf8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ReceiveTimesPdi {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ReceiveTimesPdi {
            bf0: u8,
            bf8: u32,
        }
        let proxy = ReceiveTimesPdi {
            bf0: self.bf0(),
            bf8: self.bf8(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "EEPROM PDI Access State"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RegisterEepromPdiAccessState(pub u8);
impl RegisterEepromPdiAccessState {
    #[doc = "Access to EEPROM:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::RegisterEepromPdiAccessStateBf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RegisterEepromPdiAccessStateBf0::from_bits(val as u8)
    }
    #[doc = "Access to EEPROM:"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::RegisterEepromPdiAccessStateBf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u8) & 0x7f) << 1usize);
    }
}
impl Default for RegisterEepromPdiAccessState {
    #[inline(always)]
    fn default() -> RegisterEepromPdiAccessState {
        RegisterEepromPdiAccessState(0u64 as u8)
    }
}
impl core::fmt::Debug for RegisterEepromPdiAccessState {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RegisterEepromPdiAccessState")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RegisterEepromPdiAccessState {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct RegisterEepromPdiAccessState {
            bf0: super::vals::RegisterEepromPdiAccessStateBf0,
            bf1: u8,
        }
        let proxy = RegisterEepromPdiAccessState {
            bf0: self.bf0(),
            bf1: self.bf1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "EEPROM PDI Access State"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RegisterEepromPdiAccessStatePdi(pub u8);
impl RegisterEepromPdiAccessStatePdi {
    #[doc = "Access to EEPROM:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::RegisterEepromPdiAccessStatePdiBf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RegisterEepromPdiAccessStatePdiBf0::from_bits(val as u8)
    }
    #[doc = "Access to EEPROM:"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::RegisterEepromPdiAccessStatePdiBf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u8) & 0x7f) << 1usize);
    }
}
impl Default for RegisterEepromPdiAccessStatePdi {
    #[inline(always)]
    fn default() -> RegisterEepromPdiAccessStatePdi {
        RegisterEepromPdiAccessStatePdi(0u64 as u8)
    }
}
impl core::fmt::Debug for RegisterEepromPdiAccessStatePdi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RegisterEepromPdiAccessStatePdi")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RegisterEepromPdiAccessStatePdi {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct RegisterEepromPdiAccessStatePdi {
            bf0: super::vals::RegisterEepromPdiAccessStatePdiBf0,
            bf1: u8,
        }
        let proxy = RegisterEepromPdiAccessStatePdi {
            bf0: self.bf0(),
            bf1: self.bf1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Register Write Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RegisterWriteEnable(pub u8);
impl RegisterWriteEnable {
    #[doc = "Register Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Register Write Enable."]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u8) & 0x7f) << 1usize);
    }
}
impl Default for RegisterWriteEnable {
    #[inline(always)]
    fn default() -> RegisterWriteEnable {
        RegisterWriteEnable(0u64 as u8)
    }
}
impl core::fmt::Debug for RegisterWriteEnable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RegisterWriteEnable")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RegisterWriteEnable {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct RegisterWriteEnable {
            bf0: bool,
            bf1: u8,
        }
        let proxy = RegisterWriteEnable {
            bf0: self.bf0(),
            bf1: self.bf1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Register Write Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RegisterWriteEnablePdi(pub u8);
impl RegisterWriteEnablePdi {
    #[doc = "Register Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Register Write Enable."]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u8) & 0x7f) << 1usize);
    }
}
impl Default for RegisterWriteEnablePdi {
    #[inline(always)]
    fn default() -> RegisterWriteEnablePdi {
        RegisterWriteEnablePdi(0u64 as u8)
    }
}
impl core::fmt::Debug for RegisterWriteEnablePdi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RegisterWriteEnablePdi")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RegisterWriteEnablePdi {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct RegisterWriteEnablePdi {
            bf0: bool,
            bf1: u8,
        }
        let proxy = RegisterWriteEnablePdi {
            bf0: self.bf0(),
            bf1: self.bf1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Register Write Protection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RegisterWriteProtection(pub u8);
impl RegisterWriteProtection {
    #[doc = "Register write protection."]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::RegisterWriteProtectionBf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RegisterWriteProtectionBf0::from_bits(val as u8)
    }
    #[doc = "Register write protection."]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::RegisterWriteProtectionBf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u8) & 0x7f) << 1usize);
    }
}
impl Default for RegisterWriteProtection {
    #[inline(always)]
    fn default() -> RegisterWriteProtection {
        RegisterWriteProtection(0u64 as u8)
    }
}
impl core::fmt::Debug for RegisterWriteProtection {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RegisterWriteProtection")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RegisterWriteProtection {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct RegisterWriteProtection {
            bf0: super::vals::RegisterWriteProtectionBf0,
            bf1: u8,
        }
        let proxy = RegisterWriteProtection {
            bf0: self.bf0(),
            bf1: self.bf1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Register Write Protection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RegisterWriteProtectionPdi(pub u8);
impl RegisterWriteProtectionPdi {
    #[doc = "Register write protection."]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::RegisterWriteProtectionPdiBf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RegisterWriteProtectionPdiBf0::from_bits(val as u8)
    }
    #[doc = "Register write protection."]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::RegisterWriteProtectionPdiBf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u8) & 0x7f) << 1usize);
    }
}
impl Default for RegisterWriteProtectionPdi {
    #[inline(always)]
    fn default() -> RegisterWriteProtectionPdi {
        RegisterWriteProtectionPdi(0u64 as u8)
    }
}
impl core::fmt::Debug for RegisterWriteProtectionPdi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RegisterWriteProtectionPdi")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RegisterWriteProtectionPdi {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct RegisterWriteProtectionPdi {
            bf0: super::vals::RegisterWriteProtectionPdiBf0,
            bf1: u8,
        }
        let proxy = RegisterWriteProtectionPdi {
            bf0: self.bf0(),
            bf1: self.bf1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "RUN LED Override"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RunLedOverride(pub u8);
impl RunLedOverride {
    #[doc = "LED code and AL Status"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "LED code and AL Status"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Enable Override:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf4(&self) -> super::vals::RunLedOverrideBf4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::RunLedOverrideBf4::from_bits(val as u8)
    }
    #[doc = "Enable Override:"]
    #[inline(always)]
    pub const fn set_bf4(&mut self, val: super::vals::RunLedOverrideBf4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf5(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u8) & 0x07) << 5usize);
    }
}
impl Default for RunLedOverride {
    #[inline(always)]
    fn default() -> RunLedOverride {
        RunLedOverride(0u64 as u8)
    }
}
impl core::fmt::Debug for RunLedOverride {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RunLedOverride")
            .field("bf0", &self.bf0())
            .field("bf4", &self.bf4())
            .field("bf5", &self.bf5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RunLedOverride {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct RunLedOverride {
            bf0: u8,
            bf4: super::vals::RunLedOverrideBf4,
            bf5: u8,
        }
        let proxy = RunLedOverride {
            bf0: self.bf0(),
            bf4: self.bf4(),
            bf5: self.bf5(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "RX Error Counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxErrorCounterPort(pub u16);
impl RxErrorCounterPort {
    #[doc = "Invalid frame counter of Port y (counting is stopped when 0xFF is reached)."]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Invalid frame counter of Port y (counting is stopped when 0xFF is reached)."]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "RX Error counter of Port y (counting is stopped when 0xFF is reached)."]
    #[must_use]
    #[inline(always)]
    pub const fn bf8(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "RX Error counter of Port y (counting is stopped when 0xFF is reached)."]
    #[inline(always)]
    pub const fn set_bf8(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for RxErrorCounterPort {
    #[inline(always)]
    fn default() -> RxErrorCounterPort {
        RxErrorCounterPort(0u64 as u16)
    }
}
impl core::fmt::Debug for RxErrorCounterPort {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RxErrorCounterPort")
            .field("bf0", &self.bf0())
            .field("bf8", &self.bf8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RxErrorCounterPort {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct RxErrorCounterPort {
            bf0: u8,
            bf8: u8,
        }
        let proxy = RxErrorCounterPort {
            bf0: self.bf0(),
            bf8: self.bf8(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "RX Error Counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxErrorCounterPortPdi(pub u16);
impl RxErrorCounterPortPdi {
    #[doc = "Invalid frame counter of Port y (counting is stopped when 0xFF is reached)."]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Invalid frame counter of Port y (counting is stopped when 0xFF is reached)."]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "RX Error counter of Port y (counting is stopped when 0xFF is reached)."]
    #[must_use]
    #[inline(always)]
    pub const fn bf8(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "RX Error counter of Port y (counting is stopped when 0xFF is reached)."]
    #[inline(always)]
    pub const fn set_bf8(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for RxErrorCounterPortPdi {
    #[inline(always)]
    fn default() -> RxErrorCounterPortPdi {
        RxErrorCounterPortPdi(0u64 as u16)
    }
}
impl core::fmt::Debug for RxErrorCounterPortPdi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RxErrorCounterPortPdi")
            .field("bf0", &self.bf0())
            .field("bf8", &self.bf8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RxErrorCounterPortPdi {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct RxErrorCounterPortPdi {
            bf0: u8,
            bf8: u8,
        }
        let proxy = RxErrorCounterPortPdi {
            bf0: self.bf0(),
            bf8: self.bf8(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Register Speed Counter Filter Depth"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpeedCounterFilterDepth(pub u8);
impl SpeedCounterFilterDepth {
    #[doc = "Filter depth for averaging the clock period deviation"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Filter depth for averaging the clock period deviation"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf4(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for SpeedCounterFilterDepth {
    #[inline(always)]
    fn default() -> SpeedCounterFilterDepth {
        SpeedCounterFilterDepth(12u64 as u8)
    }
}
impl core::fmt::Debug for SpeedCounterFilterDepth {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SpeedCounterFilterDepth")
            .field("bf0", &self.bf0())
            .field("bf4", &self.bf4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SpeedCounterFilterDepth {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SpeedCounterFilterDepth {
            bf0: u8,
            bf4: u8,
        }
        let proxy = SpeedCounterFilterDepth {
            bf0: self.bf0(),
            bf4: self.bf4(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Register Speed Counter Start"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpeedCounterStart(pub u16);
impl SpeedCounterStart {
    #[doc = "Bandwidth for adjustment of local copy of system Time (larger values -> smaller bandwidth and smoother adjustment)"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Bandwidth for adjustment of local copy of system Time (larger values -> smaller bandwidth and smoother adjustment)"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u16) & 0x7fff) << 0usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for SpeedCounterStart {
    #[inline(always)]
    fn default() -> SpeedCounterStart {
        SpeedCounterStart(4096u64 as u16)
    }
}
impl core::fmt::Debug for SpeedCounterStart {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SpeedCounterStart")
            .field("bf0", &self.bf0())
            .field("bf15", &self.bf15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SpeedCounterStart {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SpeedCounterStart {
            bf0: u16,
            bf15: bool,
        }
        let proxy = SpeedCounterStart {
            bf0: self.bf0(),
            bf15: self.bf15(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PDI Configuration Sync Latch 1 and 0 PDI Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SyncLatch1And0PdiConfiguration(pub u8);
impl SyncLatch1And0PdiConfiguration {
    #[doc = "SYNC0 output driver/polarity:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::SyncLatch1And0PdiConfigurationBf0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SyncLatch1And0PdiConfigurationBf0::from_bits(val as u8)
    }
    #[doc = "SYNC0 output driver/polarity:"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::SyncLatch1And0PdiConfigurationBf0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u8) & 0x03) << 0usize);
    }
    #[doc = "SYNC0/LATCH0 configuration*:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf2(&self) -> super::vals::SyncLatch1And0PdiConfigurationBf2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SyncLatch1And0PdiConfigurationBf2::from_bits(val as u8)
    }
    #[doc = "SYNC0/LATCH0 configuration*:"]
    #[inline(always)]
    pub const fn set_bf2(&mut self, val: super::vals::SyncLatch1And0PdiConfigurationBf2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u8) & 0x01) << 2usize);
    }
    #[doc = "SYNC0 mapped to AL Event Request register 0x0220\\[2\\]:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf3(&self) -> super::vals::SyncLatch1And0PdiConfigurationBf3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SyncLatch1And0PdiConfigurationBf3::from_bits(val as u8)
    }
    #[doc = "SYNC0 mapped to AL Event Request register 0x0220\\[2\\]:"]
    #[inline(always)]
    pub const fn set_bf3(&mut self, val: super::vals::SyncLatch1And0PdiConfigurationBf3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "SYNC1 output driver/polarity:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf4(&self) -> super::vals::SyncLatch1And0PdiConfigurationBf4 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SyncLatch1And0PdiConfigurationBf4::from_bits(val as u8)
    }
    #[doc = "SYNC1 output driver/polarity:"]
    #[inline(always)]
    pub const fn set_bf4(&mut self, val: super::vals::SyncLatch1And0PdiConfigurationBf4) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u8) & 0x03) << 4usize);
    }
    #[doc = "SYNC1/LATCH1 configuration*"]
    #[must_use]
    #[inline(always)]
    pub const fn bf6(&self) -> super::vals::SyncLatch1And0PdiConfigurationBf6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::SyncLatch1And0PdiConfigurationBf6::from_bits(val as u8)
    }
    #[doc = "SYNC1/LATCH1 configuration*"]
    #[inline(always)]
    pub const fn set_bf6(&mut self, val: super::vals::SyncLatch1And0PdiConfigurationBf6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "SYNC1 mapped to AL Event Request register 0x0220\\[3\\]:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf7(&self) -> super::vals::SyncLatch1And0PdiConfigurationBf7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::SyncLatch1And0PdiConfigurationBf7::from_bits(val as u8)
    }
    #[doc = "SYNC1 mapped to AL Event Request register 0x0220\\[3\\]:"]
    #[inline(always)]
    pub const fn set_bf7(&mut self, val: super::vals::SyncLatch1And0PdiConfigurationBf7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for SyncLatch1And0PdiConfiguration {
    #[inline(always)]
    fn default() -> SyncLatch1And0PdiConfiguration {
        SyncLatch1And0PdiConfiguration(238u64 as u8)
    }
}
impl core::fmt::Debug for SyncLatch1And0PdiConfiguration {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SyncLatch1And0PdiConfiguration")
            .field("bf0", &self.bf0())
            .field("bf2", &self.bf2())
            .field("bf3", &self.bf3())
            .field("bf4", &self.bf4())
            .field("bf6", &self.bf6())
            .field("bf7", &self.bf7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SyncLatch1And0PdiConfiguration {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SyncLatch1And0PdiConfiguration {
            bf0: super::vals::SyncLatch1And0PdiConfigurationBf0,
            bf2: super::vals::SyncLatch1And0PdiConfigurationBf2,
            bf3: super::vals::SyncLatch1And0PdiConfigurationBf3,
            bf4: super::vals::SyncLatch1And0PdiConfigurationBf4,
            bf6: super::vals::SyncLatch1And0PdiConfigurationBf6,
            bf7: super::vals::SyncLatch1And0PdiConfigurationBf7,
        }
        let proxy = SyncLatch1And0PdiConfiguration {
            bf0: self.bf0(),
            bf2: self.bf2(),
            bf3: self.bf3(),
            bf4: self.bf4(),
            bf6: self.bf6(),
            bf7: self.bf7(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Register Activate SyncManager"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SyncmanagerActivate(pub u8);
impl SyncmanagerActivate {
    #[doc = "SyncManager Enable/Disable:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::SyncmanagerActivateBf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SyncmanagerActivateBf0::from_bits(val as u8)
    }
    #[doc = "SyncManager Enable/Disable:"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::SyncmanagerActivateBf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Repeat Request:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Repeat Request:"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf2(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x0f;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u8) & 0x0f) << 2usize);
    }
    #[doc = "Latch Event ECAT:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf6(&self) -> super::vals::SyncmanagerActivateBf6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::SyncmanagerActivateBf6::from_bits(val as u8)
    }
    #[doc = "Latch Event ECAT:"]
    #[inline(always)]
    pub const fn set_bf6(&mut self, val: super::vals::SyncmanagerActivateBf6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Latch Event PDI:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf7(&self) -> super::vals::SyncmanagerActivateBf7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::SyncmanagerActivateBf7::from_bits(val as u8)
    }
    #[doc = "Latch Event PDI:"]
    #[inline(always)]
    pub const fn set_bf7(&mut self, val: super::vals::SyncmanagerActivateBf7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for SyncmanagerActivate {
    #[inline(always)]
    fn default() -> SyncmanagerActivate {
        SyncmanagerActivate(0u64 as u8)
    }
}
impl core::fmt::Debug for SyncmanagerActivate {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SyncmanagerActivate")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .field("bf2", &self.bf2())
            .field("bf6", &self.bf6())
            .field("bf7", &self.bf7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SyncmanagerActivate {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SyncmanagerActivate {
            bf0: super::vals::SyncmanagerActivateBf0,
            bf1: bool,
            bf2: u8,
            bf6: super::vals::SyncmanagerActivateBf6,
            bf7: super::vals::SyncmanagerActivateBf7,
        }
        let proxy = SyncmanagerActivate {
            bf0: self.bf0(),
            bf1: self.bf1(),
            bf2: self.bf2(),
            bf6: self.bf6(),
            bf7: self.bf7(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Register Activate SyncManager"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SyncmanagerActivatePdi(pub u8);
impl SyncmanagerActivatePdi {
    #[doc = "SyncManager Enable/Disable:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::SyncmanagerActivatePdiBf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SyncmanagerActivatePdiBf0::from_bits(val as u8)
    }
    #[doc = "SyncManager Enable/Disable:"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::SyncmanagerActivatePdiBf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Repeat Request:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Repeat Request:"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf2(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x0f;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u8) & 0x0f) << 2usize);
    }
    #[doc = "Latch Event ECAT:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf6(&self) -> super::vals::SyncmanagerActivatePdiBf6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::SyncmanagerActivatePdiBf6::from_bits(val as u8)
    }
    #[doc = "Latch Event ECAT:"]
    #[inline(always)]
    pub const fn set_bf6(&mut self, val: super::vals::SyncmanagerActivatePdiBf6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Latch Event PDI:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf7(&self) -> super::vals::SyncmanagerActivatePdiBf7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::SyncmanagerActivatePdiBf7::from_bits(val as u8)
    }
    #[doc = "Latch Event PDI:"]
    #[inline(always)]
    pub const fn set_bf7(&mut self, val: super::vals::SyncmanagerActivatePdiBf7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for SyncmanagerActivatePdi {
    #[inline(always)]
    fn default() -> SyncmanagerActivatePdi {
        SyncmanagerActivatePdi(0u64 as u8)
    }
}
impl core::fmt::Debug for SyncmanagerActivatePdi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SyncmanagerActivatePdi")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .field("bf2", &self.bf2())
            .field("bf6", &self.bf6())
            .field("bf7", &self.bf7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SyncmanagerActivatePdi {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SyncmanagerActivatePdi {
            bf0: super::vals::SyncmanagerActivatePdiBf0,
            bf1: bool,
            bf2: u8,
            bf6: super::vals::SyncmanagerActivatePdiBf6,
            bf7: super::vals::SyncmanagerActivatePdiBf7,
        }
        let proxy = SyncmanagerActivatePdi {
            bf0: self.bf0(),
            bf1: self.bf1(),
            bf2: self.bf2(),
            bf6: self.bf6(),
            bf7: self.bf7(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Register Control Register SyncManager"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SyncmanagerControlRegister(pub u8);
impl SyncmanagerControlRegister {
    #[doc = "Operation Mode:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::SyncmanagerControlRegisterBf0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SyncmanagerControlRegisterBf0::from_bits(val as u8)
    }
    #[doc = "Operation Mode:"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::SyncmanagerControlRegisterBf0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u8) & 0x03) << 0usize);
    }
    #[doc = "Direction:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf2(&self) -> super::vals::SyncmanagerControlRegisterBf2 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SyncmanagerControlRegisterBf2::from_bits(val as u8)
    }
    #[doc = "Direction:"]
    #[inline(always)]
    pub const fn set_bf2(&mut self, val: super::vals::SyncmanagerControlRegisterBf2) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u8) & 0x03) << 2usize);
    }
    #[doc = "Interrupt in ECAT Event Request Register:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf4(&self) -> super::vals::SyncmanagerControlRegisterBf4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::SyncmanagerControlRegisterBf4::from_bits(val as u8)
    }
    #[doc = "Interrupt in ECAT Event Request Register:"]
    #[inline(always)]
    pub const fn set_bf4(&mut self, val: super::vals::SyncmanagerControlRegisterBf4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
    #[doc = "Interrupt in AL Event Request Register:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf5(&self) -> super::vals::SyncmanagerControlRegisterBf5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::SyncmanagerControlRegisterBf5::from_bits(val as u8)
    }
    #[doc = "Interrupt in AL Event Request Register:"]
    #[inline(always)]
    pub const fn set_bf5(&mut self, val: super::vals::SyncmanagerControlRegisterBf5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "Watchdog Trigger Enable:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf6(&self) -> super::vals::SyncmanagerControlRegisterBf6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::SyncmanagerControlRegisterBf6::from_bits(val as u8)
    }
    #[doc = "Watchdog Trigger Enable:"]
    #[inline(always)]
    pub const fn set_bf6(&mut self, val: super::vals::SyncmanagerControlRegisterBf6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for SyncmanagerControlRegister {
    #[inline(always)]
    fn default() -> SyncmanagerControlRegister {
        SyncmanagerControlRegister(0u64 as u8)
    }
}
impl core::fmt::Debug for SyncmanagerControlRegister {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SyncmanagerControlRegister")
            .field("bf0", &self.bf0())
            .field("bf2", &self.bf2())
            .field("bf4", &self.bf4())
            .field("bf5", &self.bf5())
            .field("bf6", &self.bf6())
            .field("bf7", &self.bf7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SyncmanagerControlRegister {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SyncmanagerControlRegister {
            bf0: super::vals::SyncmanagerControlRegisterBf0,
            bf2: super::vals::SyncmanagerControlRegisterBf2,
            bf4: super::vals::SyncmanagerControlRegisterBf4,
            bf5: super::vals::SyncmanagerControlRegisterBf5,
            bf6: super::vals::SyncmanagerControlRegisterBf6,
            bf7: bool,
        }
        let proxy = SyncmanagerControlRegister {
            bf0: self.bf0(),
            bf2: self.bf2(),
            bf4: self.bf4(),
            bf5: self.bf5(),
            bf6: self.bf6(),
            bf7: self.bf7(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Register Control Register SyncManager"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SyncmanagerControlRegisterPdi(pub u8);
impl SyncmanagerControlRegisterPdi {
    #[doc = "Operation Mode:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::SyncmanagerControlRegisterPdiBf0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SyncmanagerControlRegisterPdiBf0::from_bits(val as u8)
    }
    #[doc = "Operation Mode:"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::SyncmanagerControlRegisterPdiBf0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u8) & 0x03) << 0usize);
    }
    #[doc = "Direction:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf2(&self) -> super::vals::SyncmanagerControlRegisterPdiBf2 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SyncmanagerControlRegisterPdiBf2::from_bits(val as u8)
    }
    #[doc = "Direction:"]
    #[inline(always)]
    pub const fn set_bf2(&mut self, val: super::vals::SyncmanagerControlRegisterPdiBf2) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u8) & 0x03) << 2usize);
    }
    #[doc = "Interrupt in ECAT Event Request Register:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf4(&self) -> super::vals::SyncmanagerControlRegisterPdiBf4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::SyncmanagerControlRegisterPdiBf4::from_bits(val as u8)
    }
    #[doc = "Interrupt in ECAT Event Request Register:"]
    #[inline(always)]
    pub const fn set_bf4(&mut self, val: super::vals::SyncmanagerControlRegisterPdiBf4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
    #[doc = "Interrupt in AL Event Request Register:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf5(&self) -> super::vals::SyncmanagerControlRegisterPdiBf5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::SyncmanagerControlRegisterPdiBf5::from_bits(val as u8)
    }
    #[doc = "Interrupt in AL Event Request Register:"]
    #[inline(always)]
    pub const fn set_bf5(&mut self, val: super::vals::SyncmanagerControlRegisterPdiBf5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "Watchdog Trigger Enable:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf6(&self) -> super::vals::SyncmanagerControlRegisterPdiBf6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::SyncmanagerControlRegisterPdiBf6::from_bits(val as u8)
    }
    #[doc = "Watchdog Trigger Enable:"]
    #[inline(always)]
    pub const fn set_bf6(&mut self, val: super::vals::SyncmanagerControlRegisterPdiBf6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for SyncmanagerControlRegisterPdi {
    #[inline(always)]
    fn default() -> SyncmanagerControlRegisterPdi {
        SyncmanagerControlRegisterPdi(0u64 as u8)
    }
}
impl core::fmt::Debug for SyncmanagerControlRegisterPdi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SyncmanagerControlRegisterPdi")
            .field("bf0", &self.bf0())
            .field("bf2", &self.bf2())
            .field("bf4", &self.bf4())
            .field("bf5", &self.bf5())
            .field("bf6", &self.bf6())
            .field("bf7", &self.bf7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SyncmanagerControlRegisterPdi {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SyncmanagerControlRegisterPdi {
            bf0: super::vals::SyncmanagerControlRegisterPdiBf0,
            bf2: super::vals::SyncmanagerControlRegisterPdiBf2,
            bf4: super::vals::SyncmanagerControlRegisterPdiBf4,
            bf5: super::vals::SyncmanagerControlRegisterPdiBf5,
            bf6: super::vals::SyncmanagerControlRegisterPdiBf6,
            bf7: bool,
        }
        let proxy = SyncmanagerControlRegisterPdi {
            bf0: self.bf0(),
            bf2: self.bf2(),
            bf4: self.bf4(),
            bf5: self.bf5(),
            bf6: self.bf6(),
            bf7: self.bf7(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Register PDI Control SyncManager"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SyncmanagerPdiControl(pub u8);
impl SyncmanagerPdiControl {
    #[doc = "Deactivate SyncManager:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Deactivate SyncManager:"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Repeat Ack:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Repeat Ack:"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf2(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u8) & 0x3f) << 2usize);
    }
}
impl Default for SyncmanagerPdiControl {
    #[inline(always)]
    fn default() -> SyncmanagerPdiControl {
        SyncmanagerPdiControl(0u64 as u8)
    }
}
impl core::fmt::Debug for SyncmanagerPdiControl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SyncmanagerPdiControl")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .field("bf2", &self.bf2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SyncmanagerPdiControl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SyncmanagerPdiControl {
            bf0: bool,
            bf1: bool,
            bf2: u8,
        }
        let proxy = SyncmanagerPdiControl {
            bf0: self.bf0(),
            bf1: self.bf1(),
            bf2: self.bf2(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Register PDI Control SyncManager"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SyncmanagerPdiControlPdi(pub u8);
impl SyncmanagerPdiControlPdi {
    #[doc = "Deactivate SyncManager:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Deactivate SyncManager:"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Repeat Ack:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Repeat Ack:"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf2(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u8) & 0x3f) << 2usize);
    }
}
impl Default for SyncmanagerPdiControlPdi {
    #[inline(always)]
    fn default() -> SyncmanagerPdiControlPdi {
        SyncmanagerPdiControlPdi(0u64 as u8)
    }
}
impl core::fmt::Debug for SyncmanagerPdiControlPdi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SyncmanagerPdiControlPdi")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .field("bf2", &self.bf2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SyncmanagerPdiControlPdi {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SyncmanagerPdiControlPdi {
            bf0: bool,
            bf1: bool,
            bf2: u8,
        }
        let proxy = SyncmanagerPdiControlPdi {
            bf0: self.bf0(),
            bf1: self.bf1(),
            bf2: self.bf2(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Register Status Register SyncManager"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SyncmanagerStatus(pub u8);
impl SyncmanagerStatus {
    #[doc = "Interrupt Write:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::SyncmanagerStatusBf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SyncmanagerStatusBf0::from_bits(val as u8)
    }
    #[doc = "Interrupt Write:"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::SyncmanagerStatusBf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Interrupt Read:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> super::vals::SyncmanagerStatusBf1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SyncmanagerStatusBf1::from_bits(val as u8)
    }
    #[doc = "Interrupt Read:"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: super::vals::SyncmanagerStatusBf1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn bf2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_bf2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "Mailbox mode: mailbox status:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf3(&self) -> super::vals::SyncmanagerStatusBf3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SyncmanagerStatusBf3::from_bits(val as u8)
    }
    #[doc = "Mailbox mode: mailbox status:"]
    #[inline(always)]
    pub const fn set_bf3(&mut self, val: super::vals::SyncmanagerStatusBf3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "Buffered mode: buffer status (last written buffer):"]
    #[must_use]
    #[inline(always)]
    pub const fn bf4(&self) -> super::vals::SyncmanagerStatusBf4 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SyncmanagerStatusBf4::from_bits(val as u8)
    }
    #[doc = "Buffered mode: buffer status (last written buffer):"]
    #[inline(always)]
    pub const fn set_bf4(&mut self, val: super::vals::SyncmanagerStatusBf4) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u8) & 0x03) << 4usize);
    }
    #[doc = "Read buffer in use (opened)"]
    #[must_use]
    #[inline(always)]
    pub const fn bf6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Read buffer in use (opened)"]
    #[inline(always)]
    pub const fn set_bf6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "Write buffer in use (opened)"]
    #[must_use]
    #[inline(always)]
    pub const fn bf7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Write buffer in use (opened)"]
    #[inline(always)]
    pub const fn set_bf7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for SyncmanagerStatus {
    #[inline(always)]
    fn default() -> SyncmanagerStatus {
        SyncmanagerStatus(48u64 as u8)
    }
}
impl core::fmt::Debug for SyncmanagerStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SyncmanagerStatus")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .field("bf2", &self.bf2())
            .field("bf3", &self.bf3())
            .field("bf4", &self.bf4())
            .field("bf6", &self.bf6())
            .field("bf7", &self.bf7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SyncmanagerStatus {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SyncmanagerStatus {
            bf0: super::vals::SyncmanagerStatusBf0,
            bf1: super::vals::SyncmanagerStatusBf1,
            bf2: bool,
            bf3: super::vals::SyncmanagerStatusBf3,
            bf4: super::vals::SyncmanagerStatusBf4,
            bf6: bool,
            bf7: bool,
        }
        let proxy = SyncmanagerStatus {
            bf0: self.bf0(),
            bf1: self.bf1(),
            bf2: self.bf2(),
            bf3: self.bf3(),
            bf4: self.bf4(),
            bf6: self.bf6(),
            bf7: self.bf7(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Register System Time Difference"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SystemTimeDifference(pub u32);
impl SystemTimeDifference {
    #[doc = "Mean difference between local copy of system Time and received System Time values"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "Mean difference between local copy of system Time and received System Time values"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
    }
    #[doc = "Bit field access for ECAT: r/-"]
    #[must_use]
    #[inline(always)]
    pub const fn bf31(&self) -> super::vals::Bf31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Bf31::from_bits(val as u8)
    }
    #[doc = "Bit field access for ECAT: r/-"]
    #[inline(always)]
    pub const fn set_bf31(&mut self, val: super::vals::Bf31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SystemTimeDifference {
    #[inline(always)]
    fn default() -> SystemTimeDifference {
        SystemTimeDifference(0u64 as u32)
    }
}
impl core::fmt::Debug for SystemTimeDifference {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SystemTimeDifference")
            .field("bf0", &self.bf0())
            .field("bf31", &self.bf31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SystemTimeDifference {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SystemTimeDifference {
            bf0: u32,
            bf31: super::vals::Bf31,
        }
        let proxy = SystemTimeDifference {
            bf0: self.bf0(),
            bf31: self.bf31(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Register System Time Difference Filter Depth"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SystemTimeDifferenceFilterDepth(pub u8);
impl SystemTimeDifferenceFilterDepth {
    #[doc = "Filter depth for averaging the received System Time deviation"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Filter depth for averaging the received System Time deviation"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Reserved, write 0"]
    #[must_use]
    #[inline(always)]
    pub const fn bf4(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Reserved, write 0"]
    #[inline(always)]
    pub const fn set_bf4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for SystemTimeDifferenceFilterDepth {
    #[inline(always)]
    fn default() -> SystemTimeDifferenceFilterDepth {
        SystemTimeDifferenceFilterDepth(4u64 as u8)
    }
}
impl core::fmt::Debug for SystemTimeDifferenceFilterDepth {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SystemTimeDifferenceFilterDepth")
            .field("bf0", &self.bf0())
            .field("bf4", &self.bf4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SystemTimeDifferenceFilterDepth {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SystemTimeDifferenceFilterDepth {
            bf0: u8,
            bf4: u8,
        }
        let proxy = SystemTimeDifferenceFilterDepth {
            bf0: self.bf0(),
            bf4: self.bf4(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Register Pulse Length of SyncSignals"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UniPulseLengthOfSyncsignals(pub u16);
impl UniPulseLengthOfSyncsignals {
    #[doc = "Pulse length of SyncSignals (in Units of 10ns)"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::UniPulseLengthOfSyncsignalsBf0 {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::UniPulseLengthOfSyncsignalsBf0::from_bits(val as u16)
    }
    #[doc = "Pulse length of SyncSignals (in Units of 10ns)"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::UniPulseLengthOfSyncsignalsBf0) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u16) & 0xffff) << 0usize);
    }
}
impl Default for UniPulseLengthOfSyncsignals {
    #[inline(always)]
    fn default() -> UniPulseLengthOfSyncsignals {
        UniPulseLengthOfSyncsignals(10u64 as u16)
    }
}
impl core::fmt::Debug for UniPulseLengthOfSyncsignals {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UniPulseLengthOfSyncsignals")
            .field("bf0", &self.bf0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UniPulseLengthOfSyncsignals {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct UniPulseLengthOfSyncsignals {
            bf0: super::vals::UniPulseLengthOfSyncsignalsBf0,
        }
        let proxy = UniPulseLengthOfSyncsignals { bf0: self.bf0() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Register Activation register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UnitActivationRegister(pub u8);
impl UnitActivationRegister {
    #[doc = "Sync Out Unit activation:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::UnitActivationRegisterBf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::UnitActivationRegisterBf0::from_bits(val as u8)
    }
    #[doc = "Sync Out Unit activation:"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::UnitActivationRegisterBf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "SYNC0 generation:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> super::vals::UnitActivationRegisterBf1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::UnitActivationRegisterBf1::from_bits(val as u8)
    }
    #[doc = "SYNC0 generation:"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: super::vals::UnitActivationRegisterBf1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "SYNC1 generation:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf2(&self) -> super::vals::UnitActivationRegisterBf2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::UnitActivationRegisterBf2::from_bits(val as u8)
    }
    #[doc = "SYNC1 generation:"]
    #[inline(always)]
    pub const fn set_bf2(&mut self, val: super::vals::UnitActivationRegisterBf2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u8) & 0x01) << 2usize);
    }
    #[doc = "Auto-activation by writing Start Time Cyclic Operation (0x0990:0x0997):"]
    #[must_use]
    #[inline(always)]
    pub const fn bf3(&self) -> super::vals::UnitActivationRegisterBf3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::UnitActivationRegisterBf3::from_bits(val as u8)
    }
    #[doc = "Auto-activation by writing Start Time Cyclic Operation (0x0990:0x0997):"]
    #[inline(always)]
    pub const fn set_bf3(&mut self, val: super::vals::UnitActivationRegisterBf3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "Extension of Start Time Cyclic Operation (0x0990:0x0993):"]
    #[must_use]
    #[inline(always)]
    pub const fn bf4(&self) -> super::vals::UnitActivationRegisterBf4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::UnitActivationRegisterBf4::from_bits(val as u8)
    }
    #[doc = "Extension of Start Time Cyclic Operation (0x0990:0x0993):"]
    #[inline(always)]
    pub const fn set_bf4(&mut self, val: super::vals::UnitActivationRegisterBf4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
    #[doc = "Start Time plausibility check:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf5(&self) -> super::vals::UnitActivationRegisterBf5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::UnitActivationRegisterBf5::from_bits(val as u8)
    }
    #[doc = "Start Time plausibility check:"]
    #[inline(always)]
    pub const fn set_bf5(&mut self, val: super::vals::UnitActivationRegisterBf5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "Near future configuration (approx.):"]
    #[must_use]
    #[inline(always)]
    pub const fn bf6(&self) -> super::vals::UnitActivationRegisterBf6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::UnitActivationRegisterBf6::from_bits(val as u8)
    }
    #[doc = "Near future configuration (approx.):"]
    #[inline(always)]
    pub const fn set_bf6(&mut self, val: super::vals::UnitActivationRegisterBf6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "SyncSignal debug pulse (Vasily bit):"]
    #[must_use]
    #[inline(always)]
    pub const fn bf7(&self) -> super::vals::UnitActivationRegisterBf7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::UnitActivationRegisterBf7::from_bits(val as u8)
    }
    #[doc = "SyncSignal debug pulse (Vasily bit):"]
    #[inline(always)]
    pub const fn set_bf7(&mut self, val: super::vals::UnitActivationRegisterBf7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for UnitActivationRegister {
    #[inline(always)]
    fn default() -> UnitActivationRegister {
        UnitActivationRegister(0u64 as u8)
    }
}
impl core::fmt::Debug for UnitActivationRegister {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UnitActivationRegister")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .field("bf2", &self.bf2())
            .field("bf3", &self.bf3())
            .field("bf4", &self.bf4())
            .field("bf5", &self.bf5())
            .field("bf6", &self.bf6())
            .field("bf7", &self.bf7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UnitActivationRegister {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct UnitActivationRegister {
            bf0: super::vals::UnitActivationRegisterBf0,
            bf1: super::vals::UnitActivationRegisterBf1,
            bf2: super::vals::UnitActivationRegisterBf2,
            bf3: super::vals::UnitActivationRegisterBf3,
            bf4: super::vals::UnitActivationRegisterBf4,
            bf5: super::vals::UnitActivationRegisterBf5,
            bf6: super::vals::UnitActivationRegisterBf6,
            bf7: super::vals::UnitActivationRegisterBf7,
        }
        let proxy = UnitActivationRegister {
            bf0: self.bf0(),
            bf1: self.bf1(),
            bf2: self.bf2(),
            bf3: self.bf3(),
            bf4: self.bf4(),
            bf5: self.bf5(),
            bf6: self.bf6(),
            bf7: self.bf7(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Register Activation Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UnitActivationStatus(pub u8);
impl UnitActivationStatus {
    #[doc = "SYNC0 activation state:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::UnitActivationStatusBf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::UnitActivationStatusBf0::from_bits(val as u8)
    }
    #[doc = "SYNC0 activation state:"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::UnitActivationStatusBf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "SYNC1 activation state:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> super::vals::UnitActivationStatusBf1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::UnitActivationStatusBf1::from_bits(val as u8)
    }
    #[doc = "SYNC1 activation state:"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: super::vals::UnitActivationStatusBf1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "Start Time Cyclic Operation (0x0990:0x0997) plausibility check result when Sync Out Unit was activated:"]
    #[must_use]
    #[inline(always)]
    pub const fn bf2(&self) -> super::vals::UnitActivationStatusBf2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::UnitActivationStatusBf2::from_bits(val as u8)
    }
    #[doc = "Start Time Cyclic Operation (0x0990:0x0997) plausibility check result when Sync Out Unit was activated:"]
    #[inline(always)]
    pub const fn set_bf2(&mut self, val: super::vals::UnitActivationStatusBf2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u8) & 0x01) << 2usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn bf3(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_bf3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u8) & 0x1f) << 3usize);
    }
}
impl Default for UnitActivationStatus {
    #[inline(always)]
    fn default() -> UnitActivationStatus {
        UnitActivationStatus(0u64 as u8)
    }
}
impl core::fmt::Debug for UnitActivationStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UnitActivationStatus")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .field("bf2", &self.bf2())
            .field("bf3", &self.bf3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UnitActivationStatus {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct UnitActivationStatus {
            bf0: super::vals::UnitActivationStatusBf0,
            bf1: super::vals::UnitActivationStatusBf1,
            bf2: super::vals::UnitActivationStatusBf2,
            bf3: u8,
        }
        let proxy = UnitActivationStatus {
            bf0: self.bf0(),
            bf1: self.bf1(),
            bf2: self.bf2(),
            bf3: self.bf3(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Register SYNC0 Cycle Time"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UnitSync0CycleTime(pub u32);
impl UnitSync0CycleTime {
    #[doc = "Time between two consecutive SYNC0 pulses in ns."]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::UnitSync0CycleTimeBf0 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::UnitSync0CycleTimeBf0::from_bits(val as u32)
    }
    #[doc = "Time between two consecutive SYNC0 pulses in ns."]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::UnitSync0CycleTimeBf0) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UnitSync0CycleTime {
    #[inline(always)]
    fn default() -> UnitSync0CycleTime {
        UnitSync0CycleTime(0u64 as u32)
    }
}
impl core::fmt::Debug for UnitSync0CycleTime {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UnitSync0CycleTime")
            .field("bf0", &self.bf0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UnitSync0CycleTime {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct UnitSync0CycleTime {
            bf0: super::vals::UnitSync0CycleTimeBf0,
        }
        let proxy = UnitSync0CycleTime { bf0: self.bf0() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Register SYNC0 Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UnitSync0Status(pub u8);
impl UnitSync0Status {
    #[doc = "SYNC0 state for Acknowledge mode."]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SYNC0 state for Acknowledge mode."]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u8) & 0x7f) << 1usize);
    }
}
impl Default for UnitSync0Status {
    #[inline(always)]
    fn default() -> UnitSync0Status {
        UnitSync0Status(0u64 as u8)
    }
}
impl core::fmt::Debug for UnitSync0Status {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UnitSync0Status")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UnitSync0Status {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct UnitSync0Status {
            bf0: bool,
            bf1: u8,
        }
        let proxy = UnitSync0Status {
            bf0: self.bf0(),
            bf1: self.bf1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Register SYNC1 Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UnitSync1Status(pub u8);
impl UnitSync1Status {
    #[doc = "SYNC1 state for Acknowledge mode."]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SYNC1 state for Acknowledge mode."]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u8) & 0x7f) << 1usize);
    }
}
impl Default for UnitSync1Status {
    #[inline(always)]
    fn default() -> UnitSync1Status {
        UnitSync1Status(0u64 as u8)
    }
}
impl core::fmt::Debug for UnitSync1Status {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UnitSync1Status")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UnitSync1Status {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct UnitSync1Status {
            bf0: bool,
            bf1: u8,
        }
        let proxy = UnitSync1Status {
            bf0: self.bf0(),
            bf1: self.bf1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Register Vendor ID IP Core"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VendorIdIpCore(pub u64);
impl VendorIdIpCore {
    #[doc = "Vendor ID"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Vendor ID"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u64) & 0xffff_ffff) << 0usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn bf32(&self) -> u32 {
        let val = (self.0 >> 32usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_bf32(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 32usize)) | (((val as u64) & 0xffff_ffff) << 32usize);
    }
}
impl Default for VendorIdIpCore {
    #[inline(always)]
    fn default() -> VendorIdIpCore {
        VendorIdIpCore(17216961135462248174u64 as u64)
    }
}
impl core::fmt::Debug for VendorIdIpCore {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VendorIdIpCore")
            .field("bf0", &self.bf0())
            .field("bf32", &self.bf32())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VendorIdIpCore {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct VendorIdIpCore {
            bf0: u32,
            bf32: u32,
        }
        let proxy = VendorIdIpCore {
            bf0: self.bf0(),
            bf32: self.bf32(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Watchdog Status Process Data"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WatchdogStatusProcessData(pub u16);
impl WatchdogStatusProcessData {
    #[doc = "Watchdog Status of Process Data (triggered by SyncManagers)"]
    #[must_use]
    #[inline(always)]
    pub const fn bf0(&self) -> super::vals::WatchdogStatusProcessDataBf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::WatchdogStatusProcessDataBf0::from_bits(val as u8)
    }
    #[doc = "Watchdog Status of Process Data (triggered by SyncManagers)"]
    #[inline(always)]
    pub const fn set_bf0(&mut self, val: super::vals::WatchdogStatusProcessDataBf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn bf1(&self) -> u16 {
        let val = (self.0 >> 1usize) & 0x7fff;
        val as u16
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_bf1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 1usize)) | (((val as u16) & 0x7fff) << 1usize);
    }
}
impl Default for WatchdogStatusProcessData {
    #[inline(always)]
    fn default() -> WatchdogStatusProcessData {
        WatchdogStatusProcessData(0u64 as u16)
    }
}
impl core::fmt::Debug for WatchdogStatusProcessData {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WatchdogStatusProcessData")
            .field("bf0", &self.bf0())
            .field("bf1", &self.bf1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WatchdogStatusProcessData {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct WatchdogStatusProcessData {
            bf0: super::vals::WatchdogStatusProcessDataBf0,
            bf1: u16,
        }
        let proxy = WatchdogStatusProcessData {
            bf0: self.bf0(),
            bf1: self.bf1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
