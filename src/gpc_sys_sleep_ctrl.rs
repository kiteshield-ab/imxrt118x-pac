#[doc = "no description available"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpcSysSleepCtrl {
    ptr: *mut u8,
}
unsafe impl Send for GpcSysSleepCtrl {}
unsafe impl Sync for GpcSysSleepCtrl {}
impl GpcSysSleepCtrl {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "System Sleep Authentication Control"]
    #[inline(always)]
    pub const fn ss_authen_ctrl(self) -> crate::common::Reg<regs::SsAuthenCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "System Sleep Misc"]
    #[inline(always)]
    pub const fn ss_misc(self) -> crate::common::Reg<regs::SsMisc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "PMIC standby control"]
    #[inline(always)]
    pub const fn pmic_ctrl(self) -> crate::common::Reg<regs::PmicCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "System Sleep STEP0 (BIAS) in control"]
    #[inline(always)]
    pub const fn ss_step0_in_ctrl(
        self,
    ) -> crate::common::Reg<regs::SsStep0InCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
    }
    #[doc = "System Sleep STEP1 (PLDO) in control"]
    #[inline(always)]
    pub const fn ss_step1_in_ctrl(
        self,
    ) -> crate::common::Reg<regs::SsStep1InCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "System Sleep STEP2 (BANDGAP) in control"]
    #[inline(always)]
    pub const fn ss_step2_in_ctrl(
        self,
    ) -> crate::common::Reg<regs::SsStep2InCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "System Sleep STEP3 (LDO) in control"]
    #[inline(always)]
    pub const fn ss_step3_in_ctrl(
        self,
    ) -> crate::common::Reg<regs::SsStep3InCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
    #[doc = "System Sleep DCDC in control"]
    #[inline(always)]
    pub const fn ss_dcdc_in_ctrl(
        self,
    ) -> crate::common::Reg<regs::SsDcdcInCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[doc = "System Sleep PMIC in control"]
    #[inline(always)]
    pub const fn ss_pmic_in_ctrl(
        self,
    ) -> crate::common::Reg<regs::SsPmicInCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0150usize) as _) }
    }
    #[doc = "System Sleep PMIC out control"]
    #[inline(always)]
    pub const fn ss_pmic_out_ctrl(
        self,
    ) -> crate::common::Reg<regs::SsPmicOutCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[doc = "System Sleep DCDC out control"]
    #[inline(always)]
    pub const fn ss_dcdc_out_ctrl(
        self,
    ) -> crate::common::Reg<regs::SsDcdcOutCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0210usize) as _) }
    }
    #[doc = "System Sleep STEP3 (LDO) out control"]
    #[inline(always)]
    pub const fn ss_step3_out_ctrl(
        self,
    ) -> crate::common::Reg<regs::SsStep3OutCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0238usize) as _) }
    }
    #[doc = "System Sleep STEP2 (BANDGAP) out control"]
    #[inline(always)]
    pub const fn ss_step2_out_ctrl(
        self,
    ) -> crate::common::Reg<regs::SsStep2OutCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0240usize) as _) }
    }
    #[doc = "System Sleep STEP1 (PLDO) out control"]
    #[inline(always)]
    pub const fn ss_step1_out_ctrl(
        self,
    ) -> crate::common::Reg<regs::SsStep1OutCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0250usize) as _) }
    }
    #[doc = "System Sleep STEP0 (BIAS) out control"]
    #[inline(always)]
    pub const fn ss_step0_out_ctrl(
        self,
    ) -> crate::common::Reg<regs::SsStep0OutCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0260usize) as _) }
    }
}
pub mod regs;
pub mod vals;
