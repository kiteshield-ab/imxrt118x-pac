#[doc = "NETC Integrated Endpoint Register Block"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcIerb {
    ptr: *mut u8,
}
unsafe impl Send for NetcIerb {}
unsafe impl Sync for NetcIerb {}
impl NetcIerb {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Capability register 0"]
    #[inline(always)]
    pub const fn capr0(self) -> crate::common::Reg<regs::Capr0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Capability register 1"]
    #[inline(always)]
    pub const fn capr1(self) -> crate::common::Reg<regs::Capr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Capability register 2"]
    #[inline(always)]
    pub const fn capr2(self) -> crate::common::Reg<regs::Capr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Capability register 3"]
    #[inline(always)]
    pub const fn capr3(self) -> crate::common::Reg<regs::Capr3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Common memory capability register"]
    #[inline(always)]
    pub const fn cmcapr(self) -> crate::common::Reg<regs::Cmcapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Ingress port filter ternary memory capability register"]
    #[inline(always)]
    pub const fn ipftmcapr(self) -> crate::common::Reg<regs::Ipftmcapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Time gate scheduling memory capability register"]
    #[inline(always)]
    pub const fn tgsmcapr(self) -> crate::common::Reg<regs::Tgsmcapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "Shared memory depletion threshold register"]
    #[inline(always)]
    pub const fn smdtr(self) -> crate::common::Reg<regs::Smdtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "ENETC receive shared memory buffer allotment register"]
    #[inline(always)]
    pub const fn ersmbar(self) -> crate::common::Reg<regs::Ersmbar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "HTA 0 HP configuration register"]
    #[inline(always)]
    pub const fn hta0hpcr(self) -> crate::common::Reg<regs::Hta0hpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[doc = "HTA 0 LP configuration register"]
    #[inline(always)]
    pub const fn hta0lpcr(self) -> crate::common::Reg<regs::Hta0lpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[doc = "Hash bucket table memory allocation register"]
    #[inline(always)]
    pub const fn hbtmar(self) -> crate::common::Reg<regs::Hbtmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "Hash bucket table configuration register"]
    #[inline(always)]
    pub const fn hbtcr(self) -> crate::common::Reg<regs::Hbtcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "Guaranteed hash table entry memory capability register"]
    #[inline(always)]
    pub const fn ghtemcapr(self) -> crate::common::Reg<regs::Ghtemcapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "NETC FLR configuration register"]
    #[inline(always)]
    pub const fn netcflrcr(self) -> crate::common::Reg<regs::Netcflrcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0170usize) as _) }
    }
    #[doc = "NETC clock period fractional register"]
    #[inline(always)]
    pub const fn netcclkfr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0178usize) as _) }
    }
    #[doc = "NETC clock configuration register"]
    #[inline(always)]
    pub const fn netcclkcr(self) -> crate::common::Reg<regs::Netcclkcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x017cusize) as _) }
    }
    #[doc = "System bus configuration register"]
    #[inline(always)]
    pub const fn sbcr(self) -> crate::common::Reg<regs::Sbcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
    }
    #[doc = "System bus outstanding transaction control register"]
    #[inline(always)]
    pub const fn sbotcr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
    }
    #[doc = "Stream gating lag time for refresh register"]
    #[inline(always)]
    pub const fn sglttr(self) -> crate::common::Reg<regs::Sglttr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0190usize) as _) }
    }
    #[doc = "Root complex 0 binding configuration register"]
    #[inline(always)]
    pub const fn r0bcr(self) -> crate::common::Reg<regs::R0bcr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[doc = "Root complex 0 MSI-X cache attribute register"]
    #[inline(always)]
    pub const fn rc0msicar(self) -> crate::common::Reg<regs::Rc0msicar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0208usize) as _) }
    }
    #[doc = "Root complex 0 MSI access management qualifier register"]
    #[inline(always)]
    pub const fn rc0msiamqr(self) -> crate::common::Reg<regs::Rc0msiamqr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x020cusize) as _) }
    }
    #[doc = "EMDIO binding configuration register"]
    #[inline(always)]
    pub const fn emdiobcr(self) -> crate::common::Reg<regs::Emdiobcr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
    }
    #[doc = "EMDIO MSI-X configuration register"]
    #[inline(always)]
    pub const fn emdiomcr(self) -> crate::common::Reg<regs::Emdiomcr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0314usize) as _) }
    }
    #[doc = "EMDIO config header device ID and vendor ID register"]
    #[inline(always)]
    pub const fn emdio_cfh_didvid(
        self,
    ) -> crate::common::Reg<regs::EmdioCfhDidvid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0320usize) as _) }
    }
    #[doc = "EMDIO config header subsystem ID and subsystem vendor ID register"]
    #[inline(always)]
    pub const fn emdio_cfh_sidsvid(
        self,
    ) -> crate::common::Reg<regs::EmdioCfhSidsvid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0324usize) as _) }
    }
    #[doc = "EMDIO boot loader parameter register a"]
    #[inline(always)]
    pub const fn emdioblpr(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0348usize + n * 4usize) as _) }
    }
    #[doc = "EMDIO configuration register"]
    #[inline(always)]
    pub const fn emdio_cfg(self) -> crate::common::Reg<regs::EmdioCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0350usize) as _) }
    }
    #[doc = "Timer 0 binding configuration register"]
    #[inline(always)]
    pub const fn t0bcr(self) -> crate::common::Reg<regs::T0bcr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
    #[doc = "Timer 0 MSI-X configuration register"]
    #[inline(always)]
    pub const fn t0mcr(self) -> crate::common::Reg<regs::T0mcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0414usize) as _) }
    }
    #[doc = "Timer 0 config header device ID and vendor ID register"]
    #[inline(always)]
    pub const fn t0_cfh_didvid(self) -> crate::common::Reg<regs::T0CfhDidvid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0420usize) as _) }
    }
    #[doc = "Timer 0 config header subsystem ID and subsystem vendor ID register"]
    #[inline(always)]
    pub const fn t0_cfh_sidsvid(self) -> crate::common::Reg<regs::T0CfhSidsvid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0424usize) as _) }
    }
    #[doc = "Timer 0 boot loader parameter register 0"]
    #[inline(always)]
    pub const fn t0blpr0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0448usize) as _) }
    }
    #[doc = "Timer 0 boot loader parameter register 1"]
    #[inline(always)]
    pub const fn t0blpr1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x044cusize) as _) }
    }
    #[doc = "Link 0 capability register"]
    #[inline(always)]
    pub const fn l0capr(self) -> crate::common::Reg<regs::L0capr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1000usize) as _) }
    }
    #[doc = "Link 0 MAC capability register"]
    #[inline(always)]
    pub const fn l0mcapr(self) -> crate::common::Reg<regs::L0mcapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1004usize) as _) }
    }
    #[doc = "Link 0 I/O capability register"]
    #[inline(always)]
    pub const fn l0iocapr(self) -> crate::common::Reg<regs::L0iocapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1008usize) as _) }
    }
    #[doc = "Link 0 binding configuration register"]
    #[inline(always)]
    pub const fn l0bcr(self) -> crate::common::Reg<regs::L0bcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1010usize) as _) }
    }
    #[doc = "Link 0 transmit byte credit comfort threshold register"]
    #[inline(always)]
    pub const fn l0txbcctr(self) -> crate::common::Reg<regs::L0txbcctr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1014usize) as _) }
    }
    #[doc = "Link 0 end 0 MAC address register 0"]
    #[inline(always)]
    pub const fn l0e0mar0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1020usize) as _) }
    }
    #[doc = "Link 0 end 0 MAC address register 1"]
    #[inline(always)]
    pub const fn l0e0mar1(self) -> crate::common::Reg<regs::L0e0mar1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1024usize) as _) }
    }
    #[doc = "Link 1 capability register"]
    #[inline(always)]
    pub const fn l1capr(self) -> crate::common::Reg<regs::L1capr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1040usize) as _) }
    }
    #[doc = "Link 1 MAC capability register"]
    #[inline(always)]
    pub const fn l1mcapr(self) -> crate::common::Reg<regs::L1mcapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1044usize) as _) }
    }
    #[doc = "Link 1 I/O capability register"]
    #[inline(always)]
    pub const fn l1iocapr(self) -> crate::common::Reg<regs::L1iocapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1048usize) as _) }
    }
    #[doc = "Link 1 binding configuration register"]
    #[inline(always)]
    pub const fn l1bcr(self) -> crate::common::Reg<regs::L1bcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1050usize) as _) }
    }
    #[doc = "Link 1 transmit byte credit comfort threshold register"]
    #[inline(always)]
    pub const fn l1txbcctr(self) -> crate::common::Reg<regs::L1txbcctr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1054usize) as _) }
    }
    #[doc = "Link 1 end 0 MAC address register 0"]
    #[inline(always)]
    pub const fn l1e0mar0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1060usize) as _) }
    }
    #[doc = "Link 1 end 0 MAC address register 1"]
    #[inline(always)]
    pub const fn l1e0mar1(self) -> crate::common::Reg<regs::L1e0mar1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1064usize) as _) }
    }
    #[doc = "Link 2 capability register"]
    #[inline(always)]
    pub const fn l2capr(self) -> crate::common::Reg<regs::L2capr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1080usize) as _) }
    }
    #[doc = "Link 2 MAC capability register"]
    #[inline(always)]
    pub const fn l2mcapr(self) -> crate::common::Reg<regs::L2mcapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1084usize) as _) }
    }
    #[doc = "Link 2 I/O capability register"]
    #[inline(always)]
    pub const fn l2iocapr(self) -> crate::common::Reg<regs::L2iocapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1088usize) as _) }
    }
    #[doc = "Link 2 binding configuration register"]
    #[inline(always)]
    pub const fn l2bcr(self) -> crate::common::Reg<regs::L2bcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1090usize) as _) }
    }
    #[doc = "Link 2 transmit byte credit comfort threshold register"]
    #[inline(always)]
    pub const fn l2txbcctr(self) -> crate::common::Reg<regs::L2txbcctr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1094usize) as _) }
    }
    #[doc = "Link 2 end 0 MAC address register 0"]
    #[inline(always)]
    pub const fn l2e0mar0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10a0usize) as _) }
    }
    #[doc = "Link 2 end 0 MAC address register 1"]
    #[inline(always)]
    pub const fn l2e0mar1(self) -> crate::common::Reg<regs::L2e0mar1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10a4usize) as _) }
    }
    #[doc = "Link 3 capability register"]
    #[inline(always)]
    pub const fn l3capr(self) -> crate::common::Reg<regs::L3capr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10c0usize) as _) }
    }
    #[doc = "Link 3 MAC capability register"]
    #[inline(always)]
    pub const fn l3mcapr(self) -> crate::common::Reg<regs::L3mcapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10c4usize) as _) }
    }
    #[doc = "Link 3 I/O capability register"]
    #[inline(always)]
    pub const fn l3iocapr(self) -> crate::common::Reg<regs::L3iocapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10c8usize) as _) }
    }
    #[doc = "Link 3 binding configuration register"]
    #[inline(always)]
    pub const fn l3bcr(self) -> crate::common::Reg<regs::L3bcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10d0usize) as _) }
    }
    #[doc = "Link 3 transmit byte credit comfort threshold register"]
    #[inline(always)]
    pub const fn l3txbcctr(self) -> crate::common::Reg<regs::L3txbcctr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10d4usize) as _) }
    }
    #[doc = "Link 3 end 0 MAC address register 0"]
    #[inline(always)]
    pub const fn l3e0mar0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10e0usize) as _) }
    }
    #[doc = "Link 3 end 0 MAC address register 1"]
    #[inline(always)]
    pub const fn l3e0mar1(self) -> crate::common::Reg<regs::L3e0mar1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10e4usize) as _) }
    }
    #[doc = "Link 4 capability register"]
    #[inline(always)]
    pub const fn l4capr(self) -> crate::common::Reg<regs::L4capr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1100usize) as _) }
    }
    #[doc = "Link 4 MAC capability register"]
    #[inline(always)]
    pub const fn l4mcapr(self) -> crate::common::Reg<regs::L4mcapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1104usize) as _) }
    }
    #[doc = "Link 4 I/O capability register"]
    #[inline(always)]
    pub const fn l4iocapr(self) -> crate::common::Reg<regs::L4iocapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1108usize) as _) }
    }
    #[doc = "Link 4 binding configuration register"]
    #[inline(always)]
    pub const fn l4bcr(self) -> crate::common::Reg<regs::L4bcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1110usize) as _) }
    }
    #[doc = "Link 4 transmit byte credit comfort threshold register"]
    #[inline(always)]
    pub const fn l4txbcctr(self) -> crate::common::Reg<regs::L4txbcctr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1114usize) as _) }
    }
    #[doc = "Link 4 end 0 MAC address register 0"]
    #[inline(always)]
    pub const fn l4e0mar0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1120usize) as _) }
    }
    #[doc = "Link 4 end 0 MAC address register 1"]
    #[inline(always)]
    pub const fn l4e0mar1(self) -> crate::common::Reg<regs::L4e0mar1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1124usize) as _) }
    }
    #[doc = "Link 5 capability register"]
    #[inline(always)]
    pub const fn l5capr(self) -> crate::common::Reg<regs::L5capr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1140usize) as _) }
    }
    #[doc = "Link 5 MAC capability register"]
    #[inline(always)]
    pub const fn l5mcapr(self) -> crate::common::Reg<regs::L5mcapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1144usize) as _) }
    }
    #[doc = "Link 5 binding configuration register"]
    #[inline(always)]
    pub const fn l5bcr(self) -> crate::common::Reg<regs::L5bcr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1150usize) as _) }
    }
    #[doc = "Link 5 transmit byte credit comfort threshold register"]
    #[inline(always)]
    pub const fn l5txbcctr(self) -> crate::common::Reg<regs::L5txbcctr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1154usize) as _) }
    }
    #[doc = "Link 5 end 0 MAC address register 0"]
    #[inline(always)]
    pub const fn l5e0mar0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1160usize) as _) }
    }
    #[doc = "Link 5 end 0 MAC address register 1"]
    #[inline(always)]
    pub const fn l5e0mar1(self) -> crate::common::Reg<regs::L5e0mar1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1164usize) as _) }
    }
    #[doc = "Link 5 end 1 MAC address register 0"]
    #[inline(always)]
    pub const fn l5e1mar0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1168usize) as _) }
    }
    #[doc = "Link 5 end 1 MAC address register 1"]
    #[inline(always)]
    pub const fn l5e1mar1(self) -> crate::common::Reg<regs::L5e1mar1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x116cusize) as _) }
    }
    #[doc = "Switch 0 binding configuration register"]
    #[inline(always)]
    pub const fn s0bcr(self) -> crate::common::Reg<regs::S0bcr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2000usize) as _) }
    }
    #[doc = "Switch 0 MSI-X configuration register"]
    #[inline(always)]
    pub const fn s0mcr(self) -> crate::common::Reg<regs::S0mcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2014usize) as _) }
    }
    #[doc = "Switch 0 config header device ID and vendor ID register"]
    #[inline(always)]
    pub const fn s0_cfh_didvid(self) -> crate::common::Reg<regs::S0CfhDidvid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2020usize) as _) }
    }
    #[doc = "Switch 0 config header subsystem ID and subsystem vendor ID register"]
    #[inline(always)]
    pub const fn s0_cfh_sidsvid(self) -> crate::common::Reg<regs::S0CfhSidsvid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2024usize) as _) }
    }
    #[doc = "Switch 0 command cache attribute register"]
    #[inline(always)]
    pub const fn s0ccar(self) -> crate::common::Reg<regs::S0ccar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2038usize) as _) }
    }
    #[doc = "Switch 0 access management qualifier register"]
    #[inline(always)]
    pub const fn s0amqr(self) -> crate::common::Reg<regs::S0amqr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2040usize) as _) }
    }
    #[doc = "Switch 0 boot loader parameter register 0"]
    #[inline(always)]
    pub const fn s0blpr0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2048usize) as _) }
    }
    #[doc = "Switch 0 boot loader parameter register 1"]
    #[inline(always)]
    pub const fn s0blpr1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x204cusize) as _) }
    }
    #[doc = "Switch 0 shared memory buffer allotment register"]
    #[inline(always)]
    pub const fn s0smbar(self) -> crate::common::Reg<regs::S0smbar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2060usize) as _) }
    }
    #[doc = "Switch 0 hash table memory allotment register"]
    #[inline(always)]
    pub const fn s0htmar(self) -> crate::common::Reg<regs::S0htmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2080usize) as _) }
    }
    #[doc = "Switch 0 index table memory allocation register"]
    #[inline(always)]
    pub const fn s0itmar(self) -> crate::common::Reg<regs::S0itmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2084usize) as _) }
    }
    #[doc = "Switch 0 ingress port filter table memory allocation register"]
    #[inline(always)]
    pub const fn s0ipftmar(self) -> crate::common::Reg<regs::S0ipftmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2088usize) as _) }
    }
    #[doc = "Switch 0 rate policer index table memory allocation register"]
    #[inline(always)]
    pub const fn s0rpitmar(self) -> crate::common::Reg<regs::S0rpitmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20a0usize) as _) }
    }
    #[doc = "Switch 0 ingress stream counter index table memory allocation register"]
    #[inline(always)]
    pub const fn s0iscitmar(self) -> crate::common::Reg<regs::S0iscitmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20a4usize) as _) }
    }
    #[doc = "Switch 0 ingress stream index table memory allocation register"]
    #[inline(always)]
    pub const fn s0isitmar(self) -> crate::common::Reg<regs::S0isitmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20a8usize) as _) }
    }
    #[doc = "Switch 0 ingress sequence generation index table memory allocation register"]
    #[inline(always)]
    pub const fn s0isqgitmar(self) -> crate::common::Reg<regs::S0isqgitmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20acusize) as _) }
    }
    #[doc = "Switch 0 stream gate instance index table memory allocation register"]
    #[inline(always)]
    pub const fn s0sgiitmar(self) -> crate::common::Reg<regs::S0sgiitmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20b4usize) as _) }
    }
    #[doc = "Switch 0 stream gate control list index table memory allocation register"]
    #[inline(always)]
    pub const fn s0sgclitmar(self) -> crate::common::Reg<regs::S0sgclitmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20b8usize) as _) }
    }
    #[doc = "Switch 0 frame modification index table memory allocation register"]
    #[inline(always)]
    pub const fn s0fmitmar(self) -> crate::common::Reg<regs::S0fmitmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20bcusize) as _) }
    }
    #[doc = "Switch 0 frame modification data index table memory allocation register"]
    #[inline(always)]
    pub const fn s0fmditmar(self) -> crate::common::Reg<regs::S0fmditmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20c0usize) as _) }
    }
    #[doc = "Switch 0 time gate scheduling table allocation register"]
    #[inline(always)]
    pub const fn s0tgstar(self) -> crate::common::Reg<regs::S0tgstar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20f0usize) as _) }
    }
    #[doc = "Switch 0 time gate scheduling lookahead register"]
    #[inline(always)]
    pub const fn s0tgslr(self) -> crate::common::Reg<regs::S0tgslr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20f4usize) as _) }
    }
    #[doc = "Switch 0 management port configuration register"]
    #[inline(always)]
    pub const fn s0mpcr(self) -> crate::common::Reg<regs::S0mpcr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2204usize) as _) }
    }
    #[doc = "Switch 0 VLAN Filter (hash) table default entry configuration registers 0"]
    #[inline(always)]
    pub const fn s0vfhtdecr0(self) -> crate::common::Reg<regs::S0vfhtdecr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2210usize) as _) }
    }
    #[doc = "Switch 0 VLAN filter hash table default entry configuration registers 1"]
    #[inline(always)]
    pub const fn s0vfhtdecr1(self) -> crate::common::Reg<regs::S0vfhtdecr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2214usize) as _) }
    }
    #[doc = "Switch 0 VLAN filter hash table default entry configuration registers 2"]
    #[inline(always)]
    pub const fn s0vfhtdecr2(self) -> crate::common::Reg<regs::S0vfhtdecr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2218usize) as _) }
    }
    #[doc = "ENETC 0 binding configuration register 0"]
    #[inline(always)]
    pub const fn e0bcr0(self) -> crate::common::Reg<regs::E0bcr0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3000usize) as _) }
    }
    #[doc = "ENETC 0 binding configuration register 1"]
    #[inline(always)]
    pub const fn e0bcr1(self) -> crate::common::Reg<regs::E0bcr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3004usize) as _) }
    }
    #[doc = "ENETC 0 binding configuration register 2"]
    #[inline(always)]
    pub const fn e0bcr2(self) -> crate::common::Reg<regs::E0bcr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3008usize) as _) }
    }
    #[doc = "ENETC 0 VSI binding configuration register"]
    #[inline(always)]
    pub const fn e0vbcr(self) -> crate::common::Reg<regs::E0vbcr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3010usize) as _) }
    }
    #[doc = "ENETC 0 MSI-X configuration register"]
    #[inline(always)]
    pub const fn e0mcr(self) -> crate::common::Reg<regs::E0mcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3014usize) as _) }
    }
    #[doc = "ENETC 0 config header device ID and vendor ID register"]
    #[inline(always)]
    pub const fn e0_cfh_didvid(self) -> crate::common::Reg<regs::E0CfhDidvid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3020usize) as _) }
    }
    #[doc = "ENETC 0 config header subsystem ID and subsystem vendor ID register"]
    #[inline(always)]
    pub const fn e0_cfh_sidsvid(self) -> crate::common::Reg<regs::E0CfhSidsvid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3024usize) as _) }
    }
    #[doc = "ENETC 0 config capability VF device ID register"]
    #[inline(always)]
    pub const fn e0_cfc_vfdid(self) -> crate::common::Reg<regs::E0CfcVfdid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3028usize) as _) }
    }
    #[doc = "ENETC 0 buffer cache attribute register 0"]
    #[inline(always)]
    pub const fn e0bcar(self) -> crate::common::Reg<regs::E0bcar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3030usize) as _) }
    }
    #[doc = "ENETC 0 message cache attribute register"]
    #[inline(always)]
    pub const fn e0mcar(self) -> crate::common::Reg<regs::E0mcar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3034usize) as _) }
    }
    #[doc = "ENETC 0 command cache attribute register"]
    #[inline(always)]
    pub const fn e0car(self) -> crate::common::Reg<regs::E0car, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3038usize) as _) }
    }
    #[doc = "ENETC 0 access management qualifier register"]
    #[inline(always)]
    pub const fn e0amqr(self) -> crate::common::Reg<regs::E0amqr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3040usize) as _) }
    }
    #[doc = "ENETC 0 boot loader parameter register 0"]
    #[inline(always)]
    pub const fn e0blpr0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3048usize) as _) }
    }
    #[doc = "ENETC 0 boot loader parameter register 1"]
    #[inline(always)]
    pub const fn e0blpr1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x304cusize) as _) }
    }
    #[doc = "ENETC 0 receive memory buffer entitlement register"]
    #[inline(always)]
    pub const fn e0rxmber(self) -> crate::common::Reg<regs::E0rxmber, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3050usize) as _) }
    }
    #[doc = "ENETC 0 receive memory buffer limit register"]
    #[inline(always)]
    pub const fn e0rxmblr(self) -> crate::common::Reg<regs::E0rxmblr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3054usize) as _) }
    }
    #[doc = "ENETC 0 transmit high priority tier byte credit register"]
    #[inline(always)]
    pub const fn e0txhptbcr(self) -> crate::common::Reg<regs::E0txhptbcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3070usize) as _) }
    }
    #[doc = "ENETC 0 transmit low priority tier byte credit register"]
    #[inline(always)]
    pub const fn e0txlptbcr(self) -> crate::common::Reg<regs::E0txlptbcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3074usize) as _) }
    }
    #[doc = "ENETC 0 hash table memory allotment register"]
    #[inline(always)]
    pub const fn e0htmar(self) -> crate::common::Reg<regs::E0htmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3080usize) as _) }
    }
    #[doc = "ENETC 0 index table memory allocation register"]
    #[inline(always)]
    pub const fn e0itmar(self) -> crate::common::Reg<regs::E0itmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3084usize) as _) }
    }
    #[doc = "ENETC 0 ingress port filter table memory allocation register"]
    #[inline(always)]
    pub const fn e0ipftmar(self) -> crate::common::Reg<regs::E0ipftmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3088usize) as _) }
    }
    #[doc = "ENETC 0 rate policer index table memory allocation register"]
    #[inline(always)]
    pub const fn e0rpitmar(self) -> crate::common::Reg<regs::E0rpitmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30a0usize) as _) }
    }
    #[doc = "ENETC 0 ingress stream counter index table memory allocation register"]
    #[inline(always)]
    pub const fn e0iscitmar(self) -> crate::common::Reg<regs::E0iscitmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30a4usize) as _) }
    }
    #[doc = "ENETC 0 ingress stream index table memory allocation register"]
    #[inline(always)]
    pub const fn e0isitmar(self) -> crate::common::Reg<regs::E0isitmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30a8usize) as _) }
    }
    #[doc = "ENETC 0 stream gate instance index table memory allocation register"]
    #[inline(always)]
    pub const fn e0sgiitmar(self) -> crate::common::Reg<regs::E0sgiitmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30b4usize) as _) }
    }
    #[doc = "ENETC 0 stream gate control list index table memory allocation register"]
    #[inline(always)]
    pub const fn e0sgclitmar(self) -> crate::common::Reg<regs::E0sgclitmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30b8usize) as _) }
    }
    #[doc = "ENETC 0 time gate scheduling table allocation register"]
    #[inline(always)]
    pub const fn e0tgstar(self) -> crate::common::Reg<regs::E0tgstar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30f0usize) as _) }
    }
    #[doc = "ENETC 0 time gate scheduling lookahead register"]
    #[inline(always)]
    pub const fn e0tgslr(self) -> crate::common::Reg<regs::E0tgslr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30f4usize) as _) }
    }
    #[doc = "ENETC 1 binding configuration register 0"]
    #[inline(always)]
    pub const fn e1bcr0(self) -> crate::common::Reg<regs::E1bcr0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3100usize) as _) }
    }
    #[doc = "ENETC 1 binding configuration register 1"]
    #[inline(always)]
    pub const fn e1bcr1(self) -> crate::common::Reg<regs::E1bcr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3104usize) as _) }
    }
    #[doc = "ENETC 1 binding configuration register 2"]
    #[inline(always)]
    pub const fn e1bcr2(self) -> crate::common::Reg<regs::E1bcr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3108usize) as _) }
    }
    #[doc = "ENETC 1 VSI binding configuration register"]
    #[inline(always)]
    pub const fn e1vbcr(self) -> crate::common::Reg<regs::E1vbcr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3110usize) as _) }
    }
    #[doc = "ENETC 1 MSI-X configuration register"]
    #[inline(always)]
    pub const fn e1mcr(self) -> crate::common::Reg<regs::E1mcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3114usize) as _) }
    }
    #[doc = "ENETC 1 config header device ID and vendor ID register"]
    #[inline(always)]
    pub const fn e1_cfh_didvid(self) -> crate::common::Reg<regs::E1CfhDidvid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3120usize) as _) }
    }
    #[doc = "ENETC 1 config header subsystem ID and subsystem vendor ID register"]
    #[inline(always)]
    pub const fn e1_cfh_sidsvid(self) -> crate::common::Reg<regs::E1CfhSidsvid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3124usize) as _) }
    }
    #[doc = "ENETC 1 config capability VF device ID register"]
    #[inline(always)]
    pub const fn e1_cfc_vfdid(self) -> crate::common::Reg<regs::E1CfcVfdid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3128usize) as _) }
    }
    #[doc = "ENETC 1 buffer cache attribute register 0"]
    #[inline(always)]
    pub const fn e1bcar(self) -> crate::common::Reg<regs::E1bcar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3130usize) as _) }
    }
    #[doc = "ENETC 1 message cache attribute register"]
    #[inline(always)]
    pub const fn e1mcar(self) -> crate::common::Reg<regs::E1mcar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3134usize) as _) }
    }
    #[doc = "ENETC 1 command cache attribute register"]
    #[inline(always)]
    pub const fn e1car(self) -> crate::common::Reg<regs::E1car, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3138usize) as _) }
    }
    #[doc = "ENETC 1 access management qualifier register"]
    #[inline(always)]
    pub const fn e1amqr(self) -> crate::common::Reg<regs::E1amqr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3140usize) as _) }
    }
    #[doc = "ENETC 1 boot loader parameter register 0"]
    #[inline(always)]
    pub const fn e1blpr0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3148usize) as _) }
    }
    #[doc = "ENETC 1 boot loader parameter register 1"]
    #[inline(always)]
    pub const fn e1blpr1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x314cusize) as _) }
    }
    #[doc = "ENETC 1 receive memory buffer entitlement register"]
    #[inline(always)]
    pub const fn e1rxmber(self) -> crate::common::Reg<regs::E1rxmber, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3150usize) as _) }
    }
    #[doc = "ENETC 1 receive memory buffer limit register"]
    #[inline(always)]
    pub const fn e1rxmblr(self) -> crate::common::Reg<regs::E1rxmblr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3154usize) as _) }
    }
    #[doc = "ENETC 1 transmit high priority tier byte credit register"]
    #[inline(always)]
    pub const fn e1txhptbcr(self) -> crate::common::Reg<regs::E1txhptbcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3170usize) as _) }
    }
    #[doc = "ENETC 1 transmit low priority tier byte credit register"]
    #[inline(always)]
    pub const fn e1txlptbcr(self) -> crate::common::Reg<regs::E1txlptbcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3174usize) as _) }
    }
    #[doc = "ENETC 1 hash table memory allotment register"]
    #[inline(always)]
    pub const fn e1htmar(self) -> crate::common::Reg<regs::E1htmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3180usize) as _) }
    }
    #[doc = "ENETC 1 index table memory allocation register"]
    #[inline(always)]
    pub const fn e1itmar(self) -> crate::common::Reg<regs::E1itmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3184usize) as _) }
    }
    #[doc = "ENETC 1 ingress port filter table memory allocation register"]
    #[inline(always)]
    pub const fn e1ipftmar(self) -> crate::common::Reg<regs::E1ipftmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3188usize) as _) }
    }
    #[doc = "ENETC 1 rate policer index table memory allocation register"]
    #[inline(always)]
    pub const fn e1rpitmar(self) -> crate::common::Reg<regs::E1rpitmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x31a0usize) as _) }
    }
    #[doc = "ENETC 1 ingress stream counter index table memory allocation register"]
    #[inline(always)]
    pub const fn e1iscitmar(self) -> crate::common::Reg<regs::E1iscitmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x31a4usize) as _) }
    }
    #[doc = "ENETC 1 ingress stream index table memory allocation register"]
    #[inline(always)]
    pub const fn e1isitmar(self) -> crate::common::Reg<regs::E1isitmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x31a8usize) as _) }
    }
    #[doc = "ENETC 1 stream gate instance index table memory allocation register"]
    #[inline(always)]
    pub const fn e1sgiitmar(self) -> crate::common::Reg<regs::E1sgiitmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x31b4usize) as _) }
    }
    #[doc = "ENETC 1 stream gate control list index table memory allocation register"]
    #[inline(always)]
    pub const fn e1sgclitmar(self) -> crate::common::Reg<regs::E1sgclitmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x31b8usize) as _) }
    }
    #[doc = "ENETC 1 time gate scheduling table allocation register"]
    #[inline(always)]
    pub const fn e1tgstar(self) -> crate::common::Reg<regs::E1tgstar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x31f0usize) as _) }
    }
    #[doc = "ENETC 1 time gate scheduling lookahead register"]
    #[inline(always)]
    pub const fn e1tgslr(self) -> crate::common::Reg<regs::E1tgslr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x31f4usize) as _) }
    }
    #[doc = "VSI 0 access management qualifier register"]
    #[inline(always)]
    pub const fn v0amqr(self) -> crate::common::Reg<regs::V0amqr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4000usize) as _) }
    }
    #[doc = "VSI 0 boot loader parameter register 0"]
    #[inline(always)]
    pub const fn v0blpr0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4008usize) as _) }
    }
    #[doc = "VSI 0 boot loader parameter register 1"]
    #[inline(always)]
    pub const fn v0blpr1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x400cusize) as _) }
    }
    #[doc = "VSI 0 primary MAC address register 0"]
    #[inline(always)]
    pub const fn v0pmar0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4010usize) as _) }
    }
    #[doc = "VSI 0 primary MAC address register 1"]
    #[inline(always)]
    pub const fn v0pmar1(self) -> crate::common::Reg<regs::V0pmar1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4014usize) as _) }
    }
}
pub mod regs;
pub mod vals;
