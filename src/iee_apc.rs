#[doc = "IEE_APC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IeeApc {
    ptr: *mut u8,
}
unsafe impl Send for IeeApc {}
unsafe impl Sync for IeeApc {}
impl IeeApc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "End address of IEE region (n)"]
    #[inline(always)]
    pub const fn region0_top_addr(
        self,
    ) -> crate::common::Reg<regs::Region0TopAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Start address of IEE region (n)"]
    #[inline(always)]
    pub const fn region0_bot_addr(
        self,
    ) -> crate::common::Reg<regs::Region0BotAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Region enable for region (n)"]
    #[inline(always)]
    pub const fn region0_ena(self) -> crate::common::Reg<regs::Region0Ena, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Access control for IEE APC registers of region (n)"]
    #[inline(always)]
    pub const fn region0_acc_ctl(
        self,
    ) -> crate::common::Reg<regs::Region0AccCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "End address of IEE region (n)"]
    #[inline(always)]
    pub const fn region1_top_addr(
        self,
    ) -> crate::common::Reg<regs::Region1TopAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Start address of IEE region (n)"]
    #[inline(always)]
    pub const fn region1_bot_addr(
        self,
    ) -> crate::common::Reg<regs::Region1BotAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Region enable for region (n)"]
    #[inline(always)]
    pub const fn region1_ena(self) -> crate::common::Reg<regs::Region1Ena, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Access control for IEE APC registers of region (n)"]
    #[inline(always)]
    pub const fn region1_acc_ctl(
        self,
    ) -> crate::common::Reg<regs::Region1AccCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "End address of IEE region (n)"]
    #[inline(always)]
    pub const fn region2_top_addr(
        self,
    ) -> crate::common::Reg<regs::Region2TopAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Start address of IEE region (n)"]
    #[inline(always)]
    pub const fn region2_bot_addr(
        self,
    ) -> crate::common::Reg<regs::Region2BotAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Region enable for region (n)"]
    #[inline(always)]
    pub const fn region2_ena(self) -> crate::common::Reg<regs::Region2Ena, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Access control for IEE APC registers of region (n)"]
    #[inline(always)]
    pub const fn region2_acc_ctl(
        self,
    ) -> crate::common::Reg<regs::Region2AccCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "End address of IEE region (n)"]
    #[inline(always)]
    pub const fn region3_top_addr(
        self,
    ) -> crate::common::Reg<regs::Region3TopAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Start address of IEE region (n)"]
    #[inline(always)]
    pub const fn region3_bot_addr(
        self,
    ) -> crate::common::Reg<regs::Region3BotAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "Region enable for region (n)"]
    #[inline(always)]
    pub const fn region3_ena(self) -> crate::common::Reg<regs::Region3Ena, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "Access control for IEE APC registers of region (n)"]
    #[inline(always)]
    pub const fn region3_acc_ctl(
        self,
    ) -> crate::common::Reg<regs::Region3AccCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "End address of IEE region (n)"]
    #[inline(always)]
    pub const fn region4_top_addr(
        self,
    ) -> crate::common::Reg<regs::Region4TopAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "Start address of IEE region (n)"]
    #[inline(always)]
    pub const fn region4_bot_addr(
        self,
    ) -> crate::common::Reg<regs::Region4BotAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "Region enable for region (n)"]
    #[inline(always)]
    pub const fn region4_ena(self) -> crate::common::Reg<regs::Region4Ena, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "Access control for IEE APC registers of region (n)"]
    #[inline(always)]
    pub const fn region4_acc_ctl(
        self,
    ) -> crate::common::Reg<regs::Region4AccCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "End address of IEE region (n)"]
    #[inline(always)]
    pub const fn region5_top_addr(
        self,
    ) -> crate::common::Reg<regs::Region5TopAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "Start address of IEE region (n)"]
    #[inline(always)]
    pub const fn region5_bot_addr(
        self,
    ) -> crate::common::Reg<regs::Region5BotAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "Region enable for region (n)"]
    #[inline(always)]
    pub const fn region5_ena(self) -> crate::common::Reg<regs::Region5Ena, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "Access control for IEE APC registers of region (n)"]
    #[inline(always)]
    pub const fn region5_acc_ctl(
        self,
    ) -> crate::common::Reg<regs::Region5AccCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "End address of IEE region (n)"]
    #[inline(always)]
    pub const fn region6_top_addr(
        self,
    ) -> crate::common::Reg<regs::Region6TopAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "Start address of IEE region (n)"]
    #[inline(always)]
    pub const fn region6_bot_addr(
        self,
    ) -> crate::common::Reg<regs::Region6BotAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "Region enable for region (n)"]
    #[inline(always)]
    pub const fn region6_ena(self) -> crate::common::Reg<regs::Region6Ena, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "Access control for IEE APC registers of region (n)"]
    #[inline(always)]
    pub const fn region6_acc_ctl(
        self,
    ) -> crate::common::Reg<regs::Region6AccCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "End address of IEE region (n)"]
    #[inline(always)]
    pub const fn region7_top_addr(
        self,
    ) -> crate::common::Reg<regs::Region7TopAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "Start address of IEE region (n)"]
    #[inline(always)]
    pub const fn region7_bot_addr(
        self,
    ) -> crate::common::Reg<regs::Region7BotAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "Region enable for region (n)"]
    #[inline(always)]
    pub const fn region7_ena(self) -> crate::common::Reg<regs::Region7Ena, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "Access control for IEE APC registers of region (n)"]
    #[inline(always)]
    pub const fn region7_acc_ctl(
        self,
    ) -> crate::common::Reg<regs::Region7AccCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
}
pub mod regs;
