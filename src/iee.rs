#[doc = "IEE"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iee {
    ptr: *mut u8,
}
unsafe impl Send for Iee {}
unsafe impl Sync for Iee {}
impl Iee {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "IEE Global Configuration"]
    #[inline(always)]
    pub const fn gcfg(self) -> crate::common::Reg<regs::Gcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "IEE Status"]
    #[inline(always)]
    pub const fn sta(self) -> crate::common::Reg<regs::Sta, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "IEE Test Mode Register"]
    #[inline(always)]
    pub const fn tstmd(self) -> crate::common::Reg<regs::Tstmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "AES Mask Generation Seed"]
    #[inline(always)]
    pub const fn dpams(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Performance Counter, AES Slave Latency Threshold Value"]
    #[inline(always)]
    pub const fn pc_s_lt(self) -> crate::common::Reg<regs::PcSLt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Performance Counter, AES Master Latency Threshold"]
    #[inline(always)]
    pub const fn pc_m_lt(self) -> crate::common::Reg<regs::PcMLt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Performance Counter, Number of AES Block Encryptions"]
    #[inline(always)]
    pub const fn pc_blk_enc(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "Performance Counter, Number of AES Block Decryptions"]
    #[inline(always)]
    pub const fn pc_blk_dec(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "Performance Counter, Number of AXI Slave Read Transactions"]
    #[inline(always)]
    pub const fn pc_sr_trans(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "Performance Counter, Number of AXI Slave Write Transactions"]
    #[inline(always)]
    pub const fn pc_sw_trans(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "Performance Counter, Number of AXI Master Read Transactions"]
    #[inline(always)]
    pub const fn pc_mr_trans(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "Performance Counter, Number of AXI Master Write Transactions"]
    #[inline(always)]
    pub const fn pc_mw_trans(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "Performance Counter, Number of AXI Master Merge Buffer Read Transactions"]
    #[inline(always)]
    pub const fn pc_m_mbr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "Performance Counter, Upper Slave Read Transactions Byte Count"]
    #[inline(always)]
    pub const fn pc_sr_tbc_u(self) -> crate::common::Reg<regs::PcSrTbcU, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "Performance Counter, Lower Slave Read Transactions Byte Count"]
    #[inline(always)]
    pub const fn pc_sr_tbc_l(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "Performance Counter, Upper Slave Write Transactions Byte Count"]
    #[inline(always)]
    pub const fn pc_sw_tbc_u(self) -> crate::common::Reg<regs::PcSwTbcU, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "Performance Counter, Lower Slave Write Transactions Byte Count"]
    #[inline(always)]
    pub const fn pc_sw_tbc_l(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[doc = "Performance Counter, Upper Master Read Transactions Byte Count"]
    #[inline(always)]
    pub const fn pc_mr_tbc_u(self) -> crate::common::Reg<regs::PcMrTbcU, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "Performance Counter, Lower Master Read Transactions Byte Count"]
    #[inline(always)]
    pub const fn pc_mr_tbc_l(self) -> crate::common::Reg<regs::PcMrTbcL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "Performance Counter, Upper Master Write Transactions Byte Count"]
    #[inline(always)]
    pub const fn pc_mw_tbc_u(self) -> crate::common::Reg<regs::PcMwTbcU, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "Performance Counter, Lower Master Write Transactions Byte Count"]
    #[inline(always)]
    pub const fn pc_mw_tbc_l(self) -> crate::common::Reg<regs::PcMwTbcL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "Performance Counter, Number of AXI Slave Read Transactions with Latency Greater than the Threshold"]
    #[inline(always)]
    pub const fn pc_sr_tlgtt(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "Performance Counter, Number of AXI Slave Write Transactions with Latency Greater than the Threshold"]
    #[inline(always)]
    pub const fn pc_sw_tlgtt(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "Performance Counter, Number of AXI Master Read Transactions with Latency Greater than the Threshold"]
    #[inline(always)]
    pub const fn pc_mr_tlgtt(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[doc = "Performance Counter, Number of AXI Master Write Transactions with Latency Greater than the Threshold"]
    #[inline(always)]
    pub const fn pc_mw_tlgtt(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "Performance Counter, Upper Slave Read Latency Count"]
    #[inline(always)]
    pub const fn pc_sr_tlat_u(self) -> crate::common::Reg<regs::PcSrTlatU, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "Performance Counter, Lower Slave Read Latency Count"]
    #[inline(always)]
    pub const fn pc_sr_tlat_l(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "Performance Counter, Upper Slave Write Latency Count"]
    #[inline(always)]
    pub const fn pc_sw_tlat_u(self) -> crate::common::Reg<regs::PcSwTlatU, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "Performance Counter, Lower Slave Write Latency Count"]
    #[inline(always)]
    pub const fn pc_sw_tlat_l(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[doc = "Performance Counter, Upper Master Read Latency Count"]
    #[inline(always)]
    pub const fn pc_mr_tlat_u(self) -> crate::common::Reg<regs::PcMrTlatU, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[doc = "Performance Counter, Lower Master Read Latency Count"]
    #[inline(always)]
    pub const fn pc_mr_tlat_l(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
    }
    #[doc = "Performance Counter, Upper Master Write Latency Count"]
    #[inline(always)]
    pub const fn pc_mw_tlat_u(self) -> crate::common::Reg<regs::PcMwTlatU, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb8usize) as _) }
    }
    #[doc = "Performance Counter, Lower Master Write Latency Count"]
    #[inline(always)]
    pub const fn pc_mw_tlat_l(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbcusize) as _) }
    }
    #[doc = "Performance Counter, Upper Slave Read Total Non-Responding Time"]
    #[inline(always)]
    pub const fn pc_sr_tnrt_u(self) -> crate::common::Reg<regs::PcSrTnrtU, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[doc = "Performance Counter, Lower Slave Read Total Non-Responding Time"]
    #[inline(always)]
    pub const fn pc_sr_tnrt_l(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[doc = "Performance Counter, Upper Slave Write Total Non-Responding Time"]
    #[inline(always)]
    pub const fn pc_sw_tnrt_u(self) -> crate::common::Reg<regs::PcSwTnrtU, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize) as _) }
    }
    #[doc = "Performance Counter, Lower Slave Write Total Non-Responding Time"]
    #[inline(always)]
    pub const fn pc_sw_tnrt_l(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xccusize) as _) }
    }
    #[doc = "IEE Version ID Register 1"]
    #[inline(always)]
    pub const fn vidr1(self) -> crate::common::Reg<regs::Vidr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
    }
    #[doc = "IEE AES Version ID Register"]
    #[inline(always)]
    pub const fn aesvid(self) -> crate::common::Reg<regs::Aesvid, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf8usize) as _) }
    }
    #[doc = "Region Registers"]
    #[inline(always)]
    pub const fn regx(self, n: usize) -> Regx {
        assert!(n < 8usize);
        unsafe { Regx::from_ptr(self.ptr.add(0x0100usize + n * 256usize) as _) }
    }
    #[doc = "IEE AES Test Mode Data Buffer"]
    #[inline(always)]
    pub const fn aes_tst_db(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0f00usize + n * 4usize) as _) }
    }
}
#[doc = "Region Registers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Regx {
    ptr: *mut u8,
}
unsafe impl Send for Regx {}
unsafe impl Sync for Regx {}
impl Regx {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "IEE Region REGION Attribute Register."]
    #[inline(always)]
    pub const fn regattr(self) -> crate::common::Reg<regs::Regattr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "IEE Region REGION Page Offset Register"]
    #[inline(always)]
    pub const fn regpo(self) -> crate::common::Reg<regs::Regpo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "IEE Region REGION Key 1 Register"]
    #[inline(always)]
    pub const fn regkey1_(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize + n * 4usize) as _) }
    }
    #[doc = "IEE Region REGION Key 2 Register"]
    #[inline(always)]
    pub const fn regkey2_(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize + n * 4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
