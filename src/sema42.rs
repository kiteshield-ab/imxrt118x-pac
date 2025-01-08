#[doc = "SEMA42"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sema42 {
    ptr: *mut u8,
}
unsafe impl Send for Sema42 {}
unsafe impl Sync for Sema42 {}
impl Sema42 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate3(self) -> crate::common::Reg<regs::Gate3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate2(self) -> crate::common::Reg<regs::Gate2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate1(self) -> crate::common::Reg<regs::Gate1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate0(self) -> crate::common::Reg<regs::Gate0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate7(self) -> crate::common::Reg<regs::Gate7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate6(self) -> crate::common::Reg<regs::Gate6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate5(self) -> crate::common::Reg<regs::Gate5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate4(self) -> crate::common::Reg<regs::Gate4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate11(self) -> crate::common::Reg<regs::Gate11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate10(self) -> crate::common::Reg<regs::Gate10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x09usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate9(self) -> crate::common::Reg<regs::Gate9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ausize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate8(self) -> crate::common::Reg<regs::Gate8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0busize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate15(self) -> crate::common::Reg<regs::Gate15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate14(self) -> crate::common::Reg<regs::Gate14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0dusize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate13(self) -> crate::common::Reg<regs::Gate13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0eusize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate12(self) -> crate::common::Reg<regs::Gate12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fusize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate19(self) -> crate::common::Reg<regs::Gate19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate18(self) -> crate::common::Reg<regs::Gate18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x11usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate17(self) -> crate::common::Reg<regs::Gate17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x12usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate16(self) -> crate::common::Reg<regs::Gate16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x13usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate23(self) -> crate::common::Reg<regs::Gate23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate22(self) -> crate::common::Reg<regs::Gate22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x15usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate21(self) -> crate::common::Reg<regs::Gate21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x16usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate20(self) -> crate::common::Reg<regs::Gate20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x17usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate27(self) -> crate::common::Reg<regs::Gate27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate26(self) -> crate::common::Reg<regs::Gate26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x19usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate25(self) -> crate::common::Reg<regs::Gate25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1ausize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate24(self) -> crate::common::Reg<regs::Gate24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1busize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate31(self) -> crate::common::Reg<regs::Gate31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate30(self) -> crate::common::Reg<regs::Gate30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1dusize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate29(self) -> crate::common::Reg<regs::Gate29, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1eusize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate28(self) -> crate::common::Reg<regs::Gate28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1fusize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate35(self) -> crate::common::Reg<regs::Gate35, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate34(self) -> crate::common::Reg<regs::Gate34, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x21usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate33(self) -> crate::common::Reg<regs::Gate33, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x22usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate32(self) -> crate::common::Reg<regs::Gate32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x23usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate39(self) -> crate::common::Reg<regs::Gate39, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate38(self) -> crate::common::Reg<regs::Gate38, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x25usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate37(self) -> crate::common::Reg<regs::Gate37, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x26usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate36(self) -> crate::common::Reg<regs::Gate36, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x27usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate43(self) -> crate::common::Reg<regs::Gate43, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate42(self) -> crate::common::Reg<regs::Gate42, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x29usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate41(self) -> crate::common::Reg<regs::Gate41, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2ausize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate40(self) -> crate::common::Reg<regs::Gate40, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2busize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate47(self) -> crate::common::Reg<regs::Gate47, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate46(self) -> crate::common::Reg<regs::Gate46, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2dusize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate45(self) -> crate::common::Reg<regs::Gate45, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2eusize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate44(self) -> crate::common::Reg<regs::Gate44, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2fusize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate51(self) -> crate::common::Reg<regs::Gate51, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate50(self) -> crate::common::Reg<regs::Gate50, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x31usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate49(self) -> crate::common::Reg<regs::Gate49, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x32usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate48(self) -> crate::common::Reg<regs::Gate48, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x33usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate55(self) -> crate::common::Reg<regs::Gate55, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate54(self) -> crate::common::Reg<regs::Gate54, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x35usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate53(self) -> crate::common::Reg<regs::Gate53, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x36usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate52(self) -> crate::common::Reg<regs::Gate52, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x37usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate59(self) -> crate::common::Reg<regs::Gate59, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate58(self) -> crate::common::Reg<regs::Gate58, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x39usize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate57(self) -> crate::common::Reg<regs::Gate57, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3ausize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate56(self) -> crate::common::Reg<regs::Gate56, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3busize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate63(self) -> crate::common::Reg<regs::Gate63, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate62(self) -> crate::common::Reg<regs::Gate62, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3dusize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate61(self) -> crate::common::Reg<regs::Gate61, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3eusize) as _) }
    }
    #[doc = "Gate"]
    #[inline(always)]
    pub const fn gate60(self) -> crate::common::Reg<regs::Gate60, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3fusize) as _) }
    }
    #[doc = "Reset Gate Read"]
    #[inline(always)]
    pub const fn rstgt_r(self) -> crate::common::Reg<regs::RstgtR, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x42usize) as _) }
    }
    #[doc = "Reset Gate Write"]
    #[inline(always)]
    pub const fn rstgt_w(self) -> crate::common::Reg<regs::RstgtW, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x42usize) as _) }
    }
}
pub mod regs;
pub mod vals;
