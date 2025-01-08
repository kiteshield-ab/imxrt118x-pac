#[doc = "Code TCM ECC Error Injection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CodeTcmEccErrorInjec(pub u32);
impl CodeTcmEccErrorInjec {
    #[doc = "Position of First Bit to Inject ECC Error"]
    #[must_use]
    #[inline(always)]
    pub const fn code_tcm_err1bit(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Position of First Bit to Inject ECC Error"]
    #[inline(always)]
    pub const fn set_code_tcm_err1bit(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Position of Second Bit to Inject ECC Error"]
    #[must_use]
    #[inline(always)]
    pub const fn code_tcm_err2bit(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Position of Second Bit to Inject ECC Error"]
    #[inline(always)]
    pub const fn set_code_tcm_err2bit(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Force One 1-Bit Data Inversion on Code TCM Write Access"]
    #[must_use]
    #[inline(always)]
    pub const fn code_tcm_fr11bi(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Force One 1-Bit Data Inversion on Code TCM Write Access"]
    #[inline(always)]
    pub const fn set_code_tcm_fr11bi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Force One Noncorrectable Data Inversion on Code TCM Write Access"]
    #[must_use]
    #[inline(always)]
    pub const fn code_tcm_fr1nci(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Force One Noncorrectable Data Inversion on Code TCM Write Access"]
    #[inline(always)]
    pub const fn set_code_tcm_fr1nci(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Force Continuous 1-Bit Data Inversions on Code TCM Write Access"]
    #[must_use]
    #[inline(always)]
    pub const fn code_tcm_frc1bi(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Force Continuous 1-Bit Data Inversions on Code TCM Write Access"]
    #[inline(always)]
    pub const fn set_code_tcm_frc1bi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Force Continuous Noncorrectable Data Inversions on Code TCM Write Access"]
    #[must_use]
    #[inline(always)]
    pub const fn code_tcm_frcnci(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Force Continuous Noncorrectable Data Inversions on Code TCM Write Access"]
    #[inline(always)]
    pub const fn set_code_tcm_frcnci(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for CodeTcmEccErrorInjec {
    #[inline(always)]
    fn default() -> CodeTcmEccErrorInjec {
        CodeTcmEccErrorInjec(0u64 as u32)
    }
}
impl core::fmt::Debug for CodeTcmEccErrorInjec {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CodeTcmEccErrorInjec")
            .field("code_tcm_err1bit", &self.code_tcm_err1bit())
            .field("code_tcm_err2bit", &self.code_tcm_err2bit())
            .field("code_tcm_fr11bi", &self.code_tcm_fr11bi())
            .field("code_tcm_fr1nci", &self.code_tcm_fr1nci())
            .field("code_tcm_frc1bi", &self.code_tcm_frc1bi())
            .field("code_tcm_frcnci", &self.code_tcm_frcnci())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CodeTcmEccErrorInjec {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct CodeTcmEccErrorInjec {
            code_tcm_err1bit: u8,
            code_tcm_err2bit: u8,
            code_tcm_fr11bi: bool,
            code_tcm_fr1nci: bool,
            code_tcm_frc1bi: bool,
            code_tcm_frcnci: bool,
        }
        let proxy = CodeTcmEccErrorInjec {
            code_tcm_err1bit: self.code_tcm_err1bit(),
            code_tcm_err2bit: self.code_tcm_err2bit(),
            code_tcm_fr11bi: self.code_tcm_fr11bi(),
            code_tcm_fr1nci: self.code_tcm_fr1nci(),
            code_tcm_frc1bi: self.code_tcm_frc1bi(),
            code_tcm_frcnci: self.code_tcm_frcnci(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Code TCM Multibit ECC Error Information"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CodeTcmEccMultiErrorInfo(pub u32);
impl CodeTcmEccMultiErrorInfo {
    #[doc = "Code TCM Multibit ECC Error for Corresponding TCM Access Size"]
    #[must_use]
    #[inline(always)]
    pub const fn code_tcm_eccm_efsiz(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "Code TCM Multibit ECC Error for Corresponding TCM Access Size"]
    #[inline(always)]
    pub const fn set_code_tcm_eccm_efsiz(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
    }
    #[doc = "Code TCM Multibit ECC Error for Corresponding TCM Master"]
    #[must_use]
    #[inline(always)]
    pub const fn code_tcm_eccm_efmst(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Code TCM Multibit ECC Error for Corresponding TCM Master"]
    #[inline(always)]
    pub const fn set_code_tcm_eccm_efmst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "CODE_TCM Multibit ECC Error for Corresponding Access Protection Attribute"]
    #[must_use]
    #[inline(always)]
    pub const fn code_tcm_eccm_efprt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "CODE_TCM Multibit ECC Error for Corresponding Access Protection Attribute"]
    #[inline(always)]
    pub const fn set_code_tcm_eccm_efprt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Code TCM Multibit ECC Error for Corresponding Syndrome"]
    #[must_use]
    #[inline(always)]
    pub const fn code_tcm_eccm_efsyn(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x7f;
        val as u8
    }
    #[doc = "Code TCM Multibit ECC Error for Corresponding Syndrome"]
    #[inline(always)]
    pub const fn set_code_tcm_eccm_efsyn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 12usize)) | (((val as u32) & 0x7f) << 12usize);
    }
}
impl Default for CodeTcmEccMultiErrorInfo {
    #[inline(always)]
    fn default() -> CodeTcmEccMultiErrorInfo {
        CodeTcmEccMultiErrorInfo(0u64 as u32)
    }
}
impl core::fmt::Debug for CodeTcmEccMultiErrorInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CodeTcmEccMultiErrorInfo")
            .field("code_tcm_eccm_efsiz", &self.code_tcm_eccm_efsiz())
            .field("code_tcm_eccm_efmst", &self.code_tcm_eccm_efmst())
            .field("code_tcm_eccm_efprt", &self.code_tcm_eccm_efprt())
            .field("code_tcm_eccm_efsyn", &self.code_tcm_eccm_efsyn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CodeTcmEccMultiErrorInfo {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct CodeTcmEccMultiErrorInfo {
            code_tcm_eccm_efsiz: u8,
            code_tcm_eccm_efmst: u8,
            code_tcm_eccm_efprt: u8,
            code_tcm_eccm_efsyn: u8,
        }
        let proxy = CodeTcmEccMultiErrorInfo {
            code_tcm_eccm_efsiz: self.code_tcm_eccm_efsiz(),
            code_tcm_eccm_efmst: self.code_tcm_eccm_efmst(),
            code_tcm_eccm_efprt: self.code_tcm_eccm_efprt(),
            code_tcm_eccm_efsyn: self.code_tcm_eccm_efsyn(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Code TCM Single-Bit ECC Error Information"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CodeTcmEccSingleErrorInfo(pub u32);
impl CodeTcmEccSingleErrorInfo {
    #[doc = "Code TCM Single-Bit ECC Error for Corresponding TCM Access Size"]
    #[must_use]
    #[inline(always)]
    pub const fn code_tcm_eccs_efsiz(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "Code TCM Single-Bit ECC Error for Corresponding TCM Access Size"]
    #[inline(always)]
    pub const fn set_code_tcm_eccs_efsiz(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
    }
    #[doc = "Code TCM Single-Bit ECC Error for Corresponding TCM Master"]
    #[must_use]
    #[inline(always)]
    pub const fn code_tcm_eccs_efmst(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Code TCM Single-Bit ECC Error for Corresponding TCM Master"]
    #[inline(always)]
    pub const fn set_code_tcm_eccs_efmst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Code TCM Single-Bit ECC Error for Corresponding TCM Access Protection"]
    #[must_use]
    #[inline(always)]
    pub const fn code_tcm_eccs_efprt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Code TCM Single-Bit ECC Error for Corresponding TCM Access Protection"]
    #[inline(always)]
    pub const fn set_code_tcm_eccs_efprt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Code TCM Single-Bit ECC Error Corresponding Syndrome"]
    #[must_use]
    #[inline(always)]
    pub const fn code_tcm_eccs_efsyn(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x7f;
        val as u8
    }
    #[doc = "Code TCM Single-Bit ECC Error Corresponding Syndrome"]
    #[inline(always)]
    pub const fn set_code_tcm_eccs_efsyn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 12usize)) | (((val as u32) & 0x7f) << 12usize);
    }
}
impl Default for CodeTcmEccSingleErrorInfo {
    #[inline(always)]
    fn default() -> CodeTcmEccSingleErrorInfo {
        CodeTcmEccSingleErrorInfo(0u64 as u32)
    }
}
impl core::fmt::Debug for CodeTcmEccSingleErrorInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CodeTcmEccSingleErrorInfo")
            .field("code_tcm_eccs_efsiz", &self.code_tcm_eccs_efsiz())
            .field("code_tcm_eccs_efmst", &self.code_tcm_eccs_efmst())
            .field("code_tcm_eccs_efprt", &self.code_tcm_eccs_efprt())
            .field("code_tcm_eccs_efsyn", &self.code_tcm_eccs_efsyn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CodeTcmEccSingleErrorInfo {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct CodeTcmEccSingleErrorInfo {
            code_tcm_eccs_efsiz: u8,
            code_tcm_eccs_efmst: u8,
            code_tcm_eccs_efprt: u8,
            code_tcm_eccs_efsyn: u8,
        }
        let proxy = CodeTcmEccSingleErrorInfo {
            code_tcm_eccs_efsiz: self.code_tcm_eccs_efsiz(),
            code_tcm_eccs_efmst: self.code_tcm_eccs_efmst(),
            code_tcm_eccs_efprt: self.code_tcm_eccs_efprt(),
            code_tcm_eccs_efsyn: self.code_tcm_eccs_efsyn(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntSigEn(pub u32);
impl IntSigEn {
    #[doc = "Code TCM Access Multibit ECC Error Interrupt Signal Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn code_tcm_errm_int_sig_en(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Code TCM Access Multibit ECC Error Interrupt Signal Enable"]
    #[inline(always)]
    pub const fn set_code_tcm_errm_int_sig_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Code TCM Access Single-Bit ECC Error Interrupt Signal Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn code_tcm_errs_int_sig_en(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Code TCM Access Single-Bit ECC Error Interrupt Signal Enable"]
    #[inline(always)]
    pub const fn set_code_tcm_errs_int_sig_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "System TCM Access Multibit ECC Error Interrupt Signal Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_tcm_errm_int_sig_en(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "System TCM Access Multibit ECC Error Interrupt Signal Enable"]
    #[inline(always)]
    pub const fn set_sys_tcm_errm_int_sig_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "System TCM Access Single-Bit ECC Error Interrupt Signal Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_tcm_errs_int_sig_en(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "System TCM Access Single-Bit ECC Error Interrupt Signal Enable"]
    #[inline(always)]
    pub const fn set_sys_tcm_errs_int_sig_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
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
            .field("code_tcm_errm_int_sig_en", &self.code_tcm_errm_int_sig_en())
            .field("code_tcm_errs_int_sig_en", &self.code_tcm_errs_int_sig_en())
            .field("sys_tcm_errm_int_sig_en", &self.sys_tcm_errm_int_sig_en())
            .field("sys_tcm_errs_int_sig_en", &self.sys_tcm_errs_int_sig_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntSigEn {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct IntSigEn {
            code_tcm_errm_int_sig_en: bool,
            code_tcm_errs_int_sig_en: bool,
            sys_tcm_errm_int_sig_en: bool,
            sys_tcm_errs_int_sig_en: bool,
        }
        let proxy = IntSigEn {
            code_tcm_errm_int_sig_en: self.code_tcm_errm_int_sig_en(),
            code_tcm_errs_int_sig_en: self.code_tcm_errs_int_sig_en(),
            sys_tcm_errm_int_sig_en: self.sys_tcm_errm_int_sig_en(),
            sys_tcm_errs_int_sig_en: self.sys_tcm_errs_int_sig_en(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Status Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntStatEn(pub u32);
impl IntStatEn {
    #[doc = "Code TCM Access Multibit ECC Error Interrupt Status Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn code_tcm_errm_int_en(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Code TCM Access Multibit ECC Error Interrupt Status Enable"]
    #[inline(always)]
    pub const fn set_code_tcm_errm_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Code TCM Access Single-Bit ECC Error Interrupt Status Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn code_tcm_errs_int_en(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Code TCM Access Single-Bit ECC Error Interrupt Status Enable"]
    #[inline(always)]
    pub const fn set_code_tcm_errs_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "System TCM Access Multibit ECC Error Interrupt Status Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_tcm_errm_int_en(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "System TCM Access Multibit ECC Error Interrupt Status Enable"]
    #[inline(always)]
    pub const fn set_sys_tcm_errm_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "System TCM Access Single-Bit ECC Error Interrupt Status Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_tcm_errs_int_en(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "System TCM Access Single-Bit ECC Error Interrupt Status Enable"]
    #[inline(always)]
    pub const fn set_sys_tcm_errs_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
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
            .field("code_tcm_errm_int_en", &self.code_tcm_errm_int_en())
            .field("code_tcm_errs_int_en", &self.code_tcm_errs_int_en())
            .field("sys_tcm_errm_int_en", &self.sys_tcm_errm_int_en())
            .field("sys_tcm_errs_int_en", &self.sys_tcm_errs_int_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntStatEn {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct IntStatEn {
            code_tcm_errm_int_en: bool,
            code_tcm_errs_int_en: bool,
            sys_tcm_errm_int_en: bool,
            sys_tcm_errs_int_en: bool,
        }
        let proxy = IntStatEn {
            code_tcm_errm_int_en: self.code_tcm_errm_int_en(),
            code_tcm_errs_int_en: self.code_tcm_errs_int_en(),
            sys_tcm_errm_int_en: self.sys_tcm_errm_int_en(),
            sys_tcm_errs_int_en: self.sys_tcm_errs_int_en(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntStatus(pub u32);
impl IntStatus {
    #[doc = "Code TCM Access Multibit ECC Error Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn code_tcm_ecc_errm_int(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Code TCM Access Multibit ECC Error Interrupt Status"]
    #[inline(always)]
    pub const fn set_code_tcm_ecc_errm_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Code TCM Access Single-Bit ECC Error Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn code_tcm_ecc_errs_int(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Code TCM Access Single-Bit ECC Error Interrupt Status"]
    #[inline(always)]
    pub const fn set_code_tcm_ecc_errs_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "System TCM Access Multibit ECC Error Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_tcm_ecc_errm_int(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "System TCM Access Multibit ECC Error Interrupt Status"]
    #[inline(always)]
    pub const fn set_sys_tcm_ecc_errm_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "System TCM Access Single-Bit ECC Error Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_tcm_ecc_errs_int(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "System TCM Access Single-Bit ECC Error Interrupt Status"]
    #[inline(always)]
    pub const fn set_sys_tcm_ecc_errs_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
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
            .field("code_tcm_ecc_errm_int", &self.code_tcm_ecc_errm_int())
            .field("code_tcm_ecc_errs_int", &self.code_tcm_ecc_errs_int())
            .field("sys_tcm_ecc_errm_int", &self.sys_tcm_ecc_errm_int())
            .field("sys_tcm_ecc_errs_int", &self.sys_tcm_ecc_errs_int())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntStatus {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct IntStatus {
            code_tcm_ecc_errm_int: bool,
            code_tcm_ecc_errs_int: bool,
            sys_tcm_ecc_errm_int: bool,
            sys_tcm_ecc_errs_int: bool,
        }
        let proxy = IntStatus {
            code_tcm_ecc_errm_int: self.code_tcm_ecc_errm_int(),
            code_tcm_ecc_errs_int: self.code_tcm_ecc_errs_int(),
            sys_tcm_ecc_errm_int: self.sys_tcm_ecc_errm_int(),
            sys_tcm_ecc_errs_int: self.sys_tcm_ecc_errs_int(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "System TCM ECC Error Injection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SysTcmEccErrorInjec(pub u32);
impl SysTcmEccErrorInjec {
    #[doc = "Position of First Bit to Inject ECC Error"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_tcm_err1bit(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Position of First Bit to Inject ECC Error"]
    #[inline(always)]
    pub const fn set_sys_tcm_err1bit(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Position of Second Bit to Inject ECC Error"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_tcm_err2bit(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Position of Second Bit to Inject ECC Error"]
    #[inline(always)]
    pub const fn set_sys_tcm_err2bit(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Force One 1-Bit Data Inversion on System TCM Write Access"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_tcm_fr11bi(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Force One 1-Bit Data Inversion on System TCM Write Access"]
    #[inline(always)]
    pub const fn set_sys_tcm_fr11bi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Force One Noncorrectable Data Inversion on System TCM Write Access"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_tcm_fr1nci(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Force One Noncorrectable Data Inversion on System TCM Write Access"]
    #[inline(always)]
    pub const fn set_sys_tcm_fr1nci(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Force Continuous 1-Bit Data Inversions on System TCM Write Access"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_tcm_frc1bi(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Force Continuous 1-Bit Data Inversions on System TCM Write Access"]
    #[inline(always)]
    pub const fn set_sys_tcm_frc1bi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Force Continuous Noncorrectable Data Inversions on System TCM Write Access"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_tcm_frcnci(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Force Continuous Noncorrectable Data Inversions on System TCM Write Access"]
    #[inline(always)]
    pub const fn set_sys_tcm_frcnci(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for SysTcmEccErrorInjec {
    #[inline(always)]
    fn default() -> SysTcmEccErrorInjec {
        SysTcmEccErrorInjec(0u64 as u32)
    }
}
impl core::fmt::Debug for SysTcmEccErrorInjec {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SysTcmEccErrorInjec")
            .field("sys_tcm_err1bit", &self.sys_tcm_err1bit())
            .field("sys_tcm_err2bit", &self.sys_tcm_err2bit())
            .field("sys_tcm_fr11bi", &self.sys_tcm_fr11bi())
            .field("sys_tcm_fr1nci", &self.sys_tcm_fr1nci())
            .field("sys_tcm_frc1bi", &self.sys_tcm_frc1bi())
            .field("sys_tcm_frcnci", &self.sys_tcm_frcnci())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SysTcmEccErrorInjec {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SysTcmEccErrorInjec {
            sys_tcm_err1bit: u8,
            sys_tcm_err2bit: u8,
            sys_tcm_fr11bi: bool,
            sys_tcm_fr1nci: bool,
            sys_tcm_frc1bi: bool,
            sys_tcm_frcnci: bool,
        }
        let proxy = SysTcmEccErrorInjec {
            sys_tcm_err1bit: self.sys_tcm_err1bit(),
            sys_tcm_err2bit: self.sys_tcm_err2bit(),
            sys_tcm_fr11bi: self.sys_tcm_fr11bi(),
            sys_tcm_fr1nci: self.sys_tcm_fr1nci(),
            sys_tcm_frc1bi: self.sys_tcm_frc1bi(),
            sys_tcm_frcnci: self.sys_tcm_frcnci(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "System TCM Multibit ECC Error Information"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SysTcmEccMultiErrorInfo(pub u32);
impl SysTcmEccMultiErrorInfo {
    #[doc = "System TCM Multibit ECC Error for Corresponding TCM Access Size"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_tcm_eccm_efsiz(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "System TCM Multibit ECC Error for Corresponding TCM Access Size"]
    #[inline(always)]
    pub const fn set_sys_tcm_eccm_efsiz(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
    }
    #[doc = "System TCM Multibit ECC Error for Corresponding TCM Master"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_tcm_eccm_efmst(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "System TCM Multibit ECC Error for Corresponding TCM Master"]
    #[inline(always)]
    pub const fn set_sys_tcm_eccm_efmst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "System TCM Multibit ECC Error for Corresponding Access Protection Attribute"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_tcm_eccm_efprt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "System TCM Multibit ECC Error for Corresponding Access Protection Attribute"]
    #[inline(always)]
    pub const fn set_sys_tcm_eccm_efprt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "System TCM Multibit ECC Error for Corresponding Syndrome"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_tcm_eccm_efsyn(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x7f;
        val as u8
    }
    #[doc = "System TCM Multibit ECC Error for Corresponding Syndrome"]
    #[inline(always)]
    pub const fn set_sys_tcm_eccm_efsyn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 12usize)) | (((val as u32) & 0x7f) << 12usize);
    }
}
impl Default for SysTcmEccMultiErrorInfo {
    #[inline(always)]
    fn default() -> SysTcmEccMultiErrorInfo {
        SysTcmEccMultiErrorInfo(0u64 as u32)
    }
}
impl core::fmt::Debug for SysTcmEccMultiErrorInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SysTcmEccMultiErrorInfo")
            .field("sys_tcm_eccm_efsiz", &self.sys_tcm_eccm_efsiz())
            .field("sys_tcm_eccm_efmst", &self.sys_tcm_eccm_efmst())
            .field("sys_tcm_eccm_efprt", &self.sys_tcm_eccm_efprt())
            .field("sys_tcm_eccm_efsyn", &self.sys_tcm_eccm_efsyn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SysTcmEccMultiErrorInfo {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SysTcmEccMultiErrorInfo {
            sys_tcm_eccm_efsiz: u8,
            sys_tcm_eccm_efmst: u8,
            sys_tcm_eccm_efprt: u8,
            sys_tcm_eccm_efsyn: u8,
        }
        let proxy = SysTcmEccMultiErrorInfo {
            sys_tcm_eccm_efsiz: self.sys_tcm_eccm_efsiz(),
            sys_tcm_eccm_efmst: self.sys_tcm_eccm_efmst(),
            sys_tcm_eccm_efprt: self.sys_tcm_eccm_efprt(),
            sys_tcm_eccm_efsyn: self.sys_tcm_eccm_efsyn(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "System TCM Single-Bit ECC Error Information"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SysTcmEccSingleErrorInfo(pub u32);
impl SysTcmEccSingleErrorInfo {
    #[doc = "System TCM Single-Bit ECC Error for Corresponding TCM Access Size"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_tcm_eccs_efsiz(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "System TCM Single-Bit ECC Error for Corresponding TCM Access Size"]
    #[inline(always)]
    pub const fn set_sys_tcm_eccs_efsiz(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
    }
    #[doc = "System TCM Single-Bit ECC Error for Corresponding TCM Master"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_tcm_eccs_efmst(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "System TCM Single-Bit ECC Error for Corresponding TCM Master"]
    #[inline(always)]
    pub const fn set_sys_tcm_eccs_efmst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "System TCM Single-Bit ECC Error for Corresponding Access Protection Attribute"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_tcm_eccs_efprt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "System TCM Single-Bit ECC Error for Corresponding Access Protection Attribute"]
    #[inline(always)]
    pub const fn set_sys_tcm_eccs_efprt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "System TCM Single-Bit ECC Error for Corresponding Syndrome"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_tcm_eccs_efsyn(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x7f;
        val as u8
    }
    #[doc = "System TCM Single-Bit ECC Error for Corresponding Syndrome"]
    #[inline(always)]
    pub const fn set_sys_tcm_eccs_efsyn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 12usize)) | (((val as u32) & 0x7f) << 12usize);
    }
}
impl Default for SysTcmEccSingleErrorInfo {
    #[inline(always)]
    fn default() -> SysTcmEccSingleErrorInfo {
        SysTcmEccSingleErrorInfo(0u64 as u32)
    }
}
impl core::fmt::Debug for SysTcmEccSingleErrorInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SysTcmEccSingleErrorInfo")
            .field("sys_tcm_eccs_efsiz", &self.sys_tcm_eccs_efsiz())
            .field("sys_tcm_eccs_efmst", &self.sys_tcm_eccs_efmst())
            .field("sys_tcm_eccs_efprt", &self.sys_tcm_eccs_efprt())
            .field("sys_tcm_eccs_efsyn", &self.sys_tcm_eccs_efsyn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SysTcmEccSingleErrorInfo {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SysTcmEccSingleErrorInfo {
            sys_tcm_eccs_efsiz: u8,
            sys_tcm_eccs_efmst: u8,
            sys_tcm_eccs_efprt: u8,
            sys_tcm_eccs_efsyn: u8,
        }
        let proxy = SysTcmEccSingleErrorInfo {
            sys_tcm_eccs_efsiz: self.sys_tcm_eccs_efsiz(),
            sys_tcm_eccs_efmst: self.sys_tcm_eccs_efmst(),
            sys_tcm_eccs_efprt: self.sys_tcm_eccs_efprt(),
            sys_tcm_eccs_efsyn: self.sys_tcm_eccs_efsyn(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "TCM ECC Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcmeccr(pub u32);
impl Tcmeccr {
    #[doc = "TCM ECC Write Generation Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn wecc_dis(&self) -> super::vals::WeccDis {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::WeccDis::from_bits(val as u8)
    }
    #[doc = "TCM ECC Write Generation Disable"]
    #[inline(always)]
    pub const fn set_wecc_dis(&mut self, val: super::vals::WeccDis) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "TCM ECC Read Check Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn recc_dis(&self) -> super::vals::ReccDis {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::ReccDis::from_bits(val as u8)
    }
    #[doc = "TCM ECC Read Check Disable"]
    #[inline(always)]
    pub const fn set_recc_dis(&mut self, val: super::vals::ReccDis) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for Tcmeccr {
    #[inline(always)]
    fn default() -> Tcmeccr {
        Tcmeccr(0u64 as u32)
    }
}
impl core::fmt::Debug for Tcmeccr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcmeccr")
            .field("wecc_dis", &self.wecc_dis())
            .field("recc_dis", &self.recc_dis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcmeccr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Tcmeccr {
            wecc_dis: super::vals::WeccDis,
            recc_dis: super::vals::ReccDis,
        }
        let proxy = Tcmeccr {
            wecc_dis: self.wecc_dis(),
            recc_dis: self.recc_dis(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
