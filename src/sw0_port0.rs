#[doc = "Port"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sw0Port0 {
    ptr: *mut u8,
}
unsafe impl Send for Sw0Port0 {}
unsafe impl Sync for Sw0Port0 {}
impl Sw0Port0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Port capability register"]
    #[inline(always)]
    pub const fn pcapr(self) -> crate::common::Reg<regs::Pcapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Port MAC capability register"]
    #[inline(always)]
    pub const fn pmcapr(self) -> crate::common::Reg<regs::Pmcapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Port I/O capability register"]
    #[inline(always)]
    pub const fn piocapr(self) -> crate::common::Reg<regs::Piocapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Port configuration register"]
    #[inline(always)]
    pub const fn pcr(self) -> crate::common::Reg<regs::Pcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Port MAC address register 0"]
    #[inline(always)]
    pub const fn pmar0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Port MAC address register 1"]
    #[inline(always)]
    pub const fn pmar1(self) -> crate::common::Reg<regs::Pmar1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Port TPID acceptance register"]
    #[inline(always)]
    pub const fn ptar(self) -> crate::common::Reg<regs::Ptar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "Port QoS mode register"]
    #[inline(always)]
    pub const fn pqosmr(self) -> crate::common::Reg<regs::Pqosmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "Port Queue Operational register"]
    #[inline(always)]
    pub const fn pqor(self) -> crate::common::Reg<regs::Pqor, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "Port parser configuration register"]
    #[inline(always)]
    pub const fn ppcr(self) -> crate::common::Reg<regs::Ppcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "Port ingress port filter configuration register"]
    #[inline(always)]
    pub const fn pipfcr(self) -> crate::common::Reg<regs::Pipfcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "Port stream gate configuration register"]
    #[inline(always)]
    pub const fn psgcr(self) -> crate::common::Reg<regs::Psgcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "Port operational register"]
    #[inline(always)]
    pub const fn por(self) -> crate::common::Reg<regs::Por, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "Port status register"]
    #[inline(always)]
    pub const fn psr(self) -> crate::common::Reg<regs::Psr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "Port receive SDU overhead register"]
    #[inline(always)]
    pub const fn prxsduor(self) -> crate::common::Reg<regs::Prxsduor, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "Port transmit SDU overhead register"]
    #[inline(always)]
    pub const fn ptxsduor(self) -> crate::common::Reg<regs::Ptxsduor, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[doc = "Port time gate scheduling control register"]
    #[inline(always)]
    pub const fn ptgscr(self) -> crate::common::Reg<regs::Ptgscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "Port time gate scheduling admin gate list status register"]
    #[inline(always)]
    pub const fn ptgaglsr(self) -> crate::common::Reg<regs::Ptgaglsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
    #[doc = "Port time gate scheduling admin gate list length register"]
    #[inline(always)]
    pub const fn ptgagllr(self) -> crate::common::Reg<regs::Ptgagllr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
    }
    #[doc = "Port time gating operational gate list length register"]
    #[inline(always)]
    pub const fn ptgogllr(self) -> crate::common::Reg<regs::Ptgogllr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
    }
    #[doc = "Port time gate scheduling hold advance register"]
    #[inline(always)]
    pub const fn ptgshar(self) -> crate::common::Reg<regs::Ptgshar, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
    }
    #[doc = "Port time gate scheduling release advance register"]
    #[inline(always)]
    pub const fn ptgsrar(self) -> crate::common::Reg<regs::Ptgsrar, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0128usize) as _) }
    }
    #[doc = "Port time gate scheduling hold configuration register"]
    #[inline(always)]
    pub const fn ptgshcr(self) -> crate::common::Reg<regs::Ptgshcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x012cusize) as _) }
    }
    #[doc = "Port frame preemption configuration register"]
    #[inline(always)]
    pub const fn pfpcr(self) -> crate::common::Reg<regs::Pfpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0134usize) as _) }
    }
    #[doc = "Port default gate state register"]
    #[inline(always)]
    pub const fn pdgsr(self) -> crate::common::Reg<regs::Pdgsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0138usize) as _) }
    }
    #[doc = "Port Rx discard count register"]
    #[inline(always)]
    pub const fn prxdcr(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c0usize) as _) }
    }
    #[doc = "Port Rx discard count reason register 0"]
    #[inline(always)]
    pub const fn prxdcrr0(self) -> crate::common::Reg<regs::Prxdcrr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c8usize) as _) }
    }
    #[doc = "Port Rx discard count reason register 1"]
    #[inline(always)]
    pub const fn prxdcrr1(self) -> crate::common::Reg<regs::Prxdcrr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01ccusize) as _) }
    }
    #[doc = "Port Tx discard count register"]
    #[inline(always)]
    pub const fn ptxdcr(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01e0usize) as _) }
    }
    #[doc = "Port Tx discard count reason register 0"]
    #[inline(always)]
    pub const fn ptxdcrr0(self) -> crate::common::Reg<regs::Ptxdcrr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01e8usize) as _) }
    }
    #[doc = "Port Tx discard count reason register 1"]
    #[inline(always)]
    pub const fn ptxdcrr1(self) -> crate::common::Reg<regs::Ptxdcrr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01ecusize) as _) }
    }
    #[doc = "Array of registers: PTGSTCSR, PTCTMSDUR, PTCCBSR0, PTCCBSR1"]
    #[inline(always)]
    pub const fn tct_num(self, n: usize) -> TctNum {
        assert!(n < 8usize);
        unsafe { TctNum::from_ptr(self.ptr.add(0x0200usize + n * 32usize) as _) }
    }
    #[doc = "Port buffer pool mapping configuration register 0"]
    #[inline(always)]
    pub const fn pbpmcr0(self) -> crate::common::Reg<regs::Pbpmcr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
    #[doc = "Port buffer pool mapping configuration register 1"]
    #[inline(always)]
    pub const fn pbpmcr1(self) -> crate::common::Reg<regs::Pbpmcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0404usize) as _) }
    }
    #[doc = "Port PCP DEI mapping register"]
    #[inline(always)]
    pub const fn ppcpdeimr(self) -> crate::common::Reg<regs::Ppcpdeimr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0438usize) as _) }
    }
    #[doc = "Port mirror configuration register"]
    #[inline(always)]
    pub const fn pmcr(self) -> crate::common::Reg<regs::Pmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0440usize) as _) }
    }
    #[doc = "Port cut through forwarding configuration register"]
    #[inline(always)]
    pub const fn pctfcr(self) -> crate::common::Reg<regs::Pctfcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0450usize) as _) }
    }
    #[doc = "Port LANID configuration register"]
    #[inline(always)]
    pub const fn planidcr(self) -> crate::common::Reg<regs::Planidcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0458usize) as _) }
    }
    #[doc = "Port ingress stream identification configuration register"]
    #[inline(always)]
    pub const fn pisidcr(self) -> crate::common::Reg<regs::Pisidcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0460usize) as _) }
    }
    #[doc = "Port frame modification configuration register"]
    #[inline(always)]
    pub const fn pfmcr(self) -> crate::common::Reg<regs::Pfmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0464usize) as _) }
    }
    #[doc = "Port IPV to queue mapping register 0"]
    #[inline(always)]
    pub const fn pipv2qmr0(self) -> crate::common::Reg<regs::Pipv2qmr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0470usize) as _) }
    }
    #[doc = "Port time capture minimum latency register"]
    #[inline(always)]
    pub const fn ptcminlr(self) -> crate::common::Reg<regs::Ptcminlr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04b0usize) as _) }
    }
    #[doc = "Port time capture maximum latency register"]
    #[inline(always)]
    pub const fn ptcmaxlr(self) -> crate::common::Reg<regs::Ptcmaxlr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04b4usize) as _) }
    }
    #[doc = "Bridge port configuration register"]
    #[inline(always)]
    pub const fn bpcr(self) -> crate::common::Reg<regs::Bpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
    }
    #[doc = "Bridge port default VLAN register"]
    #[inline(always)]
    pub const fn bpdvr(self) -> crate::common::Reg<regs::Bpdvr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
    }
    #[doc = "Bridge port spanning tree group state register"]
    #[inline(always)]
    pub const fn bpstgsr(self) -> crate::common::Reg<regs::Bpstgsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0520usize) as _) }
    }
    #[doc = "Bridge port storm control register 0"]
    #[inline(always)]
    pub const fn bpscr0(self) -> crate::common::Reg<regs::Bpscr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0528usize) as _) }
    }
    #[doc = "Bridge port storm control register 1"]
    #[inline(always)]
    pub const fn bpscr1(self) -> crate::common::Reg<regs::Bpscr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x052cusize) as _) }
    }
    #[doc = "Bridge port operational register"]
    #[inline(always)]
    pub const fn bpor(self) -> crate::common::Reg<regs::Bpor, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0530usize) as _) }
    }
    #[doc = "Bridge port discard count register"]
    #[inline(always)]
    pub const fn bpdcr(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0580usize) as _) }
    }
    #[doc = "Bridge port discard count reason register 0"]
    #[inline(always)]
    pub const fn bpdcrr0(self) -> crate::common::Reg<regs::Bpdcrr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0588usize) as _) }
    }
    #[doc = "Bridge port discard count reason register 1"]
    #[inline(always)]
    pub const fn bpdcrr1(self) -> crate::common::Reg<regs::Bpdcrr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x058cusize) as _) }
    }
    #[doc = "Bridge port MAC learning failure status register"]
    #[inline(always)]
    pub const fn bpmlfsr(self) -> crate::common::Reg<regs::Bpmlfsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0590usize) as _) }
    }
}
#[doc = "Array of registers: PTGSTCSR, PTCTMSDUR, PTCCBSR0, PTCCBSR1"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TctNum {
    ptr: *mut u8,
}
unsafe impl Send for TctNum {}
unsafe impl Sync for TctNum {}
impl TctNum {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Port time gate scheduling traffic class a status register"]
    #[inline(always)]
    pub const fn ptgstcsr(self) -> crate::common::Reg<regs::Ptgstcsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Port traffic class a transmit maximum SDU register"]
    #[inline(always)]
    pub const fn ptctmsdur(self) -> crate::common::Reg<regs::Ptctmsdur, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Port transmit traffic class a credit based shaper register 0"]
    #[inline(always)]
    pub const fn ptccbsr0(self) -> crate::common::Reg<regs::Ptccbsr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Port traffic class a credit based shaper register 1"]
    #[inline(always)]
    pub const fn ptccbsr1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
}
pub mod regs;
pub mod vals;
