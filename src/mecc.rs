#[doc = "MECC64"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mecc {
    ptr: *mut u8,
}
unsafe impl Send for Mecc {}
unsafe impl Sync for Mecc {}
impl Mecc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Error Interrupt Status Register"]
    #[inline(always)]
    pub const fn err_status(self) -> crate::common::Reg<regs::ErrStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Error Interrupt Status Enable Register"]
    #[inline(always)]
    pub const fn err_stat_en(self) -> crate::common::Reg<regs::ErrStatEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Error Interrupt Enable Register"]
    #[inline(always)]
    pub const fn err_sig_en(self) -> crate::common::Reg<regs::ErrSigEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Error Injection On LOW 32 bits Of OCRAM Bank0 Write Data"]
    #[inline(always)]
    pub const fn err_data_inj_low0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Error Injection On HIGH 32 bits Of OCRAM Bank0 Write Data"]
    #[inline(always)]
    pub const fn err_data_inj_high0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Error Injection On 8 bits ECC code Of OCRAM Bank0 Write Data"]
    #[inline(always)]
    pub const fn err_ecc_inj0(self) -> crate::common::Reg<regs::ErrEccInj0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Error Injection On LOW 32 bits Of OCRAM Bank1 Write Data"]
    #[inline(always)]
    pub const fn err_data_inj_low1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Error Injection On HIGH 32 bits Of OCRAM Bank1 Write Data"]
    #[inline(always)]
    pub const fn err_data_inj_high1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Error Injection On 8 bits ECC code Of OCRAM Bank1 Write Data"]
    #[inline(always)]
    pub const fn err_ecc_inj1(self) -> crate::common::Reg<regs::ErrEccInj1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Error Injection On LOW 32 bits Of OCRAM Bank2 Write Data"]
    #[inline(always)]
    pub const fn err_data_inj_low2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Error Injection On HIGH 32 bits Of OCRAM Bank2 Write Data"]
    #[inline(always)]
    pub const fn err_data_inj_high2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Error Injection On 8 bits ECC code Of OCRAM Bank2 Write Data"]
    #[inline(always)]
    pub const fn err_ecc_inj2(self) -> crate::common::Reg<regs::ErrEccInj2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Error Injection On LOW 32 bits Of OCRAM Bank3 Write Data"]
    #[inline(always)]
    pub const fn err_data_inj_low3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Error Injection On HIGH 32 bits Of OCRAM Bank3 Write Data"]
    #[inline(always)]
    pub const fn err_data_inj_high3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "Error Injection On 8 bits ECC code Of OCRAM Bank3 Write Data"]
    #[inline(always)]
    pub const fn err_ecc_inj3(self) -> crate::common::Reg<regs::ErrEccInj3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "Single Error Address And ECC code On OCRAM Bank0"]
    #[inline(always)]
    pub const fn single_err_addr_ecc0(
        self,
    ) -> crate::common::Reg<regs::SingleErrAddrEcc0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "LOW 32 Bits Single Error Read Data On OCRAM Bank0"]
    #[inline(always)]
    pub const fn single_err_data_low0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "HIGH 32 Bits Single Error Read Data On OCRAM Bank0"]
    #[inline(always)]
    pub const fn single_err_data_high0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "LOW Single Error Bit Position On OCRAM Bank0"]
    #[inline(always)]
    pub const fn single_err_pos_low0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "HIGH Single Error Bit Position On OCRAM Bank0"]
    #[inline(always)]
    pub const fn single_err_pos_high0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "Single Error Address And ECC code On OCRAM Bank1"]
    #[inline(always)]
    pub const fn single_err_addr_ecc1(
        self,
    ) -> crate::common::Reg<regs::SingleErrAddrEcc1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "LOW 32 Bits Single Error Read Data On OCRAM Bank1"]
    #[inline(always)]
    pub const fn single_err_data_low1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "HIGH 32 Bits Single Error Read Data On OCRAM Bank1"]
    #[inline(always)]
    pub const fn single_err_data_high1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "LOW Single Error Bit Position On OCRAM Bank1"]
    #[inline(always)]
    pub const fn single_err_pos_low1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "HIGH Single Error Bit Position On OCRAM Bank1"]
    #[inline(always)]
    pub const fn single_err_pos_high1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "Single Error Address And ECC code On OCRAM Bank2"]
    #[inline(always)]
    pub const fn single_err_addr_ecc2(
        self,
    ) -> crate::common::Reg<regs::SingleErrAddrEcc2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "LOW 32 Bits Single Error Read Data On OCRAM Bank2"]
    #[inline(always)]
    pub const fn single_err_data_low2(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "HIGH 32 Bits Single Error Read Data On OCRAM Bank2"]
    #[inline(always)]
    pub const fn single_err_data_high2(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "LOW Single Error Bit Position On OCRAM Bank2"]
    #[inline(always)]
    pub const fn single_err_pos_low2(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "HIGH Single Error Bit Position On OCRAM Bank2"]
    #[inline(always)]
    pub const fn single_err_pos_high2(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "Single Error Address And ECC code On OCRAM Bank3"]
    #[inline(always)]
    pub const fn single_err_addr_ecc3(
        self,
    ) -> crate::common::Reg<regs::SingleErrAddrEcc3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "LOW 32 Bits Single Error Read Data On OCRAM Bank3"]
    #[inline(always)]
    pub const fn single_err_data_low3(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[doc = "HIGH 32 Bits Single Error Read Data On OCRAM Bank3"]
    #[inline(always)]
    pub const fn single_err_data_high3(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "LOW Single Error Bit Position On OCRAM Bank3"]
    #[inline(always)]
    pub const fn single_err_pos_low3(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "HIGH Single Error Bit Position On OCRAM Bank3"]
    #[inline(always)]
    pub const fn single_err_pos_high3(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "Multiple Error Address And ECC code On OCRAM Bank0"]
    #[inline(always)]
    pub const fn multi_err_addr_ecc0(
        self,
    ) -> crate::common::Reg<regs::MultiErrAddrEcc0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "LOW 32 Bits Multiple Error Read Data On OCRAM Bank0"]
    #[inline(always)]
    pub const fn multi_err_data_low0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "HIGH 32 Bits Multiple Error Read Data On OCRAM Bank0"]
    #[inline(always)]
    pub const fn multi_err_data_high0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "Multiple Error Address And ECC code On OCRAM Bank1"]
    #[inline(always)]
    pub const fn multi_err_addr_ecc1(
        self,
    ) -> crate::common::Reg<regs::MultiErrAddrEcc1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[doc = "LOW 32 Bits Multiple Error Read Data On OCRAM Bank1"]
    #[inline(always)]
    pub const fn multi_err_data_low1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "HIGH 32 Bits Multiple Error Read Data On OCRAM Bank1"]
    #[inline(always)]
    pub const fn multi_err_data_high1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "Multiple Error Address And ECC code On OCRAM Bank2"]
    #[inline(always)]
    pub const fn multi_err_addr_ecc2(
        self,
    ) -> crate::common::Reg<regs::MultiErrAddrEcc2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "LOW 32 Bits Multiple Error Read Data On OCRAM Bank2"]
    #[inline(always)]
    pub const fn multi_err_data_low2(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "HIGH 32 Bits Multiple Error Read Data On OCRAM Bank2"]
    #[inline(always)]
    pub const fn multi_err_data_high2(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[doc = "Multiple Error Address And ECC code On OCRAM Bank3"]
    #[inline(always)]
    pub const fn multi_err_addr_ecc3(
        self,
    ) -> crate::common::Reg<regs::MultiErrAddrEcc3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[doc = "LOW 32 Bits Multiple Error Read Data On OCRAM Bank3"]
    #[inline(always)]
    pub const fn multi_err_data_low3(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
    }
    #[doc = "HIGH 32 Bits Multiple Error Read Data On OCRAM Bank3"]
    #[inline(always)]
    pub const fn multi_err_data_high3(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb8usize) as _) }
    }
    #[doc = "OCRAM Pipeline And ECC Enable"]
    #[inline(always)]
    pub const fn pipe_ecc_en(self) -> crate::common::Reg<regs::PipeEccEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "Pending Status"]
    #[inline(always)]
    pub const fn pending_stat(self) -> crate::common::Reg<regs::PendingStat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
}
pub mod regs;
pub mod vals;
