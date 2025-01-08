#[doc = "Block Control Secure AONMIX"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BlkCtrlSAonmix {
    ptr: *mut u8,
}
unsafe impl Send for BlkCtrlSAonmix {}
unsafe impl Sync for BlkCtrlSAonmix {}
impl BlkCtrlSAonmix {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "CM33_IRQ_MASK0"]
    #[inline(always)]
    pub const fn cm33_irq_mask0(self) -> crate::common::Reg<regs::Cm33IrqMask0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "CM33 IRQ MASK1"]
    #[inline(always)]
    pub const fn cm33_irq_mask1(self) -> crate::common::Reg<regs::Cm33IrqMask1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "CM33_IRQ_MASK2"]
    #[inline(always)]
    pub const fn cm33_irq_mask2(self) -> crate::common::Reg<regs::Cm33IrqMask2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "CM33_IRQ_MASK3"]
    #[inline(always)]
    pub const fn cm33_irq_mask3(self) -> crate::common::Reg<regs::Cm33IrqMask3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "CM33_IRQ_MASK4"]
    #[inline(always)]
    pub const fn cm33_irq_mask4(self) -> crate::common::Reg<regs::Cm33IrqMask4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "CM33_IRQ_MASK5"]
    #[inline(always)]
    pub const fn cm33_irq_mask5(self) -> crate::common::Reg<regs::Cm33IrqMask5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "CM33_IRQ_MASK6"]
    #[inline(always)]
    pub const fn cm33_irq_mask6(self) -> crate::common::Reg<regs::Cm33IrqMask6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "CM33_IRQ_MASK7"]
    #[inline(always)]
    pub const fn cm33_irq_mask7(self) -> crate::common::Reg<regs::Cm33IrqMask7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "CM7_IRQ_MASK0"]
    #[inline(always)]
    pub const fn cm7_irq_mask0(self) -> crate::common::Reg<regs::Cm7IrqMask0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "CM7_IRQ_MASK1"]
    #[inline(always)]
    pub const fn cm7_irq_mask1(self) -> crate::common::Reg<regs::Cm7IrqMask1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "CM7_IRQ_MASK2"]
    #[inline(always)]
    pub const fn cm7_irq_mask2(self) -> crate::common::Reg<regs::Cm7IrqMask2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "CM7_IRQ_MASK3"]
    #[inline(always)]
    pub const fn cm7_irq_mask3(self) -> crate::common::Reg<regs::Cm7IrqMask3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "CM7_IRQ_MASK4"]
    #[inline(always)]
    pub const fn cm7_irq_mask4(self) -> crate::common::Reg<regs::Cm7IrqMask4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "CM7_IRQ_MASK5"]
    #[inline(always)]
    pub const fn cm7_irq_mask5(self) -> crate::common::Reg<regs::Cm7IrqMask5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "CM7_IRQ_MASK6"]
    #[inline(always)]
    pub const fn cm7_irq_mask6(self) -> crate::common::Reg<regs::Cm7IrqMask6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "CM7_IRQ_MASK7"]
    #[inline(always)]
    pub const fn cm7_irq_mask7(self) -> crate::common::Reg<regs::Cm7IrqMask7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "EdgeLock reset request mask"]
    #[inline(always)]
    pub const fn edgelock_reset_req_mask(
        self,
    ) -> crate::common::Reg<regs::EdgelockResetReqMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "EdgeLock IRQ request mask"]
    #[inline(always)]
    pub const fn edgelock_irq_mask(
        self,
    ) -> crate::common::Reg<regs::EdgelockIrqMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "M33 Configuration"]
    #[inline(always)]
    pub const fn m33_cfg(self) -> crate::common::Reg<regs::M33Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "M33 INITSVTOR"]
    #[inline(always)]
    pub const fn m33_initsvtor(self) -> crate::common::Reg<regs::M33Initsvtor, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "M33 INITNSVTOR"]
    #[inline(always)]
    pub const fn m33_initnsvtor(
        self,
    ) -> crate::common::Reg<regs::M33Initnsvtor, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "M7 Configuration"]
    #[inline(always)]
    pub const fn m7_cfg(self) -> crate::common::Reg<regs::M7Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "AXBS_AON_CTRL"]
    #[inline(always)]
    pub const fn axbs_aon_ctrl(self) -> crate::common::Reg<regs::AxbsAonCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "DAP Access Sticky Bit"]
    #[inline(always)]
    pub const fn dap_access_stkybit(
        self,
    ) -> crate::common::Reg<regs::DapAccessStkybit, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "Low power handshake enable"]
    #[inline(always)]
    pub const fn lp_handshake(self) -> crate::common::Reg<regs::LpHandshake, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "EdgeLock halt status"]
    #[inline(always)]
    pub const fn edgelock_halt_st(
        self,
    ) -> crate::common::Reg<regs::EdgelockHaltSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
    #[doc = "ECC memory hardware initialization"]
    #[inline(always)]
    pub const fn ecc_mem_init(self) -> crate::common::Reg<regs::EccMemInit, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
    #[doc = "IOMUXC domain configure"]
    #[inline(always)]
    pub const fn iomuxc_domain_cfg(
        self,
    ) -> crate::common::Reg<regs::IomuxcDomainCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0148usize) as _) }
    }
    #[doc = "IOMUXC_AON domain configure"]
    #[inline(always)]
    pub const fn iomuxc_aon_domain_cfg(
        self,
    ) -> crate::common::Reg<regs::IomuxcAonDomainCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x014cusize) as _) }
    }
    #[doc = "NMI control"]
    #[inline(always)]
    pub const fn nmi_ctrl(self) -> crate::common::Reg<regs::NmiCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0154usize) as _) }
    }
    #[doc = "s401_ipi_noclk_ref1 clear control"]
    #[inline(always)]
    pub const fn s401_noclk_clear_ctrl(
        self,
    ) -> crate::common::Reg<regs::S401NoclkClearCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0158usize) as _) }
    }
}
pub mod regs;
pub mod vals;
