#[doc = "REVMII MDIO"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enetc0RevmiiMac {
    ptr: *mut u8,
}
unsafe impl Send for Enetc0RevmiiMac {}
unsafe impl Sync for Enetc0RevmiiMac {}
impl Enetc0RevmiiMac {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control register"]
    #[inline(always)]
    pub const fn control(self) -> crate::common::Reg<regs::Control, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Status register"]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01usize) as _) }
    }
    #[doc = "PHY identifier upper register"]
    #[inline(always)]
    pub const fn phy_id_u(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02usize) as _) }
    }
    #[doc = "PHY identifier lower register"]
    #[inline(always)]
    pub const fn phy_id_l(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03usize) as _) }
    }
    #[doc = "AN Advertisement"]
    #[inline(always)]
    pub const fn an_adv(self) -> crate::common::Reg<regs::AnAdv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "AN link partner ability"]
    #[inline(always)]
    pub const fn an_lp_abil(self) -> crate::common::Reg<regs::AnLpAbil, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05usize) as _) }
    }
    #[doc = "AN Expansion"]
    #[inline(always)]
    pub const fn an_exp(self) -> crate::common::Reg<regs::AnExp, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06usize) as _) }
    }
    #[doc = "AN Next Page transmit register"]
    #[inline(always)]
    pub const fn an_next_tx(self) -> crate::common::Reg<regs::AnNextTx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07usize) as _) }
    }
    #[doc = "AN Link Partner Received Next Page"]
    #[inline(always)]
    pub const fn an_lp_rx_next_pg(
        self,
    ) -> crate::common::Reg<regs::AnLpRxNextPg, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "MASTER-SLAVE Control Register"]
    #[inline(always)]
    pub const fn ms_ctl(self) -> crate::common::Reg<regs::MsCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x09usize) as _) }
    }
    #[doc = "MASTER-SLAVE Status Register"]
    #[inline(always)]
    pub const fn ms_sta(self) -> crate::common::Reg<regs::MsSta, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ausize) as _) }
    }
    #[doc = "Extended Status"]
    #[inline(always)]
    pub const fn xtnd_stat(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fusize) as _) }
    }
    #[doc = "Scratch register"]
    #[inline(always)]
    pub const fn scratch(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
}
pub mod regs;
