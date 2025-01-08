#[doc = "RT1180_ANADIG_REGISTER"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AnadigLdoBbsm {
    ptr: *mut u8,
}
unsafe impl Send for AnadigLdoBbsm {}
unsafe impl Sync for AnadigLdoBbsm {}
impl AnadigLdoBbsm {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "PMU_LDO_AON_ANA_REGISTER"]
    #[inline(always)]
    pub const fn pmu_ldo_aon_ana(
        self,
    ) -> crate::common::Reg<regs::PmuLdoAonAna, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4740usize) as _) }
    }
    #[doc = "PMU_LDO_AON_DIG_REGISTER"]
    #[inline(always)]
    pub const fn pmu_ldo_aon_dig(
        self,
    ) -> crate::common::Reg<regs::PmuLdoAonDig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4760usize) as _) }
    }
}
pub mod regs;
pub mod vals;
