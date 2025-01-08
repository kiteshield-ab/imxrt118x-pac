#[doc = "IOMUXC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iomuxc {
    ptr: *mut u8,
}
unsafe impl Send for Iomuxc {}
unsafe impl Sync for Iomuxc {}
impl Iomuxc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_00 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b1_00(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB100, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_01 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b1_01(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB101, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_02 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b1_02(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB102, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_03 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b1_03(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB103, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_04 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b1_04(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB104, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_05 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b1_05(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB105, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_06 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b1_06(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB106, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_07 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b1_07(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB107, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_08 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b1_08(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB108, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_09 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b1_09(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB109, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_10 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b1_10(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB110, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_11 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b1_11(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB111, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_12 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b1_12(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB112, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_13 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b1_13(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB113, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_14 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b1_14(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB114, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_15 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b1_15(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB115, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_16 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b1_16(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB116, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_17 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b1_17(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB117, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_18 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b1_18(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB118, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_19 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b1_19(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB119, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_20 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b1_20(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB120, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_21 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b1_21(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB121, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_22 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b1_22(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB122, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_23 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b1_23(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB123, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_24 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b1_24(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB124, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_25 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b1_25(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB125, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_26 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b1_26(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB126, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_27 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b1_27(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB127, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_28 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b1_28(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB128, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_29 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b1_29(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB129, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_30 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b1_30(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB130, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_31 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b1_31(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB131, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_32 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b1_32(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB132, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_33 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b1_33(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB133, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_34 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b1_34(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB134, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_35 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b1_35(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB135, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_36 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b1_36(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB136, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_37 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b1_37(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB137, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_38 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b1_38(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB138, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_39 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b1_39(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB139, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_40 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b1_40(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB140, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B1_41 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b1_41(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB141, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B2_00 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b2_00(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB200, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb8usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B2_01 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b2_01(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB201, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbcusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B2_02 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b2_02(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB202, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B2_03 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b2_03(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB203, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B2_04 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b2_04(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB204, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B2_05 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b2_05(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB205, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xccusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B2_06 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b2_06(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB206, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B2_07 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b2_07(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB207, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd4usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B2_08 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b2_08(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB208, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B2_09 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b2_09(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB209, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdcusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B2_10 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b2_10(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB210, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe0usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B2_11 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b2_11(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB211, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe4usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B2_12 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b2_12(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB212, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe8usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B2_13 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b2_13(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB213, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xecusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B2_14 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b2_14(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB214, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B2_15 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b2_15(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB215, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B2_16 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b2_16(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB216, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf8usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B2_17 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b2_17(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB217, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xfcusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B2_18 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b2_18(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB218, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B2_19 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b2_19(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB219, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_EMC_B2_20 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_emc_b2_20(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioEmcB220, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_00 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_00(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAd00, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_01 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_01(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAd01, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_02 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_02(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAd02, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_03 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_03(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAd03, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_04 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_04(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAd04, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_05 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_05(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAd05, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_06 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_06(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAd06, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_07 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_07(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAd07, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0128usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_08 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_08(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAd08, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x012cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_09 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_09(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAd09, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_10 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_10(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAd10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0134usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_11 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_11(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAd11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0138usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_12 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_12(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAd12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x013cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_13 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_13(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAd13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_14 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_14(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAd14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0144usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_15 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_15(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAd15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0148usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_16 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_16(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAd16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x014cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_17 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_17(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAd17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0150usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_18 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_18(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAd18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0154usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_19 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_19(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAd19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0158usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_20 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_20(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAd20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x015cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_21 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_21(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAd21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0160usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_22 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_22(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAd22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0164usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_23 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_23(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAd23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0168usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_24 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_24(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAd24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x016cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_25 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_25(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAd25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0170usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_26 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_26(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAd26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0174usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_27 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_27(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAd27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0178usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_28 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_28(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAd28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x017cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_29 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_29(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAd29, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_30 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_30(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAd30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_31 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_31(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAd31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0188usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_32 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_32(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAd32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x018cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_33 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_33(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAd33, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0190usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_34 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_34(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAd34, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0194usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_AD_35 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_ad_35(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioAd35, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0198usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_00 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b1_00(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSdB100, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x019cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_01 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b1_01(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSdB101, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a0usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_02 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b1_02(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSdB102, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a4usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_03 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b1_03(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSdB103, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a8usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_04 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b1_04(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSdB104, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01acusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_05 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b1_05(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSdB105, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01b0usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B2_00 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b2_00(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSdB200, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01b4usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B2_01 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b2_01(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSdB201, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01b8usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B2_02 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b2_02(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSdB202, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01bcusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B2_03 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b2_03(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSdB203, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c0usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B2_04 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b2_04(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSdB204, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c4usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B2_05 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b2_05(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSdB205, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c8usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B2_06 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b2_06(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSdB206, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01ccusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B2_07 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b2_07(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSdB207, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01d0usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B2_08 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b2_08(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSdB208, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01d4usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B2_09 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b2_09(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSdB209, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01d8usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B2_10 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b2_10(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSdB210, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01dcusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B2_11 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b2_11(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSdB211, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01e0usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_SD_B2_12_DUMMY SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_sd_b2_12(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioSdB212, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01e4usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_00 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b1_00(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB100, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01e8usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_01 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b1_01(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB101, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01ecusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_02 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b1_02(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB102, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01f0usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_03 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b1_03(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB103, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01f4usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_04 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b1_04(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB104, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01f8usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_05 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b1_05(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB105, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01fcusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_06 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b1_06(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB106, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_07 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b1_07(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB107, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0204usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_08 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b1_08(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB108, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0208usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_09 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b1_09(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB109, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x020cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_10 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b1_10(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB110, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0210usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_11 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b1_11(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB111, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0214usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_12 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b1_12(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB112, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0218usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B1_13 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b1_13(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB113, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x021cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B2_00 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b2_00(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB200, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0220usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B2_01 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b2_01(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB201, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0224usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B2_02 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b2_02(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB202, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0228usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B2_03 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b2_03(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB203, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x022cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B2_04 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b2_04(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB204, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0230usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B2_05 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b2_05(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB205, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0234usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B2_06 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b2_06(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB206, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0238usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B2_07 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b2_07(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB207, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x023cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B2_08 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b2_08(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB208, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0240usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B2_09 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b2_09(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB209, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0244usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B2_10 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b2_10(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB210, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0248usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B2_11 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b2_11(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB211, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x024cusize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B2_12 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b2_12(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB212, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0250usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_GPIO_B2_13 SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_gpio_b2_13(
        self,
    ) -> crate::common::Reg<regs::SwMuxCtlPadGpioB213, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0254usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B1_00 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b1_00(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB100, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0258usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B1_01 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b1_01(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB101, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x025cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B1_02 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b1_02(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB102, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0260usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B1_03 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b1_03(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB103, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0264usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B1_04 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b1_04(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB104, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0268usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B1_05 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b1_05(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB105, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x026cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B1_06 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b1_06(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB106, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0270usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B1_07 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b1_07(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB107, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0274usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B1_08 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b1_08(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB108, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0278usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B1_09 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b1_09(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB109, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x027cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B1_10 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b1_10(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB110, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0280usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B1_11 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b1_11(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB111, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0284usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B1_12 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b1_12(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB112, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0288usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B1_13 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b1_13(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB113, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x028cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B1_14 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b1_14(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB114, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0290usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B1_15 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b1_15(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB115, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0294usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B1_16 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b1_16(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB116, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0298usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B1_17 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b1_17(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB117, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x029cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B1_18 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b1_18(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB118, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02a0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B1_19 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b1_19(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB119, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02a4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B1_20 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b1_20(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB120, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02a8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B1_21 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b1_21(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB121, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02acusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B1_22 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b1_22(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB122, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02b0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B1_23 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b1_23(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB123, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02b4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B1_24 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b1_24(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB124, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02b8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B1_25 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b1_25(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB125, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02bcusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B1_26 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b1_26(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB126, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02c0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B1_27 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b1_27(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB127, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02c4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B1_28 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b1_28(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB128, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02c8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B1_29 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b1_29(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB129, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02ccusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B1_30 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b1_30(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB130, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02d0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B1_31 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b1_31(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB131, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02d4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B1_32 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b1_32(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB132, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02d8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B1_33 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b1_33(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB133, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02dcusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B1_34 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b1_34(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB134, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02e0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B1_35 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b1_35(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB135, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02e4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B1_36 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b1_36(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB136, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02e8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B1_37 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b1_37(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB137, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02ecusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B1_38 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b1_38(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB138, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02f0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B1_39 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b1_39(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB139, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02f4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B1_40 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b1_40(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB140, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02f8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B1_41 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b1_41(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB141, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02fcusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B2_00 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b2_00(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB200, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B2_01 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b2_01(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB201, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B2_02 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b2_02(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB202, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B2_03 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b2_03(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB203, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x030cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B2_04 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b2_04(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB204, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0310usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B2_05 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b2_05(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB205, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0314usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B2_06 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b2_06(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB206, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0318usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B2_07 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b2_07(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB207, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x031cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B2_08 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b2_08(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB208, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0320usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B2_09 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b2_09(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB209, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0324usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B2_10 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b2_10(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB210, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0328usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B2_11 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b2_11(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB211, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x032cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B2_12 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b2_12(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB212, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0330usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B2_13 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b2_13(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB213, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0334usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B2_14 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b2_14(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB214, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0338usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B2_15 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b2_15(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB215, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x033cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B2_16 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b2_16(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB216, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0340usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B2_17 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b2_17(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB217, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0344usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B2_18 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b2_18(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB218, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0348usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B2_19 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b2_19(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB219, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x034cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_EMC_B2_20 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_emc_b2_20(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioEmcB220, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0350usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_00 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_00(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAd00, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0354usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_01 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_01(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAd01, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0358usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_02 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_02(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAd02, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x035cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_03 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_03(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAd03, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0360usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_04 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_04(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAd04, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0364usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_05 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_05(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAd05, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0368usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_06 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_06(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAd06, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x036cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_07 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_07(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAd07, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0370usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_08 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_08(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAd08, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0374usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_09 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_09(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAd09, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0378usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_10 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_10(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAd10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x037cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_11 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_11(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAd11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0380usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_12 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_12(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAd12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0384usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_13 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_13(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAd13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0388usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_14 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_14(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAd14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x038cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_15 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_15(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAd15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0390usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_16 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_16(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAd16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0394usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_17 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_17(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAd17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0398usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_18 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_18(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAd18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x039cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_19 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_19(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAd19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03a0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_20 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_20(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAd20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03a4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_21 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_21(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAd21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03a8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_22 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_22(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAd22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03acusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_23 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_23(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAd23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03b0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_24 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_24(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAd24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03b4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_25 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_25(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAd25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03b8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_26 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_26(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAd26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03bcusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_27 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_27(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAd27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03c0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_28 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_28(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAd28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03c4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_29 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_29(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAd29, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03c8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_30 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_30(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAd30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03ccusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_31 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_31(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAd31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03d0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_32 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_32(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAd32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03d4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_33 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_33(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAd33, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03d8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_34 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_34(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAd34, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03dcusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_AD_35 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_ad_35(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioAd35, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03e0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_00 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b1_00(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSdB100, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03e4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_01 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b1_01(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSdB101, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03e8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_02 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b1_02(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSdB102, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03ecusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_03 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b1_03(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSdB103, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03f0usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_04 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b1_04(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSdB104, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03f4usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_05 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b1_05(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSdB105, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03f8usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B2_00 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b2_00(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSdB200, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03fcusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B2_01 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b2_01(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSdB201, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B2_02 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b2_02(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSdB202, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0404usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B2_03 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b2_03(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSdB203, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0408usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B2_04 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b2_04(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSdB204, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x040cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B2_05 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b2_05(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSdB205, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0410usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B2_06 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b2_06(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSdB206, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0414usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B2_07 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b2_07(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSdB207, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0418usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B2_08 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b2_08(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSdB208, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x041cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B2_09 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b2_09(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSdB209, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0420usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B2_10 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b2_10(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSdB210, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0424usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B2_11 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b2_11(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSdB211, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0428usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_SD_B2_12_DUMMY SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_sd_b2_12(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioSdB212, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x042cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_00 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_00(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioB100, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0430usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_01 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_01(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioB101, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0434usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_02 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_02(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioB102, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0438usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_03 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_03(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioB103, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x043cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_04 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_04(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioB104, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0440usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_05 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_05(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioB105, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0444usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_06 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_06(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioB106, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0448usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_07 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_07(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioB107, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x044cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_08 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_08(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioB108, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0450usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_09 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_09(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioB109, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0454usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_10 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_10(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioB110, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0458usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_11 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_11(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioB111, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x045cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_12 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_12(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioB112, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0460usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B1_13 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b1_13(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioB113, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0464usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B2_00 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b2_00(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioB200, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0468usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B2_01 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b2_01(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioB201, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x046cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B2_02 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b2_02(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioB202, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0470usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B2_03 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b2_03(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioB203, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0474usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B2_04 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b2_04(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioB204, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0478usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B2_05 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b2_05(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioB205, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x047cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B2_06 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b2_06(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioB206, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0480usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B2_07 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b2_07(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioB207, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0484usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B2_08 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b2_08(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioB208, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0488usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B2_09 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b2_09(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioB209, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x048cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B2_10 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b2_10(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioB210, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0490usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B2_11 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b2_11(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioB211, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0494usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B2_12 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b2_12(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioB212, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0498usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_GPIO_B2_13 SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_gpio_b2_13(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadGpioB213, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x049cusize) as _) }
    }
    #[doc = "CAN1_IPP_IND_CANRX_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn can1_ipp_ind_canrx_select_input(
        self,
    ) -> crate::common::Reg<regs::Can1IppIndCanrxSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04a0usize) as _) }
    }
    #[doc = "CAN2_IPP_IND_CANRX_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn can2_ipp_ind_canrx_select_input(
        self,
    ) -> crate::common::Reg<regs::Can2IppIndCanrxSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04a4usize) as _) }
    }
    #[doc = "CAN3_IPP_IND_CANRX_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn can3_ipp_ind_canrx_select_input(
        self,
    ) -> crate::common::Reg<regs::Can3IppIndCanrxSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04a8usize) as _) }
    }
    #[doc = "ECAT_ECAT_RX_CLK_0_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn ecat_ecat_rx_clk_0_select_input(
        self,
    ) -> crate::common::Reg<regs::EcatEcatRxClk0SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04acusize) as _) }
    }
    #[doc = "ECAT_ECAT_RX_CLK_1_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn ecat_ecat_rx_clk_1_select_input(
        self,
    ) -> crate::common::Reg<regs::EcatEcatRxClk1SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04b0usize) as _) }
    }
    #[doc = "ECAT_ECAT_RX_DATA0_0_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn ecat_ecat_rx_data0_0_select_input(
        self,
    ) -> crate::common::Reg<regs::EcatEcatRxData00SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04b4usize) as _) }
    }
    #[doc = "ECAT_ECAT_RX_DATA0_1_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn ecat_ecat_rx_data0_1_select_input(
        self,
    ) -> crate::common::Reg<regs::EcatEcatRxData01SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04b8usize) as _) }
    }
    #[doc = "ECAT_ECAT_RX_DATA1_0_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn ecat_ecat_rx_data1_0_select_input(
        self,
    ) -> crate::common::Reg<regs::EcatEcatRxData10SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04bcusize) as _) }
    }
    #[doc = "ECAT_ECAT_RX_DATA1_1_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn ecat_ecat_rx_data1_1_select_input(
        self,
    ) -> crate::common::Reg<regs::EcatEcatRxData11SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04c0usize) as _) }
    }
    #[doc = "ECAT_ECAT_RX_DATA2_0_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn ecat_ecat_rx_data2_0_select_input(
        self,
    ) -> crate::common::Reg<regs::EcatEcatRxData20SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04c4usize) as _) }
    }
    #[doc = "ECAT_ECAT_RX_DATA2_1_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn ecat_ecat_rx_data2_1_select_input(
        self,
    ) -> crate::common::Reg<regs::EcatEcatRxData21SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04c8usize) as _) }
    }
    #[doc = "ECAT_ECAT_RX_DATA3_0_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn ecat_ecat_rx_data3_0_select_input(
        self,
    ) -> crate::common::Reg<regs::EcatEcatRxData30SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04ccusize) as _) }
    }
    #[doc = "ECAT_ECAT_RX_DATA3_1_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn ecat_ecat_rx_data3_1_select_input(
        self,
    ) -> crate::common::Reg<regs::EcatEcatRxData31SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04d0usize) as _) }
    }
    #[doc = "ECAT_ECAT_RX_DV_0_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn ecat_ecat_rx_dv_0_select_input(
        self,
    ) -> crate::common::Reg<regs::EcatEcatRxDv0SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04d4usize) as _) }
    }
    #[doc = "ECAT_ECAT_RX_DV_1_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn ecat_ecat_rx_dv_1_select_input(
        self,
    ) -> crate::common::Reg<regs::EcatEcatRxDv1SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04d8usize) as _) }
    }
    #[doc = "ECAT_ECAT_RX_ER_0_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn ecat_ecat_rx_er_0_select_input(
        self,
    ) -> crate::common::Reg<regs::EcatEcatRxEr0SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04dcusize) as _) }
    }
    #[doc = "ECAT_ECAT_RX_ER_1_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn ecat_ecat_rx_er_1_select_input(
        self,
    ) -> crate::common::Reg<regs::EcatEcatRxEr1SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04e0usize) as _) }
    }
    #[doc = "ECAT_ECAT_TX_CLK_0_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn ecat_ecat_tx_clk_0_select_input(
        self,
    ) -> crate::common::Reg<regs::EcatEcatTxClk0SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04e4usize) as _) }
    }
    #[doc = "ECAT_ECAT_TX_CLK_1_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn ecat_ecat_tx_clk_1_select_input(
        self,
    ) -> crate::common::Reg<regs::EcatEcatTxClk1SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04e8usize) as _) }
    }
    #[doc = "ECAT_MDIO_DATA_IN_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn ecat_mdio_data_in_select_input(
        self,
    ) -> crate::common::Reg<regs::EcatMdioDataInSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04ecusize) as _) }
    }
    #[doc = "ECAT_PROM_DATA_IN_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn ecat_prom_data_in_select_input(
        self,
    ) -> crate::common::Reg<regs::EcatPromDataInSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04f0usize) as _) }
    }
    #[doc = "FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_0 DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm1_ipp_ind_pwma_select_input_0(
        self,
    ) -> crate::common::Reg<regs::Flexpwm1IppIndPwmaSelectInput0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04f4usize) as _) }
    }
    #[doc = "FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_1 DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm1_ipp_ind_pwma_select_input_1(
        self,
    ) -> crate::common::Reg<regs::Flexpwm1IppIndPwmaSelectInput1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04f8usize) as _) }
    }
    #[doc = "FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_2 DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm1_ipp_ind_pwma_select_input_2(
        self,
    ) -> crate::common::Reg<regs::Flexpwm1IppIndPwmaSelectInput2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04fcusize) as _) }
    }
    #[doc = "FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_0 DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm1_ipp_ind_pwmb_select_input_0(
        self,
    ) -> crate::common::Reg<regs::Flexpwm1IppIndPwmbSelectInput0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
    }
    #[doc = "FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_1 DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm1_ipp_ind_pwmb_select_input_1(
        self,
    ) -> crate::common::Reg<regs::Flexpwm1IppIndPwmbSelectInput1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
    }
    #[doc = "FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_2 DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm1_ipp_ind_pwmb_select_input_2(
        self,
    ) -> crate::common::Reg<regs::Flexpwm1IppIndPwmbSelectInput2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
    }
    #[doc = "FLEXPWM2_IPP_IND_PWMA_SELECT_INPUT_0 DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm2_ipp_ind_pwma_select_input_0(
        self,
    ) -> crate::common::Reg<regs::Flexpwm2IppIndPwmaSelectInput0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
    }
    #[doc = "FLEXPWM2_IPP_IND_PWMA_SELECT_INPUT_1 DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm2_ipp_ind_pwma_select_input_1(
        self,
    ) -> crate::common::Reg<regs::Flexpwm2IppIndPwmaSelectInput1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
    }
    #[doc = "FLEXPWM2_IPP_IND_PWMA_SELECT_INPUT_2 DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm2_ipp_ind_pwma_select_input_2(
        self,
    ) -> crate::common::Reg<regs::Flexpwm2IppIndPwmaSelectInput2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0514usize) as _) }
    }
    #[doc = "FLEXPWM2_IPP_IND_PWMB_SELECT_INPUT_0 DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm2_ipp_ind_pwmb_select_input_0(
        self,
    ) -> crate::common::Reg<regs::Flexpwm2IppIndPwmbSelectInput0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
    }
    #[doc = "FLEXPWM2_IPP_IND_PWMB_SELECT_INPUT_1 DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm2_ipp_ind_pwmb_select_input_1(
        self,
    ) -> crate::common::Reg<regs::Flexpwm2IppIndPwmbSelectInput1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x051cusize) as _) }
    }
    #[doc = "FLEXPWM2_IPP_IND_PWMB_SELECT_INPUT_2 DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm2_ipp_ind_pwmb_select_input_2(
        self,
    ) -> crate::common::Reg<regs::Flexpwm2IppIndPwmbSelectInput2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0520usize) as _) }
    }
    #[doc = "FLEXPWM3_IPP_IND_PWMA_SELECT_INPUT_0 DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm3_ipp_ind_pwma_select_input_0(
        self,
    ) -> crate::common::Reg<regs::Flexpwm3IppIndPwmaSelectInput0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0524usize) as _) }
    }
    #[doc = "FLEXPWM3_IPP_IND_PWMA_SELECT_INPUT_1 DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm3_ipp_ind_pwma_select_input_1(
        self,
    ) -> crate::common::Reg<regs::Flexpwm3IppIndPwmaSelectInput1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0528usize) as _) }
    }
    #[doc = "FLEXPWM3_IPP_IND_PWMA_SELECT_INPUT_2 DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm3_ipp_ind_pwma_select_input_2(
        self,
    ) -> crate::common::Reg<regs::Flexpwm3IppIndPwmaSelectInput2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x052cusize) as _) }
    }
    #[doc = "FLEXPWM3_IPP_IND_PWMA_SELECT_INPUT_3 DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm3_ipp_ind_pwma_select_input_3(
        self,
    ) -> crate::common::Reg<regs::Flexpwm3IppIndPwmaSelectInput3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0530usize) as _) }
    }
    #[doc = "FLEXPWM3_IPP_IND_PWMB_SELECT_INPUT_0 DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm3_ipp_ind_pwmb_select_input_0(
        self,
    ) -> crate::common::Reg<regs::Flexpwm3IppIndPwmbSelectInput0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0534usize) as _) }
    }
    #[doc = "FLEXPWM3_IPP_IND_PWMB_SELECT_INPUT_1 DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm3_ipp_ind_pwmb_select_input_1(
        self,
    ) -> crate::common::Reg<regs::Flexpwm3IppIndPwmbSelectInput1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0538usize) as _) }
    }
    #[doc = "FLEXPWM3_IPP_IND_PWMB_SELECT_INPUT_2 DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm3_ipp_ind_pwmb_select_input_2(
        self,
    ) -> crate::common::Reg<regs::Flexpwm3IppIndPwmbSelectInput2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x053cusize) as _) }
    }
    #[doc = "FLEXPWM3_IPP_IND_PWMB_SELECT_INPUT_3 DAISY Register"]
    #[inline(always)]
    pub const fn flexpwm3_ipp_ind_pwmb_select_input_3(
        self,
    ) -> crate::common::Reg<regs::Flexpwm3IppIndPwmbSelectInput3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0540usize) as _) }
    }
    #[doc = "FLEXSPI1_BUS2BIT_IPP_IND_DQS_FA_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspi1_bus2bit_ipp_ind_dqs_fa_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexspi1Bus2bitIppIndDqsFaSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0544usize) as _) }
    }
    #[doc = "FLEXSPI1_BUS2BIT_IPP_IND_DQS_FB_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspi1_bus2bit_ipp_ind_dqs_fb_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexspi1Bus2bitIppIndDqsFbSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0548usize) as _) }
    }
    #[doc = "FLEXSPI1_BUS2BIT_IPP_IND_IO_FB_BIT0_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspi1_bus2bit_ipp_ind_io_fb_bit0_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexspi1Bus2bitIppIndIoFbBit0SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x054cusize) as _) }
    }
    #[doc = "FLEXSPI1_BUS2BIT_IPP_IND_IO_FB_BIT1_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspi1_bus2bit_ipp_ind_io_fb_bit1_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexspi1Bus2bitIppIndIoFbBit1SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0550usize) as _) }
    }
    #[doc = "FLEXSPI1_BUS2BIT_IPP_IND_IO_FB_BIT2_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspi1_bus2bit_ipp_ind_io_fb_bit2_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexspi1Bus2bitIppIndIoFbBit2SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0554usize) as _) }
    }
    #[doc = "FLEXSPI1_BUS2BIT_IPP_IND_IO_FB_BIT3_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspi1_bus2bit_ipp_ind_io_fb_bit3_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexspi1Bus2bitIppIndIoFbBit3SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0558usize) as _) }
    }
    #[doc = "FLEXSPI1_BUS2BIT_IPP_IND_IO_FB_BIT4_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspi1_bus2bit_ipp_ind_io_fb_bit4_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexspi1Bus2bitIppIndIoFbBit4SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x055cusize) as _) }
    }
    #[doc = "FLEXSPI1_BUS2BIT_IPP_IND_IO_FB_BIT5_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspi1_bus2bit_ipp_ind_io_fb_bit5_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexspi1Bus2bitIppIndIoFbBit5SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0560usize) as _) }
    }
    #[doc = "FLEXSPI1_BUS2BIT_IPP_IND_IO_FB_BIT6_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspi1_bus2bit_ipp_ind_io_fb_bit6_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexspi1Bus2bitIppIndIoFbBit6SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0564usize) as _) }
    }
    #[doc = "FLEXSPI1_BUS2BIT_IPP_IND_IO_FB_BIT7_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspi1_bus2bit_ipp_ind_io_fb_bit7_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexspi1Bus2bitIppIndIoFbBit7SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0568usize) as _) }
    }
    #[doc = "FLEXSPI1_BUS2BIT_IPP_IND_SCK_FB_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspi1_bus2bit_ipp_ind_sck_fb_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexspi1Bus2bitIppIndSckFbSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x056cusize) as _) }
    }
    #[doc = "FLEXSPI2_BUS2BIT_IPP_IND_DQS_FA_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspi2_bus2bit_ipp_ind_dqs_fa_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexspi2Bus2bitIppIndDqsFaSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0570usize) as _) }
    }
    #[doc = "FLEXSPI2_BUS2BIT_IPP_IND_DQS_FB_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspi2_bus2bit_ipp_ind_dqs_fb_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexspi2Bus2bitIppIndDqsFbSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0574usize) as _) }
    }
    #[doc = "FLEXSPI2_BUS2BIT_IPP_IND_IO_FA_BIT0_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspi2_bus2bit_ipp_ind_io_fa_bit0_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexspi2Bus2bitIppIndIoFaBit0SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0578usize) as _) }
    }
    #[doc = "FLEXSPI2_BUS2BIT_IPP_IND_IO_FA_BIT1_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspi2_bus2bit_ipp_ind_io_fa_bit1_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexspi2Bus2bitIppIndIoFaBit1SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x057cusize) as _) }
    }
    #[doc = "FLEXSPI2_BUS2BIT_IPP_IND_IO_FA_BIT2_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspi2_bus2bit_ipp_ind_io_fa_bit2_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexspi2Bus2bitIppIndIoFaBit2SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0580usize) as _) }
    }
    #[doc = "FLEXSPI2_BUS2BIT_IPP_IND_IO_FA_BIT3_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspi2_bus2bit_ipp_ind_io_fa_bit3_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexspi2Bus2bitIppIndIoFaBit3SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0584usize) as _) }
    }
    #[doc = "FLEXSPI2_BUS2BIT_IPP_IND_IO_FB_BIT0_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspi2_bus2bit_ipp_ind_io_fb_bit0_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexspi2Bus2bitIppIndIoFbBit0SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0588usize) as _) }
    }
    #[doc = "FLEXSPI2_BUS2BIT_IPP_IND_IO_FB_BIT1_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspi2_bus2bit_ipp_ind_io_fb_bit1_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexspi2Bus2bitIppIndIoFbBit1SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x058cusize) as _) }
    }
    #[doc = "FLEXSPI2_BUS2BIT_IPP_IND_IO_FB_BIT2_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspi2_bus2bit_ipp_ind_io_fb_bit2_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexspi2Bus2bitIppIndIoFbBit2SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0590usize) as _) }
    }
    #[doc = "FLEXSPI2_BUS2BIT_IPP_IND_IO_FB_BIT3_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspi2_bus2bit_ipp_ind_io_fb_bit3_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexspi2Bus2bitIppIndIoFbBit3SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0594usize) as _) }
    }
    #[doc = "FLEXSPI2_BUS2BIT_IPP_IND_SCK_FA_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspi2_bus2bit_ipp_ind_sck_fa_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexspi2Bus2bitIppIndSckFaSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0598usize) as _) }
    }
    #[doc = "FLEXSPI2_BUS2BIT_IPP_IND_SCK_FB_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn flexspi2_bus2bit_ipp_ind_sck_fb_select_input(
        self,
    ) -> crate::common::Reg<regs::Flexspi2Bus2bitIppIndSckFbSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x059cusize) as _) }
    }
    #[doc = "I3C2_PIN_SCL_IN_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn i3c2_pin_scl_in_select_input(
        self,
    ) -> crate::common::Reg<regs::I3c2PinSclInSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05a0usize) as _) }
    }
    #[doc = "I3C2_PIN_SDA_IN_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn i3c2_pin_sda_in_select_input(
        self,
    ) -> crate::common::Reg<regs::I3c2PinSdaInSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05a4usize) as _) }
    }
    #[doc = "KPP_IPP_IND_COL_SELECT_INPUT_0 DAISY Register"]
    #[inline(always)]
    pub const fn kpp_ipp_ind_col_select_input_0(
        self,
    ) -> crate::common::Reg<regs::KppIppIndColSelectInput0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05a8usize) as _) }
    }
    #[doc = "KPP_IPP_IND_COL_SELECT_INPUT_1 DAISY Register"]
    #[inline(always)]
    pub const fn kpp_ipp_ind_col_select_input_1(
        self,
    ) -> crate::common::Reg<regs::KppIppIndColSelectInput1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05acusize) as _) }
    }
    #[doc = "KPP_IPP_IND_COL_SELECT_INPUT_2 DAISY Register"]
    #[inline(always)]
    pub const fn kpp_ipp_ind_col_select_input_2(
        self,
    ) -> crate::common::Reg<regs::KppIppIndColSelectInput2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05b0usize) as _) }
    }
    #[doc = "KPP_IPP_IND_COL_SELECT_INPUT_3 DAISY Register"]
    #[inline(always)]
    pub const fn kpp_ipp_ind_col_select_input_3(
        self,
    ) -> crate::common::Reg<regs::KppIppIndColSelectInput3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05b4usize) as _) }
    }
    #[doc = "KPP_IPP_IND_COL_SELECT_INPUT_4 DAISY Register"]
    #[inline(always)]
    pub const fn kpp_ipp_ind_col_select_input_4(
        self,
    ) -> crate::common::Reg<regs::KppIppIndColSelectInput4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05b8usize) as _) }
    }
    #[doc = "KPP_IPP_IND_COL_SELECT_INPUT_5 DAISY Register"]
    #[inline(always)]
    pub const fn kpp_ipp_ind_col_select_input_5(
        self,
    ) -> crate::common::Reg<regs::KppIppIndColSelectInput5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05bcusize) as _) }
    }
    #[doc = "KPP_IPP_IND_COL_SELECT_INPUT_6 DAISY Register"]
    #[inline(always)]
    pub const fn kpp_ipp_ind_col_select_input_6(
        self,
    ) -> crate::common::Reg<regs::KppIppIndColSelectInput6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05c0usize) as _) }
    }
    #[doc = "KPP_IPP_IND_COL_SELECT_INPUT_7 DAISY Register"]
    #[inline(always)]
    pub const fn kpp_ipp_ind_col_select_input_7(
        self,
    ) -> crate::common::Reg<regs::KppIppIndColSelectInput7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05c4usize) as _) }
    }
    #[doc = "KPP_IPP_IND_ROW_SELECT_INPUT_0 DAISY Register"]
    #[inline(always)]
    pub const fn kpp_ipp_ind_row_select_input_0(
        self,
    ) -> crate::common::Reg<regs::KppIppIndRowSelectInput0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05c8usize) as _) }
    }
    #[doc = "KPP_IPP_IND_ROW_SELECT_INPUT_1 DAISY Register"]
    #[inline(always)]
    pub const fn kpp_ipp_ind_row_select_input_1(
        self,
    ) -> crate::common::Reg<regs::KppIppIndRowSelectInput1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05ccusize) as _) }
    }
    #[doc = "KPP_IPP_IND_ROW_SELECT_INPUT_2 DAISY Register"]
    #[inline(always)]
    pub const fn kpp_ipp_ind_row_select_input_2(
        self,
    ) -> crate::common::Reg<regs::KppIppIndRowSelectInput2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05d0usize) as _) }
    }
    #[doc = "KPP_IPP_IND_ROW_SELECT_INPUT_3 DAISY Register"]
    #[inline(always)]
    pub const fn kpp_ipp_ind_row_select_input_3(
        self,
    ) -> crate::common::Reg<regs::KppIppIndRowSelectInput3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05d4usize) as _) }
    }
    #[doc = "KPP_IPP_IND_ROW_SELECT_INPUT_4 DAISY Register"]
    #[inline(always)]
    pub const fn kpp_ipp_ind_row_select_input_4(
        self,
    ) -> crate::common::Reg<regs::KppIppIndRowSelectInput4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05d8usize) as _) }
    }
    #[doc = "KPP_IPP_IND_ROW_SELECT_INPUT_5 DAISY Register"]
    #[inline(always)]
    pub const fn kpp_ipp_ind_row_select_input_5(
        self,
    ) -> crate::common::Reg<regs::KppIppIndRowSelectInput5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05dcusize) as _) }
    }
    #[doc = "KPP_IPP_IND_ROW_SELECT_INPUT_6 DAISY Register"]
    #[inline(always)]
    pub const fn kpp_ipp_ind_row_select_input_6(
        self,
    ) -> crate::common::Reg<regs::KppIppIndRowSelectInput6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05e0usize) as _) }
    }
    #[doc = "KPP_IPP_IND_ROW_SELECT_INPUT_7 DAISY Register"]
    #[inline(always)]
    pub const fn kpp_ipp_ind_row_select_input_7(
        self,
    ) -> crate::common::Reg<regs::KppIppIndRowSelectInput7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05e4usize) as _) }
    }
    #[doc = "LPI2C3_IPP_IND_LPI2C_SCL_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpi2c3_ipp_ind_lpi2c_scl_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpi2c3IppIndLpi2cSclSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05e8usize) as _) }
    }
    #[doc = "LPI2C3_IPP_IND_LPI2C_SDA_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpi2c3_ipp_ind_lpi2c_sda_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpi2c3IppIndLpi2cSdaSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05ecusize) as _) }
    }
    #[doc = "LPI2C4_IPP_IND_LPI2C_SCL_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpi2c4_ipp_ind_lpi2c_scl_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpi2c4IppIndLpi2cSclSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05f0usize) as _) }
    }
    #[doc = "LPI2C4_IPP_IND_LPI2C_SDA_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpi2c4_ipp_ind_lpi2c_sda_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpi2c4IppIndLpi2cSdaSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05f4usize) as _) }
    }
    #[doc = "LPI2C5_IPP_IND_LPI2C_SCL_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpi2c5_ipp_ind_lpi2c_scl_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpi2c5IppIndLpi2cSclSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05f8usize) as _) }
    }
    #[doc = "LPI2C5_IPP_IND_LPI2C_SDA_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpi2c5_ipp_ind_lpi2c_sda_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpi2c5IppIndLpi2cSdaSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05fcusize) as _) }
    }
    #[doc = "LPI2C6_IPP_IND_LPI2C_SCL_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpi2c6_ipp_ind_lpi2c_scl_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpi2c6IppIndLpi2cSclSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0600usize) as _) }
    }
    #[doc = "LPI2C6_IPP_IND_LPI2C_SDA_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpi2c6_ipp_ind_lpi2c_sda_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpi2c6IppIndLpi2cSdaSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0604usize) as _) }
    }
    #[doc = "LPSPI3_IPP_IND_LPSPI_PCS_SELECT_INPUT_0 DAISY Register"]
    #[inline(always)]
    pub const fn lpspi3_ipp_ind_lpspi_pcs_select_input_0(
        self,
    ) -> crate::common::Reg<regs::Lpspi3IppIndLpspiPcsSelectInput0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0608usize) as _) }
    }
    #[doc = "LPSPI3_IPP_IND_LPSPI_PCS_SELECT_INPUT_1 DAISY Register"]
    #[inline(always)]
    pub const fn lpspi3_ipp_ind_lpspi_pcs_select_input_1(
        self,
    ) -> crate::common::Reg<regs::Lpspi3IppIndLpspiPcsSelectInput1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x060cusize) as _) }
    }
    #[doc = "LPSPI3_IPP_IND_LPSPI_PCS_SELECT_INPUT_2 DAISY Register"]
    #[inline(always)]
    pub const fn lpspi3_ipp_ind_lpspi_pcs_select_input_2(
        self,
    ) -> crate::common::Reg<regs::Lpspi3IppIndLpspiPcsSelectInput2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0610usize) as _) }
    }
    #[doc = "LPSPI3_IPP_IND_LPSPI_PCS_SELECT_INPUT_3 DAISY Register"]
    #[inline(always)]
    pub const fn lpspi3_ipp_ind_lpspi_pcs_select_input_3(
        self,
    ) -> crate::common::Reg<regs::Lpspi3IppIndLpspiPcsSelectInput3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0614usize) as _) }
    }
    #[doc = "LPSPI3_IPP_IND_LPSPI_SCK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi3_ipp_ind_lpspi_sck_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi3IppIndLpspiSckSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0618usize) as _) }
    }
    #[doc = "LPSPI3_IPP_IND_LPSPI_SDI_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi3_ipp_ind_lpspi_sdi_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi3IppIndLpspiSdiSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x061cusize) as _) }
    }
    #[doc = "LPSPI3_IPP_IND_LPSPI_SDO_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi3_ipp_ind_lpspi_sdo_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi3IppIndLpspiSdoSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0620usize) as _) }
    }
    #[doc = "LPSPI4_IPP_IND_LPSPI_PCS_SELECT_INPUT_0 DAISY Register"]
    #[inline(always)]
    pub const fn lpspi4_ipp_ind_lpspi_pcs_select_input_0(
        self,
    ) -> crate::common::Reg<regs::Lpspi4IppIndLpspiPcsSelectInput0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0624usize) as _) }
    }
    #[doc = "LPSPI4_IPP_IND_LPSPI_SCK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi4_ipp_ind_lpspi_sck_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi4IppIndLpspiSckSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0628usize) as _) }
    }
    #[doc = "LPSPI4_IPP_IND_LPSPI_SDI_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi4_ipp_ind_lpspi_sdi_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi4IppIndLpspiSdiSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x062cusize) as _) }
    }
    #[doc = "LPSPI4_IPP_IND_LPSPI_SDO_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi4_ipp_ind_lpspi_sdo_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi4IppIndLpspiSdoSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0630usize) as _) }
    }
    #[doc = "LPSPI5_IPP_IND_LPSPI_PCS_SELECT_INPUT_0 DAISY Register"]
    #[inline(always)]
    pub const fn lpspi5_ipp_ind_lpspi_pcs_select_input_0(
        self,
    ) -> crate::common::Reg<regs::Lpspi5IppIndLpspiPcsSelectInput0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0634usize) as _) }
    }
    #[doc = "LPSPI5_IPP_IND_LPSPI_PCS_SELECT_INPUT_1 DAISY Register"]
    #[inline(always)]
    pub const fn lpspi5_ipp_ind_lpspi_pcs_select_input_1(
        self,
    ) -> crate::common::Reg<regs::Lpspi5IppIndLpspiPcsSelectInput1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0638usize) as _) }
    }
    #[doc = "LPSPI5_IPP_IND_LPSPI_PCS_SELECT_INPUT_2 DAISY Register"]
    #[inline(always)]
    pub const fn lpspi5_ipp_ind_lpspi_pcs_select_input_2(
        self,
    ) -> crate::common::Reg<regs::Lpspi5IppIndLpspiPcsSelectInput2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x063cusize) as _) }
    }
    #[doc = "LPSPI5_IPP_IND_LPSPI_PCS_SELECT_INPUT_3 DAISY Register"]
    #[inline(always)]
    pub const fn lpspi5_ipp_ind_lpspi_pcs_select_input_3(
        self,
    ) -> crate::common::Reg<regs::Lpspi5IppIndLpspiPcsSelectInput3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0640usize) as _) }
    }
    #[doc = "LPSPI5_IPP_IND_LPSPI_SCK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi5_ipp_ind_lpspi_sck_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi5IppIndLpspiSckSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0644usize) as _) }
    }
    #[doc = "LPSPI5_IPP_IND_LPSPI_SDI_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi5_ipp_ind_lpspi_sdi_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi5IppIndLpspiSdiSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0648usize) as _) }
    }
    #[doc = "LPSPI5_IPP_IND_LPSPI_SDO_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi5_ipp_ind_lpspi_sdo_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi5IppIndLpspiSdoSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x064cusize) as _) }
    }
    #[doc = "LPSPI6_IPP_IND_LPSPI_PCS_SELECT_INPUT_0 DAISY Register"]
    #[inline(always)]
    pub const fn lpspi6_ipp_ind_lpspi_pcs_select_input_0(
        self,
    ) -> crate::common::Reg<regs::Lpspi6IppIndLpspiPcsSelectInput0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0650usize) as _) }
    }
    #[doc = "LPSPI6_IPP_IND_LPSPI_PCS_SELECT_INPUT_1 DAISY Register"]
    #[inline(always)]
    pub const fn lpspi6_ipp_ind_lpspi_pcs_select_input_1(
        self,
    ) -> crate::common::Reg<regs::Lpspi6IppIndLpspiPcsSelectInput1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0654usize) as _) }
    }
    #[doc = "LPSPI6_IPP_IND_LPSPI_PCS_SELECT_INPUT_2 DAISY Register"]
    #[inline(always)]
    pub const fn lpspi6_ipp_ind_lpspi_pcs_select_input_2(
        self,
    ) -> crate::common::Reg<regs::Lpspi6IppIndLpspiPcsSelectInput2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0658usize) as _) }
    }
    #[doc = "LPSPI6_IPP_IND_LPSPI_PCS_SELECT_INPUT_3 DAISY Register"]
    #[inline(always)]
    pub const fn lpspi6_ipp_ind_lpspi_pcs_select_input_3(
        self,
    ) -> crate::common::Reg<regs::Lpspi6IppIndLpspiPcsSelectInput3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x065cusize) as _) }
    }
    #[doc = "LPSPI6_IPP_IND_LPSPI_SCK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi6_ipp_ind_lpspi_sck_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi6IppIndLpspiSckSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0660usize) as _) }
    }
    #[doc = "LPSPI6_IPP_IND_LPSPI_SDI_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi6_ipp_ind_lpspi_sdi_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi6IppIndLpspiSdiSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0664usize) as _) }
    }
    #[doc = "LPSPI6_IPP_IND_LPSPI_SDO_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpspi6_ipp_ind_lpspi_sdo_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpspi6IppIndLpspiSdoSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0668usize) as _) }
    }
    #[doc = "LPUART10_IPP_IND_LPUART_RXD_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart10_ipp_ind_lpuart_rxd_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart10IppIndLpuartRxdSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x066cusize) as _) }
    }
    #[doc = "LPUART10_IPP_IND_LPUART_TXD_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart10_ipp_ind_lpuart_txd_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart10IppIndLpuartTxdSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0670usize) as _) }
    }
    #[doc = "LPUART11_IPP_IND_LPUART_RXD_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart11_ipp_ind_lpuart_rxd_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart11IppIndLpuartRxdSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0674usize) as _) }
    }
    #[doc = "LPUART11_IPP_IND_LPUART_TXD_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart11_ipp_ind_lpuart_txd_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart11IppIndLpuartTxdSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0678usize) as _) }
    }
    #[doc = "LPUART3_IPP_IND_LPUART_CTS_N_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart3_ipp_ind_lpuart_cts_n_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart3IppIndLpuartCtsNSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x067cusize) as _) }
    }
    #[doc = "LPUART3_IPP_IND_LPUART_RXD_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart3_ipp_ind_lpuart_rxd_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart3IppIndLpuartRxdSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0680usize) as _) }
    }
    #[doc = "LPUART3_IPP_IND_LPUART_TXD_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart3_ipp_ind_lpuart_txd_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart3IppIndLpuartTxdSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0684usize) as _) }
    }
    #[doc = "LPUART4_IPP_IND_LPUART_CTS_N_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart4_ipp_ind_lpuart_cts_n_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart4IppIndLpuartCtsNSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0688usize) as _) }
    }
    #[doc = "LPUART5_IPP_IND_LPUART_CTS_N_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart5_ipp_ind_lpuart_cts_n_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart5IppIndLpuartCtsNSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x068cusize) as _) }
    }
    #[doc = "LPUART5_IPP_IND_LPUART_DCD_N_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart5_ipp_ind_lpuart_dcd_n_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart5IppIndLpuartDcdNSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0690usize) as _) }
    }
    #[doc = "LPUART5_IPP_IND_LPUART_DSR_N_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart5_ipp_ind_lpuart_dsr_n_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart5IppIndLpuartDsrNSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0694usize) as _) }
    }
    #[doc = "LPUART5_IPP_IND_LPUART_RI_N_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart5_ipp_ind_lpuart_ri_n_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart5IppIndLpuartRiNSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0698usize) as _) }
    }
    #[doc = "LPUART5_IPP_IND_LPUART_RXD_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart5_ipp_ind_lpuart_rxd_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart5IppIndLpuartRxdSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x069cusize) as _) }
    }
    #[doc = "LPUART5_IPP_IND_LPUART_TXD_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart5_ipp_ind_lpuart_txd_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart5IppIndLpuartTxdSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06a0usize) as _) }
    }
    #[doc = "LPUART6_IPP_IND_LPUART_CTS_N_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart6_ipp_ind_lpuart_cts_n_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart6IppIndLpuartCtsNSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06a4usize) as _) }
    }
    #[doc = "LPUART6_IPP_IND_LPUART_DCD_N_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart6_ipp_ind_lpuart_dcd_n_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart6IppIndLpuartDcdNSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06a8usize) as _) }
    }
    #[doc = "LPUART6_IPP_IND_LPUART_DSR_N_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart6_ipp_ind_lpuart_dsr_n_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart6IppIndLpuartDsrNSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06acusize) as _) }
    }
    #[doc = "LPUART6_IPP_IND_LPUART_RI_N_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart6_ipp_ind_lpuart_ri_n_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart6IppIndLpuartRiNSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06b0usize) as _) }
    }
    #[doc = "LPUART6_IPP_IND_LPUART_RXD_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart6_ipp_ind_lpuart_rxd_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart6IppIndLpuartRxdSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06b4usize) as _) }
    }
    #[doc = "LPUART6_IPP_IND_LPUART_TXD_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart6_ipp_ind_lpuart_txd_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart6IppIndLpuartTxdSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06b8usize) as _) }
    }
    #[doc = "LPUART8_IPP_IND_LPUART_CTS_N_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart8_ipp_ind_lpuart_cts_n_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart8IppIndLpuartCtsNSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06bcusize) as _) }
    }
    #[doc = "LPUART8_IPP_IND_LPUART_RXD_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart8_ipp_ind_lpuart_rxd_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart8IppIndLpuartRxdSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06c0usize) as _) }
    }
    #[doc = "LPUART8_IPP_IND_LPUART_TXD_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart8_ipp_ind_lpuart_txd_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart8IppIndLpuartTxdSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06c4usize) as _) }
    }
    #[doc = "LPUART9_IPP_IND_LPUART_RXD_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart9_ipp_ind_lpuart_rxd_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart9IppIndLpuartRxdSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06c8usize) as _) }
    }
    #[doc = "LPUART9_IPP_IND_LPUART_TXD_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn lpuart9_ipp_ind_lpuart_txd_select_input(
        self,
    ) -> crate::common::Reg<regs::Lpuart9IppIndLpuartTxdSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06ccusize) as _) }
    }
    #[doc = "MIC_IPP_IND_MIC_PDM_BITSTREAM_SELECT_INPUT_0 DAISY Register"]
    #[inline(always)]
    pub const fn mic_ipp_ind_mic_pdm_bitstream_select_input_0(
        self,
    ) -> crate::common::Reg<regs::MicIppIndMicPdmBitstreamSelectInput0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06d0usize) as _) }
    }
    #[doc = "MIC_IPP_IND_MIC_PDM_BITSTREAM_SELECT_INPUT_1 DAISY Register"]
    #[inline(always)]
    pub const fn mic_ipp_ind_mic_pdm_bitstream_select_input_1(
        self,
    ) -> crate::common::Reg<regs::MicIppIndMicPdmBitstreamSelectInput1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06d4usize) as _) }
    }
    #[doc = "MIC_IPP_IND_MIC_PDM_BITSTREAM_SELECT_INPUT_2 DAISY Register"]
    #[inline(always)]
    pub const fn mic_ipp_ind_mic_pdm_bitstream_select_input_2(
        self,
    ) -> crate::common::Reg<regs::MicIppIndMicPdmBitstreamSelectInput2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06d8usize) as _) }
    }
    #[doc = "MIC_IPP_IND_MIC_PDM_BITSTREAM_SELECT_INPUT_3 DAISY Register"]
    #[inline(always)]
    pub const fn mic_ipp_ind_mic_pdm_bitstream_select_input_3(
        self,
    ) -> crate::common::Reg<regs::MicIppIndMicPdmBitstreamSelectInput3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06dcusize) as _) }
    }
    #[doc = "NETC_EMDIO_IN_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn netc_emdio_in_select_input(
        self,
    ) -> crate::common::Reg<regs::NetcEmdioInSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0798usize) as _) }
    }
    #[doc = "NETC_ETH2_COL_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn netc_eth2_col_select_input(
        self,
    ) -> crate::common::Reg<regs::NetcEth2ColSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x079cusize) as _) }
    }
    #[doc = "NETC_ETH2_CRS_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn netc_eth2_crs_select_input(
        self,
    ) -> crate::common::Reg<regs::NetcEth2CrsSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07a0usize) as _) }
    }
    #[doc = "NETC_ETH2_SLV_MDC_IN_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn netc_eth2_slv_mdc_in_select_input(
        self,
    ) -> crate::common::Reg<regs::NetcEth2SlvMdcInSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07a4usize) as _) }
    }
    #[doc = "NETC_ETH2_SLV_MDIO_IN_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn netc_eth2_slv_mdio_in_select_input(
        self,
    ) -> crate::common::Reg<regs::NetcEth2SlvMdioInSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07a8usize) as _) }
    }
    #[doc = "NETC_ETH3_COL_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn netc_eth3_col_select_input(
        self,
    ) -> crate::common::Reg<regs::NetcEth3ColSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07acusize) as _) }
    }
    #[doc = "NETC_ETH3_CRS_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn netc_eth3_crs_select_input(
        self,
    ) -> crate::common::Reg<regs::NetcEth3CrsSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07b0usize) as _) }
    }
    #[doc = "NETC_ETH3_SLV_MDC_IN_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn netc_eth3_slv_mdc_in_select_input(
        self,
    ) -> crate::common::Reg<regs::NetcEth3SlvMdcInSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07b4usize) as _) }
    }
    #[doc = "NETC_ETH3_SLV_MDIO_IN_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn netc_eth3_slv_mdio_in_select_input(
        self,
    ) -> crate::common::Reg<regs::NetcEth3SlvMdioInSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07b8usize) as _) }
    }
    #[doc = "NETC_ETH4_COL_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn netc_eth4_col_select_input(
        self,
    ) -> crate::common::Reg<regs::NetcEth4ColSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07bcusize) as _) }
    }
    #[doc = "NETC_ETH4_CRS_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn netc_eth4_crs_select_input(
        self,
    ) -> crate::common::Reg<regs::NetcEth4CrsSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07c0usize) as _) }
    }
    #[doc = "NETC_ETH4_SLV_MDC_IN_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn netc_eth4_slv_mdc_in_select_input(
        self,
    ) -> crate::common::Reg<regs::NetcEth4SlvMdcInSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07c4usize) as _) }
    }
    #[doc = "NETC_ETH4_SLV_MDIO_IN_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn netc_eth4_slv_mdio_in_select_input(
        self,
    ) -> crate::common::Reg<regs::NetcEth4SlvMdioInSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07c8usize) as _) }
    }
    #[doc = "NETC_TMR_TRIG1_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn netc_tmr_trig1_select_input(
        self,
    ) -> crate::common::Reg<regs::NetcTmrTrig1SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07ccusize) as _) }
    }
    #[doc = "NETC_TMR_TRIG2_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn netc_tmr_trig2_select_input(
        self,
    ) -> crate::common::Reg<regs::NetcTmrTrig2SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07d0usize) as _) }
    }
    #[doc = "NETC_CLKGEN_IPP_TMR_CLK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn netc_clkgen_ipp_tmr_clk_select_input(
        self,
    ) -> crate::common::Reg<regs::NetcClkgenIppTmrClkSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07d4usize) as _) }
    }
    #[doc = "NETC_PINMUX_IPP_IND_ETH0_RX_CLK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn netc_pinmux_ipp_ind_eth0_rx_clk_select_input(
        self,
    ) -> crate::common::Reg<regs::NetcPinmuxIppIndEth0RxClkSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07d8usize) as _) }
    }
    #[doc = "NETC_PINMUX_IPP_IND_ETH0_RX_DV_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn netc_pinmux_ipp_ind_eth0_rx_dv_select_input(
        self,
    ) -> crate::common::Reg<regs::NetcPinmuxIppIndEth0RxDvSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07dcusize) as _) }
    }
    #[doc = "NETC_PINMUX_IPP_IND_ETH0_RX_ER_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn netc_pinmux_ipp_ind_eth0_rx_er_select_input(
        self,
    ) -> crate::common::Reg<regs::NetcPinmuxIppIndEth0RxErSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07e0usize) as _) }
    }
    #[doc = "NETC_PINMUX_IPP_IND_ETH0_RXD_SELECT_INPUT_0 DAISY Register"]
    #[inline(always)]
    pub const fn netc_pinmux_ipp_ind_eth0_rxd_select_input_0(
        self,
    ) -> crate::common::Reg<regs::NetcPinmuxIppIndEth0RxdSelectInput0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07e4usize) as _) }
    }
    #[doc = "NETC_PINMUX_IPP_IND_ETH0_RXD_SELECT_INPUT_1 DAISY Register"]
    #[inline(always)]
    pub const fn netc_pinmux_ipp_ind_eth0_rxd_select_input_1(
        self,
    ) -> crate::common::Reg<regs::NetcPinmuxIppIndEth0RxdSelectInput1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07e8usize) as _) }
    }
    #[doc = "NETC_PINMUX_IPP_IND_ETH0_RXD_SELECT_INPUT_2 DAISY Register"]
    #[inline(always)]
    pub const fn netc_pinmux_ipp_ind_eth0_rxd_select_input_2(
        self,
    ) -> crate::common::Reg<regs::NetcPinmuxIppIndEth0RxdSelectInput2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07ecusize) as _) }
    }
    #[doc = "NETC_PINMUX_IPP_IND_ETH0_RXD_SELECT_INPUT_3 DAISY Register"]
    #[inline(always)]
    pub const fn netc_pinmux_ipp_ind_eth0_rxd_select_input_3(
        self,
    ) -> crate::common::Reg<regs::NetcPinmuxIppIndEth0RxdSelectInput3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07f0usize) as _) }
    }
    #[doc = "NETC_PINMUX_IPP_IND_ETH0_TX_CLK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn netc_pinmux_ipp_ind_eth0_tx_clk_select_input(
        self,
    ) -> crate::common::Reg<regs::NetcPinmuxIppIndEth0TxClkSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07f4usize) as _) }
    }
    #[doc = "NETC_PINMUX_IPP_IND_ETH2_RX_CLK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn netc_pinmux_ipp_ind_eth2_rx_clk_select_input(
        self,
    ) -> crate::common::Reg<regs::NetcPinmuxIppIndEth2RxClkSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07f8usize) as _) }
    }
    #[doc = "NETC_PINMUX_IPP_IND_ETH2_RX_DV_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn netc_pinmux_ipp_ind_eth2_rx_dv_select_input(
        self,
    ) -> crate::common::Reg<regs::NetcPinmuxIppIndEth2RxDvSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07fcusize) as _) }
    }
    #[doc = "NETC_PINMUX_IPP_IND_ETH2_RX_ER_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn netc_pinmux_ipp_ind_eth2_rx_er_select_input(
        self,
    ) -> crate::common::Reg<regs::NetcPinmuxIppIndEth2RxErSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0800usize) as _) }
    }
    #[doc = "NETC_PINMUX_IPP_IND_ETH2_RXD_SELECT_INPUT_0 DAISY Register"]
    #[inline(always)]
    pub const fn netc_pinmux_ipp_ind_eth2_rxd_select_input_0(
        self,
    ) -> crate::common::Reg<regs::NetcPinmuxIppIndEth2RxdSelectInput0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0804usize) as _) }
    }
    #[doc = "NETC_PINMUX_IPP_IND_ETH2_RXD_SELECT_INPUT_1 DAISY Register"]
    #[inline(always)]
    pub const fn netc_pinmux_ipp_ind_eth2_rxd_select_input_1(
        self,
    ) -> crate::common::Reg<regs::NetcPinmuxIppIndEth2RxdSelectInput1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0808usize) as _) }
    }
    #[doc = "NETC_PINMUX_IPP_IND_ETH2_RXD_SELECT_INPUT_2 DAISY Register"]
    #[inline(always)]
    pub const fn netc_pinmux_ipp_ind_eth2_rxd_select_input_2(
        self,
    ) -> crate::common::Reg<regs::NetcPinmuxIppIndEth2RxdSelectInput2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x080cusize) as _) }
    }
    #[doc = "NETC_PINMUX_IPP_IND_ETH2_RXD_SELECT_INPUT_3 DAISY Register"]
    #[inline(always)]
    pub const fn netc_pinmux_ipp_ind_eth2_rxd_select_input_3(
        self,
    ) -> crate::common::Reg<regs::NetcPinmuxIppIndEth2RxdSelectInput3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0810usize) as _) }
    }
    #[doc = "NETC_PINMUX_IPP_IND_ETH2_TX_CLK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn netc_pinmux_ipp_ind_eth2_tx_clk_select_input(
        self,
    ) -> crate::common::Reg<regs::NetcPinmuxIppIndEth2TxClkSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0814usize) as _) }
    }
    #[doc = "NETC_PINMUX_IPP_IND_ETH3_RX_CLK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn netc_pinmux_ipp_ind_eth3_rx_clk_select_input(
        self,
    ) -> crate::common::Reg<regs::NetcPinmuxIppIndEth3RxClkSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0818usize) as _) }
    }
    #[doc = "NETC_PINMUX_IPP_IND_ETH3_RX_DV_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn netc_pinmux_ipp_ind_eth3_rx_dv_select_input(
        self,
    ) -> crate::common::Reg<regs::NetcPinmuxIppIndEth3RxDvSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x081cusize) as _) }
    }
    #[doc = "NETC_PINMUX_IPP_IND_ETH3_RX_ER_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn netc_pinmux_ipp_ind_eth3_rx_er_select_input(
        self,
    ) -> crate::common::Reg<regs::NetcPinmuxIppIndEth3RxErSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0820usize) as _) }
    }
    #[doc = "NETC_PINMUX_IPP_IND_ETH3_RXD_SELECT_INPUT_0 DAISY Register"]
    #[inline(always)]
    pub const fn netc_pinmux_ipp_ind_eth3_rxd_select_input_0(
        self,
    ) -> crate::common::Reg<regs::NetcPinmuxIppIndEth3RxdSelectInput0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0824usize) as _) }
    }
    #[doc = "NETC_PINMUX_IPP_IND_ETH3_RXD_SELECT_INPUT_1 DAISY Register"]
    #[inline(always)]
    pub const fn netc_pinmux_ipp_ind_eth3_rxd_select_input_1(
        self,
    ) -> crate::common::Reg<regs::NetcPinmuxIppIndEth3RxdSelectInput1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0828usize) as _) }
    }
    #[doc = "NETC_PINMUX_IPP_IND_ETH3_RXD_SELECT_INPUT_2 DAISY Register"]
    #[inline(always)]
    pub const fn netc_pinmux_ipp_ind_eth3_rxd_select_input_2(
        self,
    ) -> crate::common::Reg<regs::NetcPinmuxIppIndEth3RxdSelectInput2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x082cusize) as _) }
    }
    #[doc = "NETC_PINMUX_IPP_IND_ETH3_RXD_SELECT_INPUT_3 DAISY Register"]
    #[inline(always)]
    pub const fn netc_pinmux_ipp_ind_eth3_rxd_select_input_3(
        self,
    ) -> crate::common::Reg<regs::NetcPinmuxIppIndEth3RxdSelectInput3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0830usize) as _) }
    }
    #[doc = "NETC_PINMUX_IPP_IND_ETH3_TX_CLK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn netc_pinmux_ipp_ind_eth3_tx_clk_select_input(
        self,
    ) -> crate::common::Reg<regs::NetcPinmuxIppIndEth3TxClkSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0834usize) as _) }
    }
    #[doc = "NETC_PINMUX_IPP_IND_ETH4_RX_CLK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn netc_pinmux_ipp_ind_eth4_rx_clk_select_input(
        self,
    ) -> crate::common::Reg<regs::NetcPinmuxIppIndEth4RxClkSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0838usize) as _) }
    }
    #[doc = "NETC_PINMUX_IPP_IND_ETH4_RX_DV_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn netc_pinmux_ipp_ind_eth4_rx_dv_select_input(
        self,
    ) -> crate::common::Reg<regs::NetcPinmuxIppIndEth4RxDvSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x083cusize) as _) }
    }
    #[doc = "NETC_PINMUX_IPP_IND_ETH4_RX_ER_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn netc_pinmux_ipp_ind_eth4_rx_er_select_input(
        self,
    ) -> crate::common::Reg<regs::NetcPinmuxIppIndEth4RxErSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0840usize) as _) }
    }
    #[doc = "NETC_PINMUX_IPP_IND_ETH4_RXD_SELECT_INPUT_0 DAISY Register"]
    #[inline(always)]
    pub const fn netc_pinmux_ipp_ind_eth4_rxd_select_input_0(
        self,
    ) -> crate::common::Reg<regs::NetcPinmuxIppIndEth4RxdSelectInput0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0844usize) as _) }
    }
    #[doc = "NETC_PINMUX_IPP_IND_ETH4_RXD_SELECT_INPUT_1 DAISY Register"]
    #[inline(always)]
    pub const fn netc_pinmux_ipp_ind_eth4_rxd_select_input_1(
        self,
    ) -> crate::common::Reg<regs::NetcPinmuxIppIndEth4RxdSelectInput1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0848usize) as _) }
    }
    #[doc = "NETC_PINMUX_IPP_IND_ETH4_RXD_SELECT_INPUT_2 DAISY Register"]
    #[inline(always)]
    pub const fn netc_pinmux_ipp_ind_eth4_rxd_select_input_2(
        self,
    ) -> crate::common::Reg<regs::NetcPinmuxIppIndEth4RxdSelectInput2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x084cusize) as _) }
    }
    #[doc = "NETC_PINMUX_IPP_IND_ETH4_RXD_SELECT_INPUT_3 DAISY Register"]
    #[inline(always)]
    pub const fn netc_pinmux_ipp_ind_eth4_rxd_select_input_3(
        self,
    ) -> crate::common::Reg<regs::NetcPinmuxIppIndEth4RxdSelectInput3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0850usize) as _) }
    }
    #[doc = "NETC_PINMUX_IPP_IND_ETH4_TX_CLK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn netc_pinmux_ipp_ind_eth4_tx_clk_select_input(
        self,
    ) -> crate::common::Reg<regs::NetcPinmuxIppIndEth4TxClkSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0854usize) as _) }
    }
    #[doc = "QTIMER1_TMR0_INPUT_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn qtimer1_tmr0_input_select_input(
        self,
    ) -> crate::common::Reg<regs::Qtimer1Tmr0InputSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0858usize) as _) }
    }
    #[doc = "QTIMER1_TMR1_INPUT_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn qtimer1_tmr1_input_select_input(
        self,
    ) -> crate::common::Reg<regs::Qtimer1Tmr1InputSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x085cusize) as _) }
    }
    #[doc = "QTIMER1_TMR2_INPUT_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn qtimer1_tmr2_input_select_input(
        self,
    ) -> crate::common::Reg<regs::Qtimer1Tmr2InputSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0860usize) as _) }
    }
    #[doc = "QTIMER2_TMR0_INPUT_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn qtimer2_tmr0_input_select_input(
        self,
    ) -> crate::common::Reg<regs::Qtimer2Tmr0InputSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0864usize) as _) }
    }
    #[doc = "QTIMER2_TMR1_INPUT_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn qtimer2_tmr1_input_select_input(
        self,
    ) -> crate::common::Reg<regs::Qtimer2Tmr1InputSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0868usize) as _) }
    }
    #[doc = "QTIMER2_TMR2_INPUT_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn qtimer2_tmr2_input_select_input(
        self,
    ) -> crate::common::Reg<regs::Qtimer2Tmr2InputSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x086cusize) as _) }
    }
    #[doc = "QTIMER3_TMR0_INPUT_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn qtimer3_tmr0_input_select_input(
        self,
    ) -> crate::common::Reg<regs::Qtimer3Tmr0InputSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0870usize) as _) }
    }
    #[doc = "QTIMER3_TMR1_INPUT_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn qtimer3_tmr1_input_select_input(
        self,
    ) -> crate::common::Reg<regs::Qtimer3Tmr1InputSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0874usize) as _) }
    }
    #[doc = "QTIMER3_TMR2_INPUT_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn qtimer3_tmr2_input_select_input(
        self,
    ) -> crate::common::Reg<regs::Qtimer3Tmr2InputSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0878usize) as _) }
    }
    #[doc = "QTIMER4_TMR0_INPUT_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn qtimer4_tmr0_input_select_input(
        self,
    ) -> crate::common::Reg<regs::Qtimer4Tmr0InputSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x087cusize) as _) }
    }
    #[doc = "QTIMER4_TMR1_INPUT_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn qtimer4_tmr1_input_select_input(
        self,
    ) -> crate::common::Reg<regs::Qtimer4Tmr1InputSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0880usize) as _) }
    }
    #[doc = "QTIMER4_TMR2_INPUT_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn qtimer4_tmr2_input_select_input(
        self,
    ) -> crate::common::Reg<regs::Qtimer4Tmr2InputSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0884usize) as _) }
    }
    #[doc = "QTIMER5_TMR0_INPUT_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn qtimer5_tmr0_input_select_input(
        self,
    ) -> crate::common::Reg<regs::Qtimer5Tmr0InputSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0888usize) as _) }
    }
    #[doc = "QTIMER5_TMR1_INPUT_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn qtimer5_tmr1_input_select_input(
        self,
    ) -> crate::common::Reg<regs::Qtimer5Tmr1InputSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x088cusize) as _) }
    }
    #[doc = "QTIMER5_TMR2_INPUT_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn qtimer5_tmr2_input_select_input(
        self,
    ) -> crate::common::Reg<regs::Qtimer5Tmr2InputSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0890usize) as _) }
    }
    #[doc = "QTIMER6_TMR0_INPUT_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn qtimer6_tmr0_input_select_input(
        self,
    ) -> crate::common::Reg<regs::Qtimer6Tmr0InputSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0894usize) as _) }
    }
    #[doc = "QTIMER6_TMR1_INPUT_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn qtimer6_tmr1_input_select_input(
        self,
    ) -> crate::common::Reg<regs::Qtimer6Tmr1InputSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0898usize) as _) }
    }
    #[doc = "QTIMER6_TMR2_INPUT_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn qtimer6_tmr2_input_select_input(
        self,
    ) -> crate::common::Reg<regs::Qtimer6Tmr2InputSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x089cusize) as _) }
    }
    #[doc = "QTIMER7_TMR0_INPUT_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn qtimer7_tmr0_input_select_input(
        self,
    ) -> crate::common::Reg<regs::Qtimer7Tmr0InputSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08a0usize) as _) }
    }
    #[doc = "QTIMER7_TMR1_INPUT_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn qtimer7_tmr1_input_select_input(
        self,
    ) -> crate::common::Reg<regs::Qtimer7Tmr1InputSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08a4usize) as _) }
    }
    #[doc = "QTIMER8_TMR0_INPUT_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn qtimer8_tmr0_input_select_input(
        self,
    ) -> crate::common::Reg<regs::Qtimer8Tmr0InputSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08a8usize) as _) }
    }
    #[doc = "QTIMER8_TMR1_INPUT_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn qtimer8_tmr1_input_select_input(
        self,
    ) -> crate::common::Reg<regs::Qtimer8Tmr1InputSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08acusize) as _) }
    }
    #[doc = "SAI4_IPG_CLK_SAI_MCLK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai4_ipg_clk_sai_mclk_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai4IpgClkSaiMclkSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08b0usize) as _) }
    }
    #[doc = "SAI4_IPP_IND_SAI_RXBCLK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai4_ipp_ind_sai_rxbclk_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai4IppIndSaiRxbclkSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08b4usize) as _) }
    }
    #[doc = "SAI4_IPP_IND_SAI_RXDATA_SELECT_INPUT_0 DAISY Register"]
    #[inline(always)]
    pub const fn sai4_ipp_ind_sai_rxdata_select_input_0(
        self,
    ) -> crate::common::Reg<regs::Sai4IppIndSaiRxdataSelectInput0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08b8usize) as _) }
    }
    #[doc = "SAI4_IPP_IND_SAI_RXDATA_SELECT_INPUT_1 DAISY Register"]
    #[inline(always)]
    pub const fn sai4_ipp_ind_sai_rxdata_select_input_1(
        self,
    ) -> crate::common::Reg<regs::Sai4IppIndSaiRxdataSelectInput1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08bcusize) as _) }
    }
    #[doc = "SAI4_IPP_IND_SAI_RXSYNC_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai4_ipp_ind_sai_rxsync_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai4IppIndSaiRxsyncSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08c0usize) as _) }
    }
    #[doc = "SAI4_IPP_IND_SAI_TXBCLK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai4_ipp_ind_sai_txbclk_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai4IppIndSaiTxbclkSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08c4usize) as _) }
    }
    #[doc = "SAI4_IPP_IND_SAI_TXSYNC_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn sai4_ipp_ind_sai_txsync_select_input(
        self,
    ) -> crate::common::Reg<regs::Sai4IppIndSaiTxsyncSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08c8usize) as _) }
    }
    #[doc = "SINC1_IPP_IND_EMBIT_SELECT_INPUT_0 DAISY Register"]
    #[inline(always)]
    pub const fn sinc1_ipp_ind_embit_select_input_0(
        self,
    ) -> crate::common::Reg<regs::Sinc1IppIndEmbitSelectInput0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08d4usize) as _) }
    }
    #[doc = "SINC1_IPP_IND_EMBIT_SELECT_INPUT_1 DAISY Register"]
    #[inline(always)]
    pub const fn sinc1_ipp_ind_embit_select_input_1(
        self,
    ) -> crate::common::Reg<regs::Sinc1IppIndEmbitSelectInput1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08d8usize) as _) }
    }
    #[doc = "SINC1_IPP_IND_EMBIT_SELECT_INPUT_2 DAISY Register"]
    #[inline(always)]
    pub const fn sinc1_ipp_ind_embit_select_input_2(
        self,
    ) -> crate::common::Reg<regs::Sinc1IppIndEmbitSelectInput2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08dcusize) as _) }
    }
    #[doc = "SINC1_IPP_IND_EMBIT_SELECT_INPUT_3 DAISY Register"]
    #[inline(always)]
    pub const fn sinc1_ipp_ind_embit_select_input_3(
        self,
    ) -> crate::common::Reg<regs::Sinc1IppIndEmbitSelectInput3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08e0usize) as _) }
    }
    #[doc = "SINC1_IPP_IND_EMCLK_SELECT_INPUT_0 DAISY Register"]
    #[inline(always)]
    pub const fn sinc1_ipp_ind_emclk_select_input_0(
        self,
    ) -> crate::common::Reg<regs::Sinc1IppIndEmclkSelectInput0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08e4usize) as _) }
    }
    #[doc = "SINC1_IPP_IND_EMCLK_SELECT_INPUT_1 DAISY Register"]
    #[inline(always)]
    pub const fn sinc1_ipp_ind_emclk_select_input_1(
        self,
    ) -> crate::common::Reg<regs::Sinc1IppIndEmclkSelectInput1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08e8usize) as _) }
    }
    #[doc = "SINC1_IPP_IND_EMCLK_SELECT_INPUT_2 DAISY Register"]
    #[inline(always)]
    pub const fn sinc1_ipp_ind_emclk_select_input_2(
        self,
    ) -> crate::common::Reg<regs::Sinc1IppIndEmclkSelectInput2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08ecusize) as _) }
    }
    #[doc = "SINC1_IPP_IND_EMCLK_SELECT_INPUT_3 DAISY Register"]
    #[inline(always)]
    pub const fn sinc1_ipp_ind_emclk_select_input_3(
        self,
    ) -> crate::common::Reg<regs::Sinc1IppIndEmclkSelectInput3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08f0usize) as _) }
    }
    #[doc = "SINC2_IPP_IND_EMBIT_SELECT_INPUT_2 DAISY Register"]
    #[inline(always)]
    pub const fn sinc2_ipp_ind_embit_select_input_2(
        self,
    ) -> crate::common::Reg<regs::Sinc2IppIndEmbitSelectInput2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08f4usize) as _) }
    }
    #[doc = "SINC2_IPP_IND_EMBIT_SELECT_INPUT_3 DAISY Register"]
    #[inline(always)]
    pub const fn sinc2_ipp_ind_embit_select_input_3(
        self,
    ) -> crate::common::Reg<regs::Sinc2IppIndEmbitSelectInput3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08f8usize) as _) }
    }
    #[doc = "SINC2_IPP_IND_EMCLK_SELECT_INPUT_0 DAISY Register"]
    #[inline(always)]
    pub const fn sinc2_ipp_ind_emclk_select_input_0(
        self,
    ) -> crate::common::Reg<regs::Sinc2IppIndEmclkSelectInput0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08fcusize) as _) }
    }
    #[doc = "SINC2_IPP_IND_EMCLK_SELECT_INPUT_2 DAISY Register"]
    #[inline(always)]
    pub const fn sinc2_ipp_ind_emclk_select_input_2(
        self,
    ) -> crate::common::Reg<regs::Sinc2IppIndEmclkSelectInput2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0900usize) as _) }
    }
    #[doc = "SINC2_IPP_IND_EMCLK_SELECT_INPUT_3 DAISY Register"]
    #[inline(always)]
    pub const fn sinc2_ipp_ind_emclk_select_input_3(
        self,
    ) -> crate::common::Reg<regs::Sinc2IppIndEmclkSelectInput3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0904usize) as _) }
    }
    #[doc = "SPDIF_SPDIF_IN1_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn spdif_spdif_in1_select_input(
        self,
    ) -> crate::common::Reg<regs::SpdifSpdifIn1SelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0908usize) as _) }
    }
    #[doc = "USB_IPP_IND_OTG2_OC_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn usb_ipp_ind_otg2_oc_select_input(
        self,
    ) -> crate::common::Reg<regs::UsbIppIndOtg2OcSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0914usize) as _) }
    }
    #[doc = "USB_IPP_IND_OTG_OC_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn usb_ipp_ind_otg_oc_select_input(
        self,
    ) -> crate::common::Reg<regs::UsbIppIndOtgOcSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0918usize) as _) }
    }
    #[doc = "USBPHY1_USB_ID_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn usbphy1_usb_id_select_input(
        self,
    ) -> crate::common::Reg<regs::Usbphy1UsbIdSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x091cusize) as _) }
    }
    #[doc = "USBPHY2_USB_ID_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn usbphy2_usb_id_select_input(
        self,
    ) -> crate::common::Reg<regs::Usbphy2UsbIdSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0920usize) as _) }
    }
    #[doc = "USDHC1_IPP_CARD_DET_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn usdhc1_ipp_card_det_select_input(
        self,
    ) -> crate::common::Reg<regs::Usdhc1IppCardDetSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0924usize) as _) }
    }
    #[doc = "USDHC1_IPP_WP_ON_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn usdhc1_ipp_wp_on_select_input(
        self,
    ) -> crate::common::Reg<regs::Usdhc1IppWpOnSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0928usize) as _) }
    }
    #[doc = "USDHC2_IPP_CARD_DET_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn usdhc2_ipp_card_det_select_input(
        self,
    ) -> crate::common::Reg<regs::Usdhc2IppCardDetSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x092cusize) as _) }
    }
    #[doc = "USDHC2_IPP_WP_ON_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn usdhc2_ipp_wp_on_select_input(
        self,
    ) -> crate::common::Reg<regs::Usdhc2IppWpOnSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0930usize) as _) }
    }
    #[doc = "XBAR1_XBAR_IN_SELECT_INPUT_14 DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_xbar_in_select_input_14(
        self,
    ) -> crate::common::Reg<regs::Xbar1XbarInSelectInput14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0934usize) as _) }
    }
    #[doc = "XBAR1_XBAR_IN_SELECT_INPUT_15 DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_xbar_in_select_input_15(
        self,
    ) -> crate::common::Reg<regs::Xbar1XbarInSelectInput15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0938usize) as _) }
    }
    #[doc = "XBAR1_XBAR_IN_SELECT_INPUT_17 DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_xbar_in_select_input_17(
        self,
    ) -> crate::common::Reg<regs::Xbar1XbarInSelectInput17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x093cusize) as _) }
    }
    #[doc = "XBAR1_XBAR_IN_SELECT_INPUT_18 DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_xbar_in_select_input_18(
        self,
    ) -> crate::common::Reg<regs::Xbar1XbarInSelectInput18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0940usize) as _) }
    }
    #[doc = "XBAR1_XBAR_IN_SELECT_INPUT_19 DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_xbar_in_select_input_19(
        self,
    ) -> crate::common::Reg<regs::Xbar1XbarInSelectInput19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0944usize) as _) }
    }
    #[doc = "XBAR1_XBAR_IN_SELECT_INPUT_20 DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_xbar_in_select_input_20(
        self,
    ) -> crate::common::Reg<regs::Xbar1XbarInSelectInput20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0948usize) as _) }
    }
    #[doc = "XBAR1_XBAR_IN_SELECT_INPUT_21 DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_xbar_in_select_input_21(
        self,
    ) -> crate::common::Reg<regs::Xbar1XbarInSelectInput21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x094cusize) as _) }
    }
    #[doc = "XBAR1_XBAR_IN_SELECT_INPUT_22 DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_xbar_in_select_input_22(
        self,
    ) -> crate::common::Reg<regs::Xbar1XbarInSelectInput22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0950usize) as _) }
    }
    #[doc = "XBAR1_XBAR_IN_SELECT_INPUT_23 DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_xbar_in_select_input_23(
        self,
    ) -> crate::common::Reg<regs::Xbar1XbarInSelectInput23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0954usize) as _) }
    }
    #[doc = "XBAR1_XBAR_IN_SELECT_INPUT_24 DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_xbar_in_select_input_24(
        self,
    ) -> crate::common::Reg<regs::Xbar1XbarInSelectInput24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0958usize) as _) }
    }
    #[doc = "XBAR1_XBAR_IN_SELECT_INPUT_25 DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_xbar_in_select_input_25(
        self,
    ) -> crate::common::Reg<regs::Xbar1XbarInSelectInput25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x095cusize) as _) }
    }
    #[doc = "XBAR1_XBAR_IN_SELECT_INPUT_26 DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_xbar_in_select_input_26(
        self,
    ) -> crate::common::Reg<regs::Xbar1XbarInSelectInput26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0960usize) as _) }
    }
    #[doc = "XBAR1_XBAR_IN_SELECT_INPUT_27 DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_xbar_in_select_input_27(
        self,
    ) -> crate::common::Reg<regs::Xbar1XbarInSelectInput27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0964usize) as _) }
    }
    #[doc = "XBAR1_XBAR_IN_SELECT_INPUT_28 DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_xbar_in_select_input_28(
        self,
    ) -> crate::common::Reg<regs::Xbar1XbarInSelectInput28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0968usize) as _) }
    }
    #[doc = "XBAR1_XBAR_IN_SELECT_INPUT_29 DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_xbar_in_select_input_29(
        self,
    ) -> crate::common::Reg<regs::Xbar1XbarInSelectInput29, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x096cusize) as _) }
    }
    #[doc = "XBAR1_XBAR_IN_SELECT_INPUT_30 DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_xbar_in_select_input_30(
        self,
    ) -> crate::common::Reg<regs::Xbar1XbarInSelectInput30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0970usize) as _) }
    }
    #[doc = "XBAR1_XBAR_IN_SELECT_INPUT_31 DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_xbar_in_select_input_31(
        self,
    ) -> crate::common::Reg<regs::Xbar1XbarInSelectInput31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0974usize) as _) }
    }
    #[doc = "XBAR1_XBAR_IN_SELECT_INPUT_32 DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_xbar_in_select_input_32(
        self,
    ) -> crate::common::Reg<regs::Xbar1XbarInSelectInput32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0978usize) as _) }
    }
    #[doc = "XBAR1_XBAR_IN_SELECT_INPUT_33 DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_xbar_in_select_input_33(
        self,
    ) -> crate::common::Reg<regs::Xbar1XbarInSelectInput33, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x097cusize) as _) }
    }
    #[doc = "XBAR1_XBAR_IN_SELECT_INPUT_34 DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_xbar_in_select_input_34(
        self,
    ) -> crate::common::Reg<regs::Xbar1XbarInSelectInput34, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0980usize) as _) }
    }
    #[doc = "XBAR1_XBAR_IN_SELECT_INPUT_35 DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_xbar_in_select_input_35(
        self,
    ) -> crate::common::Reg<regs::Xbar1XbarInSelectInput35, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0984usize) as _) }
    }
    #[doc = "XBAR1_XBAR_IN_SELECT_INPUT_36 DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_xbar_in_select_input_36(
        self,
    ) -> crate::common::Reg<regs::Xbar1XbarInSelectInput36, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0988usize) as _) }
    }
    #[doc = "XBAR1_XBAR_IN_SELECT_INPUT_37 DAISY Register"]
    #[inline(always)]
    pub const fn xbar1_xbar_in_select_input_37(
        self,
    ) -> crate::common::Reg<regs::Xbar1XbarInSelectInput37, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x098cusize) as _) }
    }
    #[doc = "XSPI_SLV_IPP_IND_CS_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn xspi_slv_ipp_ind_cs_select_input(
        self,
    ) -> crate::common::Reg<regs::XspiSlvIppIndCsSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0a00usize) as _) }
    }
    #[doc = "XSPI_SLV_IPP_IND_DQS_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn xspi_slv_ipp_ind_dqs_select_input(
        self,
    ) -> crate::common::Reg<regs::XspiSlvIppIndDqsSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0a04usize) as _) }
    }
    #[doc = "XSPI_SLV_IPP_IND_IO_SELECT_INPUT_0 DAISY Register"]
    #[inline(always)]
    pub const fn xspi_slv_ipp_ind_io_select_input_0(
        self,
    ) -> crate::common::Reg<regs::XspiSlvIppIndIoSelectInput0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0a08usize) as _) }
    }
    #[doc = "XSPI_SLV_IPP_IND_IO_SELECT_INPUT_1 DAISY Register"]
    #[inline(always)]
    pub const fn xspi_slv_ipp_ind_io_select_input_1(
        self,
    ) -> crate::common::Reg<regs::XspiSlvIppIndIoSelectInput1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0a0cusize) as _) }
    }
    #[doc = "XSPI_SLV_IPP_IND_IO_SELECT_INPUT_2 DAISY Register"]
    #[inline(always)]
    pub const fn xspi_slv_ipp_ind_io_select_input_2(
        self,
    ) -> crate::common::Reg<regs::XspiSlvIppIndIoSelectInput2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0a10usize) as _) }
    }
    #[doc = "XSPI_SLV_IPP_IND_IO_SELECT_INPUT_3 DAISY Register"]
    #[inline(always)]
    pub const fn xspi_slv_ipp_ind_io_select_input_3(
        self,
    ) -> crate::common::Reg<regs::XspiSlvIppIndIoSelectInput3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0a14usize) as _) }
    }
    #[doc = "XSPI_SLV_IPP_IND_IO_SELECT_INPUT_4 DAISY Register"]
    #[inline(always)]
    pub const fn xspi_slv_ipp_ind_io_select_input_4(
        self,
    ) -> crate::common::Reg<regs::XspiSlvIppIndIoSelectInput4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0a18usize) as _) }
    }
    #[doc = "XSPI_SLV_IPP_IND_IO_SELECT_INPUT_5 DAISY Register"]
    #[inline(always)]
    pub const fn xspi_slv_ipp_ind_io_select_input_5(
        self,
    ) -> crate::common::Reg<regs::XspiSlvIppIndIoSelectInput5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0a1cusize) as _) }
    }
    #[doc = "XSPI_SLV_IPP_IND_IO_SELECT_INPUT_6 DAISY Register"]
    #[inline(always)]
    pub const fn xspi_slv_ipp_ind_io_select_input_6(
        self,
    ) -> crate::common::Reg<regs::XspiSlvIppIndIoSelectInput6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0a20usize) as _) }
    }
    #[doc = "XSPI_SLV_IPP_IND_IO_SELECT_INPUT_7 DAISY Register"]
    #[inline(always)]
    pub const fn xspi_slv_ipp_ind_io_select_input_7(
        self,
    ) -> crate::common::Reg<regs::XspiSlvIppIndIoSelectInput7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0a24usize) as _) }
    }
    #[doc = "XSPI_SLV_IPP_IND_SCK_SELECT_INPUT DAISY Register"]
    #[inline(always)]
    pub const fn xspi_slv_ipp_ind_sck_select_input(
        self,
    ) -> crate::common::Reg<regs::XspiSlvIppIndSckSelectInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0a28usize) as _) }
    }
}
pub mod regs;
pub mod vals;
