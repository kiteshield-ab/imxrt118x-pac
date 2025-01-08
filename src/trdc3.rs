#[doc = "TRDC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trdc3 {
    ptr: *mut u8,
}
unsafe impl Send for Trdc3 {}
unsafe impl Sync for Trdc3 {}
impl Trdc3 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "TRDC Register"]
    #[inline(always)]
    pub const fn trdc_cr(self) -> crate::common::Reg<super::trdc::regs::TrdcCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "TRDC Hardware Configuration Register 0"]
    #[inline(always)]
    pub const fn trdc_hwcfg0(
        self,
    ) -> crate::common::Reg<super::trdc::regs::TrdcHwcfg0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
    }
    #[doc = "TRDC Hardware Configuration Register 1"]
    #[inline(always)]
    pub const fn trdc_hwcfg1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::TrdcHwcfg1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4usize) as _) }
    }
    #[doc = "TRDC Hardware Configuration Register 2"]
    #[inline(always)]
    pub const fn trdc_hwcfg2(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf8usize) as _) }
    }
    #[doc = "TRDC Hardware Configuration Register 3"]
    #[inline(always)]
    pub const fn trdc_hwcfg3(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xfcusize) as _) }
    }
    #[doc = "Domain Assignment Configuration Register"]
    #[inline(always)]
    pub const fn dacfg0(self) -> crate::common::Reg<super::trdc::regs::Dacfg, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "Domain Assignment Configuration Register"]
    #[inline(always)]
    pub const fn dacfg1(self) -> crate::common::Reg<super::trdc::regs::Dacfg, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0101usize) as _) }
    }
    #[doc = "Domain Assignment Configuration Register"]
    #[inline(always)]
    pub const fn dacfg2(self) -> crate::common::Reg<super::trdc::regs::Dacfg, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0102usize) as _) }
    }
    #[doc = "Domain Assignment Configuration Register"]
    #[inline(always)]
    pub const fn dacfg3(self) -> crate::common::Reg<super::trdc::regs::Dacfg, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0103usize) as _) }
    }
    #[doc = "Domain Assignment Configuration Register"]
    #[inline(always)]
    pub const fn dacfg4(self) -> crate::common::Reg<super::trdc::regs::Dacfg, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "TRDC IDAU Control Register"]
    #[inline(always)]
    pub const fn trdc_idau_cr(
        self,
    ) -> crate::common::Reg<super::trdc::regs::TrdcIdauCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c0usize) as _) }
    }
    #[doc = "TRDC FLW Control"]
    #[inline(always)]
    pub const fn trdc_flw_ctl(
        self,
    ) -> crate::common::Reg<super::trdc::regs::TrdcFlwCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01e0usize) as _) }
    }
    #[doc = "TRDC FLW Physical Base"]
    #[inline(always)]
    pub const fn trdc_flw_pbase(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01e4usize) as _) }
    }
    #[doc = "TRDC FLW Array Base"]
    #[inline(always)]
    pub const fn trdc_flw_abase(
        self,
    ) -> crate::common::Reg<super::trdc::regs::TrdcFlwAbase, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01e8usize) as _) }
    }
    #[doc = "TRDC FLW Block Count"]
    #[inline(always)]
    pub const fn trdc_flw_bcnt(
        self,
    ) -> crate::common::Reg<super::trdc::regs::TrdcFlwBcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01ecusize) as _) }
    }
    #[doc = "TRDC Fault Domain ID"]
    #[inline(always)]
    pub const fn trdc_fdid(
        self,
    ) -> crate::common::Reg<super::trdc::regs::TrdcFdid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01fcusize) as _) }
    }
    #[doc = "TRDC Domain Error Location Register"]
    #[inline(always)]
    pub const fn trdc_derrloc(
        self,
        n: usize,
    ) -> crate::common::Reg<super::trdc::regs::TrdcDerrloc, crate::common::R> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize + n * 4usize) as _) }
    }
    #[doc = "DAC Master Domain Assignment Register"]
    #[inline(always)]
    pub const fn mda_w0_0_dfmt1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Dfmt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0800usize) as _) }
    }
    #[doc = "DAC Master Domain Assignment Register"]
    #[inline(always)]
    pub const fn mda_w0_1_dfmt1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Dfmt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0820usize) as _) }
    }
    #[doc = "DAC Master Domain Assignment Register"]
    #[inline(always)]
    pub const fn mda_w0_2_dfmt1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Dfmt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0840usize) as _) }
    }
    #[doc = "DAC Master Domain Assignment Register"]
    #[inline(always)]
    pub const fn mda_w0_3_dfmt1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Dfmt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0860usize) as _) }
    }
    #[doc = "DAC Master Domain Assignment Register"]
    #[inline(always)]
    pub const fn mda_w0_4_dfmt1(
        self,
    ) -> crate::common::Reg<super::trdc::regs::Dfmt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0880usize) as _) }
    }
}
