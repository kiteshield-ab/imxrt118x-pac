#[doc = "PCI EA capabilities register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfcEaCap(pub u16);
impl PciCfcEaCap {
    #[doc = "Number of entries Number of entries following the first DW of the capability."]
    #[must_use]
    #[inline(always)]
    pub const fn num_entries(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Number of entries Number of entries following the first DW of the capability."]
    #[inline(always)]
    pub const fn set_num_entries(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u16) & 0x3f) << 0usize);
    }
}
impl Default for PciCfcEaCap {
    #[inline(always)]
    fn default() -> PciCfcEaCap {
        PciCfcEaCap(4u64 as u16)
    }
}
impl core::fmt::Debug for PciCfcEaCap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfcEaCap")
            .field("num_entries", &self.num_entries())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfcEaCap {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfcEaCap {
            num_entries: u8,
        }
        let proxy = PciCfcEaCap {
            num_entries: self.num_entries(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI EA capabilities list register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfcEaCapList(pub u16);
impl PciCfcEaCapList {
    #[doc = "Indicates the Enhanced Allocation (EA) Capability structure. Hardwired to 14h."]
    #[must_use]
    #[inline(always)]
    pub const fn cap_id(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates the Enhanced Allocation (EA) Capability structure. Hardwired to 14h."]
    #[inline(always)]
    pub const fn set_cap_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "This field contains the offset to the next PCI Capability structure or 00h if no other items exist in the linked list of capabilities"]
    #[must_use]
    #[inline(always)]
    pub const fn next_cap_ptr(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "This field contains the offset to the next PCI Capability structure or 00h if no other items exist in the linked list of capabilities"]
    #[inline(always)]
    pub const fn set_next_cap_ptr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for PciCfcEaCapList {
    #[inline(always)]
    fn default() -> PciCfcEaCapList {
        PciCfcEaCapList(20u64 as u16)
    }
}
impl core::fmt::Debug for PciCfcEaCapList {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfcEaCapList")
            .field("cap_id", &self.cap_id())
            .field("next_cap_ptr", &self.next_cap_ptr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfcEaCapList {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfcEaCapList {
            cap_id: u8,
            next_cap_ptr: u8,
        }
        let proxy = PciCfcEaCapList {
            cap_id: self.cap_id(),
            next_cap_ptr: self.next_cap_ptr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI EA per-entry 0 base register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfcEaPe0Base(pub u32);
impl PciCfcEaPe0Base {
    #[doc = "Field size 0 32-bit 1 64-bit"]
    #[must_use]
    #[inline(always)]
    pub const fn s(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Field size 0 32-bit 1 64-bit"]
    #[inline(always)]
    pub const fn set_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Base DW address of the start of the resource range"]
    #[must_use]
    #[inline(always)]
    pub const fn base(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Base DW address of the start of the resource range"]
    #[inline(always)]
    pub const fn set_base(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for PciCfcEaPe0Base {
    #[inline(always)]
    fn default() -> PciCfcEaPe0Base {
        PciCfcEaPe0Base(1622409218u64 as u32)
    }
}
impl core::fmt::Debug for PciCfcEaPe0Base {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfcEaPe0Base")
            .field("s", &self.s())
            .field("base", &self.base())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfcEaPe0Base {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfcEaPe0Base {
            s: bool,
            base: u32,
        }
        let proxy = PciCfcEaPe0Base {
            s: self.s(),
            base: self.base(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI EA per-entry 0 format register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfcEaPe0Fmt(pub u32);
impl PciCfcEaPe0Fmt {
    #[doc = "Entry size Number of DWs following the initial DW in this entry"]
    #[must_use]
    #[inline(always)]
    pub const fn entry_size(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Entry size Number of DWs following the initial DW in this entry"]
    #[inline(always)]
    pub const fn set_entry_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "BAR equivalent indicator This field indicates the equivalent PCI BAR for this entry"]
    #[must_use]
    #[inline(always)]
    pub const fn bei(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "BAR equivalent indicator This field indicates the equivalent PCI BAR for this entry"]
    #[inline(always)]
    pub const fn set_bei(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Primary properties Value (h) Resource Definition 00 Memory Space, Non-Prefetchable"]
    #[must_use]
    #[inline(always)]
    pub const fn prim_prop(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Primary properties Value (h) Resource Definition 00 Memory Space, Non-Prefetchable"]
    #[inline(always)]
    pub const fn set_prim_prop(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Secondary properties Alternative property when system cannot use the primary property."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_prop(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Secondary properties Alternative property when system cannot use the primary property."]
    #[inline(always)]
    pub const fn set_sec_prop(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Writable 1b indicates that the Base, MaxOffset and Field Size fields for this entry are RW, 0b indicates those fields are HWInit"]
    #[must_use]
    #[inline(always)]
    pub const fn writable(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Writable 1b indicates that the Base, MaxOffset and Field Size fields for this entry are RW, 0b indicates those fields are HWInit"]
    #[inline(always)]
    pub const fn set_writable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Enable 0 Disabled 1 Enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable 0 Disabled 1 Enabled"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PciCfcEaPe0Fmt {
    #[inline(always)]
    fn default() -> PciCfcEaPe0Fmt {
        PciCfcEaPe0Fmt(2164195331u64 as u32)
    }
}
impl core::fmt::Debug for PciCfcEaPe0Fmt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfcEaPe0Fmt")
            .field("entry_size", &self.entry_size())
            .field("bei", &self.bei())
            .field("prim_prop", &self.prim_prop())
            .field("sec_prop", &self.sec_prop())
            .field("writable", &self.writable())
            .field("enable", &self.enable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfcEaPe0Fmt {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfcEaPe0Fmt {
            entry_size: u8,
            bei: u8,
            prim_prop: u8,
            sec_prop: u8,
            writable: bool,
            enable: bool,
        }
        let proxy = PciCfcEaPe0Fmt {
            entry_size: self.entry_size(),
            bei: self.bei(),
            prim_prop: self.prim_prop(),
            sec_prop: self.sec_prop(),
            writable: self.writable(),
            enable: self.enable(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI EA per-entry 0 max offset register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfcEaPe0Maxoff(pub u32);
impl PciCfcEaPe0Maxoff {
    #[doc = "Field size 0 32-bit 1 64-bit"]
    #[must_use]
    #[inline(always)]
    pub const fn s(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Field size 0 32-bit 1 64-bit"]
    #[inline(always)]
    pub const fn set_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Max offset The value in the Base field (\\[63:2\\]) plus the value in the MaxOffset field indicates the address of the last included DW of the resource range"]
    #[must_use]
    #[inline(always)]
    pub const fn max_offset(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Max offset The value in the Base field (\\[63:2\\]) plus the value in the MaxOffset field indicates the address of the last included DW of the resource range"]
    #[inline(always)]
    pub const fn set_max_offset(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for PciCfcEaPe0Maxoff {
    #[inline(always)]
    fn default() -> PciCfcEaPe0Maxoff {
        PciCfcEaPe0Maxoff(262140u64 as u32)
    }
}
impl core::fmt::Debug for PciCfcEaPe0Maxoff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfcEaPe0Maxoff")
            .field("s", &self.s())
            .field("max_offset", &self.max_offset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfcEaPe0Maxoff {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfcEaPe0Maxoff {
            s: bool,
            max_offset: u32,
        }
        let proxy = PciCfcEaPe0Maxoff {
            s: self.s(),
            max_offset: self.max_offset(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI EA per-entry 1 base register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfcEaPe1Base(pub u32);
impl PciCfcEaPe1Base {
    #[doc = "Field size 0 32-bit 1 64-bit"]
    #[must_use]
    #[inline(always)]
    pub const fn s(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Field size 0 32-bit 1 64-bit"]
    #[inline(always)]
    pub const fn set_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Base DW address of the start of the resource range"]
    #[must_use]
    #[inline(always)]
    pub const fn base(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Base DW address of the start of the resource range"]
    #[inline(always)]
    pub const fn set_base(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for PciCfcEaPe1Base {
    #[inline(always)]
    fn default() -> PciCfcEaPe1Base {
        PciCfcEaPe1Base(1623195650u64 as u32)
    }
}
impl core::fmt::Debug for PciCfcEaPe1Base {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfcEaPe1Base")
            .field("s", &self.s())
            .field("base", &self.base())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfcEaPe1Base {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfcEaPe1Base {
            s: bool,
            base: u32,
        }
        let proxy = PciCfcEaPe1Base {
            s: self.s(),
            base: self.base(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI EA per-entry 1 format register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfcEaPe1Fmt(pub u32);
impl PciCfcEaPe1Fmt {
    #[doc = "Entry size Number of DWs following the initial DW in this entry"]
    #[must_use]
    #[inline(always)]
    pub const fn entry_size(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Entry size Number of DWs following the initial DW in this entry"]
    #[inline(always)]
    pub const fn set_entry_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "BAR equivalent indicator This field indicates the equivalent PCI BAR for this entry"]
    #[must_use]
    #[inline(always)]
    pub const fn bei(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "BAR equivalent indicator This field indicates the equivalent PCI BAR for this entry"]
    #[inline(always)]
    pub const fn set_bei(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Primary properties Value (h) Resource Definition 00 Memory Space, Non-Prefetchable"]
    #[must_use]
    #[inline(always)]
    pub const fn prim_prop(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Primary properties Value (h) Resource Definition 00 Memory Space, Non-Prefetchable"]
    #[inline(always)]
    pub const fn set_prim_prop(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Secondary properties Alternative property when system cannot use the primary property."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_prop(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Secondary properties Alternative property when system cannot use the primary property."]
    #[inline(always)]
    pub const fn set_sec_prop(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Writable 1b indicates that the Base, MaxOffset and Field Size fields for this entry are RW, 0b indicates those fields are HWInit"]
    #[must_use]
    #[inline(always)]
    pub const fn writable(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Writable 1b indicates that the Base, MaxOffset and Field Size fields for this entry are RW, 0b indicates those fields are HWInit"]
    #[inline(always)]
    pub const fn set_writable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Enable 0 Disabled 1 Enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable 0 Disabled 1 Enabled"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PciCfcEaPe1Fmt {
    #[inline(always)]
    fn default() -> PciCfcEaPe1Fmt {
        PciCfcEaPe1Fmt(2147483939u64 as u32)
    }
}
impl core::fmt::Debug for PciCfcEaPe1Fmt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfcEaPe1Fmt")
            .field("entry_size", &self.entry_size())
            .field("bei", &self.bei())
            .field("prim_prop", &self.prim_prop())
            .field("sec_prop", &self.sec_prop())
            .field("writable", &self.writable())
            .field("enable", &self.enable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfcEaPe1Fmt {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfcEaPe1Fmt {
            entry_size: u8,
            bei: u8,
            prim_prop: u8,
            sec_prop: u8,
            writable: bool,
            enable: bool,
        }
        let proxy = PciCfcEaPe1Fmt {
            entry_size: self.entry_size(),
            bei: self.bei(),
            prim_prop: self.prim_prop(),
            sec_prop: self.sec_prop(),
            writable: self.writable(),
            enable: self.enable(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI EA per-entry 1 max offset register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfcEaPe1Maxoff(pub u32);
impl PciCfcEaPe1Maxoff {
    #[doc = "Field size 0 32-bit 1 64-bit"]
    #[must_use]
    #[inline(always)]
    pub const fn s(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Field size 0 32-bit 1 64-bit"]
    #[inline(always)]
    pub const fn set_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Max offset The value in the Base field (\\[63:2\\]) plus the value in the MaxOffset field indicates the address of the last included DW of the resource range"]
    #[must_use]
    #[inline(always)]
    pub const fn max_offset(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Max offset The value in the Base field (\\[63:2\\]) plus the value in the MaxOffset field indicates the address of the last included DW of the resource range"]
    #[inline(always)]
    pub const fn set_max_offset(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for PciCfcEaPe1Maxoff {
    #[inline(always)]
    fn default() -> PciCfcEaPe1Maxoff {
        PciCfcEaPe1Maxoff(65532u64 as u32)
    }
}
impl core::fmt::Debug for PciCfcEaPe1Maxoff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfcEaPe1Maxoff")
            .field("s", &self.s())
            .field("max_offset", &self.max_offset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfcEaPe1Maxoff {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfcEaPe1Maxoff {
            s: bool,
            max_offset: u32,
        }
        let proxy = PciCfcEaPe1Maxoff {
            s: self.s(),
            max_offset: self.max_offset(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI EA per-entry 2 base register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfcEaPe2Base(pub u32);
impl PciCfcEaPe2Base {
    #[doc = "Field size 0 32-bit 1 64-bit"]
    #[must_use]
    #[inline(always)]
    pub const fn s(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Field size 0 32-bit 1 64-bit"]
    #[inline(always)]
    pub const fn set_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Base DW address of the start of the resource range"]
    #[must_use]
    #[inline(always)]
    pub const fn base(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Base DW address of the start of the resource range"]
    #[inline(always)]
    pub const fn set_base(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for PciCfcEaPe2Base {
    #[inline(always)]
    fn default() -> PciCfcEaPe2Base {
        PciCfcEaPe2Base(1623261186u64 as u32)
    }
}
impl core::fmt::Debug for PciCfcEaPe2Base {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfcEaPe2Base")
            .field("s", &self.s())
            .field("base", &self.base())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfcEaPe2Base {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfcEaPe2Base {
            s: bool,
            base: u32,
        }
        let proxy = PciCfcEaPe2Base {
            s: self.s(),
            base: self.base(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI EA per-entry 2 format register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfcEaPe2Fmt(pub u32);
impl PciCfcEaPe2Fmt {
    #[doc = "Entry size Number of DWs following the initial DW in this entry"]
    #[must_use]
    #[inline(always)]
    pub const fn entry_size(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Entry size Number of DWs following the initial DW in this entry"]
    #[inline(always)]
    pub const fn set_entry_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "BAR equivalent indicator This field indicates the equivalent PCI BAR for this entry"]
    #[must_use]
    #[inline(always)]
    pub const fn bei(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "BAR equivalent indicator This field indicates the equivalent PCI BAR for this entry"]
    #[inline(always)]
    pub const fn set_bei(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Primary properties Value (h) Resource Definition 00 Memory Space, Non-Prefetchable"]
    #[must_use]
    #[inline(always)]
    pub const fn prim_prop(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Primary properties Value (h) Resource Definition 00 Memory Space, Non-Prefetchable"]
    #[inline(always)]
    pub const fn set_prim_prop(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Secondary properties Alternative property when system cannot use the primary property."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_prop(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Secondary properties Alternative property when system cannot use the primary property."]
    #[inline(always)]
    pub const fn set_sec_prop(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Writable 1b indicates that the Base, MaxOffset and Field Size fields for this entry are RW, 0b indicates those fields are HWInit"]
    #[must_use]
    #[inline(always)]
    pub const fn writable(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Writable 1b indicates that the Base, MaxOffset and Field Size fields for this entry are RW, 0b indicates those fields are HWInit"]
    #[inline(always)]
    pub const fn set_writable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Enable 0 Disabled 1 Enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable 0 Disabled 1 Enabled"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PciCfcEaPe2Fmt {
    #[inline(always)]
    fn default() -> PciCfcEaPe2Fmt {
        PciCfcEaPe2Fmt(2164196499u64 as u32)
    }
}
impl core::fmt::Debug for PciCfcEaPe2Fmt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfcEaPe2Fmt")
            .field("entry_size", &self.entry_size())
            .field("bei", &self.bei())
            .field("prim_prop", &self.prim_prop())
            .field("sec_prop", &self.sec_prop())
            .field("writable", &self.writable())
            .field("enable", &self.enable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfcEaPe2Fmt {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfcEaPe2Fmt {
            entry_size: u8,
            bei: u8,
            prim_prop: u8,
            sec_prop: u8,
            writable: bool,
            enable: bool,
        }
        let proxy = PciCfcEaPe2Fmt {
            entry_size: self.entry_size(),
            bei: self.bei(),
            prim_prop: self.prim_prop(),
            sec_prop: self.sec_prop(),
            writable: self.writable(),
            enable: self.enable(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI EA per-entry 2 max offset register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfcEaPe2Maxoff(pub u32);
impl PciCfcEaPe2Maxoff {
    #[doc = "Field size 0 32-bit 1 64-bit"]
    #[must_use]
    #[inline(always)]
    pub const fn s(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Field size 0 32-bit 1 64-bit"]
    #[inline(always)]
    pub const fn set_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Max offset The value in the Base field (\\[63:2\\]) plus the value in the MaxOffset field indicates the address of the last included DW of the resource range"]
    #[must_use]
    #[inline(always)]
    pub const fn max_offset(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Max offset The value in the Base field (\\[63:2\\]) plus the value in the MaxOffset field indicates the address of the last included DW of the resource range"]
    #[inline(always)]
    pub const fn set_max_offset(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for PciCfcEaPe2Maxoff {
    #[inline(always)]
    fn default() -> PciCfcEaPe2Maxoff {
        PciCfcEaPe2Maxoff(65532u64 as u32)
    }
}
impl core::fmt::Debug for PciCfcEaPe2Maxoff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfcEaPe2Maxoff")
            .field("s", &self.s())
            .field("max_offset", &self.max_offset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfcEaPe2Maxoff {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfcEaPe2Maxoff {
            s: bool,
            max_offset: u32,
        }
        let proxy = PciCfcEaPe2Maxoff {
            s: self.s(),
            max_offset: self.max_offset(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI EA per-entry 3 base register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfcEaPe3Base(pub u32);
impl PciCfcEaPe3Base {
    #[doc = "Field size 0 32-bit 1 64-bit"]
    #[must_use]
    #[inline(always)]
    pub const fn s(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Field size 0 32-bit 1 64-bit"]
    #[inline(always)]
    pub const fn set_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Base DW address of the start of the resource range"]
    #[must_use]
    #[inline(always)]
    pub const fn base(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Base DW address of the start of the resource range"]
    #[inline(always)]
    pub const fn set_base(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for PciCfcEaPe3Base {
    #[inline(always)]
    fn default() -> PciCfcEaPe3Base {
        PciCfcEaPe3Base(1623326722u64 as u32)
    }
}
impl core::fmt::Debug for PciCfcEaPe3Base {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfcEaPe3Base")
            .field("s", &self.s())
            .field("base", &self.base())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfcEaPe3Base {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfcEaPe3Base {
            s: bool,
            base: u32,
        }
        let proxy = PciCfcEaPe3Base {
            s: self.s(),
            base: self.base(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI EA per-entry 3 format register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfcEaPe3Fmt(pub u32);
impl PciCfcEaPe3Fmt {
    #[doc = "Entry size Number of DWs following the initial DW in this entry"]
    #[must_use]
    #[inline(always)]
    pub const fn entry_size(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Entry size Number of DWs following the initial DW in this entry"]
    #[inline(always)]
    pub const fn set_entry_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "BAR equivalent indicator This field indicates the equivalent PCI BAR for this entry"]
    #[must_use]
    #[inline(always)]
    pub const fn bei(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "BAR equivalent indicator This field indicates the equivalent PCI BAR for this entry"]
    #[inline(always)]
    pub const fn set_bei(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Primary properties Value (h) Resource Definition 00 Memory Space, Non-Prefetchable"]
    #[must_use]
    #[inline(always)]
    pub const fn prim_prop(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Primary properties Value (h) Resource Definition 00 Memory Space, Non-Prefetchable"]
    #[inline(always)]
    pub const fn set_prim_prop(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Secondary properties Alternative property when system cannot use the primary property."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_prop(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Secondary properties Alternative property when system cannot use the primary property."]
    #[inline(always)]
    pub const fn set_sec_prop(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Writable 1b indicates that the Base, MaxOffset and Field Size fields for this entry are RW, 0b indicates those fields are HWInit"]
    #[must_use]
    #[inline(always)]
    pub const fn writable(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Writable 1b indicates that the Base, MaxOffset and Field Size fields for this entry are RW, 0b indicates those fields are HWInit"]
    #[inline(always)]
    pub const fn set_writable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Enable 0 Disabled 1 Enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable 0 Disabled 1 Enabled"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PciCfcEaPe3Fmt {
    #[inline(always)]
    fn default() -> PciCfcEaPe3Fmt {
        PciCfcEaPe3Fmt(2147681203u64 as u32)
    }
}
impl core::fmt::Debug for PciCfcEaPe3Fmt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfcEaPe3Fmt")
            .field("entry_size", &self.entry_size())
            .field("bei", &self.bei())
            .field("prim_prop", &self.prim_prop())
            .field("sec_prop", &self.sec_prop())
            .field("writable", &self.writable())
            .field("enable", &self.enable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfcEaPe3Fmt {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfcEaPe3Fmt {
            entry_size: u8,
            bei: u8,
            prim_prop: u8,
            sec_prop: u8,
            writable: bool,
            enable: bool,
        }
        let proxy = PciCfcEaPe3Fmt {
            entry_size: self.entry_size(),
            bei: self.bei(),
            prim_prop: self.prim_prop(),
            sec_prop: self.sec_prop(),
            writable: self.writable(),
            enable: self.enable(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI EA per-entry 3 max offset register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfcEaPe3Maxoff(pub u32);
impl PciCfcEaPe3Maxoff {
    #[doc = "Field size 0 32-bit 1 64-bit"]
    #[must_use]
    #[inline(always)]
    pub const fn s(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Field size 0 32-bit 1 64-bit"]
    #[inline(always)]
    pub const fn set_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Max offset The value in the Base field (\\[63:2\\]) plus the value in the MaxOffset field indicates the address of the last included DW of the resource range"]
    #[must_use]
    #[inline(always)]
    pub const fn max_offset(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Max offset The value in the Base field (\\[63:2\\]) plus the value in the MaxOffset field indicates the address of the last included DW of the resource range"]
    #[inline(always)]
    pub const fn set_max_offset(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for PciCfcEaPe3Maxoff {
    #[inline(always)]
    fn default() -> PciCfcEaPe3Maxoff {
        PciCfcEaPe3Maxoff(65532u64 as u32)
    }
}
impl core::fmt::Debug for PciCfcEaPe3Maxoff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfcEaPe3Maxoff")
            .field("s", &self.s())
            .field("max_offset", &self.max_offset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfcEaPe3Maxoff {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfcEaPe3Maxoff {
            s: bool,
            max_offset: u32,
        }
        let proxy = PciCfcEaPe3Maxoff {
            s: self.s(),
            max_offset: self.max_offset(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI MSI-X capabilities list register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfcMsixCapList(pub u16);
impl PciCfcMsixCapList {
    #[doc = "Indicates the MSI-X Capability structure. Hardwired to 11h."]
    #[must_use]
    #[inline(always)]
    pub const fn cap_id(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates the MSI-X Capability structure. Hardwired to 11h."]
    #[inline(always)]
    pub const fn set_cap_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "This field contains the offset to the next PCI Capability structure or 00h if no other items exist in the linked list of capabilities"]
    #[must_use]
    #[inline(always)]
    pub const fn next_cap_ptr(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "This field contains the offset to the next PCI Capability structure or 00h if no other items exist in the linked list of capabilities"]
    #[inline(always)]
    pub const fn set_next_cap_ptr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for PciCfcMsixCapList {
    #[inline(always)]
    fn default() -> PciCfcMsixCapList {
        PciCfcMsixCapList(36881u64 as u16)
    }
}
impl core::fmt::Debug for PciCfcMsixCapList {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfcMsixCapList")
            .field("cap_id", &self.cap_id())
            .field("next_cap_ptr", &self.next_cap_ptr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfcMsixCapList {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfcMsixCapList {
            cap_id: u8,
            next_cap_ptr: u8,
        }
        let proxy = PciCfcMsixCapList {
            cap_id: self.cap_id(),
            next_cap_ptr: self.next_cap_ptr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI MSI-X message control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfcMsixMsgCtl(pub u16);
impl PciCfcMsixMsgCtl {
    #[doc = "Table size System software reads this field to determine the MSI-X Table Size N, which is encoded as N-1"]
    #[must_use]
    #[inline(always)]
    pub const fn table_size(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Table size System software reads this field to determine the MSI-X Table Size N, which is encoded as N-1"]
    #[inline(always)]
    pub const fn set_table_size(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
    }
    #[doc = "Function mask If 1, all of the vectors associated with the function are masked, regardless of their per-vector Mask bit states"]
    #[must_use]
    #[inline(always)]
    pub const fn func_mask(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Function mask If 1, all of the vectors associated with the function are masked, regardless of their per-vector Mask bit states"]
    #[inline(always)]
    pub const fn set_func_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "MSI-X enable If set, the function is permitted to use MSI-X to request service"]
    #[must_use]
    #[inline(always)]
    pub const fn msix_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "MSI-X enable If set, the function is permitted to use MSI-X to request service"]
    #[inline(always)]
    pub const fn set_msix_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for PciCfcMsixMsgCtl {
    #[inline(always)]
    fn default() -> PciCfcMsixMsgCtl {
        PciCfcMsixMsgCtl(23u64 as u16)
    }
}
impl core::fmt::Debug for PciCfcMsixMsgCtl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfcMsixMsgCtl")
            .field("table_size", &self.table_size())
            .field("func_mask", &self.func_mask())
            .field("msix_en", &self.msix_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfcMsixMsgCtl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfcMsixMsgCtl {
            table_size: u16,
            func_mask: bool,
            msix_en: bool,
        }
        let proxy = PciCfcMsixMsgCtl {
            table_size: self.table_size(),
            func_mask: self.func_mask(),
            msix_en: self.msix_en(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI MSI-X PBA offset/BIR register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfcMsixPbaOffBir(pub u32);
impl PciCfcMsixPbaOffBir {
    #[doc = "PBA BIR Indicates which entry in the Enhanced Allocation capability with a matching BEI, is used to map the Function's MSI-X PBA into Memory Space"]
    #[must_use]
    #[inline(always)]
    pub const fn pba_bir(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "PBA BIR Indicates which entry in the Enhanced Allocation capability with a matching BEI, is used to map the Function's MSI-X PBA into Memory Space"]
    #[inline(always)]
    pub const fn set_pba_bir(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "PBA Offset Used as an offset from the address contained by one of the function's Base Address registers to point to the base of the MSI-X PBA"]
    #[must_use]
    #[inline(always)]
    pub const fn pba_offset(&self) -> u32 {
        let val = (self.0 >> 3usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "PBA Offset Used as an offset from the address contained by one of the function's Base Address registers to point to the base of the MSI-X PBA"]
    #[inline(always)]
    pub const fn set_pba_offset(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
    }
}
impl Default for PciCfcMsixPbaOffBir {
    #[inline(always)]
    fn default() -> PciCfcMsixPbaOffBir {
        PciCfcMsixPbaOffBir(2049u64 as u32)
    }
}
impl core::fmt::Debug for PciCfcMsixPbaOffBir {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfcMsixPbaOffBir")
            .field("pba_bir", &self.pba_bir())
            .field("pba_offset", &self.pba_offset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfcMsixPbaOffBir {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfcMsixPbaOffBir {
            pba_bir: u8,
            pba_offset: u32,
        }
        let proxy = PciCfcMsixPbaOffBir {
            pba_bir: self.pba_bir(),
            pba_offset: self.pba_offset(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI MSI-X table offset/BIR register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfcMsixTableOffBir(pub u32);
impl PciCfcMsixTableOffBir {
    #[doc = "Table BIR Indicates which entry in the Enhanced Allocation capability with a matching BEI, is used to map the Function's MSI-X Table into Memory Space"]
    #[must_use]
    #[inline(always)]
    pub const fn table_bir(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Table BIR Indicates which entry in the Enhanced Allocation capability with a matching BEI, is used to map the Function's MSI-X Table into Memory Space"]
    #[inline(always)]
    pub const fn set_table_bir(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Table Offset Used as an offset from the address contained by one of the function's Base Address registers to point to the base of the MSI-X Table"]
    #[must_use]
    #[inline(always)]
    pub const fn table_offset(&self) -> u32 {
        let val = (self.0 >> 3usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "Table Offset Used as an offset from the address contained by one of the function's Base Address registers to point to the base of the MSI-X Table"]
    #[inline(always)]
    pub const fn set_table_offset(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
    }
}
impl Default for PciCfcMsixTableOffBir {
    #[inline(always)]
    fn default() -> PciCfcMsixTableOffBir {
        PciCfcMsixTableOffBir(1u64 as u32)
    }
}
impl core::fmt::Debug for PciCfcMsixTableOffBir {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfcMsixTableOffBir")
            .field("table_bir", &self.table_bir())
            .field("table_offset", &self.table_offset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfcMsixTableOffBir {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfcMsixTableOffBir {
            table_bir: u8,
            table_offset: u32,
        }
        let proxy = PciCfcMsixTableOffBir {
            table_bir: self.table_bir(),
            table_offset: self.table_offset(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI PCIe capabilities register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfcPcieCap(pub u16);
impl PciCfcPcieCap {
    #[doc = "Capability Version Indicates PCI-SIG defined PCI Express Capability structure version number"]
    #[must_use]
    #[inline(always)]
    pub const fn cap_ver(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capability Version Indicates PCI-SIG defined PCI Express Capability structure version number"]
    #[inline(always)]
    pub const fn set_cap_ver(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "Device/Port type Indicates the specific type of this PCI Express Function"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_port_type(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Device/Port type Indicates the specific type of this PCI Express Function"]
    #[inline(always)]
    pub const fn set_dev_port_type(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
    }
    #[doc = "Interrupt message number This field indicates which MSI/MSI-X vector is used for the interrupt message generated in association with any of the status bits of this capability structure"]
    #[must_use]
    #[inline(always)]
    pub const fn int_msg_num(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x1f;
        val as u8
    }
    #[doc = "Interrupt message number This field indicates which MSI/MSI-X vector is used for the interrupt message generated in association with any of the status bits of this capability structure"]
    #[inline(always)]
    pub const fn set_int_msg_num(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 9usize)) | (((val as u16) & 0x1f) << 9usize);
    }
}
impl Default for PciCfcPcieCap {
    #[inline(always)]
    fn default() -> PciCfcPcieCap {
        PciCfcPcieCap(146u64 as u16)
    }
}
impl core::fmt::Debug for PciCfcPcieCap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfcPcieCap")
            .field("cap_ver", &self.cap_ver())
            .field("dev_port_type", &self.dev_port_type())
            .field("int_msg_num", &self.int_msg_num())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfcPcieCap {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfcPcieCap {
            cap_ver: u8,
            dev_port_type: u8,
            int_msg_num: u8,
        }
        let proxy = PciCfcPcieCap {
            cap_ver: self.cap_ver(),
            dev_port_type: self.dev_port_type(),
            int_msg_num: self.int_msg_num(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI PCIe capabilities list register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfcPcieCapList(pub u16);
impl PciCfcPcieCapList {
    #[doc = "Indicates the PCI Express Capability structure. Hardwired to 10h."]
    #[must_use]
    #[inline(always)]
    pub const fn cap_id(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates the PCI Express Capability structure. Hardwired to 10h."]
    #[inline(always)]
    pub const fn set_cap_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "This field contains the offset to the next PCI Capability structure or 00h if no other items exist in the linked list of capabilities"]
    #[must_use]
    #[inline(always)]
    pub const fn next_cap_ptr(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "This field contains the offset to the next PCI Capability structure or 00h if no other items exist in the linked list of capabilities"]
    #[inline(always)]
    pub const fn set_next_cap_ptr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for PciCfcPcieCapList {
    #[inline(always)]
    fn default() -> PciCfcPcieCapList {
        PciCfcPcieCapList(32784u64 as u16)
    }
}
impl core::fmt::Debug for PciCfcPcieCapList {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfcPcieCapList")
            .field("cap_id", &self.cap_id())
            .field("next_cap_ptr", &self.next_cap_ptr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfcPcieCapList {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfcPcieCapList {
            cap_id: u8,
            next_cap_ptr: u8,
        }
        let proxy = PciCfcPcieCapList {
            cap_id: self.cap_id(),
            next_cap_ptr: self.next_cap_ptr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI PCIe device capabilities register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfcPcieDevCap(pub u32);
impl PciCfcPcieDevCap {
    #[doc = "Function level reset capability A value of 1b indicates the function supports the optional Function Level Reset (FLR) mechanism"]
    #[must_use]
    #[inline(always)]
    pub const fn flr_cap(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Function level reset capability A value of 1b indicates the function supports the optional Function Level Reset (FLR) mechanism"]
    #[inline(always)]
    pub const fn set_flr_cap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for PciCfcPcieDevCap {
    #[inline(always)]
    fn default() -> PciCfcPcieDevCap {
        PciCfcPcieDevCap(268435456u64 as u32)
    }
}
impl core::fmt::Debug for PciCfcPcieDevCap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfcPcieDevCap")
            .field("flr_cap", &self.flr_cap())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfcPcieDevCap {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfcPcieDevCap {
            flr_cap: bool,
        }
        let proxy = PciCfcPcieDevCap {
            flr_cap: self.flr_cap(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI PCIe device capabilities 2 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfcPcieDevCap2(pub u32);
impl PciCfcPcieDevCap2 {
    #[doc = "Completion Timeout Ranges Supported Completion Timeout programming not supported, the Function assumes a timeout value in the range 50 us to 50 ms"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpl_to_rng_supp(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Completion Timeout Ranges Supported Completion Timeout programming not supported, the Function assumes a timeout value in the range 50 us to 50 ms"]
    #[inline(always)]
    pub const fn set_cmpl_to_rng_supp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Completion Timeout Disable Supported Not supported. Hardwired to b0."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpl_to_dis_supp(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Completion Timeout Disable Supported Not supported. Hardwired to b0."]
    #[inline(always)]
    pub const fn set_cmpl_to_dis_supp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for PciCfcPcieDevCap2 {
    #[inline(always)]
    fn default() -> PciCfcPcieDevCap2 {
        PciCfcPcieDevCap2(0u64 as u32)
    }
}
impl core::fmt::Debug for PciCfcPcieDevCap2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfcPcieDevCap2")
            .field("cmpl_to_rng_supp", &self.cmpl_to_rng_supp())
            .field("cmpl_to_dis_supp", &self.cmpl_to_dis_supp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfcPcieDevCap2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfcPcieDevCap2 {
            cmpl_to_rng_supp: u8,
            cmpl_to_dis_supp: bool,
        }
        let proxy = PciCfcPcieDevCap2 {
            cmpl_to_rng_supp: self.cmpl_to_rng_supp(),
            cmpl_to_dis_supp: self.cmpl_to_dis_supp(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI PCIe device control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfcPcieDevCtl(pub u16);
impl PciCfcPcieDevCtl {
    #[doc = "Initiate function level reset A write of b1 initiates Function Level Reset (FLR)"]
    #[must_use]
    #[inline(always)]
    pub const fn init_flr(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Initiate function level reset A write of b1 initiates Function Level Reset (FLR)"]
    #[inline(always)]
    pub const fn set_init_flr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for PciCfcPcieDevCtl {
    #[inline(always)]
    fn default() -> PciCfcPcieDevCtl {
        PciCfcPcieDevCtl(0u64 as u16)
    }
}
impl core::fmt::Debug for PciCfcPcieDevCtl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfcPcieDevCtl")
            .field("init_flr", &self.init_flr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfcPcieDevCtl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfcPcieDevCtl {
            init_flr: bool,
        }
        let proxy = PciCfcPcieDevCtl {
            init_flr: self.init_flr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI PCIe device control 2 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfcPcieDevCtl2(pub u16);
impl PciCfcPcieDevCtl2 {
    #[doc = "Completion Timeout Value Completion Timeout programming not supported - the Function assumes a timeout value in the range 50 us to 50 ms"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpl_to_value(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Completion Timeout Value Completion Timeout programming not supported - the Function assumes a timeout value in the range 50 us to 50 ms"]
    #[inline(always)]
    pub const fn set_cmpl_to_value(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "Completion Timeout Enable Not supported. Hardwired to b0."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpl_to_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Completion Timeout Enable Not supported. Hardwired to b0."]
    #[inline(always)]
    pub const fn set_cmpl_to_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
}
impl Default for PciCfcPcieDevCtl2 {
    #[inline(always)]
    fn default() -> PciCfcPcieDevCtl2 {
        PciCfcPcieDevCtl2(0u64 as u16)
    }
}
impl core::fmt::Debug for PciCfcPcieDevCtl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfcPcieDevCtl2")
            .field("cmpl_to_value", &self.cmpl_to_value())
            .field("cmpl_to_en", &self.cmpl_to_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfcPcieDevCtl2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfcPcieDevCtl2 {
            cmpl_to_value: u8,
            cmpl_to_en: bool,
        }
        let proxy = PciCfcPcieDevCtl2 {
            cmpl_to_value: self.cmpl_to_value(),
            cmpl_to_en: self.cmpl_to_en(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI PCIe device status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfcPcieDevStat(pub u16);
impl PciCfcPcieDevStat {
    #[doc = "Transaction pending If set indicates that the Function has outstanding transactions on its external master interface"]
    #[must_use]
    #[inline(always)]
    pub const fn trans_pend(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Transaction pending If set indicates that the Function has outstanding transactions on its external master interface"]
    #[inline(always)]
    pub const fn set_trans_pend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
}
impl Default for PciCfcPcieDevStat {
    #[inline(always)]
    fn default() -> PciCfcPcieDevStat {
        PciCfcPcieDevStat(0u64 as u16)
    }
}
impl core::fmt::Debug for PciCfcPcieDevStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfcPcieDevStat")
            .field("trans_pend", &self.trans_pend())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfcPcieDevStat {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfcPcieDevStat {
            trans_pend: bool,
        }
        let proxy = PciCfcPcieDevStat {
            trans_pend: self.trans_pend(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI PCI-PM capabilities register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfcPcipmCap(pub u16);
impl PciCfcPcipmCap {
    #[doc = "Version ENETC complies with the PCI PM specification, rev 1.2."]
    #[must_use]
    #[inline(always)]
    pub const fn version(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Version ENETC complies with the PCI PM specification, rev 1.2."]
    #[inline(always)]
    pub const fn set_version(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u16) & 0x07) << 0usize);
    }
    #[doc = "PME support Indicates the PM states within which the function is capable of sending PME message"]
    #[must_use]
    #[inline(always)]
    pub const fn pme_support(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "PME support Indicates the PM states within which the function is capable of sending PME message"]
    #[inline(always)]
    pub const fn set_pme_support(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for PciCfcPcipmCap {
    #[inline(always)]
    fn default() -> PciCfcPcipmCap {
        PciCfcPcipmCap(18435u64 as u16)
    }
}
impl core::fmt::Debug for PciCfcPcipmCap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfcPcipmCap")
            .field("version", &self.version())
            .field("pme_support", &self.pme_support())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfcPcipmCap {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfcPcipmCap {
            version: u8,
            pme_support: u8,
        }
        let proxy = PciCfcPcipmCap {
            version: self.version(),
            pme_support: self.pme_support(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI PCI-PM capabilities list register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfcPcipmCapList(pub u16);
impl PciCfcPcipmCapList {
    #[doc = "Indicates the PCI-PM Capability structure. Hardwired to 01h."]
    #[must_use]
    #[inline(always)]
    pub const fn cap_id(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates the PCI-PM Capability structure. Hardwired to 01h."]
    #[inline(always)]
    pub const fn set_cap_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "This field contains the offset to the next PCI Capability structure or 00h if no other items exist in the linked list of capabilities"]
    #[must_use]
    #[inline(always)]
    pub const fn next_cap_ptr(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "This field contains the offset to the next PCI Capability structure or 00h if no other items exist in the linked list of capabilities"]
    #[inline(always)]
    pub const fn set_next_cap_ptr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for PciCfcPcipmCapList {
    #[inline(always)]
    fn default() -> PciCfcPcipmCapList {
        PciCfcPcipmCapList(39937u64 as u16)
    }
}
impl core::fmt::Debug for PciCfcPcipmCapList {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfcPcipmCapList")
            .field("cap_id", &self.cap_id())
            .field("next_cap_ptr", &self.next_cap_ptr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfcPcipmCapList {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfcPcipmCapList {
            cap_id: u8,
            next_cap_ptr: u8,
        }
        let proxy = PciCfcPcipmCapList {
            cap_id: self.cap_id(),
            next_cap_ptr: self.next_cap_ptr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI PCI-PM control and status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfcPcipmCtlStat(pub u16);
impl PciCfcPcipmCtlStat {
    #[doc = "Power state This field is used to set and report the power state of a function as follows: 00b = D0 01b = D1 (not supported by NETC) 10b = D2 (not supported by NETC) 11b = D3 If, for any reason, the operating system software attempts to put a function into a power management state that the function does not support, the function should handle this gracefully and remain in whatever state it was in before the request"]
    #[must_use]
    #[inline(always)]
    pub const fn pwr_state(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Power state This field is used to set and report the power state of a function as follows: 00b = D0 01b = D1 (not supported by NETC) 10b = D2 (not supported by NETC) 11b = D3 If, for any reason, the operating system software attempts to put a function into a power management state that the function does not support, the function should handle this gracefully and remain in whatever state it was in before the request"]
    #[inline(always)]
    pub const fn set_pwr_state(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u16) & 0x03) << 0usize);
    }
    #[doc = "No soft reset When set (\"1\"), this bit indicates that when NETC transitions from D3hot to D0active because of modifying Power State bits in the PCI_CFC_PCIPM_CTL_STAT register, no internal reset is issued and Configuration Context is preserved"]
    #[must_use]
    #[inline(always)]
    pub const fn no_soft_rst(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "No soft reset When set (\"1\"), this bit indicates that when NETC transitions from D3hot to D0active because of modifying Power State bits in the PCI_CFC_PCIPM_CTL_STAT register, no internal reset is issued and Configuration Context is preserved"]
    #[inline(always)]
    pub const fn set_no_soft_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
}
impl Default for PciCfcPcipmCtlStat {
    #[inline(always)]
    fn default() -> PciCfcPcipmCtlStat {
        PciCfcPcipmCtlStat(8u64 as u16)
    }
}
impl core::fmt::Debug for PciCfcPcipmCtlStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfcPcipmCtlStat")
            .field("pwr_state", &self.pwr_state())
            .field("no_soft_rst", &self.no_soft_rst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfcPcipmCtlStat {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfcPcipmCtlStat {
            pwr_state: u8,
            no_soft_rst: bool,
        }
        let proxy = PciCfcPcipmCtlStat {
            pwr_state: self.pwr_state(),
            no_soft_rst: self.no_soft_rst(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI base address register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfhBar0(pub u32);
impl PciCfhBar0 {
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    #[must_use]
    #[inline(always)]
    pub const fn mem_io_ind(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    #[inline(always)]
    pub const fn set_mem_io_ind(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    #[must_use]
    #[inline(always)]
    pub const fn mem_type(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    #[inline(always)]
    pub const fn set_mem_type(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    #[must_use]
    #[inline(always)]
    pub const fn pf_mem(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    #[inline(always)]
    pub const fn set_pf_mem(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "EA BARs used. This register is hardwired to 000000h."]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "EA BARs used. This register is hardwired to 000000h."]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
    }
}
impl Default for PciCfhBar0 {
    #[inline(always)]
    fn default() -> PciCfhBar0 {
        PciCfhBar0(0u64 as u32)
    }
}
impl core::fmt::Debug for PciCfhBar0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfhBar0")
            .field("mem_io_ind", &self.mem_io_ind())
            .field("mem_type", &self.mem_type())
            .field("pf_mem", &self.pf_mem())
            .field("addr", &self.addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfhBar0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfhBar0 {
            mem_io_ind: bool,
            mem_type: u8,
            pf_mem: bool,
            addr: u32,
        }
        let proxy = PciCfhBar0 {
            mem_io_ind: self.mem_io_ind(),
            mem_type: self.mem_type(),
            pf_mem: self.pf_mem(),
            addr: self.addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI base address register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfhBar1(pub u32);
impl PciCfhBar1 {
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    #[must_use]
    #[inline(always)]
    pub const fn mem_io_ind(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    #[inline(always)]
    pub const fn set_mem_io_ind(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    #[must_use]
    #[inline(always)]
    pub const fn mem_type(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    #[inline(always)]
    pub const fn set_mem_type(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    #[must_use]
    #[inline(always)]
    pub const fn pf_mem(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    #[inline(always)]
    pub const fn set_pf_mem(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "EA BARs used. This register is hardwired to 000000h."]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "EA BARs used. This register is hardwired to 000000h."]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
    }
}
impl Default for PciCfhBar1 {
    #[inline(always)]
    fn default() -> PciCfhBar1 {
        PciCfhBar1(0u64 as u32)
    }
}
impl core::fmt::Debug for PciCfhBar1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfhBar1")
            .field("mem_io_ind", &self.mem_io_ind())
            .field("mem_type", &self.mem_type())
            .field("pf_mem", &self.pf_mem())
            .field("addr", &self.addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfhBar1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfhBar1 {
            mem_io_ind: bool,
            mem_type: u8,
            pf_mem: bool,
            addr: u32,
        }
        let proxy = PciCfhBar1 {
            mem_io_ind: self.mem_io_ind(),
            mem_type: self.mem_type(),
            pf_mem: self.pf_mem(),
            addr: self.addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI base address register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfhBar2(pub u32);
impl PciCfhBar2 {
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    #[must_use]
    #[inline(always)]
    pub const fn mem_io_ind(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    #[inline(always)]
    pub const fn set_mem_io_ind(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    #[must_use]
    #[inline(always)]
    pub const fn mem_type(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    #[inline(always)]
    pub const fn set_mem_type(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    #[must_use]
    #[inline(always)]
    pub const fn pf_mem(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    #[inline(always)]
    pub const fn set_pf_mem(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "EA BARs used. This register is hardwired to 000000h."]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "EA BARs used. This register is hardwired to 000000h."]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
    }
}
impl Default for PciCfhBar2 {
    #[inline(always)]
    fn default() -> PciCfhBar2 {
        PciCfhBar2(0u64 as u32)
    }
}
impl core::fmt::Debug for PciCfhBar2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfhBar2")
            .field("mem_io_ind", &self.mem_io_ind())
            .field("mem_type", &self.mem_type())
            .field("pf_mem", &self.pf_mem())
            .field("addr", &self.addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfhBar2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfhBar2 {
            mem_io_ind: bool,
            mem_type: u8,
            pf_mem: bool,
            addr: u32,
        }
        let proxy = PciCfhBar2 {
            mem_io_ind: self.mem_io_ind(),
            mem_type: self.mem_type(),
            pf_mem: self.pf_mem(),
            addr: self.addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI base address register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfhBar3(pub u32);
impl PciCfhBar3 {
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    #[must_use]
    #[inline(always)]
    pub const fn mem_io_ind(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    #[inline(always)]
    pub const fn set_mem_io_ind(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    #[must_use]
    #[inline(always)]
    pub const fn mem_type(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    #[inline(always)]
    pub const fn set_mem_type(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    #[must_use]
    #[inline(always)]
    pub const fn pf_mem(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    #[inline(always)]
    pub const fn set_pf_mem(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "EA BARs used. This register is hardwired to 000000h."]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "EA BARs used. This register is hardwired to 000000h."]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
    }
}
impl Default for PciCfhBar3 {
    #[inline(always)]
    fn default() -> PciCfhBar3 {
        PciCfhBar3(0u64 as u32)
    }
}
impl core::fmt::Debug for PciCfhBar3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfhBar3")
            .field("mem_io_ind", &self.mem_io_ind())
            .field("mem_type", &self.mem_type())
            .field("pf_mem", &self.pf_mem())
            .field("addr", &self.addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfhBar3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfhBar3 {
            mem_io_ind: bool,
            mem_type: u8,
            pf_mem: bool,
            addr: u32,
        }
        let proxy = PciCfhBar3 {
            mem_io_ind: self.mem_io_ind(),
            mem_type: self.mem_type(),
            pf_mem: self.pf_mem(),
            addr: self.addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI base address register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfhBar4(pub u32);
impl PciCfhBar4 {
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    #[must_use]
    #[inline(always)]
    pub const fn mem_io_ind(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    #[inline(always)]
    pub const fn set_mem_io_ind(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    #[must_use]
    #[inline(always)]
    pub const fn mem_type(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    #[inline(always)]
    pub const fn set_mem_type(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    #[must_use]
    #[inline(always)]
    pub const fn pf_mem(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    #[inline(always)]
    pub const fn set_pf_mem(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "EA BARs used. This register is hardwired to 000000h."]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "EA BARs used. This register is hardwired to 000000h."]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
    }
}
impl Default for PciCfhBar4 {
    #[inline(always)]
    fn default() -> PciCfhBar4 {
        PciCfhBar4(0u64 as u32)
    }
}
impl core::fmt::Debug for PciCfhBar4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfhBar4")
            .field("mem_io_ind", &self.mem_io_ind())
            .field("mem_type", &self.mem_type())
            .field("pf_mem", &self.pf_mem())
            .field("addr", &self.addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfhBar4 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfhBar4 {
            mem_io_ind: bool,
            mem_type: u8,
            pf_mem: bool,
            addr: u32,
        }
        let proxy = PciCfhBar4 {
            mem_io_ind: self.mem_io_ind(),
            mem_type: self.mem_type(),
            pf_mem: self.pf_mem(),
            addr: self.addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI base address register 5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfhBar5(pub u32);
impl PciCfhBar5 {
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    #[must_use]
    #[inline(always)]
    pub const fn mem_io_ind(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    #[inline(always)]
    pub const fn set_mem_io_ind(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    #[must_use]
    #[inline(always)]
    pub const fn mem_type(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    #[inline(always)]
    pub const fn set_mem_type(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    #[must_use]
    #[inline(always)]
    pub const fn pf_mem(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    #[inline(always)]
    pub const fn set_pf_mem(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "EA BARs used. This register is hardwired to 000000h."]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "EA BARs used. This register is hardwired to 000000h."]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
    }
}
impl Default for PciCfhBar5 {
    #[inline(always)]
    fn default() -> PciCfhBar5 {
        PciCfhBar5(0u64 as u32)
    }
}
impl core::fmt::Debug for PciCfhBar5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfhBar5")
            .field("mem_io_ind", &self.mem_io_ind())
            .field("mem_type", &self.mem_type())
            .field("pf_mem", &self.pf_mem())
            .field("addr", &self.addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfhBar5 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfhBar5 {
            mem_io_ind: bool,
            mem_type: u8,
            pf_mem: bool,
            addr: u32,
        }
        let proxy = PciCfhBar5 {
            mem_io_ind: self.mem_io_ind(),
            mem_type: self.mem_type(),
            pf_mem: self.pf_mem(),
            addr: self.addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI command register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfhCmd(pub u16);
impl PciCfhCmd {
    #[doc = "Controls a device's response to memory space accesses. 0 Disabled 1 Enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn mem_access(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Controls a device's response to memory space accesses. 0 Disabled 1 Enabled"]
    #[inline(always)]
    pub const fn set_mem_access(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Bus master enable Controls the ability of a PCI Express Endpoint to issue Memory and I/O Read/Write Requests"]
    #[must_use]
    #[inline(always)]
    pub const fn bus_master_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Bus master enable Controls the ability of a PCI Express Endpoint to issue Memory and I/O Read/Write Requests"]
    #[inline(always)]
    pub const fn set_bus_master_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
}
impl Default for PciCfhCmd {
    #[inline(always)]
    fn default() -> PciCfhCmd {
        PciCfhCmd(0u64 as u16)
    }
}
impl core::fmt::Debug for PciCfhCmd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfhCmd")
            .field("mem_access", &self.mem_access())
            .field("bus_master_en", &self.bus_master_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfhCmd {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfhCmd {
            mem_access: bool,
            bus_master_en: bool,
        }
        let proxy = PciCfhCmd {
            mem_access: self.mem_access(),
            bus_master_en: self.bus_master_en(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI device ID and vendor ID register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfhDidVid(pub u32);
impl PciCfhDidVid {
    #[doc = "Vendor ID This field identifies the manufacturer of the device."]
    #[must_use]
    #[inline(always)]
    pub const fn vendor_id(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Vendor ID This field identifies the manufacturer of the device."]
    #[inline(always)]
    pub const fn set_vendor_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Device ID This field identifies the device ID of the device"]
    #[must_use]
    #[inline(always)]
    pub const fn device_id(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Device ID This field identifies the device ID of the device"]
    #[inline(always)]
    pub const fn set_device_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for PciCfhDidVid {
    #[inline(always)]
    fn default() -> PciCfhDidVid {
        PciCfhDidVid(3775928663u64 as u32)
    }
}
impl core::fmt::Debug for PciCfhDidVid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfhDidVid")
            .field("vendor_id", &self.vendor_id())
            .field("device_id", &self.device_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfhDidVid {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfhDidVid {
            vendor_id: u16,
            device_id: u16,
        }
        let proxy = PciCfhDidVid {
            vendor_id: self.vendor_id(),
            device_id: self.device_id(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI header type register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfhHdrType(pub u8);
impl PciCfhHdrType {
    #[doc = "Header type This field identifies the layout of the second part of the predefined header (beginning at byte 10h in Configuration Space)"]
    #[must_use]
    #[inline(always)]
    pub const fn hdr_type(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Header type This field identifies the layout of the second part of the predefined header (beginning at byte 10h in Configuration Space)"]
    #[inline(always)]
    pub const fn set_hdr_type(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u8) & 0x7f) << 0usize);
    }
    #[doc = "Multi-function device When set, indicates that the device may contain multiple functions, but not necessarily"]
    #[must_use]
    #[inline(always)]
    pub const fn mult_func_dev(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Multi-function device When set, indicates that the device may contain multiple functions, but not necessarily"]
    #[inline(always)]
    pub const fn set_mult_func_dev(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for PciCfhHdrType {
    #[inline(always)]
    fn default() -> PciCfhHdrType {
        PciCfhHdrType(128u64 as u8)
    }
}
impl core::fmt::Debug for PciCfhHdrType {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfhHdrType")
            .field("hdr_type", &self.hdr_type())
            .field("mult_func_dev", &self.mult_func_dev())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfhHdrType {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfhHdrType {
            hdr_type: u8,
            mult_func_dev: bool,
        }
        let proxy = PciCfhHdrType {
            hdr_type: self.hdr_type(),
            mult_func_dev: self.mult_func_dev(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI revision ID and classcode register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfhRevidClasscode(pub u32);
impl PciCfhRevidClasscode {
    #[doc = "Revision ID This register specifies a device specific revision identifier and is a vendor defined extension to the Device ID"]
    #[must_use]
    #[inline(always)]
    pub const fn rev_id(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Revision ID This register specifies a device specific revision identifier and is a vendor defined extension to the Device ID"]
    #[inline(always)]
    pub const fn set_rev_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Class code The Class Code register is read-only and is used to identify the generic function of the device and, in some cases, a specific register level programming interface"]
    #[must_use]
    #[inline(always)]
    pub const fn class_code(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Class code The Class Code register is read-only and is used to identify the generic function of the device and, in some cases, a specific register level programming interface"]
    #[inline(always)]
    pub const fn set_class_code(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for PciCfhRevidClasscode {
    #[inline(always)]
    fn default() -> PciCfhRevidClasscode {
        PciCfhRevidClasscode(33554691u64 as u32)
    }
}
impl core::fmt::Debug for PciCfhRevidClasscode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfhRevidClasscode")
            .field("rev_id", &self.rev_id())
            .field("class_code", &self.class_code())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfhRevidClasscode {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfhRevidClasscode {
            rev_id: u8,
            class_code: u32,
        }
        let proxy = PciCfhRevidClasscode {
            rev_id: self.rev_id(),
            class_code: self.class_code(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfhStat(pub u16);
impl PciCfhStat {
    #[doc = "Capabilities List Indicates the presence of an Extended Capability list item"]
    #[must_use]
    #[inline(always)]
    pub const fn cap_list(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Capabilities List Indicates the presence of an Extended Capability list item"]
    #[inline(always)]
    pub const fn set_cap_list(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
}
impl Default for PciCfhStat {
    #[inline(always)]
    fn default() -> PciCfhStat {
        PciCfhStat(16u64 as u16)
    }
}
impl core::fmt::Debug for PciCfhStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfhStat")
            .field("cap_list", &self.cap_list())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfhStat {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfhStat {
            cap_list: bool,
        }
        let proxy = PciCfhStat {
            cap_list: self.cap_list(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCIe ACS capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PcieCfcAcsCap(pub u16);
impl PcieCfcAcsCap {
    #[doc = "ACS translation blocking (B) Set to indicate that the Function supports Translation Blocking."]
    #[must_use]
    #[inline(always)]
    pub const fn acs_trans_block(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ACS translation blocking (B) Set to indicate that the Function supports Translation Blocking."]
    #[inline(always)]
    pub const fn set_acs_trans_block(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "ACS P2P request dedirect (R) Set to indicate that the Function supports peer-to-peer request redirection"]
    #[must_use]
    #[inline(always)]
    pub const fn acs_p2p_req_redir(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "ACS P2P request dedirect (R) Set to indicate that the Function supports peer-to-peer request redirection"]
    #[inline(always)]
    pub const fn set_acs_p2p_req_redir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "ACS direct translated P2P (T) Set to indicate that the Function supports direct translated peer-to-peer"]
    #[must_use]
    #[inline(always)]
    pub const fn acs_dir_trans_p2p(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "ACS direct translated P2P (T) Set to indicate that the Function supports direct translated peer-to-peer"]
    #[inline(always)]
    pub const fn set_acs_dir_trans_p2p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
}
impl Default for PcieCfcAcsCap {
    #[inline(always)]
    fn default() -> PcieCfcAcsCap {
        PcieCfcAcsCap(0u64 as u16)
    }
}
impl core::fmt::Debug for PcieCfcAcsCap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PcieCfcAcsCap")
            .field("acs_trans_block", &self.acs_trans_block())
            .field("acs_p2p_req_redir", &self.acs_p2p_req_redir())
            .field("acs_dir_trans_p2p", &self.acs_dir_trans_p2p())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PcieCfcAcsCap {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PcieCfcAcsCap {
            acs_trans_block: bool,
            acs_p2p_req_redir: bool,
            acs_dir_trans_p2p: bool,
        }
        let proxy = PcieCfcAcsCap {
            acs_trans_block: self.acs_trans_block(),
            acs_p2p_req_redir: self.acs_p2p_req_redir(),
            acs_dir_trans_p2p: self.acs_dir_trans_p2p(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCIe ACS capability header"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PcieCfcAcsCapHdr(pub u32);
impl PcieCfcAcsCapHdr {
    #[doc = "The Extended Capability ID for the ACS Extended Capability is 000Dh."]
    #[must_use]
    #[inline(always)]
    pub const fn pcie_ext_cap_id(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The Extended Capability ID for the ACS Extended Capability is 000Dh."]
    #[inline(always)]
    pub const fn set_pcie_ext_cap_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Capability version Must be 1h for this version of the specification."]
    #[must_use]
    #[inline(always)]
    pub const fn cap_ver(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Capability version Must be 1h for this version of the specification."]
    #[inline(always)]
    pub const fn set_cap_ver(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next capability offset This field contains the offset to the next PCI Express Capability structure or 000h if no other items exist in the linked list of capabilities"]
    #[must_use]
    #[inline(always)]
    pub const fn next_cap_off(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x0fff;
        val as u16
    }
    #[doc = "Next capability offset This field contains the offset to the next PCI Express Capability structure or 000h if no other items exist in the linked list of capabilities"]
    #[inline(always)]
    pub const fn set_next_cap_off(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
    }
}
impl Default for PcieCfcAcsCapHdr {
    #[inline(always)]
    fn default() -> PcieCfcAcsCapHdr {
        PcieCfcAcsCapHdr(335609869u64 as u32)
    }
}
impl core::fmt::Debug for PcieCfcAcsCapHdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PcieCfcAcsCapHdr")
            .field("pcie_ext_cap_id", &self.pcie_ext_cap_id())
            .field("cap_ver", &self.cap_ver())
            .field("next_cap_off", &self.next_cap_off())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PcieCfcAcsCapHdr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PcieCfcAcsCapHdr {
            pcie_ext_cap_id: u16,
            cap_ver: u8,
            next_cap_off: u16,
        }
        let proxy = PcieCfcAcsCapHdr {
            pcie_ext_cap_id: self.pcie_ext_cap_id(),
            cap_ver: self.cap_ver(),
            next_cap_off: self.next_cap_off(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCIe ACS control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PcieCfcAcsCtl(pub u16);
impl PcieCfcAcsCtl {
    #[doc = "ACS translation blocking enable (B) When set the Function directs peer-to-peer requests to the SoC interconnect"]
    #[must_use]
    #[inline(always)]
    pub const fn acs_trans_block_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ACS translation blocking enable (B) When set the Function directs peer-to-peer requests to the SoC interconnect"]
    #[inline(always)]
    pub const fn set_acs_trans_block_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "ACS P2P request dedirect enable (R) When set the Function directs peer-to-peer requests to the SoC interconnect"]
    #[must_use]
    #[inline(always)]
    pub const fn acs_p2p_req_redir_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "ACS P2P request dedirect enable (R) When set the Function directs peer-to-peer requests to the SoC interconnect"]
    #[inline(always)]
    pub const fn set_acs_p2p_req_redir_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "ACS direct translated P2P enable (T) When set the Function determines how to direct peer-to-peer requests based on ATS Ignored if ACS Translation Blocking Enable is set"]
    #[must_use]
    #[inline(always)]
    pub const fn acs_dir_trans_p2p_en(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "ACS direct translated P2P enable (T) When set the Function determines how to direct peer-to-peer requests based on ATS Ignored if ACS Translation Blocking Enable is set"]
    #[inline(always)]
    pub const fn set_acs_dir_trans_p2p_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
}
impl Default for PcieCfcAcsCtl {
    #[inline(always)]
    fn default() -> PcieCfcAcsCtl {
        PcieCfcAcsCtl(0u64 as u16)
    }
}
impl core::fmt::Debug for PcieCfcAcsCtl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PcieCfcAcsCtl")
            .field("acs_trans_block_en", &self.acs_trans_block_en())
            .field("acs_p2p_req_redir_en", &self.acs_p2p_req_redir_en())
            .field("acs_dir_trans_p2p_en", &self.acs_dir_trans_p2p_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PcieCfcAcsCtl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PcieCfcAcsCtl {
            acs_trans_block_en: bool,
            acs_p2p_req_redir_en: bool,
            acs_dir_trans_p2p_en: bool,
        }
        let proxy = PcieCfcAcsCtl {
            acs_trans_block_en: self.acs_trans_block_en(),
            acs_p2p_req_redir_en: self.acs_p2p_req_redir_en(),
            acs_dir_trans_p2p_en: self.acs_dir_trans_p2p_en(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCIe AER capabilities and control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PcieCfcAerCapCtl(pub u32);
impl PcieCfcAerCapCtl {
    #[doc = "First error pointer The First Error Pointer is a field that identifies the bit position of the first error reported in the Uncorrectable Error Status register"]
    #[must_use]
    #[inline(always)]
    pub const fn first_err_ptr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "First error pointer The First Error Pointer is a field that identifies the bit position of the first error reported in the Uncorrectable Error Status register"]
    #[inline(always)]
    pub const fn set_first_err_ptr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for PcieCfcAerCapCtl {
    #[inline(always)]
    fn default() -> PcieCfcAerCapCtl {
        PcieCfcAerCapCtl(0u64 as u32)
    }
}
impl core::fmt::Debug for PcieCfcAerCapCtl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PcieCfcAerCapCtl")
            .field("first_err_ptr", &self.first_err_ptr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PcieCfcAerCapCtl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PcieCfcAerCapCtl {
            first_err_ptr: u8,
        }
        let proxy = PcieCfcAerCapCtl {
            first_err_ptr: self.first_err_ptr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCIe AER correctable error mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PcieCfcAerCorrErrMask(pub u32);
impl PcieCfcAerCorrErrMask {
    #[doc = "Correctable internal error mask 0 Not masked 1 Masked This bit is \"sticky\" and will be preserved across function level resets"]
    #[must_use]
    #[inline(always)]
    pub const fn corr_int_mask(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Correctable internal error mask 0 Not masked 1 Masked This bit is \"sticky\" and will be preserved across function level resets"]
    #[inline(always)]
    pub const fn set_corr_int_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for PcieCfcAerCorrErrMask {
    #[inline(always)]
    fn default() -> PcieCfcAerCorrErrMask {
        PcieCfcAerCorrErrMask(16384u64 as u32)
    }
}
impl core::fmt::Debug for PcieCfcAerCorrErrMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PcieCfcAerCorrErrMask")
            .field("corr_int_mask", &self.corr_int_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PcieCfcAerCorrErrMask {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PcieCfcAerCorrErrMask {
            corr_int_mask: bool,
        }
        let proxy = PcieCfcAerCorrErrMask {
            corr_int_mask: self.corr_int_mask(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCIe AER correctable error status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PcieCfcAerCorrErrStat(pub u32);
impl PcieCfcAerCorrErrStat {
    #[doc = "Correctable internal error Set to indicate some correctable internal error has occurred such as a single bit error in an internal memory"]
    #[must_use]
    #[inline(always)]
    pub const fn corr_int_err(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Correctable internal error Set to indicate some correctable internal error has occurred such as a single bit error in an internal memory"]
    #[inline(always)]
    pub const fn set_corr_int_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for PcieCfcAerCorrErrStat {
    #[inline(always)]
    fn default() -> PcieCfcAerCorrErrStat {
        PcieCfcAerCorrErrStat(0u64 as u32)
    }
}
impl core::fmt::Debug for PcieCfcAerCorrErrStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PcieCfcAerCorrErrStat")
            .field("corr_int_err", &self.corr_int_err())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PcieCfcAerCorrErrStat {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PcieCfcAerCorrErrStat {
            corr_int_err: bool,
        }
        let proxy = PcieCfcAerCorrErrStat {
            corr_int_err: self.corr_int_err(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCIe AER extended capability header"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PcieCfcAerExtCapHdr(pub u32);
impl PcieCfcAerExtCapHdr {
    #[doc = "The Extended Capability ID for the AER Extended Capability is 0001h."]
    #[must_use]
    #[inline(always)]
    pub const fn pcie_ext_cap_id(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The Extended Capability ID for the AER Extended Capability is 0001h."]
    #[inline(always)]
    pub const fn set_pcie_ext_cap_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Capability version Must be 1h for this version of the specification."]
    #[must_use]
    #[inline(always)]
    pub const fn cap_ver(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Capability version Must be 1h for this version of the specification."]
    #[inline(always)]
    pub const fn set_cap_ver(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next capability offset This field contains the offset to the next PCI Express Capability structure or 000h if no other items exist in the linked list of capabilities"]
    #[must_use]
    #[inline(always)]
    pub const fn next_cap_off(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x0fff;
        val as u16
    }
    #[doc = "Next capability offset This field contains the offset to the next PCI Express Capability structure or 000h if no other items exist in the linked list of capabilities"]
    #[inline(always)]
    pub const fn set_next_cap_off(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
    }
}
impl Default for PcieCfcAerExtCapHdr {
    #[inline(always)]
    fn default() -> PcieCfcAerExtCapHdr {
        PcieCfcAerExtCapHdr(318832641u64 as u32)
    }
}
impl core::fmt::Debug for PcieCfcAerExtCapHdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PcieCfcAerExtCapHdr")
            .field("pcie_ext_cap_id", &self.pcie_ext_cap_id())
            .field("cap_ver", &self.cap_ver())
            .field("next_cap_off", &self.next_cap_off())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PcieCfcAerExtCapHdr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PcieCfcAerExtCapHdr {
            pcie_ext_cap_id: u16,
            cap_ver: u8,
            next_cap_off: u16,
        }
        let proxy = PcieCfcAerExtCapHdr {
            pcie_ext_cap_id: self.pcie_ext_cap_id(),
            cap_ver: self.cap_ver(),
            next_cap_off: self.next_cap_off(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCIe AER uncorrectable error mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PcieCfcAerUcorrErrMask(pub u32);
impl PcieCfcAerUcorrErrMask {
    #[doc = "Uncorrectable internal error mask 0 Not masked 1 Masked This bit is \"sticky\" and will be preserved across function level resets"]
    #[must_use]
    #[inline(always)]
    pub const fn ucorr_int_err_mask(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Uncorrectable internal error mask 0 Not masked 1 Masked This bit is \"sticky\" and will be preserved across function level resets"]
    #[inline(always)]
    pub const fn set_ucorr_int_err_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
}
impl Default for PcieCfcAerUcorrErrMask {
    #[inline(always)]
    fn default() -> PcieCfcAerUcorrErrMask {
        PcieCfcAerUcorrErrMask(4194304u64 as u32)
    }
}
impl core::fmt::Debug for PcieCfcAerUcorrErrMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PcieCfcAerUcorrErrMask")
            .field("ucorr_int_err_mask", &self.ucorr_int_err_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PcieCfcAerUcorrErrMask {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PcieCfcAerUcorrErrMask {
            ucorr_int_err_mask: bool,
        }
        let proxy = PcieCfcAerUcorrErrMask {
            ucorr_int_err_mask: self.ucorr_int_err_mask(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCIe AER uncorrectable error severity register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PcieCfcAerUcorrErrSev(pub u32);
impl PcieCfcAerUcorrErrSev {
    #[doc = "Uncorrectable internal severity 0 Non-fatal 1 Fatal This bit is \"sticky\" and will be preserved across function level resets"]
    #[must_use]
    #[inline(always)]
    pub const fn ucorr_int_sev(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Uncorrectable internal severity 0 Non-fatal 1 Fatal This bit is \"sticky\" and will be preserved across function level resets"]
    #[inline(always)]
    pub const fn set_ucorr_int_sev(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
}
impl Default for PcieCfcAerUcorrErrSev {
    #[inline(always)]
    fn default() -> PcieCfcAerUcorrErrSev {
        PcieCfcAerUcorrErrSev(4194304u64 as u32)
    }
}
impl core::fmt::Debug for PcieCfcAerUcorrErrSev {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PcieCfcAerUcorrErrSev")
            .field("ucorr_int_sev", &self.ucorr_int_sev())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PcieCfcAerUcorrErrSev {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PcieCfcAerUcorrErrSev {
            ucorr_int_sev: bool,
        }
        let proxy = PcieCfcAerUcorrErrSev {
            ucorr_int_sev: self.ucorr_int_sev(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCIe AER uncorrectable error status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PcieCfcAerUcorrErrStat(pub u32);
impl PcieCfcAerUcorrErrStat {
    #[doc = "ACS violation status Set to indicate an ACS violation such as an access from a disallowed source"]
    #[must_use]
    #[inline(always)]
    pub const fn acs_violation_stat(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "ACS violation status Set to indicate an ACS violation such as an access from a disallowed source"]
    #[inline(always)]
    pub const fn set_acs_violation_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Uncorrectable internal error Set to indicate some uncorrectable internal error such as a multi-bit error in an internal memory has occurred"]
    #[must_use]
    #[inline(always)]
    pub const fn ucorr_int_err(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Uncorrectable internal error Set to indicate some uncorrectable internal error such as a multi-bit error in an internal memory has occurred"]
    #[inline(always)]
    pub const fn set_ucorr_int_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
}
impl Default for PcieCfcAerUcorrErrStat {
    #[inline(always)]
    fn default() -> PcieCfcAerUcorrErrStat {
        PcieCfcAerUcorrErrStat(0u64 as u32)
    }
}
impl core::fmt::Debug for PcieCfcAerUcorrErrStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PcieCfcAerUcorrErrStat")
            .field("acs_violation_stat", &self.acs_violation_stat())
            .field("ucorr_int_err", &self.ucorr_int_err())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PcieCfcAerUcorrErrStat {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PcieCfcAerUcorrErrStat {
            acs_violation_stat: bool,
            ucorr_int_err: bool,
        }
        let proxy = PcieCfcAerUcorrErrStat {
            acs_violation_stat: self.acs_violation_stat(),
            ucorr_int_err: self.ucorr_int_err(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCIe readiness time reporting capability header"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PcieCfcRtrCapHdr(pub u32);
impl PcieCfcRtrCapHdr {
    #[doc = "The Extended Capability ID for the Readiness Time Reporting Extended Capability is 0022h."]
    #[must_use]
    #[inline(always)]
    pub const fn pcie_ext_cap_id(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The Extended Capability ID for the Readiness Time Reporting Extended Capability is 0022h."]
    #[inline(always)]
    pub const fn set_pcie_ext_cap_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Capability version Must be 1h for this version of the specification."]
    #[must_use]
    #[inline(always)]
    pub const fn cap_ver(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Capability version Must be 1h for this version of the specification."]
    #[inline(always)]
    pub const fn set_cap_ver(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next capability offset This field contains the offset to the next PCI Express Capability structure or 000h if no other items exist in the linked list of capabilities"]
    #[must_use]
    #[inline(always)]
    pub const fn next_cap_off(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x0fff;
        val as u16
    }
    #[doc = "Next capability offset This field contains the offset to the next PCI Express Capability structure or 000h if no other items exist in the linked list of capabilities"]
    #[inline(always)]
    pub const fn set_next_cap_off(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
    }
}
impl Default for PcieCfcRtrCapHdr {
    #[inline(always)]
    fn default() -> PcieCfcRtrCapHdr {
        PcieCfcRtrCapHdr(352387106u64 as u32)
    }
}
impl core::fmt::Debug for PcieCfcRtrCapHdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PcieCfcRtrCapHdr")
            .field("pcie_ext_cap_id", &self.pcie_ext_cap_id())
            .field("cap_ver", &self.cap_ver())
            .field("next_cap_off", &self.next_cap_off())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PcieCfcRtrCapHdr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PcieCfcRtrCapHdr {
            pcie_ext_cap_id: u16,
            cap_ver: u8,
            next_cap_off: u16,
        }
        let proxy = PcieCfcRtrCapHdr {
            pcie_ext_cap_id: self.pcie_ext_cap_id(),
            cap_ver: self.cap_ver(),
            next_cap_off: self.next_cap_off(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCIe RTR readiness time reporting 1 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PcieCfcRtrRtr1(pub u32);
impl PcieCfcRtrRtr1 {
    #[doc = "Reset Time"]
    #[must_use]
    #[inline(always)]
    pub const fn reset_time(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Reset Time"]
    #[inline(always)]
    pub const fn set_reset_time(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Valid"]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PcieCfcRtrRtr1 {
    #[inline(always)]
    fn default() -> PcieCfcRtrRtr1 {
        PcieCfcRtrRtr1(2147483648u64 as u32)
    }
}
impl core::fmt::Debug for PcieCfcRtrRtr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PcieCfcRtrRtr1")
            .field("reset_time", &self.reset_time())
            .field("valid", &self.valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PcieCfcRtrRtr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PcieCfcRtrRtr1 {
            reset_time: u16,
            valid: bool,
        }
        let proxy = PcieCfcRtrRtr1 {
            reset_time: self.reset_time(),
            valid: self.valid(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCIe RTR readiness time reporting 2 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PcieCfcRtrRtr2(pub u32);
impl PcieCfcRtrRtr2 {
    #[doc = "FLR Time"]
    #[must_use]
    #[inline(always)]
    pub const fn flr_time(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "FLR Time"]
    #[inline(always)]
    pub const fn set_flr_time(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "D3 hot to D0 time"]
    #[must_use]
    #[inline(always)]
    pub const fn d3hot_d0_time(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[doc = "D3 hot to D0 time"]
    #[inline(always)]
    pub const fn set_d3hot_d0_time(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
}
impl Default for PcieCfcRtrRtr2 {
    #[inline(always)]
    fn default() -> PcieCfcRtrRtr2 {
        PcieCfcRtrRtr2(1588u64 as u32)
    }
}
impl core::fmt::Debug for PcieCfcRtrRtr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PcieCfcRtrRtr2")
            .field("flr_time", &self.flr_time())
            .field("d3hot_d0_time", &self.d3hot_d0_time())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PcieCfcRtrRtr2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PcieCfcRtrRtr2 {
            flr_time: u16,
            d3hot_d0_time: u16,
        }
        let proxy = PcieCfcRtrRtr2 {
            flr_time: self.flr_time(),
            d3hot_d0_time: self.d3hot_d0_time(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCIe SR-IOV capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PcieCfcSriovCap(pub u32);
impl PcieCfcSriovCap {
    #[doc = "VF migration capable Migration is supported only with MR-IOV."]
    #[must_use]
    #[inline(always)]
    pub const fn vf_migration_cap(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "VF migration capable Migration is supported only with MR-IOV."]
    #[inline(always)]
    pub const fn set_vf_migration_cap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ARI capable hierarchy preserved ARI is not supported by Integrated End Points"]
    #[must_use]
    #[inline(always)]
    pub const fn ari_cap_hier_prsv(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ARI capable hierarchy preserved ARI is not supported by Integrated End Points"]
    #[inline(always)]
    pub const fn set_ari_cap_hier_prsv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "VF migration interrupt message number Migration is supported only with MR-IOV."]
    #[must_use]
    #[inline(always)]
    pub const fn vf_migration_ing_msg_num(&self) -> u16 {
        let val = (self.0 >> 21usize) & 0x07ff;
        val as u16
    }
    #[doc = "VF migration interrupt message number Migration is supported only with MR-IOV."]
    #[inline(always)]
    pub const fn set_vf_migration_ing_msg_num(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 21usize)) | (((val as u32) & 0x07ff) << 21usize);
    }
}
impl Default for PcieCfcSriovCap {
    #[inline(always)]
    fn default() -> PcieCfcSriovCap {
        PcieCfcSriovCap(0u64 as u32)
    }
}
impl core::fmt::Debug for PcieCfcSriovCap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PcieCfcSriovCap")
            .field("vf_migration_cap", &self.vf_migration_cap())
            .field("ari_cap_hier_prsv", &self.ari_cap_hier_prsv())
            .field("vf_migration_ing_msg_num", &self.vf_migration_ing_msg_num())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PcieCfcSriovCap {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PcieCfcSriovCap {
            vf_migration_cap: bool,
            ari_cap_hier_prsv: bool,
            vf_migration_ing_msg_num: u16,
        }
        let proxy = PcieCfcSriovCap {
            vf_migration_cap: self.vf_migration_cap(),
            ari_cap_hier_prsv: self.ari_cap_hier_prsv(),
            vf_migration_ing_msg_num: self.vf_migration_ing_msg_num(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCIe SR-IOV capability header"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PcieCfcSriovCapHdr(pub u32);
impl PcieCfcSriovCapHdr {
    #[doc = "The Extended Capability ID for the ACS Extended Capability is 0010h."]
    #[must_use]
    #[inline(always)]
    pub const fn pcie_ext_cap_id(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The Extended Capability ID for the ACS Extended Capability is 0010h."]
    #[inline(always)]
    pub const fn set_pcie_ext_cap_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Capability version Must be 1h for this version of the specification."]
    #[must_use]
    #[inline(always)]
    pub const fn cap_ver(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Capability version Must be 1h for this version of the specification."]
    #[inline(always)]
    pub const fn set_cap_ver(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next capability offset This field contains the offset to the next PCI Express Capability structure or 000h if no other items exist in the linked list of capabilities"]
    #[must_use]
    #[inline(always)]
    pub const fn next_cap_off(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x0fff;
        val as u16
    }
    #[doc = "Next capability offset This field contains the offset to the next PCI Express Capability structure or 000h if no other items exist in the linked list of capabilities"]
    #[inline(always)]
    pub const fn set_next_cap_off(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
    }
}
impl Default for PcieCfcSriovCapHdr {
    #[inline(always)]
    fn default() -> PcieCfcSriovCapHdr {
        PcieCfcSriovCapHdr(65552u64 as u32)
    }
}
impl core::fmt::Debug for PcieCfcSriovCapHdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PcieCfcSriovCapHdr")
            .field("pcie_ext_cap_id", &self.pcie_ext_cap_id())
            .field("cap_ver", &self.cap_ver())
            .field("next_cap_off", &self.next_cap_off())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PcieCfcSriovCapHdr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PcieCfcSriovCapHdr {
            pcie_ext_cap_id: u16,
            cap_ver: u8,
            next_cap_off: u16,
        }
        let proxy = PcieCfcSriovCapHdr {
            pcie_ext_cap_id: self.pcie_ext_cap_id(),
            cap_ver: self.cap_ver(),
            next_cap_off: self.next_cap_off(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCIe SR-IOV control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PcieCfcSriovCtl(pub u16);
impl PcieCfcSriovCtl {
    #[doc = "VF enable 0 Disabled 1 Enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn vf_enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "VF enable 0 Disabled 1 Enabled"]
    #[inline(always)]
    pub const fn set_vf_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "VF migration enable Migration is supported only with MR-IOV. Hardwired to 0b."]
    #[must_use]
    #[inline(always)]
    pub const fn vf_migration_enable(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "VF migration enable Migration is supported only with MR-IOV. Hardwired to 0b."]
    #[inline(always)]
    pub const fn set_vf_migration_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "VF migration interrupt enable Migration is supported only with MR-IOV. Hardwired to 0b."]
    #[must_use]
    #[inline(always)]
    pub const fn vf_migration_int_enable(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "VF migration interrupt enable Migration is supported only with MR-IOV. Hardwired to 0b."]
    #[inline(always)]
    pub const fn set_vf_migration_int_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Memory Space Enable for Virtual Functions When VF Enable is Set, VF memory space will respond only when VF MSE is Set"]
    #[must_use]
    #[inline(always)]
    pub const fn vf_mse(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Memory Space Enable for Virtual Functions When VF Enable is Set, VF memory space will respond only when VF MSE is Set"]
    #[inline(always)]
    pub const fn set_vf_mse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "ARI capable hierarchy ARI is not supported by Integrated End Points"]
    #[must_use]
    #[inline(always)]
    pub const fn ari_cap_hierarchy(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "ARI capable hierarchy ARI is not supported by Integrated End Points"]
    #[inline(always)]
    pub const fn set_ari_cap_hierarchy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "PCI-PM D2 support Not supported by ENETC. Hardwired to 0b."]
    #[must_use]
    #[inline(always)]
    pub const fn d2_support(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PCI-PM D2 support Not supported by ENETC. Hardwired to 0b."]
    #[inline(always)]
    pub const fn set_d2_support(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "PME support Wake-on-LAN events not supported by ENETC VFs. Hardwired to 00000b."]
    #[must_use]
    #[inline(always)]
    pub const fn pme_support(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "PME support Wake-on-LAN events not supported by ENETC VFs. Hardwired to 00000b."]
    #[inline(always)]
    pub const fn set_pme_support(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for PcieCfcSriovCtl {
    #[inline(always)]
    fn default() -> PcieCfcSriovCtl {
        PcieCfcSriovCtl(0u64 as u16)
    }
}
impl core::fmt::Debug for PcieCfcSriovCtl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PcieCfcSriovCtl")
            .field("vf_enable", &self.vf_enable())
            .field("vf_migration_enable", &self.vf_migration_enable())
            .field("vf_migration_int_enable", &self.vf_migration_int_enable())
            .field("vf_mse", &self.vf_mse())
            .field("ari_cap_hierarchy", &self.ari_cap_hierarchy())
            .field("d2_support", &self.d2_support())
            .field("pme_support", &self.pme_support())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PcieCfcSriovCtl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PcieCfcSriovCtl {
            vf_enable: bool,
            vf_migration_enable: bool,
            vf_migration_int_enable: bool,
            vf_mse: bool,
            ari_cap_hierarchy: bool,
            d2_support: bool,
            pme_support: u8,
        }
        let proxy = PcieCfcSriovCtl {
            vf_enable: self.vf_enable(),
            vf_migration_enable: self.vf_migration_enable(),
            vf_migration_int_enable: self.vf_migration_int_enable(),
            vf_mse: self.vf_mse(),
            ari_cap_hierarchy: self.ari_cap_hierarchy(),
            d2_support: self.d2_support(),
            pme_support: self.pme_support(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCIe SR-IOV status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PcieCfcSriovStat(pub u16);
impl PcieCfcSriovStat {
    #[doc = "VF migration status Migration is supported only with MR-IOV. Hardwired to 0b."]
    #[must_use]
    #[inline(always)]
    pub const fn vf_migration_status(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "VF migration status Migration is supported only with MR-IOV. Hardwired to 0b."]
    #[inline(always)]
    pub const fn set_vf_migration_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
}
impl Default for PcieCfcSriovStat {
    #[inline(always)]
    fn default() -> PcieCfcSriovStat {
        PcieCfcSriovStat(0u64 as u16)
    }
}
impl core::fmt::Debug for PcieCfcSriovStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PcieCfcSriovStat")
            .field("vf_migration_status", &self.vf_migration_status())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PcieCfcSriovStat {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PcieCfcSriovStat {
            vf_migration_status: bool,
        }
        let proxy = PcieCfcSriovStat {
            vf_migration_status: self.vf_migration_status(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCIe SR-IOV VF base address register a"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PcieCfcVfBar(pub u32);
impl PcieCfcVfBar {
    #[doc = "BARs used. This register is hardwired to 0h."]
    #[must_use]
    #[inline(always)]
    pub const fn mem_io_ind(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "BARs used. This register is hardwired to 0h."]
    #[inline(always)]
    pub const fn set_mem_io_ind(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "BARs used. This register is hardwired to 0h."]
    #[must_use]
    #[inline(always)]
    pub const fn mem_type(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "BARs used. This register is hardwired to 0h."]
    #[inline(always)]
    pub const fn set_mem_type(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[doc = "BARs used. This register is hardwired to 0h."]
    #[must_use]
    #[inline(always)]
    pub const fn pf_mem(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "BARs used. This register is hardwired to 0h."]
    #[inline(always)]
    pub const fn set_pf_mem(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Address space (low register for 64-bit BARs). Not supported, hardwired to 000000h."]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "Address space (low register for 64-bit BARs). Not supported, hardwired to 000000h."]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
    }
}
impl Default for PcieCfcVfBar {
    #[inline(always)]
    fn default() -> PcieCfcVfBar {
        PcieCfcVfBar(0u64 as u32)
    }
}
impl core::fmt::Debug for PcieCfcVfBar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PcieCfcVfBar")
            .field("mem_io_ind", &self.mem_io_ind())
            .field("mem_type", &self.mem_type())
            .field("pf_mem", &self.pf_mem())
            .field("addr", &self.addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PcieCfcVfBar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PcieCfcVfBar {
            mem_io_ind: bool,
            mem_type: u8,
            pf_mem: bool,
            addr: u32,
        }
        let proxy = PcieCfcVfBar {
            mem_io_ind: self.mem_io_ind(),
            mem_type: self.mem_type(),
            pf_mem: self.pf_mem(),
            addr: self.addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
