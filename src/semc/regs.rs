#[doc = "Bus (AXI) Master Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bmcr0(pub u32);
impl Bmcr0 {
    #[doc = "Weight of QOS"]
    #[must_use]
    #[inline(always)]
    pub const fn wqos(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Weight of QOS"]
    #[inline(always)]
    pub const fn set_wqos(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Weight of AGE"]
    #[must_use]
    #[inline(always)]
    pub const fn wage(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Weight of AGE"]
    #[inline(always)]
    pub const fn set_wage(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Weight of Slave Hit without read/write switch"]
    #[must_use]
    #[inline(always)]
    pub const fn wsh(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Weight of Slave Hit without read/write switch"]
    #[inline(always)]
    pub const fn set_wsh(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Weight of slave hit with Read/Write Switch"]
    #[must_use]
    #[inline(always)]
    pub const fn wrws(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Weight of slave hit with Read/Write Switch"]
    #[inline(always)]
    pub const fn set_wrws(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Bmcr0 {
    #[inline(always)]
    fn default() -> Bmcr0 {
        Bmcr0(0u64 as u32)
    }
}
impl core::fmt::Debug for Bmcr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bmcr0")
            .field("wqos", &self.wqos())
            .field("wage", &self.wage())
            .field("wsh", &self.wsh())
            .field("wrws", &self.wrws())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bmcr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Bmcr0 {
            wqos: u8,
            wage: u8,
            wsh: u8,
            wrws: u8,
        }
        let proxy = Bmcr0 {
            wqos: self.wqos(),
            wage: self.wage(),
            wsh: self.wsh(),
            wrws: self.wrws(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Bus (AXI) Master Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bmcr1(pub u32);
impl Bmcr1 {
    #[doc = "Weight of QOS"]
    #[must_use]
    #[inline(always)]
    pub const fn wqos(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Weight of QOS"]
    #[inline(always)]
    pub const fn set_wqos(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Weight of AGE"]
    #[must_use]
    #[inline(always)]
    pub const fn wage(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Weight of AGE"]
    #[inline(always)]
    pub const fn set_wage(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Weight of Page Hit"]
    #[must_use]
    #[inline(always)]
    pub const fn wph(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Weight of Page Hit"]
    #[inline(always)]
    pub const fn set_wph(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Weight of slave hit without Read/Write Switch"]
    #[must_use]
    #[inline(always)]
    pub const fn wrws(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Weight of slave hit without Read/Write Switch"]
    #[inline(always)]
    pub const fn set_wrws(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Weight of Bank Rotation"]
    #[must_use]
    #[inline(always)]
    pub const fn wbr(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Weight of Bank Rotation"]
    #[inline(always)]
    pub const fn set_wbr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Bmcr1 {
    #[inline(always)]
    fn default() -> Bmcr1 {
        Bmcr1(0u64 as u32)
    }
}
impl core::fmt::Debug for Bmcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bmcr1")
            .field("wqos", &self.wqos())
            .field("wage", &self.wage())
            .field("wph", &self.wph())
            .field("wrws", &self.wrws())
            .field("wbr", &self.wbr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bmcr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Bmcr1 {
            wqos: u8,
            wage: u8,
            wph: u8,
            wrws: u8,
            wbr: u8,
        }
        let proxy = Bmcr1 {
            wqos: self.wqos(),
            wage: self.wage(),
            wph: self.wph(),
            wrws: self.wrws(),
            wbr: self.wbr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Base Register n"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Br(pub u32);
impl Br {
    #[doc = "Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn vld(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Valid"]
    #[inline(always)]
    pub const fn set_vld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Memory size"]
    #[must_use]
    #[inline(always)]
    pub const fn ms(&self) -> super::vals::BrMs {
        let val = (self.0 >> 1usize) & 0x1f;
        super::vals::BrMs::from_bits(val as u8)
    }
    #[doc = "Memory size"]
    #[inline(always)]
    pub const fn set_ms(&mut self, val: super::vals::BrMs) {
        self.0 = (self.0 & !(0x1f << 1usize)) | (((val.to_bits() as u32) & 0x1f) << 1usize);
    }
    #[doc = "Base Address"]
    #[must_use]
    #[inline(always)]
    pub const fn ba(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Base Address"]
    #[inline(always)]
    pub const fn set_ba(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Br {
    #[inline(always)]
    fn default() -> Br {
        Br(0u64 as u32)
    }
}
impl core::fmt::Debug for Br {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Br")
            .field("vld", &self.vld())
            .field("ms", &self.ms())
            .field("ba", &self.ba())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Br {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Br {
            vld: bool,
            ms: super::vals::BrMs,
            ba: u32,
        }
        let proxy = Br {
            vld: self.vld(),
            ms: self.ms(),
            ba: self.ba(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Base Register 10"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Br10(pub u32);
impl Br10 {
    #[doc = "Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn vld(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Valid"]
    #[inline(always)]
    pub const fn set_vld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Memory size"]
    #[must_use]
    #[inline(always)]
    pub const fn ms(&self) -> super::vals::Br10Ms {
        let val = (self.0 >> 1usize) & 0x1f;
        super::vals::Br10Ms::from_bits(val as u8)
    }
    #[doc = "Memory size"]
    #[inline(always)]
    pub const fn set_ms(&mut self, val: super::vals::Br10Ms) {
        self.0 = (self.0 & !(0x1f << 1usize)) | (((val.to_bits() as u32) & 0x1f) << 1usize);
    }
    #[doc = "Base Address"]
    #[must_use]
    #[inline(always)]
    pub const fn ba(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Base Address"]
    #[inline(always)]
    pub const fn set_ba(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Br10 {
    #[inline(always)]
    fn default() -> Br10 {
        Br10(2751463448u64 as u32)
    }
}
impl core::fmt::Debug for Br10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Br10")
            .field("vld", &self.vld())
            .field("ms", &self.ms())
            .field("ba", &self.ba())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Br10 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Br10 {
            vld: bool,
            ms: super::vals::Br10Ms,
            ba: u32,
        }
        let proxy = Br10 {
            vld: self.vld(),
            ms: self.ms(),
            ba: self.ba(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Base Register 11"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Br11(pub u32);
impl Br11 {
    #[doc = "Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn vld(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Valid"]
    #[inline(always)]
    pub const fn set_vld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Memory size"]
    #[must_use]
    #[inline(always)]
    pub const fn ms(&self) -> super::vals::Br11Ms {
        let val = (self.0 >> 1usize) & 0x1f;
        super::vals::Br11Ms::from_bits(val as u8)
    }
    #[doc = "Memory size"]
    #[inline(always)]
    pub const fn set_ms(&mut self, val: super::vals::Br11Ms) {
        self.0 = (self.0 & !(0x1f << 1usize)) | (((val.to_bits() as u32) & 0x1f) << 1usize);
    }
    #[doc = "Base Address"]
    #[must_use]
    #[inline(always)]
    pub const fn ba(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Base Address"]
    #[inline(always)]
    pub const fn set_ba(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Br11 {
    #[inline(always)]
    fn default() -> Br11 {
        Br11(2818572312u64 as u32)
    }
}
impl core::fmt::Debug for Br11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Br11")
            .field("vld", &self.vld())
            .field("ms", &self.ms())
            .field("ba", &self.ba())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Br11 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Br11 {
            vld: bool,
            ms: super::vals::Br11Ms,
            ba: u32,
        }
        let proxy = Br11 {
            vld: self.vld(),
            ms: self.ms(),
            ba: self.ba(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Base Register 9"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Br9(pub u32);
impl Br9 {
    #[doc = "Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn vld(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Valid"]
    #[inline(always)]
    pub const fn set_vld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Memory size"]
    #[must_use]
    #[inline(always)]
    pub const fn ms(&self) -> super::vals::Br9Ms {
        let val = (self.0 >> 1usize) & 0x1f;
        super::vals::Br9Ms::from_bits(val as u8)
    }
    #[doc = "Memory size"]
    #[inline(always)]
    pub const fn set_ms(&mut self, val: super::vals::Br9Ms) {
        self.0 = (self.0 & !(0x1f << 1usize)) | (((val.to_bits() as u32) & 0x1f) << 1usize);
    }
    #[doc = "Base Address"]
    #[must_use]
    #[inline(always)]
    pub const fn ba(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Base Address"]
    #[inline(always)]
    pub const fn set_ba(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Br9 {
    #[inline(always)]
    fn default() -> Br9 {
        Br9(2684354584u64 as u32)
    }
}
impl core::fmt::Debug for Br9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Br9")
            .field("vld", &self.vld())
            .field("ms", &self.ms())
            .field("ba", &self.ba())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Br9 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Br9 {
            vld: bool,
            ms: super::vals::Br9Ms,
            ba: u32,
        }
        let proxy = Br9 {
            vld: self.vld(),
            ms: self.ms(),
            ba: self.ba(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "DBI-B Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbicr0(pub u32);
impl Dbicr0 {
    #[doc = "Port Size"]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> super::vals::Dbicr0Ps {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Dbicr0Ps::from_bits(val as u8)
    }
    #[doc = "Port Size"]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: super::vals::Dbicr0Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Burst Length"]
    #[must_use]
    #[inline(always)]
    pub const fn bl(&self) -> super::vals::Dbicr0Bl {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Dbicr0Bl::from_bits(val as u8)
    }
    #[doc = "Burst Length"]
    #[inline(always)]
    pub const fn set_bl(&mut self, val: super::vals::Dbicr0Bl) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Column Address bit width"]
    #[must_use]
    #[inline(always)]
    pub const fn col(&self) -> super::vals::Dbicr0Col {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Dbicr0Col::from_bits(val as u8)
    }
    #[doc = "Column Address bit width"]
    #[inline(always)]
    pub const fn set_col(&mut self, val: super::vals::Dbicr0Col) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
}
impl Default for Dbicr0 {
    #[inline(always)]
    fn default() -> Dbicr0 {
        Dbicr0(0u64 as u32)
    }
}
impl core::fmt::Debug for Dbicr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dbicr0")
            .field("ps", &self.ps())
            .field("bl", &self.bl())
            .field("col", &self.col())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dbicr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Dbicr0 {
            ps: super::vals::Dbicr0Ps,
            bl: super::vals::Dbicr0Bl,
            col: super::vals::Dbicr0Col,
        }
        let proxy = Dbicr0 {
            ps: self.ps(),
            bl: self.bl(),
            col: self.col(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "DBI-B Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbicr1(pub u32);
impl Dbicr1 {
    #[doc = "CSX Setup Time"]
    #[must_use]
    #[inline(always)]
    pub const fn ces(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "CSX Setup Time"]
    #[inline(always)]
    pub const fn set_ces(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "CSX Hold Time"]
    #[must_use]
    #[inline(always)]
    pub const fn ceh(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "CSX Hold Time"]
    #[inline(always)]
    pub const fn set_ceh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "WRX Low Time"]
    #[must_use]
    #[inline(always)]
    pub const fn wel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "WRX Low Time"]
    #[inline(always)]
    pub const fn set_wel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "WRX High Time"]
    #[must_use]
    #[inline(always)]
    pub const fn weh(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "WRX High Time"]
    #[inline(always)]
    pub const fn set_weh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "RDX Low Time"]
    #[must_use]
    #[inline(always)]
    pub const fn rel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "RDX Low Time"]
    #[inline(always)]
    pub const fn set_rel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
    #[doc = "RDX High Time"]
    #[must_use]
    #[inline(always)]
    pub const fn reh(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x7f;
        val as u8
    }
    #[doc = "RDX High Time"]
    #[inline(always)]
    pub const fn set_reh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
    }
}
impl Default for Dbicr1 {
    #[inline(always)]
    fn default() -> Dbicr1 {
        Dbicr1(0u64 as u32)
    }
}
impl core::fmt::Debug for Dbicr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dbicr1")
            .field("ces", &self.ces())
            .field("ceh", &self.ceh())
            .field("wel", &self.wel())
            .field("weh", &self.weh())
            .field("rel", &self.rel())
            .field("reh", &self.reh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dbicr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Dbicr1 {
            ces: u8,
            ceh: u8,
            wel: u8,
            weh: u8,
            rel: u8,
            reh: u8,
        }
        let proxy = Dbicr1 {
            ces: self.ces(),
            ceh: self.ceh(),
            wel: self.wel(),
            weh: self.weh(),
            rel: self.rel(),
            reh: self.reh(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "DBI-B Control Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbicr2(pub u32);
impl Dbicr2 {
    #[doc = "CSX interval time"]
    #[must_use]
    #[inline(always)]
    pub const fn ceitv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "CSX interval time"]
    #[inline(always)]
    pub const fn set_ceitv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Dbicr2 {
    #[inline(always)]
    fn default() -> Dbicr2 {
        Dbicr2(0u64 as u32)
    }
}
impl core::fmt::Debug for Dbicr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dbicr2")
            .field("ceitv", &self.ceitv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dbicr2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Dbicr2 {
            ceitv: u8,
        }
        let proxy = Dbicr2 {
            ceitv: self.ceitv(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Delay Chain Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dccr(pub u32);
impl Dccr {
    #[doc = "Delay chain insertion enable for SRAM device."]
    #[must_use]
    #[inline(always)]
    pub const fn sdramen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Delay chain insertion enable for SRAM device."]
    #[inline(always)]
    pub const fn set_sdramen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Clock delay line delay cell number selection value for SDRAM device."]
    #[must_use]
    #[inline(always)]
    pub const fn sdramval(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x1f;
        val as u8
    }
    #[doc = "Clock delay line delay cell number selection value for SDRAM device."]
    #[inline(always)]
    pub const fn set_sdramval(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 1usize)) | (((val as u32) & 0x1f) << 1usize);
    }
    #[doc = "Delay chain insertion enable for NOR device."]
    #[must_use]
    #[inline(always)]
    pub const fn noren(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Delay chain insertion enable for NOR device."]
    #[inline(always)]
    pub const fn set_noren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Clock delay line delay cell number selection value for NOR device."]
    #[must_use]
    #[inline(always)]
    pub const fn norval(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x1f;
        val as u8
    }
    #[doc = "Clock delay line delay cell number selection value for NOR device."]
    #[inline(always)]
    pub const fn set_norval(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 9usize)) | (((val as u32) & 0x1f) << 9usize);
    }
    #[doc = "Delay chain insertion enable for SRAM device 0."]
    #[must_use]
    #[inline(always)]
    pub const fn sram0en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Delay chain insertion enable for SRAM device 0."]
    #[inline(always)]
    pub const fn set_sram0en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Clock delay line delay cell number selection value for SRAM device 0."]
    #[must_use]
    #[inline(always)]
    pub const fn sram0val(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x1f;
        val as u8
    }
    #[doc = "Clock delay line delay cell number selection value for SRAM device 0."]
    #[inline(always)]
    pub const fn set_sram0val(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 17usize)) | (((val as u32) & 0x1f) << 17usize);
    }
    #[doc = "Delay chain insertion enable for SRAM device 1-3."]
    #[must_use]
    #[inline(always)]
    pub const fn sramxen(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Delay chain insertion enable for SRAM device 1-3."]
    #[inline(always)]
    pub const fn set_sramxen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Clock delay line delay cell number selection value for SRAM device 1-3."]
    #[must_use]
    #[inline(always)]
    pub const fn sramxval(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x1f;
        val as u8
    }
    #[doc = "Clock delay line delay cell number selection value for SRAM device 1-3."]
    #[inline(always)]
    pub const fn set_sramxval(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 25usize)) | (((val as u32) & 0x1f) << 25usize);
    }
}
impl Default for Dccr {
    #[inline(always)]
    fn default() -> Dccr {
        Dccr(0u64 as u32)
    }
}
impl core::fmt::Debug for Dccr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dccr")
            .field("sdramen", &self.sdramen())
            .field("sdramval", &self.sdramval())
            .field("noren", &self.noren())
            .field("norval", &self.norval())
            .field("sram0en", &self.sram0en())
            .field("sram0val", &self.sram0val())
            .field("sramxen", &self.sramxen())
            .field("sramxval", &self.sramxval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dccr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Dccr {
            sdramen: bool,
            sdramval: u8,
            noren: bool,
            norval: u8,
            sram0en: bool,
            sram0val: u8,
            sramxen: bool,
            sramxval: u8,
        }
        let proxy = Dccr {
            sdramen: self.sdramen(),
            sdramval: self.sdramval(),
            noren: self.noren(),
            norval: self.norval(),
            sram0en: self.sram0en(),
            sram0val: self.sram0val(),
            sramxen: self.sramxen(),
            sramxval: self.sramxval(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "DLL Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dllcr(pub u32);
impl Dllcr {
    #[doc = "DLL calibration enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dllen(&self) -> super::vals::Dllen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Dllen::from_bits(val as u8)
    }
    #[doc = "DLL calibration enable"]
    #[inline(always)]
    pub const fn set_dllen(&mut self, val: super::vals::Dllen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "DLL Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn dllreset(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DLL Reset"]
    #[inline(always)]
    pub const fn set_dllreset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Delay Target for Slave"]
    #[must_use]
    #[inline(always)]
    pub const fn slvdlytarget(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x0f;
        val as u8
    }
    #[doc = "Delay Target for Slave"]
    #[inline(always)]
    pub const fn set_slvdlytarget(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
    }
    #[doc = "Override Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ovrden(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Override Enable"]
    #[inline(always)]
    pub const fn set_ovrden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Override Value"]
    #[must_use]
    #[inline(always)]
    pub const fn ovrdval(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x3f;
        val as u8
    }
    #[doc = "Override Value"]
    #[inline(always)]
    pub const fn set_ovrdval(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 9usize)) | (((val as u32) & 0x3f) << 9usize);
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
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dllcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Dllcr {
            dllen: super::vals::Dllen,
            dllreset: bool,
            slvdlytarget: u8,
            ovrden: bool,
            ovrdval: u8,
        }
        let proxy = Dllcr {
            dllen: self.dllen(),
            dllreset: self.dllreset(),
            slvdlytarget: self.slvdlytarget(),
            ovrden: self.ovrden(),
            ovrdval: self.ovrdval(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "IP command done interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ipcmddoneen(&self) -> super::vals::Ipcmddoneen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ipcmddoneen::from_bits(val as u8)
    }
    #[doc = "IP command done interrupt enable"]
    #[inline(always)]
    pub const fn set_ipcmddoneen(&mut self, val: super::vals::Ipcmddoneen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "IP command error interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ipcmderren(&self) -> super::vals::Ipcmderren {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ipcmderren::from_bits(val as u8)
    }
    #[doc = "IP command error interrupt enable"]
    #[inline(always)]
    pub const fn set_ipcmderren(&mut self, val: super::vals::Ipcmderren) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "AXI command error interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn axicmderren(&self) -> super::vals::Axicmderren {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Axicmderren::from_bits(val as u8)
    }
    #[doc = "AXI command error interrupt enable"]
    #[inline(always)]
    pub const fn set_axicmderren(&mut self, val: super::vals::Axicmderren) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "AXI bus error interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn axibuserren(&self) -> super::vals::Axibuserren {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Axibuserren::from_bits(val as u8)
    }
    #[doc = "AXI bus error interrupt enable"]
    #[inline(always)]
    pub const fn set_axibuserren(&mut self, val: super::vals::Axibuserren) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "NAND page end interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ndpageenden(&self) -> super::vals::Ndpageenden {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Ndpageenden::from_bits(val as u8)
    }
    #[doc = "NAND page end interrupt enable"]
    #[inline(always)]
    pub const fn set_ndpageenden(&mut self, val: super::vals::Ndpageenden) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "NAND no pending AXI access interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ndnopenden(&self) -> super::vals::Ndnopenden {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Ndnopenden::from_bits(val as u8)
    }
    #[doc = "NAND no pending AXI access interrupt enable"]
    #[inline(always)]
    pub const fn set_ndnopenden(&mut self, val: super::vals::Ndnopenden) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "NAND ECC fail interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ndeccfailen(&self) -> super::vals::Ndeccfailen {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Ndeccfailen::from_bits(val as u8)
    }
    #[doc = "NAND ECC fail interrupt enable"]
    #[inline(always)]
    pub const fn set_ndeccfailen(&mut self, val: super::vals::Ndeccfailen) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "NAND buffer end interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ndbufenden(&self) -> super::vals::Ndbufenden {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Ndbufenden::from_bits(val as u8)
    }
    #[doc = "NAND buffer end interrupt enable"]
    #[inline(always)]
    pub const fn set_ndbufenden(&mut self, val: super::vals::Ndbufenden) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
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
            .field("ipcmderren", &self.ipcmderren())
            .field("axicmderren", &self.axicmderren())
            .field("axibuserren", &self.axibuserren())
            .field("ndpageenden", &self.ndpageenden())
            .field("ndnopenden", &self.ndnopenden())
            .field("ndeccfailen", &self.ndeccfailen())
            .field("ndbufenden", &self.ndbufenden())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Inten {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Inten {
            ipcmddoneen: super::vals::Ipcmddoneen,
            ipcmderren: super::vals::Ipcmderren,
            axicmderren: super::vals::Axicmderren,
            axibuserren: super::vals::Axibuserren,
            ndpageenden: super::vals::Ndpageenden,
            ndnopenden: super::vals::Ndnopenden,
            ndeccfailen: super::vals::Ndeccfailen,
            ndbufenden: super::vals::Ndbufenden,
        }
        let proxy = Inten {
            ipcmddoneen: self.ipcmddoneen(),
            ipcmderren: self.ipcmderren(),
            axicmderren: self.axicmderren(),
            axibuserren: self.axibuserren(),
            ndpageenden: self.ndpageenden(),
            ndnopenden: self.ndnopenden(),
            ndeccfailen: self.ndeccfailen(),
            ndbufenden: self.ndbufenden(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intr(pub u32);
impl Intr {
    #[doc = "IP command normal done interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn ipcmddone(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "IP command normal done interrupt"]
    #[inline(always)]
    pub const fn set_ipcmddone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "IP command error done interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn ipcmderr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "IP command error done interrupt"]
    #[inline(always)]
    pub const fn set_ipcmderr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "AXI command error interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn axicmderr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "AXI command error interrupt"]
    #[inline(always)]
    pub const fn set_axicmderr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "AXI bus error interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn axibuserr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "AXI bus error interrupt"]
    #[inline(always)]
    pub const fn set_axibuserr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "NAND page end interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn ndpageend(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "NAND page end interrupt"]
    #[inline(always)]
    pub const fn set_ndpageend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "NAND no pending AXI write transaction interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn ndnopend(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "NAND no pending AXI write transaction interrupt"]
    #[inline(always)]
    pub const fn set_ndnopend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "NAND ECC fail interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn ndeccfail(&self) -> super::vals::Ndeccfail {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Ndeccfail::from_bits(val as u8)
    }
    #[doc = "NAND ECC fail interrupt"]
    #[inline(always)]
    pub const fn set_ndeccfail(&mut self, val: super::vals::Ndeccfail) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "NAND buffer end interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn ndbufend(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "NAND buffer end interrupt"]
    #[inline(always)]
    pub const fn set_ndbufend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
            .field("ipcmderr", &self.ipcmderr())
            .field("axicmderr", &self.axicmderr())
            .field("axibuserr", &self.axibuserr())
            .field("ndpageend", &self.ndpageend())
            .field("ndnopend", &self.ndnopend())
            .field("ndeccfail", &self.ndeccfail())
            .field("ndbufend", &self.ndbufend())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Intr {
            ipcmddone: bool,
            ipcmderr: bool,
            axicmderr: bool,
            axibuserr: bool,
            ndpageend: bool,
            ndnopend: bool,
            ndeccfail: super::vals::Ndeccfail,
            ndbufend: bool,
        }
        let proxy = Intr {
            ipcmddone: self.ipcmddone(),
            ipcmderr: self.ipcmderr(),
            axicmderr: self.axicmderr(),
            axibuserr: self.axibuserr(),
            ndpageend: self.ndpageend(),
            ndnopend: self.ndnopend(),
            ndeccfail: self.ndeccfail(),
            ndbufend: self.ndbufend(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "IO MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iocr(pub u32);
impl Iocr {
    #[doc = "SEMC_ADDR08 output selection"]
    #[must_use]
    #[inline(always)]
    pub const fn mux_a8(&self) -> super::vals::MuxA8 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::MuxA8::from_bits(val as u8)
    }
    #[doc = "SEMC_ADDR08 output selection"]
    #[inline(always)]
    pub const fn set_mux_a8(&mut self, val: super::vals::MuxA8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "SEMC_CSX0 output selection"]
    #[must_use]
    #[inline(always)]
    pub const fn mux_csx0(&self) -> super::vals::MuxCsx0 {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::MuxCsx0::from_bits(val as u8)
    }
    #[doc = "SEMC_CSX0 output selection"]
    #[inline(always)]
    pub const fn set_mux_csx0(&mut self, val: super::vals::MuxCsx0) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "SEMC_CSX1 output selection"]
    #[must_use]
    #[inline(always)]
    pub const fn mux_csx1(&self) -> super::vals::MuxCsx1 {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::MuxCsx1::from_bits(val as u8)
    }
    #[doc = "SEMC_CSX1 output selection"]
    #[inline(always)]
    pub const fn set_mux_csx1(&mut self, val: super::vals::MuxCsx1) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "SEMC_CSX2 output selection"]
    #[must_use]
    #[inline(always)]
    pub const fn mux_csx2(&self) -> super::vals::MuxCsx2 {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::MuxCsx2::from_bits(val as u8)
    }
    #[doc = "SEMC_CSX2 output selection"]
    #[inline(always)]
    pub const fn set_mux_csx2(&mut self, val: super::vals::MuxCsx2) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "SEMC_CSX3 output selection"]
    #[must_use]
    #[inline(always)]
    pub const fn mux_csx3(&self) -> super::vals::MuxCsx3 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::MuxCsx3::from_bits(val as u8)
    }
    #[doc = "SEMC_CSX3 output selection"]
    #[inline(always)]
    pub const fn set_mux_csx3(&mut self, val: super::vals::MuxCsx3) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "SEMC_RDY function selection"]
    #[must_use]
    #[inline(always)]
    pub const fn mux_rdy(&self) -> super::vals::MuxRdy {
        let val = (self.0 >> 20usize) & 0x0f;
        super::vals::MuxRdy::from_bits(val as u8)
    }
    #[doc = "SEMC_RDY function selection"]
    #[inline(always)]
    pub const fn set_mux_rdy(&mut self, val: super::vals::MuxRdy) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
    }
    #[doc = "SEMC_CLKX0 function selection"]
    #[must_use]
    #[inline(always)]
    pub const fn mux_clkx0(&self) -> super::vals::MuxClkx0 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::MuxClkx0::from_bits(val as u8)
    }
    #[doc = "SEMC_CLKX0 function selection"]
    #[inline(always)]
    pub const fn set_mux_clkx0(&mut self, val: super::vals::MuxClkx0) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "SEMC_CLKX1 function selection"]
    #[must_use]
    #[inline(always)]
    pub const fn mux_clkx1(&self) -> super::vals::MuxClkx1 {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::MuxClkx1::from_bits(val as u8)
    }
    #[doc = "SEMC_CLKX1 function selection"]
    #[inline(always)]
    pub const fn set_mux_clkx1(&mut self, val: super::vals::MuxClkx1) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "SEMC_CLKX0 Always On"]
    #[must_use]
    #[inline(always)]
    pub const fn clkx0_ao(&self) -> super::vals::Clkx0Ao {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Clkx0Ao::from_bits(val as u8)
    }
    #[doc = "SEMC_CLKX0 Always On"]
    #[inline(always)]
    pub const fn set_clkx0_ao(&mut self, val: super::vals::Clkx0Ao) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "SEMC_CLKX1 Always On"]
    #[must_use]
    #[inline(always)]
    pub const fn clkx1_ao(&self) -> super::vals::Clkx1Ao {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Clkx1Ao::from_bits(val as u8)
    }
    #[doc = "SEMC_CLKX1 Always On"]
    #[inline(always)]
    pub const fn set_clkx1_ao(&mut self, val: super::vals::Clkx1Ao) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
}
impl Default for Iocr {
    #[inline(always)]
    fn default() -> Iocr {
        Iocr(0u64 as u32)
    }
}
impl core::fmt::Debug for Iocr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iocr")
            .field("mux_a8", &self.mux_a8())
            .field("mux_csx0", &self.mux_csx0())
            .field("mux_csx1", &self.mux_csx1())
            .field("mux_csx2", &self.mux_csx2())
            .field("mux_csx3", &self.mux_csx3())
            .field("mux_rdy", &self.mux_rdy())
            .field("mux_clkx0", &self.mux_clkx0())
            .field("mux_clkx1", &self.mux_clkx1())
            .field("clkx0_ao", &self.clkx0_ao())
            .field("clkx1_ao", &self.clkx1_ao())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iocr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Iocr {
            mux_a8: super::vals::MuxA8,
            mux_csx0: super::vals::MuxCsx0,
            mux_csx1: super::vals::MuxCsx1,
            mux_csx2: super::vals::MuxCsx2,
            mux_csx3: super::vals::MuxCsx3,
            mux_rdy: super::vals::MuxRdy,
            mux_clkx0: super::vals::MuxClkx0,
            mux_clkx1: super::vals::MuxClkx1,
            clkx0_ao: super::vals::Clkx0Ao,
            clkx1_ao: super::vals::Clkx1Ao,
        }
        let proxy = Iocr {
            mux_a8: self.mux_a8(),
            mux_csx0: self.mux_csx0(),
            mux_csx1: self.mux_csx1(),
            mux_csx2: self.mux_csx2(),
            mux_csx3: self.mux_csx3(),
            mux_rdy: self.mux_rdy(),
            mux_clkx0: self.mux_clkx0(),
            mux_clkx1: self.mux_clkx1(),
            clkx0_ao: self.clkx0_ao(),
            clkx1_ao: self.clkx1_ao(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "IP Command Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipcmd(pub u32);
impl Ipcmd {
    #[doc = "SDRAM Commands: 0x5: Extended Mode Register Set 0x6: Deep Power Down 0x8: Read 0x9: Write 0xA: Mode Register Set 0xB: Active 0xC: Auto Refresh 0xD: Self Refresh 0xE: Precharge 0xF: Precharge All Others: Reserved Self Refresh is sent to all SDRAM devices because they share the same SEMC_CLK pin"]
    #[must_use]
    #[inline(always)]
    pub const fn cmd(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "SDRAM Commands: 0x5: Extended Mode Register Set 0x6: Deep Power Down 0x8: Read 0x9: Write 0xA: Mode Register Set 0xB: Active 0xC: Auto Refresh 0xD: Self Refresh 0xE: Precharge 0xF: Precharge All Others: Reserved Self Refresh is sent to all SDRAM devices because they share the same SEMC_CLK pin"]
    #[inline(always)]
    pub const fn set_cmd(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "This field should be written with 0xA55A when trigging an IP command for all device types"]
    #[must_use]
    #[inline(always)]
    pub const fn key(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "This field should be written with 0xA55A when trigging an IP command for all device types"]
    #[inline(always)]
    pub const fn set_key(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
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
        f.debug_struct("Ipcmd")
            .field("cmd", &self.cmd())
            .field("key", &self.key())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipcmd {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ipcmd {
            cmd: u16,
            key: u16,
        }
        let proxy = Ipcmd {
            cmd: self.cmd(),
            key: self.key(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "IP Command Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipcr1(pub u32);
impl Ipcr1 {
    #[doc = "Data Size in Byte"]
    #[must_use]
    #[inline(always)]
    pub const fn datsz(&self) -> super::vals::Datsz {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Datsz::from_bits(val as u8)
    }
    #[doc = "Data Size in Byte"]
    #[inline(always)]
    pub const fn set_datsz(&mut self, val: super::vals::Datsz) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "NAND Extended Address"]
    #[must_use]
    #[inline(always)]
    pub const fn nand_ext_addr(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "NAND Extended Address"]
    #[inline(always)]
    pub const fn set_nand_ext_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
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
            .field("datsz", &self.datsz())
            .field("nand_ext_addr", &self.nand_ext_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipcr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ipcr1 {
            datsz: super::vals::Datsz,
            nand_ext_addr: u8,
        }
        let proxy = Ipcr1 {
            datsz: self.datsz(),
            nand_ext_addr: self.nand_ext_addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "IP Command Control Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipcr2(pub u32);
impl Ipcr2 {
    #[doc = "Byte Mask for Byte 0 (IPTXDAT bit 7:0)"]
    #[must_use]
    #[inline(always)]
    pub const fn bm0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Byte Mask for Byte 0 (IPTXDAT bit 7:0)"]
    #[inline(always)]
    pub const fn set_bm0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Byte Mask for Byte 1 (IPTXDAT bit 15:8)"]
    #[must_use]
    #[inline(always)]
    pub const fn bm1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Byte Mask for Byte 1 (IPTXDAT bit 15:8)"]
    #[inline(always)]
    pub const fn set_bm1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Byte Mask for Byte 2 (IPTXDAT bit 23:16)"]
    #[must_use]
    #[inline(always)]
    pub const fn bm2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Byte Mask for Byte 2 (IPTXDAT bit 23:16)"]
    #[inline(always)]
    pub const fn set_bm2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Byte Mask for Byte 3 (IPTXDAT bit 31:24)"]
    #[must_use]
    #[inline(always)]
    pub const fn bm3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Byte Mask for Byte 3 (IPTXDAT bit 31:24)"]
    #[inline(always)]
    pub const fn set_bm3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Ipcr2 {
    #[inline(always)]
    fn default() -> Ipcr2 {
        Ipcr2(0u64 as u32)
    }
}
impl core::fmt::Debug for Ipcr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipcr2")
            .field("bm0", &self.bm0())
            .field("bm1", &self.bm1())
            .field("bm2", &self.bm2())
            .field("bm3", &self.bm3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipcr2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ipcr2 {
            bm0: bool,
            bm1: bool,
            bm2: bool,
            bm3: bool,
        }
        let proxy = Ipcr2 {
            bm0: self.bm0(),
            bm1: self.bm1(),
            bm2: self.bm2(),
            bm3: self.bm3(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Module Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcr(pub u32);
impl Mcr {
    #[doc = "Software Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn swrst(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn set_swrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
    #[doc = "DQS (read strobe) mode"]
    #[must_use]
    #[inline(always)]
    pub const fn dqsmd(&self) -> super::vals::Dqsmd {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Dqsmd::from_bits(val as u8)
    }
    #[doc = "DQS (read strobe) mode"]
    #[inline(always)]
    pub const fn set_dqsmd(&mut self, val: super::vals::Dqsmd) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "WAIT/RDY polarity for SRAM/NOR"]
    #[must_use]
    #[inline(always)]
    pub const fn wpol0(&self) -> super::vals::Wpol0 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Wpol0::from_bits(val as u8)
    }
    #[doc = "WAIT/RDY polarity for SRAM/NOR"]
    #[inline(always)]
    pub const fn set_wpol0(&mut self, val: super::vals::Wpol0) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "R/B# polarity for NAND device"]
    #[must_use]
    #[inline(always)]
    pub const fn wpol1(&self) -> super::vals::Wpol1 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Wpol1::from_bits(val as u8)
    }
    #[doc = "R/B# polarity for NAND device"]
    #[inline(always)]
    pub const fn set_wpol1(&mut self, val: super::vals::Wpol1) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Command Execution timeout cycles"]
    #[must_use]
    #[inline(always)]
    pub const fn cto(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Command Execution timeout cycles"]
    #[inline(always)]
    pub const fn set_cto(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Bus timeout cycles"]
    #[must_use]
    #[inline(always)]
    pub const fn bto(&self) -> super::vals::Bto {
        let val = (self.0 >> 24usize) & 0x1f;
        super::vals::Bto::from_bits(val as u8)
    }
    #[doc = "Bus timeout cycles"]
    #[inline(always)]
    pub const fn set_bto(&mut self, val: super::vals::Bto) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val.to_bits() as u32) & 0x1f) << 24usize);
    }
}
impl Default for Mcr {
    #[inline(always)]
    fn default() -> Mcr {
        Mcr(268435458u64 as u32)
    }
}
impl core::fmt::Debug for Mcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mcr")
            .field("swrst", &self.swrst())
            .field("mdis", &self.mdis())
            .field("dqsmd", &self.dqsmd())
            .field("wpol0", &self.wpol0())
            .field("wpol1", &self.wpol1())
            .field("cto", &self.cto())
            .field("bto", &self.bto())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Mcr {
            swrst: bool,
            mdis: super::vals::Mdis,
            dqsmd: super::vals::Dqsmd,
            wpol0: super::vals::Wpol0,
            wpol1: super::vals::Wpol1,
            cto: u8,
            bto: super::vals::Bto,
        }
        let proxy = Mcr {
            swrst: self.swrst(),
            mdis: self.mdis(),
            dqsmd: self.dqsmd(),
            wpol0: self.wpol0(),
            wpol1: self.wpol1(),
            cto: self.cto(),
            bto: self.bto(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NAND Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nandcr0(pub u32);
impl Nandcr0 {
    #[doc = "Port Size"]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> super::vals::Nandcr0Ps {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Nandcr0Ps::from_bits(val as u8)
    }
    #[doc = "Port Size"]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: super::vals::Nandcr0Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Synchronous Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn syncen(&self) -> super::vals::Nandcr0Syncen {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Nandcr0Syncen::from_bits(val as u8)
    }
    #[doc = "Synchronous Mode Enable"]
    #[inline(always)]
    pub const fn set_syncen(&mut self, val: super::vals::Nandcr0Syncen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Burst Length"]
    #[must_use]
    #[inline(always)]
    pub const fn bl(&self) -> super::vals::Nandcr0Bl {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Nandcr0Bl::from_bits(val as u8)
    }
    #[doc = "Burst Length"]
    #[inline(always)]
    pub const fn set_bl(&mut self, val: super::vals::Nandcr0Bl) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "EDO mode enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn edo(&self) -> super::vals::Edo {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Edo::from_bits(val as u8)
    }
    #[doc = "EDO mode enabled"]
    #[inline(always)]
    pub const fn set_edo(&mut self, val: super::vals::Edo) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Column address bit number"]
    #[must_use]
    #[inline(always)]
    pub const fn col(&self) -> super::vals::Nandcr0Col {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Nandcr0Col::from_bits(val as u8)
    }
    #[doc = "Column address bit number"]
    #[inline(always)]
    pub const fn set_col(&mut self, val: super::vals::Nandcr0Col) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "NAND buffer enable for AXI access"]
    #[must_use]
    #[inline(always)]
    pub const fn bufen(&self) -> super::vals::Bufen {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Bufen::from_bits(val as u8)
    }
    #[doc = "NAND buffer enable for AXI access"]
    #[inline(always)]
    pub const fn set_bufen(&mut self, val: super::vals::Bufen) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "ECC mode selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ecc_mode(&self) -> super::vals::EccMode {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::EccMode::from_bits(val as u8)
    }
    #[doc = "ECC mode selection"]
    #[inline(always)]
    pub const fn set_ecc_mode(&mut self, val: super::vals::EccMode) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Sector numbers in NAND buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn sector_num(&self) -> super::vals::SectorNum {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::SectorNum::from_bits(val as u8)
    }
    #[doc = "Sector numbers in NAND buffer"]
    #[inline(always)]
    pub const fn set_sector_num(&mut self, val: super::vals::SectorNum) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Size in bytes of one elementary unit of ECC correction."]
    #[must_use]
    #[inline(always)]
    pub const fn sector_size(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Size in bytes of one elementary unit of ECC correction."]
    #[inline(always)]
    pub const fn set_sector_size(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Nandcr0 {
    #[inline(always)]
    fn default() -> Nandcr0 {
        Nandcr0(0u64 as u32)
    }
}
impl core::fmt::Debug for Nandcr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nandcr0")
            .field("ps", &self.ps())
            .field("syncen", &self.syncen())
            .field("bl", &self.bl())
            .field("edo", &self.edo())
            .field("col", &self.col())
            .field("bufen", &self.bufen())
            .field("ecc_mode", &self.ecc_mode())
            .field("sector_num", &self.sector_num())
            .field("sector_size", &self.sector_size())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nandcr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Nandcr0 {
            ps: super::vals::Nandcr0Ps,
            syncen: super::vals::Nandcr0Syncen,
            bl: super::vals::Nandcr0Bl,
            edo: super::vals::Edo,
            col: super::vals::Nandcr0Col,
            bufen: super::vals::Bufen,
            ecc_mode: super::vals::EccMode,
            sector_num: super::vals::SectorNum,
            sector_size: u16,
        }
        let proxy = Nandcr0 {
            ps: self.ps(),
            syncen: self.syncen(),
            bl: self.bl(),
            edo: self.edo(),
            col: self.col(),
            bufen: self.bufen(),
            ecc_mode: self.ecc_mode(),
            sector_num: self.sector_num(),
            sector_size: self.sector_size(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NAND Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nandcr1(pub u32);
impl Nandcr1 {
    #[doc = "CE# setup time"]
    #[must_use]
    #[inline(always)]
    pub const fn ces(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "CE# setup time"]
    #[inline(always)]
    pub const fn set_ces(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "CE# hold time"]
    #[must_use]
    #[inline(always)]
    pub const fn ceh(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "CE# hold time"]
    #[inline(always)]
    pub const fn set_ceh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "WE# low time"]
    #[must_use]
    #[inline(always)]
    pub const fn wel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "WE# low time"]
    #[inline(always)]
    pub const fn set_wel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "WE# high time"]
    #[must_use]
    #[inline(always)]
    pub const fn weh(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "WE# high time"]
    #[inline(always)]
    pub const fn set_weh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "RE# low time"]
    #[must_use]
    #[inline(always)]
    pub const fn rel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "RE# low time"]
    #[inline(always)]
    pub const fn set_rel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "RE# high time"]
    #[must_use]
    #[inline(always)]
    pub const fn reh(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "RE# high time"]
    #[inline(always)]
    pub const fn set_reh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Turnaround time"]
    #[must_use]
    #[inline(always)]
    pub const fn ta(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Turnaround time"]
    #[inline(always)]
    pub const fn set_ta(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "CE# interval time"]
    #[must_use]
    #[inline(always)]
    pub const fn ceitv(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "CE# interval time"]
    #[inline(always)]
    pub const fn set_ceitv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Nandcr1 {
    #[inline(always)]
    fn default() -> Nandcr1 {
        Nandcr1(0u64 as u32)
    }
}
impl core::fmt::Debug for Nandcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nandcr1")
            .field("ces", &self.ces())
            .field("ceh", &self.ceh())
            .field("wel", &self.wel())
            .field("weh", &self.weh())
            .field("rel", &self.rel())
            .field("reh", &self.reh())
            .field("ta", &self.ta())
            .field("ceitv", &self.ceitv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nandcr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Nandcr1 {
            ces: u8,
            ceh: u8,
            wel: u8,
            weh: u8,
            rel: u8,
            reh: u8,
            ta: u8,
            ceitv: u8,
        }
        let proxy = Nandcr1 {
            ces: self.ces(),
            ceh: self.ceh(),
            wel: self.wel(),
            weh: self.weh(),
            rel: self.rel(),
            reh: self.reh(),
            ta: self.ta(),
            ceitv: self.ceitv(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NAND Control Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nandcr2(pub u32);
impl Nandcr2 {
    #[doc = "WE# high to RE# low time"]
    #[must_use]
    #[inline(always)]
    pub const fn twhr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "WE# high to RE# low time"]
    #[inline(always)]
    pub const fn set_twhr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "RE# high to WE# low time"]
    #[must_use]
    #[inline(always)]
    pub const fn trhw(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x3f;
        val as u8
    }
    #[doc = "RE# high to WE# low time"]
    #[inline(always)]
    pub const fn set_trhw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 6usize)) | (((val as u32) & 0x3f) << 6usize);
    }
    #[doc = "Address cycle to data loading time"]
    #[must_use]
    #[inline(always)]
    pub const fn tadl(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x3f;
        val as u8
    }
    #[doc = "Address cycle to data loading time"]
    #[inline(always)]
    pub const fn set_tadl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 12usize)) | (((val as u32) & 0x3f) << 12usize);
    }
    #[doc = "Ready to RE# low time"]
    #[must_use]
    #[inline(always)]
    pub const fn trr(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x3f;
        val as u8
    }
    #[doc = "Ready to RE# low time"]
    #[inline(always)]
    pub const fn set_trr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 18usize)) | (((val as u32) & 0x3f) << 18usize);
    }
    #[doc = "WE# high to busy time"]
    #[must_use]
    #[inline(always)]
    pub const fn twb(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "WE# high to busy time"]
    #[inline(always)]
    pub const fn set_twb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for Nandcr2 {
    #[inline(always)]
    fn default() -> Nandcr2 {
        Nandcr2(66576u64 as u32)
    }
}
impl core::fmt::Debug for Nandcr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nandcr2")
            .field("twhr", &self.twhr())
            .field("trhw", &self.trhw())
            .field("tadl", &self.tadl())
            .field("trr", &self.trr())
            .field("twb", &self.twb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nandcr2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Nandcr2 {
            twhr: u8,
            trhw: u8,
            tadl: u8,
            trr: u8,
            twb: u8,
        }
        let proxy = Nandcr2 {
            twhr: self.twhr(),
            trhw: self.trhw(),
            tadl: self.tadl(),
            trr: self.trr(),
            twb: self.twb(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NAND Control Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nandcr3(pub u32);
impl Nandcr3 {
    #[doc = "NAND option bit 1"]
    #[must_use]
    #[inline(always)]
    pub const fn ndopt1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "NAND option bit 1"]
    #[inline(always)]
    pub const fn set_ndopt1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "NAND option bit 2"]
    #[must_use]
    #[inline(always)]
    pub const fn ndopt2(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "NAND option bit 2"]
    #[inline(always)]
    pub const fn set_ndopt2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "NAND option bit 3"]
    #[must_use]
    #[inline(always)]
    pub const fn ndopt3(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "NAND option bit 3"]
    #[inline(always)]
    pub const fn set_ndopt3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "NAND CLE Option"]
    #[must_use]
    #[inline(always)]
    pub const fn cle(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "NAND CLE Option"]
    #[inline(always)]
    pub const fn set_cle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Read Data Setup time"]
    #[must_use]
    #[inline(always)]
    pub const fn rds(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Read Data Setup time"]
    #[inline(always)]
    pub const fn set_rds(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Read Data Hold time"]
    #[must_use]
    #[inline(always)]
    pub const fn rdh(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Read Data Hold time"]
    #[inline(always)]
    pub const fn set_rdh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Write Data Setup time"]
    #[must_use]
    #[inline(always)]
    pub const fn wds(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Write Data Setup time"]
    #[inline(always)]
    pub const fn set_wds(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Write Data Hold time"]
    #[must_use]
    #[inline(always)]
    pub const fn wdh(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Write Data Hold time"]
    #[inline(always)]
    pub const fn set_wdh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Nandcr3 {
    #[inline(always)]
    fn default() -> Nandcr3 {
        Nandcr3(0u64 as u32)
    }
}
impl core::fmt::Debug for Nandcr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nandcr3")
            .field("ndopt1", &self.ndopt1())
            .field("ndopt2", &self.ndopt2())
            .field("ndopt3", &self.ndopt3())
            .field("cle", &self.cle())
            .field("rds", &self.rds())
            .field("rdh", &self.rdh())
            .field("wds", &self.wds())
            .field("wdh", &self.wdh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nandcr3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Nandcr3 {
            ndopt1: bool,
            ndopt2: bool,
            ndopt3: bool,
            cle: bool,
            rds: u8,
            rdh: u8,
            wds: u8,
            wdh: u8,
        }
        let proxy = Nandcr3 {
            ndopt1: self.ndopt1(),
            ndopt2: self.ndopt2(),
            ndopt3: self.ndopt3(),
            cle: self.cle(),
            rds: self.rds(),
            rdh: self.rdh(),
            wds: self.wds(),
            wdh: self.wdh(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NAND Buffer Address Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ndba(pub u32);
impl Ndba {
    #[doc = "NAND Buffer address. It is used for program or read operation from IPS bus. It should be configured to proper value before access to NDBD register."]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "NAND Buffer address. It is used for program or read operation from IPS bus. It should be configured to proper value before access to NDBD register."]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for Ndba {
    #[inline(always)]
    fn default() -> Ndba {
        Ndba(0u64 as u32)
    }
}
impl core::fmt::Debug for Ndba {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ndba").field("addr", &self.addr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ndba {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ndba {
            addr: u16,
        }
        let proxy = Ndba { addr: self.addr() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NOR Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Norcr0(pub u32);
impl Norcr0 {
    #[doc = "Port Size"]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> super::vals::Norcr0Ps {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Norcr0Ps::from_bits(val as u8)
    }
    #[doc = "Port Size"]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: super::vals::Norcr0Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Synchronous Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn syncen(&self) -> super::vals::Norcr0Syncen {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Norcr0Syncen::from_bits(val as u8)
    }
    #[doc = "Synchronous Mode Enable"]
    #[inline(always)]
    pub const fn set_syncen(&mut self, val: super::vals::Norcr0Syncen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Burst Length"]
    #[must_use]
    #[inline(always)]
    pub const fn bl(&self) -> super::vals::Norcr0Bl {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Norcr0Bl::from_bits(val as u8)
    }
    #[doc = "Burst Length"]
    #[inline(always)]
    pub const fn set_bl(&mut self, val: super::vals::Norcr0Bl) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Address Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> super::vals::Norcr0Am {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Norcr0Am::from_bits(val as u8)
    }
    #[doc = "Address Mode"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: super::vals::Norcr0Am) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "ADV# Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn advp(&self) -> super::vals::Norcr0Advp {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Norcr0Advp::from_bits(val as u8)
    }
    #[doc = "ADV# Polarity"]
    #[inline(always)]
    pub const fn set_advp(&mut self, val: super::vals::Norcr0Advp) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "ADV# level control during address hold state"]
    #[must_use]
    #[inline(always)]
    pub const fn advh(&self) -> super::vals::Norcr0Advh {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Norcr0Advh::from_bits(val as u8)
    }
    #[doc = "ADV# level control during address hold state"]
    #[inline(always)]
    pub const fn set_advh(&mut self, val: super::vals::Norcr0Advh) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Column Address bit width"]
    #[must_use]
    #[inline(always)]
    pub const fn col(&self) -> super::vals::Norcr0Col {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Norcr0Col::from_bits(val as u8)
    }
    #[doc = "Column Address bit width"]
    #[inline(always)]
    pub const fn set_col(&mut self, val: super::vals::Norcr0Col) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
}
impl Default for Norcr0 {
    #[inline(always)]
    fn default() -> Norcr0 {
        Norcr0(0u64 as u32)
    }
}
impl core::fmt::Debug for Norcr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Norcr0")
            .field("ps", &self.ps())
            .field("syncen", &self.syncen())
            .field("bl", &self.bl())
            .field("am", &self.am())
            .field("advp", &self.advp())
            .field("advh", &self.advh())
            .field("col", &self.col())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Norcr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Norcr0 {
            ps: super::vals::Norcr0Ps,
            syncen: super::vals::Norcr0Syncen,
            bl: super::vals::Norcr0Bl,
            am: super::vals::Norcr0Am,
            advp: super::vals::Norcr0Advp,
            advh: super::vals::Norcr0Advh,
            col: super::vals::Norcr0Col,
        }
        let proxy = Norcr0 {
            ps: self.ps(),
            syncen: self.syncen(),
            bl: self.bl(),
            am: self.am(),
            advp: self.advp(),
            advh: self.advh(),
            col: self.col(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NOR Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Norcr1(pub u32);
impl Norcr1 {
    #[doc = "CE setup time"]
    #[must_use]
    #[inline(always)]
    pub const fn ces(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "CE setup time"]
    #[inline(always)]
    pub const fn set_ces(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "CE hold time"]
    #[must_use]
    #[inline(always)]
    pub const fn ceh(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "CE hold time"]
    #[inline(always)]
    pub const fn set_ceh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Address setup time"]
    #[must_use]
    #[inline(always)]
    pub const fn as_(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Address setup time"]
    #[inline(always)]
    pub const fn set_as_(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Address hold time"]
    #[must_use]
    #[inline(always)]
    pub const fn ah(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Address hold time"]
    #[inline(always)]
    pub const fn set_ah(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "WE low time"]
    #[must_use]
    #[inline(always)]
    pub const fn wel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "WE low time"]
    #[inline(always)]
    pub const fn set_wel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "WE high time"]
    #[must_use]
    #[inline(always)]
    pub const fn weh(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "WE high time"]
    #[inline(always)]
    pub const fn set_weh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "RE low time"]
    #[must_use]
    #[inline(always)]
    pub const fn rel(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "RE low time"]
    #[inline(always)]
    pub const fn set_rel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "RE high time"]
    #[must_use]
    #[inline(always)]
    pub const fn reh(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "RE high time"]
    #[inline(always)]
    pub const fn set_reh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Norcr1 {
    #[inline(always)]
    fn default() -> Norcr1 {
        Norcr1(0u64 as u32)
    }
}
impl core::fmt::Debug for Norcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Norcr1")
            .field("ces", &self.ces())
            .field("ceh", &self.ceh())
            .field("as_", &self.as_())
            .field("ah", &self.ah())
            .field("wel", &self.wel())
            .field("weh", &self.weh())
            .field("rel", &self.rel())
            .field("reh", &self.reh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Norcr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Norcr1 {
            ces: u8,
            ceh: u8,
            as_: u8,
            ah: u8,
            wel: u8,
            weh: u8,
            rel: u8,
            reh: u8,
        }
        let proxy = Norcr1 {
            ces: self.ces(),
            ceh: self.ceh(),
            as_: self.as_(),
            ah: self.ah(),
            wel: self.wel(),
            weh: self.weh(),
            rel: self.rel(),
            reh: self.reh(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NOR Control Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Norcr2(pub u32);
impl Norcr2 {
    #[doc = "Turnaround time"]
    #[must_use]
    #[inline(always)]
    pub const fn ta(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Turnaround time"]
    #[inline(always)]
    pub const fn set_ta(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Address to write data hold time"]
    #[must_use]
    #[inline(always)]
    pub const fn awdh(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Address to write data hold time"]
    #[inline(always)]
    pub const fn set_awdh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Latency count"]
    #[must_use]
    #[inline(always)]
    pub const fn lc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Latency count"]
    #[inline(always)]
    pub const fn set_lc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Read time"]
    #[must_use]
    #[inline(always)]
    pub const fn rd(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Read time"]
    #[inline(always)]
    pub const fn set_rd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "CE# interval time"]
    #[must_use]
    #[inline(always)]
    pub const fn ceitv(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "CE# interval time"]
    #[inline(always)]
    pub const fn set_ceitv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Read hold time"]
    #[must_use]
    #[inline(always)]
    pub const fn rdh(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Read hold time"]
    #[inline(always)]
    pub const fn set_rdh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Norcr2 {
    #[inline(always)]
    fn default() -> Norcr2 {
        Norcr2(0u64 as u32)
    }
}
impl core::fmt::Debug for Norcr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Norcr2")
            .field("ta", &self.ta())
            .field("awdh", &self.awdh())
            .field("lc", &self.lc())
            .field("rd", &self.rd())
            .field("ceitv", &self.ceitv())
            .field("rdh", &self.rdh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Norcr2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Norcr2 {
            ta: u8,
            awdh: u8,
            lc: u8,
            rd: u8,
            ceitv: u8,
            rdh: u8,
        }
        let proxy = Norcr2 {
            ta: self.ta(),
            awdh: self.awdh(),
            lc: self.lc(),
            rd: self.rd(),
            ceitv: self.ceitv(),
            rdh: self.rdh(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NOR Control Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Norcr3(pub u32);
impl Norcr3 {
    #[doc = "Address setup time for SYNC read"]
    #[must_use]
    #[inline(always)]
    pub const fn assr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Address setup time for SYNC read"]
    #[inline(always)]
    pub const fn set_assr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Address hold time for SYNC read"]
    #[must_use]
    #[inline(always)]
    pub const fn ahsr(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Address hold time for SYNC read"]
    #[inline(always)]
    pub const fn set_ahsr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Norcr3 {
    #[inline(always)]
    fn default() -> Norcr3 {
        Norcr3(0u64 as u32)
    }
}
impl core::fmt::Debug for Norcr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Norcr3")
            .field("assr", &self.assr())
            .field("ahsr", &self.ahsr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Norcr3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Norcr3 {
            assr: u8,
            ahsr: u8,
        }
        let proxy = Norcr3 {
            assr: self.assr(),
            ahsr: self.ahsr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SDRAM Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdramcr0(pub u32);
impl Sdramcr0 {
    #[doc = "Port Size"]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> super::vals::Sdramcr0Ps {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sdramcr0Ps::from_bits(val as u8)
    }
    #[doc = "Port Size"]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: super::vals::Sdramcr0Ps) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Burst Length"]
    #[must_use]
    #[inline(always)]
    pub const fn bl(&self) -> super::vals::Sdramcr0Bl {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Sdramcr0Bl::from_bits(val as u8)
    }
    #[doc = "Burst Length"]
    #[inline(always)]
    pub const fn set_bl(&mut self, val: super::vals::Sdramcr0Bl) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Column 8 selection"]
    #[must_use]
    #[inline(always)]
    pub const fn col8(&self) -> super::vals::Col8 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Col8::from_bits(val as u8)
    }
    #[doc = "Column 8 selection"]
    #[inline(always)]
    pub const fn set_col8(&mut self, val: super::vals::Col8) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Column address bit number"]
    #[must_use]
    #[inline(always)]
    pub const fn col(&self) -> super::vals::Sdramcr0Col {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Sdramcr0Col::from_bits(val as u8)
    }
    #[doc = "Column address bit number"]
    #[inline(always)]
    pub const fn set_col(&mut self, val: super::vals::Sdramcr0Col) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "CAS Latency"]
    #[must_use]
    #[inline(always)]
    pub const fn cl(&self) -> super::vals::Cl {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Cl::from_bits(val as u8)
    }
    #[doc = "CAS Latency"]
    #[inline(always)]
    pub const fn set_cl(&mut self, val: super::vals::Cl) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "2 Bank selection bit"]
    #[must_use]
    #[inline(always)]
    pub const fn bank2(&self) -> super::vals::Bank2 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Bank2::from_bits(val as u8)
    }
    #[doc = "2 Bank selection bit"]
    #[inline(always)]
    pub const fn set_bank2(&mut self, val: super::vals::Bank2) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
}
impl Default for Sdramcr0 {
    #[inline(always)]
    fn default() -> Sdramcr0 {
        Sdramcr0(3110u64 as u32)
    }
}
impl core::fmt::Debug for Sdramcr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sdramcr0")
            .field("ps", &self.ps())
            .field("bl", &self.bl())
            .field("col8", &self.col8())
            .field("col", &self.col())
            .field("cl", &self.cl())
            .field("bank2", &self.bank2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sdramcr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sdramcr0 {
            ps: super::vals::Sdramcr0Ps,
            bl: super::vals::Sdramcr0Bl,
            col8: super::vals::Col8,
            col: super::vals::Sdramcr0Col,
            cl: super::vals::Cl,
            bank2: super::vals::Bank2,
        }
        let proxy = Sdramcr0 {
            ps: self.ps(),
            bl: self.bl(),
            col8: self.col8(),
            col: self.col(),
            cl: self.cl(),
            bank2: self.bank2(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SDRAM Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdramcr1(pub u32);
impl Sdramcr1 {
    #[doc = "PRECHARGE to ACTIVE/REFRESH command wait time"]
    #[must_use]
    #[inline(always)]
    pub const fn pre2act(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "PRECHARGE to ACTIVE/REFRESH command wait time"]
    #[inline(always)]
    pub const fn set_pre2act(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "ACTIVE to READ/WRITE delay"]
    #[must_use]
    #[inline(always)]
    pub const fn act2rw(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "ACTIVE to READ/WRITE delay"]
    #[inline(always)]
    pub const fn set_act2rw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "REFRESH recovery time"]
    #[must_use]
    #[inline(always)]
    pub const fn rfrc(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "REFRESH recovery time"]
    #[inline(always)]
    pub const fn set_rfrc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "WRITE recovery time"]
    #[must_use]
    #[inline(always)]
    pub const fn wrc(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "WRITE recovery time"]
    #[inline(always)]
    pub const fn set_wrc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "CKE off minimum time"]
    #[must_use]
    #[inline(always)]
    pub const fn ckeoff(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "CKE off minimum time"]
    #[inline(always)]
    pub const fn set_ckeoff(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "ACTIVE to PRECHARGE minimum time"]
    #[must_use]
    #[inline(always)]
    pub const fn act2pre(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "ACTIVE to PRECHARGE minimum time"]
    #[inline(always)]
    pub const fn set_act2pre(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
}
impl Default for Sdramcr1 {
    #[inline(always)]
    fn default() -> Sdramcr1 {
        Sdramcr1(10045748u64 as u32)
    }
}
impl core::fmt::Debug for Sdramcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sdramcr1")
            .field("pre2act", &self.pre2act())
            .field("act2rw", &self.act2rw())
            .field("rfrc", &self.rfrc())
            .field("wrc", &self.wrc())
            .field("ckeoff", &self.ckeoff())
            .field("act2pre", &self.act2pre())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sdramcr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sdramcr1 {
            pre2act: u8,
            act2rw: u8,
            rfrc: u8,
            wrc: u8,
            ckeoff: u8,
            act2pre: u8,
        }
        let proxy = Sdramcr1 {
            pre2act: self.pre2act(),
            act2rw: self.act2rw(),
            rfrc: self.rfrc(),
            wrc: self.wrc(),
            ckeoff: self.ckeoff(),
            act2pre: self.act2pre(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SDRAM Control Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdramcr2(pub u32);
impl Sdramcr2 {
    #[doc = "SELF REFRESH recovery time"]
    #[must_use]
    #[inline(always)]
    pub const fn srrc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SELF REFRESH recovery time"]
    #[inline(always)]
    pub const fn set_srrc(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "REFRESH to REFRESH delay"]
    #[must_use]
    #[inline(always)]
    pub const fn ref2ref(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "REFRESH to REFRESH delay"]
    #[inline(always)]
    pub const fn set_ref2ref(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "ACTIVE to ACTIVE delay"]
    #[must_use]
    #[inline(always)]
    pub const fn act2act(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "ACTIVE to ACTIVE delay"]
    #[inline(always)]
    pub const fn set_act2act(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "SDRAM idle timeout"]
    #[must_use]
    #[inline(always)]
    pub const fn ito(&self) -> super::vals::Ito {
        let val = (self.0 >> 24usize) & 0xff;
        super::vals::Ito::from_bits(val as u8)
    }
    #[doc = "SDRAM idle timeout"]
    #[inline(always)]
    pub const fn set_ito(&mut self, val: super::vals::Ito) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val.to_bits() as u32) & 0xff) << 24usize);
    }
}
impl Default for Sdramcr2 {
    #[inline(always)]
    fn default() -> Sdramcr2 {
        Sdramcr2(2147487470u64 as u32)
    }
}
impl core::fmt::Debug for Sdramcr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sdramcr2")
            .field("srrc", &self.srrc())
            .field("ref2ref", &self.ref2ref())
            .field("act2act", &self.act2act())
            .field("ito", &self.ito())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sdramcr2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sdramcr2 {
            srrc: u8,
            ref2ref: u8,
            act2act: u8,
            ito: super::vals::Ito,
        }
        let proxy = Sdramcr2 {
            srrc: self.srrc(),
            ref2ref: self.ref2ref(),
            act2act: self.act2act(),
            ito: self.ito(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SDRAM Control Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdramcr3(pub u32);
impl Sdramcr3 {
    #[doc = "Refresh enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ren(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Refresh enable"]
    #[inline(always)]
    pub const fn set_ren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Refresh burst length"]
    #[must_use]
    #[inline(always)]
    pub const fn rebl(&self) -> super::vals::Rebl {
        let val = (self.0 >> 1usize) & 0x07;
        super::vals::Rebl::from_bits(val as u8)
    }
    #[doc = "Refresh burst length"]
    #[inline(always)]
    pub const fn set_rebl(&mut self, val: super::vals::Rebl) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val.to_bits() as u32) & 0x07) << 1usize);
    }
    #[doc = "Prescaler period"]
    #[must_use]
    #[inline(always)]
    pub const fn prescale(&self) -> super::vals::Prescale {
        let val = (self.0 >> 8usize) & 0xff;
        super::vals::Prescale::from_bits(val as u8)
    }
    #[doc = "Prescaler period"]
    #[inline(always)]
    pub const fn set_prescale(&mut self, val: super::vals::Prescale) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u32) & 0xff) << 8usize);
    }
    #[doc = "Refresh timer period"]
    #[must_use]
    #[inline(always)]
    pub const fn rt(&self) -> super::vals::Rt {
        let val = (self.0 >> 16usize) & 0xff;
        super::vals::Rt::from_bits(val as u8)
    }
    #[doc = "Refresh timer period"]
    #[inline(always)]
    pub const fn set_rt(&mut self, val: super::vals::Rt) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
    }
    #[doc = "Urgent refresh threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn ut(&self) -> super::vals::Ut {
        let val = (self.0 >> 24usize) & 0xff;
        super::vals::Ut::from_bits(val as u8)
    }
    #[doc = "Urgent refresh threshold"]
    #[inline(always)]
    pub const fn set_ut(&mut self, val: super::vals::Ut) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val.to_bits() as u32) & 0xff) << 24usize);
    }
}
impl Default for Sdramcr3 {
    #[inline(always)]
    fn default() -> Sdramcr3 {
        Sdramcr3(1082163200u64 as u32)
    }
}
impl core::fmt::Debug for Sdramcr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sdramcr3")
            .field("ren", &self.ren())
            .field("rebl", &self.rebl())
            .field("prescale", &self.prescale())
            .field("rt", &self.rt())
            .field("ut", &self.ut())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sdramcr3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sdramcr3 {
            ren: bool,
            rebl: super::vals::Rebl,
            prescale: super::vals::Prescale,
            rt: super::vals::Rt,
            ut: super::vals::Ut,
        }
        let proxy = Sdramcr3 {
            ren: self.ren(),
            rebl: self.rebl(),
            prescale: self.prescale(),
            rt: self.rt(),
            ut: self.ut(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SDRAM Prefetch Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdrampcr(pub u32);
impl Sdrampcr {
    #[doc = "SDRAM prefetch enable."]
    #[must_use]
    #[inline(always)]
    pub const fn prefetch_en(&self) -> super::vals::PrefetchEn {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::PrefetchEn::from_bits(val as u8)
    }
    #[doc = "SDRAM prefetch enable."]
    #[inline(always)]
    pub const fn set_prefetch_en(&mut self, val: super::vals::PrefetchEn) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "SDRAM prefetch delay cycle."]
    #[must_use]
    #[inline(always)]
    pub const fn prefetch_dly(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "SDRAM prefetch delay cycle."]
    #[inline(always)]
    pub const fn set_prefetch_dly(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
    }
}
impl Default for Sdrampcr {
    #[inline(always)]
    fn default() -> Sdrampcr {
        Sdrampcr(0u64 as u32)
    }
}
impl core::fmt::Debug for Sdrampcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sdrampcr")
            .field("prefetch_en", &self.prefetch_en())
            .field("prefetch_dly", &self.prefetch_dly())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sdrampcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sdrampcr {
            prefetch_en: super::vals::PrefetchEn,
            prefetch_dly: u8,
        }
        let proxy = Sdrampcr {
            prefetch_en: self.prefetch_en(),
            prefetch_dly: self.prefetch_dly(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SRAM Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramcr0(pub u32);
impl Sramcr0 {
    #[doc = "Port Size"]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> super::vals::Sramcr0Ps {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sramcr0Ps::from_bits(val as u8)
    }
    #[doc = "Port Size"]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: super::vals::Sramcr0Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Synchronous Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn syncen(&self) -> super::vals::Sramcr0Syncen {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sramcr0Syncen::from_bits(val as u8)
    }
    #[doc = "Synchronous Mode Enable"]
    #[inline(always)]
    pub const fn set_syncen(&mut self, val: super::vals::Sramcr0Syncen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Wait Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn waiten(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Wait Enable"]
    #[inline(always)]
    pub const fn set_waiten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Wait Sample"]
    #[must_use]
    #[inline(always)]
    pub const fn waitsp(&self) -> super::vals::Sramcr0Waitsp {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Sramcr0Waitsp::from_bits(val as u8)
    }
    #[doc = "Wait Sample"]
    #[inline(always)]
    pub const fn set_waitsp(&mut self, val: super::vals::Sramcr0Waitsp) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Burst Length"]
    #[must_use]
    #[inline(always)]
    pub const fn bl(&self) -> super::vals::Sramcr0Bl {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Sramcr0Bl::from_bits(val as u8)
    }
    #[doc = "Burst Length"]
    #[inline(always)]
    pub const fn set_bl(&mut self, val: super::vals::Sramcr0Bl) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Address Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> super::vals::Sramcr0Am {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Sramcr0Am::from_bits(val as u8)
    }
    #[doc = "Address Mode"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: super::vals::Sramcr0Am) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "ADV# polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn advp(&self) -> super::vals::Sramcr0Advp {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Sramcr0Advp::from_bits(val as u8)
    }
    #[doc = "ADV# polarity"]
    #[inline(always)]
    pub const fn set_advp(&mut self, val: super::vals::Sramcr0Advp) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "ADV# level control during address hold state"]
    #[must_use]
    #[inline(always)]
    pub const fn advh(&self) -> super::vals::Sramcr0Advh {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Sramcr0Advh::from_bits(val as u8)
    }
    #[doc = "ADV# level control during address hold state"]
    #[inline(always)]
    pub const fn set_advh(&mut self, val: super::vals::Sramcr0Advh) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Column Address bit width"]
    #[must_use]
    #[inline(always)]
    pub const fn col(&self) -> super::vals::Sramcr0Col {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Sramcr0Col::from_bits(val as u8)
    }
    #[doc = "Column Address bit width"]
    #[inline(always)]
    pub const fn set_col(&mut self, val: super::vals::Sramcr0Col) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
}
impl Default for Sramcr0 {
    #[inline(always)]
    fn default() -> Sramcr0 {
        Sramcr0(0u64 as u32)
    }
}
impl core::fmt::Debug for Sramcr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sramcr0")
            .field("ps", &self.ps())
            .field("syncen", &self.syncen())
            .field("waiten", &self.waiten())
            .field("waitsp", &self.waitsp())
            .field("bl", &self.bl())
            .field("am", &self.am())
            .field("advp", &self.advp())
            .field("advh", &self.advh())
            .field("col", &self.col())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sramcr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sramcr0 {
            ps: super::vals::Sramcr0Ps,
            syncen: super::vals::Sramcr0Syncen,
            waiten: bool,
            waitsp: super::vals::Sramcr0Waitsp,
            bl: super::vals::Sramcr0Bl,
            am: super::vals::Sramcr0Am,
            advp: super::vals::Sramcr0Advp,
            advh: super::vals::Sramcr0Advh,
            col: super::vals::Sramcr0Col,
        }
        let proxy = Sramcr0 {
            ps: self.ps(),
            syncen: self.syncen(),
            waiten: self.waiten(),
            waitsp: self.waitsp(),
            bl: self.bl(),
            am: self.am(),
            advp: self.advp(),
            advh: self.advh(),
            col: self.col(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SRAM Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramcr1(pub u32);
impl Sramcr1 {
    #[doc = "CE setup time"]
    #[must_use]
    #[inline(always)]
    pub const fn ces(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "CE setup time"]
    #[inline(always)]
    pub const fn set_ces(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "CE hold time"]
    #[must_use]
    #[inline(always)]
    pub const fn ceh(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "CE hold time"]
    #[inline(always)]
    pub const fn set_ceh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Address setup time"]
    #[must_use]
    #[inline(always)]
    pub const fn as_(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Address setup time"]
    #[inline(always)]
    pub const fn set_as_(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Address hold time"]
    #[must_use]
    #[inline(always)]
    pub const fn ah(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Address hold time"]
    #[inline(always)]
    pub const fn set_ah(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "WE low time"]
    #[must_use]
    #[inline(always)]
    pub const fn wel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "WE low time"]
    #[inline(always)]
    pub const fn set_wel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "WE high time"]
    #[must_use]
    #[inline(always)]
    pub const fn weh(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "WE high time"]
    #[inline(always)]
    pub const fn set_weh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "RE low time"]
    #[must_use]
    #[inline(always)]
    pub const fn rel(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "RE low time"]
    #[inline(always)]
    pub const fn set_rel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "RE high time"]
    #[must_use]
    #[inline(always)]
    pub const fn reh(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "RE high time"]
    #[inline(always)]
    pub const fn set_reh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Sramcr1 {
    #[inline(always)]
    fn default() -> Sramcr1 {
        Sramcr1(0u64 as u32)
    }
}
impl core::fmt::Debug for Sramcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sramcr1")
            .field("ces", &self.ces())
            .field("ceh", &self.ceh())
            .field("as_", &self.as_())
            .field("ah", &self.ah())
            .field("wel", &self.wel())
            .field("weh", &self.weh())
            .field("rel", &self.rel())
            .field("reh", &self.reh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sramcr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sramcr1 {
            ces: u8,
            ceh: u8,
            as_: u8,
            ah: u8,
            wel: u8,
            weh: u8,
            rel: u8,
            reh: u8,
        }
        let proxy = Sramcr1 {
            ces: self.ces(),
            ceh: self.ceh(),
            as_: self.as_(),
            ah: self.ah(),
            wel: self.wel(),
            weh: self.weh(),
            rel: self.rel(),
            reh: self.reh(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SRAM Control Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramcr2(pub u32);
impl Sramcr2 {
    #[doc = "Write Data setup time"]
    #[must_use]
    #[inline(always)]
    pub const fn wds(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Write Data setup time"]
    #[inline(always)]
    pub const fn set_wds(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Write Data hold time"]
    #[must_use]
    #[inline(always)]
    pub const fn wdh(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Write Data hold time"]
    #[inline(always)]
    pub const fn set_wdh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Turnaround time"]
    #[must_use]
    #[inline(always)]
    pub const fn ta(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Turnaround time"]
    #[inline(always)]
    pub const fn set_ta(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Address to write data hold time"]
    #[must_use]
    #[inline(always)]
    pub const fn awdh(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Address to write data hold time"]
    #[inline(always)]
    pub const fn set_awdh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Latency count"]
    #[must_use]
    #[inline(always)]
    pub const fn lc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Latency count"]
    #[inline(always)]
    pub const fn set_lc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Read time"]
    #[must_use]
    #[inline(always)]
    pub const fn rd(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Read time"]
    #[inline(always)]
    pub const fn set_rd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "CE# interval time"]
    #[must_use]
    #[inline(always)]
    pub const fn ceitv(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "CE# interval time"]
    #[inline(always)]
    pub const fn set_ceitv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Read hold time"]
    #[must_use]
    #[inline(always)]
    pub const fn rdh(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Read hold time"]
    #[inline(always)]
    pub const fn set_rdh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Sramcr2 {
    #[inline(always)]
    fn default() -> Sramcr2 {
        Sramcr2(0u64 as u32)
    }
}
impl core::fmt::Debug for Sramcr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sramcr2")
            .field("wds", &self.wds())
            .field("wdh", &self.wdh())
            .field("ta", &self.ta())
            .field("awdh", &self.awdh())
            .field("lc", &self.lc())
            .field("rd", &self.rd())
            .field("ceitv", &self.ceitv())
            .field("rdh", &self.rdh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sramcr2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sramcr2 {
            wds: u8,
            wdh: u8,
            ta: u8,
            awdh: u8,
            lc: u8,
            rd: u8,
            ceitv: u8,
            rdh: u8,
        }
        let proxy = Sramcr2 {
            wds: self.wds(),
            wdh: self.wdh(),
            ta: self.ta(),
            awdh: self.awdh(),
            lc: self.lc(),
            rd: self.rd(),
            ceitv: self.ceitv(),
            rdh: self.rdh(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SRAM Control Register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramcr4(pub u32);
impl Sramcr4 {
    #[doc = "Port Size"]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> super::vals::Sramcr4Ps {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sramcr4Ps::from_bits(val as u8)
    }
    #[doc = "Port Size"]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: super::vals::Sramcr4Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Synchronous Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn syncen(&self) -> super::vals::Sramcr4Syncen {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sramcr4Syncen::from_bits(val as u8)
    }
    #[doc = "Synchronous Mode Enable"]
    #[inline(always)]
    pub const fn set_syncen(&mut self, val: super::vals::Sramcr4Syncen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Wait Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn waiten(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Wait Enable"]
    #[inline(always)]
    pub const fn set_waiten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Wait Sample"]
    #[must_use]
    #[inline(always)]
    pub const fn waitsp(&self) -> super::vals::Sramcr4Waitsp {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Sramcr4Waitsp::from_bits(val as u8)
    }
    #[doc = "Wait Sample"]
    #[inline(always)]
    pub const fn set_waitsp(&mut self, val: super::vals::Sramcr4Waitsp) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Burst Length"]
    #[must_use]
    #[inline(always)]
    pub const fn bl(&self) -> super::vals::Sramcr4Bl {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Sramcr4Bl::from_bits(val as u8)
    }
    #[doc = "Burst Length"]
    #[inline(always)]
    pub const fn set_bl(&mut self, val: super::vals::Sramcr4Bl) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Address Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> super::vals::Sramcr4Am {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Sramcr4Am::from_bits(val as u8)
    }
    #[doc = "Address Mode"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: super::vals::Sramcr4Am) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "ADV# polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn advp(&self) -> super::vals::Sramcr4Advp {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Sramcr4Advp::from_bits(val as u8)
    }
    #[doc = "ADV# polarity"]
    #[inline(always)]
    pub const fn set_advp(&mut self, val: super::vals::Sramcr4Advp) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "ADV# level control during address hold state"]
    #[must_use]
    #[inline(always)]
    pub const fn advh(&self) -> super::vals::Sramcr4Advh {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Sramcr4Advh::from_bits(val as u8)
    }
    #[doc = "ADV# level control during address hold state"]
    #[inline(always)]
    pub const fn set_advh(&mut self, val: super::vals::Sramcr4Advh) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Column Address bit width"]
    #[must_use]
    #[inline(always)]
    pub const fn col(&self) -> super::vals::Sramcr4Col {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Sramcr4Col::from_bits(val as u8)
    }
    #[doc = "Column Address bit width"]
    #[inline(always)]
    pub const fn set_col(&mut self, val: super::vals::Sramcr4Col) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
}
impl Default for Sramcr4 {
    #[inline(always)]
    fn default() -> Sramcr4 {
        Sramcr4(0u64 as u32)
    }
}
impl core::fmt::Debug for Sramcr4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sramcr4")
            .field("ps", &self.ps())
            .field("syncen", &self.syncen())
            .field("waiten", &self.waiten())
            .field("waitsp", &self.waitsp())
            .field("bl", &self.bl())
            .field("am", &self.am())
            .field("advp", &self.advp())
            .field("advh", &self.advh())
            .field("col", &self.col())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sramcr4 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sramcr4 {
            ps: super::vals::Sramcr4Ps,
            syncen: super::vals::Sramcr4Syncen,
            waiten: bool,
            waitsp: super::vals::Sramcr4Waitsp,
            bl: super::vals::Sramcr4Bl,
            am: super::vals::Sramcr4Am,
            advp: super::vals::Sramcr4Advp,
            advh: super::vals::Sramcr4Advh,
            col: super::vals::Sramcr4Col,
        }
        let proxy = Sramcr4 {
            ps: self.ps(),
            syncen: self.syncen(),
            waiten: self.waiten(),
            waitsp: self.waitsp(),
            bl: self.bl(),
            am: self.am(),
            advp: self.advp(),
            advh: self.advh(),
            col: self.col(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SRAM Control Register 5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramcr5(pub u32);
impl Sramcr5 {
    #[doc = "CE setup time"]
    #[must_use]
    #[inline(always)]
    pub const fn ces(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "CE setup time"]
    #[inline(always)]
    pub const fn set_ces(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "CE hold time"]
    #[must_use]
    #[inline(always)]
    pub const fn ceh(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "CE hold time"]
    #[inline(always)]
    pub const fn set_ceh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Address setup time"]
    #[must_use]
    #[inline(always)]
    pub const fn as_(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Address setup time"]
    #[inline(always)]
    pub const fn set_as_(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Address hold time"]
    #[must_use]
    #[inline(always)]
    pub const fn ah(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Address hold time"]
    #[inline(always)]
    pub const fn set_ah(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "WE low time"]
    #[must_use]
    #[inline(always)]
    pub const fn wel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "WE low time"]
    #[inline(always)]
    pub const fn set_wel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "WE high time"]
    #[must_use]
    #[inline(always)]
    pub const fn weh(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "WE high time"]
    #[inline(always)]
    pub const fn set_weh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "RE low time"]
    #[must_use]
    #[inline(always)]
    pub const fn rel(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "RE low time"]
    #[inline(always)]
    pub const fn set_rel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "RE high time"]
    #[must_use]
    #[inline(always)]
    pub const fn reh(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "RE high time"]
    #[inline(always)]
    pub const fn set_reh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Sramcr5 {
    #[inline(always)]
    fn default() -> Sramcr5 {
        Sramcr5(0u64 as u32)
    }
}
impl core::fmt::Debug for Sramcr5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sramcr5")
            .field("ces", &self.ces())
            .field("ceh", &self.ceh())
            .field("as_", &self.as_())
            .field("ah", &self.ah())
            .field("wel", &self.wel())
            .field("weh", &self.weh())
            .field("rel", &self.rel())
            .field("reh", &self.reh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sramcr5 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sramcr5 {
            ces: u8,
            ceh: u8,
            as_: u8,
            ah: u8,
            wel: u8,
            weh: u8,
            rel: u8,
            reh: u8,
        }
        let proxy = Sramcr5 {
            ces: self.ces(),
            ceh: self.ceh(),
            as_: self.as_(),
            ah: self.ah(),
            wel: self.wel(),
            weh: self.weh(),
            rel: self.rel(),
            reh: self.reh(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SRAM Control Register 6"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramcr6(pub u32);
impl Sramcr6 {
    #[doc = "Write Data setup time"]
    #[must_use]
    #[inline(always)]
    pub const fn wds(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Write Data setup time"]
    #[inline(always)]
    pub const fn set_wds(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Write Data hold time"]
    #[must_use]
    #[inline(always)]
    pub const fn wdh(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Write Data hold time"]
    #[inline(always)]
    pub const fn set_wdh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Turnaround time"]
    #[must_use]
    #[inline(always)]
    pub const fn ta(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Turnaround time"]
    #[inline(always)]
    pub const fn set_ta(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Address to write data hold time"]
    #[must_use]
    #[inline(always)]
    pub const fn awdh(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Address to write data hold time"]
    #[inline(always)]
    pub const fn set_awdh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Latency count"]
    #[must_use]
    #[inline(always)]
    pub const fn lc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Latency count"]
    #[inline(always)]
    pub const fn set_lc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Read time"]
    #[must_use]
    #[inline(always)]
    pub const fn rd(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Read time"]
    #[inline(always)]
    pub const fn set_rd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "CE# interval time"]
    #[must_use]
    #[inline(always)]
    pub const fn ceitv(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "CE# interval time"]
    #[inline(always)]
    pub const fn set_ceitv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Read hold time"]
    #[must_use]
    #[inline(always)]
    pub const fn rdh(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Read hold time"]
    #[inline(always)]
    pub const fn set_rdh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Sramcr6 {
    #[inline(always)]
    fn default() -> Sramcr6 {
        Sramcr6(0u64 as u32)
    }
}
impl core::fmt::Debug for Sramcr6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sramcr6")
            .field("wds", &self.wds())
            .field("wdh", &self.wdh())
            .field("ta", &self.ta())
            .field("awdh", &self.awdh())
            .field("lc", &self.lc())
            .field("rd", &self.rd())
            .field("ceitv", &self.ceitv())
            .field("rdh", &self.rdh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sramcr6 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sramcr6 {
            wds: u8,
            wdh: u8,
            ta: u8,
            awdh: u8,
            lc: u8,
            rd: u8,
            ceitv: u8,
            rdh: u8,
        }
        let proxy = Sramcr6 {
            wds: self.wds(),
            wdh: self.wdh(),
            ta: self.ta(),
            awdh: self.awdh(),
            lc: self.lc(),
            rd: self.rd(),
            ceitv: self.ceitv(),
            rdh: self.rdh(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Status Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sts0(pub u32);
impl Sts0 {
    #[doc = "Indicating whether the SEMC is in idle state."]
    #[must_use]
    #[inline(always)]
    pub const fn idle(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indicating whether the SEMC is in idle state."]
    #[inline(always)]
    pub const fn set_idle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicating NAND device Ready/WAIT# pin level."]
    #[must_use]
    #[inline(always)]
    pub const fn nardy(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Indicating NAND device Ready/WAIT# pin level."]
    #[inline(always)]
    pub const fn set_nardy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Sts0 {
    #[inline(always)]
    fn default() -> Sts0 {
        Sts0(1u64 as u32)
    }
}
impl core::fmt::Debug for Sts0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sts0")
            .field("idle", &self.idle())
            .field("nardy", &self.nardy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sts0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sts0 {
            idle: bool,
            nardy: bool,
        }
        let proxy = Sts0 {
            idle: self.idle(),
            nardy: self.nardy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Status Register 13"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sts13(pub u32);
impl Sts13 {
    #[doc = "Sample clock slave delay line locked."]
    #[must_use]
    #[inline(always)]
    pub const fn slvlock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Sample clock slave delay line locked."]
    #[inline(always)]
    pub const fn set_slvlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Sample clock reference delay line locked."]
    #[must_use]
    #[inline(always)]
    pub const fn reflock(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Sample clock reference delay line locked."]
    #[inline(always)]
    pub const fn set_reflock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Sample clock slave delay line delay cell number selection."]
    #[must_use]
    #[inline(always)]
    pub const fn slvsel(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[doc = "Sample clock slave delay line delay cell number selection."]
    #[inline(always)]
    pub const fn set_slvsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u32) & 0x3f) << 2usize);
    }
    #[doc = "Sample clock reference delay line delay cell number selection."]
    #[must_use]
    #[inline(always)]
    pub const fn refsel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Sample clock reference delay line delay cell number selection."]
    #[inline(always)]
    pub const fn set_refsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
}
impl Default for Sts13 {
    #[inline(always)]
    fn default() -> Sts13 {
        Sts13(256u64 as u32)
    }
}
impl core::fmt::Debug for Sts13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sts13")
            .field("slvlock", &self.slvlock())
            .field("reflock", &self.reflock())
            .field("slvsel", &self.slvsel())
            .field("refsel", &self.refsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sts13 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sts13 {
            slvlock: bool,
            reflock: bool,
            slvsel: u8,
            refsel: u8,
        }
        let proxy = Sts13 {
            slvlock: self.slvlock(),
            reflock: self.reflock(),
            slvsel: self.slvsel(),
            refsel: self.refsel(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Status Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sts2(pub u32);
impl Sts2 {
    #[doc = "This field indicating whether there is pending AXI command (write) to NAND device."]
    #[must_use]
    #[inline(always)]
    pub const fn ndwrpend(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This field indicating whether there is pending AXI command (write) to NAND device."]
    #[inline(always)]
    pub const fn set_ndwrpend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Sts2 {
    #[inline(always)]
    fn default() -> Sts2 {
        Sts2(0u64 as u32)
    }
}
impl core::fmt::Debug for Sts2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sts2")
            .field("ndwrpend", &self.ndwrpend())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sts2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sts2 {
            ndwrpend: bool,
        }
        let proxy = Sts2 {
            ndwrpend: self.ndwrpend(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
