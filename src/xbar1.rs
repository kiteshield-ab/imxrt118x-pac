#[doc = "XBAR"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbar1 {
    ptr: *mut u8,
}
unsafe impl Send for Xbar1 {}
unsafe impl Sync for Xbar1 {}
impl Xbar1 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel0(self) -> crate::common::Reg<regs::Sel0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel1(self) -> crate::common::Reg<regs::Sel1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel2(self) -> crate::common::Reg<regs::Sel2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel3(self) -> crate::common::Reg<regs::Sel3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel4(self) -> crate::common::Reg<regs::Sel4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel5(self) -> crate::common::Reg<regs::Sel5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ausize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel6(self) -> crate::common::Reg<regs::Sel6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel7(self) -> crate::common::Reg<regs::Sel7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0eusize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel8(self) -> crate::common::Reg<regs::Sel8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel9(self) -> crate::common::Reg<regs::Sel9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x12usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel10(self) -> crate::common::Reg<regs::Sel10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel11(self) -> crate::common::Reg<regs::Sel11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x16usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel12(self) -> crate::common::Reg<regs::Sel12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel13(self) -> crate::common::Reg<regs::Sel13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1ausize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel14(self) -> crate::common::Reg<regs::Sel14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel15(self) -> crate::common::Reg<regs::Sel15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1eusize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel16(self) -> crate::common::Reg<regs::Sel16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel17(self) -> crate::common::Reg<regs::Sel17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x22usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel18(self) -> crate::common::Reg<regs::Sel18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel19(self) -> crate::common::Reg<regs::Sel19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x26usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel20(self) -> crate::common::Reg<regs::Sel20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel21(self) -> crate::common::Reg<regs::Sel21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2ausize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel22(self) -> crate::common::Reg<regs::Sel22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel23(self) -> crate::common::Reg<regs::Sel23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2eusize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel24(self) -> crate::common::Reg<regs::Sel24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel25(self) -> crate::common::Reg<regs::Sel25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x32usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel26(self) -> crate::common::Reg<regs::Sel26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel27(self) -> crate::common::Reg<regs::Sel27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x36usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel28(self) -> crate::common::Reg<regs::Sel28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel29(self) -> crate::common::Reg<regs::Sel29, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3ausize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel30(self) -> crate::common::Reg<regs::Sel30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel31(self) -> crate::common::Reg<regs::Sel31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3eusize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel32(self) -> crate::common::Reg<regs::Sel32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel33(self) -> crate::common::Reg<regs::Sel33, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x42usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel34(self) -> crate::common::Reg<regs::Sel34, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel35(self) -> crate::common::Reg<regs::Sel35, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x46usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel36(self) -> crate::common::Reg<regs::Sel36, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel37(self) -> crate::common::Reg<regs::Sel37, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4ausize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel38(self) -> crate::common::Reg<regs::Sel38, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel39(self) -> crate::common::Reg<regs::Sel39, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4eusize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel40(self) -> crate::common::Reg<regs::Sel40, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel41(self) -> crate::common::Reg<regs::Sel41, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x52usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel42(self) -> crate::common::Reg<regs::Sel42, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel43(self) -> crate::common::Reg<regs::Sel43, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x56usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel44(self) -> crate::common::Reg<regs::Sel44, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel45(self) -> crate::common::Reg<regs::Sel45, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5ausize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel46(self) -> crate::common::Reg<regs::Sel46, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel47(self) -> crate::common::Reg<regs::Sel47, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5eusize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel48(self) -> crate::common::Reg<regs::Sel48, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel49(self) -> crate::common::Reg<regs::Sel49, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x62usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel50(self) -> crate::common::Reg<regs::Sel50, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel51(self) -> crate::common::Reg<regs::Sel51, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x66usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel52(self) -> crate::common::Reg<regs::Sel52, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel53(self) -> crate::common::Reg<regs::Sel53, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6ausize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel54(self) -> crate::common::Reg<regs::Sel54, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel55(self) -> crate::common::Reg<regs::Sel55, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6eusize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel56(self) -> crate::common::Reg<regs::Sel56, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel57(self) -> crate::common::Reg<regs::Sel57, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x72usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel58(self) -> crate::common::Reg<regs::Sel58, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel59(self) -> crate::common::Reg<regs::Sel59, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x76usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel60(self) -> crate::common::Reg<regs::Sel60, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel61(self) -> crate::common::Reg<regs::Sel61, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7ausize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel62(self) -> crate::common::Reg<regs::Sel62, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel63(self) -> crate::common::Reg<regs::Sel63, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7eusize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel64(self) -> crate::common::Reg<regs::Sel64, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel65(self) -> crate::common::Reg<regs::Sel65, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x82usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel66(self) -> crate::common::Reg<regs::Sel66, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel67(self) -> crate::common::Reg<regs::Sel67, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x86usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel68(self) -> crate::common::Reg<regs::Sel68, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel69(self) -> crate::common::Reg<regs::Sel69, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8ausize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel70(self) -> crate::common::Reg<regs::Sel70, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel71(self) -> crate::common::Reg<regs::Sel71, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8eusize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel72(self) -> crate::common::Reg<regs::Sel72, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel73(self) -> crate::common::Reg<regs::Sel73, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x92usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel74(self) -> crate::common::Reg<regs::Sel74, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel75(self) -> crate::common::Reg<regs::Sel75, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x96usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel76(self) -> crate::common::Reg<regs::Sel76, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel77(self) -> crate::common::Reg<regs::Sel77, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9ausize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel78(self) -> crate::common::Reg<regs::Sel78, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel79(self) -> crate::common::Reg<regs::Sel79, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9eusize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel80(self) -> crate::common::Reg<regs::Sel80, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel81(self) -> crate::common::Reg<regs::Sel81, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa2usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel82(self) -> crate::common::Reg<regs::Sel82, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel83(self) -> crate::common::Reg<regs::Sel83, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa6usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel84(self) -> crate::common::Reg<regs::Sel84, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel85(self) -> crate::common::Reg<regs::Sel85, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xaausize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel86(self) -> crate::common::Reg<regs::Sel86, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel87(self) -> crate::common::Reg<regs::Sel87, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xaeusize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel88(self) -> crate::common::Reg<regs::Sel88, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel89(self) -> crate::common::Reg<regs::Sel89, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb2usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel90(self) -> crate::common::Reg<regs::Sel90, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel91(self) -> crate::common::Reg<regs::Sel91, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb6usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel92(self) -> crate::common::Reg<regs::Sel92, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb8usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel93(self) -> crate::common::Reg<regs::Sel93, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbausize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel94(self) -> crate::common::Reg<regs::Sel94, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbcusize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel95(self) -> crate::common::Reg<regs::Sel95, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbeusize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel96(self) -> crate::common::Reg<regs::Sel96, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel97(self) -> crate::common::Reg<regs::Sel97, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc2usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel98(self) -> crate::common::Reg<regs::Sel98, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel99(self) -> crate::common::Reg<regs::Sel99, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc6usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel100(self) -> crate::common::Reg<regs::Sel100, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel101(self) -> crate::common::Reg<regs::Sel101, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xcausize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel102(self) -> crate::common::Reg<regs::Sel102, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xccusize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel103(self) -> crate::common::Reg<regs::Sel103, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xceusize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel104(self) -> crate::common::Reg<regs::Sel104, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel105(self) -> crate::common::Reg<regs::Sel105, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd2usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel106(self) -> crate::common::Reg<regs::Sel106, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd4usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel107(self) -> crate::common::Reg<regs::Sel107, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd6usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel108(self) -> crate::common::Reg<regs::Sel108, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel109(self) -> crate::common::Reg<regs::Sel109, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdausize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel110(self) -> crate::common::Reg<regs::Sel110, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdcusize) as _) }
    }
    #[doc = "Crossbar Control Register"]
    #[inline(always)]
    pub const fn ctrl0(self) -> crate::common::Reg<regs::Ctrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdeusize) as _) }
    }
    #[doc = "Crossbar Control Register"]
    #[inline(always)]
    pub const fn ctrl1(self) -> crate::common::Reg<regs::Ctrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe0usize) as _) }
    }
}
pub mod regs;
pub mod vals;
