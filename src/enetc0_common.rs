#[doc = "Switch and ENETC common base"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enetc0Common {
    ptr: *mut u8,
}
unsafe impl Send for Enetc0Common {}
unsafe impl Sync for Enetc0Common {}
impl Enetc0Common {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Ingress port capability register"]
    #[inline(always)]
    pub const fn ipcapr(self) -> crate::common::Reg<regs::Ipcapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1000usize) as _) }
    }
    #[doc = "Egress port capability register"]
    #[inline(always)]
    pub const fn epcapr(self) -> crate::common::Reg<regs::Epcapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1004usize) as _) }
    }
    #[doc = "Operational state register"]
    #[inline(always)]
    pub const fn osr(self) -> crate::common::Reg<regs::Osr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1010usize) as _) }
    }
    #[doc = "Correctable memory error configuration register"]
    #[inline(always)]
    pub const fn cmecr(self) -> crate::common::Reg<regs::Cmecr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1040usize) as _) }
    }
    #[doc = "Correctable memory error status register"]
    #[inline(always)]
    pub const fn cmesr(self) -> crate::common::Reg<regs::Cmesr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1044usize) as _) }
    }
    #[doc = "Correctable memory error count register"]
    #[inline(always)]
    pub const fn cmectr(self) -> crate::common::Reg<regs::Cmectr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x104cusize) as _) }
    }
    #[doc = "Uncorrectable non-fatal MAC error configuration register"]
    #[inline(always)]
    pub const fn unmacecr(self) -> crate::common::Reg<regs::Unmacecr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1060usize) as _) }
    }
    #[doc = "Uncorrectable non-fatal MAC error status register"]
    #[inline(always)]
    pub const fn unmacesr(self) -> crate::common::Reg<regs::Unmacesr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1064usize) as _) }
    }
    #[doc = "Uncorrectable non-fatal memory error configuration register"]
    #[inline(always)]
    pub const fn unmecr(self) -> crate::common::Reg<regs::Unmecr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1090usize) as _) }
    }
    #[doc = "Uncorrectable non-fatal memory error status register 0"]
    #[inline(always)]
    pub const fn unmesr0(self) -> crate::common::Reg<regs::Unmesr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1094usize) as _) }
    }
    #[doc = "Uncorrectable non-fatal memory error status register 1"]
    #[inline(always)]
    pub const fn unmesr1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1098usize) as _) }
    }
    #[doc = "Uncorrectable non-fatal memory error count register"]
    #[inline(always)]
    pub const fn unmectr(self) -> crate::common::Reg<regs::Unmectr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x109cusize) as _) }
    }
    #[doc = "Uncorrectable fatal memory error configuration register"]
    #[inline(always)]
    pub const fn ufmecr(self) -> crate::common::Reg<regs::Ufmecr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10a0usize) as _) }
    }
    #[doc = "Uncorrectable fatal memory error status register 0"]
    #[inline(always)]
    pub const fn ufmesr0(self) -> crate::common::Reg<regs::Ufmesr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10a4usize) as _) }
    }
    #[doc = "Uncorrectable fatal memory error status register 1"]
    #[inline(always)]
    pub const fn ufmesr1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10a8usize) as _) }
    }
    #[doc = "Internal MDIO interrupt reason register"]
    #[inline(always)]
    pub const fn imdioirr(self) -> crate::common::Reg<regs::Imdioirr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10e0usize) as _) }
    }
    #[doc = "Internal MDIO MSI-X vector register"]
    #[inline(always)]
    pub const fn imdiomsivr(self) -> crate::common::Reg<regs::Imdiomsivr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10e4usize) as _) }
    }
    #[doc = "External MDIO interrupt reason register"]
    #[inline(always)]
    pub const fn emdioirr(self) -> crate::common::Reg<regs::Emdioirr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10e8usize) as _) }
    }
    #[doc = "External MDIO MSI-X vector register"]
    #[inline(always)]
    pub const fn emdiomsivr(self) -> crate::common::Reg<regs::Emdiomsivr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10ecusize) as _) }
    }
    #[doc = "Custom VLAN Ethertype register 1"]
    #[inline(always)]
    pub const fn cvlanr1(self) -> crate::common::Reg<regs::Cvlanr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1200usize) as _) }
    }
    #[doc = "Custom VLAN Ethertype register 2"]
    #[inline(always)]
    pub const fn cvlanr2(self) -> crate::common::Reg<regs::Cvlanr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1204usize) as _) }
    }
    #[doc = "DoS L2 configuration register"]
    #[inline(always)]
    pub const fn dosl2cr(self) -> crate::common::Reg<regs::Dosl2cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1220usize) as _) }
    }
    #[doc = "VLAN to IPV mapping profile 0 register 0"]
    #[inline(always)]
    pub const fn vlanipvmp0r0(self) -> crate::common::Reg<regs::Vlanipvmp0r0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1300usize) as _) }
    }
    #[doc = "VLAN to IPV mapping profile 0 register 1"]
    #[inline(always)]
    pub const fn vlanipvmp0r1(self) -> crate::common::Reg<regs::Vlanipvmp0r1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1304usize) as _) }
    }
    #[doc = "VLAN to DR mapping profile 0 register"]
    #[inline(always)]
    pub const fn vlandrmp0r(self) -> crate::common::Reg<regs::Vlandrmp0r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1308usize) as _) }
    }
    #[doc = "Ingress port filter capability register"]
    #[inline(always)]
    pub const fn ipfcapr(self) -> crate::common::Reg<regs::Ipfcapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1640usize) as _) }
    }
    #[doc = "Ingress port filter table capability register"]
    #[inline(always)]
    pub const fn ipftcapr(self) -> crate::common::Reg<regs::Ipftcapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1644usize) as _) }
    }
    #[doc = "Ingress port filter table memory operational register"]
    #[inline(always)]
    pub const fn ipftmor(self) -> crate::common::Reg<regs::Ipftmor, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1648usize) as _) }
    }
    #[doc = "Index table memory capability register"]
    #[inline(always)]
    pub const fn itmcapr(self) -> crate::common::Reg<regs::Itmcapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1800usize) as _) }
    }
    #[doc = "Rate policer capability register"]
    #[inline(always)]
    pub const fn rpcapr(self) -> crate::common::Reg<regs::Rpcapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1810usize) as _) }
    }
    #[doc = "Rate policer index table capability register"]
    #[inline(always)]
    pub const fn rpitcapr(self) -> crate::common::Reg<regs::Rpitcapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1814usize) as _) }
    }
    #[doc = "Rate policer index table memory allocation register"]
    #[inline(always)]
    pub const fn rpitmar(self) -> crate::common::Reg<regs::Rpitmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1818usize) as _) }
    }
    #[doc = "Rate policer index table operational register"]
    #[inline(always)]
    pub const fn rpitor(self) -> crate::common::Reg<regs::Rpitor, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x181cusize) as _) }
    }
    #[doc = "Ingress stream counter index table capability register"]
    #[inline(always)]
    pub const fn iscitcapr(self) -> crate::common::Reg<regs::Iscitcapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1824usize) as _) }
    }
    #[doc = "Ingress stream counter index table memory allocation register"]
    #[inline(always)]
    pub const fn iscitmar(self) -> crate::common::Reg<regs::Iscitmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1828usize) as _) }
    }
    #[doc = "Ingress stream counter index table operational register"]
    #[inline(always)]
    pub const fn iscitor(self) -> crate::common::Reg<regs::Iscitor, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x182cusize) as _) }
    }
    #[doc = "Ingress stream capability register"]
    #[inline(always)]
    pub const fn iscapr(self) -> crate::common::Reg<regs::Iscapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1830usize) as _) }
    }
    #[doc = "Ingress stream index table capability register"]
    #[inline(always)]
    pub const fn isitcapr(self) -> crate::common::Reg<regs::Isitcapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1834usize) as _) }
    }
    #[doc = "Ingress stream index table memory allocation register"]
    #[inline(always)]
    pub const fn isitmar(self) -> crate::common::Reg<regs::Isitmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1838usize) as _) }
    }
    #[doc = "Ingress stream index table operational register"]
    #[inline(always)]
    pub const fn isitor(self) -> crate::common::Reg<regs::Isitor, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x183cusize) as _) }
    }
    #[doc = "Stream gate capability register"]
    #[inline(always)]
    pub const fn sgcapr(self) -> crate::common::Reg<regs::Sgcapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1860usize) as _) }
    }
    #[doc = "Stream gate instance index table capability register"]
    #[inline(always)]
    pub const fn sgiitcapr(self) -> crate::common::Reg<regs::Sgiitcapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1864usize) as _) }
    }
    #[doc = "Stream gate instance index table memory allocation register"]
    #[inline(always)]
    pub const fn sgiitmar(self) -> crate::common::Reg<regs::Sgiitmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1868usize) as _) }
    }
    #[doc = "Stream gate instance index table operational register"]
    #[inline(always)]
    pub const fn sgiitor(self) -> crate::common::Reg<regs::Sgiitor, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x186cusize) as _) }
    }
    #[doc = "Stream gate control list index table capability register"]
    #[inline(always)]
    pub const fn sgclitcapr(self) -> crate::common::Reg<regs::Sgclitcapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1874usize) as _) }
    }
    #[doc = "Stream gate control list index table memory allocation register"]
    #[inline(always)]
    pub const fn sgclitmar(self) -> crate::common::Reg<regs::Sgclitmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1878usize) as _) }
    }
    #[doc = "Stream gate control list table memory operational register"]
    #[inline(always)]
    pub const fn sgcltmor(self) -> crate::common::Reg<regs::Sgcltmor, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x187cusize) as _) }
    }
    #[doc = "Time gate scheduling table capability register"]
    #[inline(always)]
    pub const fn tgstcapr(self) -> crate::common::Reg<regs::Tgstcapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18d4usize) as _) }
    }
    #[doc = "Time gate scheduling table memory operation register"]
    #[inline(always)]
    pub const fn tgstmor(self) -> crate::common::Reg<regs::Tgstmor, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18dcusize) as _) }
    }
    #[doc = "Hash table memory capability register"]
    #[inline(always)]
    pub const fn htmcapr(self) -> crate::common::Reg<regs::Htmcapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1900usize) as _) }
    }
    #[doc = "Hash table memory operational register"]
    #[inline(always)]
    pub const fn htmor(self) -> crate::common::Reg<regs::Htmor, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1904usize) as _) }
    }
    #[doc = "Ingress stream identification capability register"]
    #[inline(always)]
    pub const fn isidcapr(self) -> crate::common::Reg<regs::Isidcapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1910usize) as _) }
    }
    #[doc = "Ingress stream identification hash table capability register"]
    #[inline(always)]
    pub const fn isidhtcapr(self) -> crate::common::Reg<regs::Isidhtcapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1914usize) as _) }
    }
    #[doc = "Ingress stream identification key construction 0 operational register"]
    #[inline(always)]
    pub const fn isidkc0or(self) -> crate::common::Reg<regs::Isidkc0or, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1920usize) as _) }
    }
    #[doc = "Ingress stream identification key construction 0 configuration register 0"]
    #[inline(always)]
    pub const fn isidkc0cr0(self) -> crate::common::Reg<regs::Isidkc0cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1924usize) as _) }
    }
    #[doc = "Ingress stream identification key construction 0 payload field 0 configuration register"]
    #[inline(always)]
    pub const fn isidkc0pf0cr(self) -> crate::common::Reg<regs::Isidkc0pf0cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1930usize) as _) }
    }
    #[doc = "Ingress stream identification key construction 0 payload field 1 configuration register"]
    #[inline(always)]
    pub const fn isidkc0pf1cr(self) -> crate::common::Reg<regs::Isidkc0pf1cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1934usize) as _) }
    }
    #[doc = "Ingress stream identification key construction 0 payload field 2 configuration register"]
    #[inline(always)]
    pub const fn isidkc0pf2cr(self) -> crate::common::Reg<regs::Isidkc0pf2cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1938usize) as _) }
    }
    #[doc = "Ingress stream identification key construction 0 payload field 3 configuration register"]
    #[inline(always)]
    pub const fn isidkc0pf3cr(self) -> crate::common::Reg<regs::Isidkc0pf3cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x193cusize) as _) }
    }
    #[doc = "Ingress stream identification key construction 1 operational register"]
    #[inline(always)]
    pub const fn isidkc1or(self) -> crate::common::Reg<regs::Isidkc1or, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1940usize) as _) }
    }
    #[doc = "Ingress stream identification key construction 1 configuration register 0"]
    #[inline(always)]
    pub const fn isidkc1cr0(self) -> crate::common::Reg<regs::Isidkc1cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1944usize) as _) }
    }
    #[doc = "Ingress stream identification key construction 1 payload field 0 configuration register"]
    #[inline(always)]
    pub const fn isidkc1pf0cr(self) -> crate::common::Reg<regs::Isidkc1pf0cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1950usize) as _) }
    }
    #[doc = "Ingress stream identification key construction 1 payload field 1 configuration register"]
    #[inline(always)]
    pub const fn isidkc1pf1cr(self) -> crate::common::Reg<regs::Isidkc1pf1cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1954usize) as _) }
    }
    #[doc = "Ingress stream identification key construction 1 payload field 2 configuration register"]
    #[inline(always)]
    pub const fn isidkc1pf2cr(self) -> crate::common::Reg<regs::Isidkc1pf2cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1958usize) as _) }
    }
    #[doc = "Ingress stream identification key construction 1 payload field 3 configuration register"]
    #[inline(always)]
    pub const fn isidkc1pf3cr(self) -> crate::common::Reg<regs::Isidkc1pf3cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x195cusize) as _) }
    }
    #[doc = "Ingress stream filter hash table capability register"]
    #[inline(always)]
    pub const fn isfhtcapr(self) -> crate::common::Reg<regs::Isfhtcapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1a00usize) as _) }
    }
    #[doc = "Ingress stream filter hash table operational register"]
    #[inline(always)]
    pub const fn isfhtor(self) -> crate::common::Reg<regs::Isfhtor, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1a04usize) as _) }
    }
}
pub mod regs;
pub mod vals;
