#[doc = "CM33_TCM_MCM"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CpCm33Imx9rtc_cm33TcmMcm {
    ptr: *mut u8,
}
unsafe impl Send for CpCm33Imx9rtc_cm33TcmMcm {}
unsafe impl Sync for CpCm33Imx9rtc_cm33TcmMcm {}
impl CpCm33Imx9rtc_cm33TcmMcm {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "TCM ECC Control"]
    #[inline(always)]
    pub const fn tcmeccr(self) -> crate::common::Reg<regs::Tcmeccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
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
    #[doc = "Code TCM Single-Bit ECC Error Information"]
    #[inline(always)]
    pub const fn code_tcm_ecc_single_error_info(
        self,
    ) -> crate::common::Reg<regs::CodeTcmEccSingleErrorInfo, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "Code TCM Single-Bit ECC Error Address"]
    #[inline(always)]
    pub const fn code_tcm_ecc_single_error_addr(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "Code TCM Multibit ECC Error Information"]
    #[inline(always)]
    pub const fn code_tcm_ecc_multi_error_info(
        self,
    ) -> crate::common::Reg<regs::CodeTcmEccMultiErrorInfo, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "Code TCM Multibit ECC Error Address"]
    #[inline(always)]
    pub const fn code_tcm_ecc_multi_error_addr(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "System TCM Single-Bit ECC Error Information"]
    #[inline(always)]
    pub const fn sys_tcm_ecc_single_error_info(
        self,
    ) -> crate::common::Reg<regs::SysTcmEccSingleErrorInfo, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "System TCM Single-Bit ECC Error Address"]
    #[inline(always)]
    pub const fn sys_tcm_ecc_single_error_addr(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "System TCM Multibit ECC Error Information"]
    #[inline(always)]
    pub const fn sys_tcm_ecc_multi_error_info(
        self,
    ) -> crate::common::Reg<regs::SysTcmEccMultiErrorInfo, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "System TCM Multibit ECC Error Address"]
    #[inline(always)]
    pub const fn sys_tcm_ecc_multi_error_addr(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "Code TCM ECC Error Injection"]
    #[inline(always)]
    pub const fn code_tcm_ecc_error_injec(
        self,
    ) -> crate::common::Reg<regs::CodeTcmEccErrorInjec, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "System TCM ECC Error Injection"]
    #[inline(always)]
    pub const fn sys_tcm_ecc_error_injec(
        self,
    ) -> crate::common::Reg<regs::SysTcmEccErrorInjec, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
}
pub mod regs;
pub mod vals;
