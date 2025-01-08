#[doc = "NETC EMDIO base function"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmdioBase {
    ptr: *mut u8,
}
unsafe impl Send for EmdioBase {}
unsafe impl Sync for EmdioBase {}
impl EmdioBase {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "External MDIO configuration register"]
    #[inline(always)]
    pub const fn emdio_cfg(self) -> crate::common::Reg<regs::EmdioCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1c00usize) as _) }
    }
    #[doc = "External MDIO interface control register"]
    #[inline(always)]
    pub const fn emdio_ctl(self) -> crate::common::Reg<regs::EmdioCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1c04usize) as _) }
    }
    #[doc = "External MDIO interface data register"]
    #[inline(always)]
    pub const fn emdio_data(self) -> crate::common::Reg<regs::EmdioData, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1c08usize) as _) }
    }
    #[doc = "External MDIO register address register"]
    #[inline(always)]
    pub const fn emdio_addr(self) -> crate::common::Reg<regs::EmdioAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1c0cusize) as _) }
    }
    #[doc = "External MDIO status register"]
    #[inline(always)]
    pub const fn emdio_stat(self) -> crate::common::Reg<regs::EmdioStat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1c10usize) as _) }
    }
    #[doc = "PHY status configuration register"]
    #[inline(always)]
    pub const fn phy_status_cfg(self) -> crate::common::Reg<regs::PhyStatusCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1c20usize) as _) }
    }
    #[doc = "PHY status control register"]
    #[inline(always)]
    pub const fn phy_status_ctl(self) -> crate::common::Reg<regs::PhyStatusCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1c24usize) as _) }
    }
    #[doc = "PHY status data register"]
    #[inline(always)]
    pub const fn phy_status_data(
        self,
    ) -> crate::common::Reg<regs::PhyStatusData, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1c28usize) as _) }
    }
    #[doc = "PHY status register address register"]
    #[inline(always)]
    pub const fn phy_status_addr(
        self,
    ) -> crate::common::Reg<regs::PhyStatusAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1c2cusize) as _) }
    }
    #[doc = "PHY status event register"]
    #[inline(always)]
    pub const fn phy_status_event(
        self,
    ) -> crate::common::Reg<regs::PhyStatusEvent, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1c30usize) as _) }
    }
    #[doc = "PHY status mask register"]
    #[inline(always)]
    pub const fn phy_status_mask(
        self,
    ) -> crate::common::Reg<regs::PhyStatusMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1c34usize) as _) }
    }
    #[doc = "MDIO configuration register"]
    #[inline(always)]
    pub const fn mdio_cfg(self) -> crate::common::Reg<regs::MdioCfg, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1c40usize) as _) }
    }
}
pub mod regs;
pub mod vals;
