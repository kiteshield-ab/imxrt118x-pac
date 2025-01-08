#[doc = "Fractional PLL"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pll {
    ptr: *mut u8,
}
unsafe impl Send for Pll {}
unsafe impl Sync for Pll {}
impl Pll {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Fractional PLL Control Register"]
    #[inline(always)]
    pub const fn ctrl0(self) -> crate::common::Reg<regs::Ctrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Fractional PLL Control Register"]
    #[inline(always)]
    pub const fn ctrl0_set(self) -> crate::common::Reg<regs::Ctrl0Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Fractional PLL Control Register"]
    #[inline(always)]
    pub const fn ctrl0_clr(self) -> crate::common::Reg<regs::Ctrl0Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Fractional PLL Control Register"]
    #[inline(always)]
    pub const fn ctrl0_tog(self) -> crate::common::Reg<regs::Ctrl0Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Fractional PLL Spread Spectrum Control Register"]
    #[inline(always)]
    pub const fn spread_spectrum(
        self,
    ) -> crate::common::Reg<regs::SpreadSpectrum, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Fractional PLL Spread Spectrum Control Register"]
    #[inline(always)]
    pub const fn spread_spectrum_set(
        self,
    ) -> crate::common::Reg<regs::SpreadSpectrumSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Fractional PLL Spread Spectrum Control Register"]
    #[inline(always)]
    pub const fn spread_spectrum_clr(
        self,
    ) -> crate::common::Reg<regs::SpreadSpectrumClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Fractional PLL Spread Spectrum Control Register"]
    #[inline(always)]
    pub const fn spread_spectrum_tog(
        self,
    ) -> crate::common::Reg<regs::SpreadSpectrumTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Fractional PLL Numerator Control Register"]
    #[inline(always)]
    pub const fn numerator(self) -> crate::common::Reg<regs::Numerator, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Fractional PLL Numerator Control Register"]
    #[inline(always)]
    pub const fn numerator_set(self) -> crate::common::Reg<regs::NumeratorSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Fractional PLL Numerator Control Register"]
    #[inline(always)]
    pub const fn numerator_clr(self) -> crate::common::Reg<regs::NumeratorClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Fractional PLL Numerator Control Register"]
    #[inline(always)]
    pub const fn numerator_tog(self) -> crate::common::Reg<regs::NumeratorTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Fractional PLL Denominator Control Register"]
    #[inline(always)]
    pub const fn denominator(self) -> crate::common::Reg<regs::Denominator, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Fractional PLL Denominator Control Register"]
    #[inline(always)]
    pub const fn denominator_set(
        self,
    ) -> crate::common::Reg<regs::DenominatorSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "Fractional PLL Denominator Control Register"]
    #[inline(always)]
    pub const fn denominator_clr(
        self,
    ) -> crate::common::Reg<regs::DenominatorClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "Fractional PLL Denominator Control Register"]
    #[inline(always)]
    pub const fn denominator_tog(
        self,
    ) -> crate::common::Reg<regs::DenominatorTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
