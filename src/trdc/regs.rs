#[doc = "MBC Memory Block Configuration Word"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BlkCfg(pub u32);
impl BlkCfg {
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel0(&self) -> super::vals::Mbacsel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Mbacsel::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel0(&mut self, val: super::vals::Mbacsel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel1(&self) -> super::vals::Mbacsel {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Mbacsel::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel1(&mut self, val: super::vals::Mbacsel) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel2(&self) -> super::vals::Mbacsel {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Mbacsel::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel2(&mut self, val: super::vals::Mbacsel) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse2(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel3(&self) -> super::vals::Mbacsel {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Mbacsel::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel3(&mut self, val: super::vals::Mbacsel) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel4(&self) -> super::vals::Mbacsel {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Mbacsel::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel4(&mut self, val: super::vals::Mbacsel) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse4(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel5(&self) -> super::vals::Mbacsel {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Mbacsel::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel5(&mut self, val: super::vals::Mbacsel) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse5(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel6(&self) -> super::vals::Mbacsel {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Mbacsel::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel6(&mut self, val: super::vals::Mbacsel) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse6(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel7(&self) -> super::vals::Mbacsel {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Mbacsel::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel7(&mut self, val: super::vals::Mbacsel) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse7(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for BlkCfg {
    #[inline(always)]
    fn default() -> BlkCfg {
        BlkCfg(0u64 as u32)
    }
}
impl core::fmt::Debug for BlkCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BlkCfg")
            .field("mbacsel0", &self.mbacsel0())
            .field("nse0", &self.nse0())
            .field("mbacsel1", &self.mbacsel1())
            .field("nse1", &self.nse1())
            .field("mbacsel2", &self.mbacsel2())
            .field("nse2", &self.nse2())
            .field("mbacsel3", &self.mbacsel3())
            .field("nse3", &self.nse3())
            .field("mbacsel4", &self.mbacsel4())
            .field("nse4", &self.nse4())
            .field("mbacsel5", &self.mbacsel5())
            .field("nse5", &self.nse5())
            .field("mbacsel6", &self.mbacsel6())
            .field("nse6", &self.nse6())
            .field("mbacsel7", &self.mbacsel7())
            .field("nse7", &self.nse7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BlkCfg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct BlkCfg {
            mbacsel0: super::vals::Mbacsel,
            nse0: bool,
            mbacsel1: super::vals::Mbacsel,
            nse1: bool,
            mbacsel2: super::vals::Mbacsel,
            nse2: bool,
            mbacsel3: super::vals::Mbacsel,
            nse3: bool,
            mbacsel4: super::vals::Mbacsel,
            nse4: bool,
            mbacsel5: super::vals::Mbacsel,
            nse5: bool,
            mbacsel6: super::vals::Mbacsel,
            nse6: bool,
            mbacsel7: super::vals::Mbacsel,
            nse7: bool,
        }
        let proxy = BlkCfg {
            mbacsel0: self.mbacsel0(),
            nse0: self.nse0(),
            mbacsel1: self.mbacsel1(),
            nse1: self.nse1(),
            mbacsel2: self.mbacsel2(),
            nse2: self.nse2(),
            mbacsel3: self.mbacsel3(),
            nse3: self.nse3(),
            mbacsel4: self.mbacsel4(),
            nse4: self.nse4(),
            mbacsel5: self.mbacsel5(),
            nse5: self.nse5(),
            mbacsel6: self.mbacsel6(),
            nse6: self.nse6(),
            mbacsel7: self.mbacsel7(),
            nse7: self.nse7(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Domain Assignment Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dacfg(pub u8);
impl Dacfg {
    #[doc = "Number of master domain assignment registers for bus master m"]
    #[must_use]
    #[inline(always)]
    pub const fn nmdar(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of master domain assignment registers for bus master m"]
    #[inline(always)]
    pub const fn set_nmdar(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Non-CPU Master"]
    #[must_use]
    #[inline(always)]
    pub const fn ncm(&self) -> super::vals::Ncm {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Ncm::from_bits(val as u8)
    }
    #[doc = "Non-CPU Master"]
    #[inline(always)]
    pub const fn set_ncm(&mut self, val: super::vals::Ncm) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Dacfg {
    #[inline(always)]
    fn default() -> Dacfg {
        Dacfg(1u64 as u8)
    }
}
impl core::fmt::Debug for Dacfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dacfg")
            .field("nmdar", &self.nmdar())
            .field("ncm", &self.ncm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dacfg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Dacfg {
            nmdar: u8,
            ncm: super::vals::Ncm,
        }
        let proxy = Dacfg {
            nmdar: self.nmdar(),
            ncm: self.ncm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "DAC Master Domain Assignment Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dfmt0(pub u32);
impl Dfmt0 {
    #[doc = "Domain identifier"]
    #[must_use]
    #[inline(always)]
    pub const fn did(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain identifier"]
    #[inline(always)]
    pub const fn set_did(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "DID Select"]
    #[must_use]
    #[inline(always)]
    pub const fn dids(&self) -> super::vals::Dids {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Dids::from_bits(val as u8)
    }
    #[doc = "DID Select"]
    #[inline(always)]
    pub const fn set_dids(&mut self, val: super::vals::Dids) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Process identifier enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> super::vals::Pe {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Pe::from_bits(val as u8)
    }
    #[doc = "Process identifier enable"]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: super::vals::Pe) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Process Identifier Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn pidm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Process Identifier Mask"]
    #[inline(always)]
    pub const fn set_pidm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Secure attribute"]
    #[must_use]
    #[inline(always)]
    pub const fn sa(&self) -> super::vals::Sa {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Sa::from_bits(val as u8)
    }
    #[doc = "Secure attribute"]
    #[inline(always)]
    pub const fn set_sa(&mut self, val: super::vals::Sa) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Process Identifier"]
    #[must_use]
    #[inline(always)]
    pub const fn pid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Process Identifier"]
    #[inline(always)]
    pub const fn set_pid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Domain format"]
    #[must_use]
    #[inline(always)]
    pub const fn dfmt(&self) -> super::vals::Dfmt {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Dfmt::from_bits(val as u8)
    }
    #[doc = "Domain format"]
    #[inline(always)]
    pub const fn set_dfmt(&mut self, val: super::vals::Dfmt) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "1-bit Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lk1(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "1-bit Lock"]
    #[inline(always)]
    pub const fn set_lk1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn vld(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Valid"]
    #[inline(always)]
    pub const fn set_vld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dfmt0 {
    #[inline(always)]
    fn default() -> Dfmt0 {
        Dfmt0(0u64 as u32)
    }
}
impl core::fmt::Debug for Dfmt0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dfmt0")
            .field("did", &self.did())
            .field("dids", &self.dids())
            .field("pe", &self.pe())
            .field("pidm", &self.pidm())
            .field("sa", &self.sa())
            .field("pid", &self.pid())
            .field("dfmt", &self.dfmt())
            .field("lk1", &self.lk1())
            .field("vld", &self.vld())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dfmt0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Dfmt0 {
            did: u8,
            dids: super::vals::Dids,
            pe: super::vals::Pe,
            pidm: u8,
            sa: super::vals::Sa,
            pid: u8,
            dfmt: super::vals::Dfmt,
            lk1: bool,
            vld: bool,
        }
        let proxy = Dfmt0 {
            did: self.did(),
            dids: self.dids(),
            pe: self.pe(),
            pidm: self.pidm(),
            sa: self.sa(),
            pid: self.pid(),
            dfmt: self.dfmt(),
            lk1: self.lk1(),
            vld: self.vld(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "DAC Master Domain Assignment Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dfmt1(pub u32);
impl Dfmt1 {
    #[doc = "Domain identifier"]
    #[must_use]
    #[inline(always)]
    pub const fn did(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain identifier"]
    #[inline(always)]
    pub const fn set_did(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Privileged attribute"]
    #[must_use]
    #[inline(always)]
    pub const fn pa(&self) -> super::vals::Pa {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pa::from_bits(val as u8)
    }
    #[doc = "Privileged attribute"]
    #[inline(always)]
    pub const fn set_pa(&mut self, val: super::vals::Pa) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Secure attribute"]
    #[must_use]
    #[inline(always)]
    pub const fn sa(&self) -> super::vals::Sa {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Sa::from_bits(val as u8)
    }
    #[doc = "Secure attribute"]
    #[inline(always)]
    pub const fn set_sa(&mut self, val: super::vals::Sa) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "DID Bypass"]
    #[must_use]
    #[inline(always)]
    pub const fn didb(&self) -> super::vals::Didb {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Didb::from_bits(val as u8)
    }
    #[doc = "DID Bypass"]
    #[inline(always)]
    pub const fn set_didb(&mut self, val: super::vals::Didb) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Domain format"]
    #[must_use]
    #[inline(always)]
    pub const fn dfmt(&self) -> super::vals::Dfmt {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Dfmt::from_bits(val as u8)
    }
    #[doc = "Domain format"]
    #[inline(always)]
    pub const fn set_dfmt(&mut self, val: super::vals::Dfmt) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "1-bit Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lk1(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "1-bit Lock"]
    #[inline(always)]
    pub const fn set_lk1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn vld(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Valid"]
    #[inline(always)]
    pub const fn set_vld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dfmt1 {
    #[inline(always)]
    fn default() -> Dfmt1 {
        Dfmt1(536870912u64 as u32)
    }
}
impl core::fmt::Debug for Dfmt1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dfmt1")
            .field("did", &self.did())
            .field("pa", &self.pa())
            .field("sa", &self.sa())
            .field("didb", &self.didb())
            .field("dfmt", &self.dfmt())
            .field("lk1", &self.lk1())
            .field("vld", &self.vld())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dfmt1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Dfmt1 {
            did: u8,
            pa: super::vals::Pa,
            sa: super::vals::Sa,
            didb: super::vals::Didb,
            dfmt: super::vals::Dfmt,
            lk1: bool,
            vld: bool,
        }
        let proxy = Dfmt1 {
            did: self.did(),
            pa: self.pa(),
            sa: self.sa(),
            didb: self.didb(),
            dfmt: self.dfmt(),
            lk1: self.lk1(),
            vld: self.vld(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "MBC Global Access Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Glbac(pub u32);
impl Glbac {
    #[doc = "NonsecureUser Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn nux(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Execute"]
    #[inline(always)]
    pub const fn set_nux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "NonsecureUser Write"]
    #[must_use]
    #[inline(always)]
    pub const fn nuw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Write"]
    #[inline(always)]
    pub const fn set_nuw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "NonsecureUser Read"]
    #[must_use]
    #[inline(always)]
    pub const fn nur(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Read"]
    #[inline(always)]
    pub const fn set_nur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "NonsecurePriv Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn npx(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Execute"]
    #[inline(always)]
    pub const fn set_npx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "NonsecurePriv Write"]
    #[must_use]
    #[inline(always)]
    pub const fn npw(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Write"]
    #[inline(always)]
    pub const fn set_npw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "NonsecurePriv Read"]
    #[must_use]
    #[inline(always)]
    pub const fn npr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Read"]
    #[inline(always)]
    pub const fn set_npr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SecureUser Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn sux(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Execute"]
    #[inline(always)]
    pub const fn set_sux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SecureUser Write"]
    #[must_use]
    #[inline(always)]
    pub const fn suw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Write"]
    #[inline(always)]
    pub const fn set_suw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SecureUser Read"]
    #[must_use]
    #[inline(always)]
    pub const fn sur(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Read"]
    #[inline(always)]
    pub const fn set_sur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SecurePriv Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn spx(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Execute"]
    #[inline(always)]
    pub const fn set_spx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SecurePriv Write"]
    #[must_use]
    #[inline(always)]
    pub const fn spw(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Write"]
    #[inline(always)]
    pub const fn set_spw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "SecurePriv Read"]
    #[must_use]
    #[inline(always)]
    pub const fn spr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Read"]
    #[inline(always)]
    pub const fn set_spr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "LOCK"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LOCK"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Glbac {
    #[inline(always)]
    fn default() -> Glbac {
        Glbac(0u64 as u32)
    }
}
impl core::fmt::Debug for Glbac {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Glbac")
            .field("nux", &self.nux())
            .field("nuw", &self.nuw())
            .field("nur", &self.nur())
            .field("npx", &self.npx())
            .field("npw", &self.npw())
            .field("npr", &self.npr())
            .field("sux", &self.sux())
            .field("suw", &self.suw())
            .field("sur", &self.sur())
            .field("spx", &self.spx())
            .field("spw", &self.spw())
            .field("spr", &self.spr())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Glbac {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Glbac {
            nux: bool,
            nuw: bool,
            nur: bool,
            npx: bool,
            npw: bool,
            npr: bool,
            sux: bool,
            suw: bool,
            sur: bool,
            spx: bool,
            spw: bool,
            spr: bool,
            lk: bool,
        }
        let proxy = Glbac {
            nux: self.nux(),
            nuw: self.nuw(),
            nur: self.nur(),
            npx: self.npx(),
            npw: self.npw(),
            npr: self.npr(),
            sux: self.sux(),
            suw: self.suw(),
            sur: self.sur(),
            spx: self.spx(),
            spw: self.spw(),
            spr: self.spr(),
            lk: self.lk(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "MBC Global Access Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Glbac0(pub u32);
impl Glbac0 {
    #[doc = "NonsecureUser Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn nux(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Execute"]
    #[inline(always)]
    pub const fn set_nux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "NonsecureUser Write"]
    #[must_use]
    #[inline(always)]
    pub const fn nuw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Write"]
    #[inline(always)]
    pub const fn set_nuw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "NonsecureUser Read"]
    #[must_use]
    #[inline(always)]
    pub const fn nur(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Read"]
    #[inline(always)]
    pub const fn set_nur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "NonsecurePriv Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn npx(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Execute"]
    #[inline(always)]
    pub const fn set_npx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "NonsecurePriv Write"]
    #[must_use]
    #[inline(always)]
    pub const fn npw(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Write"]
    #[inline(always)]
    pub const fn set_npw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "NonsecurePriv Read"]
    #[must_use]
    #[inline(always)]
    pub const fn npr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Read"]
    #[inline(always)]
    pub const fn set_npr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SecureUser Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn sux(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Execute"]
    #[inline(always)]
    pub const fn set_sux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SecureUser Write"]
    #[must_use]
    #[inline(always)]
    pub const fn suw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Write"]
    #[inline(always)]
    pub const fn set_suw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SecureUser Read"]
    #[must_use]
    #[inline(always)]
    pub const fn sur(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Read"]
    #[inline(always)]
    pub const fn set_sur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SecurePriv Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn spx(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Execute"]
    #[inline(always)]
    pub const fn set_spx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SecurePriv Write"]
    #[must_use]
    #[inline(always)]
    pub const fn spw(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Write"]
    #[inline(always)]
    pub const fn set_spw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "SecurePriv Read"]
    #[must_use]
    #[inline(always)]
    pub const fn spr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Read"]
    #[inline(always)]
    pub const fn set_spr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for Glbac0 {
    #[inline(always)]
    fn default() -> Glbac0 {
        Glbac0(0u64 as u32)
    }
}
impl core::fmt::Debug for Glbac0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Glbac0")
            .field("nux", &self.nux())
            .field("nuw", &self.nuw())
            .field("nur", &self.nur())
            .field("npx", &self.npx())
            .field("npw", &self.npw())
            .field("npr", &self.npr())
            .field("sux", &self.sux())
            .field("suw", &self.suw())
            .field("sur", &self.sur())
            .field("spx", &self.spx())
            .field("spw", &self.spw())
            .field("spr", &self.spr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Glbac0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Glbac0 {
            nux: bool,
            nuw: bool,
            nur: bool,
            npx: bool,
            npw: bool,
            npr: bool,
            sux: bool,
            suw: bool,
            sur: bool,
            spx: bool,
            spw: bool,
            spr: bool,
        }
        let proxy = Glbac0 {
            nux: self.nux(),
            nuw: self.nuw(),
            nur: self.nur(),
            npx: self.npx(),
            npw: self.npw(),
            npr: self.npr(),
            sux: self.sux(),
            suw: self.suw(),
            sur: self.sur(),
            spx: self.spx(),
            spw: self.spw(),
            spr: self.spr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "MBC Domain Error Word1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MbcDerrW1(pub u32);
impl MbcDerrW1 {
    #[doc = "Error domain identifier"]
    #[must_use]
    #[inline(always)]
    pub const fn edid(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Error domain identifier"]
    #[inline(always)]
    pub const fn set_edid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Error attributes"]
    #[must_use]
    #[inline(always)]
    pub const fn eatr(&self) -> super::vals::Eatr {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Eatr::from_bits(val as u8)
    }
    #[doc = "Error attributes"]
    #[inline(always)]
    pub const fn set_eatr(&mut self, val: super::vals::Eatr) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Error read/write"]
    #[must_use]
    #[inline(always)]
    pub const fn erw(&self) -> super::vals::Erw {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Erw::from_bits(val as u8)
    }
    #[doc = "Error read/write"]
    #[inline(always)]
    pub const fn set_erw(&mut self, val: super::vals::Erw) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Error port"]
    #[must_use]
    #[inline(always)]
    pub const fn eport(&self) -> super::vals::Eport {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Eport::from_bits(val as u8)
    }
    #[doc = "Error port"]
    #[inline(always)]
    pub const fn set_eport(&mut self, val: super::vals::Eport) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "Error state"]
    #[must_use]
    #[inline(always)]
    pub const fn est(&self) -> super::vals::Est {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::Est::from_bits(val as u8)
    }
    #[doc = "Error state"]
    #[inline(always)]
    pub const fn set_est(&mut self, val: super::vals::Est) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for MbcDerrW1 {
    #[inline(always)]
    fn default() -> MbcDerrW1 {
        MbcDerrW1(0u64 as u32)
    }
}
impl core::fmt::Debug for MbcDerrW1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MbcDerrW1")
            .field("edid", &self.edid())
            .field("eatr", &self.eatr())
            .field("erw", &self.erw())
            .field("eport", &self.eport())
            .field("est", &self.est())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MbcDerrW1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MbcDerrW1 {
            edid: u8,
            eatr: super::vals::Eatr,
            erw: super::vals::Erw,
            eport: super::vals::Eport,
            est: super::vals::Est,
        }
        let proxy = MbcDerrW1 {
            edid: self.edid(),
            eatr: self.eatr(),
            erw: self.erw(),
            eport: self.eport(),
            est: self.est(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "MBC Domain Error Word3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MbcDerrW3(pub u32);
impl MbcDerrW3 {
    #[doc = "Rearm Error Capture Registers"]
    #[must_use]
    #[inline(always)]
    pub const fn recr(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Rearm Error Capture Registers"]
    #[inline(always)]
    pub const fn set_recr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for MbcDerrW3 {
    #[inline(always)]
    fn default() -> MbcDerrW3 {
        MbcDerrW3(0u64 as u32)
    }
}
impl core::fmt::Debug for MbcDerrW3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MbcDerrW3")
            .field("recr", &self.recr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MbcDerrW3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MbcDerrW3 {
            recr: u8,
        }
        let proxy = MbcDerrW3 { recr: self.recr() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "MBC Global Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MbcGlbcfg(pub u32);
impl MbcGlbcfg {
    #[doc = "Number of blocks in this memory"]
    #[must_use]
    #[inline(always)]
    pub const fn nblks(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Number of blocks in this memory"]
    #[inline(always)]
    pub const fn set_nblks(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Log2 size per block"]
    #[must_use]
    #[inline(always)]
    pub const fn size_log2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Log2 size per block"]
    #[inline(always)]
    pub const fn set_size_log2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for MbcGlbcfg {
    #[inline(always)]
    fn default() -> MbcGlbcfg {
        MbcGlbcfg(786560u64 as u32)
    }
}
impl core::fmt::Debug for MbcGlbcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MbcGlbcfg")
            .field("nblks", &self.nblks())
            .field("size_log2", &self.size_log2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MbcGlbcfg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MbcGlbcfg {
            nblks: u16,
            size_log2: u8,
        }
        let proxy = MbcGlbcfg {
            nblks: self.nblks(),
            size_log2: self.size_log2(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "MRC Domain Error Word1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrcDerrW1(pub u32);
impl MrcDerrW1 {
    #[doc = "Error domain identifier"]
    #[must_use]
    #[inline(always)]
    pub const fn edid(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Error domain identifier"]
    #[inline(always)]
    pub const fn set_edid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Error attributes"]
    #[must_use]
    #[inline(always)]
    pub const fn eatr(&self) -> super::vals::Eatr {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Eatr::from_bits(val as u8)
    }
    #[doc = "Error attributes"]
    #[inline(always)]
    pub const fn set_eatr(&mut self, val: super::vals::Eatr) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Error read/write"]
    #[must_use]
    #[inline(always)]
    pub const fn erw(&self) -> super::vals::Erw {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Erw::from_bits(val as u8)
    }
    #[doc = "Error read/write"]
    #[inline(always)]
    pub const fn set_erw(&mut self, val: super::vals::Erw) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Error port"]
    #[must_use]
    #[inline(always)]
    pub const fn eport(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Error port"]
    #[inline(always)]
    pub const fn set_eport(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "Error state"]
    #[must_use]
    #[inline(always)]
    pub const fn est(&self) -> super::vals::Est {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::Est::from_bits(val as u8)
    }
    #[doc = "Error state"]
    #[inline(always)]
    pub const fn set_est(&mut self, val: super::vals::Est) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for MrcDerrW1 {
    #[inline(always)]
    fn default() -> MrcDerrW1 {
        MrcDerrW1(0u64 as u32)
    }
}
impl core::fmt::Debug for MrcDerrW1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrcDerrW1")
            .field("edid", &self.edid())
            .field("eatr", &self.eatr())
            .field("erw", &self.erw())
            .field("eport", &self.eport())
            .field("est", &self.est())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrcDerrW1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MrcDerrW1 {
            edid: u8,
            eatr: super::vals::Eatr,
            erw: super::vals::Erw,
            eport: u8,
            est: super::vals::Est,
        }
        let proxy = MrcDerrW1 {
            edid: self.edid(),
            eatr: self.eatr(),
            erw: self.erw(),
            eport: self.eport(),
            est: self.est(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "MRC Domain Error Word3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrcDerrW3(pub u32);
impl MrcDerrW3 {
    #[doc = "Rearm Error Capture Registers"]
    #[must_use]
    #[inline(always)]
    pub const fn recr(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Rearm Error Capture Registers"]
    #[inline(always)]
    pub const fn set_recr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for MrcDerrW3 {
    #[inline(always)]
    fn default() -> MrcDerrW3 {
        MrcDerrW3(0u64 as u32)
    }
}
impl core::fmt::Debug for MrcDerrW3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrcDerrW3")
            .field("recr", &self.recr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrcDerrW3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MrcDerrW3 {
            recr: u8,
        }
        let proxy = MrcDerrW3 { recr: self.recr() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "MRC Global Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrcGlbcfg(pub u32);
impl MrcGlbcfg {
    #[doc = "Number of regions \\[1-16\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn nrgns(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Number of regions \\[1-16\\]"]
    #[inline(always)]
    pub const fn set_nrgns(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for MrcGlbcfg {
    #[inline(always)]
    fn default() -> MrcGlbcfg {
        MrcGlbcfg(8u64 as u32)
    }
}
impl core::fmt::Debug for MrcGlbcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrcGlbcfg")
            .field("nrgns", &self.nrgns())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrcGlbcfg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MrcGlbcfg {
            nrgns: u8,
        }
        let proxy = MrcGlbcfg {
            nrgns: self.nrgns(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "MBC Memory Block NonSecure Enable Word"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nse(pub u32);
impl Nse {
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Nse {
    #[inline(always)]
    fn default() -> Nse {
        Nse(0u64 as u32)
    }
}
impl core::fmt::Debug for Nse {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nse")
            .field("bit0", &self.bit0())
            .field("bit1", &self.bit1())
            .field("bit2", &self.bit2())
            .field("bit3", &self.bit3())
            .field("bit4", &self.bit4())
            .field("bit5", &self.bit5())
            .field("bit6", &self.bit6())
            .field("bit7", &self.bit7())
            .field("bit8", &self.bit8())
            .field("bit9", &self.bit9())
            .field("bit10", &self.bit10())
            .field("bit11", &self.bit11())
            .field("bit12", &self.bit12())
            .field("bit13", &self.bit13())
            .field("bit14", &self.bit14())
            .field("bit15", &self.bit15())
            .field("bit16", &self.bit16())
            .field("bit17", &self.bit17())
            .field("bit18", &self.bit18())
            .field("bit19", &self.bit19())
            .field("bit20", &self.bit20())
            .field("bit21", &self.bit21())
            .field("bit22", &self.bit22())
            .field("bit23", &self.bit23())
            .field("bit24", &self.bit24())
            .field("bit25", &self.bit25())
            .field("bit26", &self.bit26())
            .field("bit27", &self.bit27())
            .field("bit28", &self.bit28())
            .field("bit29", &self.bit29())
            .field("bit30", &self.bit30())
            .field("bit31", &self.bit31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nse {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Nse {
            bit0: bool,
            bit1: bool,
            bit2: bool,
            bit3: bool,
            bit4: bool,
            bit5: bool,
            bit6: bool,
            bit7: bool,
            bit8: bool,
            bit9: bool,
            bit10: bool,
            bit11: bool,
            bit12: bool,
            bit13: bool,
            bit14: bool,
            bit15: bool,
            bit16: bool,
            bit17: bool,
            bit18: bool,
            bit19: bool,
            bit20: bool,
            bit21: bool,
            bit22: bool,
            bit23: bool,
            bit24: bool,
            bit25: bool,
            bit26: bool,
            bit27: bool,
            bit28: bool,
            bit29: bool,
            bit30: bool,
            bit31: bool,
        }
        let proxy = Nse {
            bit0: self.bit0(),
            bit1: self.bit1(),
            bit2: self.bit2(),
            bit3: self.bit3(),
            bit4: self.bit4(),
            bit5: self.bit5(),
            bit6: self.bit6(),
            bit7: self.bit7(),
            bit8: self.bit8(),
            bit9: self.bit9(),
            bit10: self.bit10(),
            bit11: self.bit11(),
            bit12: self.bit12(),
            bit13: self.bit13(),
            bit14: self.bit14(),
            bit15: self.bit15(),
            bit16: self.bit16(),
            bit17: self.bit17(),
            bit18: self.bit18(),
            bit19: self.bit19(),
            bit20: self.bit20(),
            bit21: self.bit21(),
            bit22: self.bit22(),
            bit23: self.bit23(),
            bit24: self.bit24(),
            bit25: self.bit25(),
            bit26: self.bit26(),
            bit27: self.bit27(),
            bit28: self.bit28(),
            bit29: self.bit29(),
            bit30: self.bit30(),
            bit31: self.bit31(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "MBC NonSecure Enable Block Clear All"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NseBlkClrAll(pub u32);
impl NseBlkClrAll {
    #[doc = "Memory Select"]
    #[must_use]
    #[inline(always)]
    pub const fn memsel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Memory Select"]
    #[inline(always)]
    pub const fn set_memsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "DID Select"]
    #[must_use]
    #[inline(always)]
    pub const fn did_sel0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "DID Select"]
    #[inline(always)]
    pub const fn set_did_sel0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "DID Select"]
    #[must_use]
    #[inline(always)]
    pub const fn did_sel1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "DID Select"]
    #[inline(always)]
    pub const fn set_did_sel1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "DID Select"]
    #[must_use]
    #[inline(always)]
    pub const fn did_sel2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "DID Select"]
    #[inline(always)]
    pub const fn set_did_sel2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "DID Select"]
    #[must_use]
    #[inline(always)]
    pub const fn did_sel3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "DID Select"]
    #[inline(always)]
    pub const fn set_did_sel3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "DID Select"]
    #[must_use]
    #[inline(always)]
    pub const fn did_sel4(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "DID Select"]
    #[inline(always)]
    pub const fn set_did_sel4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DID Select"]
    #[must_use]
    #[inline(always)]
    pub const fn did_sel5(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "DID Select"]
    #[inline(always)]
    pub const fn set_did_sel5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "DID Select"]
    #[must_use]
    #[inline(always)]
    pub const fn did_sel6(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "DID Select"]
    #[inline(always)]
    pub const fn set_did_sel6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "DID Select"]
    #[must_use]
    #[inline(always)]
    pub const fn did_sel7(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "DID Select"]
    #[inline(always)]
    pub const fn set_did_sel7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "DID Select"]
    #[must_use]
    #[inline(always)]
    pub const fn did_sel8(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "DID Select"]
    #[inline(always)]
    pub const fn set_did_sel8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "DID Select"]
    #[must_use]
    #[inline(always)]
    pub const fn did_sel9(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "DID Select"]
    #[inline(always)]
    pub const fn set_did_sel9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "DID Select"]
    #[must_use]
    #[inline(always)]
    pub const fn did_sel10(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "DID Select"]
    #[inline(always)]
    pub const fn set_did_sel10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "DID Select"]
    #[must_use]
    #[inline(always)]
    pub const fn did_sel11(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "DID Select"]
    #[inline(always)]
    pub const fn set_did_sel11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "DID Select"]
    #[must_use]
    #[inline(always)]
    pub const fn did_sel12(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "DID Select"]
    #[inline(always)]
    pub const fn set_did_sel12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "DID Select"]
    #[must_use]
    #[inline(always)]
    pub const fn did_sel13(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "DID Select"]
    #[inline(always)]
    pub const fn set_did_sel13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "DID Select"]
    #[must_use]
    #[inline(always)]
    pub const fn did_sel14(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "DID Select"]
    #[inline(always)]
    pub const fn set_did_sel14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "DID Select"]
    #[must_use]
    #[inline(always)]
    pub const fn did_sel15(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DID Select"]
    #[inline(always)]
    pub const fn set_did_sel15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for NseBlkClrAll {
    #[inline(always)]
    fn default() -> NseBlkClrAll {
        NseBlkClrAll(0u64 as u32)
    }
}
impl core::fmt::Debug for NseBlkClrAll {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NseBlkClrAll")
            .field("memsel", &self.memsel())
            .field("did_sel0", &self.did_sel0())
            .field("did_sel1", &self.did_sel1())
            .field("did_sel2", &self.did_sel2())
            .field("did_sel3", &self.did_sel3())
            .field("did_sel4", &self.did_sel4())
            .field("did_sel5", &self.did_sel5())
            .field("did_sel6", &self.did_sel6())
            .field("did_sel7", &self.did_sel7())
            .field("did_sel8", &self.did_sel8())
            .field("did_sel9", &self.did_sel9())
            .field("did_sel10", &self.did_sel10())
            .field("did_sel11", &self.did_sel11())
            .field("did_sel12", &self.did_sel12())
            .field("did_sel13", &self.did_sel13())
            .field("did_sel14", &self.did_sel14())
            .field("did_sel15", &self.did_sel15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NseBlkClrAll {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NseBlkClrAll {
            memsel: u8,
            did_sel0: bool,
            did_sel1: bool,
            did_sel2: bool,
            did_sel3: bool,
            did_sel4: bool,
            did_sel5: bool,
            did_sel6: bool,
            did_sel7: bool,
            did_sel8: bool,
            did_sel9: bool,
            did_sel10: bool,
            did_sel11: bool,
            did_sel12: bool,
            did_sel13: bool,
            did_sel14: bool,
            did_sel15: bool,
        }
        let proxy = NseBlkClrAll {
            memsel: self.memsel(),
            did_sel0: self.did_sel0(),
            did_sel1: self.did_sel1(),
            did_sel2: self.did_sel2(),
            did_sel3: self.did_sel3(),
            did_sel4: self.did_sel4(),
            did_sel5: self.did_sel5(),
            did_sel6: self.did_sel6(),
            did_sel7: self.did_sel7(),
            did_sel8: self.did_sel8(),
            did_sel9: self.did_sel9(),
            did_sel10: self.did_sel10(),
            did_sel11: self.did_sel11(),
            did_sel12: self.did_sel12(),
            did_sel13: self.did_sel13(),
            did_sel14: self.did_sel14(),
            did_sel15: self.did_sel15(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "MBC NonSecure Enable Block Index"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NseBlkIndex(pub u32);
impl NseBlkIndex {
    #[doc = "Auto Increment"]
    #[must_use]
    #[inline(always)]
    pub const fn ai(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Auto Increment"]
    #[inline(always)]
    pub const fn set_ai(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Word index into the block NSE bitmap. It selects the BLK_NSE_Wn register, where WNDX determines the value of n."]
    #[must_use]
    #[inline(always)]
    pub const fn wndx(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x0f;
        val as u8
    }
    #[doc = "Word index into the block NSE bitmap. It selects the BLK_NSE_Wn register, where WNDX determines the value of n."]
    #[inline(always)]
    pub const fn set_wndx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
    }
    #[doc = "Memory Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mem_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Memory Select"]
    #[inline(always)]
    pub const fn set_mem_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "DID Select"]
    #[must_use]
    #[inline(always)]
    pub const fn did_sel0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "DID Select"]
    #[inline(always)]
    pub const fn set_did_sel0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "DID Select"]
    #[must_use]
    #[inline(always)]
    pub const fn did_sel1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "DID Select"]
    #[inline(always)]
    pub const fn set_did_sel1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "DID Select"]
    #[must_use]
    #[inline(always)]
    pub const fn did_sel2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "DID Select"]
    #[inline(always)]
    pub const fn set_did_sel2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "DID Select"]
    #[must_use]
    #[inline(always)]
    pub const fn did_sel3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "DID Select"]
    #[inline(always)]
    pub const fn set_did_sel3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "DID Select"]
    #[must_use]
    #[inline(always)]
    pub const fn did_sel4(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "DID Select"]
    #[inline(always)]
    pub const fn set_did_sel4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DID Select"]
    #[must_use]
    #[inline(always)]
    pub const fn did_sel5(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "DID Select"]
    #[inline(always)]
    pub const fn set_did_sel5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "DID Select"]
    #[must_use]
    #[inline(always)]
    pub const fn did_sel6(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "DID Select"]
    #[inline(always)]
    pub const fn set_did_sel6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "DID Select"]
    #[must_use]
    #[inline(always)]
    pub const fn did_sel7(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "DID Select"]
    #[inline(always)]
    pub const fn set_did_sel7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "DID Select"]
    #[must_use]
    #[inline(always)]
    pub const fn did_sel8(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "DID Select"]
    #[inline(always)]
    pub const fn set_did_sel8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "DID Select"]
    #[must_use]
    #[inline(always)]
    pub const fn did_sel9(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "DID Select"]
    #[inline(always)]
    pub const fn set_did_sel9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "DID Select"]
    #[must_use]
    #[inline(always)]
    pub const fn did_sel10(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "DID Select"]
    #[inline(always)]
    pub const fn set_did_sel10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "DID Select"]
    #[must_use]
    #[inline(always)]
    pub const fn did_sel11(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "DID Select"]
    #[inline(always)]
    pub const fn set_did_sel11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "DID Select"]
    #[must_use]
    #[inline(always)]
    pub const fn did_sel12(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "DID Select"]
    #[inline(always)]
    pub const fn set_did_sel12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "DID Select"]
    #[must_use]
    #[inline(always)]
    pub const fn did_sel13(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "DID Select"]
    #[inline(always)]
    pub const fn set_did_sel13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "DID Select"]
    #[must_use]
    #[inline(always)]
    pub const fn did_sel14(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "DID Select"]
    #[inline(always)]
    pub const fn set_did_sel14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "DID Select"]
    #[must_use]
    #[inline(always)]
    pub const fn did_sel15(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DID Select"]
    #[inline(always)]
    pub const fn set_did_sel15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for NseBlkIndex {
    #[inline(always)]
    fn default() -> NseBlkIndex {
        NseBlkIndex(0u64 as u32)
    }
}
impl core::fmt::Debug for NseBlkIndex {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NseBlkIndex")
            .field("ai", &self.ai())
            .field("wndx", &self.wndx())
            .field("mem_sel", &self.mem_sel())
            .field("did_sel0", &self.did_sel0())
            .field("did_sel1", &self.did_sel1())
            .field("did_sel2", &self.did_sel2())
            .field("did_sel3", &self.did_sel3())
            .field("did_sel4", &self.did_sel4())
            .field("did_sel5", &self.did_sel5())
            .field("did_sel6", &self.did_sel6())
            .field("did_sel7", &self.did_sel7())
            .field("did_sel8", &self.did_sel8())
            .field("did_sel9", &self.did_sel9())
            .field("did_sel10", &self.did_sel10())
            .field("did_sel11", &self.did_sel11())
            .field("did_sel12", &self.did_sel12())
            .field("did_sel13", &self.did_sel13())
            .field("did_sel14", &self.did_sel14())
            .field("did_sel15", &self.did_sel15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NseBlkIndex {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NseBlkIndex {
            ai: bool,
            wndx: u8,
            mem_sel: u8,
            did_sel0: bool,
            did_sel1: bool,
            did_sel2: bool,
            did_sel3: bool,
            did_sel4: bool,
            did_sel5: bool,
            did_sel6: bool,
            did_sel7: bool,
            did_sel8: bool,
            did_sel9: bool,
            did_sel10: bool,
            did_sel11: bool,
            did_sel12: bool,
            did_sel13: bool,
            did_sel14: bool,
            did_sel15: bool,
        }
        let proxy = NseBlkIndex {
            ai: self.ai(),
            wndx: self.wndx(),
            mem_sel: self.mem_sel(),
            did_sel0: self.did_sel0(),
            did_sel1: self.did_sel1(),
            did_sel2: self.did_sel2(),
            did_sel3: self.did_sel3(),
            did_sel4: self.did_sel4(),
            did_sel5: self.did_sel5(),
            did_sel6: self.did_sel6(),
            did_sel7: self.did_sel7(),
            did_sel8: self.did_sel8(),
            did_sel9: self.did_sel9(),
            did_sel10: self.did_sel10(),
            did_sel11: self.did_sel11(),
            did_sel12: self.did_sel12(),
            did_sel13: self.did_sel13(),
            did_sel14: self.did_sel14(),
            did_sel15: self.did_sel15(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "MRC NonSecure Enable Region Clear"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NseRgnClr(pub u32);
impl NseRgnClr {
    #[doc = "Write-1 Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn w1clr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Write-1 Clear"]
    #[inline(always)]
    pub const fn set_w1clr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for NseRgnClr {
    #[inline(always)]
    fn default() -> NseRgnClr {
        NseRgnClr(0u64 as u32)
    }
}
impl core::fmt::Debug for NseRgnClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NseRgnClr")
            .field("w1clr", &self.w1clr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NseRgnClr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NseRgnClr {
            w1clr: u16,
        }
        let proxy = NseRgnClr {
            w1clr: self.w1clr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "MRC NonSecure Enable Region Clear All"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NseRgnClrAll(pub u32);
impl NseRgnClrAll {
    #[doc = "DID Select"]
    #[must_use]
    #[inline(always)]
    pub const fn did_sel(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "DID Select"]
    #[inline(always)]
    pub const fn set_did_sel(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for NseRgnClrAll {
    #[inline(always)]
    fn default() -> NseRgnClrAll {
        NseRgnClrAll(0u64 as u32)
    }
}
impl core::fmt::Debug for NseRgnClrAll {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NseRgnClrAll")
            .field("did_sel", &self.did_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NseRgnClrAll {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NseRgnClrAll {
            did_sel: u16,
        }
        let proxy = NseRgnClrAll {
            did_sel: self.did_sel(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "MRC NonSecure Enable Region Indirect"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NseRgnIndirect(pub u32);
impl NseRgnIndirect {
    #[doc = "DID Select"]
    #[must_use]
    #[inline(always)]
    pub const fn did_sel(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "DID Select"]
    #[inline(always)]
    pub const fn set_did_sel(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for NseRgnIndirect {
    #[inline(always)]
    fn default() -> NseRgnIndirect {
        NseRgnIndirect(0u64 as u32)
    }
}
impl core::fmt::Debug for NseRgnIndirect {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NseRgnIndirect")
            .field("did_sel", &self.did_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NseRgnIndirect {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NseRgnIndirect {
            did_sel: u16,
        }
        let proxy = NseRgnIndirect {
            did_sel: self.did_sel(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "MRC NonSecure Enable Region Set"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NseRgnSet(pub u32);
impl NseRgnSet {
    #[doc = "Write-1 Set"]
    #[must_use]
    #[inline(always)]
    pub const fn w1set(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Write-1 Set"]
    #[inline(always)]
    pub const fn set_w1set(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for NseRgnSet {
    #[inline(always)]
    fn default() -> NseRgnSet {
        NseRgnSet(0u64 as u32)
    }
}
impl core::fmt::Debug for NseRgnSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NseRgnSet")
            .field("w1set", &self.w1set())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NseRgnSet {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NseRgnSet {
            w1set: u16,
        }
        let proxy = NseRgnSet {
            w1set: self.w1set(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Process Identifier"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pid(pub u32);
impl Pid {
    #[doc = "Process identifier"]
    #[must_use]
    #[inline(always)]
    pub const fn pid(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Process identifier"]
    #[inline(always)]
    pub const fn set_pid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lk2(&self) -> super::vals::Lk {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Lk::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lk2(&mut self, val: super::vals::Lk) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Pid {
    #[inline(always)]
    fn default() -> Pid {
        Pid(0u64 as u32)
    }
}
impl core::fmt::Debug for Pid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pid")
            .field("pid", &self.pid())
            .field("lk2", &self.lk2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pid {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pid {
            pid: u8,
            lk2: super::vals::Lk,
        }
        let proxy = Pid {
            pid: self.pid(),
            lk2: self.lk2(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "MRC Region Descriptor NonSecure Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RgdNse16(pub u32);
impl RgdNse16 {
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[inline(always)]
    pub const fn set_bit0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[inline(always)]
    pub const fn set_bit1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[inline(always)]
    pub const fn set_bit2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[inline(always)]
    pub const fn set_bit3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[inline(always)]
    pub const fn set_bit4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[inline(always)]
    pub const fn set_bit5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[inline(always)]
    pub const fn set_bit6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[inline(always)]
    pub const fn set_bit7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[inline(always)]
    pub const fn set_bit8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[inline(always)]
    pub const fn set_bit9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[inline(always)]
    pub const fn set_bit10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[inline(always)]
    pub const fn set_bit11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[inline(always)]
    pub const fn set_bit12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[inline(always)]
    pub const fn set_bit13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[inline(always)]
    pub const fn set_bit14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[inline(always)]
    pub const fn set_bit15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for RgdNse16 {
    #[inline(always)]
    fn default() -> RgdNse16 {
        RgdNse16(0u64 as u32)
    }
}
impl core::fmt::Debug for RgdNse16 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RgdNse16")
            .field("bit0", &self.bit0())
            .field("bit1", &self.bit1())
            .field("bit2", &self.bit2())
            .field("bit3", &self.bit3())
            .field("bit4", &self.bit4())
            .field("bit5", &self.bit5())
            .field("bit6", &self.bit6())
            .field("bit7", &self.bit7())
            .field("bit8", &self.bit8())
            .field("bit9", &self.bit9())
            .field("bit10", &self.bit10())
            .field("bit11", &self.bit11())
            .field("bit12", &self.bit12())
            .field("bit13", &self.bit13())
            .field("bit14", &self.bit14())
            .field("bit15", &self.bit15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RgdNse16 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct RgdNse16 {
            bit0: bool,
            bit1: bool,
            bit2: bool,
            bit3: bool,
            bit4: bool,
            bit5: bool,
            bit6: bool,
            bit7: bool,
            bit8: bool,
            bit9: bool,
            bit10: bool,
            bit11: bool,
            bit12: bool,
            bit13: bool,
            bit14: bool,
            bit15: bool,
        }
        let proxy = RgdNse16 {
            bit0: self.bit0(),
            bit1: self.bit1(),
            bit2: self.bit2(),
            bit3: self.bit3(),
            bit4: self.bit4(),
            bit5: self.bit5(),
            bit6: self.bit6(),
            bit7: self.bit7(),
            bit8: self.bit8(),
            bit9: self.bit9(),
            bit10: self.bit10(),
            bit11: self.bit11(),
            bit12: self.bit12(),
            bit13: self.bit13(),
            bit14: self.bit14(),
            bit15: self.bit15(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "MRC Region Descriptor NonSecure Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RgdNse8(pub u32);
impl RgdNse8 {
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[inline(always)]
    pub const fn set_bit0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[inline(always)]
    pub const fn set_bit1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[inline(always)]
    pub const fn set_bit2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[inline(always)]
    pub const fn set_bit3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[inline(always)]
    pub const fn set_bit4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[inline(always)]
    pub const fn set_bit5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[inline(always)]
    pub const fn set_bit6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Bit n NonSecure Enable \\[n = 0 - 15\\]"]
    #[inline(always)]
    pub const fn set_bit7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for RgdNse8 {
    #[inline(always)]
    fn default() -> RgdNse8 {
        RgdNse8(0u64 as u32)
    }
}
impl core::fmt::Debug for RgdNse8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RgdNse8")
            .field("bit0", &self.bit0())
            .field("bit1", &self.bit1())
            .field("bit2", &self.bit2())
            .field("bit3", &self.bit3())
            .field("bit4", &self.bit4())
            .field("bit5", &self.bit5())
            .field("bit6", &self.bit6())
            .field("bit7", &self.bit7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RgdNse8 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct RgdNse8 {
            bit0: bool,
            bit1: bool,
            bit2: bool,
            bit3: bool,
            bit4: bool,
            bit5: bool,
            bit6: bool,
            bit7: bool,
        }
        let proxy = RgdNse8 {
            bit0: self.bit0(),
            bit1: self.bit1(),
            bit2: self.bit2(),
            bit3: self.bit3(),
            bit4: self.bit4(),
            bit5: self.bit5(),
            bit6: self.bit6(),
            bit7: self.bit7(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "MRC Region Descriptor Word 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RgdW0(pub u32);
impl RgdW0 {
    #[doc = "Memory Region Access Control Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mracsel(&self) -> super::vals::Mracsel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Mracsel::from_bits(val as u8)
    }
    #[doc = "Memory Region Access Control Select"]
    #[inline(always)]
    pub const fn set_mracsel(&mut self, val: super::vals::Mracsel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Start Address"]
    #[must_use]
    #[inline(always)]
    pub const fn strt_addr(&self) -> u32 {
        let val = (self.0 >> 14usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "Start Address"]
    #[inline(always)]
    pub const fn set_strt_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 14usize)) | (((val as u32) & 0x0003_ffff) << 14usize);
    }
}
impl Default for RgdW0 {
    #[inline(always)]
    fn default() -> RgdW0 {
        RgdW0(0u64 as u32)
    }
}
impl core::fmt::Debug for RgdW0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RgdW0")
            .field("mracsel", &self.mracsel())
            .field("strt_addr", &self.strt_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RgdW0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct RgdW0 {
            mracsel: super::vals::Mracsel,
            strt_addr: u32,
        }
        let proxy = RgdW0 {
            mracsel: self.mracsel(),
            strt_addr: self.strt_addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "MRC Region Descriptor Word 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RgdW1(pub u32);
impl RgdW1 {
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
    #[doc = "NonSecure Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nse(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "NonSecure Enable"]
    #[inline(always)]
    pub const fn set_nse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "End Address"]
    #[must_use]
    #[inline(always)]
    pub const fn end_addr(&self) -> u32 {
        let val = (self.0 >> 14usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "End Address"]
    #[inline(always)]
    pub const fn set_end_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 14usize)) | (((val as u32) & 0x0003_ffff) << 14usize);
    }
}
impl Default for RgdW1 {
    #[inline(always)]
    fn default() -> RgdW1 {
        RgdW1(0u64 as u32)
    }
}
impl core::fmt::Debug for RgdW1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RgdW1")
            .field("vld", &self.vld())
            .field("nse", &self.nse())
            .field("end_addr", &self.end_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RgdW1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct RgdW1 {
            vld: bool,
            nse: bool,
            end_addr: u32,
        }
        let proxy = RgdW1 {
            vld: self.vld(),
            nse: self.nse(),
            end_addr: self.end_addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "TRDC Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrdcCr(pub u32);
impl TrdcCr {
    #[doc = "Global Valid for Domain Assignment Controllers"]
    #[must_use]
    #[inline(always)]
    pub const fn gvldm(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Global Valid for Domain Assignment Controllers"]
    #[inline(always)]
    pub const fn set_gvldm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Hardware Revision Level"]
    #[must_use]
    #[inline(always)]
    pub const fn hrl(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x0f;
        val as u8
    }
    #[doc = "Hardware Revision Level"]
    #[inline(always)]
    pub const fn set_hrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 1usize)) | (((val as u32) & 0x0f) << 1usize);
    }
    #[doc = "Global Valid for Memory Block Checkers"]
    #[must_use]
    #[inline(always)]
    pub const fn gvldb(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Global Valid for Memory Block Checkers"]
    #[inline(always)]
    pub const fn set_gvldb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Global Valid for Memory Region Checkers"]
    #[must_use]
    #[inline(always)]
    pub const fn gvldr(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Global Valid for Memory Region Checkers"]
    #[inline(always)]
    pub const fn set_gvldr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Lock Status"]
    #[must_use]
    #[inline(always)]
    pub const fn lk1(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Lock Status"]
    #[inline(always)]
    pub const fn set_lk1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for TrdcCr {
    #[inline(always)]
    fn default() -> TrdcCr {
        TrdcCr(16u64 as u32)
    }
}
impl core::fmt::Debug for TrdcCr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TrdcCr")
            .field("gvldm", &self.gvldm())
            .field("hrl", &self.hrl())
            .field("gvldb", &self.gvldb())
            .field("gvldr", &self.gvldr())
            .field("lk1", &self.lk1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrdcCr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TrdcCr {
            gvldm: bool,
            hrl: u8,
            gvldb: bool,
            gvldr: bool,
            lk1: bool,
        }
        let proxy = TrdcCr {
            gvldm: self.gvldm(),
            hrl: self.hrl(),
            gvldb: self.gvldb(),
            gvldr: self.gvldr(),
            lk1: self.lk1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "TRDC Domain Error Location Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrdcDerrloc(pub u32);
impl TrdcDerrloc {
    #[doc = "MBC instance"]
    #[must_use]
    #[inline(always)]
    pub const fn mbcinst(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "MBC instance"]
    #[inline(always)]
    pub const fn set_mbcinst(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "MRC instance"]
    #[must_use]
    #[inline(always)]
    pub const fn mrcinst(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "MRC instance"]
    #[inline(always)]
    pub const fn set_mrcinst(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for TrdcDerrloc {
    #[inline(always)]
    fn default() -> TrdcDerrloc {
        TrdcDerrloc(0u64 as u32)
    }
}
impl core::fmt::Debug for TrdcDerrloc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TrdcDerrloc")
            .field("mbcinst", &self.mbcinst())
            .field("mrcinst", &self.mrcinst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrdcDerrloc {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TrdcDerrloc {
            mbcinst: u8,
            mrcinst: u16,
        }
        let proxy = TrdcDerrloc {
            mbcinst: self.mbcinst(),
            mrcinst: self.mrcinst(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "TRDC Fault Domain ID"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrdcFdid(pub u32);
impl TrdcFdid {
    #[doc = "Domain ID of Faulted Access"]
    #[must_use]
    #[inline(always)]
    pub const fn fdid(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain ID of Faulted Access"]
    #[inline(always)]
    pub const fn set_fdid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for TrdcFdid {
    #[inline(always)]
    fn default() -> TrdcFdid {
        TrdcFdid(0u64 as u32)
    }
}
impl core::fmt::Debug for TrdcFdid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TrdcFdid")
            .field("fdid", &self.fdid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrdcFdid {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TrdcFdid {
            fdid: u8,
        }
        let proxy = TrdcFdid { fdid: self.fdid() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "TRDC FLW Array Base"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrdcFlwAbase(pub u32);
impl TrdcFlwAbase {
    #[doc = "Array base address low"]
    #[must_use]
    #[inline(always)]
    pub const fn abase_l(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x7f;
        val as u8
    }
    #[doc = "Array base address low"]
    #[inline(always)]
    pub const fn set_abase_l(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 15usize)) | (((val as u32) & 0x7f) << 15usize);
    }
    #[doc = "Array base address high"]
    #[must_use]
    #[inline(always)]
    pub const fn abase_h(&self) -> u16 {
        let val = (self.0 >> 22usize) & 0x03ff;
        val as u16
    }
    #[doc = "Array base address high"]
    #[inline(always)]
    pub const fn set_abase_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 22usize)) | (((val as u32) & 0x03ff) << 22usize);
    }
}
impl Default for TrdcFlwAbase {
    #[inline(always)]
    fn default() -> TrdcFlwAbase {
        TrdcFlwAbase(0u64 as u32)
    }
}
impl core::fmt::Debug for TrdcFlwAbase {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TrdcFlwAbase")
            .field("abase_l", &self.abase_l())
            .field("abase_h", &self.abase_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrdcFlwAbase {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TrdcFlwAbase {
            abase_l: u8,
            abase_h: u16,
        }
        let proxy = TrdcFlwAbase {
            abase_l: self.abase_l(),
            abase_h: self.abase_h(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "TRDC FLW Block Count"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrdcFlwBcnt(pub u32);
impl TrdcFlwBcnt {
    #[doc = "Block Count"]
    #[must_use]
    #[inline(always)]
    pub const fn bcnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Block Count"]
    #[inline(always)]
    pub const fn set_bcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
}
impl Default for TrdcFlwBcnt {
    #[inline(always)]
    fn default() -> TrdcFlwBcnt {
        TrdcFlwBcnt(0u64 as u32)
    }
}
impl core::fmt::Debug for TrdcFlwBcnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TrdcFlwBcnt")
            .field("bcnt", &self.bcnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrdcFlwBcnt {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TrdcFlwBcnt {
            bcnt: u16,
        }
        let proxy = TrdcFlwBcnt { bcnt: self.bcnt() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "TRDC FLW Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrdcFlwCtl(pub u32);
impl TrdcFlwCtl {
    #[doc = "Lock bit"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Valid bit"]
    #[must_use]
    #[inline(always)]
    pub const fn v(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Valid bit"]
    #[inline(always)]
    pub const fn set_v(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for TrdcFlwCtl {
    #[inline(always)]
    fn default() -> TrdcFlwCtl {
        TrdcFlwCtl(0u64 as u32)
    }
}
impl core::fmt::Debug for TrdcFlwCtl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TrdcFlwCtl")
            .field("lk", &self.lk())
            .field("v", &self.v())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrdcFlwCtl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TrdcFlwCtl {
            lk: bool,
            v: bool,
        }
        let proxy = TrdcFlwCtl {
            lk: self.lk(),
            v: self.v(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "TRDC Hardware Configuration Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrdcHwcfg0(pub u32);
impl TrdcHwcfg0 {
    #[doc = "Number of domains"]
    #[must_use]
    #[inline(always)]
    pub const fn ndid(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Number of domains"]
    #[inline(always)]
    pub const fn set_ndid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Number of bus masters"]
    #[must_use]
    #[inline(always)]
    pub const fn nmstr(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Number of bus masters"]
    #[inline(always)]
    pub const fn set_nmstr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Number of MBCs"]
    #[must_use]
    #[inline(always)]
    pub const fn nmbc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of MBCs"]
    #[inline(always)]
    pub const fn set_nmbc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Number of MRCs"]
    #[must_use]
    #[inline(always)]
    pub const fn nmrc(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "Number of MRCs"]
    #[inline(always)]
    pub const fn set_nmrc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "Module ID"]
    #[must_use]
    #[inline(always)]
    pub const fn mid(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x07;
        val as u8
    }
    #[doc = "Module ID"]
    #[inline(always)]
    pub const fn set_mid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
    }
}
impl Default for TrdcHwcfg0 {
    #[inline(always)]
    fn default() -> TrdcHwcfg0 {
        TrdcHwcfg0(570557456u64 as u32)
    }
}
impl core::fmt::Debug for TrdcHwcfg0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TrdcHwcfg0")
            .field("ndid", &self.ndid())
            .field("nmstr", &self.nmstr())
            .field("nmbc", &self.nmbc())
            .field("nmrc", &self.nmrc())
            .field("mid", &self.mid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrdcHwcfg0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TrdcHwcfg0 {
            ndid: u8,
            nmstr: u8,
            nmbc: u8,
            nmrc: u8,
            mid: u8,
        }
        let proxy = TrdcHwcfg0 {
            ndid: self.ndid(),
            nmstr: self.nmstr(),
            nmbc: self.nmbc(),
            nmrc: self.nmrc(),
            mid: self.mid(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "TRDC Hardware Configuration Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrdcHwcfg1(pub u32);
impl TrdcHwcfg1 {
    #[doc = "Domain identifier number"]
    #[must_use]
    #[inline(always)]
    pub const fn did(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain identifier number"]
    #[inline(always)]
    pub const fn set_did(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for TrdcHwcfg1 {
    #[inline(always)]
    fn default() -> TrdcHwcfg1 {
        TrdcHwcfg1(0u64 as u32)
    }
}
impl core::fmt::Debug for TrdcHwcfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TrdcHwcfg1")
            .field("did", &self.did())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrdcHwcfg1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TrdcHwcfg1 {
            did: u8,
        }
        let proxy = TrdcHwcfg1 { did: self.did() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "TRDC IDAU Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrdcIdauCr(pub u32);
impl TrdcIdauCr {
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
    #[doc = "Configure Security Extension"]
    #[must_use]
    #[inline(always)]
    pub const fn cfgsecext(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Configure Security Extension"]
    #[inline(always)]
    pub const fn set_cfgsecext(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Secure Memory Protection Unit Disabled"]
    #[must_use]
    #[inline(always)]
    pub const fn mpusdis(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Secure Memory Protection Unit Disabled"]
    #[inline(always)]
    pub const fn set_mpusdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "NonSecure Memory Protection Unit Disabled"]
    #[must_use]
    #[inline(always)]
    pub const fn mpunsdis(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "NonSecure Memory Protection Unit Disabled"]
    #[inline(always)]
    pub const fn set_mpunsdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Security Attribution Unit Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn saudis(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Security Attribution Unit Disable"]
    #[inline(always)]
    pub const fn set_saudis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Lock Secure VTOR, Application interrupt and Reset Control Registers"]
    #[must_use]
    #[inline(always)]
    pub const fn lksvtaircr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Lock Secure VTOR, Application interrupt and Reset Control Registers"]
    #[inline(always)]
    pub const fn set_lksvtaircr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Lock Nonsecure Vector Table Offset Register"]
    #[must_use]
    #[inline(always)]
    pub const fn lknsvtor(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Lock Nonsecure Vector Table Offset Register"]
    #[inline(always)]
    pub const fn set_lknsvtor(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Lock Secure MPU"]
    #[must_use]
    #[inline(always)]
    pub const fn lksmpu(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Lock Secure MPU"]
    #[inline(always)]
    pub const fn set_lksmpu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Lock Nonsecure MPU"]
    #[must_use]
    #[inline(always)]
    pub const fn lknsmpu(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Lock Nonsecure MPU"]
    #[inline(always)]
    pub const fn set_lknsmpu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Lock SAU"]
    #[must_use]
    #[inline(always)]
    pub const fn lksau(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Lock SAU"]
    #[inline(always)]
    pub const fn set_lksau(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Processor current security"]
    #[must_use]
    #[inline(always)]
    pub const fn pcurrns(&self) -> super::vals::Pcurrns {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Pcurrns::from_bits(val as u8)
    }
    #[doc = "Processor current security"]
    #[inline(always)]
    pub const fn set_pcurrns(&mut self, val: super::vals::Pcurrns) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for TrdcIdauCr {
    #[inline(always)]
    fn default() -> TrdcIdauCr {
        TrdcIdauCr(8u64 as u32)
    }
}
impl core::fmt::Debug for TrdcIdauCr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TrdcIdauCr")
            .field("vld", &self.vld())
            .field("cfgsecext", &self.cfgsecext())
            .field("mpusdis", &self.mpusdis())
            .field("mpunsdis", &self.mpunsdis())
            .field("saudis", &self.saudis())
            .field("lksvtaircr", &self.lksvtaircr())
            .field("lknsvtor", &self.lknsvtor())
            .field("lksmpu", &self.lksmpu())
            .field("lknsmpu", &self.lknsmpu())
            .field("lksau", &self.lksau())
            .field("pcurrns", &self.pcurrns())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrdcIdauCr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TrdcIdauCr {
            vld: bool,
            cfgsecext: bool,
            mpusdis: bool,
            mpunsdis: bool,
            saudis: bool,
            lksvtaircr: bool,
            lknsvtor: bool,
            lksmpu: bool,
            lknsmpu: bool,
            lksau: bool,
            pcurrns: super::vals::Pcurrns,
        }
        let proxy = TrdcIdauCr {
            vld: self.vld(),
            cfgsecext: self.cfgsecext(),
            mpusdis: self.mpusdis(),
            mpunsdis: self.mpunsdis(),
            saudis: self.saudis(),
            lksvtaircr: self.lksvtaircr(),
            lknsvtor: self.lknsvtor(),
            lksmpu: self.lksmpu(),
            lknsmpu: self.lknsmpu(),
            lksau: self.lksau(),
            pcurrns: self.pcurrns(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
