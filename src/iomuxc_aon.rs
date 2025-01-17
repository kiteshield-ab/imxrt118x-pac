#[doc = "IOMUXC_AON"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IomuxcAon {
    ptr: *mut u8,
}
unsafe impl Send for IomuxcAon {}
unsafe impl Sync for IomuxcAon {}
impl IomuxcAon {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AON_00 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_aon_00(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAon00, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AON_01 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_aon_01(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAon01, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AON_02 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_aon_02(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAon02, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AON_03 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_aon_03(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAon03, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AON_04 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_aon_04(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAon04, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AON_05 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_aon_05(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAon05, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AON_06 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_aon_06(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAon06, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AON_07 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_aon_07(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAon07, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AON_08 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_aon_08(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAon08, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AON_09 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_aon_09(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAon09, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AON_10 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_aon_10(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAon10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AON_11 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_aon_11(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAon11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AON_12 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_aon_12(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAon12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AON_13 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_aon_13(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAon13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AON_14 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_aon_14(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAon14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AON_15 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_aon_15(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAon15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AON_16 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_aon_16(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAon16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AON_17 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_aon_17(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAon17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AON_18 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_aon_18(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAon18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AON_19 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_aon_19(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAon19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AON_20 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_aon_20(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAon20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AON_21 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_aon_21(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAon21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AON_22 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_aon_22(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAon22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AON_23 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_aon_23(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAon23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AON_24 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_aon_24(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAon24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AON_25 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_aon_25(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAon25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AON_26 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_aon_26(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAon26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AON_27 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_aon_27(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAon27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AON_28_DUMMY SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_aon_28(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAon28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AON_00 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_aon_00(
        self,
    ) -> crate::common::Reg<super::iomuxc::regs::SwPadCtlPadGpioAon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AON_01 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_aon_01(
        self,
    ) -> crate::common::Reg<super::iomuxc::regs::SwPadCtlPadGpioAon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AON_02 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_aon_02(
        self,
    ) -> crate::common::Reg<super::iomuxc::regs::SwPadCtlPadGpioAon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AON_03 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_aon_03(
        self,
    ) -> crate::common::Reg<super::iomuxc::regs::SwPadCtlPadGpioAon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AON_04 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_aon_04(
        self,
    ) -> crate::common::Reg<super::iomuxc::regs::SwPadCtlPadGpioAon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AON_05 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_aon_05(
        self,
    ) -> crate::common::Reg<super::iomuxc::regs::SwPadCtlPadGpioAon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AON_06 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_aon_06(
        self,
    ) -> crate::common::Reg<super::iomuxc::regs::SwPadCtlPadGpioAon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AON_07 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_aon_07(
        self,
    ) -> crate::common::Reg<super::iomuxc::regs::SwPadCtlPadGpioAon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AON_08 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_aon_08(
        self,
    ) -> crate::common::Reg<super::iomuxc::regs::SwPadCtlPadGpioAon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AON_09 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_aon_09(
        self,
    ) -> crate::common::Reg<super::iomuxc::regs::SwPadCtlPadGpioAon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AON_10 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_aon_10(
        self,
    ) -> crate::common::Reg<super::iomuxc::regs::SwPadCtlPadGpioAon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AON_11 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_aon_11(
        self,
    ) -> crate::common::Reg<super::iomuxc::regs::SwPadCtlPadGpioAon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AON_12 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_aon_12(
        self,
    ) -> crate::common::Reg<super::iomuxc::regs::SwPadCtlPadGpioAon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AON_13 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_aon_13(
        self,
    ) -> crate::common::Reg<super::iomuxc::regs::SwPadCtlPadGpioAon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AON_14 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_aon_14(
        self,
    ) -> crate::common::Reg<super::iomuxc::regs::SwPadCtlPadGpioAon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AON_15 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_aon_15(
        self,
    ) -> crate::common::Reg<super::iomuxc::regs::SwPadCtlPadGpioAon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AON_16 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_aon_16(
        self,
    ) -> crate::common::Reg<super::iomuxc::regs::SwPadCtlPadGpioAon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AON_17 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_aon_17(
        self,
    ) -> crate::common::Reg<super::iomuxc::regs::SwPadCtlPadGpioAon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AON_18 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_aon_18(
        self,
    ) -> crate::common::Reg<super::iomuxc::regs::SwPadCtlPadGpioAon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbcusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AON_19 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_aon_19(
        self,
    ) -> crate::common::Reg<super::iomuxc::regs::SwPadCtlPadGpioAon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AON_20 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_aon_20(
        self,
    ) -> crate::common::Reg<super::iomuxc::regs::SwPadCtlPadGpioAon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AON_21 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_aon_21(
        self,
    ) -> crate::common::Reg<super::iomuxc::regs::SwPadCtlPadGpioAon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AON_22 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_aon_22(
        self,
    ) -> crate::common::Reg<super::iomuxc::regs::SwPadCtlPadGpioAon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xccusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AON_23 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_aon_23(
        self,
    ) -> crate::common::Reg<super::iomuxc::regs::SwPadCtlPadGpioAon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AON_24 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_aon_24(
        self,
    ) -> crate::common::Reg<super::iomuxc::regs::SwPadCtlPadGpioAon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AON_25 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_aon_25(
        self,
    ) -> crate::common::Reg<super::iomuxc::regs::SwPadCtlPadGpioAon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AON_26 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_aon_26(
        self,
    ) -> crate::common::Reg<super::iomuxc::regs::SwPadCtlPadGpioAon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdcusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AON_27 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_aon_27(
        self,
    ) -> crate::common::Reg<super::iomuxc::regs::SwPadCtlPadGpioAon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AON_28_DUMMY SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_aon_28(
        self,
    ) -> crate::common::Reg<super::iomuxc::regs::SwPadCtlPadGpioAon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe4usize) as _) }
    }
    #[doc = "I3C1_PIN_SCL_IN_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn i3c1_pin_scl_in_select_input(
        self,
    ) -> crate::common::Reg<regs::I3c1PinSclInSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe8usize) as _) }
    }
    #[doc = "I3C1_PIN_SDA_IN_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn i3c1_pin_sda_in_select_input(
        self,
    ) -> crate::common::Reg<regs::I3c1PinSdaInSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xecusize) as _) }
    }
    #[doc = "LPI2C1_IPP_IND_LPI2C_SCL_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpi2c1_ipp_ind_lpi2c_scl_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpi2c1IppIndLpi2cSclSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
    }
    #[doc = "LPI2C1_IPP_IND_LPI2C_SDA_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpi2c1_ipp_ind_lpi2c_sda_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpi2c1IppIndLpi2cSdaSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4usize) as _) }
    }
    #[doc = "LPI2C2_IPP_IND_LPI2C_SCL_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpi2c2_ipp_ind_lpi2c_scl_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpi2c2IppIndLpi2cSclSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf8usize) as _) }
    }
    #[doc = "LPI2C2_IPP_IND_LPI2C_SDA_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpi2c2_ipp_ind_lpi2c_sda_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpi2c2IppIndLpi2cSdaSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xfcusize) as _) }
    }
    #[doc = "LPSPI1_IPP_IND_LPSPI_PCS_SELECT_INPUT_0 DAISY Register"]
    #[inline(always)]
    pub const fn lpspi1_ipp_ind_lpspi_pcs_select_input_0(
        self,
    ) -> crate::common::Reg<regs::Lpspi1IppIndLpspiPcsSelectInput0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "LPSPI1_IPP_IND_LPSPI_PCS_SELECT_INPUT_1 DAISY Register"]
    #[inline(always)]
    pub const fn lpspi1_ipp_ind_lpspi_pcs_select_input_1(
        self,
    ) -> crate::common::Reg<regs::Lpspi1IppIndLpspiPcsSelectInput1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "LPSPI1_IPP_IND_LPSPI_SCK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi1_ipp_ind_lpspi_sck_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi1IppIndLpspiSckSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "LPSPI1_IPP_IND_LPSPI_SDI_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi1_ipp_ind_lpspi_sdi_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi1IppIndLpspiSdiSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[doc = "LPSPI1_IPP_IND_LPSPI_SDO_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi1_ipp_ind_lpspi_sdo_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi1IppIndLpspiSdoSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "LPSPI2_IPP_IND_LPSPI_PCS_SELECT_INPUT_0 DAISY Register"]
    #[inline(always)]
    pub const fn lpspi2_ipp_ind_lpspi_pcs_select_input_0(
        self,
    ) -> crate::common::Reg<regs::Lpspi2IppIndLpspiPcsSelectInput0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
    #[doc = "LPSPI2_IPP_IND_LPSPI_PCS_SELECT_INPUT_1 DAISY Register"]
    #[inline(always)]
    pub const fn lpspi2_ipp_ind_lpspi_pcs_select_input_1(
        self,
    ) -> crate::common::Reg<regs::Lpspi2IppIndLpspiPcsSelectInput1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
    }
    #[doc = "LPSPI2_IPP_IND_LPSPI_PCS_SELECT_INPUT_3 DAISY Register"]
    #[inline(always)]
    pub const fn lpspi2_ipp_ind_lpspi_pcs_select_input_3(
        self,
    ) -> crate::common::Reg<regs::Lpspi2IppIndLpspiPcsSelectInput3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
    }
    #[doc = "LPSPI2_IPP_IND_LPSPI_SCK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi2_ipp_ind_lpspi_sck_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi2IppIndLpspiSckSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
    #[doc = "LPSPI2_IPP_IND_LPSPI_SDI_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi2_ipp_ind_lpspi_sdi_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi2IppIndLpspiSdiSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
    }
    #[doc = "LPSPI2_IPP_IND_LPSPI_SDO_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi2_ipp_ind_lpspi_sdo_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi2IppIndLpspiSdoSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0128usize) as _) }
    }
    #[doc = "LPTMR1_IPP_IND_LPTIMER_SELECT_INPUT_1 DAISY Register"]
    #[inline(always)]
    pub const fn lptmr1_ipp_ind_lptimer_select_input_1(
        self,
    ) -> crate::common::Reg<regs::Lptmr1IppIndLptimerSelectInput1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x012cusize) as _) }
    }
    #[doc = "LPTMR1_IPP_IND_LPTIMER_SELECT_INPUT_2 DAISY Register"]
    #[inline(always)]
    pub const fn lptmr1_ipp_ind_lptimer_select_input_2(
        self,
    ) -> crate::common::Reg<regs::Lptmr1IppIndLptimerSelectInput2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize) as _) }
    }
    #[doc = "LPTMR1_IPP_IND_LPTIMER_SELECT_INPUT_3 DAISY Register"]
    #[inline(always)]
    pub const fn lptmr1_ipp_ind_lptimer_select_input_3(
        self,
    ) -> crate::common::Reg<regs::Lptmr1IppIndLptimerSelectInput3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0134usize) as _) }
    }
    #[doc = "LPUART1_IPP_IND_LPUART_CTS_N_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart1_ipp_ind_lpuart_cts_n_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart1IppIndLpuartCtsNSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0138usize) as _) }
    }
    #[doc = "LPUART1_IPP_IND_LPUART_DCD_N_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart1_ipp_ind_lpuart_dcd_n_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart1IppIndLpuartDcdNSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x013cusize) as _) }
    }
    #[doc = "LPUART1_IPP_IND_LPUART_DSR_N_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart1_ipp_ind_lpuart_dsr_n_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart1IppIndLpuartDsrNSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[doc = "LPUART12_IPP_IND_LPUART_CTS_N_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart12_ipp_ind_lpuart_cts_n_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart12IppIndLpuartCtsNSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0144usize) as _) }
    }
    #[doc = "LPUART12_IPP_IND_LPUART_RXD_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart12_ipp_ind_lpuart_rxd_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart12IppIndLpuartRxdSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0148usize) as _) }
    }
    #[doc = "LPUART12_IPP_IND_LPUART_TXD_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart12_ipp_ind_lpuart_txd_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart12IppIndLpuartTxdSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x014cusize) as _) }
    }
    #[doc = "LPUART2_IPP_IND_LPUART_CTS_N_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart2_ipp_ind_lpuart_cts_n_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart2IppIndLpuartCtsNSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0150usize) as _) }
    }
    #[doc = "LPUART2_IPP_IND_LPUART_RXD_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart2_ipp_ind_lpuart_rxd_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart2IppIndLpuartRxdSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0154usize) as _) }
    }
    #[doc = "LPUART2_IPP_IND_LPUART_TXD_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart2_ipp_ind_lpuart_txd_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart2IppIndLpuartTxdSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0158usize) as _) }
    }
    #[doc = "LPUART7_IPP_IND_LPUART_CTS_N_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart7_ipp_ind_lpuart_cts_n_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart7IppIndLpuartCtsNSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x015cusize) as _) }
    }
    #[doc = "LPUART7_IPP_IND_LPUART_RXD_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart7_ipp_ind_lpuart_rxd_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart7IppIndLpuartRxdSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0160usize) as _) }
    }
    #[doc = "LPUART7_IPP_IND_LPUART_TXD_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart7_ipp_ind_lpuart_txd_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart7IppIndLpuartTxdSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0164usize) as _) }
    }
    #[doc = "SAI1_IPG_CLK_SAI_MCLK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai1_ipg_clk_sai_mclk_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai1IpgClkSaiMclkSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0168usize) as _) }
    }
    #[doc = "SAI1_IPP_IND_SAI_RXBCLK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai1_ipp_ind_sai_rxbclk_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai1IppIndSaiRxbclkSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x016cusize) as _) }
    }
    #[doc = "SAI1_IPP_IND_SAI_RXDATA_SELECT_INPUT_0 DAISY Register"]
    #[inline(always)]
    pub const fn sai1_ipp_ind_sai_rxdata_select_input_0(
        self,
    ) -> crate::common::Reg<regs::Sai1IppIndSaiRxdataSelectInput0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0170usize) as _) }
    }
    #[doc = "SAI1_IPP_IND_SAI_RXDATA_SELECT_INPUT_1 DAISY Register"]
    #[inline(always)]
    pub const fn sai1_ipp_ind_sai_rxdata_select_input_1(
        self,
    ) -> crate::common::Reg<regs::Sai1IppIndSaiRxdataSelectInput1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0174usize) as _) }
    }
    #[doc = "SAI1_IPP_IND_SAI_RXSYNC_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai1_ipp_ind_sai_rxsync_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai1IppIndSaiRxsyncSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0178usize) as _) }
    }
    #[doc = "SAI1_IPP_IND_SAI_TXBCLK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai1_ipp_ind_sai_txbclk_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai1IppIndSaiTxbclkSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x017cusize) as _) }
    }
    #[doc = "SAI1_IPP_IND_SAI_TXSYNC_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai1_ipp_ind_sai_txsync_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai1IppIndSaiTxsyncSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
    }
}
pub mod regs;
pub mod vals;
