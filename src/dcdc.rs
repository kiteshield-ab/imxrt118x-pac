#[doc = "DCDC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcdc {
    ptr: *mut u8,
}
unsafe impl Send for Dcdc {}
unsafe impl Sync for Dcdc {}
impl Dcdc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "DCDC Register 0"]
    #[inline(always)]
    pub const fn reg0(self) -> crate::common::Reg<regs::Reg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "DCDC Register 1"]
    #[inline(always)]
    pub const fn reg1(self) -> crate::common::Reg<regs::Reg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "DCDC Register 2"]
    #[inline(always)]
    pub const fn reg2(self) -> crate::common::Reg<regs::Reg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "DCDC Register 3"]
    #[inline(always)]
    pub const fn reg3(self) -> crate::common::Reg<regs::Reg3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "DCDC Control Register 0"]
    #[inline(always)]
    pub const fn ctrl0(self) -> crate::common::Reg<regs::Ctrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "OK CNT"]
    #[inline(always)]
    pub const fn ok_cnt(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "CURRENT TARGET VALUE for DCDC ANALOG"]
    #[inline(always)]
    pub const fn current_trg(self) -> crate::common::Reg<regs::CurrentTrg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "FILTER CNT"]
    #[inline(always)]
    pub const fn filter_cnt(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "TRG_0 Authentication Control"]
    #[inline(always)]
    pub const fn trg_0_authen(self) -> crate::common::Reg<regs::Trg0Authen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Target SW Control for CORE 0"]
    #[inline(always)]
    pub const fn trg_sw_0(self) -> crate::common::Reg<regs::TrgSw0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Target GPC Control for CORE 0"]
    #[inline(always)]
    pub const fn trg_gpc_0(self) -> crate::common::Reg<regs::TrgGpc0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "TRG_1 Authentication Control"]
    #[inline(always)]
    pub const fn trg_1_authen(self) -> crate::common::Reg<regs::Trg1Authen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Target SW Control for CORE 1"]
    #[inline(always)]
    pub const fn trg_sw_1(self) -> crate::common::Reg<regs::TrgSw1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "Target GPC Control for CORE 1"]
    #[inline(always)]
    pub const fn trg_gpc_1(self) -> crate::common::Reg<regs::TrgGpc1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
}
pub mod regs;
pub mod vals;
