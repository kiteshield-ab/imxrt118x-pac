#[doc = "IEE AES Version ID Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aesvid(pub u32);
impl Aesvid {
    #[doc = "AES revision number."]
    #[must_use]
    #[inline(always)]
    pub const fn aesrn(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "AES revision number."]
    #[inline(always)]
    pub const fn set_aesrn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "AES version ID."]
    #[must_use]
    #[inline(always)]
    pub const fn aesvid(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "AES version ID."]
    #[inline(always)]
    pub const fn set_aesvid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Aesvid {
    #[inline(always)]
    fn default() -> Aesvid {
        Aesvid(32u64 as u32)
    }
}
impl core::fmt::Debug for Aesvid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Aesvid")
            .field("aesrn", &self.aesrn())
            .field("aesvid", &self.aesvid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Aesvid {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Aesvid {
            aesrn: u8,
            aesvid: u8,
        }
        let proxy = Aesvid {
            aesrn: self.aesrn(),
            aesvid: self.aesvid(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "IEE Global Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gcfg(pub u32);
impl Gcfg {
    #[doc = "Region lock 0 bit"]
    #[must_use]
    #[inline(always)]
    pub const fn rl0(&self) -> super::vals::Rl0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Rl0::from_bits(val as u8)
    }
    #[doc = "Region lock 0 bit"]
    #[inline(always)]
    pub const fn set_rl0(&mut self, val: super::vals::Rl0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Region lock 1 bit"]
    #[must_use]
    #[inline(always)]
    pub const fn rl1(&self) -> super::vals::Rl1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Rl1::from_bits(val as u8)
    }
    #[doc = "Region lock 1 bit"]
    #[inline(always)]
    pub const fn set_rl1(&mut self, val: super::vals::Rl1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Region lock 2 bit"]
    #[must_use]
    #[inline(always)]
    pub const fn rl2(&self) -> super::vals::Rl2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Rl2::from_bits(val as u8)
    }
    #[doc = "Region lock 2 bit"]
    #[inline(always)]
    pub const fn set_rl2(&mut self, val: super::vals::Rl2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Region lock 3 bit"]
    #[must_use]
    #[inline(always)]
    pub const fn rl3(&self) -> super::vals::Rl3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Rl3::from_bits(val as u8)
    }
    #[doc = "Region lock 3 bit"]
    #[inline(always)]
    pub const fn set_rl3(&mut self, val: super::vals::Rl3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Region lock 4 bit"]
    #[must_use]
    #[inline(always)]
    pub const fn rl4(&self) -> super::vals::Rl4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Rl4::from_bits(val as u8)
    }
    #[doc = "Region lock 4 bit"]
    #[inline(always)]
    pub const fn set_rl4(&mut self, val: super::vals::Rl4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Region lock 5 bit"]
    #[must_use]
    #[inline(always)]
    pub const fn rl5(&self) -> super::vals::Rl5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Rl5::from_bits(val as u8)
    }
    #[doc = "Region lock 5 bit"]
    #[inline(always)]
    pub const fn set_rl5(&mut self, val: super::vals::Rl5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Region lock 6 bit"]
    #[must_use]
    #[inline(always)]
    pub const fn rl6(&self) -> super::vals::Rl6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Rl6::from_bits(val as u8)
    }
    #[doc = "Region lock 6 bit"]
    #[inline(always)]
    pub const fn set_rl6(&mut self, val: super::vals::Rl6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Region lock 7 bit"]
    #[must_use]
    #[inline(always)]
    pub const fn rl7(&self) -> super::vals::Rl7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Rl7::from_bits(val as u8)
    }
    #[doc = "Region lock 7 bit"]
    #[inline(always)]
    pub const fn set_rl7(&mut self, val: super::vals::Rl7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Test mode enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn tme(&self) -> super::vals::Tme {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Tme::from_bits(val as u8)
    }
    #[doc = "Test mode enable bit"]
    #[inline(always)]
    pub const fn set_tme(&mut self, val: super::vals::Tme) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Test mode disable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn tmd(&self) -> super::vals::Tmd {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Tmd::from_bits(val as u8)
    }
    #[doc = "Test mode disable bit"]
    #[inline(always)]
    pub const fn set_tmd(&mut self, val: super::vals::Tmd) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Key read disable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn key_rd_dis(&self) -> super::vals::KeyRdDis {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::KeyRdDis::from_bits(val as u8)
    }
    #[doc = "Key read disable bit"]
    #[inline(always)]
    pub const fn set_key_rd_dis(&mut self, val: super::vals::KeyRdDis) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Monitor enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn mon_en(&self) -> super::vals::MonEn {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::MonEn::from_bits(val as u8)
    }
    #[doc = "Monitor enable bit"]
    #[inline(always)]
    pub const fn set_mon_en(&mut self, val: super::vals::MonEn) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Clear monitor bit"]
    #[must_use]
    #[inline(always)]
    pub const fn clr_mon(&self) -> super::vals::ClrMon {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::ClrMon::from_bits(val as u8)
    }
    #[doc = "Clear monitor bit"]
    #[inline(always)]
    pub const fn set_clr_mon(&mut self, val: super::vals::ClrMon) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Reset bit"]
    #[must_use]
    #[inline(always)]
    pub const fn rst(&self) -> super::vals::Rst {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Rst::from_bits(val as u8)
    }
    #[doc = "Reset bit"]
    #[inline(always)]
    pub const fn set_rst(&mut self, val: super::vals::Rst) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Gcfg {
    #[inline(always)]
    fn default() -> Gcfg {
        Gcfg(33554432u64 as u32)
    }
}
impl core::fmt::Debug for Gcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gcfg")
            .field("rl0", &self.rl0())
            .field("rl1", &self.rl1())
            .field("rl2", &self.rl2())
            .field("rl3", &self.rl3())
            .field("rl4", &self.rl4())
            .field("rl5", &self.rl5())
            .field("rl6", &self.rl6())
            .field("rl7", &self.rl7())
            .field("tme", &self.tme())
            .field("tmd", &self.tmd())
            .field("key_rd_dis", &self.key_rd_dis())
            .field("mon_en", &self.mon_en())
            .field("clr_mon", &self.clr_mon())
            .field("rst", &self.rst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gcfg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gcfg {
            rl0: super::vals::Rl0,
            rl1: super::vals::Rl1,
            rl2: super::vals::Rl2,
            rl3: super::vals::Rl3,
            rl4: super::vals::Rl4,
            rl5: super::vals::Rl5,
            rl6: super::vals::Rl6,
            rl7: super::vals::Rl7,
            tme: super::vals::Tme,
            tmd: super::vals::Tmd,
            key_rd_dis: super::vals::KeyRdDis,
            mon_en: super::vals::MonEn,
            clr_mon: super::vals::ClrMon,
            rst: super::vals::Rst,
        }
        let proxy = Gcfg {
            rl0: self.rl0(),
            rl1: self.rl1(),
            rl2: self.rl2(),
            rl3: self.rl3(),
            rl4: self.rl4(),
            rl5: self.rl5(),
            rl6: self.rl6(),
            rl7: self.rl7(),
            tme: self.tme(),
            tmd: self.tmd(),
            key_rd_dis: self.key_rd_dis(),
            mon_en: self.mon_en(),
            clr_mon: self.clr_mon(),
            rst: self.rst(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Performance Counter, AES Master Latency Threshold"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PcMLt(pub u32);
impl PcMLt {
    #[doc = "Master write latency threshold in AXI clock cycles."]
    #[must_use]
    #[inline(always)]
    pub const fn mw_lt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Master write latency threshold in AXI clock cycles."]
    #[inline(always)]
    pub const fn set_mw_lt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Master read latency threshold in AXI clock cycles."]
    #[must_use]
    #[inline(always)]
    pub const fn mr_lt(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Master read latency threshold in AXI clock cycles."]
    #[inline(always)]
    pub const fn set_mr_lt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for PcMLt {
    #[inline(always)]
    fn default() -> PcMLt {
        PcMLt(0u64 as u32)
    }
}
impl core::fmt::Debug for PcMLt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PcMLt")
            .field("mw_lt", &self.mw_lt())
            .field("mr_lt", &self.mr_lt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PcMLt {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PcMLt {
            mw_lt: u16,
            mr_lt: u16,
        }
        let proxy = PcMLt {
            mw_lt: self.mw_lt(),
            mr_lt: self.mr_lt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Performance Counter, Lower Master Read Transactions Byte Count"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PcMrTbcL(pub u32);
impl PcMrTbcL {
    #[doc = "Number of bytes in master read transactions. 4 LSBs, always 0."]
    #[must_use]
    #[inline(always)]
    pub const fn mr_tbc_lsb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of bytes in master read transactions. 4 LSBs, always 0."]
    #[inline(always)]
    pub const fn set_mr_tbc_lsb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Number of bytes in master read transactions. 44 MSBs. Lower 28 bits of MR_TBC\\[43:0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn mr_tbc(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "Number of bytes in master read transactions. 44 MSBs. Lower 28 bits of MR_TBC\\[43:0\\]."]
    #[inline(always)]
    pub const fn set_mr_tbc(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
    }
}
impl Default for PcMrTbcL {
    #[inline(always)]
    fn default() -> PcMrTbcL {
        PcMrTbcL(0u64 as u32)
    }
}
impl core::fmt::Debug for PcMrTbcL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PcMrTbcL")
            .field("mr_tbc_lsb", &self.mr_tbc_lsb())
            .field("mr_tbc", &self.mr_tbc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PcMrTbcL {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PcMrTbcL {
            mr_tbc_lsb: u8,
            mr_tbc: u32,
        }
        let proxy = PcMrTbcL {
            mr_tbc_lsb: self.mr_tbc_lsb(),
            mr_tbc: self.mr_tbc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Performance Counter, Upper Master Read Transactions Byte Count"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PcMrTbcU(pub u32);
impl PcMrTbcU {
    #[doc = "Number of bytes in master read transactions. 44 MSBs. Upper 16 bits of MR_TBC\\[43:0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn mr_tbc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of bytes in master read transactions. 44 MSBs. Upper 16 bits of MR_TBC\\[43:0\\]."]
    #[inline(always)]
    pub const fn set_mr_tbc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for PcMrTbcU {
    #[inline(always)]
    fn default() -> PcMrTbcU {
        PcMrTbcU(0u64 as u32)
    }
}
impl core::fmt::Debug for PcMrTbcU {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PcMrTbcU")
            .field("mr_tbc", &self.mr_tbc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PcMrTbcU {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PcMrTbcU {
            mr_tbc: u16,
        }
        let proxy = PcMrTbcU {
            mr_tbc: self.mr_tbc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Performance Counter, Upper Master Read Latency Count"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PcMrTlatU(pub u32);
impl PcMrTlatU {
    #[doc = "Total master read latency in AXI clock cycles. Upper 16 bits of MR_TLAT\\[47:0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn mr_tlat(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Total master read latency in AXI clock cycles. Upper 16 bits of MR_TLAT\\[47:0\\]."]
    #[inline(always)]
    pub const fn set_mr_tlat(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for PcMrTlatU {
    #[inline(always)]
    fn default() -> PcMrTlatU {
        PcMrTlatU(0u64 as u32)
    }
}
impl core::fmt::Debug for PcMrTlatU {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PcMrTlatU")
            .field("mr_tlat", &self.mr_tlat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PcMrTlatU {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PcMrTlatU {
            mr_tlat: u16,
        }
        let proxy = PcMrTlatU {
            mr_tlat: self.mr_tlat(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Performance Counter, Lower Master Write Transactions Byte Count"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PcMwTbcL(pub u32);
impl PcMwTbcL {
    #[doc = "Number of bytes in master write transactions. 4 LSBs, always 0."]
    #[must_use]
    #[inline(always)]
    pub const fn mw_tbc_lsb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of bytes in master write transactions. 4 LSBs, always 0."]
    #[inline(always)]
    pub const fn set_mw_tbc_lsb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Number of bytes in master write transactions. 44 MSBs. Lower 28 bits of MR_TBC\\[43:0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn mw_tbc(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "Number of bytes in master write transactions. 44 MSBs. Lower 28 bits of MR_TBC\\[43:0\\]."]
    #[inline(always)]
    pub const fn set_mw_tbc(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
    }
}
impl Default for PcMwTbcL {
    #[inline(always)]
    fn default() -> PcMwTbcL {
        PcMwTbcL(0u64 as u32)
    }
}
impl core::fmt::Debug for PcMwTbcL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PcMwTbcL")
            .field("mw_tbc_lsb", &self.mw_tbc_lsb())
            .field("mw_tbc", &self.mw_tbc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PcMwTbcL {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PcMwTbcL {
            mw_tbc_lsb: u8,
            mw_tbc: u32,
        }
        let proxy = PcMwTbcL {
            mw_tbc_lsb: self.mw_tbc_lsb(),
            mw_tbc: self.mw_tbc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Performance Counter, Upper Master Write Transactions Byte Count"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PcMwTbcU(pub u32);
impl PcMwTbcU {
    #[doc = "Number of bytes in master write transactions. 44 MSBs. Upper 16 bits of MW_TBC\\[43:0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn mw_tbc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of bytes in master write transactions. 44 MSBs. Upper 16 bits of MW_TBC\\[43:0\\]."]
    #[inline(always)]
    pub const fn set_mw_tbc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for PcMwTbcU {
    #[inline(always)]
    fn default() -> PcMwTbcU {
        PcMwTbcU(0u64 as u32)
    }
}
impl core::fmt::Debug for PcMwTbcU {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PcMwTbcU")
            .field("mw_tbc", &self.mw_tbc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PcMwTbcU {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PcMwTbcU {
            mw_tbc: u16,
        }
        let proxy = PcMwTbcU {
            mw_tbc: self.mw_tbc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Performance Counter, Upper Master Write Latency Count"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PcMwTlatU(pub u32);
impl PcMwTlatU {
    #[doc = "Total master write latency in AXI clock cycles. Upper 16 bits of MW_TLAT\\[47:0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn mw_tlat(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Total master write latency in AXI clock cycles. Upper 16 bits of MW_TLAT\\[47:0\\]."]
    #[inline(always)]
    pub const fn set_mw_tlat(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for PcMwTlatU {
    #[inline(always)]
    fn default() -> PcMwTlatU {
        PcMwTlatU(0u64 as u32)
    }
}
impl core::fmt::Debug for PcMwTlatU {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PcMwTlatU")
            .field("mw_tlat", &self.mw_tlat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PcMwTlatU {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PcMwTlatU {
            mw_tlat: u16,
        }
        let proxy = PcMwTlatU {
            mw_tlat: self.mw_tlat(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Performance Counter, AES Slave Latency Threshold Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PcSLt(pub u32);
impl PcSLt {
    #[doc = "Slave write latency threshold in AXI clock cycles."]
    #[must_use]
    #[inline(always)]
    pub const fn sw_lt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Slave write latency threshold in AXI clock cycles."]
    #[inline(always)]
    pub const fn set_sw_lt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Slave read latency threshold in AXI clock cycles."]
    #[must_use]
    #[inline(always)]
    pub const fn sr_lt(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Slave read latency threshold in AXI clock cycles."]
    #[inline(always)]
    pub const fn set_sr_lt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for PcSLt {
    #[inline(always)]
    fn default() -> PcSLt {
        PcSLt(0u64 as u32)
    }
}
impl core::fmt::Debug for PcSLt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PcSLt")
            .field("sw_lt", &self.sw_lt())
            .field("sr_lt", &self.sr_lt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PcSLt {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PcSLt {
            sw_lt: u16,
            sr_lt: u16,
        }
        let proxy = PcSLt {
            sw_lt: self.sw_lt(),
            sr_lt: self.sr_lt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Performance Counter, Upper Slave Read Transactions Byte Count"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PcSrTbcU(pub u32);
impl PcSrTbcU {
    #[doc = "Number of bytes in slave read transactions. Upper 16 bits of SR_TBC\\[47:0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn sr_tbc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of bytes in slave read transactions. Upper 16 bits of SR_TBC\\[47:0\\]."]
    #[inline(always)]
    pub const fn set_sr_tbc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for PcSrTbcU {
    #[inline(always)]
    fn default() -> PcSrTbcU {
        PcSrTbcU(0u64 as u32)
    }
}
impl core::fmt::Debug for PcSrTbcU {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PcSrTbcU")
            .field("sr_tbc", &self.sr_tbc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PcSrTbcU {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PcSrTbcU {
            sr_tbc: u16,
        }
        let proxy = PcSrTbcU {
            sr_tbc: self.sr_tbc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Performance Counter, Upper Slave Read Latency Count"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PcSrTlatU(pub u32);
impl PcSrTlatU {
    #[doc = "Total slave read latency in AXI clock cycles. Upper 16 bits of SR_TLAT\\[47:0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn sr_tlat(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Total slave read latency in AXI clock cycles. Upper 16 bits of SR_TLAT\\[47:0\\]."]
    #[inline(always)]
    pub const fn set_sr_tlat(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for PcSrTlatU {
    #[inline(always)]
    fn default() -> PcSrTlatU {
        PcSrTlatU(0u64 as u32)
    }
}
impl core::fmt::Debug for PcSrTlatU {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PcSrTlatU")
            .field("sr_tlat", &self.sr_tlat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PcSrTlatU {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PcSrTlatU {
            sr_tlat: u16,
        }
        let proxy = PcSrTlatU {
            sr_tlat: self.sr_tlat(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Performance Counter, Upper Slave Read Total Non-Responding Time"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PcSrTnrtU(pub u32);
impl PcSrTnrtU {
    #[doc = "Total slave read non-responding time in AXI clock cycles. Upper 16 bits of SR_TNRT\\[47:0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn sr_tnrt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Total slave read non-responding time in AXI clock cycles. Upper 16 bits of SR_TNRT\\[47:0\\]."]
    #[inline(always)]
    pub const fn set_sr_tnrt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for PcSrTnrtU {
    #[inline(always)]
    fn default() -> PcSrTnrtU {
        PcSrTnrtU(0u64 as u32)
    }
}
impl core::fmt::Debug for PcSrTnrtU {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PcSrTnrtU")
            .field("sr_tnrt", &self.sr_tnrt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PcSrTnrtU {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PcSrTnrtU {
            sr_tnrt: u16,
        }
        let proxy = PcSrTnrtU {
            sr_tnrt: self.sr_tnrt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Performance Counter, Upper Slave Write Transactions Byte Count"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PcSwTbcU(pub u32);
impl PcSwTbcU {
    #[doc = "Number of bytes in slave write transactions. Upper 16 bits of SW_TBC\\[47:0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn sw_tbc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of bytes in slave write transactions. Upper 16 bits of SW_TBC\\[47:0\\]."]
    #[inline(always)]
    pub const fn set_sw_tbc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for PcSwTbcU {
    #[inline(always)]
    fn default() -> PcSwTbcU {
        PcSwTbcU(0u64 as u32)
    }
}
impl core::fmt::Debug for PcSwTbcU {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PcSwTbcU")
            .field("sw_tbc", &self.sw_tbc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PcSwTbcU {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PcSwTbcU {
            sw_tbc: u16,
        }
        let proxy = PcSwTbcU {
            sw_tbc: self.sw_tbc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Performance Counter, Upper Slave Write Latency Count"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PcSwTlatU(pub u32);
impl PcSwTlatU {
    #[doc = "Total slave write latency in AXI clock cycles. Upper 16 bits of SW_TLAT\\[47:0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn sw_tlat(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Total slave write latency in AXI clock cycles. Upper 16 bits of SW_TLAT\\[47:0\\]."]
    #[inline(always)]
    pub const fn set_sw_tlat(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for PcSwTlatU {
    #[inline(always)]
    fn default() -> PcSwTlatU {
        PcSwTlatU(0u64 as u32)
    }
}
impl core::fmt::Debug for PcSwTlatU {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PcSwTlatU")
            .field("sw_tlat", &self.sw_tlat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PcSwTlatU {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PcSwTlatU {
            sw_tlat: u16,
        }
        let proxy = PcSwTlatU {
            sw_tlat: self.sw_tlat(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Performance Counter, Upper Slave Write Total Non-Responding Time"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PcSwTnrtU(pub u32);
impl PcSwTnrtU {
    #[doc = "Total slave write non-responding time in AXI clock cycles. Upper 16 bits of SW_TNRT\\[47:0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn sw_tnrt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Total slave write non-responding time in AXI clock cycles. Upper 16 bits of SW_TNRT\\[47:0\\]."]
    #[inline(always)]
    pub const fn set_sw_tnrt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for PcSwTnrtU {
    #[inline(always)]
    fn default() -> PcSwTnrtU {
        PcSwTnrtU(0u64 as u32)
    }
}
impl core::fmt::Debug for PcSwTnrtU {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PcSwTnrtU")
            .field("sw_tnrt", &self.sw_tnrt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PcSwTnrtU {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PcSwTnrtU {
            sw_tnrt: u16,
        }
        let proxy = PcSwTnrtU {
            sw_tnrt: self.sw_tnrt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "IEE Region REGION Attribute Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Regattr(pub u32);
impl Regattr {
    #[doc = "AES key size."]
    #[must_use]
    #[inline(always)]
    pub const fn ks(&self) -> super::vals::Ks {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ks::from_bits(val as u8)
    }
    #[doc = "AES key size."]
    #[inline(always)]
    pub const fn set_ks(&mut self, val: super::vals::Ks) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "AES Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn md(&self) -> super::vals::Md {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Md::from_bits(val as u8)
    }
    #[doc = "AES Mode."]
    #[inline(always)]
    pub const fn set_md(&mut self, val: super::vals::Md) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "AES Bypass."]
    #[must_use]
    #[inline(always)]
    pub const fn byp(&self) -> super::vals::Byp {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Byp::from_bits(val as u8)
    }
    #[doc = "AES Bypass."]
    #[inline(always)]
    pub const fn set_byp(&mut self, val: super::vals::Byp) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
}
impl Default for Regattr {
    #[inline(always)]
    fn default() -> Regattr {
        Regattr(0u64 as u32)
    }
}
impl core::fmt::Debug for Regattr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Regattr")
            .field("ks", &self.ks())
            .field("md", &self.md())
            .field("byp", &self.byp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Regattr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Regattr {
            ks: super::vals::Ks,
            md: super::vals::Md,
            byp: super::vals::Byp,
        }
        let proxy = Regattr {
            ks: self.ks(),
            md: self.md(),
            byp: self.byp(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "IEE Region REGION Page Offset Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Regpo(pub u32);
impl Regpo {
    #[doc = "This field represents a 4Kb page offset"]
    #[must_use]
    #[inline(always)]
    pub const fn pgoff(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "This field represents a 4Kb page offset"]
    #[inline(always)]
    pub const fn set_pgoff(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Regpo {
    #[inline(always)]
    fn default() -> Regpo {
        Regpo(0u64 as u32)
    }
}
impl core::fmt::Debug for Regpo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Regpo")
            .field("pgoff", &self.pgoff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Regpo {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Regpo {
            pgoff: u32,
        }
        let proxy = Regpo {
            pgoff: self.pgoff(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "IEE Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sta(pub u32);
impl Sta {
    #[doc = "DPA seed request bit"]
    #[must_use]
    #[inline(always)]
    pub const fn dsr(&self) -> super::vals::Dsr {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Dsr::from_bits(val as u8)
    }
    #[doc = "DPA seed request bit"]
    #[inline(always)]
    pub const fn set_dsr(&mut self, val: super::vals::Dsr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "AES fault detected bit"]
    #[must_use]
    #[inline(always)]
    pub const fn afd(&self) -> super::vals::Afd {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Afd::from_bits(val as u8)
    }
    #[doc = "AES fault detected bit"]
    #[inline(always)]
    pub const fn set_afd(&mut self, val: super::vals::Afd) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
}
impl Default for Sta {
    #[inline(always)]
    fn default() -> Sta {
        Sta(1u64 as u32)
    }
}
impl core::fmt::Debug for Sta {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sta")
            .field("dsr", &self.dsr())
            .field("afd", &self.afd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sta {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sta {
            dsr: super::vals::Dsr,
            afd: super::vals::Afd,
        }
        let proxy = Sta {
            dsr: self.dsr(),
            afd: self.afd(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "IEE Test Mode Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tstmd(pub u32);
impl Tstmd {
    #[doc = "Test mode ready bit. All AXI transactions have stopped and test can begin."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrdy(&self) -> super::vals::Tmrdy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Tmrdy::from_bits(val as u8)
    }
    #[doc = "Test mode ready bit. All AXI transactions have stopped and test can begin."]
    #[inline(always)]
    pub const fn set_tmrdy(&mut self, val: super::vals::Tmrdy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Test mode run bit"]
    #[must_use]
    #[inline(always)]
    pub const fn tmr(&self) -> super::vals::Tmr {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Tmr::from_bits(val as u8)
    }
    #[doc = "Test mode run bit"]
    #[inline(always)]
    pub const fn set_tmr(&mut self, val: super::vals::Tmr) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Test mode encrypt/decrypt bit."]
    #[must_use]
    #[inline(always)]
    pub const fn tmencr(&self) -> super::vals::Tmencr {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Tmencr::from_bits(val as u8)
    }
    #[doc = "Test mode encrypt/decrypt bit."]
    #[inline(always)]
    pub const fn set_tmencr(&mut self, val: super::vals::Tmencr) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Test mode continue bit. Set to indicate that operation will be followed by more data."]
    #[must_use]
    #[inline(always)]
    pub const fn tmcont(&self) -> super::vals::Tmcont {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Tmcont::from_bits(val as u8)
    }
    #[doc = "Test mode continue bit. Set to indicate that operation will be followed by more data."]
    #[inline(always)]
    pub const fn set_tmcont(&mut self, val: super::vals::Tmcont) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Test mode done bit"]
    #[must_use]
    #[inline(always)]
    pub const fn tmdone(&self) -> super::vals::Tmdone {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Tmdone::from_bits(val as u8)
    }
    #[doc = "Test mode done bit"]
    #[inline(always)]
    pub const fn set_tmdone(&mut self, val: super::vals::Tmdone) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Test mode length field"]
    #[must_use]
    #[inline(always)]
    pub const fn tmlen(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Test mode length field"]
    #[inline(always)]
    pub const fn set_tmlen(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
}
impl Default for Tstmd {
    #[inline(always)]
    fn default() -> Tstmd {
        Tstmd(0u64 as u32)
    }
}
impl core::fmt::Debug for Tstmd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tstmd")
            .field("tmrdy", &self.tmrdy())
            .field("tmr", &self.tmr())
            .field("tmencr", &self.tmencr())
            .field("tmcont", &self.tmcont())
            .field("tmdone", &self.tmdone())
            .field("tmlen", &self.tmlen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tstmd {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Tstmd {
            tmrdy: super::vals::Tmrdy,
            tmr: super::vals::Tmr,
            tmencr: super::vals::Tmencr,
            tmcont: super::vals::Tmcont,
            tmdone: super::vals::Tmdone,
            tmlen: u8,
        }
        let proxy = Tstmd {
            tmrdy: self.tmrdy(),
            tmr: self.tmr(),
            tmencr: self.tmencr(),
            tmcont: self.tmcont(),
            tmdone: self.tmdone(),
            tmlen: self.tmlen(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "IEE Version ID Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vidr1(pub u32);
impl Vidr1 {
    #[doc = "Minor revision number for IEE."]
    #[must_use]
    #[inline(always)]
    pub const fn min_rev(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Minor revision number for IEE."]
    #[inline(always)]
    pub const fn set_min_rev(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Major revision number for IEE."]
    #[must_use]
    #[inline(always)]
    pub const fn maj_rev(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Major revision number for IEE."]
    #[inline(always)]
    pub const fn set_maj_rev(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "ID for IEE."]
    #[must_use]
    #[inline(always)]
    pub const fn ip_id(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "ID for IEE."]
    #[inline(always)]
    pub const fn set_ip_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Vidr1 {
    #[inline(always)]
    fn default() -> Vidr1 {
        Vidr1(3408130u64 as u32)
    }
}
impl core::fmt::Debug for Vidr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vidr1")
            .field("min_rev", &self.min_rev())
            .field("maj_rev", &self.maj_rev())
            .field("ip_id", &self.ip_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vidr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Vidr1 {
            min_rev: u8,
            maj_rev: u8,
            ip_id: u16,
        }
        let proxy = Vidr1 {
            min_rev: self.min_rev(),
            maj_rev: self.maj_rev(),
            ip_id: self.ip_id(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
