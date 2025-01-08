#[doc = "QoS to VLAN mapping profile register set."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MapPcp {
    ptr: *mut u8,
}
unsafe impl Send for MapPcp {}
unsafe impl Sync for MapPcp {}
impl MapPcp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "QoS to VLAN mapping profile a register b"]
    #[inline(always)]
    pub const fn qosvlanmpr(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Qosvlanmpr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
}
#[doc = "Set of registers for available Common BDRs."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NumCbdr {
    ptr: *mut u8,
}
unsafe impl Send for NumCbdr {}
unsafe impl Sync for NumCbdr {}
impl NumCbdr {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Command BDR a mode register"]
    #[inline(always)]
    pub const fn cbdrmr(self) -> crate::common::Reg<regs::Cbdrmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Command BDR a status register"]
    #[inline(always)]
    pub const fn cbdrsr(self) -> crate::common::Reg<regs::Cbdrsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Command BDR base address register 0"]
    #[inline(always)]
    pub const fn cbdrbar0(self) -> crate::common::Reg<regs::Cbdrbar0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Command BDR a base address register 1"]
    #[inline(always)]
    pub const fn cbdrbar1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Command BDR a producer index register"]
    #[inline(always)]
    pub const fn cbdrpir(self) -> crate::common::Reg<regs::Cbdrpir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Command BDR a consumer index register"]
    #[inline(always)]
    pub const fn cbdrcir(self) -> crate::common::Reg<regs::Cbdrcir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Command BDR a length register"]
    #[inline(always)]
    pub const fn cbdrlenr(self) -> crate::common::Reg<regs::Cbdrlenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
}
#[doc = "Set of interrupt registers for common BD rings."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NumCbdrInt {
    ptr: *mut u8,
}
unsafe impl Send for NumCbdrInt {}
unsafe impl Sync for NumCbdrInt {}
impl NumCbdrInt {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Command BDR a interrupt enable register"]
    #[inline(always)]
    pub const fn cbdrier(self) -> crate::common::Reg<regs::Cbdrier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Command BDR a interrupt detect register"]
    #[inline(always)]
    pub const fn cbdridr(self) -> crate::common::Reg<regs::Cbdridr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Command BDR a MSI-X vector register"]
    #[inline(always)]
    pub const fn cbdrmsivr(self) -> crate::common::Reg<regs::Cbdrmsivr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
}
#[doc = "Switch base"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sw0Base {
    ptr: *mut u8,
}
unsafe impl Send for Sw0Base {}
unsafe impl Sync for Sw0Base {}
impl Sw0Base {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Switch capability register 0"]
    #[inline(always)]
    pub const fn scapr0(self) -> crate::common::Reg<regs::Scapr0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Switch capability register 1"]
    #[inline(always)]
    pub const fn scapr1(self) -> crate::common::Reg<regs::Scapr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Buffer pool capability register"]
    #[inline(always)]
    pub const fn bpcapr(self) -> crate::common::Reg<regs::Bpcapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Forwarding capability register"]
    #[inline(always)]
    pub const fn fcapr(self) -> crate::common::Reg<regs::Fcapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Shared memory buffer capability register"]
    #[inline(always)]
    pub const fn smbcapr(self) -> crate::common::Reg<regs::Smbcapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "Shared memory buffer operational register 0"]
    #[inline(always)]
    pub const fn smbor0(self) -> crate::common::Reg<regs::Smbor0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "Shared memory buffer operational register 1"]
    #[inline(always)]
    pub const fn smbor1(self) -> crate::common::Reg<regs::Smbor1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "Command cache attribute register"]
    #[inline(always)]
    pub const fn ccar(self) -> crate::common::Reg<regs::Ccar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "Management port configuration register"]
    #[inline(always)]
    pub const fn mpcr(self) -> crate::common::Reg<regs::Mpcr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
    #[doc = "Ingress mirror destination configuration register 0"]
    #[inline(always)]
    pub const fn imdcr0(self) -> crate::common::Reg<regs::Imdcr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0420usize) as _) }
    }
    #[doc = "Ingress mirror destination configuration register 1"]
    #[inline(always)]
    pub const fn imdcr1(self) -> crate::common::Reg<regs::Imdcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0424usize) as _) }
    }
    #[doc = "Cut-through forwarding count register"]
    #[inline(always)]
    pub const fn ctfcr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0440usize) as _) }
    }
    #[doc = "Set of registers for available Common BDRs."]
    #[inline(always)]
    pub const fn num_cbdr(self, n: usize) -> NumCbdr {
        assert!(n < 2usize);
        unsafe { NumCbdr::from_ptr(self.ptr.add(0x0800usize + n * 48usize) as _) }
    }
    #[doc = "Set of interrupt registers for common BD rings."]
    #[inline(always)]
    pub const fn num_cbdr_int(self, n: usize) -> NumCbdrInt {
        assert!(n < 2usize);
        unsafe { NumCbdrInt::from_ptr(self.ptr.add(0x08a0usize + n * 16usize) as _) }
    }
    #[doc = "QoS to VLAN mapping profile register set."]
    #[inline(always)]
    pub const fn map_pcp(self, n: usize) -> MapPcp {
        assert!(n < 2usize);
        unsafe { MapPcp::from_ptr(self.ptr.add(0x0900usize + n * 32usize) as _) }
    }
    #[doc = "PCP to PCP mapping profile a register"]
    #[inline(always)]
    pub const fn pcp2pcpmpr(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Pcp2pcpmpr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b00usize + n * 4usize) as _) }
    }
    #[doc = "Bridge capability register"]
    #[inline(always)]
    pub const fn brcapr(self) -> crate::common::Reg<regs::Brcapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2000usize) as _) }
    }
    #[doc = "VLAN filter hash table capability register"]
    #[inline(always)]
    pub const fn vfhtcapr(self) -> crate::common::Reg<regs::Vfhtcapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2008usize) as _) }
    }
    #[doc = "VLAN filter hash table operational register"]
    #[inline(always)]
    pub const fn vfhtor(self) -> crate::common::Reg<regs::Vfhtor, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x200cusize) as _) }
    }
    #[doc = "VLAN Filter (hash) table default entry configuration registers 0"]
    #[inline(always)]
    pub const fn vfhtdecr0(self) -> crate::common::Reg<regs::Vfhtdecr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2010usize) as _) }
    }
    #[doc = "VLAN filter hash table default entry configuration registers 1"]
    #[inline(always)]
    pub const fn vfhtdecr1(self) -> crate::common::Reg<regs::Vfhtdecr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2014usize) as _) }
    }
    #[doc = "VLAN filter hash table default entry configuration registers 2"]
    #[inline(always)]
    pub const fn vfhtdecr2(self) -> crate::common::Reg<regs::Vfhtdecr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2018usize) as _) }
    }
    #[doc = "FDB hash table capability register"]
    #[inline(always)]
    pub const fn fdbhtcapr(self) -> crate::common::Reg<regs::Fdbhtcapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2020usize) as _) }
    }
    #[doc = "FDB hash table memory configuration register"]
    #[inline(always)]
    pub const fn fdbhtmcr(self) -> crate::common::Reg<regs::Fdbhtmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2024usize) as _) }
    }
    #[doc = "FDB hash table operational register 0"]
    #[inline(always)]
    pub const fn fdbhtor0(self) -> crate::common::Reg<regs::Fdbhtor0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2028usize) as _) }
    }
    #[doc = "FDB hash table operational register 1"]
    #[inline(always)]
    pub const fn fdbhtor1(self) -> crate::common::Reg<regs::Fdbhtor1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x202cusize) as _) }
    }
    #[doc = "IP multicast filter hash table capability register"]
    #[inline(always)]
    pub const fn ipmfhtcapr(self) -> crate::common::Reg<regs::Ipmfhtcapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2040usize) as _) }
    }
    #[doc = "IPv4 multicast filter hash table operation register"]
    #[inline(always)]
    pub const fn ipv4mfhtor(self) -> crate::common::Reg<regs::Ipv4mfhtor, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2044usize) as _) }
    }
}
pub mod regs;
pub mod vals;
