#[doc = "CCM"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccm {
    ptr: *mut u8,
}
unsafe impl Send for Ccm {}
unsafe impl Sync for Ccm {}
impl Ccm {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Clock root section."]
    #[inline(always)]
    pub const fn clock_root(self, n: usize) -> ClockRoot {
        assert!(n < 74usize);
        unsafe { ClockRoot::from_ptr(self.ptr.add(0x0usize + n * 128usize) as _) }
    }
    #[doc = "Clock root section."]
    #[inline(always)]
    pub const fn observe(self, n: usize) -> Observe {
        assert!(n < 2usize);
        unsafe { Observe::from_ptr(self.ptr.add(0x4400usize + n * 128usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4800usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared0_set(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4804usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared0_clr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4808usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared0_tog(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x480cusize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared0_authen(
        self,
    ) -> crate::common::Reg<regs::GprShared0Authen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4810usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared0_authen_set(
        self,
    ) -> crate::common::Reg<regs::GprShared0AuthenSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4814usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared0_authen_clr(
        self,
    ) -> crate::common::Reg<regs::GprShared0AuthenClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4818usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared0_authen_tog(
        self,
    ) -> crate::common::Reg<regs::GprShared0AuthenTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x481cusize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4820usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared1_set(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4824usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared1_clr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4828usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared1_tog(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x482cusize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared1_authen(
        self,
    ) -> crate::common::Reg<regs::GprShared1Authen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4830usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared1_authen_set(
        self,
    ) -> crate::common::Reg<regs::GprShared1AuthenSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4834usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared1_authen_clr(
        self,
    ) -> crate::common::Reg<regs::GprShared1AuthenClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4838usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared1_authen_tog(
        self,
    ) -> crate::common::Reg<regs::GprShared1AuthenTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x483cusize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared2(self) -> crate::common::Reg<regs::GprShared2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4840usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared2_set(
        self,
    ) -> crate::common::Reg<regs::GprShared2Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4844usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared2_clr(
        self,
    ) -> crate::common::Reg<regs::GprShared2Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4848usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared2_tog(
        self,
    ) -> crate::common::Reg<regs::GprShared2Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x484cusize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared2_authen(
        self,
    ) -> crate::common::Reg<regs::GprShared2Authen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4850usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared2_authen_set(
        self,
    ) -> crate::common::Reg<regs::GprShared2AuthenSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4854usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared2_authen_clr(
        self,
    ) -> crate::common::Reg<regs::GprShared2AuthenClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4858usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared2_authen_tog(
        self,
    ) -> crate::common::Reg<regs::GprShared2AuthenTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x485cusize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared3(self) -> crate::common::Reg<regs::GprShared3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4860usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared3_set(
        self,
    ) -> crate::common::Reg<regs::GprShared3Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4864usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared3_clr(
        self,
    ) -> crate::common::Reg<regs::GprShared3Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4868usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared3_tog(
        self,
    ) -> crate::common::Reg<regs::GprShared3Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x486cusize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared3_authen(
        self,
    ) -> crate::common::Reg<regs::GprShared3Authen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4870usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared3_authen_set(
        self,
    ) -> crate::common::Reg<regs::GprShared3AuthenSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4874usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared3_authen_clr(
        self,
    ) -> crate::common::Reg<regs::GprShared3AuthenClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4878usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared3_authen_tog(
        self,
    ) -> crate::common::Reg<regs::GprShared3AuthenTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x487cusize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared4(self) -> crate::common::Reg<regs::GprShared4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4880usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared4_set(
        self,
    ) -> crate::common::Reg<regs::GprShared4Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4884usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared4_clr(
        self,
    ) -> crate::common::Reg<regs::GprShared4Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4888usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared4_tog(
        self,
    ) -> crate::common::Reg<regs::GprShared4Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x488cusize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared4_authen(
        self,
    ) -> crate::common::Reg<regs::GprShared4Authen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4890usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared4_authen_set(
        self,
    ) -> crate::common::Reg<regs::GprShared4AuthenSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4894usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared4_authen_clr(
        self,
    ) -> crate::common::Reg<regs::GprShared4AuthenClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4898usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared4_authen_tog(
        self,
    ) -> crate::common::Reg<regs::GprShared4AuthenTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x489cusize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared5(self) -> crate::common::Reg<regs::GprShared5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48a0usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared5_set(
        self,
    ) -> crate::common::Reg<regs::GprShared5Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48a4usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared5_clr(
        self,
    ) -> crate::common::Reg<regs::GprShared5Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48a8usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared5_tog(
        self,
    ) -> crate::common::Reg<regs::GprShared5Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48acusize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared5_authen(
        self,
    ) -> crate::common::Reg<regs::GprShared5Authen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48b0usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared5_authen_set(
        self,
    ) -> crate::common::Reg<regs::GprShared5AuthenSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48b4usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared5_authen_clr(
        self,
    ) -> crate::common::Reg<regs::GprShared5AuthenClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48b8usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared5_authen_tog(
        self,
    ) -> crate::common::Reg<regs::GprShared5AuthenTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48bcusize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared6(self) -> crate::common::Reg<regs::GprShared6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48c0usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared6_set(
        self,
    ) -> crate::common::Reg<regs::GprShared6Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48c4usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared6_clr(
        self,
    ) -> crate::common::Reg<regs::GprShared6Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48c8usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared6_tog(
        self,
    ) -> crate::common::Reg<regs::GprShared6Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48ccusize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared6_authen(
        self,
    ) -> crate::common::Reg<regs::GprShared6Authen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48d0usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared6_authen_set(
        self,
    ) -> crate::common::Reg<regs::GprShared6AuthenSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48d4usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared6_authen_clr(
        self,
    ) -> crate::common::Reg<regs::GprShared6AuthenClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48d8usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared6_authen_tog(
        self,
    ) -> crate::common::Reg<regs::GprShared6AuthenTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48dcusize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared7(self) -> crate::common::Reg<regs::GprShared7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48e0usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared7_set(
        self,
    ) -> crate::common::Reg<regs::GprShared7Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48e4usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared7_clr(
        self,
    ) -> crate::common::Reg<regs::GprShared7Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48e8usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared7_tog(
        self,
    ) -> crate::common::Reg<regs::GprShared7Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48ecusize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared7_authen(
        self,
    ) -> crate::common::Reg<regs::GprShared7Authen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48f0usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared7_authen_set(
        self,
    ) -> crate::common::Reg<regs::GprShared7AuthenSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48f4usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared7_authen_clr(
        self,
    ) -> crate::common::Reg<regs::GprShared7AuthenClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48f8usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared7_authen_tog(
        self,
    ) -> crate::common::Reg<regs::GprShared7AuthenTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48fcusize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared8(self) -> crate::common::Reg<regs::GprShared8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4900usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared8_set(
        self,
    ) -> crate::common::Reg<regs::GprShared8Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4904usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared8_clr(
        self,
    ) -> crate::common::Reg<regs::GprShared8Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4908usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared8_tog(
        self,
    ) -> crate::common::Reg<regs::GprShared8Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x490cusize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared8_authen(
        self,
    ) -> crate::common::Reg<regs::GprShared8Authen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4910usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared8_authen_set(
        self,
    ) -> crate::common::Reg<regs::GprShared8AuthenSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4914usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared8_authen_clr(
        self,
    ) -> crate::common::Reg<regs::GprShared8AuthenClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4918usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared8_authen_tog(
        self,
    ) -> crate::common::Reg<regs::GprShared8AuthenTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x491cusize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared9(self) -> crate::common::Reg<regs::GprShared9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4920usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared9_set(
        self,
    ) -> crate::common::Reg<regs::GprShared9Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4924usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared9_clr(
        self,
    ) -> crate::common::Reg<regs::GprShared9Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4928usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared9_tog(
        self,
    ) -> crate::common::Reg<regs::GprShared9Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x492cusize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared9_authen(
        self,
    ) -> crate::common::Reg<regs::GprShared9Authen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4930usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared9_authen_set(
        self,
    ) -> crate::common::Reg<regs::GprShared9AuthenSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4934usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared9_authen_clr(
        self,
    ) -> crate::common::Reg<regs::GprShared9AuthenClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4938usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared9_authen_tog(
        self,
    ) -> crate::common::Reg<regs::GprShared9AuthenTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x493cusize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared10(self) -> crate::common::Reg<regs::GprShared10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4940usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared10_set(
        self,
    ) -> crate::common::Reg<regs::GprShared10Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4944usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared10_clr(
        self,
    ) -> crate::common::Reg<regs::GprShared10Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4948usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared10_tog(
        self,
    ) -> crate::common::Reg<regs::GprShared10Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x494cusize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared10_authen(
        self,
    ) -> crate::common::Reg<regs::GprShared10Authen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4950usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared10_authen_set(
        self,
    ) -> crate::common::Reg<regs::GprShared10AuthenSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4954usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared10_authen_clr(
        self,
    ) -> crate::common::Reg<regs::GprShared10AuthenClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4958usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared10_authen_tog(
        self,
    ) -> crate::common::Reg<regs::GprShared10AuthenTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x495cusize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared11(self) -> crate::common::Reg<regs::GprShared11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4960usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared11_set(
        self,
    ) -> crate::common::Reg<regs::GprShared11Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4964usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared11_clr(
        self,
    ) -> crate::common::Reg<regs::GprShared11Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4968usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared11_tog(
        self,
    ) -> crate::common::Reg<regs::GprShared11Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x496cusize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared11_authen(
        self,
    ) -> crate::common::Reg<regs::GprShared11Authen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4970usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared11_authen_set(
        self,
    ) -> crate::common::Reg<regs::GprShared11AuthenSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4974usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared11_authen_clr(
        self,
    ) -> crate::common::Reg<regs::GprShared11AuthenClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4978usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared11_authen_tog(
        self,
    ) -> crate::common::Reg<regs::GprShared11AuthenTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x497cusize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared12(self) -> crate::common::Reg<regs::GprShared12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4980usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared12_set(
        self,
    ) -> crate::common::Reg<regs::GprShared12Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4984usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared12_clr(
        self,
    ) -> crate::common::Reg<regs::GprShared12Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4988usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared12_tog(
        self,
    ) -> crate::common::Reg<regs::GprShared12Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x498cusize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared12_authen(
        self,
    ) -> crate::common::Reg<regs::GprShared12Authen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4990usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared12_authen_set(
        self,
    ) -> crate::common::Reg<regs::GprShared12AuthenSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4994usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared12_authen_clr(
        self,
    ) -> crate::common::Reg<regs::GprShared12AuthenClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4998usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared12_authen_tog(
        self,
    ) -> crate::common::Reg<regs::GprShared12AuthenTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x499cusize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared13(self) -> crate::common::Reg<regs::GprShared13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x49a0usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared13_set(
        self,
    ) -> crate::common::Reg<regs::GprShared13Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x49a4usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared13_clr(
        self,
    ) -> crate::common::Reg<regs::GprShared13Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x49a8usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared13_tog(
        self,
    ) -> crate::common::Reg<regs::GprShared13Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x49acusize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared13_authen(
        self,
    ) -> crate::common::Reg<regs::GprShared13Authen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x49b0usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared13_authen_set(
        self,
    ) -> crate::common::Reg<regs::GprShared13AuthenSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x49b4usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared13_authen_clr(
        self,
    ) -> crate::common::Reg<regs::GprShared13AuthenClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x49b8usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared13_authen_tog(
        self,
    ) -> crate::common::Reg<regs::GprShared13AuthenTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x49bcusize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared14(self) -> crate::common::Reg<regs::GprShared14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x49c0usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared14_set(
        self,
    ) -> crate::common::Reg<regs::GprShared14Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x49c4usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared14_clr(
        self,
    ) -> crate::common::Reg<regs::GprShared14Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x49c8usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared14_tog(
        self,
    ) -> crate::common::Reg<regs::GprShared14Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x49ccusize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared14_authen(
        self,
    ) -> crate::common::Reg<regs::GprShared14Authen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x49d0usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared14_authen_set(
        self,
    ) -> crate::common::Reg<regs::GprShared14AuthenSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x49d4usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared14_authen_clr(
        self,
    ) -> crate::common::Reg<regs::GprShared14AuthenClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x49d8usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared14_authen_tog(
        self,
    ) -> crate::common::Reg<regs::GprShared14AuthenTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x49dcusize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared15(self) -> crate::common::Reg<regs::GprShared15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x49e0usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared15_set(
        self,
    ) -> crate::common::Reg<regs::GprShared15Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x49e4usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared15_clr(
        self,
    ) -> crate::common::Reg<regs::GprShared15Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x49e8usize) as _) }
    }
    #[doc = "General Purpose Register"]
    #[inline(always)]
    pub const fn gpr_shared15_tog(
        self,
    ) -> crate::common::Reg<regs::GprShared15Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x49ecusize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared15_authen(
        self,
    ) -> crate::common::Reg<regs::GprShared15Authen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x49f0usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared15_authen_set(
        self,
    ) -> crate::common::Reg<regs::GprShared15AuthenSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x49f4usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared15_authen_clr(
        self,
    ) -> crate::common::Reg<regs::GprShared15AuthenClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x49f8usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_shared15_authen_tog(
        self,
    ) -> crate::common::Reg<regs::GprShared15AuthenTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x49fcusize) as _) }
    }
    #[doc = "General purpose status register for CM33"]
    #[inline(always)]
    pub const fn gpr_shared_status0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4a00usize) as _) }
    }
    #[doc = "General purpose status register for CM33"]
    #[inline(always)]
    pub const fn gpr_shared_status1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4a04usize) as _) }
    }
    #[doc = "General purpose status register for CM33"]
    #[inline(always)]
    pub const fn gpr_shared_status2(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4a08usize) as _) }
    }
    #[doc = "General purpose status register for CM33"]
    #[inline(always)]
    pub const fn gpr_shared_status3(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4a0cusize) as _) }
    }
    #[doc = "General status register for CM7"]
    #[inline(always)]
    pub const fn gpr_shared_status4(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4a10usize) as _) }
    }
    #[doc = "General purpose status register for CM7"]
    #[inline(always)]
    pub const fn gpr_shared_status5(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4a14usize) as _) }
    }
    #[doc = "General status register for CM7"]
    #[inline(always)]
    pub const fn gpr_shared_status6(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4a18usize) as _) }
    }
    #[doc = "General purpose status register for CM7"]
    #[inline(always)]
    pub const fn gpr_shared_status7(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4a1cusize) as _) }
    }
    #[doc = "General purpose register section."]
    #[inline(always)]
    pub const fn gpr_private(self, n: usize) -> GprPrivate {
        assert!(n < 4usize);
        unsafe { GprPrivate::from_ptr(self.ptr.add(0x4c00usize + n * 32usize) as _) }
    }
    #[doc = "Clock source section."]
    #[inline(always)]
    pub const fn oscpll(self, n: usize) -> Oscpll {
        assert!(n < 25usize);
        unsafe { Oscpll::from_ptr(self.ptr.add(0x5000usize + n * 64usize) as _) }
    }
    #[doc = "LPCG section."]
    #[inline(always)]
    pub const fn lpcg(self, n: usize) -> Lpcg {
        assert!(n < 149usize);
        unsafe { Lpcg::from_ptr(self.ptr.add(0x8000usize + n * 64usize) as _) }
    }
}
#[doc = "Clock root section."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClockRoot {
    ptr: *mut u8,
}
unsafe impl Send for ClockRoot {}
unsafe impl Sync for ClockRoot {}
impl ClockRoot {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Clock Root Control Register"]
    #[inline(always)]
    pub const fn clock_root_control(
        self,
    ) -> crate::common::Reg<regs::ClockRootControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Clock Root Control Register"]
    #[inline(always)]
    pub const fn clock_root_control_set(
        self,
    ) -> crate::common::Reg<regs::ClockRootControlSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Clock Root Control Register"]
    #[inline(always)]
    pub const fn clock_root_control_clr(
        self,
    ) -> crate::common::Reg<regs::ClockRootControlClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Clock Root Control Register"]
    #[inline(always)]
    pub const fn clock_root_control_tog(
        self,
    ) -> crate::common::Reg<regs::ClockRootControlTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Clock root working status"]
    #[inline(always)]
    pub const fn clock_root_status0(
        self,
    ) -> crate::common::Reg<regs::ClockRootStatus0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Clock root access control"]
    #[inline(always)]
    pub const fn clock_root_authen(
        self,
    ) -> crate::common::Reg<regs::ClockRootAuthen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
}
#[doc = "General purpose register section."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GprPrivate {
    ptr: *mut u8,
}
unsafe impl Send for GprPrivate {}
unsafe impl Sync for GprPrivate {}
impl GprPrivate {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "General purpose register"]
    #[inline(always)]
    pub const fn gpr_private(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "General purpose register"]
    #[inline(always)]
    pub const fn gpr_private_set(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "General purpose register"]
    #[inline(always)]
    pub const fn gpr_private_clr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "General purpose register"]
    #[inline(always)]
    pub const fn gpr_private_tog(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_private_authen(
        self,
    ) -> crate::common::Reg<regs::GprPrivateAuthen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_private_authen_set(
        self,
    ) -> crate::common::Reg<regs::GprPrivateAuthenSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_private_authen_clr(
        self,
    ) -> crate::common::Reg<regs::GprPrivateAuthenClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "GPR access control"]
    #[inline(always)]
    pub const fn gpr_private_authen_tog(
        self,
    ) -> crate::common::Reg<regs::GprPrivateAuthenTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
}
#[doc = "LPCG section."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpcg {
    ptr: *mut u8,
}
unsafe impl Send for Lpcg {}
unsafe impl Sync for Lpcg {}
impl Lpcg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "LPCG direct control"]
    #[inline(always)]
    pub const fn lpcg_direct(self) -> crate::common::Reg<regs::LpcgDirect, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Clock source low power mode setting"]
    #[inline(always)]
    pub const fn lpcg_lpm0(self) -> crate::common::Reg<regs::LpcgLpm0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "clock source low power mode setting"]
    #[inline(always)]
    pub const fn lpcg_lpm1(self) -> crate::common::Reg<regs::LpcgLpm1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "LPM setting of current CPU domain"]
    #[inline(always)]
    pub const fn lpcg_lpm_cur(self) -> crate::common::Reg<regs::LpcgLpmCur, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "LPCG working status"]
    #[inline(always)]
    pub const fn lpcg_status0(self) -> crate::common::Reg<regs::LpcgStatus0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "LPCG domain status"]
    #[inline(always)]
    pub const fn lpcg_status1(self) -> crate::common::Reg<regs::LpcgStatus1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "LPCG access control"]
    #[inline(always)]
    pub const fn lpcg_authen(self) -> crate::common::Reg<regs::LpcgAuthen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
}
#[doc = "Clock root section."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Observe {
    ptr: *mut u8,
}
unsafe impl Send for Observe {}
unsafe impl Sync for Observe {}
impl Observe {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Observe control"]
    #[inline(always)]
    pub const fn observe_control(
        self,
    ) -> crate::common::Reg<regs::ObserveControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Observe control"]
    #[inline(always)]
    pub const fn observe_control_set(
        self,
    ) -> crate::common::Reg<regs::ObserveControlSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Observe control"]
    #[inline(always)]
    pub const fn observe_control_clr(
        self,
    ) -> crate::common::Reg<regs::ObserveControlClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Observe control"]
    #[inline(always)]
    pub const fn observe_control_tog(
        self,
    ) -> crate::common::Reg<regs::ObserveControlTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Observe status"]
    #[inline(always)]
    pub const fn observe_status(self) -> crate::common::Reg<regs::ObserveStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Observe access control"]
    #[inline(always)]
    pub const fn observe_authen(
        self,
    ) -> crate::common::Reg<regs::ObserveAuthen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Observe access control"]
    #[inline(always)]
    pub const fn observe_authen_set(
        self,
    ) -> crate::common::Reg<regs::ObserveAuthenSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "Observe access control"]
    #[inline(always)]
    pub const fn observe_authen_clr(
        self,
    ) -> crate::common::Reg<regs::ObserveAuthenClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "Observe access control"]
    #[inline(always)]
    pub const fn observe_authen_tog(
        self,
    ) -> crate::common::Reg<regs::ObserveAuthenTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "Current frequency detected"]
    #[inline(always)]
    pub const fn observe_frequency_current(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "Minimum frequency detected"]
    #[inline(always)]
    pub const fn observe_frequency_min(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "Maximum frequency detected"]
    #[inline(always)]
    pub const fn observe_frequency_max(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "Current period time detected"]
    #[inline(always)]
    pub const fn observe_period_current(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "Minimum period time detected"]
    #[inline(always)]
    pub const fn observe_period_min(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "Maximum period time detected"]
    #[inline(always)]
    pub const fn observe_period_max(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "Current high level time detected"]
    #[inline(always)]
    pub const fn observe_high_current(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "Minimum high level time detected"]
    #[inline(always)]
    pub const fn observe_high_min(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "Maximum high level time detected"]
    #[inline(always)]
    pub const fn observe_high_max(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "Current high level time detected"]
    #[inline(always)]
    pub const fn observe_low_current(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "Minimum high level time detected"]
    #[inline(always)]
    pub const fn observe_low_min(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "Maximum high level time detected"]
    #[inline(always)]
    pub const fn observe_low_max(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
}
#[doc = "Clock source section."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oscpll {
    ptr: *mut u8,
}
unsafe impl Send for Oscpll {}
unsafe impl Sync for Oscpll {}
impl Oscpll {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Clock source direct control"]
    #[inline(always)]
    pub const fn oscpll_direct(self) -> crate::common::Reg<regs::OscpllDirect, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Clock source low power mode setting"]
    #[inline(always)]
    pub const fn oscpll_lpm0(self) -> crate::common::Reg<regs::OscpllLpm0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "clock source low power mode setting"]
    #[inline(always)]
    pub const fn oscpll_lpm1(self) -> crate::common::Reg<regs::OscpllLpm1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "LPM setting of current CPU domain"]
    #[inline(always)]
    pub const fn oscpll_lpm_cur(self) -> crate::common::Reg<regs::OscpllLpmCur, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Clock source working status"]
    #[inline(always)]
    pub const fn oscpll_status0(self) -> crate::common::Reg<regs::OscpllStatus0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Clock source domain status"]
    #[inline(always)]
    pub const fn oscpll_status1(self) -> crate::common::Reg<regs::OscpllStatus1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Clock Source access control"]
    #[inline(always)]
    pub const fn oscpll_authen(self) -> crate::common::Reg<regs::OscpllAuthen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
}
pub mod regs;
pub mod vals;
