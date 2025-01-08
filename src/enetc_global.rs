#[doc = "NETC global"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EnetcGlobal {
    ptr: *mut u8,
}
unsafe impl Send for EnetcGlobal {}
unsafe impl Sync for EnetcGlobal {}
impl EnetcGlobal {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Shared memory capability register"]
    #[inline(always)]
    pub const fn smcapr(self) -> crate::common::Reg<regs::Smcapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Shared memory depletion threshold register"]
    #[inline(always)]
    pub const fn smdtr(self) -> crate::common::Reg<regs::Smdtr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Shared memory available count register"]
    #[inline(always)]
    pub const fn smacr(self) -> crate::common::Reg<regs::Smacr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Shared memory count low watermark register"]
    #[inline(always)]
    pub const fn smclwmr(self) -> crate::common::Reg<regs::Smclwmr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Shared memory buffer unassigned count register"]
    #[inline(always)]
    pub const fn smbucr(self) -> crate::common::Reg<regs::Smbucr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Shared memory buffer unassigned count high watermark register"]
    #[inline(always)]
    pub const fn smbuchwmr(self) -> crate::common::Reg<regs::Smbuchwmr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Shared memory loss count register"]
    #[inline(always)]
    pub const fn smlcr(self) -> crate::common::Reg<regs::Smlcr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Hash bucket table capability register"]
    #[inline(always)]
    pub const fn hbtcapr(self) -> crate::common::Reg<regs::Hbtcapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Hash bucket table operational register 0"]
    #[inline(always)]
    pub const fn hbtor0(self) -> crate::common::Reg<regs::Hbtor0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Hash bucket table operational register 2"]
    #[inline(always)]
    pub const fn hbtor2(self) -> crate::common::Reg<regs::Hbtor2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Shared memory ENETC receive buffer capability register"]
    #[inline(always)]
    pub const fn smerbcapr(self) -> crate::common::Reg<regs::Smerbcapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "Shared memory ENETC receive buffer operational register 0"]
    #[inline(always)]
    pub const fn smerbor0(self) -> crate::common::Reg<regs::Smerbor0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "Shared memory ENETC receive buffer operational 1"]
    #[inline(always)]
    pub const fn smerbor1(self) -> crate::common::Reg<regs::Smerbor1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "PCE 0 operational register"]
    #[inline(always)]
    pub const fn pce0or(self) -> crate::common::Reg<regs::Pce0or, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "Replication Forwarding Engine 0 operational register"]
    #[inline(always)]
    pub const fn rfe0or(self) -> crate::common::Reg<regs::Rfe0or, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "NETC clock register"]
    #[inline(always)]
    pub const fn netcclkr(self) -> crate::common::Reg<regs::Netcclkr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0164usize) as _) }
    }
    #[doc = "HTA 0 capability register"]
    #[inline(always)]
    pub const fn hta0capr(self) -> crate::common::Reg<regs::Hta0capr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[doc = "HTA 0 receive frame count operational register"]
    #[inline(always)]
    pub const fn hta0rfcor(self) -> crate::common::Reg<regs::Hta0rfcor, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0204usize) as _) }
    }
    #[doc = "HTA 0 high priority byte count operational register"]
    #[inline(always)]
    pub const fn hta0hpbcor(self) -> crate::common::Reg<regs::Hta0hpbcor, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0208usize) as _) }
    }
    #[doc = "HTA 0 low priority byte count operational register"]
    #[inline(always)]
    pub const fn hta0lpbcor(self) -> crate::common::Reg<regs::Hta0lpbcor, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x020cusize) as _) }
    }
    #[doc = "HTA 0 transmit frame count operational register"]
    #[inline(always)]
    pub const fn hta0tfcor(self) -> crate::common::Reg<regs::Hta0tfcor, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0224usize) as _) }
    }
    #[doc = "Root complex 0 system bus read latency average register"]
    #[inline(always)]
    pub const fn rc0sbrlar(self) -> crate::common::Reg<regs::Rc0sbrlar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
    }
    #[doc = "Root complex 0 system bus read latency high watermark register"]
    #[inline(always)]
    pub const fn rc0sbrlhwmr(self) -> crate::common::Reg<regs::Rc0sbrlhwmr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
    }
    #[doc = "Root complex 0 system bus write latency average register"]
    #[inline(always)]
    pub const fn rc0sbwlar(self) -> crate::common::Reg<regs::Rc0sbwlar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
    }
    #[doc = "Root complex 0 system bus write latency high watermark register"]
    #[inline(always)]
    pub const fn rc0sbwlhwmr(self) -> crate::common::Reg<regs::Rc0sbwlhwmr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x030cusize) as _) }
    }
    #[doc = "IP block revision register 0"]
    #[inline(always)]
    pub const fn ipbrr0(self) -> crate::common::Reg<regs::Ipbrr0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0bf8usize) as _) }
    }
    #[doc = "IP block revision register 1"]
    #[inline(always)]
    pub const fn ipbrr1(self) -> crate::common::Reg<regs::Ipbrr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0bfcusize) as _) }
    }
    #[doc = "Function boot loader parameter register a"]
    #[inline(always)]
    pub const fn fblpr(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d00usize + n * 4usize) as _) }
    }
}
pub mod regs;
