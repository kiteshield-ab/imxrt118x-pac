#[doc = "Transmitter and Receiver Buffer descriptor ring register set."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BdrNum {
    ptr: *mut u8,
}
unsafe impl Send for BdrNum {}
unsafe impl Sync for BdrNum {}
impl BdrNum {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Tx BDR a mode register"]
    #[inline(always)]
    pub const fn tbmr(self) -> crate::common::Reg<regs::Tbmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Tx BDR a status register"]
    #[inline(always)]
    pub const fn tbsr(self) -> crate::common::Reg<regs::Tbsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Tx BDR a base address register 0"]
    #[inline(always)]
    pub const fn tbbar0(self) -> crate::common::Reg<regs::Tbbar0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Tx BDR a base address register 1"]
    #[inline(always)]
    pub const fn tbbar1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Tx BDR a producer index register"]
    #[inline(always)]
    pub const fn tbpir(self) -> crate::common::Reg<regs::Tbpir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Tx BDR a consumer index register"]
    #[inline(always)]
    pub const fn tbcir(self) -> crate::common::Reg<regs::Tbcir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Tx BDR a length register"]
    #[inline(always)]
    pub const fn tblenr(self) -> crate::common::Reg<regs::Tblenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Tx BDR a interrupt enable register"]
    #[inline(always)]
    pub const fn tbier(self) -> crate::common::Reg<regs::Tbier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "Tx BDR a interrupt detect register"]
    #[inline(always)]
    pub const fn tbidr(self) -> crate::common::Reg<regs::Tbidr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "Tx BDR a interrupt coalescing register 0"]
    #[inline(always)]
    pub const fn tbicr0(self) -> crate::common::Reg<regs::Tbicr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "Tx BDR a interrupt coalescing register 1"]
    #[inline(always)]
    pub const fn tbicr1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[doc = "Rx BDR a mode register"]
    #[inline(always)]
    pub const fn rbmr(self) -> crate::common::Reg<regs::Rbmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "Rx BDR a status register"]
    #[inline(always)]
    pub const fn rbsr(self) -> crate::common::Reg<regs::Rbsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "Rx BDR a buffer size register"]
    #[inline(always)]
    pub const fn rbbsr(self) -> crate::common::Reg<regs::Rbbsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "Rx BDR a consumer index register"]
    #[inline(always)]
    pub const fn rbcir(self) -> crate::common::Reg<regs::Rbcir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[doc = "Rx BDR a base address register 0"]
    #[inline(always)]
    pub const fn rbbar0(self) -> crate::common::Reg<regs::Rbbar0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "Rx BDR a base address register 1"]
    #[inline(always)]
    pub const fn rbbar1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
    #[doc = "Rx BDR a producer index register"]
    #[inline(always)]
    pub const fn rbpir(self) -> crate::common::Reg<regs::Rbpir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
    }
    #[doc = "Rx BDR a length register"]
    #[inline(always)]
    pub const fn rblenr(self) -> crate::common::Reg<regs::Rblenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
    #[doc = "Rx BDR a drop count register"]
    #[inline(always)]
    pub const fn rbdcr(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
    }
    #[doc = "Rx BDR a interrupt enable register"]
    #[inline(always)]
    pub const fn rbier(self) -> crate::common::Reg<regs::Rbier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a0usize) as _) }
    }
    #[doc = "Rx BDR a interrupt detect register"]
    #[inline(always)]
    pub const fn rbidr(self) -> crate::common::Reg<regs::Rbidr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a4usize) as _) }
    }
    #[doc = "Rx BDR a interrupt coalescing register 0"]
    #[inline(always)]
    pub const fn rbicr0(self) -> crate::common::Reg<regs::Rbicr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a8usize) as _) }
    }
    #[doc = "Rx BDR a interrupt coalescing register 1"]
    #[inline(always)]
    pub const fn rbicr1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01acusize) as _) }
    }
}
#[doc = "ENETC Station Interface"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enetc1Si0 {
    ptr: *mut u8,
}
unsafe impl Send for Enetc1Si0 {}
unsafe impl Sync for Enetc1Si0 {}
impl Enetc1Si0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Station interface mode register"]
    #[inline(always)]
    pub const fn simr(self) -> crate::common::Reg<regs::Simr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Station interface status register"]
    #[inline(always)]
    pub const fn sisr(self) -> crate::common::Reg<regs::Sisr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Station interface current time register 0"]
    #[inline(always)]
    pub const fn sictr0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Station interface current time register 1"]
    #[inline(always)]
    pub const fn sictr1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Station interface port capability register 0"]
    #[inline(always)]
    pub const fn sipcapr0(self) -> crate::common::Reg<regs::Sipcapr0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Station interface port capability register 1"]
    #[inline(always)]
    pub const fn sipcapr1(self) -> crate::common::Reg<regs::Sipcapr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Station interface timer status register"]
    #[inline(always)]
    pub const fn sitsr(self) -> crate::common::Reg<regs::Sitsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Station interface receive BDR group control register"]
    #[inline(always)]
    pub const fn sirbgcr(self) -> crate::common::Reg<regs::Sirbgcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "Station interface buffer cache attribute register"]
    #[inline(always)]
    pub const fn sibcar(self) -> crate::common::Reg<regs::Sibcar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "Station interface message cache attribute register"]
    #[inline(always)]
    pub const fn simcar(self) -> crate::common::Reg<regs::Simcar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "Station interface command cache attribute register"]
    #[inline(always)]
    pub const fn siccar(self) -> crate::common::Reg<regs::Siccar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "Station interface primary MAC address register 0"]
    #[inline(always)]
    pub const fn sipmar0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "Station interface primary MAC address register 1"]
    #[inline(always)]
    pub const fn sipmar1(self) -> crate::common::Reg<regs::Sipmar1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "Station interface custom VLAN register 1"]
    #[inline(always)]
    pub const fn sicvlanr1(self) -> crate::common::Reg<regs::Sicvlanr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "Station interface custom VLAN register 2"]
    #[inline(always)]
    pub const fn sicvlanr2(self) -> crate::common::Reg<regs::Sicvlanr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "Station interface VLAN to IPV mapping register 0"]
    #[inline(always)]
    pub const fn sivlanipvmr0(self) -> crate::common::Reg<regs::Sivlanipvmr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "Station interface VLAN to IPV mapping register 1"]
    #[inline(always)]
    pub const fn sivlanipvmr1(self) -> crate::common::Reg<regs::Sivlanipvmr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "Station interface IPV to ring mapping register"]
    #[inline(always)]
    pub const fn siipvbdrmr0(self) -> crate::common::Reg<regs::Siipvbdrmr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0150usize) as _) }
    }
    #[doc = "Physical station interface message receive register"]
    #[inline(always)]
    pub const fn psimsgrr(self) -> crate::common::Reg<regs::Psimsgrr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0204usize) as _) }
    }
    #[doc = "Physical station interface message send register"]
    #[inline(always)]
    pub const fn psimsgsr(self) -> crate::common::Reg<regs::Psimsgsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0208usize) as _) }
    }
    #[doc = "PSI VSI 1 message receive address register 0"]
    #[inline(always)]
    pub const fn psiv1msgrcvar0(
        self,
    ) -> crate::common::Reg<regs::Psiv1msgrcvar0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0210usize) as _) }
    }
    #[doc = "PSI VSI 1 message receive address register 1"]
    #[inline(always)]
    pub const fn psiv1msgrcvar1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0214usize) as _) }
    }
    #[doc = "Station interface receive octets counter (ifInOctets) 0"]
    #[inline(always)]
    pub const fn siroct0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
    }
    #[doc = "Station interface receive octets counter (ifInOctets) 1"]
    #[inline(always)]
    pub const fn siroct1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
    }
    #[doc = "Station interface receive frame counter (aFrameReceivedOK) 0"]
    #[inline(always)]
    pub const fn sirfrm0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
    }
    #[doc = "Station interface receive frame counter (aFrameReceivedOK) 1"]
    #[inline(always)]
    pub const fn sirfrm1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x030cusize) as _) }
    }
    #[doc = "Station interface receive unicast frame counter (ifInUcastPkts) 0"]
    #[inline(always)]
    pub const fn siruca0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0310usize) as _) }
    }
    #[doc = "Station interface receive unicast frame counter (ifInUcastPkts) 1"]
    #[inline(always)]
    pub const fn siruca1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0314usize) as _) }
    }
    #[doc = "Station interface receive multicast frame counter (ifInMulticastPkts) 0"]
    #[inline(always)]
    pub const fn sirmca0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0318usize) as _) }
    }
    #[doc = "Station interface receive multicast frame counter (ifInMulticastPkts) 1"]
    #[inline(always)]
    pub const fn sirmca1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x031cusize) as _) }
    }
    #[doc = "Station interface transmit octets counter (ifOutOctets) 0"]
    #[inline(always)]
    pub const fn sitoct0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0320usize) as _) }
    }
    #[doc = "Station interface transmit octets counter (ifOutOctets) 1"]
    #[inline(always)]
    pub const fn sitoct1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0324usize) as _) }
    }
    #[doc = "Station interface transmit frame counter (aFrameTransmittedOK) 0"]
    #[inline(always)]
    pub const fn sitfrm0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0328usize) as _) }
    }
    #[doc = "Station interface transmit frame counter (aFrameTransmittedOK) 1"]
    #[inline(always)]
    pub const fn sitfrm1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x032cusize) as _) }
    }
    #[doc = "Station interface transmit unicast frame counter (ifOutUcastPkts) 0"]
    #[inline(always)]
    pub const fn situca0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0330usize) as _) }
    }
    #[doc = "Station interface transmit unicast frame counter (ifOutUcastPkts) 1"]
    #[inline(always)]
    pub const fn situca1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0334usize) as _) }
    }
    #[doc = "Station interface transmit multicast frame counter (ifOutMulticastPkts) 0"]
    #[inline(always)]
    pub const fn sitmca0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0338usize) as _) }
    }
    #[doc = "Station interface transmit multicast frame counter (ifOutMulticastPkts) 1"]
    #[inline(always)]
    pub const fn sitmca1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x033cusize) as _) }
    }
    #[doc = "Station interface command BDR mode register"]
    #[inline(always)]
    pub const fn sicbdrmr(self) -> crate::common::Reg<regs::Sicbdrmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0800usize) as _) }
    }
    #[doc = "Station interface command BDR status register"]
    #[inline(always)]
    pub const fn sicbdrsr(self) -> crate::common::Reg<regs::Sicbdrsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0804usize) as _) }
    }
    #[doc = "Station interface command BDR base address register 0"]
    #[inline(always)]
    pub const fn sicbdrbar0(self) -> crate::common::Reg<regs::Sicbdrbar0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0810usize) as _) }
    }
    #[doc = "Station interface command BDR base address register 1"]
    #[inline(always)]
    pub const fn sicbdrbar1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0814usize) as _) }
    }
    #[doc = "Station interface command BDR producer index register"]
    #[inline(always)]
    pub const fn sicbdrpir(self) -> crate::common::Reg<regs::Sicbdrpir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0818usize) as _) }
    }
    #[doc = "Station interface command BDR consumer index register"]
    #[inline(always)]
    pub const fn sicbdrcir(self) -> crate::common::Reg<regs::Sicbdrcir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x081cusize) as _) }
    }
    #[doc = "Station interface command BDR length register"]
    #[inline(always)]
    pub const fn sicbdrlenr(self) -> crate::common::Reg<regs::Sicbdrlenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0820usize) as _) }
    }
    #[doc = "Station interface command BDR interrupt enable register"]
    #[inline(always)]
    pub const fn sicbdrier(self) -> crate::common::Reg<regs::Sicbdrier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08a0usize) as _) }
    }
    #[doc = "Station interface command BDR interrupt detect register"]
    #[inline(always)]
    pub const fn sicbdridr(self) -> crate::common::Reg<regs::Sicbdridr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08a4usize) as _) }
    }
    #[doc = "Station interface capability register 0"]
    #[inline(always)]
    pub const fn sicapr0(self) -> crate::common::Reg<regs::Sicapr0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0900usize) as _) }
    }
    #[doc = "Station interface capability register 1"]
    #[inline(always)]
    pub const fn sicapr1(self) -> crate::common::Reg<regs::Sicapr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0904usize) as _) }
    }
    #[doc = "Station interface capability register 2"]
    #[inline(always)]
    pub const fn sicapr2(self) -> crate::common::Reg<regs::Sicapr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0908usize) as _) }
    }
    #[doc = "Physical station interface interrupt enable register"]
    #[inline(always)]
    pub const fn psiier(self) -> crate::common::Reg<regs::Psiier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0a00usize) as _) }
    }
    #[doc = "Physical station interface interrupt detect register"]
    #[inline(always)]
    pub const fn psiidr(self) -> crate::common::Reg<regs::Psiidr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0a08usize) as _) }
    }
    #[doc = "Station interface transmit interrupt detect register 0"]
    #[inline(always)]
    pub const fn sitxidr0(self) -> crate::common::Reg<regs::Sitxidr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0a18usize) as _) }
    }
    #[doc = "Station interface receive interrupt detect register 0"]
    #[inline(always)]
    pub const fn sirxidr0(self) -> crate::common::Reg<regs::Sirxidr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0a28usize) as _) }
    }
    #[doc = "Station interface MSI-X vector register"]
    #[inline(always)]
    pub const fn simsivr(self) -> crate::common::Reg<regs::Simsivr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0a30usize) as _) }
    }
    #[doc = "Station interface command MSI-X vector register"]
    #[inline(always)]
    pub const fn sicmsivr(self) -> crate::common::Reg<regs::Sicmsivr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0a34usize) as _) }
    }
    #[doc = "Station interface timer interrupt enable register"]
    #[inline(always)]
    pub const fn sitmrier(self) -> crate::common::Reg<regs::Sitmrier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0a40usize) as _) }
    }
    #[doc = "Station interface timer interrupt detect register"]
    #[inline(always)]
    pub const fn sitmridr(self) -> crate::common::Reg<regs::Sitmridr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0a44usize) as _) }
    }
    #[doc = "Station interface timer MSI-X vector register"]
    #[inline(always)]
    pub const fn sitmrmsivr(self) -> crate::common::Reg<regs::Sitmrmsivr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0a4cusize) as _) }
    }
    #[doc = "Station interface MSI-X transmit ring a vector register"]
    #[inline(always)]
    pub const fn simsitrvr(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Simsitrvr, crate::common::RW> {
        assert!(n < 10usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b00usize + n * 4usize) as _) }
    }
    #[doc = "Station interface MSI-X receive ring a vector register"]
    #[inline(always)]
    pub const fn simsirrvr(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Simsirrvr, crate::common::RW> {
        assert!(n < 10usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b80usize + n * 4usize) as _) }
    }
    #[doc = "Station interface correctable memory error configuration register"]
    #[inline(always)]
    pub const fn sicmecr(self) -> crate::common::Reg<regs::Sicmecr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e00usize) as _) }
    }
    #[doc = "Station interface correctable memory error status register"]
    #[inline(always)]
    pub const fn sicmesr(self) -> crate::common::Reg<regs::Sicmesr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e04usize) as _) }
    }
    #[doc = "Station interface correctable memory error count register"]
    #[inline(always)]
    pub const fn sicmectr(self) -> crate::common::Reg<regs::Sicmectr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e0cusize) as _) }
    }
    #[doc = "Station interface uncorrectable programming error configuration register"]
    #[inline(always)]
    pub const fn siupecr(self) -> crate::common::Reg<regs::Siupecr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e10usize) as _) }
    }
    #[doc = "Station interface uncorrectable programming error status register"]
    #[inline(always)]
    pub const fn siupesr(self) -> crate::common::Reg<regs::Siupesr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e14usize) as _) }
    }
    #[doc = "Station interface uncorrectable programming error count register"]
    #[inline(always)]
    pub const fn siupectr(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e1cusize) as _) }
    }
    #[doc = "Station interface uncorrectable non-fatal system bus error configuration register"]
    #[inline(always)]
    pub const fn siunsbecr(self) -> crate::common::Reg<regs::Siunsbecr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e20usize) as _) }
    }
    #[doc = "Station interface uncorrectable non-fatal system bus error status register"]
    #[inline(always)]
    pub const fn siunsbesr(self) -> crate::common::Reg<regs::Siunsbesr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e24usize) as _) }
    }
    #[doc = "Station interface uncorrectable non-fatal system bus error count register"]
    #[inline(always)]
    pub const fn siunsbectr(self) -> crate::common::Reg<regs::Siunsbectr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e2cusize) as _) }
    }
    #[doc = "Station interface uncorrectable fatal system bus error configuration register"]
    #[inline(always)]
    pub const fn siufsbecr(self) -> crate::common::Reg<regs::Siufsbecr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e30usize) as _) }
    }
    #[doc = "Station interface uncorrectable fatal system bus error status register"]
    #[inline(always)]
    pub const fn siufsbesr(self) -> crate::common::Reg<regs::Siufsbesr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e34usize) as _) }
    }
    #[doc = "Station interface uncorrectable non-fatal memory error configuration register"]
    #[inline(always)]
    pub const fn siunmecr(self) -> crate::common::Reg<regs::Siunmecr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e40usize) as _) }
    }
    #[doc = "Station interface uncorrectable non-fatal memory error status register 0"]
    #[inline(always)]
    pub const fn siunmesr0(self) -> crate::common::Reg<regs::Siunmesr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e44usize) as _) }
    }
    #[doc = "Station interface uncorrectable non-fatal memory error status register 1"]
    #[inline(always)]
    pub const fn siunmesr1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e48usize) as _) }
    }
    #[doc = "Station interface uncorrectable non-fatal memory error count register"]
    #[inline(always)]
    pub const fn siunmectr(self) -> crate::common::Reg<regs::Siunmectr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e4cusize) as _) }
    }
    #[doc = "Station interface uncorrectable fatal memory error configuration register"]
    #[inline(always)]
    pub const fn siufmecr(self) -> crate::common::Reg<regs::Siufmecr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e50usize) as _) }
    }
    #[doc = "Station interface uncorrectable fatal memory error status register 0"]
    #[inline(always)]
    pub const fn siufmesr0(self) -> crate::common::Reg<regs::Siufmesr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e54usize) as _) }
    }
    #[doc = "Station interface uncorrectable fatal memory error status register 1"]
    #[inline(always)]
    pub const fn siufmesr1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e58usize) as _) }
    }
    #[doc = "Station interface MAC address filter table capability register"]
    #[inline(always)]
    pub const fn simaftcapr(self) -> crate::common::Reg<regs::Simaftcapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1000usize) as _) }
    }
    #[doc = "Station interface VLAN filter table capability register"]
    #[inline(always)]
    pub const fn sivftcapr(self) -> crate::common::Reg<regs::Sivftcapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1100usize) as _) }
    }
    #[doc = "Transmitter and Receiver Buffer descriptor ring register set."]
    #[inline(always)]
    pub const fn bdr_num(self, n: usize) -> BdrNum {
        assert!(n < 10usize);
        unsafe { BdrNum::from_ptr(self.ptr.add(0x8000usize + n * 512usize) as _) }
    }
}
pub mod regs;
