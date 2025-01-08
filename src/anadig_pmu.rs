#[doc = "RT1180_ANADIG_REGISTER"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AnadigPmu {
    ptr: *mut u8,
}
unsafe impl Send for AnadigPmu {}
unsafe impl Sync for AnadigPmu {}
impl AnadigPmu {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "PMU_BIAS_CTRL_REGISTER"]
    #[inline(always)]
    pub const fn pmu_bias_ctrl(self) -> crate::common::Reg<regs::PmuBiasCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4600usize) as _) }
    }
    #[doc = "PMU_BIAS_CTRL2_REGISTER"]
    #[inline(always)]
    pub const fn pmu_bias_ctrl2(self) -> crate::common::Reg<regs::PmuBiasCtrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4610usize) as _) }
    }
    #[doc = "PMU_LDO_PLL_REGISTER"]
    #[inline(always)]
    pub const fn pmu_ldo_pll(self) -> crate::common::Reg<regs::PmuLdoPll, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4640usize) as _) }
    }
    #[doc = "PMU_POWER_DETECT_CTRL_REGISTER"]
    #[inline(always)]
    pub const fn pmu_power_detect_ctrl(
        self,
    ) -> crate::common::Reg<regs::PmuPowerDetectCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4700usize) as _) }
    }
    #[doc = "PMU_REF_CTRL_REGISTER"]
    #[inline(always)]
    pub const fn pmu_ref_ctrl(self) -> crate::common::Reg<regs::PmuRefCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4710usize) as _) }
    }
}
pub mod regs;
pub mod vals;
