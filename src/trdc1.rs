#[doc = "TRDC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trdc1 {
    ptr: *mut u8,
}
unsafe impl Send for Trdc1 {}
unsafe impl Sync for Trdc1 {}
impl Trdc1 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "TRDC Register"]
    #[inline(always)]
    pub const fn trdc_cr(self) -> crate::common::Reg<super::trdc::regs::TrdcCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "TRDC Hardware Configuration Register 0"]
    #[inline(always)]
    pub const fn trdc_hwcfg0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::TrdcHwcfg0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
    }
    #[doc = "TRDC Hardware Configuration Register 1"]
    #[inline(always)]
    pub const fn trdc_hwcfg1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::TrdcHwcfg1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4usize) as _) }
    }
    #[doc = "TRDC Hardware Configuration Register 2"]
    #[inline(always)]
    pub const fn trdc_hwcfg2(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf8usize) as _) }
    }
    #[doc = "TRDC Hardware Configuration Register 3"]
    #[inline(always)]
    pub const fn trdc_hwcfg3(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xfcusize) as _) }
    }
    #[doc = "Domain Assignment Configuration Register"]
    #[inline(always)]
    pub const fn dacfg0(self) -> crate::common::Reg<super::trdc::regs::Dacfg, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "Domain Assignment Configuration Register"]
    #[inline(always)]
    pub const fn dacfg1(self) -> crate::common::Reg<super::trdc::regs::Dacfg, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0101usize) as _) }
    }
    #[doc = "Domain Assignment Configuration Register"]
    #[inline(always)]
    pub const fn dacfg2(self) -> crate::common::Reg<super::trdc::regs::Dacfg, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0102usize) as _) }
    }
    #[doc = "Domain Assignment Configuration Register"]
    #[inline(always)]
    pub const fn dacfg3(self) -> crate::common::Reg<super::trdc::regs::Dacfg, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0103usize) as _) }
    }
    #[doc = "TRDC IDAU Control Register"]
    #[inline(always)]
    pub const fn trdc_idau_cr(
        self,
    ) -> crate::common::Reg<super::trdc::regs::TrdcIdauCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c0usize) as _) }
    }
    #[doc = "TRDC FLW Control"]
    #[inline(always)]
    pub const fn trdc_flw_ctl(
        self,
    ) -> crate::common::Reg<super::trdc::regs::TrdcFlwCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01e0usize) as _) }
    }
    #[doc = "TRDC FLW Physical Base"]
    #[inline(always)]
    pub const fn trdc_flw_pbase(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01e4usize) as _) }
    }
    #[doc = "TRDC FLW Array Base"]
    #[inline(always)]
    pub const fn trdc_flw_abase(
        self,
    ) -> crate::common::Reg<super::trdc::regs::TrdcFlwAbase, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01e8usize) as _) }
    }
    #[doc = "TRDC FLW Block Count"]
    #[inline(always)]
    pub const fn trdc_flw_bcnt(
        self,
    ) -> crate::common::Reg<super::trdc::regs::TrdcFlwBcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01ecusize) as _) }
    }
    #[doc = "TRDC Fault Domain ID"]
    #[inline(always)]
    pub const fn trdc_fdid(
        self,
    ) -> crate::common::Reg<super::trdc::regs::TrdcFdid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01fcusize) as _) }
    }
    #[doc = "TRDC Domain Error Location Register"]
    #[inline(always)]
    pub const fn trdc_derrloc(
        self,
        n: usize,
    ) -> crate::common::Reg<super::trdc::regs::TrdcDerrloc, crate::common::R> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize + n * 4usize) as _) }
    }
    #[doc = "MBC Domain Error Word0 Register"]
    #[inline(always)]
    pub const fn mbc0_derr_w0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
    #[doc = "MBC Domain Error Word1 Register"]
    #[inline(always)]
    pub const fn mbc0_derr_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::MbcDerrW1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0404usize) as _) }
    }
    #[doc = "MBC Domain Error Word3 Register"]
    #[inline(always)]
    pub const fn mbc0_derr_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::MbcDerrW3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x040cusize) as _) }
    }
    #[doc = "MBC Domain Error Word0 Register"]
    #[inline(always)]
    pub const fn mbc1_derr_w0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0410usize) as _) }
    }
    #[doc = "MBC Domain Error Word1 Register"]
    #[inline(always)]
    pub const fn mbc1_derr_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::MbcDerrW1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0414usize) as _) }
    }
    #[doc = "MBC Domain Error Word3 Register"]
    #[inline(always)]
    pub const fn mbc1_derr_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::MbcDerrW3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x041cusize) as _) }
    }
    #[doc = "MRC Domain Error Word0 Register"]
    #[inline(always)]
    pub const fn mrc0_derr_w0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0480usize) as _) }
    }
    #[doc = "MRC Domain Error Word1 Register"]
    #[inline(always)]
    pub const fn mrc0_derr_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::MrcDerrW1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0484usize) as _) }
    }
    #[doc = "MRC Domain Error Word3 Register"]
    #[inline(always)]
    pub const fn mrc0_derr_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::MrcDerrW3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x048cusize) as _) }
    }
    #[doc = "MRC Domain Error Word0 Register"]
    #[inline(always)]
    pub const fn mrc1_derr_w0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0490usize) as _) }
    }
    #[doc = "MRC Domain Error Word1 Register"]
    #[inline(always)]
    pub const fn mrc1_derr_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::MrcDerrW1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0494usize) as _) }
    }
    #[doc = "MRC Domain Error Word3 Register"]
    #[inline(always)]
    pub const fn mrc1_derr_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::MrcDerrW3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x049cusize) as _) }
    }
    #[doc = "Process Identifier"]
    #[inline(always)]
    pub const fn pid0(self) -> crate::common::Reg<super::trdc::regs::Pid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0700usize) as _) }
    }
    #[doc = "Process Identifier"]
    #[inline(always)]
    pub const fn pid1(self) -> crate::common::Reg<super::trdc::regs::Pid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0704usize) as _) }
    }
    #[doc = "DAC Master Domain Assignment Register"]
    #[inline(always)]
    pub const fn mda_w0_0_dfmt0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Dfmt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0800usize) as _) }
    }
    #[doc = "DAC Master Domain Assignment Register"]
    #[inline(always)]
    pub const fn mda_w0_1_dfmt0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Dfmt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0820usize) as _) }
    }
    #[doc = "DAC Master Domain Assignment Register"]
    #[inline(always)]
    pub const fn mda_w1_1_dfmt0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Dfmt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0824usize) as _) }
    }
    #[doc = "DAC Master Domain Assignment Register"]
    #[inline(always)]
    pub const fn mda_w2_1_dfmt0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Dfmt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0828usize) as _) }
    }
    #[doc = "DAC Master Domain Assignment Register"]
    #[inline(always)]
    pub const fn mda_w0_2_dfmt1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Dfmt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0840usize) as _) }
    }
    #[doc = "DAC Master Domain Assignment Register"]
    #[inline(always)]
    pub const fn mda_w0_3_dfmt1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Dfmt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0860usize) as _) }
    }
    #[doc = "MBC Global Configuration Register"]
    #[inline(always)]
    pub const fn mbc0_mem0_glbcfg(
        self,
    ) -> crate::common::Reg<super::trdc::regs::MbcGlbcfg, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0000usize) as _) }
    }
    #[doc = "MBC Global Configuration Register"]
    #[inline(always)]
    pub const fn mbc0_mem1_glbcfg(
        self,
    ) -> crate::common::Reg<super::trdc::regs::MbcGlbcfg, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0004usize) as _) }
    }
    #[doc = "MBC Global Configuration Register"]
    #[inline(always)]
    pub const fn mbc0_mem2_glbcfg(
        self,
    ) -> crate::common::Reg<super::trdc::regs::MbcGlbcfg, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0008usize) as _) }
    }
    #[doc = "MBC Global Configuration Register"]
    #[inline(always)]
    pub const fn mbc0_mem3_glbcfg(
        self,
    ) -> crate::common::Reg<super::trdc::regs::MbcGlbcfg, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_000cusize) as _) }
    }
    #[doc = "MBC NonSecure Enable Block Index"]
    #[inline(always)]
    pub const fn mbc0_nse_blk_index(
        self,
    ) -> crate::common::Reg<super::trdc::regs::NseBlkIndex, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0010usize) as _) }
    }
    #[doc = "MBC NonSecure Enable Block Set"]
    #[inline(always)]
    pub const fn mbc0_nse_blk_set(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0014usize) as _) }
    }
    #[doc = "MBC NonSecure Enable Block Clear"]
    #[inline(always)]
    pub const fn mbc0_nse_blk_clr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0018usize) as _) }
    }
    #[doc = "MBC NonSecure Enable Block Clear All"]
    #[inline(always)]
    pub const fn mbc0_nse_blk_clr_all(
        self,
    ) -> crate::common::Reg<super::trdc::regs::NseBlkClrAll, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_001cusize) as _) }
    }
    #[doc = "MBC Global Access Control"]
    #[inline(always)]
    pub const fn mbc0_memn_glbac0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Glbac0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0020usize) as _) }
    }
    #[doc = "MBC Global Access Control"]
    #[inline(always)]
    pub const fn mbc0_memn_glbac1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Glbac, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0024usize) as _) }
    }
    #[doc = "MBC Global Access Control"]
    #[inline(always)]
    pub const fn mbc0_memn_glbac2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Glbac, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0028usize) as _) }
    }
    #[doc = "MBC Global Access Control"]
    #[inline(always)]
    pub const fn mbc0_memn_glbac3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Glbac, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_002cusize) as _) }
    }
    #[doc = "MBC Global Access Control"]
    #[inline(always)]
    pub const fn mbc0_memn_glbac4(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Glbac, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0030usize) as _) }
    }
    #[doc = "MBC Global Access Control"]
    #[inline(always)]
    pub const fn mbc0_memn_glbac5(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Glbac, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0034usize) as _) }
    }
    #[doc = "MBC Global Access Control"]
    #[inline(always)]
    pub const fn mbc0_memn_glbac6(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Glbac, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0038usize) as _) }
    }
    #[doc = "MBC Global Access Control"]
    #[inline(always)]
    pub const fn mbc0_memn_glbac7(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Glbac, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_003cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0040usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0044usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0048usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_004cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_cfg_w4(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0050usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_cfg_w5(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0054usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_cfg_w6(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0058usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_cfg_w7(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_005cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_cfg_w8(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0060usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_cfg_w9(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0064usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_cfg_w10(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0068usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_cfg_w11(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_006cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_cfg_w12(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0070usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_cfg_w13(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0074usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_cfg_w14(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0078usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_cfg_w15(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_007cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0140usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_nse_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0144usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_nse_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0148usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_nse_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_014cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem1_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0180usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem1_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_01a0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem2_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_01a8usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem2_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_01c8usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem3_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_01d0usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem3_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_01f0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom1_mem0_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0240usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom1_mem0_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0244usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom1_mem0_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0248usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom1_mem0_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_024cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom1_mem0_blk_cfg_w4(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0250usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom1_mem0_blk_cfg_w5(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0254usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom1_mem0_blk_cfg_w6(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0258usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom1_mem0_blk_cfg_w7(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_025cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom1_mem0_blk_cfg_w8(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0260usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom1_mem0_blk_cfg_w9(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0264usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom1_mem0_blk_cfg_w10(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0268usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom1_mem0_blk_cfg_w11(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_026cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom1_mem0_blk_cfg_w12(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0270usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom1_mem0_blk_cfg_w13(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0274usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom1_mem0_blk_cfg_w14(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0278usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom1_mem0_blk_cfg_w15(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_027cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom1_mem0_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0340usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom1_mem0_blk_nse_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0344usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom1_mem0_blk_nse_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0348usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom1_mem0_blk_nse_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_034cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom1_mem1_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0380usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom1_mem1_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_03a0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom1_mem2_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_03a8usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom1_mem2_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_03c8usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom1_mem3_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_03d0usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom1_mem3_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_03f0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom2_mem0_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0440usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom2_mem0_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0444usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom2_mem0_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0448usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom2_mem0_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_044cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom2_mem0_blk_cfg_w4(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0450usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom2_mem0_blk_cfg_w5(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0454usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom2_mem0_blk_cfg_w6(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0458usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom2_mem0_blk_cfg_w7(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_045cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom2_mem0_blk_cfg_w8(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0460usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom2_mem0_blk_cfg_w9(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0464usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom2_mem0_blk_cfg_w10(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0468usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom2_mem0_blk_cfg_w11(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_046cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom2_mem0_blk_cfg_w12(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0470usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom2_mem0_blk_cfg_w13(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0474usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom2_mem0_blk_cfg_w14(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0478usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom2_mem0_blk_cfg_w15(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_047cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom2_mem0_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0540usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom2_mem0_blk_nse_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0544usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom2_mem0_blk_nse_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0548usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom2_mem0_blk_nse_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_054cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom2_mem1_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0580usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom2_mem1_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_05a0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom2_mem2_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_05a8usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom2_mem2_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_05c8usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom2_mem3_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_05d0usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom2_mem3_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_05f0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom3_mem0_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0640usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom3_mem0_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0644usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom3_mem0_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0648usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom3_mem0_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_064cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom3_mem0_blk_cfg_w4(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0650usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom3_mem0_blk_cfg_w5(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0654usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom3_mem0_blk_cfg_w6(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0658usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom3_mem0_blk_cfg_w7(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_065cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom3_mem0_blk_cfg_w8(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0660usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom3_mem0_blk_cfg_w9(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0664usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom3_mem0_blk_cfg_w10(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0668usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom3_mem0_blk_cfg_w11(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_066cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom3_mem0_blk_cfg_w12(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0670usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom3_mem0_blk_cfg_w13(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0674usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom3_mem0_blk_cfg_w14(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0678usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom3_mem0_blk_cfg_w15(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_067cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom3_mem0_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0740usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom3_mem0_blk_nse_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0744usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom3_mem0_blk_nse_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0748usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom3_mem0_blk_nse_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_074cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom3_mem1_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0780usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom3_mem1_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_07a0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom3_mem2_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_07a8usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom3_mem2_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_07c8usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom3_mem3_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_07d0usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom3_mem3_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_07f0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom4_mem0_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0840usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom4_mem0_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0844usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom4_mem0_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0848usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom4_mem0_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_084cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom4_mem0_blk_cfg_w4(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0850usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom4_mem0_blk_cfg_w5(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0854usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom4_mem0_blk_cfg_w6(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0858usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom4_mem0_blk_cfg_w7(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_085cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom4_mem0_blk_cfg_w8(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0860usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom4_mem0_blk_cfg_w9(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0864usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom4_mem0_blk_cfg_w10(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0868usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom4_mem0_blk_cfg_w11(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_086cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom4_mem0_blk_cfg_w12(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0870usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom4_mem0_blk_cfg_w13(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0874usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom4_mem0_blk_cfg_w14(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0878usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom4_mem0_blk_cfg_w15(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_087cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom4_mem0_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0940usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom4_mem0_blk_nse_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0944usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom4_mem0_blk_nse_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0948usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom4_mem0_blk_nse_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_094cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom4_mem1_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0980usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom4_mem1_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_09a0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom4_mem2_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_09a8usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom4_mem2_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_09c8usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom4_mem3_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_09d0usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom4_mem3_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_09f0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom5_mem0_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0a40usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom5_mem0_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0a44usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom5_mem0_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0a48usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom5_mem0_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0a4cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom5_mem0_blk_cfg_w4(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0a50usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom5_mem0_blk_cfg_w5(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0a54usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom5_mem0_blk_cfg_w6(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0a58usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom5_mem0_blk_cfg_w7(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0a5cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom5_mem0_blk_cfg_w8(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0a60usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom5_mem0_blk_cfg_w9(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0a64usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom5_mem0_blk_cfg_w10(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0a68usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom5_mem0_blk_cfg_w11(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0a6cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom5_mem0_blk_cfg_w12(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0a70usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom5_mem0_blk_cfg_w13(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0a74usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom5_mem0_blk_cfg_w14(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0a78usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom5_mem0_blk_cfg_w15(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0a7cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom5_mem0_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0b40usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom5_mem0_blk_nse_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0b44usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom5_mem0_blk_nse_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0b48usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom5_mem0_blk_nse_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0b4cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom5_mem1_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0b80usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom5_mem1_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0ba0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom5_mem2_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0ba8usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom5_mem2_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0bc8usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom5_mem3_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0bd0usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom5_mem3_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0bf0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom6_mem0_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0c40usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom6_mem0_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0c44usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom6_mem0_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0c48usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom6_mem0_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0c4cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom6_mem0_blk_cfg_w4(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0c50usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom6_mem0_blk_cfg_w5(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0c54usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom6_mem0_blk_cfg_w6(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0c58usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom6_mem0_blk_cfg_w7(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0c5cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom6_mem0_blk_cfg_w8(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0c60usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom6_mem0_blk_cfg_w9(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0c64usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom6_mem0_blk_cfg_w10(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0c68usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom6_mem0_blk_cfg_w11(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0c6cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom6_mem0_blk_cfg_w12(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0c70usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom6_mem0_blk_cfg_w13(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0c74usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom6_mem0_blk_cfg_w14(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0c78usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom6_mem0_blk_cfg_w15(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0c7cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom6_mem0_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0d40usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom6_mem0_blk_nse_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0d44usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom6_mem0_blk_nse_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0d48usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom6_mem0_blk_nse_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0d4cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom6_mem1_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0d80usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom6_mem1_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0da0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom6_mem2_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0da8usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom6_mem2_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0dc8usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom6_mem3_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0dd0usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom6_mem3_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0df0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom7_mem0_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0e40usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom7_mem0_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0e44usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom7_mem0_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0e48usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom7_mem0_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0e4cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom7_mem0_blk_cfg_w4(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0e50usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom7_mem0_blk_cfg_w5(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0e54usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom7_mem0_blk_cfg_w6(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0e58usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom7_mem0_blk_cfg_w7(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0e5cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom7_mem0_blk_cfg_w8(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0e60usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom7_mem0_blk_cfg_w9(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0e64usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom7_mem0_blk_cfg_w10(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0e68usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom7_mem0_blk_cfg_w11(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0e6cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom7_mem0_blk_cfg_w12(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0e70usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom7_mem0_blk_cfg_w13(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0e74usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom7_mem0_blk_cfg_w14(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0e78usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom7_mem0_blk_cfg_w15(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0e7cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom7_mem0_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0f40usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom7_mem0_blk_nse_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0f44usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom7_mem0_blk_nse_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0f48usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom7_mem0_blk_nse_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0f4cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom7_mem1_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0f80usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom7_mem1_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0fa0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom7_mem2_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0fa8usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom7_mem2_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0fc8usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom7_mem3_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0fd0usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom7_mem3_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_0ff0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom8_mem0_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1040usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom8_mem0_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1044usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom8_mem0_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1048usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom8_mem0_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_104cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom8_mem0_blk_cfg_w4(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1050usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom8_mem0_blk_cfg_w5(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1054usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom8_mem0_blk_cfg_w6(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1058usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom8_mem0_blk_cfg_w7(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_105cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom8_mem0_blk_cfg_w8(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1060usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom8_mem0_blk_cfg_w9(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1064usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom8_mem0_blk_cfg_w10(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1068usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom8_mem0_blk_cfg_w11(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_106cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom8_mem0_blk_cfg_w12(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1070usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom8_mem0_blk_cfg_w13(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1074usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom8_mem0_blk_cfg_w14(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1078usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom8_mem0_blk_cfg_w15(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_107cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom8_mem0_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1140usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom8_mem0_blk_nse_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1144usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom8_mem0_blk_nse_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1148usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom8_mem0_blk_nse_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_114cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom8_mem1_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1180usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom8_mem1_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_11a0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom8_mem2_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_11a8usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom8_mem2_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_11c8usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom8_mem3_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_11d0usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom8_mem3_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_11f0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom9_mem0_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1240usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom9_mem0_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1244usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom9_mem0_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1248usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom9_mem0_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_124cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom9_mem0_blk_cfg_w4(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1250usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom9_mem0_blk_cfg_w5(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1254usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom9_mem0_blk_cfg_w6(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1258usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom9_mem0_blk_cfg_w7(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_125cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom9_mem0_blk_cfg_w8(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1260usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom9_mem0_blk_cfg_w9(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1264usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom9_mem0_blk_cfg_w10(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1268usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom9_mem0_blk_cfg_w11(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_126cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom9_mem0_blk_cfg_w12(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1270usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom9_mem0_blk_cfg_w13(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1274usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom9_mem0_blk_cfg_w14(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1278usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom9_mem0_blk_cfg_w15(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_127cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom9_mem0_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1340usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom9_mem0_blk_nse_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1344usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom9_mem0_blk_nse_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1348usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom9_mem0_blk_nse_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_134cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom9_mem1_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1380usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom9_mem1_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_13a0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom9_mem2_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_13a8usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom9_mem2_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_13c8usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom9_mem3_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_13d0usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom9_mem3_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_13f0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom10_mem0_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1440usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom10_mem0_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1444usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom10_mem0_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1448usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom10_mem0_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_144cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom10_mem0_blk_cfg_w4(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1450usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom10_mem0_blk_cfg_w5(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1454usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom10_mem0_blk_cfg_w6(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1458usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom10_mem0_blk_cfg_w7(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_145cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom10_mem0_blk_cfg_w8(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1460usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom10_mem0_blk_cfg_w9(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1464usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom10_mem0_blk_cfg_w10(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1468usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom10_mem0_blk_cfg_w11(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_146cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom10_mem0_blk_cfg_w12(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1470usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom10_mem0_blk_cfg_w13(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1474usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom10_mem0_blk_cfg_w14(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1478usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom10_mem0_blk_cfg_w15(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_147cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom10_mem0_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1540usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom10_mem0_blk_nse_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1544usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom10_mem0_blk_nse_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1548usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom10_mem0_blk_nse_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_154cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom10_mem1_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1580usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom10_mem1_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_15a0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom10_mem2_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_15a8usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom10_mem2_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_15c8usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom10_mem3_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_15d0usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom10_mem3_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_15f0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom11_mem0_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1640usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom11_mem0_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1644usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom11_mem0_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1648usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom11_mem0_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_164cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom11_mem0_blk_cfg_w4(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1650usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom11_mem0_blk_cfg_w5(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1654usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom11_mem0_blk_cfg_w6(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1658usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom11_mem0_blk_cfg_w7(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_165cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom11_mem0_blk_cfg_w8(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1660usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom11_mem0_blk_cfg_w9(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1664usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom11_mem0_blk_cfg_w10(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1668usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom11_mem0_blk_cfg_w11(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_166cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom11_mem0_blk_cfg_w12(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1670usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom11_mem0_blk_cfg_w13(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1674usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom11_mem0_blk_cfg_w14(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1678usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom11_mem0_blk_cfg_w15(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_167cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom11_mem0_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1740usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom11_mem0_blk_nse_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1744usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom11_mem0_blk_nse_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1748usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom11_mem0_blk_nse_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_174cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom11_mem1_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1780usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom11_mem1_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_17a0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom11_mem2_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_17a8usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom11_mem2_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_17c8usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom11_mem3_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_17d0usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom11_mem3_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_17f0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom12_mem0_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1840usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom12_mem0_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1844usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom12_mem0_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1848usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom12_mem0_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_184cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom12_mem0_blk_cfg_w4(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1850usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom12_mem0_blk_cfg_w5(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1854usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom12_mem0_blk_cfg_w6(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1858usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom12_mem0_blk_cfg_w7(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_185cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom12_mem0_blk_cfg_w8(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1860usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom12_mem0_blk_cfg_w9(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1864usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom12_mem0_blk_cfg_w10(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1868usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom12_mem0_blk_cfg_w11(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_186cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom12_mem0_blk_cfg_w12(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1870usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom12_mem0_blk_cfg_w13(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1874usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom12_mem0_blk_cfg_w14(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1878usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom12_mem0_blk_cfg_w15(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_187cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom12_mem0_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1940usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom12_mem0_blk_nse_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1944usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom12_mem0_blk_nse_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1948usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom12_mem0_blk_nse_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_194cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom12_mem1_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1980usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom12_mem1_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_19a0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom12_mem2_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_19a8usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom12_mem2_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_19c8usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom12_mem3_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_19d0usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom12_mem3_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_19f0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom13_mem0_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1a40usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom13_mem0_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1a44usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom13_mem0_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1a48usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom13_mem0_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1a4cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom13_mem0_blk_cfg_w4(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1a50usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom13_mem0_blk_cfg_w5(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1a54usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom13_mem0_blk_cfg_w6(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1a58usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom13_mem0_blk_cfg_w7(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1a5cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom13_mem0_blk_cfg_w8(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1a60usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom13_mem0_blk_cfg_w9(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1a64usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom13_mem0_blk_cfg_w10(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1a68usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom13_mem0_blk_cfg_w11(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1a6cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom13_mem0_blk_cfg_w12(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1a70usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom13_mem0_blk_cfg_w13(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1a74usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom13_mem0_blk_cfg_w14(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1a78usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom13_mem0_blk_cfg_w15(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1a7cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom13_mem0_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1b40usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom13_mem0_blk_nse_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1b44usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom13_mem0_blk_nse_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1b48usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom13_mem0_blk_nse_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1b4cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom13_mem1_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1b80usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom13_mem1_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1ba0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom13_mem2_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1ba8usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom13_mem2_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1bc8usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom13_mem3_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1bd0usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom13_mem3_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1bf0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom14_mem0_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1c40usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom14_mem0_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1c44usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom14_mem0_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1c48usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom14_mem0_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1c4cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom14_mem0_blk_cfg_w4(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1c50usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom14_mem0_blk_cfg_w5(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1c54usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom14_mem0_blk_cfg_w6(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1c58usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom14_mem0_blk_cfg_w7(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1c5cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom14_mem0_blk_cfg_w8(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1c60usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom14_mem0_blk_cfg_w9(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1c64usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom14_mem0_blk_cfg_w10(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1c68usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom14_mem0_blk_cfg_w11(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1c6cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom14_mem0_blk_cfg_w12(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1c70usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom14_mem0_blk_cfg_w13(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1c74usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom14_mem0_blk_cfg_w14(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1c78usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom14_mem0_blk_cfg_w15(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1c7cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom14_mem0_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1d40usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom14_mem0_blk_nse_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1d44usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom14_mem0_blk_nse_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1d48usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom14_mem0_blk_nse_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1d4cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom14_mem1_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1d80usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom14_mem1_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1da0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom14_mem2_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1da8usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom14_mem2_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1dc8usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom14_mem3_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1dd0usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom14_mem3_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1df0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom15_mem0_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1e40usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom15_mem0_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1e44usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom15_mem0_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1e48usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom15_mem0_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1e4cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom15_mem0_blk_cfg_w4(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1e50usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom15_mem0_blk_cfg_w5(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1e54usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom15_mem0_blk_cfg_w6(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1e58usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom15_mem0_blk_cfg_w7(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1e5cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom15_mem0_blk_cfg_w8(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1e60usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom15_mem0_blk_cfg_w9(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1e64usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom15_mem0_blk_cfg_w10(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1e68usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom15_mem0_blk_cfg_w11(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1e6cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom15_mem0_blk_cfg_w12(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1e70usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom15_mem0_blk_cfg_w13(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1e74usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom15_mem0_blk_cfg_w14(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1e78usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom15_mem0_blk_cfg_w15(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1e7cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom15_mem0_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1f40usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom15_mem0_blk_nse_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1f44usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom15_mem0_blk_nse_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1f48usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom15_mem0_blk_nse_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1f4cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom15_mem1_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1f80usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom15_mem1_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1fa0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom15_mem2_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1fa8usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom15_mem2_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1fc8usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom15_mem3_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1fd0usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom15_mem3_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_1ff0usize) as _) }
    }
    #[doc = "MBC Global Configuration Register"]
    #[inline(always)]
    pub const fn mbc1_mem0_glbcfg(
        self,
    ) -> crate::common::Reg<super::trdc::regs::MbcGlbcfg, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2000usize) as _) }
    }
    #[doc = "MBC Global Configuration Register"]
    #[inline(always)]
    pub const fn mbc1_mem1_glbcfg(
        self,
    ) -> crate::common::Reg<super::trdc::regs::MbcGlbcfg, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2004usize) as _) }
    }
    #[doc = "MBC Global Configuration Register"]
    #[inline(always)]
    pub const fn mbc1_mem2_glbcfg(
        self,
    ) -> crate::common::Reg<super::trdc::regs::MbcGlbcfg, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2008usize) as _) }
    }
    #[doc = "MBC Global Configuration Register"]
    #[inline(always)]
    pub const fn mbc1_mem3_glbcfg(
        self,
    ) -> crate::common::Reg<super::trdc::regs::MbcGlbcfg, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_200cusize) as _) }
    }
    #[doc = "MBC NonSecure Enable Block Index"]
    #[inline(always)]
    pub const fn mbc1_nse_blk_index(
        self,
    ) -> crate::common::Reg<super::trdc::regs::NseBlkIndex, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2010usize) as _) }
    }
    #[doc = "MBC NonSecure Enable Block Set"]
    #[inline(always)]
    pub const fn mbc1_nse_blk_set(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2014usize) as _) }
    }
    #[doc = "MBC NonSecure Enable Block Clear"]
    #[inline(always)]
    pub const fn mbc1_nse_blk_clr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2018usize) as _) }
    }
    #[doc = "MBC NonSecure Enable Block Clear All"]
    #[inline(always)]
    pub const fn mbc1_nse_blk_clr_all(
        self,
    ) -> crate::common::Reg<super::trdc::regs::NseBlkClrAll, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_201cusize) as _) }
    }
    #[doc = "MBC Global Access Control"]
    #[inline(always)]
    pub const fn mbc1_memn_glbac0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Glbac0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2020usize) as _) }
    }
    #[doc = "MBC Global Access Control"]
    #[inline(always)]
    pub const fn mbc1_memn_glbac1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Glbac, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2024usize) as _) }
    }
    #[doc = "MBC Global Access Control"]
    #[inline(always)]
    pub const fn mbc1_memn_glbac2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Glbac, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2028usize) as _) }
    }
    #[doc = "MBC Global Access Control"]
    #[inline(always)]
    pub const fn mbc1_memn_glbac3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Glbac, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_202cusize) as _) }
    }
    #[doc = "MBC Global Access Control"]
    #[inline(always)]
    pub const fn mbc1_memn_glbac4(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Glbac, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2030usize) as _) }
    }
    #[doc = "MBC Global Access Control"]
    #[inline(always)]
    pub const fn mbc1_memn_glbac5(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Glbac, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2034usize) as _) }
    }
    #[doc = "MBC Global Access Control"]
    #[inline(always)]
    pub const fn mbc1_memn_glbac6(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Glbac, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2038usize) as _) }
    }
    #[doc = "MBC Global Access Control"]
    #[inline(always)]
    pub const fn mbc1_memn_glbac7(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Glbac, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_203cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom0_mem0_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2040usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom0_mem0_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2044usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom0_mem0_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2048usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom0_mem0_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_204cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc1_dom0_mem0_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2140usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom0_mem1_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2180usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom0_mem1_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2184usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom0_mem1_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2188usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom0_mem1_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_218cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc1_dom0_mem1_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_21a0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom1_mem0_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2240usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom1_mem0_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2244usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom1_mem0_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2248usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom1_mem0_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_224cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc1_dom1_mem0_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2340usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom1_mem1_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2380usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom1_mem1_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2384usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom1_mem1_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2388usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom1_mem1_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_238cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc1_dom1_mem1_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_23a0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom2_mem0_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2440usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom2_mem0_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2444usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom2_mem0_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2448usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom2_mem0_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_244cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc1_dom2_mem0_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2540usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom2_mem1_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2580usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom2_mem1_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2584usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom2_mem1_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2588usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom2_mem1_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_258cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc1_dom2_mem1_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_25a0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom3_mem0_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2640usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom3_mem0_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2644usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom3_mem0_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2648usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom3_mem0_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_264cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc1_dom3_mem0_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2740usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom3_mem1_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2780usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom3_mem1_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2784usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom3_mem1_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2788usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom3_mem1_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_278cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc1_dom3_mem1_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_27a0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom4_mem0_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2840usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom4_mem0_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2844usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom4_mem0_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2848usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom4_mem0_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_284cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc1_dom4_mem0_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2940usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom4_mem1_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2980usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom4_mem1_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2984usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom4_mem1_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2988usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom4_mem1_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_298cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc1_dom4_mem1_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_29a0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom5_mem0_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2a40usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom5_mem0_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2a44usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom5_mem0_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2a48usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom5_mem0_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2a4cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc1_dom5_mem0_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2b40usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom5_mem1_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2b80usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom5_mem1_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2b84usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom5_mem1_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2b88usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom5_mem1_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2b8cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc1_dom5_mem1_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2ba0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom6_mem0_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2c40usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom6_mem0_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2c44usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom6_mem0_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2c48usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom6_mem0_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2c4cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc1_dom6_mem0_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2d40usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom6_mem1_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2d80usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom6_mem1_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2d84usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom6_mem1_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2d88usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom6_mem1_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2d8cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc1_dom6_mem1_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2da0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom7_mem0_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2e40usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom7_mem0_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2e44usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom7_mem0_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2e48usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom7_mem0_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2e4cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc1_dom7_mem0_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2f40usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom7_mem1_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2f80usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom7_mem1_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2f84usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom7_mem1_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2f88usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom7_mem1_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2f8cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc1_dom7_mem1_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_2fa0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom8_mem0_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3040usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom8_mem0_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3044usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom8_mem0_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3048usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom8_mem0_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_304cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc1_dom8_mem0_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3140usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom8_mem1_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3180usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom8_mem1_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3184usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom8_mem1_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3188usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom8_mem1_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_318cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc1_dom8_mem1_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_31a0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom9_mem0_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3240usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom9_mem0_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3244usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom9_mem0_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3248usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom9_mem0_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_324cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc1_dom9_mem0_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3340usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom9_mem1_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3380usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom9_mem1_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3384usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom9_mem1_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3388usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom9_mem1_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_338cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc1_dom9_mem1_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_33a0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom10_mem0_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3440usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom10_mem0_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3444usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom10_mem0_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3448usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom10_mem0_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_344cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc1_dom10_mem0_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3540usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom10_mem1_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3580usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom10_mem1_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3584usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom10_mem1_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3588usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom10_mem1_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_358cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc1_dom10_mem1_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_35a0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom11_mem0_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3640usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom11_mem0_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3644usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom11_mem0_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3648usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom11_mem0_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_364cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc1_dom11_mem0_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3740usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom11_mem1_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3780usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom11_mem1_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3784usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom11_mem1_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3788usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom11_mem1_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_378cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc1_dom11_mem1_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_37a0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom12_mem0_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3840usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom12_mem0_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3844usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom12_mem0_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3848usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom12_mem0_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_384cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc1_dom12_mem0_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3940usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom12_mem1_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3980usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom12_mem1_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3984usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom12_mem1_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3988usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom12_mem1_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_398cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc1_dom12_mem1_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_39a0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom13_mem0_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3a40usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom13_mem0_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3a44usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom13_mem0_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3a48usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom13_mem0_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3a4cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc1_dom13_mem0_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3b40usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom13_mem1_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3b80usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom13_mem1_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3b84usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom13_mem1_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3b88usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom13_mem1_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3b8cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc1_dom13_mem1_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3ba0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom14_mem0_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3c40usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom14_mem0_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3c44usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom14_mem0_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3c48usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom14_mem0_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3c4cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc1_dom14_mem0_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3d40usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom14_mem1_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3d80usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom14_mem1_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3d84usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom14_mem1_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3d88usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom14_mem1_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3d8cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc1_dom14_mem1_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3da0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom15_mem0_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3e40usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom15_mem0_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3e44usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom15_mem0_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3e48usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom15_mem0_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3e4cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc1_dom15_mem0_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3f40usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom15_mem1_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3f80usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom15_mem1_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3f84usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom15_mem1_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3f88usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc1_dom15_mem1_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::BlkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3f8cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc1_dom15_mem1_blk_nse_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Nse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_3fa0usize) as _) }
    }
    #[doc = "MRC Global Configuration Register"]
    #[inline(always)]
    pub const fn mrc0_glbcfg(
        self,
    ) -> crate::common::Reg<super::trdc::regs::MrcGlbcfg, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4000usize) as _) }
    }
    #[doc = "MRC NonSecure Enable Region Indirect"]
    #[inline(always)]
    pub const fn mrc0_nse_rgn_indirect(
        self,
    ) -> crate::common::Reg<super::trdc::regs::NseRgnIndirect, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4010usize) as _) }
    }
    #[doc = "MRC NonSecure Enable Region Set"]
    #[inline(always)]
    pub const fn mrc0_nse_rgn_set(
        self,
    ) -> crate::common::Reg<super::trdc::regs::NseRgnSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4014usize) as _) }
    }
    #[doc = "MRC NonSecure Enable Region Clear"]
    #[inline(always)]
    pub const fn mrc0_nse_rgn_clr(
        self,
    ) -> crate::common::Reg<super::trdc::regs::NseRgnClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4018usize) as _) }
    }
    #[doc = "MRC NonSecure Enable Region Clear All"]
    #[inline(always)]
    pub const fn mrc0_nse_rgn_clr_all(
        self,
    ) -> crate::common::Reg<super::trdc::regs::NseRgnClrAll, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_401cusize) as _) }
    }
    #[doc = "MRC Global Access Control"]
    #[inline(always)]
    pub const fn mrc0_glbac0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Glbac0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4020usize) as _) }
    }
    #[doc = "MRC Global Access Control"]
    #[inline(always)]
    pub const fn mrc0_glbac1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Glbac, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4024usize) as _) }
    }
    #[doc = "MRC Global Access Control"]
    #[inline(always)]
    pub const fn mrc0_glbac2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Glbac, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4028usize) as _) }
    }
    #[doc = "MRC Global Access Control"]
    #[inline(always)]
    pub const fn mrc0_glbac3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Glbac, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_402cusize) as _) }
    }
    #[doc = "MRC Global Access Control"]
    #[inline(always)]
    pub const fn mrc0_glbac4(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Glbac, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4030usize) as _) }
    }
    #[doc = "MRC Global Access Control"]
    #[inline(always)]
    pub const fn mrc0_glbac5(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Glbac, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4034usize) as _) }
    }
    #[doc = "MRC Global Access Control"]
    #[inline(always)]
    pub const fn mrc0_glbac6(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Glbac, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4038usize) as _) }
    }
    #[doc = "MRC Global Access Control"]
    #[inline(always)]
    pub const fn mrc0_glbac7(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Glbac, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_403cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom0_rgd0_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4040usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom0_rgd0_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4044usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom0_rgd1_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4048usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom0_rgd1_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_404cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom0_rgd2_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4050usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom0_rgd2_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4054usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom0_rgd3_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4058usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom0_rgd3_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_405cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom0_rgd4_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4060usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom0_rgd4_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4064usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom0_rgd5_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4068usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom0_rgd5_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_406cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom0_rgd6_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4070usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom0_rgd6_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4074usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom0_rgd7_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4078usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom0_rgd7_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_407cusize) as _) }
    }
    #[doc = "MRC Region Descriptor NonSecure Enable"]
    #[inline(always)]
    pub const fn mrc0_dom0_rgd_nse(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdNse8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_40c0usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom1_rgd0_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4140usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom1_rgd0_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4144usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom1_rgd1_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4148usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom1_rgd1_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_414cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom1_rgd2_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4150usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom1_rgd2_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4154usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom1_rgd3_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4158usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom1_rgd3_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_415cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom1_rgd4_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4160usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom1_rgd4_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4164usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom1_rgd5_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4168usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom1_rgd5_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_416cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom1_rgd6_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4170usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom1_rgd6_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4174usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom1_rgd7_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4178usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom1_rgd7_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_417cusize) as _) }
    }
    #[doc = "MRC Region Descriptor NonSecure Enable"]
    #[inline(always)]
    pub const fn mrc0_dom1_rgd_nse(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdNse8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_41c0usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom2_rgd0_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4240usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom2_rgd0_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4244usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom2_rgd1_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4248usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom2_rgd1_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_424cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom2_rgd2_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4250usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom2_rgd2_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4254usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom2_rgd3_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4258usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom2_rgd3_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_425cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom2_rgd4_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4260usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom2_rgd4_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4264usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom2_rgd5_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4268usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom2_rgd5_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_426cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom2_rgd6_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4270usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom2_rgd6_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4274usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom2_rgd7_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4278usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom2_rgd7_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_427cusize) as _) }
    }
    #[doc = "MRC Region Descriptor NonSecure Enable"]
    #[inline(always)]
    pub const fn mrc0_dom2_rgd_nse(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdNse8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_42c0usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom3_rgd0_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4340usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom3_rgd0_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4344usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom3_rgd1_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4348usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom3_rgd1_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_434cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom3_rgd2_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4350usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom3_rgd2_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4354usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom3_rgd3_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4358usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom3_rgd3_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_435cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom3_rgd4_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4360usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom3_rgd4_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4364usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom3_rgd5_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4368usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom3_rgd5_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_436cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom3_rgd6_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4370usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom3_rgd6_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4374usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom3_rgd7_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4378usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom3_rgd7_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_437cusize) as _) }
    }
    #[doc = "MRC Region Descriptor NonSecure Enable"]
    #[inline(always)]
    pub const fn mrc0_dom3_rgd_nse(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdNse8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_43c0usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom4_rgd0_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4440usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom4_rgd0_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4444usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom4_rgd1_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4448usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom4_rgd1_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_444cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom4_rgd2_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4450usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom4_rgd2_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4454usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom4_rgd3_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4458usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom4_rgd3_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_445cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom4_rgd4_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4460usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom4_rgd4_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4464usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom4_rgd5_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4468usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom4_rgd5_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_446cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom4_rgd6_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4470usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom4_rgd6_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4474usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom4_rgd7_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4478usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom4_rgd7_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_447cusize) as _) }
    }
    #[doc = "MRC Region Descriptor NonSecure Enable"]
    #[inline(always)]
    pub const fn mrc0_dom4_rgd_nse(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdNse8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_44c0usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom5_rgd0_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4540usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom5_rgd0_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4544usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom5_rgd1_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4548usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom5_rgd1_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_454cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom5_rgd2_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4550usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom5_rgd2_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4554usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom5_rgd3_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4558usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom5_rgd3_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_455cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom5_rgd4_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4560usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom5_rgd4_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4564usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom5_rgd5_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4568usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom5_rgd5_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_456cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom5_rgd6_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4570usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom5_rgd6_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4574usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom5_rgd7_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4578usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom5_rgd7_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_457cusize) as _) }
    }
    #[doc = "MRC Region Descriptor NonSecure Enable"]
    #[inline(always)]
    pub const fn mrc0_dom5_rgd_nse(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdNse8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_45c0usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom6_rgd0_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4640usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom6_rgd0_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4644usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom6_rgd1_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4648usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom6_rgd1_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_464cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom6_rgd2_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4650usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom6_rgd2_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4654usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom6_rgd3_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4658usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom6_rgd3_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_465cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom6_rgd4_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4660usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom6_rgd4_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4664usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom6_rgd5_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4668usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom6_rgd5_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_466cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom6_rgd6_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4670usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom6_rgd6_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4674usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom6_rgd7_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4678usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom6_rgd7_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_467cusize) as _) }
    }
    #[doc = "MRC Region Descriptor NonSecure Enable"]
    #[inline(always)]
    pub const fn mrc0_dom6_rgd_nse(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdNse8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_46c0usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom7_rgd0_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4740usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom7_rgd0_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4744usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom7_rgd1_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4748usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom7_rgd1_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_474cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom7_rgd2_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4750usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom7_rgd2_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4754usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom7_rgd3_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4758usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom7_rgd3_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_475cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom7_rgd4_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4760usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom7_rgd4_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4764usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom7_rgd5_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4768usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom7_rgd5_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_476cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom7_rgd6_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4770usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom7_rgd6_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4774usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom7_rgd7_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4778usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom7_rgd7_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_477cusize) as _) }
    }
    #[doc = "MRC Region Descriptor NonSecure Enable"]
    #[inline(always)]
    pub const fn mrc0_dom7_rgd_nse(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdNse8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_47c0usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom8_rgd0_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4840usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom8_rgd0_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4844usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom8_rgd1_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4848usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom8_rgd1_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_484cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom8_rgd2_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4850usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom8_rgd2_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4854usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom8_rgd3_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4858usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom8_rgd3_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_485cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom8_rgd4_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4860usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom8_rgd4_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4864usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom8_rgd5_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4868usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom8_rgd5_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_486cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom8_rgd6_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4870usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom8_rgd6_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4874usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom8_rgd7_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4878usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom8_rgd7_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_487cusize) as _) }
    }
    #[doc = "MRC Region Descriptor NonSecure Enable"]
    #[inline(always)]
    pub const fn mrc0_dom8_rgd_nse(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdNse8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_48c0usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom9_rgd0_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4940usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom9_rgd0_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4944usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom9_rgd1_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4948usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom9_rgd1_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_494cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom9_rgd2_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4950usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom9_rgd2_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4954usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom9_rgd3_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4958usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom9_rgd3_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_495cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom9_rgd4_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4960usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom9_rgd4_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4964usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom9_rgd5_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4968usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom9_rgd5_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_496cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom9_rgd6_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4970usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom9_rgd6_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4974usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom9_rgd7_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4978usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom9_rgd7_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_497cusize) as _) }
    }
    #[doc = "MRC Region Descriptor NonSecure Enable"]
    #[inline(always)]
    pub const fn mrc0_dom9_rgd_nse(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdNse8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_49c0usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom10_rgd0_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4a40usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom10_rgd0_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4a44usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom10_rgd1_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4a48usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom10_rgd1_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4a4cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom10_rgd2_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4a50usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom10_rgd2_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4a54usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom10_rgd3_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4a58usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom10_rgd3_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4a5cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom10_rgd4_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4a60usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom10_rgd4_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4a64usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom10_rgd5_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4a68usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom10_rgd5_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4a6cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom10_rgd6_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4a70usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom10_rgd6_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4a74usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom10_rgd7_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4a78usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom10_rgd7_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4a7cusize) as _) }
    }
    #[doc = "MRC Region Descriptor NonSecure Enable"]
    #[inline(always)]
    pub const fn mrc0_dom10_rgd_nse(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdNse8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4ac0usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom11_rgd0_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4b40usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom11_rgd0_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4b44usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom11_rgd1_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4b48usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom11_rgd1_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4b4cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom11_rgd2_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4b50usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom11_rgd2_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4b54usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom11_rgd3_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4b58usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom11_rgd3_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4b5cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom11_rgd4_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4b60usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom11_rgd4_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4b64usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom11_rgd5_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4b68usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom11_rgd5_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4b6cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom11_rgd6_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4b70usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom11_rgd6_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4b74usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom11_rgd7_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4b78usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom11_rgd7_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4b7cusize) as _) }
    }
    #[doc = "MRC Region Descriptor NonSecure Enable"]
    #[inline(always)]
    pub const fn mrc0_dom11_rgd_nse(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdNse8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4bc0usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom12_rgd0_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4c40usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom12_rgd0_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4c44usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom12_rgd1_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4c48usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom12_rgd1_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4c4cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom12_rgd2_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4c50usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom12_rgd2_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4c54usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom12_rgd3_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4c58usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom12_rgd3_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4c5cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom12_rgd4_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4c60usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom12_rgd4_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4c64usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom12_rgd5_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4c68usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom12_rgd5_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4c6cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom12_rgd6_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4c70usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom12_rgd6_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4c74usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom12_rgd7_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4c78usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom12_rgd7_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4c7cusize) as _) }
    }
    #[doc = "MRC Region Descriptor NonSecure Enable"]
    #[inline(always)]
    pub const fn mrc0_dom12_rgd_nse(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdNse8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4cc0usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom13_rgd0_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4d40usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom13_rgd0_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4d44usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom13_rgd1_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4d48usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom13_rgd1_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4d4cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom13_rgd2_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4d50usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom13_rgd2_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4d54usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom13_rgd3_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4d58usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom13_rgd3_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4d5cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom13_rgd4_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4d60usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom13_rgd4_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4d64usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom13_rgd5_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4d68usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom13_rgd5_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4d6cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom13_rgd6_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4d70usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom13_rgd6_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4d74usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom13_rgd7_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4d78usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom13_rgd7_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4d7cusize) as _) }
    }
    #[doc = "MRC Region Descriptor NonSecure Enable"]
    #[inline(always)]
    pub const fn mrc0_dom13_rgd_nse(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdNse8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4dc0usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom14_rgd0_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4e40usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom14_rgd0_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4e44usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom14_rgd1_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4e48usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom14_rgd1_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4e4cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom14_rgd2_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4e50usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom14_rgd2_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4e54usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom14_rgd3_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4e58usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom14_rgd3_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4e5cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom14_rgd4_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4e60usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom14_rgd4_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4e64usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom14_rgd5_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4e68usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom14_rgd5_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4e6cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom14_rgd6_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4e70usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom14_rgd6_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4e74usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom14_rgd7_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4e78usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom14_rgd7_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4e7cusize) as _) }
    }
    #[doc = "MRC Region Descriptor NonSecure Enable"]
    #[inline(always)]
    pub const fn mrc0_dom14_rgd_nse(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdNse8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4ec0usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom15_rgd0_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4f40usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom15_rgd0_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4f44usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom15_rgd1_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4f48usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom15_rgd1_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4f4cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom15_rgd2_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4f50usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom15_rgd2_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4f54usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom15_rgd3_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4f58usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom15_rgd3_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4f5cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom15_rgd4_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4f60usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom15_rgd4_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4f64usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom15_rgd5_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4f68usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom15_rgd5_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4f6cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom15_rgd6_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4f70usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom15_rgd6_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4f74usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc0_dom15_rgd7_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4f78usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc0_dom15_rgd7_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4f7cusize) as _) }
    }
    #[doc = "MRC Region Descriptor NonSecure Enable"]
    #[inline(always)]
    pub const fn mrc0_dom15_rgd_nse(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdNse8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_4fc0usize) as _) }
    }
    #[doc = "MRC Global Configuration Register"]
    #[inline(always)]
    pub const fn mrc1_glbcfg(
        self,
    ) -> crate::common::Reg<super::trdc::regs::MrcGlbcfg, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5000usize) as _) }
    }
    #[doc = "MRC NonSecure Enable Region Indirect"]
    #[inline(always)]
    pub const fn mrc1_nse_rgn_indirect(
        self,
    ) -> crate::common::Reg<super::trdc::regs::NseRgnIndirect, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5010usize) as _) }
    }
    #[doc = "MRC NonSecure Enable Region Set"]
    #[inline(always)]
    pub const fn mrc1_nse_rgn_set(
        self,
    ) -> crate::common::Reg<super::trdc::regs::NseRgnSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5014usize) as _) }
    }
    #[doc = "MRC NonSecure Enable Region Clear"]
    #[inline(always)]
    pub const fn mrc1_nse_rgn_clr(
        self,
    ) -> crate::common::Reg<super::trdc::regs::NseRgnClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5018usize) as _) }
    }
    #[doc = "MRC NonSecure Enable Region Clear All"]
    #[inline(always)]
    pub const fn mrc1_nse_rgn_clr_all(
        self,
    ) -> crate::common::Reg<super::trdc::regs::NseRgnClrAll, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_501cusize) as _) }
    }
    #[doc = "MRC Global Access Control"]
    #[inline(always)]
    pub const fn mrc1_glbac0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Glbac0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5020usize) as _) }
    }
    #[doc = "MRC Global Access Control"]
    #[inline(always)]
    pub const fn mrc1_glbac1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Glbac, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5024usize) as _) }
    }
    #[doc = "MRC Global Access Control"]
    #[inline(always)]
    pub const fn mrc1_glbac2(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Glbac, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5028usize) as _) }
    }
    #[doc = "MRC Global Access Control"]
    #[inline(always)]
    pub const fn mrc1_glbac3(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Glbac, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_502cusize) as _) }
    }
    #[doc = "MRC Global Access Control"]
    #[inline(always)]
    pub const fn mrc1_glbac4(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Glbac, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5030usize) as _) }
    }
    #[doc = "MRC Global Access Control"]
    #[inline(always)]
    pub const fn mrc1_glbac5(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Glbac, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5034usize) as _) }
    }
    #[doc = "MRC Global Access Control"]
    #[inline(always)]
    pub const fn mrc1_glbac6(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Glbac, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5038usize) as _) }
    }
    #[doc = "MRC Global Access Control"]
    #[inline(always)]
    pub const fn mrc1_glbac7(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Glbac, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_503cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom0_rgd0_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5040usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom0_rgd0_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5044usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom0_rgd1_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5048usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom0_rgd1_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_504cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom0_rgd2_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5050usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom0_rgd2_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5054usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom0_rgd3_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5058usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom0_rgd3_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_505cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom0_rgd4_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5060usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom0_rgd4_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5064usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom0_rgd5_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5068usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom0_rgd5_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_506cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom0_rgd6_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5070usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom0_rgd6_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5074usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom0_rgd7_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5078usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom0_rgd7_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_507cusize) as _) }
    }
    #[doc = "MRC Region Descriptor NonSecure Enable"]
    #[inline(always)]
    pub const fn mrc1_dom0_rgd_nse(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdNse8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_50c0usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom1_rgd0_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5140usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom1_rgd0_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5144usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom1_rgd1_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5148usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom1_rgd1_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_514cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom1_rgd2_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5150usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom1_rgd2_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5154usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom1_rgd3_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5158usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom1_rgd3_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_515cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom1_rgd4_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5160usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom1_rgd4_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5164usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom1_rgd5_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5168usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom1_rgd5_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_516cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom1_rgd6_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5170usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom1_rgd6_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5174usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom1_rgd7_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5178usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom1_rgd7_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_517cusize) as _) }
    }
    #[doc = "MRC Region Descriptor NonSecure Enable"]
    #[inline(always)]
    pub const fn mrc1_dom1_rgd_nse(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdNse8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_51c0usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom2_rgd0_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5240usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom2_rgd0_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5244usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom2_rgd1_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5248usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom2_rgd1_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_524cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom2_rgd2_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5250usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom2_rgd2_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5254usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom2_rgd3_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5258usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom2_rgd3_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_525cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom2_rgd4_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5260usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom2_rgd4_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5264usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom2_rgd5_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5268usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom2_rgd5_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_526cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom2_rgd6_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5270usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom2_rgd6_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5274usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom2_rgd7_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5278usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom2_rgd7_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_527cusize) as _) }
    }
    #[doc = "MRC Region Descriptor NonSecure Enable"]
    #[inline(always)]
    pub const fn mrc1_dom2_rgd_nse(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdNse8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_52c0usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom3_rgd0_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5340usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom3_rgd0_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5344usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom3_rgd1_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5348usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom3_rgd1_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_534cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom3_rgd2_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5350usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom3_rgd2_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5354usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom3_rgd3_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5358usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom3_rgd3_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_535cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom3_rgd4_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5360usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom3_rgd4_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5364usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom3_rgd5_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5368usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom3_rgd5_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_536cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom3_rgd6_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5370usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom3_rgd6_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5374usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom3_rgd7_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5378usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom3_rgd7_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_537cusize) as _) }
    }
    #[doc = "MRC Region Descriptor NonSecure Enable"]
    #[inline(always)]
    pub const fn mrc1_dom3_rgd_nse(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdNse8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_53c0usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom4_rgd0_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5440usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom4_rgd0_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5444usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom4_rgd1_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5448usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom4_rgd1_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_544cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom4_rgd2_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5450usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom4_rgd2_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5454usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom4_rgd3_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5458usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom4_rgd3_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_545cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom4_rgd4_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5460usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom4_rgd4_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5464usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom4_rgd5_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5468usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom4_rgd5_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_546cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom4_rgd6_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5470usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom4_rgd6_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5474usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom4_rgd7_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5478usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom4_rgd7_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_547cusize) as _) }
    }
    #[doc = "MRC Region Descriptor NonSecure Enable"]
    #[inline(always)]
    pub const fn mrc1_dom4_rgd_nse(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdNse8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_54c0usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom5_rgd0_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5540usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom5_rgd0_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5544usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom5_rgd1_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5548usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom5_rgd1_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_554cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom5_rgd2_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5550usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom5_rgd2_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5554usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom5_rgd3_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5558usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom5_rgd3_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_555cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom5_rgd4_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5560usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom5_rgd4_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5564usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom5_rgd5_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5568usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom5_rgd5_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_556cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom5_rgd6_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5570usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom5_rgd6_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5574usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom5_rgd7_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5578usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom5_rgd7_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_557cusize) as _) }
    }
    #[doc = "MRC Region Descriptor NonSecure Enable"]
    #[inline(always)]
    pub const fn mrc1_dom5_rgd_nse(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdNse8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_55c0usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom6_rgd0_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5640usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom6_rgd0_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5644usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom6_rgd1_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5648usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom6_rgd1_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_564cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom6_rgd2_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5650usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom6_rgd2_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5654usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom6_rgd3_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5658usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom6_rgd3_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_565cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom6_rgd4_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5660usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom6_rgd4_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5664usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom6_rgd5_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5668usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom6_rgd5_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_566cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom6_rgd6_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5670usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom6_rgd6_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5674usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom6_rgd7_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5678usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom6_rgd7_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_567cusize) as _) }
    }
    #[doc = "MRC Region Descriptor NonSecure Enable"]
    #[inline(always)]
    pub const fn mrc1_dom6_rgd_nse(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdNse8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_56c0usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom7_rgd0_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5740usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom7_rgd0_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5744usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom7_rgd1_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5748usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom7_rgd1_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_574cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom7_rgd2_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5750usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom7_rgd2_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5754usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom7_rgd3_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5758usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom7_rgd3_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_575cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom7_rgd4_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5760usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom7_rgd4_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5764usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom7_rgd5_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5768usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom7_rgd5_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_576cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom7_rgd6_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5770usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom7_rgd6_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5774usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom7_rgd7_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5778usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom7_rgd7_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_577cusize) as _) }
    }
    #[doc = "MRC Region Descriptor NonSecure Enable"]
    #[inline(always)]
    pub const fn mrc1_dom7_rgd_nse(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdNse8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_57c0usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom8_rgd0_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5840usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom8_rgd0_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5844usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom8_rgd1_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5848usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom8_rgd1_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_584cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom8_rgd2_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5850usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom8_rgd2_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5854usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom8_rgd3_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5858usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom8_rgd3_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_585cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom8_rgd4_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5860usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom8_rgd4_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5864usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom8_rgd5_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5868usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom8_rgd5_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_586cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom8_rgd6_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5870usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom8_rgd6_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5874usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom8_rgd7_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5878usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom8_rgd7_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_587cusize) as _) }
    }
    #[doc = "MRC Region Descriptor NonSecure Enable"]
    #[inline(always)]
    pub const fn mrc1_dom8_rgd_nse(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdNse8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_58c0usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom9_rgd0_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5940usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom9_rgd0_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5944usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom9_rgd1_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5948usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom9_rgd1_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_594cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom9_rgd2_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5950usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom9_rgd2_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5954usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom9_rgd3_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5958usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom9_rgd3_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_595cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom9_rgd4_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5960usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom9_rgd4_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5964usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom9_rgd5_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5968usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom9_rgd5_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_596cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom9_rgd6_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5970usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom9_rgd6_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5974usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom9_rgd7_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5978usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom9_rgd7_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_597cusize) as _) }
    }
    #[doc = "MRC Region Descriptor NonSecure Enable"]
    #[inline(always)]
    pub const fn mrc1_dom9_rgd_nse(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdNse8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_59c0usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom10_rgd0_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5a40usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom10_rgd0_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5a44usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom10_rgd1_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5a48usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom10_rgd1_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5a4cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom10_rgd2_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5a50usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom10_rgd2_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5a54usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom10_rgd3_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5a58usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom10_rgd3_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5a5cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom10_rgd4_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5a60usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom10_rgd4_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5a64usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom10_rgd5_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5a68usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom10_rgd5_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5a6cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom10_rgd6_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5a70usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom10_rgd6_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5a74usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom10_rgd7_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5a78usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom10_rgd7_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5a7cusize) as _) }
    }
    #[doc = "MRC Region Descriptor NonSecure Enable"]
    #[inline(always)]
    pub const fn mrc1_dom10_rgd_nse(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdNse8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5ac0usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom11_rgd0_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5b40usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom11_rgd0_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5b44usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom11_rgd1_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5b48usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom11_rgd1_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5b4cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom11_rgd2_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5b50usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom11_rgd2_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5b54usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom11_rgd3_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5b58usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom11_rgd3_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5b5cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom11_rgd4_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5b60usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom11_rgd4_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5b64usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom11_rgd5_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5b68usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom11_rgd5_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5b6cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom11_rgd6_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5b70usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom11_rgd6_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5b74usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom11_rgd7_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5b78usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom11_rgd7_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5b7cusize) as _) }
    }
    #[doc = "MRC Region Descriptor NonSecure Enable"]
    #[inline(always)]
    pub const fn mrc1_dom11_rgd_nse(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdNse8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5bc0usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom12_rgd0_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5c40usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom12_rgd0_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5c44usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom12_rgd1_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5c48usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom12_rgd1_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5c4cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom12_rgd2_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5c50usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom12_rgd2_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5c54usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom12_rgd3_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5c58usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom12_rgd3_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5c5cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom12_rgd4_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5c60usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom12_rgd4_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5c64usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom12_rgd5_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5c68usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom12_rgd5_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5c6cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom12_rgd6_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5c70usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom12_rgd6_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5c74usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom12_rgd7_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5c78usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom12_rgd7_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5c7cusize) as _) }
    }
    #[doc = "MRC Region Descriptor NonSecure Enable"]
    #[inline(always)]
    pub const fn mrc1_dom12_rgd_nse(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdNse8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5cc0usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom13_rgd0_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5d40usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom13_rgd0_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5d44usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom13_rgd1_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5d48usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom13_rgd1_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5d4cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom13_rgd2_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5d50usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom13_rgd2_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5d54usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom13_rgd3_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5d58usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom13_rgd3_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5d5cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom13_rgd4_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5d60usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom13_rgd4_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5d64usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom13_rgd5_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5d68usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom13_rgd5_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5d6cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom13_rgd6_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5d70usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom13_rgd6_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5d74usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom13_rgd7_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5d78usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom13_rgd7_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5d7cusize) as _) }
    }
    #[doc = "MRC Region Descriptor NonSecure Enable"]
    #[inline(always)]
    pub const fn mrc1_dom13_rgd_nse(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdNse8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5dc0usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom14_rgd0_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5e40usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom14_rgd0_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5e44usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom14_rgd1_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5e48usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom14_rgd1_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5e4cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom14_rgd2_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5e50usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom14_rgd2_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5e54usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom14_rgd3_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5e58usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom14_rgd3_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5e5cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom14_rgd4_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5e60usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom14_rgd4_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5e64usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom14_rgd5_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5e68usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom14_rgd5_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5e6cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom14_rgd6_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5e70usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom14_rgd6_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5e74usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom14_rgd7_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5e78usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom14_rgd7_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5e7cusize) as _) }
    }
    #[doc = "MRC Region Descriptor NonSecure Enable"]
    #[inline(always)]
    pub const fn mrc1_dom14_rgd_nse(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdNse8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5ec0usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom15_rgd0_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5f40usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom15_rgd0_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5f44usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom15_rgd1_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5f48usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom15_rgd1_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5f4cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom15_rgd2_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5f50usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom15_rgd2_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5f54usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom15_rgd3_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5f58usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom15_rgd3_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5f5cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom15_rgd4_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5f60usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom15_rgd4_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5f64usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom15_rgd5_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5f68usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom15_rgd5_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5f6cusize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom15_rgd6_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5f70usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom15_rgd6_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5f74usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 0"]
    #[inline(always)]
    pub const fn mrc1_dom15_rgd7_w0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5f78usize) as _) }
    }
    #[doc = "MRC Region Descriptor Word 1"]
    #[inline(always)]
    pub const fn mrc1_dom15_rgd7_w1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5f7cusize) as _) }
    }
    #[doc = "MRC Region Descriptor NonSecure Enable"]
    #[inline(always)]
    pub const fn mrc1_dom15_rgd_nse(
        self,
    ) -> crate::common::Reg<super::trdc::regs::RgdNse8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0001_5fc0usize) as _) }
    }
}
