#[doc = "CACHE ECC Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CacheEccr(pub u32);
impl CacheEccr {
    #[doc = "Disable CACHE ECC Write Generation"]
    #[must_use]
    #[inline(always)]
    pub const fn wecc_dis(&self) -> super::vals::WeccDis {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::WeccDis::from_bits(val as u8)
    }
    #[doc = "Disable CACHE ECC Write Generation"]
    #[inline(always)]
    pub const fn set_wecc_dis(&mut self, val: super::vals::WeccDis) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Disable Cache ECC Read Check"]
    #[must_use]
    #[inline(always)]
    pub const fn recc_dis(&self) -> super::vals::ReccDis {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::ReccDis::from_bits(val as u8)
    }
    #[doc = "Disable Cache ECC Read Check"]
    #[inline(always)]
    pub const fn set_recc_dis(&mut self, val: super::vals::ReccDis) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for CacheEccr {
    #[inline(always)]
    fn default() -> CacheEccr {
        CacheEccr(0u64 as u32)
    }
}
impl core::fmt::Debug for CacheEccr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CacheEccr")
            .field("wecc_dis", &self.wecc_dis())
            .field("recc_dis", &self.recc_dis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CacheEccr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct CacheEccr {
            wecc_dis: super::vals::WeccDis,
            recc_dis: super::vals::ReccDis,
        }
        let proxy = CacheEccr {
            wecc_dis: self.wecc_dis(),
            recc_dis: self.recc_dis(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Code Cache DATA0 ECC Error Injection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CodeCacheData0EccErrorInjec(pub u32);
impl CodeCacheData0EccErrorInjec {
    #[doc = "Position of First Bit to Inject ECC Error"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_data0_err1bit(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Position of First Bit to Inject ECC Error"]
    #[inline(always)]
    pub const fn set_code_cache_data0_err1bit(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Position of Second Bit to Inject ECC Error"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_data0_err2bit(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Position of Second Bit to Inject ECC Error"]
    #[inline(always)]
    pub const fn set_code_cache_data0_err2bit(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Force One 1-Bit Data Inversion on Code Cache DATA0 Write Access"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_data0_fr11bi(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Force One 1-Bit Data Inversion on Code Cache DATA0 Write Access"]
    #[inline(always)]
    pub const fn set_code_cache_data0_fr11bi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Force One Noncorrectable Data Inversion on Code Cache DATA0 Write Access"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_data0_fr1nci(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Force One Noncorrectable Data Inversion on Code Cache DATA0 Write Access"]
    #[inline(always)]
    pub const fn set_code_cache_data0_fr1nci(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Force Continuous 1-Bit Data Inversions on Code Cache DATA0 Write Access"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_data0_frc1bi(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Force Continuous 1-Bit Data Inversions on Code Cache DATA0 Write Access"]
    #[inline(always)]
    pub const fn set_code_cache_data0_frc1bi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Force Continuous Noncorrectable Data Inversions on Code Cache DATA0 Write Access"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_data0_frcnci(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Force Continuous Noncorrectable Data Inversions on Code Cache DATA0 Write Access"]
    #[inline(always)]
    pub const fn set_code_cache_data0_frcnci(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for CodeCacheData0EccErrorInjec {
    #[inline(always)]
    fn default() -> CodeCacheData0EccErrorInjec {
        CodeCacheData0EccErrorInjec(0u64 as u32)
    }
}
impl core::fmt::Debug for CodeCacheData0EccErrorInjec {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CodeCacheData0EccErrorInjec")
            .field("code_cache_data0_err1bit", &self.code_cache_data0_err1bit())
            .field("code_cache_data0_err2bit", &self.code_cache_data0_err2bit())
            .field("code_cache_data0_fr11bi", &self.code_cache_data0_fr11bi())
            .field("code_cache_data0_fr1nci", &self.code_cache_data0_fr1nci())
            .field("code_cache_data0_frc1bi", &self.code_cache_data0_frc1bi())
            .field("code_cache_data0_frcnci", &self.code_cache_data0_frcnci())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CodeCacheData0EccErrorInjec {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct CodeCacheData0EccErrorInjec {
            code_cache_data0_err1bit: u8,
            code_cache_data0_err2bit: u8,
            code_cache_data0_fr11bi: bool,
            code_cache_data0_fr1nci: bool,
            code_cache_data0_frc1bi: bool,
            code_cache_data0_frcnci: bool,
        }
        let proxy = CodeCacheData0EccErrorInjec {
            code_cache_data0_err1bit: self.code_cache_data0_err1bit(),
            code_cache_data0_err2bit: self.code_cache_data0_err2bit(),
            code_cache_data0_fr11bi: self.code_cache_data0_fr11bi(),
            code_cache_data0_fr1nci: self.code_cache_data0_fr1nci(),
            code_cache_data0_frc1bi: self.code_cache_data0_frc1bi(),
            code_cache_data0_frcnci: self.code_cache_data0_frcnci(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Code Cache DATA1 ECC Error Injection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CodeCacheData1EccErrorInjec(pub u32);
impl CodeCacheData1EccErrorInjec {
    #[doc = "Position of First Bit to Inject ECC Error"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_data1_err1bit(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Position of First Bit to Inject ECC Error"]
    #[inline(always)]
    pub const fn set_code_cache_data1_err1bit(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Position of Second Bit to Inject ECC Error"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_data1_err2bit(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Position of Second Bit to Inject ECC Error"]
    #[inline(always)]
    pub const fn set_code_cache_data1_err2bit(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Force One 1-Bit Data Inversion on Code Cache DATA1 Write Access"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_data1_fr11bi(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Force One 1-Bit Data Inversion on Code Cache DATA1 Write Access"]
    #[inline(always)]
    pub const fn set_code_cache_data1_fr11bi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Force One Noncorrectable Data Inversion on Code Cache DATA1 Write Access"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_data1_fr1nci(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Force One Noncorrectable Data Inversion on Code Cache DATA1 Write Access"]
    #[inline(always)]
    pub const fn set_code_cache_data1_fr1nci(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Force Continuous 1-Bit Data Inversions on Code Cache DATA1 Write Access"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_data1_frc1bi(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Force Continuous 1-Bit Data Inversions on Code Cache DATA1 Write Access"]
    #[inline(always)]
    pub const fn set_code_cache_data1_frc1bi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Force Continuous Noncorrectable Data Inversions on Code Cache DATA1 Write Access"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_data1_frcnci(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Force Continuous Noncorrectable Data Inversions on Code Cache DATA1 Write Access"]
    #[inline(always)]
    pub const fn set_code_cache_data1_frcnci(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for CodeCacheData1EccErrorInjec {
    #[inline(always)]
    fn default() -> CodeCacheData1EccErrorInjec {
        CodeCacheData1EccErrorInjec(0u64 as u32)
    }
}
impl core::fmt::Debug for CodeCacheData1EccErrorInjec {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CodeCacheData1EccErrorInjec")
            .field("code_cache_data1_err1bit", &self.code_cache_data1_err1bit())
            .field("code_cache_data1_err2bit", &self.code_cache_data1_err2bit())
            .field("code_cache_data1_fr11bi", &self.code_cache_data1_fr11bi())
            .field("code_cache_data1_fr1nci", &self.code_cache_data1_fr1nci())
            .field("code_cache_data1_frc1bi", &self.code_cache_data1_frc1bi())
            .field("code_cache_data1_frcnci", &self.code_cache_data1_frcnci())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CodeCacheData1EccErrorInjec {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct CodeCacheData1EccErrorInjec {
            code_cache_data1_err1bit: u8,
            code_cache_data1_err2bit: u8,
            code_cache_data1_fr11bi: bool,
            code_cache_data1_fr1nci: bool,
            code_cache_data1_frc1bi: bool,
            code_cache_data1_frcnci: bool,
        }
        let proxy = CodeCacheData1EccErrorInjec {
            code_cache_data1_err1bit: self.code_cache_data1_err1bit(),
            code_cache_data1_err2bit: self.code_cache_data1_err2bit(),
            code_cache_data1_fr11bi: self.code_cache_data1_fr11bi(),
            code_cache_data1_fr1nci: self.code_cache_data1_fr1nci(),
            code_cache_data1_frc1bi: self.code_cache_data1_frc1bi(),
            code_cache_data1_frcnci: self.code_cache_data1_frcnci(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Code Cache Multibit ECC Error Information"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CodeCacheEccMultiErrorInfo(pub u32);
impl CodeCacheEccMultiErrorInfo {
    #[doc = "Code Cache Multibit ECC Error"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_eccm_tag(&self) -> super::vals::CodeCacheEccmTag {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::CodeCacheEccmTag::from_bits(val as u8)
    }
    #[doc = "Code Cache Multibit ECC Error"]
    #[inline(always)]
    pub const fn set_code_cache_eccm_tag(&mut self, val: super::vals::CodeCacheEccmTag) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Code Cache Multibit ECC Error on Code Cache Command"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_eccm_cmd(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Code Cache Multibit ECC Error on Code Cache Command"]
    #[inline(always)]
    pub const fn set_code_cache_eccm_cmd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Code Cache Multibit ECC Error Master Number"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_eccm_efmst(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Code Cache Multibit ECC Error Master Number"]
    #[inline(always)]
    pub const fn set_code_cache_eccm_efmst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Code Cache Multibit ECC Error Protection"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_eccm_efprt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Code Cache Multibit ECC Error Protection"]
    #[inline(always)]
    pub const fn set_code_cache_eccm_efprt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Code Cache Multibit ECC Error Corresponding Syndrome"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_eccm_efsyn(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "Code Cache Multibit ECC Error Corresponding Syndrome"]
    #[inline(always)]
    pub const fn set_code_cache_eccm_efsyn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
}
impl Default for CodeCacheEccMultiErrorInfo {
    #[inline(always)]
    fn default() -> CodeCacheEccMultiErrorInfo {
        CodeCacheEccMultiErrorInfo(0u64 as u32)
    }
}
impl core::fmt::Debug for CodeCacheEccMultiErrorInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CodeCacheEccMultiErrorInfo")
            .field("code_cache_eccm_tag", &self.code_cache_eccm_tag())
            .field("code_cache_eccm_cmd", &self.code_cache_eccm_cmd())
            .field("code_cache_eccm_efmst", &self.code_cache_eccm_efmst())
            .field("code_cache_eccm_efprt", &self.code_cache_eccm_efprt())
            .field("code_cache_eccm_efsyn", &self.code_cache_eccm_efsyn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CodeCacheEccMultiErrorInfo {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct CodeCacheEccMultiErrorInfo {
            code_cache_eccm_tag: super::vals::CodeCacheEccmTag,
            code_cache_eccm_cmd: bool,
            code_cache_eccm_efmst: u8,
            code_cache_eccm_efprt: u8,
            code_cache_eccm_efsyn: u8,
        }
        let proxy = CodeCacheEccMultiErrorInfo {
            code_cache_eccm_tag: self.code_cache_eccm_tag(),
            code_cache_eccm_cmd: self.code_cache_eccm_cmd(),
            code_cache_eccm_efmst: self.code_cache_eccm_efmst(),
            code_cache_eccm_efprt: self.code_cache_eccm_efprt(),
            code_cache_eccm_efsyn: self.code_cache_eccm_efsyn(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Code Cache Single-Bit ECC Error Information"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CodeCacheEccSingleErrorInfo(pub u32);
impl CodeCacheEccSingleErrorInfo {
    #[doc = "Code Cache Single-Bit ECC Error"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_eccs_tag(&self) -> super::vals::CodeCacheEccsTag {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::CodeCacheEccsTag::from_bits(val as u8)
    }
    #[doc = "Code Cache Single-Bit ECC Error"]
    #[inline(always)]
    pub const fn set_code_cache_eccs_tag(&mut self, val: super::vals::CodeCacheEccsTag) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Code Cache Single-Bit ECC Error on Cache Command"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_eccs_cmd(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Code Cache Single-Bit ECC Error on Cache Command"]
    #[inline(always)]
    pub const fn set_code_cache_eccs_cmd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Code Cache Single-Bit ECC Error Master Number"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_eccs_efmst(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Code Cache Single-Bit ECC Error Master Number"]
    #[inline(always)]
    pub const fn set_code_cache_eccs_efmst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Code Cache Single-Bit ECC Error Protection"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_eccs_efprt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Code Cache Single-Bit ECC Error Protection"]
    #[inline(always)]
    pub const fn set_code_cache_eccs_efprt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Code Cache Single-Bit ECC Error Corresponding Syndrome"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_eccs_efsyn(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "Code Cache Single-Bit ECC Error Corresponding Syndrome"]
    #[inline(always)]
    pub const fn set_code_cache_eccs_efsyn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
}
impl Default for CodeCacheEccSingleErrorInfo {
    #[inline(always)]
    fn default() -> CodeCacheEccSingleErrorInfo {
        CodeCacheEccSingleErrorInfo(0u64 as u32)
    }
}
impl core::fmt::Debug for CodeCacheEccSingleErrorInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CodeCacheEccSingleErrorInfo")
            .field("code_cache_eccs_tag", &self.code_cache_eccs_tag())
            .field("code_cache_eccs_cmd", &self.code_cache_eccs_cmd())
            .field("code_cache_eccs_efmst", &self.code_cache_eccs_efmst())
            .field("code_cache_eccs_efprt", &self.code_cache_eccs_efprt())
            .field("code_cache_eccs_efsyn", &self.code_cache_eccs_efsyn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CodeCacheEccSingleErrorInfo {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct CodeCacheEccSingleErrorInfo {
            code_cache_eccs_tag: super::vals::CodeCacheEccsTag,
            code_cache_eccs_cmd: bool,
            code_cache_eccs_efmst: u8,
            code_cache_eccs_efprt: u8,
            code_cache_eccs_efsyn: u8,
        }
        let proxy = CodeCacheEccSingleErrorInfo {
            code_cache_eccs_tag: self.code_cache_eccs_tag(),
            code_cache_eccs_cmd: self.code_cache_eccs_cmd(),
            code_cache_eccs_efmst: self.code_cache_eccs_efmst(),
            code_cache_eccs_efprt: self.code_cache_eccs_efprt(),
            code_cache_eccs_efsyn: self.code_cache_eccs_efsyn(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Code Cache TAG0 ECC Error Injection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CodeCacheTag0EccErrorInjec(pub u32);
impl CodeCacheTag0EccErrorInjec {
    #[doc = "Position of First Bit to Inject ECC Error"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_tag0_err1bit(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Position of First Bit to Inject ECC Error"]
    #[inline(always)]
    pub const fn set_code_cache_tag0_err1bit(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Position of Second Bit to Inject ECC Error"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_tag0_err2bit(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Position of Second Bit to Inject ECC Error"]
    #[inline(always)]
    pub const fn set_code_cache_tag0_err2bit(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Force One 1-Bit Data Inversion on Code Cache TAG0 Write Access"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_tag0_fr11bi(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Force One 1-Bit Data Inversion on Code Cache TAG0 Write Access"]
    #[inline(always)]
    pub const fn set_code_cache_tag0_fr11bi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Force One Noncorrectable Data Inversion on Code Cache TAG0 Write Access"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_tag0_fr1nci(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Force One Noncorrectable Data Inversion on Code Cache TAG0 Write Access"]
    #[inline(always)]
    pub const fn set_code_cache_tag0_fr1nci(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Force Continuous 1-Bit Data Inversions on Code Cache TAG0 Write Access"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_tag0_frc1bi(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Force Continuous 1-Bit Data Inversions on Code Cache TAG0 Write Access"]
    #[inline(always)]
    pub const fn set_code_cache_tag0_frc1bi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Force Continuous Noncorrectable Data Inversions on Code Cache TAG0 Write Access"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_tag0_frcnci(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Force Continuous Noncorrectable Data Inversions on Code Cache TAG0 Write Access"]
    #[inline(always)]
    pub const fn set_code_cache_tag0_frcnci(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for CodeCacheTag0EccErrorInjec {
    #[inline(always)]
    fn default() -> CodeCacheTag0EccErrorInjec {
        CodeCacheTag0EccErrorInjec(0u64 as u32)
    }
}
impl core::fmt::Debug for CodeCacheTag0EccErrorInjec {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CodeCacheTag0EccErrorInjec")
            .field("code_cache_tag0_err1bit", &self.code_cache_tag0_err1bit())
            .field("code_cache_tag0_err2bit", &self.code_cache_tag0_err2bit())
            .field("code_cache_tag0_fr11bi", &self.code_cache_tag0_fr11bi())
            .field("code_cache_tag0_fr1nci", &self.code_cache_tag0_fr1nci())
            .field("code_cache_tag0_frc1bi", &self.code_cache_tag0_frc1bi())
            .field("code_cache_tag0_frcnci", &self.code_cache_tag0_frcnci())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CodeCacheTag0EccErrorInjec {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct CodeCacheTag0EccErrorInjec {
            code_cache_tag0_err1bit: u8,
            code_cache_tag0_err2bit: u8,
            code_cache_tag0_fr11bi: bool,
            code_cache_tag0_fr1nci: bool,
            code_cache_tag0_frc1bi: bool,
            code_cache_tag0_frcnci: bool,
        }
        let proxy = CodeCacheTag0EccErrorInjec {
            code_cache_tag0_err1bit: self.code_cache_tag0_err1bit(),
            code_cache_tag0_err2bit: self.code_cache_tag0_err2bit(),
            code_cache_tag0_fr11bi: self.code_cache_tag0_fr11bi(),
            code_cache_tag0_fr1nci: self.code_cache_tag0_fr1nci(),
            code_cache_tag0_frc1bi: self.code_cache_tag0_frc1bi(),
            code_cache_tag0_frcnci: self.code_cache_tag0_frcnci(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Code Cache TAG1 ECC Error Injection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CodeCacheTag1EccErrorInjec(pub u32);
impl CodeCacheTag1EccErrorInjec {
    #[doc = "Position of First Bit to Inject ECC Error"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_tag1_err1bit(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Position of First Bit to Inject ECC Error"]
    #[inline(always)]
    pub const fn set_code_cache_tag1_err1bit(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Position of Second Bit to Inject ECC Error"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_tag1_err2bit(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Position of Second Bit to Inject ECC Error"]
    #[inline(always)]
    pub const fn set_code_cache_tag1_err2bit(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Force One 1-Bit Data Inversion on Code Cache TAG1 Write Access"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_tag1_fr11bi(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Force One 1-Bit Data Inversion on Code Cache TAG1 Write Access"]
    #[inline(always)]
    pub const fn set_code_cache_tag1_fr11bi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Force One Noncorrectable Data Inversion on Code Cache TAG1 Write Access"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_tag1_fr1nci(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Force One Noncorrectable Data Inversion on Code Cache TAG1 Write Access"]
    #[inline(always)]
    pub const fn set_code_cache_tag1_fr1nci(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Force Continuous 1-Bit Data Inversions on Code Cache TAG1 Write Access"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_tag1_frc1bi(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Force Continuous 1-Bit Data Inversions on Code Cache TAG1 Write Access"]
    #[inline(always)]
    pub const fn set_code_cache_tag1_frc1bi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Force Continuous Noncorrectable Data Inversions on Code Cache TAG1 Write Access"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_tag1_frcnci(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Force Continuous Noncorrectable Data Inversions on Code Cache TAG1 Write Access"]
    #[inline(always)]
    pub const fn set_code_cache_tag1_frcnci(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for CodeCacheTag1EccErrorInjec {
    #[inline(always)]
    fn default() -> CodeCacheTag1EccErrorInjec {
        CodeCacheTag1EccErrorInjec(0u64 as u32)
    }
}
impl core::fmt::Debug for CodeCacheTag1EccErrorInjec {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CodeCacheTag1EccErrorInjec")
            .field("code_cache_tag1_err1bit", &self.code_cache_tag1_err1bit())
            .field("code_cache_tag1_err2bit", &self.code_cache_tag1_err2bit())
            .field("code_cache_tag1_fr11bi", &self.code_cache_tag1_fr11bi())
            .field("code_cache_tag1_fr1nci", &self.code_cache_tag1_fr1nci())
            .field("code_cache_tag1_frc1bi", &self.code_cache_tag1_frc1bi())
            .field("code_cache_tag1_frcnci", &self.code_cache_tag1_frcnci())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CodeCacheTag1EccErrorInjec {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct CodeCacheTag1EccErrorInjec {
            code_cache_tag1_err1bit: u8,
            code_cache_tag1_err2bit: u8,
            code_cache_tag1_fr11bi: bool,
            code_cache_tag1_fr1nci: bool,
            code_cache_tag1_frc1bi: bool,
            code_cache_tag1_frcnci: bool,
        }
        let proxy = CodeCacheTag1EccErrorInjec {
            code_cache_tag1_err1bit: self.code_cache_tag1_err1bit(),
            code_cache_tag1_err2bit: self.code_cache_tag1_err2bit(),
            code_cache_tag1_fr11bi: self.code_cache_tag1_fr11bi(),
            code_cache_tag1_fr1nci: self.code_cache_tag1_fr1nci(),
            code_cache_tag1_frc1bi: self.code_cache_tag1_frc1bi(),
            code_cache_tag1_frcnci: self.code_cache_tag1_frcnci(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntSigEn(pub u32);
impl IntSigEn {
    #[doc = "Code Cache Access Multibit ECC Error Interrupt Signal Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_errm_int_sig_en(&self) -> super::vals::CodeCacheErrmIntSigEn {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::CodeCacheErrmIntSigEn::from_bits(val as u8)
    }
    #[doc = "Code Cache Access Multibit ECC Error Interrupt Signal Enable"]
    #[inline(always)]
    pub const fn set_code_cache_errm_int_sig_en(
        &mut self,
        val: super::vals::CodeCacheErrmIntSigEn,
    ) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Code Cache Access Single-Bit ECC Error Interrupt Signal Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_errs_int_sig_en(&self) -> super::vals::CodeCacheErrsIntSigEn {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::CodeCacheErrsIntSigEn::from_bits(val as u8)
    }
    #[doc = "Code Cache Access Single-Bit ECC Error Interrupt Signal Enable"]
    #[inline(always)]
    pub const fn set_code_cache_errs_int_sig_en(
        &mut self,
        val: super::vals::CodeCacheErrsIntSigEn,
    ) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Code Cache Access Multiple Multibit ECC Error Interrupt Signal Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_errm_over_int_sig_en(&self) -> super::vals::CodeCacheErrmOverIntSigEn {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::CodeCacheErrmOverIntSigEn::from_bits(val as u8)
    }
    #[doc = "Code Cache Access Multiple Multibit ECC Error Interrupt Signal Enable"]
    #[inline(always)]
    pub const fn set_code_cache_errm_over_int_sig_en(
        &mut self,
        val: super::vals::CodeCacheErrmOverIntSigEn,
    ) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Code Cache Access Multiple Single-Bit ECC Error Interrupt Signal Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_errs_over_int_sig_en(&self) -> super::vals::CodeCacheErrsOverIntSigEn {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::CodeCacheErrsOverIntSigEn::from_bits(val as u8)
    }
    #[doc = "Code Cache Access Multiple Single-Bit ECC Error Interrupt Signal Enable"]
    #[inline(always)]
    pub const fn set_code_cache_errs_over_int_sig_en(
        &mut self,
        val: super::vals::CodeCacheErrsOverIntSigEn,
    ) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "System Cache Access Multibit ECC Error Interrupt Signal Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn system_cache_errm_int_sig_en(&self) -> super::vals::SystemCacheErrmIntSigEn {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::SystemCacheErrmIntSigEn::from_bits(val as u8)
    }
    #[doc = "System Cache Access Multibit ECC Error Interrupt Signal Enable"]
    #[inline(always)]
    pub const fn set_system_cache_errm_int_sig_en(
        &mut self,
        val: super::vals::SystemCacheErrmIntSigEn,
    ) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "System Cache Access Single-Bit ECC Error Interrupt Signal Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn system_cache_errs_int_sig_en(&self) -> super::vals::SystemCacheErrsIntSigEn {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::SystemCacheErrsIntSigEn::from_bits(val as u8)
    }
    #[doc = "System Cache Access Single-Bit ECC Error Interrupt Signal Enable"]
    #[inline(always)]
    pub const fn set_system_cache_errs_int_sig_en(
        &mut self,
        val: super::vals::SystemCacheErrsIntSigEn,
    ) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "System Cache Access Multiple Multibit ECC Error Interrupt Signal Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn system_cache_errm_over_int_sig_en(
        &self,
    ) -> super::vals::SystemCacheErrmOverIntSigEn {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::SystemCacheErrmOverIntSigEn::from_bits(val as u8)
    }
    #[doc = "System Cache Access Multiple Multibit ECC Error Interrupt Signal Enable"]
    #[inline(always)]
    pub const fn set_system_cache_errm_over_int_sig_en(
        &mut self,
        val: super::vals::SystemCacheErrmOverIntSigEn,
    ) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "System Cache Access Multiple Single-Bit ECC Error Interrupt Signal Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn system_cache_errs_over_int_sig_en(
        &self,
    ) -> super::vals::SystemCacheErrsOverIntSigEn {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::SystemCacheErrsOverIntSigEn::from_bits(val as u8)
    }
    #[doc = "System Cache Access Multiple Single-Bit ECC Error Interrupt Signal Enable"]
    #[inline(always)]
    pub const fn set_system_cache_errs_over_int_sig_en(
        &mut self,
        val: super::vals::SystemCacheErrsOverIntSigEn,
    ) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for IntSigEn {
    #[inline(always)]
    fn default() -> IntSigEn {
        IntSigEn(0u64 as u32)
    }
}
impl core::fmt::Debug for IntSigEn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntSigEn")
            .field(
                "code_cache_errm_int_sig_en",
                &self.code_cache_errm_int_sig_en(),
            )
            .field(
                "code_cache_errs_int_sig_en",
                &self.code_cache_errs_int_sig_en(),
            )
            .field(
                "code_cache_errm_over_int_sig_en",
                &self.code_cache_errm_over_int_sig_en(),
            )
            .field(
                "code_cache_errs_over_int_sig_en",
                &self.code_cache_errs_over_int_sig_en(),
            )
            .field(
                "system_cache_errm_int_sig_en",
                &self.system_cache_errm_int_sig_en(),
            )
            .field(
                "system_cache_errs_int_sig_en",
                &self.system_cache_errs_int_sig_en(),
            )
            .field(
                "system_cache_errm_over_int_sig_en",
                &self.system_cache_errm_over_int_sig_en(),
            )
            .field(
                "system_cache_errs_over_int_sig_en",
                &self.system_cache_errs_over_int_sig_en(),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntSigEn {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct IntSigEn {
            code_cache_errm_int_sig_en: super::vals::CodeCacheErrmIntSigEn,
            code_cache_errs_int_sig_en: super::vals::CodeCacheErrsIntSigEn,
            code_cache_errm_over_int_sig_en: super::vals::CodeCacheErrmOverIntSigEn,
            code_cache_errs_over_int_sig_en: super::vals::CodeCacheErrsOverIntSigEn,
            system_cache_errm_int_sig_en: super::vals::SystemCacheErrmIntSigEn,
            system_cache_errs_int_sig_en: super::vals::SystemCacheErrsIntSigEn,
            system_cache_errm_over_int_sig_en: super::vals::SystemCacheErrmOverIntSigEn,
            system_cache_errs_over_int_sig_en: super::vals::SystemCacheErrsOverIntSigEn,
        }
        let proxy = IntSigEn {
            code_cache_errm_int_sig_en: self.code_cache_errm_int_sig_en(),
            code_cache_errs_int_sig_en: self.code_cache_errs_int_sig_en(),
            code_cache_errm_over_int_sig_en: self.code_cache_errm_over_int_sig_en(),
            code_cache_errs_over_int_sig_en: self.code_cache_errs_over_int_sig_en(),
            system_cache_errm_int_sig_en: self.system_cache_errm_int_sig_en(),
            system_cache_errs_int_sig_en: self.system_cache_errs_int_sig_en(),
            system_cache_errm_over_int_sig_en: self.system_cache_errm_over_int_sig_en(),
            system_cache_errs_over_int_sig_en: self.system_cache_errs_over_int_sig_en(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Status Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntStatEn(pub u32);
impl IntStatEn {
    #[doc = "Code Cache Access Multibit ECC Error Interrupt Status Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_errm_int_en(&self) -> super::vals::CodeCacheErrmIntEn {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::CodeCacheErrmIntEn::from_bits(val as u8)
    }
    #[doc = "Code Cache Access Multibit ECC Error Interrupt Status Enable"]
    #[inline(always)]
    pub const fn set_code_cache_errm_int_en(&mut self, val: super::vals::CodeCacheErrmIntEn) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Code Cache Access Single-Bit ECC Error Interrupt Status Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_errs_int_en(&self) -> super::vals::CodeCacheErrsIntEn {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::CodeCacheErrsIntEn::from_bits(val as u8)
    }
    #[doc = "Code Cache Access Single-Bit ECC Error Interrupt Status Enable"]
    #[inline(always)]
    pub const fn set_code_cache_errs_int_en(&mut self, val: super::vals::CodeCacheErrsIntEn) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Code Cache Access Multiple Multibit ECC Error Interrupt Status Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_errm_over_int_en(&self) -> super::vals::CodeCacheErrmOverIntEn {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::CodeCacheErrmOverIntEn::from_bits(val as u8)
    }
    #[doc = "Code Cache Access Multiple Multibit ECC Error Interrupt Status Enable"]
    #[inline(always)]
    pub const fn set_code_cache_errm_over_int_en(
        &mut self,
        val: super::vals::CodeCacheErrmOverIntEn,
    ) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Code Cache Access Multiple Single-Bit ECC Error Interrupt Status Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_errs_over_int_en(&self) -> super::vals::CodeCacheErrsOverIntEn {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::CodeCacheErrsOverIntEn::from_bits(val as u8)
    }
    #[doc = "Code Cache Access Multiple Single-Bit ECC Error Interrupt Status Enable"]
    #[inline(always)]
    pub const fn set_code_cache_errs_over_int_en(
        &mut self,
        val: super::vals::CodeCacheErrsOverIntEn,
    ) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "System Cache Access Multibit ECC Error Interrupt Status Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn system_cache_ecc_errm_int_en(&self) -> super::vals::SystemCacheEccErrmIntEn {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::SystemCacheEccErrmIntEn::from_bits(val as u8)
    }
    #[doc = "System Cache Access Multibit ECC Error Interrupt Status Enable"]
    #[inline(always)]
    pub const fn set_system_cache_ecc_errm_int_en(
        &mut self,
        val: super::vals::SystemCacheEccErrmIntEn,
    ) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "System Cache Access Single-Bit ECC Error Interrupt Status Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn system_cache_ecc_errs_int_en(&self) -> super::vals::SystemCacheEccErrsIntEn {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::SystemCacheEccErrsIntEn::from_bits(val as u8)
    }
    #[doc = "System Cache Access Single-Bit ECC Error Interrupt Status Enable"]
    #[inline(always)]
    pub const fn set_system_cache_ecc_errs_int_en(
        &mut self,
        val: super::vals::SystemCacheEccErrsIntEn,
    ) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "System Cache Access Multiple Multibit ECC Error Interrupt Status Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn system_cache_ecc_errm_over_int_en(
        &self,
    ) -> super::vals::SystemCacheEccErrmOverIntEn {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::SystemCacheEccErrmOverIntEn::from_bits(val as u8)
    }
    #[doc = "System Cache Access Multiple Multibit ECC Error Interrupt Status Enable"]
    #[inline(always)]
    pub const fn set_system_cache_ecc_errm_over_int_en(
        &mut self,
        val: super::vals::SystemCacheEccErrmOverIntEn,
    ) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "System Cache Access Multiple Single-Bit ECC Error Interrupt Status Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn system_cache_ecc_errs_over_int_en(
        &self,
    ) -> super::vals::SystemCacheEccErrsOverIntEn {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::SystemCacheEccErrsOverIntEn::from_bits(val as u8)
    }
    #[doc = "System Cache Access Multiple Single-Bit ECC Error Interrupt Status Enable"]
    #[inline(always)]
    pub const fn set_system_cache_ecc_errs_over_int_en(
        &mut self,
        val: super::vals::SystemCacheEccErrsOverIntEn,
    ) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for IntStatEn {
    #[inline(always)]
    fn default() -> IntStatEn {
        IntStatEn(0u64 as u32)
    }
}
impl core::fmt::Debug for IntStatEn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntStatEn")
            .field("code_cache_errm_int_en", &self.code_cache_errm_int_en())
            .field("code_cache_errs_int_en", &self.code_cache_errs_int_en())
            .field(
                "code_cache_errm_over_int_en",
                &self.code_cache_errm_over_int_en(),
            )
            .field(
                "code_cache_errs_over_int_en",
                &self.code_cache_errs_over_int_en(),
            )
            .field(
                "system_cache_ecc_errm_int_en",
                &self.system_cache_ecc_errm_int_en(),
            )
            .field(
                "system_cache_ecc_errs_int_en",
                &self.system_cache_ecc_errs_int_en(),
            )
            .field(
                "system_cache_ecc_errm_over_int_en",
                &self.system_cache_ecc_errm_over_int_en(),
            )
            .field(
                "system_cache_ecc_errs_over_int_en",
                &self.system_cache_ecc_errs_over_int_en(),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntStatEn {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct IntStatEn {
            code_cache_errm_int_en: super::vals::CodeCacheErrmIntEn,
            code_cache_errs_int_en: super::vals::CodeCacheErrsIntEn,
            code_cache_errm_over_int_en: super::vals::CodeCacheErrmOverIntEn,
            code_cache_errs_over_int_en: super::vals::CodeCacheErrsOverIntEn,
            system_cache_ecc_errm_int_en: super::vals::SystemCacheEccErrmIntEn,
            system_cache_ecc_errs_int_en: super::vals::SystemCacheEccErrsIntEn,
            system_cache_ecc_errm_over_int_en: super::vals::SystemCacheEccErrmOverIntEn,
            system_cache_ecc_errs_over_int_en: super::vals::SystemCacheEccErrsOverIntEn,
        }
        let proxy = IntStatEn {
            code_cache_errm_int_en: self.code_cache_errm_int_en(),
            code_cache_errs_int_en: self.code_cache_errs_int_en(),
            code_cache_errm_over_int_en: self.code_cache_errm_over_int_en(),
            code_cache_errs_over_int_en: self.code_cache_errs_over_int_en(),
            system_cache_ecc_errm_int_en: self.system_cache_ecc_errm_int_en(),
            system_cache_ecc_errs_int_en: self.system_cache_ecc_errs_int_en(),
            system_cache_ecc_errm_over_int_en: self.system_cache_ecc_errm_over_int_en(),
            system_cache_ecc_errs_over_int_en: self.system_cache_ecc_errs_over_int_en(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntStatus(pub u32);
impl IntStatus {
    #[doc = "Code Cache Access Multibit ECC Error Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_ecc_errm_int(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Code Cache Access Multibit ECC Error Interrupt Status"]
    #[inline(always)]
    pub const fn set_code_cache_ecc_errm_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Code Cache Access Single-Bit ECC Error Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_ecc_errs_int(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Code Cache Access Single-Bit ECC Error Interrupt Status"]
    #[inline(always)]
    pub const fn set_code_cache_ecc_errs_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Code Cache Access Multiple Multibit ECC Error Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_ecc_errm_over_int(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Code Cache Access Multiple Multibit ECC Error Interrupt Status"]
    #[inline(always)]
    pub const fn set_code_cache_ecc_errm_over_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Code Cache Access Multiple Single-Bit ECC Error Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn code_cache_ecc_errs_over_int(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Code Cache Access Multiple Single-Bit ECC Error Interrupt Status"]
    #[inline(always)]
    pub const fn set_code_cache_ecc_errs_over_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "System Cache Access Multibit ECC Error Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn system_cache_ecc_errm_int(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "System Cache Access Multibit ECC Error Interrupt Status"]
    #[inline(always)]
    pub const fn set_system_cache_ecc_errm_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "System Cache Access Single-Bit ECC Error Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn system_cache_ecc_errs_int(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "System Cache Access Single-Bit ECC Error Interrupt Status"]
    #[inline(always)]
    pub const fn set_system_cache_ecc_errs_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "System Cache Access Multiple Multibit ECC Error Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn system_cache_ecc_errm_over_int(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "System Cache Access Multiple Multibit ECC Error Interrupt Status"]
    #[inline(always)]
    pub const fn set_system_cache_ecc_errm_over_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "System Cache Access Multiple Single-Bit ECC Error Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn system_cache_ecc_errs_over_int(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "System Cache Access Multiple Single-Bit ECC Error Interrupt Status"]
    #[inline(always)]
    pub const fn set_system_cache_ecc_errs_over_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for IntStatus {
    #[inline(always)]
    fn default() -> IntStatus {
        IntStatus(0u64 as u32)
    }
}
impl core::fmt::Debug for IntStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntStatus")
            .field("code_cache_ecc_errm_int", &self.code_cache_ecc_errm_int())
            .field("code_cache_ecc_errs_int", &self.code_cache_ecc_errs_int())
            .field(
                "code_cache_ecc_errm_over_int",
                &self.code_cache_ecc_errm_over_int(),
            )
            .field(
                "code_cache_ecc_errs_over_int",
                &self.code_cache_ecc_errs_over_int(),
            )
            .field(
                "system_cache_ecc_errm_int",
                &self.system_cache_ecc_errm_int(),
            )
            .field(
                "system_cache_ecc_errs_int",
                &self.system_cache_ecc_errs_int(),
            )
            .field(
                "system_cache_ecc_errm_over_int",
                &self.system_cache_ecc_errm_over_int(),
            )
            .field(
                "system_cache_ecc_errs_over_int",
                &self.system_cache_ecc_errs_over_int(),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntStatus {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct IntStatus {
            code_cache_ecc_errm_int: bool,
            code_cache_ecc_errs_int: bool,
            code_cache_ecc_errm_over_int: bool,
            code_cache_ecc_errs_over_int: bool,
            system_cache_ecc_errm_int: bool,
            system_cache_ecc_errs_int: bool,
            system_cache_ecc_errm_over_int: bool,
            system_cache_ecc_errs_over_int: bool,
        }
        let proxy = IntStatus {
            code_cache_ecc_errm_int: self.code_cache_ecc_errm_int(),
            code_cache_ecc_errs_int: self.code_cache_ecc_errs_int(),
            code_cache_ecc_errm_over_int: self.code_cache_ecc_errm_over_int(),
            code_cache_ecc_errs_over_int: self.code_cache_ecc_errs_over_int(),
            system_cache_ecc_errm_int: self.system_cache_ecc_errm_int(),
            system_cache_ecc_errs_int: self.system_cache_ecc_errs_int(),
            system_cache_ecc_errm_over_int: self.system_cache_ecc_errm_over_int(),
            system_cache_ecc_errs_over_int: self.system_cache_ecc_errs_over_int(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "System Cache DATA1 ECC Error Injection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct StstemCacheData1EccErrorInjec(pub u32);
impl StstemCacheData1EccErrorInjec {
    #[doc = "Position of First Bit to Inject ECC Error"]
    #[must_use]
    #[inline(always)]
    pub const fn system_cache_data1_err1bit(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Position of First Bit to Inject ECC Error"]
    #[inline(always)]
    pub const fn set_system_cache_data1_err1bit(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Position of Second Bit to Inject ECC Error"]
    #[must_use]
    #[inline(always)]
    pub const fn system_cache_data1_err2bit(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Position of Second Bit to Inject ECC Error"]
    #[inline(always)]
    pub const fn set_system_cache_data1_err2bit(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Force One 1-Bit Data Inversion on System Cache DATA1 Write Access"]
    #[must_use]
    #[inline(always)]
    pub const fn system_cache_data1_fr11bi(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Force One 1-Bit Data Inversion on System Cache DATA1 Write Access"]
    #[inline(always)]
    pub const fn set_system_cache_data1_fr11bi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Force One Noncorrectable Data Inversion on System Cache DATA1 Write Access"]
    #[must_use]
    #[inline(always)]
    pub const fn system_cache_data1_fr1nci(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Force One Noncorrectable Data Inversion on System Cache DATA1 Write Access"]
    #[inline(always)]
    pub const fn set_system_cache_data1_fr1nci(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Force Continuous 1-Bit Data Inversions on System Cache DATA1 Write Access"]
    #[must_use]
    #[inline(always)]
    pub const fn system_cache_data1_frc1bi(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Force Continuous 1-Bit Data Inversions on System Cache DATA1 Write Access"]
    #[inline(always)]
    pub const fn set_system_cache_data1_frc1bi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Force Continuous Noncorrectable Data Inversions on System Cache DATA1 Write Access"]
    #[must_use]
    #[inline(always)]
    pub const fn system_cache_data1_frcnci(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Force Continuous Noncorrectable Data Inversions on System Cache DATA1 Write Access"]
    #[inline(always)]
    pub const fn set_system_cache_data1_frcnci(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for StstemCacheData1EccErrorInjec {
    #[inline(always)]
    fn default() -> StstemCacheData1EccErrorInjec {
        StstemCacheData1EccErrorInjec(0u64 as u32)
    }
}
impl core::fmt::Debug for StstemCacheData1EccErrorInjec {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("StstemCacheData1EccErrorInjec")
            .field(
                "system_cache_data1_err1bit",
                &self.system_cache_data1_err1bit(),
            )
            .field(
                "system_cache_data1_err2bit",
                &self.system_cache_data1_err2bit(),
            )
            .field(
                "system_cache_data1_fr11bi",
                &self.system_cache_data1_fr11bi(),
            )
            .field(
                "system_cache_data1_fr1nci",
                &self.system_cache_data1_fr1nci(),
            )
            .field(
                "system_cache_data1_frc1bi",
                &self.system_cache_data1_frc1bi(),
            )
            .field(
                "system_cache_data1_frcnci",
                &self.system_cache_data1_frcnci(),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for StstemCacheData1EccErrorInjec {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct StstemCacheData1EccErrorInjec {
            system_cache_data1_err1bit: u8,
            system_cache_data1_err2bit: u8,
            system_cache_data1_fr11bi: bool,
            system_cache_data1_fr1nci: bool,
            system_cache_data1_frc1bi: bool,
            system_cache_data1_frcnci: bool,
        }
        let proxy = StstemCacheData1EccErrorInjec {
            system_cache_data1_err1bit: self.system_cache_data1_err1bit(),
            system_cache_data1_err2bit: self.system_cache_data1_err2bit(),
            system_cache_data1_fr11bi: self.system_cache_data1_fr11bi(),
            system_cache_data1_fr1nci: self.system_cache_data1_fr1nci(),
            system_cache_data1_frc1bi: self.system_cache_data1_frc1bi(),
            system_cache_data1_frcnci: self.system_cache_data1_frcnci(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "System Cache DATA0 ECC Error Injection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SystemCacheData0EccErrorInjec(pub u32);
impl SystemCacheData0EccErrorInjec {
    #[doc = "Position of First Bit to Inject ECC Error"]
    #[must_use]
    #[inline(always)]
    pub const fn system_cache_data0_err1bit(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Position of First Bit to Inject ECC Error"]
    #[inline(always)]
    pub const fn set_system_cache_data0_err1bit(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Position of Second Bit to Inject ECC Error"]
    #[must_use]
    #[inline(always)]
    pub const fn system_cache_data0_err2bit(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Position of Second Bit to Inject ECC Error"]
    #[inline(always)]
    pub const fn set_system_cache_data0_err2bit(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Force One 1-Bit Data Inversion on System Cache DATA0 Write Access"]
    #[must_use]
    #[inline(always)]
    pub const fn system_cache_data0_fr11bi(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Force One 1-Bit Data Inversion on System Cache DATA0 Write Access"]
    #[inline(always)]
    pub const fn set_system_cache_data0_fr11bi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Force One Noncorrectable Data Inversion on System Cache DATA0 Write Access"]
    #[must_use]
    #[inline(always)]
    pub const fn system_cache_data0_fr1nci(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Force One Noncorrectable Data Inversion on System Cache DATA0 Write Access"]
    #[inline(always)]
    pub const fn set_system_cache_data0_fr1nci(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Force Continuous 1-Bit Data Inversions on System Cache DATA0 Write Access"]
    #[must_use]
    #[inline(always)]
    pub const fn system_cache_data0_frc1bi(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Force Continuous 1-Bit Data Inversions on System Cache DATA0 Write Access"]
    #[inline(always)]
    pub const fn set_system_cache_data0_frc1bi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Force Continuous Noncorrectable Data Inversions on System Cache DATA0 Write Access"]
    #[must_use]
    #[inline(always)]
    pub const fn system_cache_data0_frcnci(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Force Continuous Noncorrectable Data Inversions on System Cache DATA0 Write Access"]
    #[inline(always)]
    pub const fn set_system_cache_data0_frcnci(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for SystemCacheData0EccErrorInjec {
    #[inline(always)]
    fn default() -> SystemCacheData0EccErrorInjec {
        SystemCacheData0EccErrorInjec(0u64 as u32)
    }
}
impl core::fmt::Debug for SystemCacheData0EccErrorInjec {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SystemCacheData0EccErrorInjec")
            .field(
                "system_cache_data0_err1bit",
                &self.system_cache_data0_err1bit(),
            )
            .field(
                "system_cache_data0_err2bit",
                &self.system_cache_data0_err2bit(),
            )
            .field(
                "system_cache_data0_fr11bi",
                &self.system_cache_data0_fr11bi(),
            )
            .field(
                "system_cache_data0_fr1nci",
                &self.system_cache_data0_fr1nci(),
            )
            .field(
                "system_cache_data0_frc1bi",
                &self.system_cache_data0_frc1bi(),
            )
            .field(
                "system_cache_data0_frcnci",
                &self.system_cache_data0_frcnci(),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SystemCacheData0EccErrorInjec {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SystemCacheData0EccErrorInjec {
            system_cache_data0_err1bit: u8,
            system_cache_data0_err2bit: u8,
            system_cache_data0_fr11bi: bool,
            system_cache_data0_fr1nci: bool,
            system_cache_data0_frc1bi: bool,
            system_cache_data0_frcnci: bool,
        }
        let proxy = SystemCacheData0EccErrorInjec {
            system_cache_data0_err1bit: self.system_cache_data0_err1bit(),
            system_cache_data0_err2bit: self.system_cache_data0_err2bit(),
            system_cache_data0_fr11bi: self.system_cache_data0_fr11bi(),
            system_cache_data0_fr1nci: self.system_cache_data0_fr1nci(),
            system_cache_data0_frc1bi: self.system_cache_data0_frc1bi(),
            system_cache_data0_frcnci: self.system_cache_data0_frcnci(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "System Cache Multibit ECC Error Information"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SystemCacheEccMultiErrorInfo(pub u32);
impl SystemCacheEccMultiErrorInfo {
    #[doc = "System Cache Multibit ECC Error"]
    #[must_use]
    #[inline(always)]
    pub const fn system_cache_eccm_tag(&self) -> super::vals::SystemCacheEccmTag {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SystemCacheEccmTag::from_bits(val as u8)
    }
    #[doc = "System Cache Multibit ECC Error"]
    #[inline(always)]
    pub const fn set_system_cache_eccm_tag(&mut self, val: super::vals::SystemCacheEccmTag) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "System Cache Multibit ECC Error on System Cache Command"]
    #[must_use]
    #[inline(always)]
    pub const fn system_cache_eccm_cmd(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "System Cache Multibit ECC Error on System Cache Command"]
    #[inline(always)]
    pub const fn set_system_cache_eccm_cmd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "System Cache Multibit ECC Error Master Number"]
    #[must_use]
    #[inline(always)]
    pub const fn system_cache_eccm_efmst(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "System Cache Multibit ECC Error Master Number"]
    #[inline(always)]
    pub const fn set_system_cache_eccm_efmst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "System Cache Multibit ECC Error Protection"]
    #[must_use]
    #[inline(always)]
    pub const fn system_cache_eccm_efprt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "System Cache Multibit ECC Error Protection"]
    #[inline(always)]
    pub const fn set_system_cache_eccm_efprt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "System Cache Multibit ECC Error Corresponding Syndrome"]
    #[must_use]
    #[inline(always)]
    pub const fn system_cache_eccm_efsyn(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "System Cache Multibit ECC Error Corresponding Syndrome"]
    #[inline(always)]
    pub const fn set_system_cache_eccm_efsyn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
}
impl Default for SystemCacheEccMultiErrorInfo {
    #[inline(always)]
    fn default() -> SystemCacheEccMultiErrorInfo {
        SystemCacheEccMultiErrorInfo(0u64 as u32)
    }
}
impl core::fmt::Debug for SystemCacheEccMultiErrorInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SystemCacheEccMultiErrorInfo")
            .field("system_cache_eccm_tag", &self.system_cache_eccm_tag())
            .field("system_cache_eccm_cmd", &self.system_cache_eccm_cmd())
            .field("system_cache_eccm_efmst", &self.system_cache_eccm_efmst())
            .field("system_cache_eccm_efprt", &self.system_cache_eccm_efprt())
            .field("system_cache_eccm_efsyn", &self.system_cache_eccm_efsyn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SystemCacheEccMultiErrorInfo {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SystemCacheEccMultiErrorInfo {
            system_cache_eccm_tag: super::vals::SystemCacheEccmTag,
            system_cache_eccm_cmd: bool,
            system_cache_eccm_efmst: u8,
            system_cache_eccm_efprt: u8,
            system_cache_eccm_efsyn: u8,
        }
        let proxy = SystemCacheEccMultiErrorInfo {
            system_cache_eccm_tag: self.system_cache_eccm_tag(),
            system_cache_eccm_cmd: self.system_cache_eccm_cmd(),
            system_cache_eccm_efmst: self.system_cache_eccm_efmst(),
            system_cache_eccm_efprt: self.system_cache_eccm_efprt(),
            system_cache_eccm_efsyn: self.system_cache_eccm_efsyn(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "System Cache Single-Bit ECC Error Information"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SystemCacheEccSingleErrorInfo(pub u32);
impl SystemCacheEccSingleErrorInfo {
    #[doc = "System Cache Single-Bit ECC Error"]
    #[must_use]
    #[inline(always)]
    pub const fn system_cache_eccs_tag(&self) -> super::vals::SystemCacheEccsTag {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SystemCacheEccsTag::from_bits(val as u8)
    }
    #[doc = "System Cache Single-Bit ECC Error"]
    #[inline(always)]
    pub const fn set_system_cache_eccs_tag(&mut self, val: super::vals::SystemCacheEccsTag) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "System Cache Single-Bit ECC Error on Cache Command"]
    #[must_use]
    #[inline(always)]
    pub const fn system_cache_eccs_cmd(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "System Cache Single-Bit ECC Error on Cache Command"]
    #[inline(always)]
    pub const fn set_system_cache_eccs_cmd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "System Cache Single-Bit ECC Error Master Number"]
    #[must_use]
    #[inline(always)]
    pub const fn system_cache_eccs_efmst(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "System Cache Single-Bit ECC Error Master Number"]
    #[inline(always)]
    pub const fn set_system_cache_eccs_efmst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "System Cache Single-Bit ECC Error Protection"]
    #[must_use]
    #[inline(always)]
    pub const fn system_cache_eccs_efprt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "System Cache Single-Bit ECC Error Protection"]
    #[inline(always)]
    pub const fn set_system_cache_eccs_efprt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "System Cache Single-Bit ECC Error Corresponding Syndrome"]
    #[must_use]
    #[inline(always)]
    pub const fn system_cache_eccs_efsyn(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "System Cache Single-Bit ECC Error Corresponding Syndrome"]
    #[inline(always)]
    pub const fn set_system_cache_eccs_efsyn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
}
impl Default for SystemCacheEccSingleErrorInfo {
    #[inline(always)]
    fn default() -> SystemCacheEccSingleErrorInfo {
        SystemCacheEccSingleErrorInfo(0u64 as u32)
    }
}
impl core::fmt::Debug for SystemCacheEccSingleErrorInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SystemCacheEccSingleErrorInfo")
            .field("system_cache_eccs_tag", &self.system_cache_eccs_tag())
            .field("system_cache_eccs_cmd", &self.system_cache_eccs_cmd())
            .field("system_cache_eccs_efmst", &self.system_cache_eccs_efmst())
            .field("system_cache_eccs_efprt", &self.system_cache_eccs_efprt())
            .field("system_cache_eccs_efsyn", &self.system_cache_eccs_efsyn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SystemCacheEccSingleErrorInfo {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SystemCacheEccSingleErrorInfo {
            system_cache_eccs_tag: super::vals::SystemCacheEccsTag,
            system_cache_eccs_cmd: bool,
            system_cache_eccs_efmst: u8,
            system_cache_eccs_efprt: u8,
            system_cache_eccs_efsyn: u8,
        }
        let proxy = SystemCacheEccSingleErrorInfo {
            system_cache_eccs_tag: self.system_cache_eccs_tag(),
            system_cache_eccs_cmd: self.system_cache_eccs_cmd(),
            system_cache_eccs_efmst: self.system_cache_eccs_efmst(),
            system_cache_eccs_efprt: self.system_cache_eccs_efprt(),
            system_cache_eccs_efsyn: self.system_cache_eccs_efsyn(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "System Cache TAG1 ECC Error Injection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SystemCacheTag1EccErrorInjec(pub u32);
impl SystemCacheTag1EccErrorInjec {
    #[doc = "Position of First Bit to Inject ECC Error"]
    #[must_use]
    #[inline(always)]
    pub const fn system_cache_tag1_err1bit(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Position of First Bit to Inject ECC Error"]
    #[inline(always)]
    pub const fn set_system_cache_tag1_err1bit(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Position of Second Bit to Inject ECC Error"]
    #[must_use]
    #[inline(always)]
    pub const fn systemcache_tag1_err2bit(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Position of Second Bit to Inject ECC Error"]
    #[inline(always)]
    pub const fn set_systemcache_tag1_err2bit(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Force One 1-Bit Data Inversion on System Cache TAG1 Write Access"]
    #[must_use]
    #[inline(always)]
    pub const fn system_cache_tag1_fr11bi(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Force One 1-Bit Data Inversion on System Cache TAG1 Write Access"]
    #[inline(always)]
    pub const fn set_system_cache_tag1_fr11bi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Force One Noncorrectable Data Inversion on System Cache TAG1 Write Access"]
    #[must_use]
    #[inline(always)]
    pub const fn system_cache_tag1_fr1nci(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Force One Noncorrectable Data Inversion on System Cache TAG1 Write Access"]
    #[inline(always)]
    pub const fn set_system_cache_tag1_fr1nci(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Force Continuous 1-Bit Data Inversions on System Cache TAG1 Write Access"]
    #[must_use]
    #[inline(always)]
    pub const fn system_cache_tag1_frc1bi(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Force Continuous 1-Bit Data Inversions on System Cache TAG1 Write Access"]
    #[inline(always)]
    pub const fn set_system_cache_tag1_frc1bi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Force Continuous Noncorrectable Data Inversions on System Cache TAG1 Write Access"]
    #[must_use]
    #[inline(always)]
    pub const fn system_cache_tag1_frcnci(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Force Continuous Noncorrectable Data Inversions on System Cache TAG1 Write Access"]
    #[inline(always)]
    pub const fn set_system_cache_tag1_frcnci(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for SystemCacheTag1EccErrorInjec {
    #[inline(always)]
    fn default() -> SystemCacheTag1EccErrorInjec {
        SystemCacheTag1EccErrorInjec(0u64 as u32)
    }
}
impl core::fmt::Debug for SystemCacheTag1EccErrorInjec {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SystemCacheTag1EccErrorInjec")
            .field(
                "system_cache_tag1_err1bit",
                &self.system_cache_tag1_err1bit(),
            )
            .field("systemcache_tag1_err2bit", &self.systemcache_tag1_err2bit())
            .field("system_cache_tag1_fr11bi", &self.system_cache_tag1_fr11bi())
            .field("system_cache_tag1_fr1nci", &self.system_cache_tag1_fr1nci())
            .field("system_cache_tag1_frc1bi", &self.system_cache_tag1_frc1bi())
            .field("system_cache_tag1_frcnci", &self.system_cache_tag1_frcnci())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SystemCacheTag1EccErrorInjec {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SystemCacheTag1EccErrorInjec {
            system_cache_tag1_err1bit: u8,
            systemcache_tag1_err2bit: u8,
            system_cache_tag1_fr11bi: bool,
            system_cache_tag1_fr1nci: bool,
            system_cache_tag1_frc1bi: bool,
            system_cache_tag1_frcnci: bool,
        }
        let proxy = SystemCacheTag1EccErrorInjec {
            system_cache_tag1_err1bit: self.system_cache_tag1_err1bit(),
            systemcache_tag1_err2bit: self.systemcache_tag1_err2bit(),
            system_cache_tag1_fr11bi: self.system_cache_tag1_fr11bi(),
            system_cache_tag1_fr1nci: self.system_cache_tag1_fr1nci(),
            system_cache_tag1_frc1bi: self.system_cache_tag1_frc1bi(),
            system_cache_tag1_frcnci: self.system_cache_tag1_frcnci(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "System Cache TAG0 ECC Error Injection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SytemCacheTag0EccErrorInjec(pub u32);
impl SytemCacheTag0EccErrorInjec {
    #[doc = "Position of First Bit to Inject ECC Error"]
    #[must_use]
    #[inline(always)]
    pub const fn system_cache_tag0_err1bit(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Position of First Bit to Inject ECC Error"]
    #[inline(always)]
    pub const fn set_system_cache_tag0_err1bit(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Position of Second Bit to Inject ECC Error"]
    #[must_use]
    #[inline(always)]
    pub const fn system_cache_tag0_err2bit(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Position of Second Bit to Inject ECC Error"]
    #[inline(always)]
    pub const fn set_system_cache_tag0_err2bit(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Force One 1-Bit Data Inversion on System Cache TAG0 Write Access"]
    #[must_use]
    #[inline(always)]
    pub const fn system_cache_tag0_fr11bi(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Force One 1-Bit Data Inversion on System Cache TAG0 Write Access"]
    #[inline(always)]
    pub const fn set_system_cache_tag0_fr11bi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Force One Noncorrectable Data Inversion on System Cache TAG0 Write Access"]
    #[must_use]
    #[inline(always)]
    pub const fn system_cache_tag0_fr1nci(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Force One Noncorrectable Data Inversion on System Cache TAG0 Write Access"]
    #[inline(always)]
    pub const fn set_system_cache_tag0_fr1nci(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Force Continuous 1-Bit Data Inversions on System Cache TAG0 Write Access"]
    #[must_use]
    #[inline(always)]
    pub const fn system_cache_tag0_frc1bi(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Force Continuous 1-Bit Data Inversions on System Cache TAG0 Write Access"]
    #[inline(always)]
    pub const fn set_system_cache_tag0_frc1bi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Force Continuous Noncorrectable Data Inversions on System Cache TAG0 Write Access"]
    #[must_use]
    #[inline(always)]
    pub const fn system_cache_tag0_frcnci(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Force Continuous Noncorrectable Data Inversions on System Cache TAG0 Write Access"]
    #[inline(always)]
    pub const fn set_system_cache_tag0_frcnci(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for SytemCacheTag0EccErrorInjec {
    #[inline(always)]
    fn default() -> SytemCacheTag0EccErrorInjec {
        SytemCacheTag0EccErrorInjec(0u64 as u32)
    }
}
impl core::fmt::Debug for SytemCacheTag0EccErrorInjec {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SytemCacheTag0EccErrorInjec")
            .field(
                "system_cache_tag0_err1bit",
                &self.system_cache_tag0_err1bit(),
            )
            .field(
                "system_cache_tag0_err2bit",
                &self.system_cache_tag0_err2bit(),
            )
            .field("system_cache_tag0_fr11bi", &self.system_cache_tag0_fr11bi())
            .field("system_cache_tag0_fr1nci", &self.system_cache_tag0_fr1nci())
            .field("system_cache_tag0_frc1bi", &self.system_cache_tag0_frc1bi())
            .field("system_cache_tag0_frcnci", &self.system_cache_tag0_frcnci())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SytemCacheTag0EccErrorInjec {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SytemCacheTag0EccErrorInjec {
            system_cache_tag0_err1bit: u8,
            system_cache_tag0_err2bit: u8,
            system_cache_tag0_fr11bi: bool,
            system_cache_tag0_fr1nci: bool,
            system_cache_tag0_frc1bi: bool,
            system_cache_tag0_frcnci: bool,
        }
        let proxy = SytemCacheTag0EccErrorInjec {
            system_cache_tag0_err1bit: self.system_cache_tag0_err1bit(),
            system_cache_tag0_err2bit: self.system_cache_tag0_err2bit(),
            system_cache_tag0_fr11bi: self.system_cache_tag0_fr11bi(),
            system_cache_tag0_fr1nci: self.system_cache_tag0_fr1nci(),
            system_cache_tag0_frc1bi: self.system_cache_tag0_frc1bi(),
            system_cache_tag0_frcnci: self.system_cache_tag0_frcnci(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
