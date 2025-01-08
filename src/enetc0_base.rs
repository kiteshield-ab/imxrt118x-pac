#[doc = "ENETC base"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enetc0Base {
    ptr: *mut u8,
}
unsafe impl Send for Enetc0Base {}
unsafe impl Sync for Enetc0Base {}
impl Enetc0Base {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "ENETC capability register 0"]
    #[inline(always)]
    pub const fn ecapr0(self) -> crate::common::Reg<regs::Ecapr0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "ENETC capability register 1"]
    #[inline(always)]
    pub const fn ecapr1(self) -> crate::common::Reg<regs::Ecapr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "ENETC capability register 2"]
    #[inline(always)]
    pub const fn ecapr2(self) -> crate::common::Reg<regs::Ecapr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Port mode register"]
    #[inline(always)]
    pub const fn pmr(self) -> crate::common::Reg<regs::Pmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Port outer native VLAN register"]
    #[inline(always)]
    pub const fn ponvlanr(self) -> crate::common::Reg<regs::Ponvlanr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "Port inner native VLAN register"]
    #[inline(always)]
    pub const fn pinvlanr(self) -> crate::common::Reg<regs::Pinvlanr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "Port VLAN classification control register"]
    #[inline(always)]
    pub const fn pvclctr(self) -> crate::common::Reg<regs::Pvclctr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "Parser checksum configuration register"]
    #[inline(always)]
    pub const fn parcscr(self) -> crate::common::Reg<regs::Parcscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "Parser custom Ethertype i configuration register"]
    #[inline(always)]
    pub const fn parcecr(self, n: usize) -> crate::common::Reg<regs::Parcecr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize + n * 4usize) as _) }
    }
    #[doc = "Port pause ON threshold register"]
    #[inline(always)]
    pub const fn ppauontr(self) -> crate::common::Reg<regs::Ppauontr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "Port pause OFF threshold register"]
    #[inline(always)]
    pub const fn ppauofftr(self) -> crate::common::Reg<regs::Ppauofftr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[doc = "Port receive memory buffer entitlement register"]
    #[inline(always)]
    pub const fn prxmber(self) -> crate::common::Reg<regs::Prxmber, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
    #[doc = "Port receive memory buffer limit register"]
    #[inline(always)]
    pub const fn prxmblr(self) -> crate::common::Reg<regs::Prxmblr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
    }
    #[doc = "Port receive buffer count register"]
    #[inline(always)]
    pub const fn prxbcr(self) -> crate::common::Reg<regs::Prxbcr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0128usize) as _) }
    }
    #[doc = "Port receive buffer count high watermark register"]
    #[inline(always)]
    pub const fn prxbchwmr(self) -> crate::common::Reg<regs::Prxbchwmr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x012cusize) as _) }
    }
    #[doc = "Set of registers which provides number of frame discarded by the Ingress Congestion Manager."]
    #[inline(always)]
    pub const fn picdra_dcr(self, n: usize) -> PicdraDcr {
        assert!(n < 4usize);
        unsafe { PicdraDcr::from_ptr(self.ptr.add(0x0140usize + n * 16usize) as _) }
    }
    #[doc = "Port ingress congestion priority discard status register"]
    #[inline(always)]
    pub const fn picpdsr(self) -> crate::common::Reg<regs::Picpdsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
    }
    #[doc = "Port station interface promiscuous MAC mode register"]
    #[inline(always)]
    pub const fn psipmmr(self) -> crate::common::Reg<regs::Psipmmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[doc = "Port station interface promiscuous VLAN mode register"]
    #[inline(always)]
    pub const fn psipvmr(self) -> crate::common::Reg<regs::Psipvmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0204usize) as _) }
    }
    #[doc = "Port broadcast frames dropped due to MAC filtering register"]
    #[inline(always)]
    pub const fn pbfdsir(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0208usize) as _) }
    }
    #[doc = "Port frame drop MAC source address pruning register"]
    #[inline(always)]
    pub const fn pfdmsapr(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x020cusize) as _) }
    }
    #[doc = "Port station interface MAC address filtering capability register"]
    #[inline(always)]
    pub const fn psimafcapr(self) -> crate::common::Reg<regs::Psimafcapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0280usize) as _) }
    }
    #[doc = "Port unicast frames dropped due to MAC filtering register"]
    #[inline(always)]
    pub const fn pufdmfr(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0284usize) as _) }
    }
    #[doc = "Port multicast frames dropped due to MAC filtering register"]
    #[inline(always)]
    pub const fn pmfdmfr(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0288usize) as _) }
    }
    #[doc = "Port station interface VLAN filtering capability register"]
    #[inline(always)]
    pub const fn psivlanfcapr(self) -> crate::common::Reg<regs::Psivlanfcapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02c0usize) as _) }
    }
    #[doc = "Port station interface VLAN filtering mode register"]
    #[inline(always)]
    pub const fn psivlanfmr(self) -> crate::common::Reg<regs::Psivlanfmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02c4usize) as _) }
    }
    #[doc = "Port unicast frames dropped VLAN filtering register"]
    #[inline(always)]
    pub const fn pufdvfr(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02d0usize) as _) }
    }
    #[doc = "Port multicast frames dropped VLAN filtering register"]
    #[inline(always)]
    pub const fn pmfdvfr(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02d4usize) as _) }
    }
    #[doc = "Port broadcast frames dropped VLAN filtering register"]
    #[inline(always)]
    pub const fn pbfdvfr(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02d8usize) as _) }
    }
    #[doc = "Port low power mode register"]
    #[inline(always)]
    pub const fn plpmr(self) -> crate::common::Reg<regs::Plpmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0340usize) as _) }
    }
    #[doc = "Port wake-on status register"]
    #[inline(always)]
    pub const fn pwosr(self) -> crate::common::Reg<regs::Pwosr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0344usize) as _) }
    }
    #[doc = "Receive IPV to ICM priority mapping register 0"]
    #[inline(always)]
    pub const fn ipv2icmpmr0(self) -> crate::common::Reg<regs::Ipv2icmpmr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0370usize) as _) }
    }
    #[doc = "Transmit priority to traffic class mapping register 0"]
    #[inline(always)]
    pub const fn prio2tcmr0(self) -> crate::common::Reg<regs::Prio2tcmr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0380usize) as _) }
    }
    #[doc = "Port traffic class a time specific departure register"]
    #[inline(always)]
    pub const fn ptctsdr(self, n: usize) -> crate::common::Reg<regs::Ptctsdr, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0390usize + n * 4usize) as _) }
    }
    #[doc = "Switch management capability register"]
    #[inline(always)]
    pub const fn smcapr(self) -> crate::common::Reg<regs::Smcapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0800usize) as _) }
    }
    #[doc = "Port station interface 0 primary MAC address register 0"]
    #[inline(always)]
    pub const fn psi0pmar0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2000usize) as _) }
    }
    #[doc = "Port station interface 0 primary MAC address register 1"]
    #[inline(always)]
    pub const fn psi0pmar1(self) -> crate::common::Reg<regs::Psi0pmar1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2004usize) as _) }
    }
    #[doc = "Port station interface 0 VLAN register"]
    #[inline(always)]
    pub const fn psi0vlanr(self) -> crate::common::Reg<regs::Psi0vlanr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2008usize) as _) }
    }
    #[doc = "Port station interface 0 configuration register 0"]
    #[inline(always)]
    pub const fn psi0cfgr0(self) -> crate::common::Reg<regs::Psi0cfgr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2010usize) as _) }
    }
    #[doc = "Port station interface 0 configuration register 2"]
    #[inline(always)]
    pub const fn psi0cfgr2(self) -> crate::common::Reg<regs::Psi0cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2018usize) as _) }
    }
    #[doc = "Port station interface 0 VSI MAC address filtering configuration register"]
    #[inline(always)]
    pub const fn psi0vmafcfgr(self) -> crate::common::Reg<regs::Psi0vmafcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2030usize) as _) }
    }
    #[doc = "Port station interface 0 VLAN filtering configuration register"]
    #[inline(always)]
    pub const fn psi0vlanfcfgr(self) -> crate::common::Reg<regs::Psi0vlanfcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2034usize) as _) }
    }
    #[doc = "Port station interface 0 unicast MAC hash filter register 0"]
    #[inline(always)]
    pub const fn psi0umhfr0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2050usize) as _) }
    }
    #[doc = "Port station interface 0 unicast MAC hash filter register 1"]
    #[inline(always)]
    pub const fn psi0umhfr1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2054usize) as _) }
    }
    #[doc = "Port station interface 0 multicast MAC hash filter register 0"]
    #[inline(always)]
    pub const fn psi0mmhfr0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2058usize) as _) }
    }
    #[doc = "Port station interface 0 multicast MAC hash filter register 1"]
    #[inline(always)]
    pub const fn psi0mmhfr1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x205cusize) as _) }
    }
    #[doc = "Port station interface 0 VLAN hash filter register 0"]
    #[inline(always)]
    pub const fn psi0vhfr0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2060usize) as _) }
    }
    #[doc = "Port station interface 0 VLAN hash filter register 1"]
    #[inline(always)]
    pub const fn psi0vhfr1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2064usize) as _) }
    }
}
#[doc = "Set of registers which provides number of frame discarded by the Ingress Congestion Manager."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PicdraDcr {
    ptr: *mut u8,
}
unsafe impl Send for PicdraDcr {}
unsafe impl Sync for PicdraDcr {}
impl PicdraDcr {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Port ingress congestion DRindex discard count register"]
    #[inline(always)]
    pub const fn picdrdcr(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Port ingress congestion DRindex discard count read-reset register"]
    #[inline(always)]
    pub const fn picdrdcrrr(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
}
pub mod regs;
pub mod vals;
