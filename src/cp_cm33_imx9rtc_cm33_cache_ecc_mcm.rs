#[doc = "CM33_CACHE_ECC_MCM"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CpCm33Imx9rtc_cm33CacheEccMcm {
    ptr: *mut u8,
}
unsafe impl Send for CpCm33Imx9rtc_cm33CacheEccMcm {}
unsafe impl Sync for CpCm33Imx9rtc_cm33CacheEccMcm {}
impl CpCm33Imx9rtc_cm33CacheEccMcm {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "CACHE ECC Control"]
    #[inline(always)]
    pub const fn cache_eccr(self) -> crate::common::Reg<regs::CacheEccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Interrupt Status"]
    #[inline(always)]
    pub const fn int_status(self) -> crate::common::Reg<regs::IntStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Interrupt Status Enable"]
    #[inline(always)]
    pub const fn int_stat_en(self) -> crate::common::Reg<regs::IntStatEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub const fn int_sig_en(self) -> crate::common::Reg<regs::IntSigEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Code Cache Single-Bit ECC Error Information"]
    #[inline(always)]
    pub const fn code_cache_ecc_single_error_info(
        self,
    ) -> crate::common::Reg<regs::CodeCacheEccSingleErrorInfo, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "Code Cache Single-Bit ECC Error Address"]
    #[inline(always)]
    pub const fn code_cache_ecc_single_error_addr(
        self,
    ) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "Code Cache Multibit ECC Error Information"]
    #[inline(always)]
    pub const fn code_cache_ecc_multi_error_info(
        self,
    ) -> crate::common::Reg<regs::CodeCacheEccMultiErrorInfo, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "System Cache Single-Bit ECC Error Information"]
    #[inline(always)]
    pub const fn system_cache_ecc_single_error_info(
        self,
    ) -> crate::common::Reg<regs::SystemCacheEccSingleErrorInfo, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "System Cache Single-Bit ECC Error Address"]
    #[inline(always)]
    pub const fn system_cache_ecc_single_error_addr(
        self,
    ) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "System Cache Multibit ECC Error Information"]
    #[inline(always)]
    pub const fn system_cache_ecc_multi_error_info(
        self,
    ) -> crate::common::Reg<regs::SystemCacheEccMultiErrorInfo, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "System Cache Multibit ECC Error Data"]
    #[inline(always)]
    pub const fn system_cache_ecc_multi_error_data(
        self,
    ) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "Code Cache TAG0 ECC Error Injection"]
    #[inline(always)]
    pub const fn code_cache_tag0_ecc_error_injec(
        self,
    ) -> crate::common::Reg<regs::CodeCacheTag0EccErrorInjec, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "Code Cache TAG1 ECC Error Injection"]
    #[inline(always)]
    pub const fn code_cache_tag1_ecc_error_injec(
        self,
    ) -> crate::common::Reg<regs::CodeCacheTag1EccErrorInjec, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "Code Cache DATA0 ECC Error Injection"]
    #[inline(always)]
    pub const fn code_cache_data0_ecc_error_injec(
        self,
    ) -> crate::common::Reg<regs::CodeCacheData0EccErrorInjec, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "Code Cache DATA1 ECC Error Injection"]
    #[inline(always)]
    pub const fn code_cache_data1_ecc_error_injec(
        self,
    ) -> crate::common::Reg<regs::CodeCacheData1EccErrorInjec, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[doc = "System Cache TAG0 ECC Error Injection"]
    #[inline(always)]
    pub const fn sytem_cache_tag0_ecc_error_injec(
        self,
    ) -> crate::common::Reg<regs::SytemCacheTag0EccErrorInjec, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "System Cache TAG1 ECC Error Injection"]
    #[inline(always)]
    pub const fn system_cache_tag1_ecc_error_injec(
        self,
    ) -> crate::common::Reg<regs::SystemCacheTag1EccErrorInjec, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "System Cache DATA0 ECC Error Injection"]
    #[inline(always)]
    pub const fn system_cache_data0_ecc_error_injec(
        self,
    ) -> crate::common::Reg<regs::SystemCacheData0EccErrorInjec, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "System Cache DATA1 ECC Error Injection"]
    #[inline(always)]
    pub const fn ststem_cache_data1_ecc_error_injec(
        self,
    ) -> crate::common::Reg<regs::StstemCacheData1EccErrorInjec, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
}
pub mod regs;
pub mod vals;
