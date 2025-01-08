#[doc = "Error Injection On 8 bits ECC code Of OCRAM Bank0 Write Data"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ErrEccInj0(pub u32);
impl ErrEccInj0 {
    #[doc = "Error Injection On 8 bits ECC code Of OCRAM Bank0 Write Data"]
    #[must_use]
    #[inline(always)]
    pub const fn err_ecc_inj(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Error Injection On 8 bits ECC code Of OCRAM Bank0 Write Data"]
    #[inline(always)]
    pub const fn set_err_ecc_inj(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ErrEccInj0 {
    #[inline(always)]
    fn default() -> ErrEccInj0 {
        ErrEccInj0(0u64 as u32)
    }
}
impl core::fmt::Debug for ErrEccInj0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ErrEccInj0")
            .field("err_ecc_inj", &self.err_ecc_inj())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ErrEccInj0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ErrEccInj0 {
            err_ecc_inj: u8,
        }
        let proxy = ErrEccInj0 {
            err_ecc_inj: self.err_ecc_inj(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Error Injection On 8 bits ECC code Of OCRAM Bank1 Write Data"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ErrEccInj1(pub u32);
impl ErrEccInj1 {
    #[doc = "Error Injection On 8 bits ECC code Of OCRAM Bank1 Write Data"]
    #[must_use]
    #[inline(always)]
    pub const fn err_ecc_inj(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Error Injection On 8 bits ECC code Of OCRAM Bank1 Write Data"]
    #[inline(always)]
    pub const fn set_err_ecc_inj(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ErrEccInj1 {
    #[inline(always)]
    fn default() -> ErrEccInj1 {
        ErrEccInj1(0u64 as u32)
    }
}
impl core::fmt::Debug for ErrEccInj1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ErrEccInj1")
            .field("err_ecc_inj", &self.err_ecc_inj())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ErrEccInj1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ErrEccInj1 {
            err_ecc_inj: u8,
        }
        let proxy = ErrEccInj1 {
            err_ecc_inj: self.err_ecc_inj(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Error Injection On 8 bits ECC code Of OCRAM Bank2 Write Data"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ErrEccInj2(pub u32);
impl ErrEccInj2 {
    #[doc = "Error Injection On 8 bits ECC code Of OCRAM Bank2 Write Data"]
    #[must_use]
    #[inline(always)]
    pub const fn err_ecc_inj(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Error Injection On 8 bits ECC code Of OCRAM Bank2 Write Data"]
    #[inline(always)]
    pub const fn set_err_ecc_inj(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ErrEccInj2 {
    #[inline(always)]
    fn default() -> ErrEccInj2 {
        ErrEccInj2(0u64 as u32)
    }
}
impl core::fmt::Debug for ErrEccInj2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ErrEccInj2")
            .field("err_ecc_inj", &self.err_ecc_inj())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ErrEccInj2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ErrEccInj2 {
            err_ecc_inj: u8,
        }
        let proxy = ErrEccInj2 {
            err_ecc_inj: self.err_ecc_inj(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Error Injection On 8 bits ECC code Of OCRAM Bank3 Write Data"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ErrEccInj3(pub u32);
impl ErrEccInj3 {
    #[doc = "Error Injection On 8 bits ECC code Of OCRAM Bank3 Write Data"]
    #[must_use]
    #[inline(always)]
    pub const fn err_ecc_inj(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Error Injection On 8 bits ECC code Of OCRAM Bank3 Write Data"]
    #[inline(always)]
    pub const fn set_err_ecc_inj(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ErrEccInj3 {
    #[inline(always)]
    fn default() -> ErrEccInj3 {
        ErrEccInj3(0u64 as u32)
    }
}
impl core::fmt::Debug for ErrEccInj3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ErrEccInj3")
            .field("err_ecc_inj", &self.err_ecc_inj())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ErrEccInj3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ErrEccInj3 {
            err_ecc_inj: u8,
        }
        let proxy = ErrEccInj3 {
            err_ecc_inj: self.err_ecc_inj(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Error Interrupt Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ErrSigEn(pub u32);
impl ErrSigEn {
    #[doc = "Single Bit Error Interrupt Enable On OCRAM Bank0"]
    #[must_use]
    #[inline(always)]
    pub const fn single_err0_sig_en(&self) -> super::vals::SingleErr0SigEn {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SingleErr0SigEn::from_bits(val as u8)
    }
    #[doc = "Single Bit Error Interrupt Enable On OCRAM Bank0"]
    #[inline(always)]
    pub const fn set_single_err0_sig_en(&mut self, val: super::vals::SingleErr0SigEn) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Single Bit Error Interrupt Enable On OCRAM Bank1"]
    #[must_use]
    #[inline(always)]
    pub const fn single_err1_sig_en(&self) -> super::vals::SingleErr1SigEn {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SingleErr1SigEn::from_bits(val as u8)
    }
    #[doc = "Single Bit Error Interrupt Enable On OCRAM Bank1"]
    #[inline(always)]
    pub const fn set_single_err1_sig_en(&mut self, val: super::vals::SingleErr1SigEn) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Single Bit Error Interrupt Enable On OCRAM Bank2"]
    #[must_use]
    #[inline(always)]
    pub const fn single_err2_sig_en(&self) -> super::vals::SingleErr2SigEn {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SingleErr2SigEn::from_bits(val as u8)
    }
    #[doc = "Single Bit Error Interrupt Enable On OCRAM Bank2"]
    #[inline(always)]
    pub const fn set_single_err2_sig_en(&mut self, val: super::vals::SingleErr2SigEn) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Single Bit Error Interrupt Enable On OCRAM Bank3"]
    #[must_use]
    #[inline(always)]
    pub const fn single_err3_sig_en(&self) -> super::vals::SingleErr3SigEn {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SingleErr3SigEn::from_bits(val as u8)
    }
    #[doc = "Single Bit Error Interrupt Enable On OCRAM Bank3"]
    #[inline(always)]
    pub const fn set_single_err3_sig_en(&mut self, val: super::vals::SingleErr3SigEn) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Multiple Bits Error Interrupt Enable On OCRAM Bank0"]
    #[must_use]
    #[inline(always)]
    pub const fn multi_err0_sig_en(&self) -> super::vals::MultiErr0SigEn {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::MultiErr0SigEn::from_bits(val as u8)
    }
    #[doc = "Multiple Bits Error Interrupt Enable On OCRAM Bank0"]
    #[inline(always)]
    pub const fn set_multi_err0_sig_en(&mut self, val: super::vals::MultiErr0SigEn) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Multiple Bits Error Interrupt Enable On OCRAM Bank1"]
    #[must_use]
    #[inline(always)]
    pub const fn multi_err1_sig_en(&self) -> super::vals::MultiErr1SigEn {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::MultiErr1SigEn::from_bits(val as u8)
    }
    #[doc = "Multiple Bits Error Interrupt Enable On OCRAM Bank1"]
    #[inline(always)]
    pub const fn set_multi_err1_sig_en(&mut self, val: super::vals::MultiErr1SigEn) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Multiple Bits Error Interrupt Enable On OCRAM Bank2"]
    #[must_use]
    #[inline(always)]
    pub const fn multi_err2_sig_en(&self) -> super::vals::MultiErr2SigEn {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::MultiErr2SigEn::from_bits(val as u8)
    }
    #[doc = "Multiple Bits Error Interrupt Enable On OCRAM Bank2"]
    #[inline(always)]
    pub const fn set_multi_err2_sig_en(&mut self, val: super::vals::MultiErr2SigEn) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Multiple Bits Error Interrupt Enable On OCRAM Bank3"]
    #[must_use]
    #[inline(always)]
    pub const fn multi_err3_sig_en(&self) -> super::vals::MultiErr3SigEn {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::MultiErr3SigEn::from_bits(val as u8)
    }
    #[doc = "Multiple Bits Error Interrupt Enable On OCRAM Bank3"]
    #[inline(always)]
    pub const fn set_multi_err3_sig_en(&mut self, val: super::vals::MultiErr3SigEn) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "AXI Strobe Error Interrupt Enable On OCRAM Bank0"]
    #[must_use]
    #[inline(always)]
    pub const fn strb_err0_sig_en(&self) -> super::vals::StrbErr0SigEn {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::StrbErr0SigEn::from_bits(val as u8)
    }
    #[doc = "AXI Strobe Error Interrupt Enable On OCRAM Bank0"]
    #[inline(always)]
    pub const fn set_strb_err0_sig_en(&mut self, val: super::vals::StrbErr0SigEn) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "AXI Strobe Error Interrupt Enable On OCRAM Bank1"]
    #[must_use]
    #[inline(always)]
    pub const fn strb_err1_sig_en(&self) -> super::vals::StrbErr1SigEn {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::StrbErr1SigEn::from_bits(val as u8)
    }
    #[doc = "AXI Strobe Error Interrupt Enable On OCRAM Bank1"]
    #[inline(always)]
    pub const fn set_strb_err1_sig_en(&mut self, val: super::vals::StrbErr1SigEn) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "AXI Strobe Error Interrupt Enable On OCRAM Bank2"]
    #[must_use]
    #[inline(always)]
    pub const fn strb_err2_sig_en(&self) -> super::vals::StrbErr2SigEn {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::StrbErr2SigEn::from_bits(val as u8)
    }
    #[doc = "AXI Strobe Error Interrupt Enable On OCRAM Bank2"]
    #[inline(always)]
    pub const fn set_strb_err2_sig_en(&mut self, val: super::vals::StrbErr2SigEn) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "AXI Strobe Error Interrupt Enable On OCRAM Bank3"]
    #[must_use]
    #[inline(always)]
    pub const fn strb_err3_sig_en(&self) -> super::vals::StrbErr3SigEn {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::StrbErr3SigEn::from_bits(val as u8)
    }
    #[doc = "AXI Strobe Error Interrupt Enable On OCRAM Bank3"]
    #[inline(always)]
    pub const fn set_strb_err3_sig_en(&mut self, val: super::vals::StrbErr3SigEn) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "OCRAM Access Error Interrupt Enable On Bank0"]
    #[must_use]
    #[inline(always)]
    pub const fn addr_err0_sig_en(&self) -> super::vals::AddrErr0SigEn {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::AddrErr0SigEn::from_bits(val as u8)
    }
    #[doc = "OCRAM Access Error Interrupt Enable On Bank0"]
    #[inline(always)]
    pub const fn set_addr_err0_sig_en(&mut self, val: super::vals::AddrErr0SigEn) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "OCRAM Access Error Interrupt Enable On Bank1"]
    #[must_use]
    #[inline(always)]
    pub const fn addr_err1_sig_en(&self) -> super::vals::AddrErr1SigEn {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::AddrErr1SigEn::from_bits(val as u8)
    }
    #[doc = "OCRAM Access Error Interrupt Enable On Bank1"]
    #[inline(always)]
    pub const fn set_addr_err1_sig_en(&mut self, val: super::vals::AddrErr1SigEn) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "OCRAM Access Error Interrupt Enable On Bank2"]
    #[must_use]
    #[inline(always)]
    pub const fn addr_err2_sig_en(&self) -> super::vals::AddrErr2SigEn {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::AddrErr2SigEn::from_bits(val as u8)
    }
    #[doc = "OCRAM Access Error Interrupt Enable On Bank2"]
    #[inline(always)]
    pub const fn set_addr_err2_sig_en(&mut self, val: super::vals::AddrErr2SigEn) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "OCRAM Access Error Interrupt Enable On Bank3"]
    #[must_use]
    #[inline(always)]
    pub const fn addr_err3_sig_en(&self) -> super::vals::AddrErr3SigEn {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::AddrErr3SigEn::from_bits(val as u8)
    }
    #[doc = "OCRAM Access Error Interrupt Enable On Bank3"]
    #[inline(always)]
    pub const fn set_addr_err3_sig_en(&mut self, val: super::vals::AddrErr3SigEn) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for ErrSigEn {
    #[inline(always)]
    fn default() -> ErrSigEn {
        ErrSigEn(0u64 as u32)
    }
}
impl core::fmt::Debug for ErrSigEn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ErrSigEn")
            .field("single_err0_sig_en", &self.single_err0_sig_en())
            .field("single_err1_sig_en", &self.single_err1_sig_en())
            .field("single_err2_sig_en", &self.single_err2_sig_en())
            .field("single_err3_sig_en", &self.single_err3_sig_en())
            .field("multi_err0_sig_en", &self.multi_err0_sig_en())
            .field("multi_err1_sig_en", &self.multi_err1_sig_en())
            .field("multi_err2_sig_en", &self.multi_err2_sig_en())
            .field("multi_err3_sig_en", &self.multi_err3_sig_en())
            .field("strb_err0_sig_en", &self.strb_err0_sig_en())
            .field("strb_err1_sig_en", &self.strb_err1_sig_en())
            .field("strb_err2_sig_en", &self.strb_err2_sig_en())
            .field("strb_err3_sig_en", &self.strb_err3_sig_en())
            .field("addr_err0_sig_en", &self.addr_err0_sig_en())
            .field("addr_err1_sig_en", &self.addr_err1_sig_en())
            .field("addr_err2_sig_en", &self.addr_err2_sig_en())
            .field("addr_err3_sig_en", &self.addr_err3_sig_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ErrSigEn {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ErrSigEn {
            single_err0_sig_en: super::vals::SingleErr0SigEn,
            single_err1_sig_en: super::vals::SingleErr1SigEn,
            single_err2_sig_en: super::vals::SingleErr2SigEn,
            single_err3_sig_en: super::vals::SingleErr3SigEn,
            multi_err0_sig_en: super::vals::MultiErr0SigEn,
            multi_err1_sig_en: super::vals::MultiErr1SigEn,
            multi_err2_sig_en: super::vals::MultiErr2SigEn,
            multi_err3_sig_en: super::vals::MultiErr3SigEn,
            strb_err0_sig_en: super::vals::StrbErr0SigEn,
            strb_err1_sig_en: super::vals::StrbErr1SigEn,
            strb_err2_sig_en: super::vals::StrbErr2SigEn,
            strb_err3_sig_en: super::vals::StrbErr3SigEn,
            addr_err0_sig_en: super::vals::AddrErr0SigEn,
            addr_err1_sig_en: super::vals::AddrErr1SigEn,
            addr_err2_sig_en: super::vals::AddrErr2SigEn,
            addr_err3_sig_en: super::vals::AddrErr3SigEn,
        }
        let proxy = ErrSigEn {
            single_err0_sig_en: self.single_err0_sig_en(),
            single_err1_sig_en: self.single_err1_sig_en(),
            single_err2_sig_en: self.single_err2_sig_en(),
            single_err3_sig_en: self.single_err3_sig_en(),
            multi_err0_sig_en: self.multi_err0_sig_en(),
            multi_err1_sig_en: self.multi_err1_sig_en(),
            multi_err2_sig_en: self.multi_err2_sig_en(),
            multi_err3_sig_en: self.multi_err3_sig_en(),
            strb_err0_sig_en: self.strb_err0_sig_en(),
            strb_err1_sig_en: self.strb_err1_sig_en(),
            strb_err2_sig_en: self.strb_err2_sig_en(),
            strb_err3_sig_en: self.strb_err3_sig_en(),
            addr_err0_sig_en: self.addr_err0_sig_en(),
            addr_err1_sig_en: self.addr_err1_sig_en(),
            addr_err2_sig_en: self.addr_err2_sig_en(),
            addr_err3_sig_en: self.addr_err3_sig_en(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Error Interrupt Status Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ErrStatEn(pub u32);
impl ErrStatEn {
    #[doc = "Single Bit Error Status Enable On OCRAM Bank0"]
    #[must_use]
    #[inline(always)]
    pub const fn single_err0_stat_en(&self) -> super::vals::SingleErr0StatEn {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SingleErr0StatEn::from_bits(val as u8)
    }
    #[doc = "Single Bit Error Status Enable On OCRAM Bank0"]
    #[inline(always)]
    pub const fn set_single_err0_stat_en(&mut self, val: super::vals::SingleErr0StatEn) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Single Bit Error Status Enable On OCRAM Bank1"]
    #[must_use]
    #[inline(always)]
    pub const fn single_err1_stat_en(&self) -> super::vals::SingleErr1StatEn {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SingleErr1StatEn::from_bits(val as u8)
    }
    #[doc = "Single Bit Error Status Enable On OCRAM Bank1"]
    #[inline(always)]
    pub const fn set_single_err1_stat_en(&mut self, val: super::vals::SingleErr1StatEn) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Single Bit Error Status Enable On OCRAM Bank2"]
    #[must_use]
    #[inline(always)]
    pub const fn single_err2_stat_en(&self) -> super::vals::SingleErr2StatEn {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SingleErr2StatEn::from_bits(val as u8)
    }
    #[doc = "Single Bit Error Status Enable On OCRAM Bank2"]
    #[inline(always)]
    pub const fn set_single_err2_stat_en(&mut self, val: super::vals::SingleErr2StatEn) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Single Bit Error Status Enable On OCRAM Bank3"]
    #[must_use]
    #[inline(always)]
    pub const fn single_err3_stat_en(&self) -> super::vals::SingleErr3StatEn {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SingleErr3StatEn::from_bits(val as u8)
    }
    #[doc = "Single Bit Error Status Enable On OCRAM Bank3"]
    #[inline(always)]
    pub const fn set_single_err3_stat_en(&mut self, val: super::vals::SingleErr3StatEn) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Multiple Bits Error Status Enable On OCRAM Bank0"]
    #[must_use]
    #[inline(always)]
    pub const fn multi_err0_stat_en(&self) -> super::vals::MultiErr0StatEn {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::MultiErr0StatEn::from_bits(val as u8)
    }
    #[doc = "Multiple Bits Error Status Enable On OCRAM Bank0"]
    #[inline(always)]
    pub const fn set_multi_err0_stat_en(&mut self, val: super::vals::MultiErr0StatEn) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Multiple Bits Error Status Enable On OCRAM Bank1"]
    #[must_use]
    #[inline(always)]
    pub const fn multi_err1_stat_en(&self) -> super::vals::MultiErr1StatEn {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::MultiErr1StatEn::from_bits(val as u8)
    }
    #[doc = "Multiple Bits Error Status Enable On OCRAM Bank1"]
    #[inline(always)]
    pub const fn set_multi_err1_stat_en(&mut self, val: super::vals::MultiErr1StatEn) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Multiple Bits Error Status Enable On OCRAM Bank2"]
    #[must_use]
    #[inline(always)]
    pub const fn multi_err2_stat_en(&self) -> super::vals::MultiErr2StatEn {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::MultiErr2StatEn::from_bits(val as u8)
    }
    #[doc = "Multiple Bits Error Status Enable On OCRAM Bank2"]
    #[inline(always)]
    pub const fn set_multi_err2_stat_en(&mut self, val: super::vals::MultiErr2StatEn) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Multiple Bits Error Status Enable On OCRAM Bank3"]
    #[must_use]
    #[inline(always)]
    pub const fn multi_err3_stat_en(&self) -> super::vals::MultiErr3StatEn {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::MultiErr3StatEn::from_bits(val as u8)
    }
    #[doc = "Multiple Bits Error Status Enable On OCRAM Bank3"]
    #[inline(always)]
    pub const fn set_multi_err3_stat_en(&mut self, val: super::vals::MultiErr3StatEn) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "AXI Strobe Error Status Enable On OCRAM Bank0"]
    #[must_use]
    #[inline(always)]
    pub const fn strb_err0_stat_en(&self) -> super::vals::StrbErr0StatEn {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::StrbErr0StatEn::from_bits(val as u8)
    }
    #[doc = "AXI Strobe Error Status Enable On OCRAM Bank0"]
    #[inline(always)]
    pub const fn set_strb_err0_stat_en(&mut self, val: super::vals::StrbErr0StatEn) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "AXI Strobe Error Status Enable On OCRAM Bank1"]
    #[must_use]
    #[inline(always)]
    pub const fn strb_err1_stat_en(&self) -> super::vals::StrbErr1StatEn {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::StrbErr1StatEn::from_bits(val as u8)
    }
    #[doc = "AXI Strobe Error Status Enable On OCRAM Bank1"]
    #[inline(always)]
    pub const fn set_strb_err1_stat_en(&mut self, val: super::vals::StrbErr1StatEn) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "AXI Strobe Error Status Enable On OCRAM Bank2"]
    #[must_use]
    #[inline(always)]
    pub const fn strb_err2_stat_en(&self) -> super::vals::StrbErr2StatEn {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::StrbErr2StatEn::from_bits(val as u8)
    }
    #[doc = "AXI Strobe Error Status Enable On OCRAM Bank2"]
    #[inline(always)]
    pub const fn set_strb_err2_stat_en(&mut self, val: super::vals::StrbErr2StatEn) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "AXI Strobe Error Status Enable On OCRAM Bank3"]
    #[must_use]
    #[inline(always)]
    pub const fn strb_err3_stat_en(&self) -> super::vals::StrbErr3StatEn {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::StrbErr3StatEn::from_bits(val as u8)
    }
    #[doc = "AXI Strobe Error Status Enable On OCRAM Bank3"]
    #[inline(always)]
    pub const fn set_strb_err3_stat_en(&mut self, val: super::vals::StrbErr3StatEn) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "OCRAM Access Error Status Enable On Bank0"]
    #[must_use]
    #[inline(always)]
    pub const fn addr_err0_stat_en(&self) -> super::vals::AddrErr0StatEn {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::AddrErr0StatEn::from_bits(val as u8)
    }
    #[doc = "OCRAM Access Error Status Enable On Bank0"]
    #[inline(always)]
    pub const fn set_addr_err0_stat_en(&mut self, val: super::vals::AddrErr0StatEn) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "OCRAM Access Error Status Enable On Bank1"]
    #[must_use]
    #[inline(always)]
    pub const fn addr_err1_stat_en(&self) -> super::vals::AddrErr1StatEn {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::AddrErr1StatEn::from_bits(val as u8)
    }
    #[doc = "OCRAM Access Error Status Enable On Bank1"]
    #[inline(always)]
    pub const fn set_addr_err1_stat_en(&mut self, val: super::vals::AddrErr1StatEn) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "OCRAM Access Error Status Enable On Bank2"]
    #[must_use]
    #[inline(always)]
    pub const fn addr_err2_stat_en(&self) -> super::vals::AddrErr2StatEn {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::AddrErr2StatEn::from_bits(val as u8)
    }
    #[doc = "OCRAM Access Error Status Enable On Bank2"]
    #[inline(always)]
    pub const fn set_addr_err2_stat_en(&mut self, val: super::vals::AddrErr2StatEn) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "OCRAM Access Error Status Enable On Bank3"]
    #[must_use]
    #[inline(always)]
    pub const fn addr_err3_stat_en(&self) -> super::vals::AddrErr3StatEn {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::AddrErr3StatEn::from_bits(val as u8)
    }
    #[doc = "OCRAM Access Error Status Enable On Bank3"]
    #[inline(always)]
    pub const fn set_addr_err3_stat_en(&mut self, val: super::vals::AddrErr3StatEn) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for ErrStatEn {
    #[inline(always)]
    fn default() -> ErrStatEn {
        ErrStatEn(0u64 as u32)
    }
}
impl core::fmt::Debug for ErrStatEn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ErrStatEn")
            .field("single_err0_stat_en", &self.single_err0_stat_en())
            .field("single_err1_stat_en", &self.single_err1_stat_en())
            .field("single_err2_stat_en", &self.single_err2_stat_en())
            .field("single_err3_stat_en", &self.single_err3_stat_en())
            .field("multi_err0_stat_en", &self.multi_err0_stat_en())
            .field("multi_err1_stat_en", &self.multi_err1_stat_en())
            .field("multi_err2_stat_en", &self.multi_err2_stat_en())
            .field("multi_err3_stat_en", &self.multi_err3_stat_en())
            .field("strb_err0_stat_en", &self.strb_err0_stat_en())
            .field("strb_err1_stat_en", &self.strb_err1_stat_en())
            .field("strb_err2_stat_en", &self.strb_err2_stat_en())
            .field("strb_err3_stat_en", &self.strb_err3_stat_en())
            .field("addr_err0_stat_en", &self.addr_err0_stat_en())
            .field("addr_err1_stat_en", &self.addr_err1_stat_en())
            .field("addr_err2_stat_en", &self.addr_err2_stat_en())
            .field("addr_err3_stat_en", &self.addr_err3_stat_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ErrStatEn {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ErrStatEn {
            single_err0_stat_en: super::vals::SingleErr0StatEn,
            single_err1_stat_en: super::vals::SingleErr1StatEn,
            single_err2_stat_en: super::vals::SingleErr2StatEn,
            single_err3_stat_en: super::vals::SingleErr3StatEn,
            multi_err0_stat_en: super::vals::MultiErr0StatEn,
            multi_err1_stat_en: super::vals::MultiErr1StatEn,
            multi_err2_stat_en: super::vals::MultiErr2StatEn,
            multi_err3_stat_en: super::vals::MultiErr3StatEn,
            strb_err0_stat_en: super::vals::StrbErr0StatEn,
            strb_err1_stat_en: super::vals::StrbErr1StatEn,
            strb_err2_stat_en: super::vals::StrbErr2StatEn,
            strb_err3_stat_en: super::vals::StrbErr3StatEn,
            addr_err0_stat_en: super::vals::AddrErr0StatEn,
            addr_err1_stat_en: super::vals::AddrErr1StatEn,
            addr_err2_stat_en: super::vals::AddrErr2StatEn,
            addr_err3_stat_en: super::vals::AddrErr3StatEn,
        }
        let proxy = ErrStatEn {
            single_err0_stat_en: self.single_err0_stat_en(),
            single_err1_stat_en: self.single_err1_stat_en(),
            single_err2_stat_en: self.single_err2_stat_en(),
            single_err3_stat_en: self.single_err3_stat_en(),
            multi_err0_stat_en: self.multi_err0_stat_en(),
            multi_err1_stat_en: self.multi_err1_stat_en(),
            multi_err2_stat_en: self.multi_err2_stat_en(),
            multi_err3_stat_en: self.multi_err3_stat_en(),
            strb_err0_stat_en: self.strb_err0_stat_en(),
            strb_err1_stat_en: self.strb_err1_stat_en(),
            strb_err2_stat_en: self.strb_err2_stat_en(),
            strb_err3_stat_en: self.strb_err3_stat_en(),
            addr_err0_stat_en: self.addr_err0_stat_en(),
            addr_err1_stat_en: self.addr_err1_stat_en(),
            addr_err2_stat_en: self.addr_err2_stat_en(),
            addr_err3_stat_en: self.addr_err3_stat_en(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Error Interrupt Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ErrStatus(pub u32);
impl ErrStatus {
    #[doc = "Single Bit Error On OCRAM Bank0"]
    #[must_use]
    #[inline(always)]
    pub const fn single_err0(&self) -> super::vals::SingleErr0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SingleErr0::from_bits(val as u8)
    }
    #[doc = "Single Bit Error On OCRAM Bank0"]
    #[inline(always)]
    pub const fn set_single_err0(&mut self, val: super::vals::SingleErr0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Single Bit Error On OCRAM Bank1"]
    #[must_use]
    #[inline(always)]
    pub const fn single_err1(&self) -> super::vals::SingleErr1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SingleErr1::from_bits(val as u8)
    }
    #[doc = "Single Bit Error On OCRAM Bank1"]
    #[inline(always)]
    pub const fn set_single_err1(&mut self, val: super::vals::SingleErr1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Single Bit Error On OCRAM Bank2"]
    #[must_use]
    #[inline(always)]
    pub const fn single_err2(&self) -> super::vals::SingleErr2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SingleErr2::from_bits(val as u8)
    }
    #[doc = "Single Bit Error On OCRAM Bank2"]
    #[inline(always)]
    pub const fn set_single_err2(&mut self, val: super::vals::SingleErr2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Single Bit Error On OCRAM Bank3"]
    #[must_use]
    #[inline(always)]
    pub const fn single_err3(&self) -> super::vals::SingleErr3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SingleErr3::from_bits(val as u8)
    }
    #[doc = "Single Bit Error On OCRAM Bank3"]
    #[inline(always)]
    pub const fn set_single_err3(&mut self, val: super::vals::SingleErr3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Multiple Bits Error On OCRAM Bank0"]
    #[must_use]
    #[inline(always)]
    pub const fn multi_err0(&self) -> super::vals::MultiErr0 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::MultiErr0::from_bits(val as u8)
    }
    #[doc = "Multiple Bits Error On OCRAM Bank0"]
    #[inline(always)]
    pub const fn set_multi_err0(&mut self, val: super::vals::MultiErr0) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Multiple Bits Error On OCRAM Bank1"]
    #[must_use]
    #[inline(always)]
    pub const fn multi_err1(&self) -> super::vals::MultiErr1 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::MultiErr1::from_bits(val as u8)
    }
    #[doc = "Multiple Bits Error On OCRAM Bank1"]
    #[inline(always)]
    pub const fn set_multi_err1(&mut self, val: super::vals::MultiErr1) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Multiple Bits Error On OCRAM Bank2"]
    #[must_use]
    #[inline(always)]
    pub const fn multi_err2(&self) -> super::vals::MultiErr2 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::MultiErr2::from_bits(val as u8)
    }
    #[doc = "Multiple Bits Error On OCRAM Bank2"]
    #[inline(always)]
    pub const fn set_multi_err2(&mut self, val: super::vals::MultiErr2) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Multiple Bits Error On OCRAM Bank3"]
    #[must_use]
    #[inline(always)]
    pub const fn multi_err3(&self) -> super::vals::MultiErr3 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::MultiErr3::from_bits(val as u8)
    }
    #[doc = "Multiple Bits Error On OCRAM Bank3"]
    #[inline(always)]
    pub const fn set_multi_err3(&mut self, val: super::vals::MultiErr3) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "AXI Strobe Error On OCRAM Bank0"]
    #[must_use]
    #[inline(always)]
    pub const fn strb_err0(&self) -> super::vals::StrbErr0 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::StrbErr0::from_bits(val as u8)
    }
    #[doc = "AXI Strobe Error On OCRAM Bank0"]
    #[inline(always)]
    pub const fn set_strb_err0(&mut self, val: super::vals::StrbErr0) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "AXI Strobe Error On OCRAM Bank1"]
    #[must_use]
    #[inline(always)]
    pub const fn strb_err1(&self) -> super::vals::StrbErr1 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::StrbErr1::from_bits(val as u8)
    }
    #[doc = "AXI Strobe Error On OCRAM Bank1"]
    #[inline(always)]
    pub const fn set_strb_err1(&mut self, val: super::vals::StrbErr1) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "AXI Strobe Error On OCRAM Bank2"]
    #[must_use]
    #[inline(always)]
    pub const fn strb_err2(&self) -> super::vals::StrbErr2 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::StrbErr2::from_bits(val as u8)
    }
    #[doc = "AXI Strobe Error On OCRAM Bank2"]
    #[inline(always)]
    pub const fn set_strb_err2(&mut self, val: super::vals::StrbErr2) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "AXI Strobe Error On OCRAM Bank3"]
    #[must_use]
    #[inline(always)]
    pub const fn strb_err3(&self) -> super::vals::StrbErr3 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::StrbErr3::from_bits(val as u8)
    }
    #[doc = "AXI Strobe Error On OCRAM Bank3"]
    #[inline(always)]
    pub const fn set_strb_err3(&mut self, val: super::vals::StrbErr3) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "OCRAM Access Error On Bank0"]
    #[must_use]
    #[inline(always)]
    pub const fn addr_err0(&self) -> super::vals::AddrErr0 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::AddrErr0::from_bits(val as u8)
    }
    #[doc = "OCRAM Access Error On Bank0"]
    #[inline(always)]
    pub const fn set_addr_err0(&mut self, val: super::vals::AddrErr0) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "OCRAM Access Error On Bank1"]
    #[must_use]
    #[inline(always)]
    pub const fn addr_err1(&self) -> super::vals::AddrErr1 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::AddrErr1::from_bits(val as u8)
    }
    #[doc = "OCRAM Access Error On Bank1"]
    #[inline(always)]
    pub const fn set_addr_err1(&mut self, val: super::vals::AddrErr1) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "OCRAM Access Error On Bank2"]
    #[must_use]
    #[inline(always)]
    pub const fn addr_err2(&self) -> super::vals::AddrErr2 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::AddrErr2::from_bits(val as u8)
    }
    #[doc = "OCRAM Access Error On Bank2"]
    #[inline(always)]
    pub const fn set_addr_err2(&mut self, val: super::vals::AddrErr2) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "OCRAM Access Error On Bank3"]
    #[must_use]
    #[inline(always)]
    pub const fn addr_err3(&self) -> super::vals::AddrErr3 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::AddrErr3::from_bits(val as u8)
    }
    #[doc = "OCRAM Access Error On Bank3"]
    #[inline(always)]
    pub const fn set_addr_err3(&mut self, val: super::vals::AddrErr3) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for ErrStatus {
    #[inline(always)]
    fn default() -> ErrStatus {
        ErrStatus(0u64 as u32)
    }
}
impl core::fmt::Debug for ErrStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ErrStatus")
            .field("single_err0", &self.single_err0())
            .field("single_err1", &self.single_err1())
            .field("single_err2", &self.single_err2())
            .field("single_err3", &self.single_err3())
            .field("multi_err0", &self.multi_err0())
            .field("multi_err1", &self.multi_err1())
            .field("multi_err2", &self.multi_err2())
            .field("multi_err3", &self.multi_err3())
            .field("strb_err0", &self.strb_err0())
            .field("strb_err1", &self.strb_err1())
            .field("strb_err2", &self.strb_err2())
            .field("strb_err3", &self.strb_err3())
            .field("addr_err0", &self.addr_err0())
            .field("addr_err1", &self.addr_err1())
            .field("addr_err2", &self.addr_err2())
            .field("addr_err3", &self.addr_err3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ErrStatus {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ErrStatus {
            single_err0: super::vals::SingleErr0,
            single_err1: super::vals::SingleErr1,
            single_err2: super::vals::SingleErr2,
            single_err3: super::vals::SingleErr3,
            multi_err0: super::vals::MultiErr0,
            multi_err1: super::vals::MultiErr1,
            multi_err2: super::vals::MultiErr2,
            multi_err3: super::vals::MultiErr3,
            strb_err0: super::vals::StrbErr0,
            strb_err1: super::vals::StrbErr1,
            strb_err2: super::vals::StrbErr2,
            strb_err3: super::vals::StrbErr3,
            addr_err0: super::vals::AddrErr0,
            addr_err1: super::vals::AddrErr1,
            addr_err2: super::vals::AddrErr2,
            addr_err3: super::vals::AddrErr3,
        }
        let proxy = ErrStatus {
            single_err0: self.single_err0(),
            single_err1: self.single_err1(),
            single_err2: self.single_err2(),
            single_err3: self.single_err3(),
            multi_err0: self.multi_err0(),
            multi_err1: self.multi_err1(),
            multi_err2: self.multi_err2(),
            multi_err3: self.multi_err3(),
            strb_err0: self.strb_err0(),
            strb_err1: self.strb_err1(),
            strb_err2: self.strb_err2(),
            strb_err3: self.strb_err3(),
            addr_err0: self.addr_err0(),
            addr_err1: self.addr_err1(),
            addr_err2: self.addr_err2(),
            addr_err3: self.addr_err3(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Multiple Error Address And ECC code On OCRAM Bank0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MultiErrAddrEcc0(pub u32);
impl MultiErrAddrEcc0 {
    #[doc = "Multiple Error ECC code On OCRAM Bank0"]
    #[must_use]
    #[inline(always)]
    pub const fn multi_err_ecc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Multiple Error ECC code On OCRAM Bank0"]
    #[inline(always)]
    pub const fn set_multi_err_ecc(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Multiple Error Address On OCRAM Bank0"]
    #[must_use]
    #[inline(always)]
    pub const fn multi_err_addr(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x0007_ffff;
        val as u32
    }
    #[doc = "Multiple Error Address On OCRAM Bank0"]
    #[inline(always)]
    pub const fn set_multi_err_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0007_ffff << 8usize)) | (((val as u32) & 0x0007_ffff) << 8usize);
    }
}
impl Default for MultiErrAddrEcc0 {
    #[inline(always)]
    fn default() -> MultiErrAddrEcc0 {
        MultiErrAddrEcc0(0u64 as u32)
    }
}
impl core::fmt::Debug for MultiErrAddrEcc0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MultiErrAddrEcc0")
            .field("multi_err_ecc", &self.multi_err_ecc())
            .field("multi_err_addr", &self.multi_err_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MultiErrAddrEcc0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MultiErrAddrEcc0 {
            multi_err_ecc: u8,
            multi_err_addr: u32,
        }
        let proxy = MultiErrAddrEcc0 {
            multi_err_ecc: self.multi_err_ecc(),
            multi_err_addr: self.multi_err_addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Multiple Error Address And ECC code On OCRAM Bank1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MultiErrAddrEcc1(pub u32);
impl MultiErrAddrEcc1 {
    #[doc = "Multiple Error ECC code On OCRAM Bank1"]
    #[must_use]
    #[inline(always)]
    pub const fn multi_err_ecc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Multiple Error ECC code On OCRAM Bank1"]
    #[inline(always)]
    pub const fn set_multi_err_ecc(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Multiple Error Address On OCRAM Bank1"]
    #[must_use]
    #[inline(always)]
    pub const fn multi_err_addr(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x0007_ffff;
        val as u32
    }
    #[doc = "Multiple Error Address On OCRAM Bank1"]
    #[inline(always)]
    pub const fn set_multi_err_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0007_ffff << 8usize)) | (((val as u32) & 0x0007_ffff) << 8usize);
    }
}
impl Default for MultiErrAddrEcc1 {
    #[inline(always)]
    fn default() -> MultiErrAddrEcc1 {
        MultiErrAddrEcc1(0u64 as u32)
    }
}
impl core::fmt::Debug for MultiErrAddrEcc1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MultiErrAddrEcc1")
            .field("multi_err_ecc", &self.multi_err_ecc())
            .field("multi_err_addr", &self.multi_err_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MultiErrAddrEcc1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MultiErrAddrEcc1 {
            multi_err_ecc: u8,
            multi_err_addr: u32,
        }
        let proxy = MultiErrAddrEcc1 {
            multi_err_ecc: self.multi_err_ecc(),
            multi_err_addr: self.multi_err_addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Multiple Error Address And ECC code On OCRAM Bank2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MultiErrAddrEcc2(pub u32);
impl MultiErrAddrEcc2 {
    #[doc = "Multiple Error ECC code On OCRAM Bank2"]
    #[must_use]
    #[inline(always)]
    pub const fn multi_err_ecc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Multiple Error ECC code On OCRAM Bank2"]
    #[inline(always)]
    pub const fn set_multi_err_ecc(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Multiple Error Address On OCRAM Bank2"]
    #[must_use]
    #[inline(always)]
    pub const fn multi_err_addr(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x0007_ffff;
        val as u32
    }
    #[doc = "Multiple Error Address On OCRAM Bank2"]
    #[inline(always)]
    pub const fn set_multi_err_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0007_ffff << 8usize)) | (((val as u32) & 0x0007_ffff) << 8usize);
    }
}
impl Default for MultiErrAddrEcc2 {
    #[inline(always)]
    fn default() -> MultiErrAddrEcc2 {
        MultiErrAddrEcc2(0u64 as u32)
    }
}
impl core::fmt::Debug for MultiErrAddrEcc2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MultiErrAddrEcc2")
            .field("multi_err_ecc", &self.multi_err_ecc())
            .field("multi_err_addr", &self.multi_err_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MultiErrAddrEcc2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MultiErrAddrEcc2 {
            multi_err_ecc: u8,
            multi_err_addr: u32,
        }
        let proxy = MultiErrAddrEcc2 {
            multi_err_ecc: self.multi_err_ecc(),
            multi_err_addr: self.multi_err_addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Multiple Error Address And ECC code On OCRAM Bank3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MultiErrAddrEcc3(pub u32);
impl MultiErrAddrEcc3 {
    #[doc = "Multiple Error ECC code On OCRAM Bank3"]
    #[must_use]
    #[inline(always)]
    pub const fn multi_err_ecc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Multiple Error ECC code On OCRAM Bank3"]
    #[inline(always)]
    pub const fn set_multi_err_ecc(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Multiple Error Address On OCRAM Bank3"]
    #[must_use]
    #[inline(always)]
    pub const fn multi_err_addr(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x0007_ffff;
        val as u32
    }
    #[doc = "Multiple Error Address On OCRAM Bank3"]
    #[inline(always)]
    pub const fn set_multi_err_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0007_ffff << 8usize)) | (((val as u32) & 0x0007_ffff) << 8usize);
    }
}
impl Default for MultiErrAddrEcc3 {
    #[inline(always)]
    fn default() -> MultiErrAddrEcc3 {
        MultiErrAddrEcc3(0u64 as u32)
    }
}
impl core::fmt::Debug for MultiErrAddrEcc3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MultiErrAddrEcc3")
            .field("multi_err_ecc", &self.multi_err_ecc())
            .field("multi_err_addr", &self.multi_err_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MultiErrAddrEcc3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MultiErrAddrEcc3 {
            multi_err_ecc: u8,
            multi_err_addr: u32,
        }
        let proxy = MultiErrAddrEcc3 {
            multi_err_ecc: self.multi_err_ecc(),
            multi_err_addr: self.multi_err_addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Pending Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PendingStat(pub u32);
impl PendingStat {
    #[doc = "Read Data Wait Pending"]
    #[must_use]
    #[inline(always)]
    pub const fn read_data_wait_pending(&self) -> super::vals::ReadDataWaitPending {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ReadDataWaitPending::from_bits(val as u8)
    }
    #[doc = "Read Data Wait Pending"]
    #[inline(always)]
    pub const fn set_read_data_wait_pending(&mut self, val: super::vals::ReadDataWaitPending) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Read Address Pipeline Pending"]
    #[must_use]
    #[inline(always)]
    pub const fn read_addr_pipe_pending(&self) -> super::vals::ReadAddrPipePending {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::ReadAddrPipePending::from_bits(val as u8)
    }
    #[doc = "Read Address Pipeline Pending"]
    #[inline(always)]
    pub const fn set_read_addr_pipe_pending(&mut self, val: super::vals::ReadAddrPipePending) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Write Data Pipeline Pending"]
    #[must_use]
    #[inline(always)]
    pub const fn write_data_pipe_pending(&self) -> super::vals::WriteDataPipePending {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::WriteDataPipePending::from_bits(val as u8)
    }
    #[doc = "Write Data Pipeline Pending"]
    #[inline(always)]
    pub const fn set_write_data_pipe_pending(&mut self, val: super::vals::WriteDataPipePending) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Write Address Pipeline Pending"]
    #[must_use]
    #[inline(always)]
    pub const fn write_addr_pipe_pending(&self) -> super::vals::WriteAddrPipePending {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::WriteAddrPipePending::from_bits(val as u8)
    }
    #[doc = "Write Address Pipeline Pending"]
    #[inline(always)]
    pub const fn set_write_addr_pipe_pending(&mut self, val: super::vals::WriteAddrPipePending) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for PendingStat {
    #[inline(always)]
    fn default() -> PendingStat {
        PendingStat(0u64 as u32)
    }
}
impl core::fmt::Debug for PendingStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PendingStat")
            .field("read_data_wait_pending", &self.read_data_wait_pending())
            .field("read_addr_pipe_pending", &self.read_addr_pipe_pending())
            .field("write_data_pipe_pending", &self.write_data_pipe_pending())
            .field("write_addr_pipe_pending", &self.write_addr_pipe_pending())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PendingStat {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PendingStat {
            read_data_wait_pending: super::vals::ReadDataWaitPending,
            read_addr_pipe_pending: super::vals::ReadAddrPipePending,
            write_data_pipe_pending: super::vals::WriteDataPipePending,
            write_addr_pipe_pending: super::vals::WriteAddrPipePending,
        }
        let proxy = PendingStat {
            read_data_wait_pending: self.read_data_wait_pending(),
            read_addr_pipe_pending: self.read_addr_pipe_pending(),
            write_data_pipe_pending: self.write_data_pipe_pending(),
            write_addr_pipe_pending: self.write_addr_pipe_pending(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "OCRAM Pipeline And ECC Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PipeEccEn(pub u32);
impl PipeEccEn {
    #[doc = "Read Data Wait Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn read_data_wait_en(&self) -> super::vals::ReadDataWaitEn {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ReadDataWaitEn::from_bits(val as u8)
    }
    #[doc = "Read Data Wait Enable"]
    #[inline(always)]
    pub const fn set_read_data_wait_en(&mut self, val: super::vals::ReadDataWaitEn) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Read Address Pipeline Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn read_addr_pipe_en(&self) -> super::vals::ReadAddrPipeEn {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::ReadAddrPipeEn::from_bits(val as u8)
    }
    #[doc = "Read Address Pipeline Enable"]
    #[inline(always)]
    pub const fn set_read_addr_pipe_en(&mut self, val: super::vals::ReadAddrPipeEn) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Write Data Pipeline Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn write_data_pipe_en(&self) -> super::vals::WriteDataPipeEn {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::WriteDataPipeEn::from_bits(val as u8)
    }
    #[doc = "Write Data Pipeline Enable"]
    #[inline(always)]
    pub const fn set_write_data_pipe_en(&mut self, val: super::vals::WriteDataPipeEn) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Write Address Pipeline Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn write_addr_pipe_en(&self) -> super::vals::WriteAddrPipeEn {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::WriteAddrPipeEn::from_bits(val as u8)
    }
    #[doc = "Write Address Pipeline Enable"]
    #[inline(always)]
    pub const fn set_write_addr_pipe_en(&mut self, val: super::vals::WriteAddrPipeEn) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "ECC Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ecc_en(&self) -> super::vals::EccEn {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::EccEn::from_bits(val as u8)
    }
    #[doc = "ECC Function Enable"]
    #[inline(always)]
    pub const fn set_ecc_en(&mut self, val: super::vals::EccEn) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
}
impl Default for PipeEccEn {
    #[inline(always)]
    fn default() -> PipeEccEn {
        PipeEccEn(0u64 as u32)
    }
}
impl core::fmt::Debug for PipeEccEn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PipeEccEn")
            .field("read_data_wait_en", &self.read_data_wait_en())
            .field("read_addr_pipe_en", &self.read_addr_pipe_en())
            .field("write_data_pipe_en", &self.write_data_pipe_en())
            .field("write_addr_pipe_en", &self.write_addr_pipe_en())
            .field("ecc_en", &self.ecc_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PipeEccEn {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PipeEccEn {
            read_data_wait_en: super::vals::ReadDataWaitEn,
            read_addr_pipe_en: super::vals::ReadAddrPipeEn,
            write_data_pipe_en: super::vals::WriteDataPipeEn,
            write_addr_pipe_en: super::vals::WriteAddrPipeEn,
            ecc_en: super::vals::EccEn,
        }
        let proxy = PipeEccEn {
            read_data_wait_en: self.read_data_wait_en(),
            read_addr_pipe_en: self.read_addr_pipe_en(),
            write_data_pipe_en: self.write_data_pipe_en(),
            write_addr_pipe_en: self.write_addr_pipe_en(),
            ecc_en: self.ecc_en(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Single Error Address And ECC code On OCRAM Bank0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SingleErrAddrEcc0(pub u32);
impl SingleErrAddrEcc0 {
    #[doc = "Single Error ECC code On OCRAM Bank0"]
    #[must_use]
    #[inline(always)]
    pub const fn single_err_ecc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Single Error ECC code On OCRAM Bank0"]
    #[inline(always)]
    pub const fn set_single_err_ecc(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Single Error Address On OCRAM Bank0"]
    #[must_use]
    #[inline(always)]
    pub const fn single_err_addr(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x0007_ffff;
        val as u32
    }
    #[doc = "Single Error Address On OCRAM Bank0"]
    #[inline(always)]
    pub const fn set_single_err_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0007_ffff << 8usize)) | (((val as u32) & 0x0007_ffff) << 8usize);
    }
}
impl Default for SingleErrAddrEcc0 {
    #[inline(always)]
    fn default() -> SingleErrAddrEcc0 {
        SingleErrAddrEcc0(0u64 as u32)
    }
}
impl core::fmt::Debug for SingleErrAddrEcc0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SingleErrAddrEcc0")
            .field("single_err_ecc", &self.single_err_ecc())
            .field("single_err_addr", &self.single_err_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SingleErrAddrEcc0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SingleErrAddrEcc0 {
            single_err_ecc: u8,
            single_err_addr: u32,
        }
        let proxy = SingleErrAddrEcc0 {
            single_err_ecc: self.single_err_ecc(),
            single_err_addr: self.single_err_addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Single Error Address And ECC code On OCRAM Bank1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SingleErrAddrEcc1(pub u32);
impl SingleErrAddrEcc1 {
    #[doc = "Single Error ECC code On OCRAM Bank1"]
    #[must_use]
    #[inline(always)]
    pub const fn single_err_ecc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Single Error ECC code On OCRAM Bank1"]
    #[inline(always)]
    pub const fn set_single_err_ecc(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Single Error Address On OCRAM Bank1"]
    #[must_use]
    #[inline(always)]
    pub const fn single_err_addr(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x0007_ffff;
        val as u32
    }
    #[doc = "Single Error Address On OCRAM Bank1"]
    #[inline(always)]
    pub const fn set_single_err_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0007_ffff << 8usize)) | (((val as u32) & 0x0007_ffff) << 8usize);
    }
}
impl Default for SingleErrAddrEcc1 {
    #[inline(always)]
    fn default() -> SingleErrAddrEcc1 {
        SingleErrAddrEcc1(0u64 as u32)
    }
}
impl core::fmt::Debug for SingleErrAddrEcc1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SingleErrAddrEcc1")
            .field("single_err_ecc", &self.single_err_ecc())
            .field("single_err_addr", &self.single_err_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SingleErrAddrEcc1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SingleErrAddrEcc1 {
            single_err_ecc: u8,
            single_err_addr: u32,
        }
        let proxy = SingleErrAddrEcc1 {
            single_err_ecc: self.single_err_ecc(),
            single_err_addr: self.single_err_addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Single Error Address And ECC code On OCRAM Bank2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SingleErrAddrEcc2(pub u32);
impl SingleErrAddrEcc2 {
    #[doc = "Single Error ECC code On OCRAM Bank2"]
    #[must_use]
    #[inline(always)]
    pub const fn single_err_ecc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Single Error ECC code On OCRAM Bank2"]
    #[inline(always)]
    pub const fn set_single_err_ecc(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Single Error Address On OCRAM Bank2"]
    #[must_use]
    #[inline(always)]
    pub const fn single_err_addr(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x0007_ffff;
        val as u32
    }
    #[doc = "Single Error Address On OCRAM Bank2"]
    #[inline(always)]
    pub const fn set_single_err_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0007_ffff << 8usize)) | (((val as u32) & 0x0007_ffff) << 8usize);
    }
}
impl Default for SingleErrAddrEcc2 {
    #[inline(always)]
    fn default() -> SingleErrAddrEcc2 {
        SingleErrAddrEcc2(0u64 as u32)
    }
}
impl core::fmt::Debug for SingleErrAddrEcc2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SingleErrAddrEcc2")
            .field("single_err_ecc", &self.single_err_ecc())
            .field("single_err_addr", &self.single_err_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SingleErrAddrEcc2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SingleErrAddrEcc2 {
            single_err_ecc: u8,
            single_err_addr: u32,
        }
        let proxy = SingleErrAddrEcc2 {
            single_err_ecc: self.single_err_ecc(),
            single_err_addr: self.single_err_addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Single Error Address And ECC code On OCRAM Bank3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SingleErrAddrEcc3(pub u32);
impl SingleErrAddrEcc3 {
    #[doc = "Single Error ECC code On OCRAM Bank3"]
    #[must_use]
    #[inline(always)]
    pub const fn single_err_ecc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Single Error ECC code On OCRAM Bank3"]
    #[inline(always)]
    pub const fn set_single_err_ecc(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Single Error Address On OCRAM Bank3"]
    #[must_use]
    #[inline(always)]
    pub const fn single_err_addr(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x0007_ffff;
        val as u32
    }
    #[doc = "Single Error Address On OCRAM Bank3"]
    #[inline(always)]
    pub const fn set_single_err_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0007_ffff << 8usize)) | (((val as u32) & 0x0007_ffff) << 8usize);
    }
}
impl Default for SingleErrAddrEcc3 {
    #[inline(always)]
    fn default() -> SingleErrAddrEcc3 {
        SingleErrAddrEcc3(0u64 as u32)
    }
}
impl core::fmt::Debug for SingleErrAddrEcc3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SingleErrAddrEcc3")
            .field("single_err_ecc", &self.single_err_ecc())
            .field("single_err_addr", &self.single_err_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SingleErrAddrEcc3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SingleErrAddrEcc3 {
            single_err_ecc: u8,
            single_err_addr: u32,
        }
        let proxy = SingleErrAddrEcc3 {
            single_err_ecc: self.single_err_ecc(),
            single_err_addr: self.single_err_addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
