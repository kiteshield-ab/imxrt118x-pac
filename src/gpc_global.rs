#[doc = "no description available"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpcGlobal {
    ptr: *mut u8,
}
unsafe impl Send for GpcGlobal {}
unsafe impl Sync for GpcGlobal {}
impl GpcGlobal {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "GPC Global Authentication Control"]
    #[inline(always)]
    pub const fn authen_ctrl(self) -> crate::common::Reg<regs::AuthenCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "GPC CPU0 domain assignment"]
    #[inline(always)]
    pub const fn gpc_cpu0_domain(
        self,
    ) -> crate::common::Reg<regs::GpcCpu0Domain, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "GPC CPU1 domain assignment"]
    #[inline(always)]
    pub const fn gpc_cpu1_domain(
        self,
    ) -> crate::common::Reg<regs::GpcCpu1Domain, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "GPC master CPU configuration"]
    #[inline(always)]
    pub const fn gpc_master(self) -> crate::common::Reg<regs::GpcMaster, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "RCOSC control"]
    #[inline(always)]
    pub const fn gpc_rosc_ctrl(self) -> crate::common::Reg<regs::GpcRoscCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
}
pub mod regs;
pub mod vals;
