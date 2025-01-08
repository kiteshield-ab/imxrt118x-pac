#[doc = "Receive Buffer Region 0 End Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbbufregionend0(pub u32);
impl Ahbbufregionend0 {
    #[doc = "End Address of Prefetch Sub-Buffer Region"]
    #[must_use]
    #[inline(always)]
    pub const fn end_address(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "End Address of Prefetch Sub-Buffer Region"]
    #[inline(always)]
    pub const fn set_end_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Ahbbufregionend0 {
    #[inline(always)]
    fn default() -> Ahbbufregionend0 {
        Ahbbufregionend0(0u64 as u32)
    }
}
impl core::fmt::Debug for Ahbbufregionend0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbbufregionend0")
            .field("end_address", &self.end_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbbufregionend0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ahbbufregionend0 {
            end_address: u32,
        }
        let proxy = Ahbbufregionend0 {
            end_address: self.end_address(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Receive Buffer Region 1 End Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbbufregionend1(pub u32);
impl Ahbbufregionend1 {
    #[doc = "End Address of Prefetch Sub-Buffer Region"]
    #[must_use]
    #[inline(always)]
    pub const fn end_address(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "End Address of Prefetch Sub-Buffer Region"]
    #[inline(always)]
    pub const fn set_end_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Ahbbufregionend1 {
    #[inline(always)]
    fn default() -> Ahbbufregionend1 {
        Ahbbufregionend1(0u64 as u32)
    }
}
impl core::fmt::Debug for Ahbbufregionend1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbbufregionend1")
            .field("end_address", &self.end_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbbufregionend1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ahbbufregionend1 {
            end_address: u32,
        }
        let proxy = Ahbbufregionend1 {
            end_address: self.end_address(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Receive Buffer Region 2 End Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbbufregionend2(pub u32);
impl Ahbbufregionend2 {
    #[doc = "End Address of Prefetch Sub-Buffer Region"]
    #[must_use]
    #[inline(always)]
    pub const fn end_address(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "End Address of Prefetch Sub-Buffer Region"]
    #[inline(always)]
    pub const fn set_end_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Ahbbufregionend2 {
    #[inline(always)]
    fn default() -> Ahbbufregionend2 {
        Ahbbufregionend2(0u64 as u32)
    }
}
impl core::fmt::Debug for Ahbbufregionend2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbbufregionend2")
            .field("end_address", &self.end_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbbufregionend2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ahbbufregionend2 {
            end_address: u32,
        }
        let proxy = Ahbbufregionend2 {
            end_address: self.end_address(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Receive Buffer Region 3 End Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbbufregionend3(pub u32);
impl Ahbbufregionend3 {
    #[doc = "End Address of Prefetch Sub-Buffer Region"]
    #[must_use]
    #[inline(always)]
    pub const fn end_address(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "End Address of Prefetch Sub-Buffer Region"]
    #[inline(always)]
    pub const fn set_end_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Ahbbufregionend3 {
    #[inline(always)]
    fn default() -> Ahbbufregionend3 {
        Ahbbufregionend3(0u64 as u32)
    }
}
impl core::fmt::Debug for Ahbbufregionend3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbbufregionend3")
            .field("end_address", &self.end_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbbufregionend3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ahbbufregionend3 {
            end_address: u32,
        }
        let proxy = Ahbbufregionend3 {
            end_address: self.end_address(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Receive Buffer Start Address of Region 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbbufregionstart0(pub u32);
impl Ahbbufregionstart0 {
    #[doc = "Start Address of Prefetch Sub-Buffer Region"]
    #[must_use]
    #[inline(always)]
    pub const fn start_address(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Start Address of Prefetch Sub-Buffer Region"]
    #[inline(always)]
    pub const fn set_start_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Ahbbufregionstart0 {
    #[inline(always)]
    fn default() -> Ahbbufregionstart0 {
        Ahbbufregionstart0(0u64 as u32)
    }
}
impl core::fmt::Debug for Ahbbufregionstart0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbbufregionstart0")
            .field("start_address", &self.start_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbbufregionstart0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ahbbufregionstart0 {
            start_address: u32,
        }
        let proxy = Ahbbufregionstart0 {
            start_address: self.start_address(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Receive Buffer Start Address of Region 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbbufregionstart1(pub u32);
impl Ahbbufregionstart1 {
    #[doc = "Start Address of Prefetch Sub-Buffer Region"]
    #[must_use]
    #[inline(always)]
    pub const fn start_address(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Start Address of Prefetch Sub-Buffer Region"]
    #[inline(always)]
    pub const fn set_start_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Ahbbufregionstart1 {
    #[inline(always)]
    fn default() -> Ahbbufregionstart1 {
        Ahbbufregionstart1(0u64 as u32)
    }
}
impl core::fmt::Debug for Ahbbufregionstart1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbbufregionstart1")
            .field("start_address", &self.start_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbbufregionstart1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ahbbufregionstart1 {
            start_address: u32,
        }
        let proxy = Ahbbufregionstart1 {
            start_address: self.start_address(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Receive Buffer Start Address of Region 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbbufregionstart2(pub u32);
impl Ahbbufregionstart2 {
    #[doc = "Start Address of Prefetch Sub-Buffer Region"]
    #[must_use]
    #[inline(always)]
    pub const fn start_address(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Start Address of Prefetch Sub-Buffer Region"]
    #[inline(always)]
    pub const fn set_start_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Ahbbufregionstart2 {
    #[inline(always)]
    fn default() -> Ahbbufregionstart2 {
        Ahbbufregionstart2(0u64 as u32)
    }
}
impl core::fmt::Debug for Ahbbufregionstart2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbbufregionstart2")
            .field("start_address", &self.start_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbbufregionstart2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ahbbufregionstart2 {
            start_address: u32,
        }
        let proxy = Ahbbufregionstart2 {
            start_address: self.start_address(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Receive Buffer Start Address of Region 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbbufregionstart3(pub u32);
impl Ahbbufregionstart3 {
    #[doc = "Start Address of Prefetch Sub-Buffer Region"]
    #[must_use]
    #[inline(always)]
    pub const fn start_address(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Start Address of Prefetch Sub-Buffer Region"]
    #[inline(always)]
    pub const fn set_start_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Ahbbufregionstart3 {
    #[inline(always)]
    fn default() -> Ahbbufregionstart3 {
        Ahbbufregionstart3(0u64 as u32)
    }
}
impl core::fmt::Debug for Ahbbufregionstart3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbbufregionstart3")
            .field("start_address", &self.start_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbbufregionstart3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ahbbufregionstart3 {
            start_address: u32,
        }
        let proxy = Ahbbufregionstart3 {
            start_address: self.start_address(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "AHB Bus Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbcr(pub u32);
impl Ahbcr {
    #[doc = "AHB Parallel Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn aparen(&self) -> super::vals::Aparen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Aparen::from_bits(val as u8)
    }
    #[doc = "AHB Parallel Mode Enable"]
    #[inline(always)]
    pub const fn set_aparen(&mut self, val: super::vals::Aparen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Clear AHB Transmit Buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn clrahbtxbuf(&self) -> super::vals::Clrahbtxbuf {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Clrahbtxbuf::from_bits(val as u8)
    }
    #[doc = "Clear AHB Transmit Buffer"]
    #[inline(always)]
    pub const fn set_clrahbtxbuf(&mut self, val: super::vals::Clrahbtxbuf) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Cacheable Read Access Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cachableen(&self) -> super::vals::Cachableen {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Cachableen::from_bits(val as u8)
    }
    #[doc = "Cacheable Read Access Enable"]
    #[inline(always)]
    pub const fn set_cachableen(&mut self, val: super::vals::Cachableen) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Bufferable Write Access Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bufferableen(&self) -> super::vals::Bufferableen {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Bufferableen::from_bits(val as u8)
    }
    #[doc = "Bufferable Write Access Enable"]
    #[inline(always)]
    pub const fn set_bufferableen(&mut self, val: super::vals::Bufferableen) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "AHB Read Prefetch Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn prefetchen(&self) -> super::vals::AhbcrPrefetchen {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::AhbcrPrefetchen::from_bits(val as u8)
    }
    #[doc = "AHB Read Prefetch Enable"]
    #[inline(always)]
    pub const fn set_prefetchen(&mut self, val: super::vals::AhbcrPrefetchen) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "AHB Read Address Option"]
    #[must_use]
    #[inline(always)]
    pub const fn readaddropt(&self) -> super::vals::Readaddropt {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Readaddropt::from_bits(val as u8)
    }
    #[doc = "AHB Read Address Option"]
    #[inline(always)]
    pub const fn set_readaddropt(&mut self, val: super::vals::Readaddropt) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "AHB Read Resume Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn resumedisable(&self) -> super::vals::Resumedisable {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Resumedisable::from_bits(val as u8)
    }
    #[doc = "AHB Read Resume Disable"]
    #[inline(always)]
    pub const fn set_resumedisable(&mut self, val: super::vals::Resumedisable) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "AHB Read Size Alignment"]
    #[must_use]
    #[inline(always)]
    pub const fn readszalign(&self) -> super::vals::Readszalign {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Readszalign::from_bits(val as u8)
    }
    #[doc = "AHB Read Size Alignment"]
    #[inline(always)]
    pub const fn set_readszalign(&mut self, val: super::vals::Readszalign) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "AHB Boundary Alignment"]
    #[must_use]
    #[inline(always)]
    pub const fn alignment(&self) -> super::vals::Alignment {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Alignment::from_bits(val as u8)
    }
    #[doc = "AHB Boundary Alignment"]
    #[inline(always)]
    pub const fn set_alignment(&mut self, val: super::vals::Alignment) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "AHB Memory-Mapped Flash Base Address"]
    #[must_use]
    #[inline(always)]
    pub const fn aflashbase(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x1f;
        val as u8
    }
    #[doc = "AHB Memory-Mapped Flash Base Address"]
    #[inline(always)]
    pub const fn set_aflashbase(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 27usize)) | (((val as u32) & 0x1f) << 27usize);
    }
}
impl Default for Ahbcr {
    #[inline(always)]
    fn default() -> Ahbcr {
        Ahbcr(24u64 as u32)
    }
}
impl core::fmt::Debug for Ahbcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbcr")
            .field("aparen", &self.aparen())
            .field("clrahbtxbuf", &self.clrahbtxbuf())
            .field("cachableen", &self.cachableen())
            .field("bufferableen", &self.bufferableen())
            .field("prefetchen", &self.prefetchen())
            .field("readaddropt", &self.readaddropt())
            .field("resumedisable", &self.resumedisable())
            .field("readszalign", &self.readszalign())
            .field("alignment", &self.alignment())
            .field("aflashbase", &self.aflashbase())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ahbcr {
            aparen: super::vals::Aparen,
            clrahbtxbuf: super::vals::Clrahbtxbuf,
            cachableen: super::vals::Cachableen,
            bufferableen: super::vals::Bufferableen,
            prefetchen: super::vals::AhbcrPrefetchen,
            readaddropt: super::vals::Readaddropt,
            resumedisable: super::vals::Resumedisable,
            readszalign: super::vals::Readszalign,
            alignment: super::vals::Alignment,
            aflashbase: u8,
        }
        let proxy = Ahbcr {
            aparen: self.aparen(),
            clrahbtxbuf: self.clrahbtxbuf(),
            cachableen: self.cachableen(),
            bufferableen: self.bufferableen(),
            prefetchen: self.prefetchen(),
            readaddropt: self.readaddropt(),
            resumedisable: self.resumedisable(),
            readszalign: self.readszalign(),
            alignment: self.alignment(),
            aflashbase: self.aflashbase(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "AHB Receive Buffer 0 Control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbrxbuf0cr0(pub u32);
impl Ahbrxbuf0cr0 {
    #[doc = "AHB Receive Buffer Size"]
    #[must_use]
    #[inline(always)]
    pub const fn bufsz(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "AHB Receive Buffer Size"]
    #[inline(always)]
    pub const fn set_bufsz(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "AHB Controller ID"]
    #[must_use]
    #[inline(always)]
    pub const fn mstrid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "AHB Controller ID"]
    #[inline(always)]
    pub const fn set_mstrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "AHB Controller Read Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn priority(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "AHB Controller Read Priority"]
    #[inline(always)]
    pub const fn set_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "AHB Receive Buffer Address Region Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn regionen(&self) -> super::vals::Ahbrxbuf0cr0Regionen {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Ahbrxbuf0cr0Regionen::from_bits(val as u8)
    }
    #[doc = "AHB Receive Buffer Address Region Enable"]
    #[inline(always)]
    pub const fn set_regionen(&mut self, val: super::vals::Ahbrxbuf0cr0Regionen) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "AHB Read Prefetch Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn prefetchen(&self) -> super::vals::Ahbrxbuf0cr0Prefetchen {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Ahbrxbuf0cr0Prefetchen::from_bits(val as u8)
    }
    #[doc = "AHB Read Prefetch Enable"]
    #[inline(always)]
    pub const fn set_prefetchen(&mut self, val: super::vals::Ahbrxbuf0cr0Prefetchen) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ahbrxbuf0cr0 {
    #[inline(always)]
    fn default() -> Ahbrxbuf0cr0 {
        Ahbrxbuf0cr0(2147483712u64 as u32)
    }
}
impl core::fmt::Debug for Ahbrxbuf0cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbrxbuf0cr0")
            .field("bufsz", &self.bufsz())
            .field("mstrid", &self.mstrid())
            .field("priority", &self.priority())
            .field("regionen", &self.regionen())
            .field("prefetchen", &self.prefetchen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbrxbuf0cr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ahbrxbuf0cr0 {
            bufsz: u16,
            mstrid: u8,
            priority: u8,
            regionen: super::vals::Ahbrxbuf0cr0Regionen,
            prefetchen: super::vals::Ahbrxbuf0cr0Prefetchen,
        }
        let proxy = Ahbrxbuf0cr0 {
            bufsz: self.bufsz(),
            mstrid: self.mstrid(),
            priority: self.priority(),
            regionen: self.regionen(),
            prefetchen: self.prefetchen(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "AHB Receive Buffer 1 Control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbrxbuf1cr0(pub u32);
impl Ahbrxbuf1cr0 {
    #[doc = "AHB Receive Buffer Size"]
    #[must_use]
    #[inline(always)]
    pub const fn bufsz(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "AHB Receive Buffer Size"]
    #[inline(always)]
    pub const fn set_bufsz(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "AHB Controller ID"]
    #[must_use]
    #[inline(always)]
    pub const fn mstrid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "AHB Controller ID"]
    #[inline(always)]
    pub const fn set_mstrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "AHB Controller Read Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn priority(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "AHB Controller Read Priority"]
    #[inline(always)]
    pub const fn set_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "AHB Receive Buffer Address Region Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn regionen(&self) -> super::vals::Ahbrxbuf1cr0Regionen {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Ahbrxbuf1cr0Regionen::from_bits(val as u8)
    }
    #[doc = "AHB Receive Buffer Address Region Enable"]
    #[inline(always)]
    pub const fn set_regionen(&mut self, val: super::vals::Ahbrxbuf1cr0Regionen) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "AHB Read Prefetch Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn prefetchen(&self) -> super::vals::Ahbrxbuf1cr0Prefetchen {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Ahbrxbuf1cr0Prefetchen::from_bits(val as u8)
    }
    #[doc = "AHB Read Prefetch Enable"]
    #[inline(always)]
    pub const fn set_prefetchen(&mut self, val: super::vals::Ahbrxbuf1cr0Prefetchen) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ahbrxbuf1cr0 {
    #[inline(always)]
    fn default() -> Ahbrxbuf1cr0 {
        Ahbrxbuf1cr0(2147549248u64 as u32)
    }
}
impl core::fmt::Debug for Ahbrxbuf1cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbrxbuf1cr0")
            .field("bufsz", &self.bufsz())
            .field("mstrid", &self.mstrid())
            .field("priority", &self.priority())
            .field("regionen", &self.regionen())
            .field("prefetchen", &self.prefetchen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbrxbuf1cr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ahbrxbuf1cr0 {
            bufsz: u16,
            mstrid: u8,
            priority: u8,
            regionen: super::vals::Ahbrxbuf1cr0Regionen,
            prefetchen: super::vals::Ahbrxbuf1cr0Prefetchen,
        }
        let proxy = Ahbrxbuf1cr0 {
            bufsz: self.bufsz(),
            mstrid: self.mstrid(),
            priority: self.priority(),
            regionen: self.regionen(),
            prefetchen: self.prefetchen(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "AHB Receive Buffer 2 Control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbrxbuf2cr0(pub u32);
impl Ahbrxbuf2cr0 {
    #[doc = "AHB Receive Buffer Size"]
    #[must_use]
    #[inline(always)]
    pub const fn bufsz(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "AHB Receive Buffer Size"]
    #[inline(always)]
    pub const fn set_bufsz(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "AHB Controller ID"]
    #[must_use]
    #[inline(always)]
    pub const fn mstrid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "AHB Controller ID"]
    #[inline(always)]
    pub const fn set_mstrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "AHB Controller Read Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn priority(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "AHB Controller Read Priority"]
    #[inline(always)]
    pub const fn set_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "AHB Receive Buffer Address Region Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn regionen(&self) -> super::vals::Ahbrxbuf2cr0Regionen {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Ahbrxbuf2cr0Regionen::from_bits(val as u8)
    }
    #[doc = "AHB Receive Buffer Address Region Enable"]
    #[inline(always)]
    pub const fn set_regionen(&mut self, val: super::vals::Ahbrxbuf2cr0Regionen) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "AHB Read Prefetch Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn prefetchen(&self) -> super::vals::Ahbrxbuf2cr0Prefetchen {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Ahbrxbuf2cr0Prefetchen::from_bits(val as u8)
    }
    #[doc = "AHB Read Prefetch Enable"]
    #[inline(always)]
    pub const fn set_prefetchen(&mut self, val: super::vals::Ahbrxbuf2cr0Prefetchen) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ahbrxbuf2cr0 {
    #[inline(always)]
    fn default() -> Ahbrxbuf2cr0 {
        Ahbrxbuf2cr0(2147614784u64 as u32)
    }
}
impl core::fmt::Debug for Ahbrxbuf2cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbrxbuf2cr0")
            .field("bufsz", &self.bufsz())
            .field("mstrid", &self.mstrid())
            .field("priority", &self.priority())
            .field("regionen", &self.regionen())
            .field("prefetchen", &self.prefetchen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbrxbuf2cr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ahbrxbuf2cr0 {
            bufsz: u16,
            mstrid: u8,
            priority: u8,
            regionen: super::vals::Ahbrxbuf2cr0Regionen,
            prefetchen: super::vals::Ahbrxbuf2cr0Prefetchen,
        }
        let proxy = Ahbrxbuf2cr0 {
            bufsz: self.bufsz(),
            mstrid: self.mstrid(),
            priority: self.priority(),
            regionen: self.regionen(),
            prefetchen: self.prefetchen(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "AHB Receive Buffer 3 Control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbrxbuf3cr0(pub u32);
impl Ahbrxbuf3cr0 {
    #[doc = "AHB Receive Buffer Size"]
    #[must_use]
    #[inline(always)]
    pub const fn bufsz(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "AHB Receive Buffer Size"]
    #[inline(always)]
    pub const fn set_bufsz(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "AHB Controller ID"]
    #[must_use]
    #[inline(always)]
    pub const fn mstrid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "AHB Controller ID"]
    #[inline(always)]
    pub const fn set_mstrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "AHB Controller Read Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn priority(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "AHB Controller Read Priority"]
    #[inline(always)]
    pub const fn set_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "AHB Receive Buffer Address Region Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn regionen(&self) -> super::vals::Ahbrxbuf3cr0Regionen {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Ahbrxbuf3cr0Regionen::from_bits(val as u8)
    }
    #[doc = "AHB Receive Buffer Address Region Enable"]
    #[inline(always)]
    pub const fn set_regionen(&mut self, val: super::vals::Ahbrxbuf3cr0Regionen) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "AHB Read Prefetch Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn prefetchen(&self) -> super::vals::Ahbrxbuf3cr0Prefetchen {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Ahbrxbuf3cr0Prefetchen::from_bits(val as u8)
    }
    #[doc = "AHB Read Prefetch Enable"]
    #[inline(always)]
    pub const fn set_prefetchen(&mut self, val: super::vals::Ahbrxbuf3cr0Prefetchen) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ahbrxbuf3cr0 {
    #[inline(always)]
    fn default() -> Ahbrxbuf3cr0 {
        Ahbrxbuf3cr0(2147680320u64 as u32)
    }
}
impl core::fmt::Debug for Ahbrxbuf3cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbrxbuf3cr0")
            .field("bufsz", &self.bufsz())
            .field("mstrid", &self.mstrid())
            .field("priority", &self.priority())
            .field("regionen", &self.regionen())
            .field("prefetchen", &self.prefetchen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbrxbuf3cr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ahbrxbuf3cr0 {
            bufsz: u16,
            mstrid: u8,
            priority: u8,
            regionen: super::vals::Ahbrxbuf3cr0Regionen,
            prefetchen: super::vals::Ahbrxbuf3cr0Prefetchen,
        }
        let proxy = Ahbrxbuf3cr0 {
            bufsz: self.bufsz(),
            mstrid: self.mstrid(),
            priority: self.priority(),
            regionen: self.regionen(),
            prefetchen: self.prefetchen(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "AHB Receive Buffer 4 Control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbrxbuf4cr0(pub u32);
impl Ahbrxbuf4cr0 {
    #[doc = "AHB Receive Buffer Size"]
    #[must_use]
    #[inline(always)]
    pub const fn bufsz(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "AHB Receive Buffer Size"]
    #[inline(always)]
    pub const fn set_bufsz(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "AHB Controller ID"]
    #[must_use]
    #[inline(always)]
    pub const fn mstrid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "AHB Controller ID"]
    #[inline(always)]
    pub const fn set_mstrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "AHB Controller Read Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn priority(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "AHB Controller Read Priority"]
    #[inline(always)]
    pub const fn set_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "AHB Receive Buffer Address Region Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn regionen(&self) -> super::vals::Ahbrxbuf4cr0Regionen {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Ahbrxbuf4cr0Regionen::from_bits(val as u8)
    }
    #[doc = "AHB Receive Buffer Address Region Enable"]
    #[inline(always)]
    pub const fn set_regionen(&mut self, val: super::vals::Ahbrxbuf4cr0Regionen) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "AHB Read Prefetch Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn prefetchen(&self) -> super::vals::Ahbrxbuf4cr0Prefetchen {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Ahbrxbuf4cr0Prefetchen::from_bits(val as u8)
    }
    #[doc = "AHB Read Prefetch Enable"]
    #[inline(always)]
    pub const fn set_prefetchen(&mut self, val: super::vals::Ahbrxbuf4cr0Prefetchen) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ahbrxbuf4cr0 {
    #[inline(always)]
    fn default() -> Ahbrxbuf4cr0 {
        Ahbrxbuf4cr0(2147745856u64 as u32)
    }
}
impl core::fmt::Debug for Ahbrxbuf4cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbrxbuf4cr0")
            .field("bufsz", &self.bufsz())
            .field("mstrid", &self.mstrid())
            .field("priority", &self.priority())
            .field("regionen", &self.regionen())
            .field("prefetchen", &self.prefetchen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbrxbuf4cr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ahbrxbuf4cr0 {
            bufsz: u16,
            mstrid: u8,
            priority: u8,
            regionen: super::vals::Ahbrxbuf4cr0Regionen,
            prefetchen: super::vals::Ahbrxbuf4cr0Prefetchen,
        }
        let proxy = Ahbrxbuf4cr0 {
            bufsz: self.bufsz(),
            mstrid: self.mstrid(),
            priority: self.priority(),
            regionen: self.regionen(),
            prefetchen: self.prefetchen(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "AHB Receive Buffer 5 Control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbrxbuf5cr0(pub u32);
impl Ahbrxbuf5cr0 {
    #[doc = "AHB Receive Buffer Size"]
    #[must_use]
    #[inline(always)]
    pub const fn bufsz(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "AHB Receive Buffer Size"]
    #[inline(always)]
    pub const fn set_bufsz(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "AHB Controller ID"]
    #[must_use]
    #[inline(always)]
    pub const fn mstrid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "AHB Controller ID"]
    #[inline(always)]
    pub const fn set_mstrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "AHB Controller Read Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn priority(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "AHB Controller Read Priority"]
    #[inline(always)]
    pub const fn set_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "AHB Receive Buffer Address Region Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn regionen(&self) -> super::vals::Ahbrxbuf5cr0Regionen {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Ahbrxbuf5cr0Regionen::from_bits(val as u8)
    }
    #[doc = "AHB Receive Buffer Address Region Enable"]
    #[inline(always)]
    pub const fn set_regionen(&mut self, val: super::vals::Ahbrxbuf5cr0Regionen) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "AHB Read Prefetch Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn prefetchen(&self) -> super::vals::Ahbrxbuf5cr0Prefetchen {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Ahbrxbuf5cr0Prefetchen::from_bits(val as u8)
    }
    #[doc = "AHB Read Prefetch Enable"]
    #[inline(always)]
    pub const fn set_prefetchen(&mut self, val: super::vals::Ahbrxbuf5cr0Prefetchen) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ahbrxbuf5cr0 {
    #[inline(always)]
    fn default() -> Ahbrxbuf5cr0 {
        Ahbrxbuf5cr0(2147811392u64 as u32)
    }
}
impl core::fmt::Debug for Ahbrxbuf5cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbrxbuf5cr0")
            .field("bufsz", &self.bufsz())
            .field("mstrid", &self.mstrid())
            .field("priority", &self.priority())
            .field("regionen", &self.regionen())
            .field("prefetchen", &self.prefetchen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbrxbuf5cr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ahbrxbuf5cr0 {
            bufsz: u16,
            mstrid: u8,
            priority: u8,
            regionen: super::vals::Ahbrxbuf5cr0Regionen,
            prefetchen: super::vals::Ahbrxbuf5cr0Prefetchen,
        }
        let proxy = Ahbrxbuf5cr0 {
            bufsz: self.bufsz(),
            mstrid: self.mstrid(),
            priority: self.priority(),
            regionen: self.regionen(),
            prefetchen: self.prefetchen(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "AHB Receive Buffer 6 Control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbrxbuf6cr0(pub u32);
impl Ahbrxbuf6cr0 {
    #[doc = "AHB Receive Buffer Size"]
    #[must_use]
    #[inline(always)]
    pub const fn bufsz(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "AHB Receive Buffer Size"]
    #[inline(always)]
    pub const fn set_bufsz(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "AHB Controller ID"]
    #[must_use]
    #[inline(always)]
    pub const fn mstrid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "AHB Controller ID"]
    #[inline(always)]
    pub const fn set_mstrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "AHB Controller Read Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn priority(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "AHB Controller Read Priority"]
    #[inline(always)]
    pub const fn set_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "AHB Receive Buffer Address Region Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn regionen(&self) -> super::vals::Ahbrxbuf6cr0Regionen {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Ahbrxbuf6cr0Regionen::from_bits(val as u8)
    }
    #[doc = "AHB Receive Buffer Address Region Enable"]
    #[inline(always)]
    pub const fn set_regionen(&mut self, val: super::vals::Ahbrxbuf6cr0Regionen) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "AHB Read Prefetch Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn prefetchen(&self) -> super::vals::Ahbrxbuf6cr0Prefetchen {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Ahbrxbuf6cr0Prefetchen::from_bits(val as u8)
    }
    #[doc = "AHB Read Prefetch Enable"]
    #[inline(always)]
    pub const fn set_prefetchen(&mut self, val: super::vals::Ahbrxbuf6cr0Prefetchen) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ahbrxbuf6cr0 {
    #[inline(always)]
    fn default() -> Ahbrxbuf6cr0 {
        Ahbrxbuf6cr0(2147876928u64 as u32)
    }
}
impl core::fmt::Debug for Ahbrxbuf6cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbrxbuf6cr0")
            .field("bufsz", &self.bufsz())
            .field("mstrid", &self.mstrid())
            .field("priority", &self.priority())
            .field("regionen", &self.regionen())
            .field("prefetchen", &self.prefetchen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbrxbuf6cr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ahbrxbuf6cr0 {
            bufsz: u16,
            mstrid: u8,
            priority: u8,
            regionen: super::vals::Ahbrxbuf6cr0Regionen,
            prefetchen: super::vals::Ahbrxbuf6cr0Prefetchen,
        }
        let proxy = Ahbrxbuf6cr0 {
            bufsz: self.bufsz(),
            mstrid: self.mstrid(),
            priority: self.priority(),
            regionen: self.regionen(),
            prefetchen: self.prefetchen(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "AHB Receive Buffer 7 Control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbrxbuf7cr0(pub u32);
impl Ahbrxbuf7cr0 {
    #[doc = "AHB Receive Buffer Size"]
    #[must_use]
    #[inline(always)]
    pub const fn bufsz(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "AHB Receive Buffer Size"]
    #[inline(always)]
    pub const fn set_bufsz(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "AHB Controller ID"]
    #[must_use]
    #[inline(always)]
    pub const fn mstrid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "AHB Controller ID"]
    #[inline(always)]
    pub const fn set_mstrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "AHB Controller Read Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn priority(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "AHB Controller Read Priority"]
    #[inline(always)]
    pub const fn set_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "AHB Receive Buffer Address Region Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn regionen(&self) -> super::vals::Ahbrxbuf7cr0Regionen {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Ahbrxbuf7cr0Regionen::from_bits(val as u8)
    }
    #[doc = "AHB Receive Buffer Address Region Enable"]
    #[inline(always)]
    pub const fn set_regionen(&mut self, val: super::vals::Ahbrxbuf7cr0Regionen) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "AHB Read Prefetch Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn prefetchen(&self) -> super::vals::Ahbrxbuf7cr0Prefetchen {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Ahbrxbuf7cr0Prefetchen::from_bits(val as u8)
    }
    #[doc = "AHB Read Prefetch Enable"]
    #[inline(always)]
    pub const fn set_prefetchen(&mut self, val: super::vals::Ahbrxbuf7cr0Prefetchen) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ahbrxbuf7cr0 {
    #[inline(always)]
    fn default() -> Ahbrxbuf7cr0 {
        Ahbrxbuf7cr0(2147942464u64 as u32)
    }
}
impl core::fmt::Debug for Ahbrxbuf7cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbrxbuf7cr0")
            .field("bufsz", &self.bufsz())
            .field("mstrid", &self.mstrid())
            .field("priority", &self.priority())
            .field("regionen", &self.regionen())
            .field("prefetchen", &self.prefetchen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbrxbuf7cr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ahbrxbuf7cr0 {
            bufsz: u16,
            mstrid: u8,
            priority: u8,
            regionen: super::vals::Ahbrxbuf7cr0Regionen,
            prefetchen: super::vals::Ahbrxbuf7cr0Prefetchen,
        }
        let proxy = Ahbrxbuf7cr0 {
            bufsz: self.bufsz(),
            mstrid: self.mstrid(),
            priority: self.priority(),
            regionen: self.regionen(),
            prefetchen: self.prefetchen(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "AHB Suspend Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbspndsts(pub u32);
impl Ahbspndsts {
    #[doc = "Active AHB Read Prefetch Suspended"]
    #[must_use]
    #[inline(always)]
    pub const fn active(&self) -> super::vals::Active {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Active::from_bits(val as u8)
    }
    #[doc = "Active AHB Read Prefetch Suspended"]
    #[inline(always)]
    pub const fn set_active(&mut self, val: super::vals::Active) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "AHB Receive Buffer ID for Suspended Command Sequence"]
    #[must_use]
    #[inline(always)]
    pub const fn bufid(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "AHB Receive Buffer ID for Suspended Command Sequence"]
    #[inline(always)]
    pub const fn set_bufid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
    }
    #[doc = "Data Left"]
    #[must_use]
    #[inline(always)]
    pub const fn datlft(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Data Left"]
    #[inline(always)]
    pub const fn set_datlft(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Ahbspndsts {
    #[inline(always)]
    fn default() -> Ahbspndsts {
        Ahbspndsts(0u64 as u32)
    }
}
impl core::fmt::Debug for Ahbspndsts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbspndsts")
            .field("active", &self.active())
            .field("bufid", &self.bufid())
            .field("datlft", &self.datlft())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbspndsts {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ahbspndsts {
            active: super::vals::Active,
            bufid: u8,
            datlft: u16,
        }
        let proxy = Ahbspndsts {
            active: self.active(),
            bufid: self.bufid(),
            datlft: self.datlft(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "DLL Control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dllcr(pub u32);
impl Dllcr {
    #[doc = "DLL Calibration Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dllen(&self) -> super::vals::Dllen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Dllen::from_bits(val as u8)
    }
    #[doc = "DLL Calibration Enable"]
    #[inline(always)]
    pub const fn set_dllen(&mut self, val: super::vals::Dllen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "DLL reset"]
    #[must_use]
    #[inline(always)]
    pub const fn dllreset(&self) -> super::vals::Dllreset {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Dllreset::from_bits(val as u8)
    }
    #[doc = "DLL reset"]
    #[inline(always)]
    pub const fn set_dllreset(&mut self, val: super::vals::Dllreset) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Target Delay Line"]
    #[must_use]
    #[inline(always)]
    pub const fn slvdlytarget(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x0f;
        val as u8
    }
    #[doc = "Target Delay Line"]
    #[inline(always)]
    pub const fn set_slvdlytarget(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
    }
    #[doc = "Target Clock Delay Line Override Value Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ovrden(&self) -> super::vals::Ovrden {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Ovrden::from_bits(val as u8)
    }
    #[doc = "Target Clock Delay Line Override Value Enable"]
    #[inline(always)]
    pub const fn set_ovrden(&mut self, val: super::vals::Ovrden) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Target Clock Delay Line Override Value"]
    #[must_use]
    #[inline(always)]
    pub const fn ovrdval(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x3f;
        val as u8
    }
    #[doc = "Target Clock Delay Line Override Value"]
    #[inline(always)]
    pub const fn set_ovrdval(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 9usize)) | (((val as u32) & 0x3f) << 9usize);
    }
    #[doc = "Reference Clock Delay Line Phase Adjust Gap. REFPHASEGAP setting of 2h is recommended if DLLEN is set."]
    #[must_use]
    #[inline(always)]
    pub const fn refphasegap(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x03;
        val as u8
    }
    #[doc = "Reference Clock Delay Line Phase Adjust Gap. REFPHASEGAP setting of 2h is recommended if DLLEN is set."]
    #[inline(always)]
    pub const fn set_refphasegap(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 15usize)) | (((val as u32) & 0x03) << 15usize);
    }
}
impl Default for Dllcr {
    #[inline(always)]
    fn default() -> Dllcr {
        Dllcr(256u64 as u32)
    }
}
impl core::fmt::Debug for Dllcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dllcr")
            .field("dllen", &self.dllen())
            .field("dllreset", &self.dllreset())
            .field("slvdlytarget", &self.slvdlytarget())
            .field("ovrden", &self.ovrden())
            .field("ovrdval", &self.ovrdval())
            .field("refphasegap", &self.refphasegap())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dllcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Dllcr {
            dllen: super::vals::Dllen,
            dllreset: super::vals::Dllreset,
            slvdlytarget: u8,
            ovrden: super::vals::Ovrden,
            ovrdval: u8,
            refphasegap: u8,
        }
        let proxy = Dllcr {
            dllen: self.dllen(),
            dllreset: self.dllreset(),
            slvdlytarget: self.slvdlytarget(),
            ovrden: self.ovrden(),
            ovrdval: self.ovrdval(),
            refphasegap: self.refphasegap(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Flash Control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flsha1cr0(pub u32);
impl Flsha1cr0 {
    #[doc = "Flash Size in KB"]
    #[must_use]
    #[inline(always)]
    pub const fn flshsz(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "Flash Size in KB"]
    #[inline(always)]
    pub const fn set_flshsz(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
    }
    #[doc = "AHB Address Shift Function control"]
    #[must_use]
    #[inline(always)]
    pub const fn addrshift(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Address Shift Function control"]
    #[inline(always)]
    pub const fn set_addrshift(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for Flsha1cr0 {
    #[inline(always)]
    fn default() -> Flsha1cr0 {
        Flsha1cr0(0u64 as u32)
    }
}
impl core::fmt::Debug for Flsha1cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flsha1cr0")
            .field("flshsz", &self.flshsz())
            .field("addrshift", &self.addrshift())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flsha1cr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flsha1cr0 {
            flshsz: u32,
            addrshift: bool,
        }
        let proxy = Flsha1cr0 {
            flshsz: self.flshsz(),
            addrshift: self.addrshift(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Flash Control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flsha2cr0(pub u32);
impl Flsha2cr0 {
    #[doc = "Flash Size in KB"]
    #[must_use]
    #[inline(always)]
    pub const fn flshsz(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "Flash Size in KB"]
    #[inline(always)]
    pub const fn set_flshsz(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
    }
    #[doc = "AHB Address Shift Function control"]
    #[must_use]
    #[inline(always)]
    pub const fn addrshift(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Address Shift Function control"]
    #[inline(always)]
    pub const fn set_addrshift(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for Flsha2cr0 {
    #[inline(always)]
    fn default() -> Flsha2cr0 {
        Flsha2cr0(0u64 as u32)
    }
}
impl core::fmt::Debug for Flsha2cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flsha2cr0")
            .field("flshsz", &self.flshsz())
            .field("addrshift", &self.addrshift())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flsha2cr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flsha2cr0 {
            flshsz: u32,
            addrshift: bool,
        }
        let proxy = Flsha2cr0 {
            flshsz: self.flshsz(),
            addrshift: self.addrshift(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Flash Control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flshb1cr0(pub u32);
impl Flshb1cr0 {
    #[doc = "Flash Size in KB"]
    #[must_use]
    #[inline(always)]
    pub const fn flshsz(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "Flash Size in KB"]
    #[inline(always)]
    pub const fn set_flshsz(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
    }
    #[doc = "AHB Address Shift Function control"]
    #[must_use]
    #[inline(always)]
    pub const fn addrshift(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Address Shift Function control"]
    #[inline(always)]
    pub const fn set_addrshift(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for Flshb1cr0 {
    #[inline(always)]
    fn default() -> Flshb1cr0 {
        Flshb1cr0(0u64 as u32)
    }
}
impl core::fmt::Debug for Flshb1cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flshb1cr0")
            .field("flshsz", &self.flshsz())
            .field("addrshift", &self.addrshift())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flshb1cr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flshb1cr0 {
            flshsz: u32,
            addrshift: bool,
        }
        let proxy = Flshb1cr0 {
            flshsz: self.flshsz(),
            addrshift: self.addrshift(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Flash Control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flshb2cr0(pub u32);
impl Flshb2cr0 {
    #[doc = "Flash Size in KB"]
    #[must_use]
    #[inline(always)]
    pub const fn flshsz(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "Flash Size in KB"]
    #[inline(always)]
    pub const fn set_flshsz(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
    }
    #[doc = "AHB Address Shift Function control"]
    #[must_use]
    #[inline(always)]
    pub const fn addrshift(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Address Shift Function control"]
    #[inline(always)]
    pub const fn set_addrshift(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for Flshb2cr0 {
    #[inline(always)]
    fn default() -> Flshb2cr0 {
        Flshb2cr0(0u64 as u32)
    }
}
impl core::fmt::Debug for Flshb2cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flshb2cr0")
            .field("flshsz", &self.flshsz())
            .field("addrshift", &self.addrshift())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flshb2cr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flshb2cr0 {
            flshsz: u32,
            addrshift: bool,
        }
        let proxy = Flshb2cr0 {
            flshsz: self.flshsz(),
            addrshift: self.addrshift(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Flash Control 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flshcr1(pub u32);
impl Flshcr1 {
    #[doc = "Serial Flash CS Setup Time"]
    #[must_use]
    #[inline(always)]
    pub const fn tcss(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Serial Flash CS Setup Time"]
    #[inline(always)]
    pub const fn set_tcss(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Serial Flash CS Hold Time"]
    #[must_use]
    #[inline(always)]
    pub const fn tcsh(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x1f;
        val as u8
    }
    #[doc = "Serial Flash CS Hold Time"]
    #[inline(always)]
    pub const fn set_tcsh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
    }
    #[doc = "Word-Addressable"]
    #[must_use]
    #[inline(always)]
    pub const fn wa(&self) -> super::vals::Wa {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Wa::from_bits(val as u8)
    }
    #[doc = "Word-Addressable"]
    #[inline(always)]
    pub const fn set_wa(&mut self, val: super::vals::Wa) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Column Address Size"]
    #[must_use]
    #[inline(always)]
    pub const fn cas(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x0f;
        val as u8
    }
    #[doc = "Column Address Size"]
    #[inline(always)]
    pub const fn set_cas(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
    }
    #[doc = "Chip Select Interval Unit"]
    #[must_use]
    #[inline(always)]
    pub const fn csintervalunit(&self) -> super::vals::Csintervalunit {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Csintervalunit::from_bits(val as u8)
    }
    #[doc = "Chip Select Interval Unit"]
    #[inline(always)]
    pub const fn set_csintervalunit(&mut self, val: super::vals::Csintervalunit) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Chip Select Interval"]
    #[must_use]
    #[inline(always)]
    pub const fn csinterval(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Chip Select Interval"]
    #[inline(always)]
    pub const fn set_csinterval(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Flshcr1 {
    #[inline(always)]
    fn default() -> Flshcr1 {
        Flshcr1(99u64 as u32)
    }
}
impl core::fmt::Debug for Flshcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flshcr1")
            .field("tcss", &self.tcss())
            .field("tcsh", &self.tcsh())
            .field("wa", &self.wa())
            .field("cas", &self.cas())
            .field("csintervalunit", &self.csintervalunit())
            .field("csinterval", &self.csinterval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flshcr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flshcr1 {
            tcss: u8,
            tcsh: u8,
            wa: super::vals::Wa,
            cas: u8,
            csintervalunit: super::vals::Csintervalunit,
            csinterval: u16,
        }
        let proxy = Flshcr1 {
            tcss: self.tcss(),
            tcsh: self.tcsh(),
            wa: self.wa(),
            cas: self.cas(),
            csintervalunit: self.csintervalunit(),
            csinterval: self.csinterval(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Flash Control 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flshcr2(pub u32);
impl Flshcr2 {
    #[doc = "Sequence Index for AHB Read-Triggered Command in LUT"]
    #[must_use]
    #[inline(always)]
    pub const fn ardseqid(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Sequence Index for AHB Read-Triggered Command in LUT"]
    #[inline(always)]
    pub const fn set_ardseqid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Sequence Number for AHB Read-Triggered Command"]
    #[must_use]
    #[inline(always)]
    pub const fn ardseqnum(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "Sequence Number for AHB Read-Triggered Command"]
    #[inline(always)]
    pub const fn set_ardseqnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
    #[doc = "Sequence Index for AHB Write-Triggered Command"]
    #[must_use]
    #[inline(always)]
    pub const fn awrseqid(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Sequence Index for AHB Write-Triggered Command"]
    #[inline(always)]
    pub const fn set_awrseqid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Sequence Number for AHB Write-Triggered Command"]
    #[must_use]
    #[inline(always)]
    pub const fn awrseqnum(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Sequence Number for AHB Write-Triggered Command"]
    #[inline(always)]
    pub const fn set_awrseqnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "AHB Write Wait"]
    #[must_use]
    #[inline(always)]
    pub const fn awrwait(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "AHB Write Wait"]
    #[inline(always)]
    pub const fn set_awrwait(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
    #[doc = "AWRWAIT Unit"]
    #[must_use]
    #[inline(always)]
    pub const fn awrwaitunit(&self) -> super::vals::Awrwaitunit {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Awrwaitunit::from_bits(val as u8)
    }
    #[doc = "AWRWAIT Unit"]
    #[inline(always)]
    pub const fn set_awrwaitunit(&mut self, val: super::vals::Awrwaitunit) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
    #[doc = "Clear Instruction Pointer"]
    #[must_use]
    #[inline(always)]
    pub const fn clrinstrptr(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Clear Instruction Pointer"]
    #[inline(always)]
    pub const fn set_clrinstrptr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Flshcr2 {
    #[inline(always)]
    fn default() -> Flshcr2 {
        Flshcr2(0u64 as u32)
    }
}
impl core::fmt::Debug for Flshcr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flshcr2")
            .field("ardseqid", &self.ardseqid())
            .field("ardseqnum", &self.ardseqnum())
            .field("awrseqid", &self.awrseqid())
            .field("awrseqnum", &self.awrseqnum())
            .field("awrwait", &self.awrwait())
            .field("awrwaitunit", &self.awrwaitunit())
            .field("clrinstrptr", &self.clrinstrptr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flshcr2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flshcr2 {
            ardseqid: u8,
            ardseqnum: u8,
            awrseqid: u8,
            awrseqnum: u8,
            awrwait: u16,
            awrwaitunit: super::vals::Awrwaitunit,
            clrinstrptr: bool,
        }
        let proxy = Flshcr2 {
            ardseqid: self.ardseqid(),
            ardseqnum: self.ardseqnum(),
            awrseqid: self.awrseqid(),
            awrseqnum: self.awrseqnum(),
            awrwait: self.awrwait(),
            awrwaitunit: self.awrwaitunit(),
            clrinstrptr: self.clrinstrptr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Flash Control 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flshcr4(pub u32);
impl Flshcr4 {
    #[doc = "Write Mask Option 1"]
    #[must_use]
    #[inline(always)]
    pub const fn wmopt1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write Mask Option 1"]
    #[inline(always)]
    pub const fn set_wmopt1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write Mask Option 2"]
    #[must_use]
    #[inline(always)]
    pub const fn wmopt2(&self) -> super::vals::Wmopt2 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Wmopt2::from_bits(val as u8)
    }
    #[doc = "Write Mask Option 2"]
    #[inline(always)]
    pub const fn set_wmopt2(&mut self, val: super::vals::Wmopt2) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Write Mask Enable for Port A"]
    #[must_use]
    #[inline(always)]
    pub const fn wmena(&self) -> super::vals::Wmena {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Wmena::from_bits(val as u8)
    }
    #[doc = "Write Mask Enable for Port A"]
    #[inline(always)]
    pub const fn set_wmena(&mut self, val: super::vals::Wmena) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Write Mask Enable for Port B"]
    #[must_use]
    #[inline(always)]
    pub const fn wmenb(&self) -> super::vals::Wmenb {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Wmenb::from_bits(val as u8)
    }
    #[doc = "Write Mask Enable for Port B"]
    #[inline(always)]
    pub const fn set_wmenb(&mut self, val: super::vals::Wmenb) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for Flshcr4 {
    #[inline(always)]
    fn default() -> Flshcr4 {
        Flshcr4(195u64 as u32)
    }
}
impl core::fmt::Debug for Flshcr4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flshcr4")
            .field("wmopt1", &self.wmopt1())
            .field("wmopt2", &self.wmopt2())
            .field("wmena", &self.wmena())
            .field("wmenb", &self.wmenb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flshcr4 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flshcr4 {
            wmopt1: bool,
            wmopt2: super::vals::Wmopt2,
            wmena: super::vals::Wmena,
            wmenb: super::vals::Wmenb,
        }
        let proxy = Flshcr4 {
            wmopt1: self.wmopt1(),
            wmopt2: self.wmopt2(),
            wmena: self.wmena(),
            wmenb: self.wmenb(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "IP-Triggered Command Sequences Execution Finished Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ipcmddoneen(&self) -> super::vals::Ipcmddoneen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ipcmddoneen::from_bits(val as u8)
    }
    #[doc = "IP-Triggered Command Sequences Execution Finished Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ipcmddoneen(&mut self, val: super::vals::Ipcmddoneen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "IP-Triggered Command Sequences Grant Timeout Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ipcmdgeen(&self) -> super::vals::Ipcmdgeen {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ipcmdgeen::from_bits(val as u8)
    }
    #[doc = "IP-Triggered Command Sequences Grant Timeout Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ipcmdgeen(&mut self, val: super::vals::Ipcmdgeen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "AHB-Triggered Command Sequences Grant Timeout Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ahbcmdgeen(&self) -> super::vals::Ahbcmdgeen {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Ahbcmdgeen::from_bits(val as u8)
    }
    #[doc = "AHB-Triggered Command Sequences Grant Timeout Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ahbcmdgeen(&mut self, val: super::vals::Ahbcmdgeen) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "IP-Triggered Command Sequences Error Detected Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ipcmderren(&self) -> super::vals::Ipcmderren {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Ipcmderren::from_bits(val as u8)
    }
    #[doc = "IP-Triggered Command Sequences Error Detected Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ipcmderren(&mut self, val: super::vals::Ipcmderren) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "AHB-Triggered Command Sequences Error Detected Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ahbcmderren(&self) -> super::vals::Ahbcmderren {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Ahbcmderren::from_bits(val as u8)
    }
    #[doc = "AHB-Triggered Command Sequences Error Detected Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ahbcmderren(&mut self, val: super::vals::Ahbcmderren) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "IP Receive FIFO Watermark Available Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn iprxwaen(&self) -> super::vals::Iprxwaen {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Iprxwaen::from_bits(val as u8)
    }
    #[doc = "IP Receive FIFO Watermark Available Interrupt Enable"]
    #[inline(always)]
    pub const fn set_iprxwaen(&mut self, val: super::vals::Iprxwaen) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "IP Transmit FIFO Watermark Empty Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn iptxween(&self) -> super::vals::Iptxween {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Iptxween::from_bits(val as u8)
    }
    #[doc = "IP Transmit FIFO Watermark Empty Interrupt Enable"]
    #[inline(always)]
    pub const fn set_iptxween(&mut self, val: super::vals::Iptxween) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "SCLK Stopped By Read Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sckstopbyrden(&self) -> super::vals::Sckstopbyrden {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Sckstopbyrden::from_bits(val as u8)
    }
    #[doc = "SCLK Stopped By Read Interrupt Enable"]
    #[inline(always)]
    pub const fn set_sckstopbyrden(&mut self, val: super::vals::Sckstopbyrden) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "SCLK Stopped By Write Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sckstopbywren(&self) -> super::vals::Sckstopbywren {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Sckstopbywren::from_bits(val as u8)
    }
    #[doc = "SCLK Stopped By Write Interrupt Enable"]
    #[inline(always)]
    pub const fn set_sckstopbywren(&mut self, val: super::vals::Sckstopbywren) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "AHB Bus Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ahbbuserroren(&self) -> super::vals::Ahbbuserroren {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Ahbbuserroren::from_bits(val as u8)
    }
    #[doc = "AHB Bus Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ahbbuserroren(&mut self, val: super::vals::Ahbbuserroren) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Sequence execution Timeout Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn seqtimeouten(&self) -> super::vals::Seqtimeouten {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Seqtimeouten::from_bits(val as u8)
    }
    #[doc = "Sequence execution Timeout Interrupt Enable"]
    #[inline(always)]
    pub const fn set_seqtimeouten(&mut self, val: super::vals::Seqtimeouten) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "OTFAD Key Blob Processing Done Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn keydoneen(&self) -> super::vals::Keydoneen {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Keydoneen::from_bits(val as u8)
    }
    #[doc = "OTFAD Key Blob Processing Done Interrupt Enable"]
    #[inline(always)]
    pub const fn set_keydoneen(&mut self, val: super::vals::Keydoneen) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "OTFAD Key Blob Processing Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn keyerroren(&self) -> super::vals::Keyerroren {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Keyerroren::from_bits(val as u8)
    }
    #[doc = "OTFAD Key Blob Processing Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_keyerroren(&mut self, val: super::vals::Keyerroren) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0u64 as u32)
    }
}
impl core::fmt::Debug for Inten {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Inten")
            .field("ipcmddoneen", &self.ipcmddoneen())
            .field("ipcmdgeen", &self.ipcmdgeen())
            .field("ahbcmdgeen", &self.ahbcmdgeen())
            .field("ipcmderren", &self.ipcmderren())
            .field("ahbcmderren", &self.ahbcmderren())
            .field("iprxwaen", &self.iprxwaen())
            .field("iptxween", &self.iptxween())
            .field("sckstopbyrden", &self.sckstopbyrden())
            .field("sckstopbywren", &self.sckstopbywren())
            .field("ahbbuserroren", &self.ahbbuserroren())
            .field("seqtimeouten", &self.seqtimeouten())
            .field("keydoneen", &self.keydoneen())
            .field("keyerroren", &self.keyerroren())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Inten {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Inten {
            ipcmddoneen: super::vals::Ipcmddoneen,
            ipcmdgeen: super::vals::Ipcmdgeen,
            ahbcmdgeen: super::vals::Ahbcmdgeen,
            ipcmderren: super::vals::Ipcmderren,
            ahbcmderren: super::vals::Ahbcmderren,
            iprxwaen: super::vals::Iprxwaen,
            iptxween: super::vals::Iptxween,
            sckstopbyrden: super::vals::Sckstopbyrden,
            sckstopbywren: super::vals::Sckstopbywren,
            ahbbuserroren: super::vals::Ahbbuserroren,
            seqtimeouten: super::vals::Seqtimeouten,
            keydoneen: super::vals::Keydoneen,
            keyerroren: super::vals::Keyerroren,
        }
        let proxy = Inten {
            ipcmddoneen: self.ipcmddoneen(),
            ipcmdgeen: self.ipcmdgeen(),
            ahbcmdgeen: self.ahbcmdgeen(),
            ipcmderren: self.ipcmderren(),
            ahbcmderren: self.ahbcmderren(),
            iprxwaen: self.iprxwaen(),
            iptxween: self.iptxween(),
            sckstopbyrden: self.sckstopbyrden(),
            sckstopbywren: self.sckstopbywren(),
            ahbbuserroren: self.ahbbuserroren(),
            seqtimeouten: self.seqtimeouten(),
            keydoneen: self.keydoneen(),
            keyerroren: self.keyerroren(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intr(pub u32);
impl Intr {
    #[doc = "IP-Triggered Command Sequences Execution Finished"]
    #[must_use]
    #[inline(always)]
    pub const fn ipcmddone(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "IP-Triggered Command Sequences Execution Finished"]
    #[inline(always)]
    pub const fn set_ipcmddone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "IP-Triggered Command Sequences Grant Timeout"]
    #[must_use]
    #[inline(always)]
    pub const fn ipcmdge(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "IP-Triggered Command Sequences Grant Timeout"]
    #[inline(always)]
    pub const fn set_ipcmdge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "AHB-Triggered Command Sequences Grant Timeout"]
    #[must_use]
    #[inline(always)]
    pub const fn ahbcmdge(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "AHB-Triggered Command Sequences Grant Timeout"]
    #[inline(always)]
    pub const fn set_ahbcmdge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "IP-Triggered Command Sequences Error"]
    #[must_use]
    #[inline(always)]
    pub const fn ipcmderr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "IP-Triggered Command Sequences Error"]
    #[inline(always)]
    pub const fn set_ipcmderr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "AHB-Triggered Command Sequences Error"]
    #[must_use]
    #[inline(always)]
    pub const fn ahbcmderr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "AHB-Triggered Command Sequences Error"]
    #[inline(always)]
    pub const fn set_ahbcmderr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "IP Receive FIFO Watermark Available"]
    #[must_use]
    #[inline(always)]
    pub const fn iprxwa(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "IP Receive FIFO Watermark Available"]
    #[inline(always)]
    pub const fn set_iprxwa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "IP Transmit FIFO Watermark Empty"]
    #[must_use]
    #[inline(always)]
    pub const fn iptxwe(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "IP Transmit FIFO Watermark Empty"]
    #[inline(always)]
    pub const fn set_iptxwe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SCLK Stopped Due To Full Receive FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn sckstopbyrd(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SCLK Stopped Due To Full Receive FIFO"]
    #[inline(always)]
    pub const fn set_sckstopbyrd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SCLK Stopped Due To Empty Transmit FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn sckstopbywr(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SCLK Stopped Due To Empty Transmit FIFO"]
    #[inline(always)]
    pub const fn set_sckstopbywr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "AHB Bus Error"]
    #[must_use]
    #[inline(always)]
    pub const fn ahbbuserror(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Bus Error"]
    #[inline(always)]
    pub const fn set_ahbbuserror(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Sequence Execution Timeout"]
    #[must_use]
    #[inline(always)]
    pub const fn seqtimeout(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Sequence Execution Timeout"]
    #[inline(always)]
    pub const fn set_seqtimeout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "OTFAD key blob processing done interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn keydone(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "OTFAD key blob processing done interrupt."]
    #[inline(always)]
    pub const fn set_keydone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "OTFAD Key Blob Processing Error"]
    #[must_use]
    #[inline(always)]
    pub const fn keyerror(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "OTFAD Key Blob Processing Error"]
    #[inline(always)]
    pub const fn set_keyerror(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
}
impl Default for Intr {
    #[inline(always)]
    fn default() -> Intr {
        Intr(0u64 as u32)
    }
}
impl core::fmt::Debug for Intr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intr")
            .field("ipcmddone", &self.ipcmddone())
            .field("ipcmdge", &self.ipcmdge())
            .field("ahbcmdge", &self.ahbcmdge())
            .field("ipcmderr", &self.ipcmderr())
            .field("ahbcmderr", &self.ahbcmderr())
            .field("iprxwa", &self.iprxwa())
            .field("iptxwe", &self.iptxwe())
            .field("sckstopbyrd", &self.sckstopbyrd())
            .field("sckstopbywr", &self.sckstopbywr())
            .field("ahbbuserror", &self.ahbbuserror())
            .field("seqtimeout", &self.seqtimeout())
            .field("keydone", &self.keydone())
            .field("keyerror", &self.keyerror())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Intr {
            ipcmddone: bool,
            ipcmdge: bool,
            ahbcmdge: bool,
            ipcmderr: bool,
            ahbcmderr: bool,
            iprxwa: bool,
            iptxwe: bool,
            sckstopbyrd: bool,
            sckstopbywr: bool,
            ahbbuserror: bool,
            seqtimeout: bool,
            keydone: bool,
            keyerror: bool,
        }
        let proxy = Intr {
            ipcmddone: self.ipcmddone(),
            ipcmdge: self.ipcmdge(),
            ahbcmdge: self.ahbcmdge(),
            ipcmderr: self.ipcmderr(),
            ahbcmderr: self.ahbcmderr(),
            iprxwa: self.iprxwa(),
            iptxwe: self.iptxwe(),
            sckstopbyrd: self.sckstopbyrd(),
            sckstopbywr: self.sckstopbywr(),
            ahbbuserror: self.ahbbuserror(),
            seqtimeout: self.seqtimeout(),
            keydone: self.keydone(),
            keyerror: self.keyerror(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "IP Command"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipcmd(pub u32);
impl Ipcmd {
    #[doc = "Command Trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn trg(&self) -> super::vals::Trg {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Trg::from_bits(val as u8)
    }
    #[doc = "Command Trigger"]
    #[inline(always)]
    pub const fn set_trg(&mut self, val: super::vals::Trg) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Ipcmd {
    #[inline(always)]
    fn default() -> Ipcmd {
        Ipcmd(0u64 as u32)
    }
}
impl core::fmt::Debug for Ipcmd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipcmd").field("trg", &self.trg()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipcmd {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ipcmd {
            trg: super::vals::Trg,
        }
        let proxy = Ipcmd { trg: self.trg() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "IP Control 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipcr1(pub u32);
impl Ipcr1 {
    #[doc = "Flash Read/Program Data Size (in bytes) for IP command."]
    #[must_use]
    #[inline(always)]
    pub const fn idatsz(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Flash Read/Program Data Size (in bytes) for IP command."]
    #[inline(always)]
    pub const fn set_idatsz(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Sequence Index in LUT for IP command."]
    #[must_use]
    #[inline(always)]
    pub const fn iseqid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Sequence Index in LUT for IP command."]
    #[inline(always)]
    pub const fn set_iseqid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Sequence Number for IP command: ISEQNUM+1."]
    #[must_use]
    #[inline(always)]
    pub const fn iseqnum(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Sequence Number for IP command: ISEQNUM+1."]
    #[inline(always)]
    pub const fn set_iseqnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "Parallel Mode Enable for IP Commands"]
    #[must_use]
    #[inline(always)]
    pub const fn iparen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Parallel Mode Enable for IP Commands"]
    #[inline(always)]
    pub const fn set_iparen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ipcr1 {
    #[inline(always)]
    fn default() -> Ipcr1 {
        Ipcr1(0u64 as u32)
    }
}
impl core::fmt::Debug for Ipcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipcr1")
            .field("idatsz", &self.idatsz())
            .field("iseqid", &self.iseqid())
            .field("iseqnum", &self.iseqnum())
            .field("iparen", &self.iparen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipcr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ipcr1 {
            idatsz: u16,
            iseqid: u8,
            iseqnum: u8,
            iparen: bool,
        }
        let proxy = Ipcr1 {
            idatsz: self.idatsz(),
            iseqid: self.iseqid(),
            iseqnum: self.iseqnum(),
            iparen: self.iparen(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "IP Receive FIFO Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iprxfcr(pub u32);
impl Iprxfcr {
    #[doc = "Clear IP Receive FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn clriprxf(&self) -> super::vals::Clriprxf {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Clriprxf::from_bits(val as u8)
    }
    #[doc = "Clear IP Receive FIFO"]
    #[inline(always)]
    pub const fn set_clriprxf(&mut self, val: super::vals::Clriprxf) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "IP Receive FIFO Reading by DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rxdmaen(&self) -> super::vals::Rxdmaen {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Rxdmaen::from_bits(val as u8)
    }
    #[doc = "IP Receive FIFO Reading by DMA Enable"]
    #[inline(always)]
    pub const fn set_rxdmaen(&mut self, val: super::vals::Rxdmaen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "IP Receive FIFO Watermark Level"]
    #[must_use]
    #[inline(always)]
    pub const fn rxwmrk(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x1f;
        val as u8
    }
    #[doc = "IP Receive FIFO Watermark Level"]
    #[inline(always)]
    pub const fn set_rxwmrk(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 2usize)) | (((val as u32) & 0x1f) << 2usize);
    }
}
impl Default for Iprxfcr {
    #[inline(always)]
    fn default() -> Iprxfcr {
        Iprxfcr(0u64 as u32)
    }
}
impl core::fmt::Debug for Iprxfcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iprxfcr")
            .field("clriprxf", &self.clriprxf())
            .field("rxdmaen", &self.rxdmaen())
            .field("rxwmrk", &self.rxwmrk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iprxfcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Iprxfcr {
            clriprxf: super::vals::Clriprxf,
            rxdmaen: super::vals::Rxdmaen,
            rxwmrk: u8,
        }
        let proxy = Iprxfcr {
            clriprxf: self.clriprxf(),
            rxdmaen: self.rxdmaen(),
            rxwmrk: self.rxwmrk(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "IP Receive FIFO Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iprxfsts(pub u32);
impl Iprxfsts {
    #[doc = "Fill Level of IP Receive FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn fill(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Fill Level of IP Receive FIFO"]
    #[inline(always)]
    pub const fn set_fill(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Read Data Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn rdcntr(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Read Data Counter"]
    #[inline(always)]
    pub const fn set_rdcntr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Iprxfsts {
    #[inline(always)]
    fn default() -> Iprxfsts {
        Iprxfsts(0u64 as u32)
    }
}
impl core::fmt::Debug for Iprxfsts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iprxfsts")
            .field("fill", &self.fill())
            .field("rdcntr", &self.rdcntr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iprxfsts {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Iprxfsts {
            fill: u8,
            rdcntr: u16,
        }
        let proxy = Iprxfsts {
            fill: self.fill(),
            rdcntr: self.rdcntr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "IP Transmit FIFO Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iptxfcr(pub u32);
impl Iptxfcr {
    #[doc = "Clear IP Transmit FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn clriptxf(&self) -> super::vals::Clriptxf {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Clriptxf::from_bits(val as u8)
    }
    #[doc = "Clear IP Transmit FIFO"]
    #[inline(always)]
    pub const fn set_clriptxf(&mut self, val: super::vals::Clriptxf) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn txdmaen(&self) -> super::vals::Txdmaen {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Txdmaen::from_bits(val as u8)
    }
    #[doc = "Transmit FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_txdmaen(&mut self, val: super::vals::Txdmaen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmit Watermark Level"]
    #[must_use]
    #[inline(always)]
    pub const fn txwmrk(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x7f;
        val as u8
    }
    #[doc = "Transmit Watermark Level"]
    #[inline(always)]
    pub const fn set_txwmrk(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 2usize)) | (((val as u32) & 0x7f) << 2usize);
    }
}
impl Default for Iptxfcr {
    #[inline(always)]
    fn default() -> Iptxfcr {
        Iptxfcr(0u64 as u32)
    }
}
impl core::fmt::Debug for Iptxfcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iptxfcr")
            .field("clriptxf", &self.clriptxf())
            .field("txdmaen", &self.txdmaen())
            .field("txwmrk", &self.txwmrk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iptxfcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Iptxfcr {
            clriptxf: super::vals::Clriptxf,
            txdmaen: super::vals::Txdmaen,
            txwmrk: u8,
        }
        let proxy = Iptxfcr {
            clriptxf: self.clriptxf(),
            txdmaen: self.txdmaen(),
            txwmrk: self.txwmrk(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "IP Transmit FIFO Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iptxfsts(pub u32);
impl Iptxfsts {
    #[doc = "Fill Level of IP Transmit FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn fill(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Fill Level of IP Transmit FIFO"]
    #[inline(always)]
    pub const fn set_fill(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Write Data Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn wrcntr(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Write Data Counter"]
    #[inline(always)]
    pub const fn set_wrcntr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Iptxfsts {
    #[inline(always)]
    fn default() -> Iptxfsts {
        Iptxfsts(0u64 as u32)
    }
}
impl core::fmt::Debug for Iptxfsts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iptxfsts")
            .field("fill", &self.fill())
            .field("wrcntr", &self.wrcntr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iptxfsts {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Iptxfsts {
            fill: u8,
            wrcntr: u16,
        }
        let proxy = Iptxfsts {
            fill: self.fill(),
            wrcntr: self.wrcntr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Lookup Table x"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lut(pub u32);
impl Lut {
    #[doc = "OPERAND0"]
    #[must_use]
    #[inline(always)]
    pub const fn operand0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "OPERAND0"]
    #[inline(always)]
    pub const fn set_operand0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "NUM_PADS0"]
    #[must_use]
    #[inline(always)]
    pub const fn num_pads0(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "NUM_PADS0"]
    #[inline(always)]
    pub const fn set_num_pads0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "OPCODE"]
    #[must_use]
    #[inline(always)]
    pub const fn opcode0(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x3f;
        val as u8
    }
    #[doc = "OPCODE"]
    #[inline(always)]
    pub const fn set_opcode0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 10usize)) | (((val as u32) & 0x3f) << 10usize);
    }
    #[doc = "OPERAND1"]
    #[must_use]
    #[inline(always)]
    pub const fn operand1(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "OPERAND1"]
    #[inline(always)]
    pub const fn set_operand1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "NUM_PADS1"]
    #[must_use]
    #[inline(always)]
    pub const fn num_pads1(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "NUM_PADS1"]
    #[inline(always)]
    pub const fn set_num_pads1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "OPCODE1"]
    #[must_use]
    #[inline(always)]
    pub const fn opcode1(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x3f;
        val as u8
    }
    #[doc = "OPCODE1"]
    #[inline(always)]
    pub const fn set_opcode1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 26usize)) | (((val as u32) & 0x3f) << 26usize);
    }
}
impl Default for Lut {
    #[inline(always)]
    fn default() -> Lut {
        Lut(0u64 as u32)
    }
}
impl core::fmt::Debug for Lut {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lut")
            .field("operand0", &self.operand0())
            .field("num_pads0", &self.num_pads0())
            .field("opcode0", &self.opcode0())
            .field("operand1", &self.operand1())
            .field("num_pads1", &self.num_pads1())
            .field("opcode1", &self.opcode1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lut {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lut {
            operand0: u8,
            num_pads0: u8,
            opcode0: u8,
            operand1: u8,
            num_pads1: u8,
            opcode1: u8,
        }
        let proxy = Lut {
            operand0: self.operand0(),
            num_pads0: self.num_pads0(),
            opcode0: self.opcode0(),
            operand1: self.operand1(),
            num_pads1: self.num_pads1(),
            opcode1: self.opcode1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LUT Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lutcr(pub u32);
impl Lutcr {
    #[doc = "Lock LUT"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> super::vals::Lock {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lock::from_bits(val as u8)
    }
    #[doc = "Lock LUT"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: super::vals::Lock) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Unlock LUT"]
    #[must_use]
    #[inline(always)]
    pub const fn unlock(&self) -> super::vals::Unlock {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Unlock::from_bits(val as u8)
    }
    #[doc = "Unlock LUT"]
    #[inline(always)]
    pub const fn set_unlock(&mut self, val: super::vals::Unlock) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for Lutcr {
    #[inline(always)]
    fn default() -> Lutcr {
        Lutcr(2u64 as u32)
    }
}
impl core::fmt::Debug for Lutcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lutcr")
            .field("lock", &self.lock())
            .field("unlock", &self.unlock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lutcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lutcr {
            lock: super::vals::Lock,
            unlock: super::vals::Unlock,
        }
        let proxy = Lutcr {
            lock: self.lock(),
            unlock: self.unlock(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Module Control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcr0(pub u32);
impl Mcr0 {
    #[doc = "Software Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn swreset(&self) -> super::vals::Swreset {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Swreset::from_bits(val as u8)
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn set_swreset(&mut self, val: super::vals::Swreset) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Module Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn mdis(&self) -> super::vals::Mdis {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Mdis::from_bits(val as u8)
    }
    #[doc = "Module Disable"]
    #[inline(always)]
    pub const fn set_mdis(&mut self, val: super::vals::Mdis) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Sample Clock Source for Flash Reading"]
    #[must_use]
    #[inline(always)]
    pub const fn rxclksrc(&self) -> super::vals::Rxclksrc {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Rxclksrc::from_bits(val as u8)
    }
    #[doc = "Sample Clock Source for Flash Reading"]
    #[inline(always)]
    pub const fn set_rxclksrc(&mut self, val: super::vals::Rxclksrc) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "AHB Read Access to IP Receive FIFO Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ardfen(&self) -> super::vals::Ardfen {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Ardfen::from_bits(val as u8)
    }
    #[doc = "AHB Read Access to IP Receive FIFO Enable"]
    #[inline(always)]
    pub const fn set_ardfen(&mut self, val: super::vals::Ardfen) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "AHB Write Access to IP Transmit FIFO Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn atdfen(&self) -> super::vals::Atdfen {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Atdfen::from_bits(val as u8)
    }
    #[doc = "AHB Write Access to IP Transmit FIFO Enable"]
    #[inline(always)]
    pub const fn set_atdfen(&mut self, val: super::vals::Atdfen) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Serial Root Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn serclkdiv(&self) -> super::vals::Serclkdiv {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Serclkdiv::from_bits(val as u8)
    }
    #[doc = "Serial Root Clock Divider"]
    #[inline(always)]
    pub const fn set_serclkdiv(&mut self, val: super::vals::Serclkdiv) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Half Speed Serial Flash Memory Access Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hsen(&self) -> super::vals::Hsen {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Hsen::from_bits(val as u8)
    }
    #[doc = "Half Speed Serial Flash Memory Access Enable"]
    #[inline(always)]
    pub const fn set_hsen(&mut self, val: super::vals::Hsen) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Doze Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dozeen(&self) -> super::vals::Dozeen {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Dozeen::from_bits(val as u8)
    }
    #[doc = "Doze Mode Enable"]
    #[inline(always)]
    pub const fn set_dozeen(&mut self, val: super::vals::Dozeen) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Combination Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn combinationen(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Combination Mode Enable"]
    #[inline(always)]
    pub const fn set_combinationen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "SCLK Free-running Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sckfreerunen(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SCLK Free-running Enable"]
    #[inline(always)]
    pub const fn set_sckfreerunen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Timeout Wait Cycle for IP Command Grant"]
    #[must_use]
    #[inline(always)]
    pub const fn ipgrantwait(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Timeout Wait Cycle for IP Command Grant"]
    #[inline(always)]
    pub const fn set_ipgrantwait(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Timeouts Wait Cycle for AHB command Grant"]
    #[must_use]
    #[inline(always)]
    pub const fn ahbgrantwait(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Timeouts Wait Cycle for AHB command Grant"]
    #[inline(always)]
    pub const fn set_ahbgrantwait(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Mcr0 {
    #[inline(always)]
    fn default() -> Mcr0 {
        Mcr0(4294934722u64 as u32)
    }
}
impl core::fmt::Debug for Mcr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mcr0")
            .field("swreset", &self.swreset())
            .field("mdis", &self.mdis())
            .field("rxclksrc", &self.rxclksrc())
            .field("ardfen", &self.ardfen())
            .field("atdfen", &self.atdfen())
            .field("serclkdiv", &self.serclkdiv())
            .field("hsen", &self.hsen())
            .field("dozeen", &self.dozeen())
            .field("combinationen", &self.combinationen())
            .field("sckfreerunen", &self.sckfreerunen())
            .field("ipgrantwait", &self.ipgrantwait())
            .field("ahbgrantwait", &self.ahbgrantwait())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mcr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Mcr0 {
            swreset: super::vals::Swreset,
            mdis: super::vals::Mdis,
            rxclksrc: super::vals::Rxclksrc,
            ardfen: super::vals::Ardfen,
            atdfen: super::vals::Atdfen,
            serclkdiv: super::vals::Serclkdiv,
            hsen: super::vals::Hsen,
            dozeen: super::vals::Dozeen,
            combinationen: bool,
            sckfreerunen: bool,
            ipgrantwait: u8,
            ahbgrantwait: u8,
        }
        let proxy = Mcr0 {
            swreset: self.swreset(),
            mdis: self.mdis(),
            rxclksrc: self.rxclksrc(),
            ardfen: self.ardfen(),
            atdfen: self.atdfen(),
            serclkdiv: self.serclkdiv(),
            hsen: self.hsen(),
            dozeen: self.dozeen(),
            combinationen: self.combinationen(),
            sckfreerunen: self.sckfreerunen(),
            ipgrantwait: self.ipgrantwait(),
            ahbgrantwait: self.ahbgrantwait(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Module Control 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcr1(pub u32);
impl Mcr1 {
    #[doc = "AHB Bus Wait"]
    #[must_use]
    #[inline(always)]
    pub const fn ahbbuswait(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "AHB Bus Wait"]
    #[inline(always)]
    pub const fn set_ahbbuswait(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Command Sequence Wait"]
    #[must_use]
    #[inline(always)]
    pub const fn seqwait(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Command Sequence Wait"]
    #[inline(always)]
    pub const fn set_seqwait(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Mcr1 {
    #[inline(always)]
    fn default() -> Mcr1 {
        Mcr1(4294967295u64 as u32)
    }
}
impl core::fmt::Debug for Mcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mcr1")
            .field("ahbbuswait", &self.ahbbuswait())
            .field("seqwait", &self.seqwait())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mcr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Mcr1 {
            ahbbuswait: u16,
            seqwait: u16,
        }
        let proxy = Mcr1 {
            ahbbuswait: self.ahbbuswait(),
            seqwait: self.seqwait(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Module Control 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcr2(pub u32);
impl Mcr2 {
    #[doc = "Clear AHB Buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn clrahbbufopt(&self) -> super::vals::Clrahbbufopt {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Clrahbbufopt::from_bits(val as u8)
    }
    #[doc = "Clear AHB Buffer"]
    #[inline(always)]
    pub const fn set_clrahbbufopt(&mut self, val: super::vals::Clrahbbufopt) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Same Device Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn samedeviceen(&self) -> super::vals::Samedeviceen {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Samedeviceen::from_bits(val as u8)
    }
    #[doc = "Same Device Enable"]
    #[inline(always)]
    pub const fn set_samedeviceen(&mut self, val: super::vals::Samedeviceen) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "SCLK Port B Differential Output"]
    #[must_use]
    #[inline(always)]
    pub const fn sckbdiffopt(&self) -> super::vals::Sckbdiffopt {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Sckbdiffopt::from_bits(val as u8)
    }
    #[doc = "SCLK Port B Differential Output"]
    #[inline(always)]
    pub const fn set_sckbdiffopt(&mut self, val: super::vals::Sckbdiffopt) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Port B Receiver Clock Source"]
    #[must_use]
    #[inline(always)]
    pub const fn rxclksrc_b(&self) -> super::vals::RxclksrcB {
        let val = (self.0 >> 21usize) & 0x03;
        super::vals::RxclksrcB::from_bits(val as u8)
    }
    #[doc = "Port B Receiver Clock Source"]
    #[inline(always)]
    pub const fn set_rxclksrc_b(&mut self, val: super::vals::RxclksrcB) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
    #[doc = "Sample Clock Source Different"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_clk_src_diff(&self) -> super::vals::RxClkSrcDiff {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::RxClkSrcDiff::from_bits(val as u8)
    }
    #[doc = "Sample Clock Source Different"]
    #[inline(always)]
    pub const fn set_rx_clk_src_diff(&mut self, val: super::vals::RxClkSrcDiff) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Resume Wait Duration"]
    #[must_use]
    #[inline(always)]
    pub const fn resumewait(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Resume Wait Duration"]
    #[inline(always)]
    pub const fn set_resumewait(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Mcr2 {
    #[inline(always)]
    fn default() -> Mcr2 {
        Mcr2(536904183u64 as u32)
    }
}
impl core::fmt::Debug for Mcr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mcr2")
            .field("clrahbbufopt", &self.clrahbbufopt())
            .field("samedeviceen", &self.samedeviceen())
            .field("sckbdiffopt", &self.sckbdiffopt())
            .field("rxclksrc_b", &self.rxclksrc_b())
            .field("rx_clk_src_diff", &self.rx_clk_src_diff())
            .field("resumewait", &self.resumewait())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mcr2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Mcr2 {
            clrahbbufopt: super::vals::Clrahbbufopt,
            samedeviceen: super::vals::Samedeviceen,
            sckbdiffopt: super::vals::Sckbdiffopt,
            rxclksrc_b: super::vals::RxclksrcB,
            rx_clk_src_diff: super::vals::RxClkSrcDiff,
            resumewait: u8,
        }
        let proxy = Mcr2 {
            clrahbbufopt: self.clrahbbufopt(),
            samedeviceen: self.samedeviceen(),
            sckbdiffopt: self.sckbdiffopt(),
            rxclksrc_b: self.rxclksrc_b(),
            rx_clk_src_diff: self.rx_clk_src_diff(),
            resumewait: self.resumewait(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Status 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sts0(pub u32);
impl Sts0 {
    #[doc = "SEQ_CTL State Machine Idle"]
    #[must_use]
    #[inline(always)]
    pub const fn seqidle(&self) -> super::vals::Seqidle {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Seqidle::from_bits(val as u8)
    }
    #[doc = "SEQ_CTL State Machine Idle"]
    #[inline(always)]
    pub const fn set_seqidle(&mut self, val: super::vals::Seqidle) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "ARB_CTL State Machine Idle"]
    #[must_use]
    #[inline(always)]
    pub const fn arbidle(&self) -> super::vals::Arbidle {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Arbidle::from_bits(val as u8)
    }
    #[doc = "ARB_CTL State Machine Idle"]
    #[inline(always)]
    pub const fn set_arbidle(&mut self, val: super::vals::Arbidle) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "ARB Command Source"]
    #[must_use]
    #[inline(always)]
    pub const fn arbcmdsrc(&self) -> super::vals::Arbcmdsrc {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Arbcmdsrc::from_bits(val as u8)
    }
    #[doc = "ARB Command Source"]
    #[inline(always)]
    pub const fn set_arbcmdsrc(&mut self, val: super::vals::Arbcmdsrc) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for Sts0 {
    #[inline(always)]
    fn default() -> Sts0 {
        Sts0(2u64 as u32)
    }
}
impl core::fmt::Debug for Sts0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sts0")
            .field("seqidle", &self.seqidle())
            .field("arbidle", &self.arbidle())
            .field("arbcmdsrc", &self.arbcmdsrc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sts0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sts0 {
            seqidle: super::vals::Seqidle,
            arbidle: super::vals::Arbidle,
            arbcmdsrc: super::vals::Arbcmdsrc,
        }
        let proxy = Sts0 {
            seqidle: self.seqidle(),
            arbidle: self.arbidle(),
            arbcmdsrc: self.arbcmdsrc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Status 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sts1(pub u32);
impl Sts1 {
    #[doc = "AHB Command Error ID"]
    #[must_use]
    #[inline(always)]
    pub const fn ahbcmderrid(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "AHB Command Error ID"]
    #[inline(always)]
    pub const fn set_ahbcmderrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "AHB Command Error Code"]
    #[must_use]
    #[inline(always)]
    pub const fn ahbcmderrcode(&self) -> super::vals::Ahbcmderrcode {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Ahbcmderrcode::from_bits(val as u8)
    }
    #[doc = "AHB Command Error Code"]
    #[inline(always)]
    pub const fn set_ahbcmderrcode(&mut self, val: super::vals::Ahbcmderrcode) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "IP Command Error ID"]
    #[must_use]
    #[inline(always)]
    pub const fn ipcmderrid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "IP Command Error ID"]
    #[inline(always)]
    pub const fn set_ipcmderrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "IP Command Error Code"]
    #[must_use]
    #[inline(always)]
    pub const fn ipcmderrcode(&self) -> super::vals::Ipcmderrcode {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Ipcmderrcode::from_bits(val as u8)
    }
    #[doc = "IP Command Error Code"]
    #[inline(always)]
    pub const fn set_ipcmderrcode(&mut self, val: super::vals::Ipcmderrcode) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Sts1 {
    #[inline(always)]
    fn default() -> Sts1 {
        Sts1(0u64 as u32)
    }
}
impl core::fmt::Debug for Sts1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sts1")
            .field("ahbcmderrid", &self.ahbcmderrid())
            .field("ahbcmderrcode", &self.ahbcmderrcode())
            .field("ipcmderrid", &self.ipcmderrid())
            .field("ipcmderrcode", &self.ipcmderrcode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sts1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sts1 {
            ahbcmderrid: u8,
            ahbcmderrcode: super::vals::Ahbcmderrcode,
            ipcmderrid: u8,
            ipcmderrcode: super::vals::Ipcmderrcode,
        }
        let proxy = Sts1 {
            ahbcmderrid: self.ahbcmderrid(),
            ahbcmderrcode: self.ahbcmderrcode(),
            ipcmderrid: self.ipcmderrid(),
            ipcmderrcode: self.ipcmderrcode(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Status 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sts2(pub u32);
impl Sts2 {
    #[doc = "Flash A Sample Target Delay Line Locked"]
    #[must_use]
    #[inline(always)]
    pub const fn aslvlock(&self) -> super::vals::Aslvlock {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Aslvlock::from_bits(val as u8)
    }
    #[doc = "Flash A Sample Target Delay Line Locked"]
    #[inline(always)]
    pub const fn set_aslvlock(&mut self, val: super::vals::Aslvlock) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Flash A Sample Clock Reference Delay Line Locked"]
    #[must_use]
    #[inline(always)]
    pub const fn areflock(&self) -> super::vals::Areflock {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Areflock::from_bits(val as u8)
    }
    #[doc = "Flash A Sample Clock Reference Delay Line Locked"]
    #[inline(always)]
    pub const fn set_areflock(&mut self, val: super::vals::Areflock) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Flash A Sample Clock Target Delay Line Delay Cell Number"]
    #[must_use]
    #[inline(always)]
    pub const fn aslvsel(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[doc = "Flash A Sample Clock Target Delay Line Delay Cell Number"]
    #[inline(always)]
    pub const fn set_aslvsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u32) & 0x3f) << 2usize);
    }
    #[doc = "Flash A Sample Clock Reference Delay Line Delay Cell Number"]
    #[must_use]
    #[inline(always)]
    pub const fn arefsel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Flash A Sample Clock Reference Delay Line Delay Cell Number"]
    #[inline(always)]
    pub const fn set_arefsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Flash B Sample Target Reference Delay Line Locked"]
    #[must_use]
    #[inline(always)]
    pub const fn bslvlock(&self) -> super::vals::Bslvlock {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Bslvlock::from_bits(val as u8)
    }
    #[doc = "Flash B Sample Target Reference Delay Line Locked"]
    #[inline(always)]
    pub const fn set_bslvlock(&mut self, val: super::vals::Bslvlock) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Flash B Sample Clock Reference Delay Line Locked"]
    #[must_use]
    #[inline(always)]
    pub const fn breflock(&self) -> super::vals::Breflock {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Breflock::from_bits(val as u8)
    }
    #[doc = "Flash B Sample Clock Reference Delay Line Locked"]
    #[inline(always)]
    pub const fn set_breflock(&mut self, val: super::vals::Breflock) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Flash B Sample Clock Target Delay Line Delay Cell Number"]
    #[must_use]
    #[inline(always)]
    pub const fn bslvsel(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x3f;
        val as u8
    }
    #[doc = "Flash B Sample Clock Target Delay Line Delay Cell Number"]
    #[inline(always)]
    pub const fn set_bslvsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 18usize)) | (((val as u32) & 0x3f) << 18usize);
    }
    #[doc = "Flash B Sample Clock Reference Delay Line Delay Cell Number"]
    #[must_use]
    #[inline(always)]
    pub const fn brefsel(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Flash B Sample Clock Reference Delay Line Delay Cell Number"]
    #[inline(always)]
    pub const fn set_brefsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for Sts2 {
    #[inline(always)]
    fn default() -> Sts2 {
        Sts2(16777472u64 as u32)
    }
}
impl core::fmt::Debug for Sts2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sts2")
            .field("aslvlock", &self.aslvlock())
            .field("areflock", &self.areflock())
            .field("aslvsel", &self.aslvsel())
            .field("arefsel", &self.arefsel())
            .field("bslvlock", &self.bslvlock())
            .field("breflock", &self.breflock())
            .field("bslvsel", &self.bslvsel())
            .field("brefsel", &self.brefsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sts2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sts2 {
            aslvlock: super::vals::Aslvlock,
            areflock: super::vals::Areflock,
            aslvsel: u8,
            arefsel: u8,
            bslvlock: super::vals::Bslvlock,
            breflock: super::vals::Breflock,
            bslvsel: u8,
            brefsel: u8,
        }
        let proxy = Sts2 {
            aslvlock: self.aslvlock(),
            areflock: self.areflock(),
            aslvsel: self.aslvsel(),
            arefsel: self.arefsel(),
            bslvlock: self.bslvlock(),
            breflock: self.breflock(),
            bslvsel: self.bslvsel(),
            brefsel: self.brefsel(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
