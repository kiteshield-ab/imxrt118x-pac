#[doc = "FlexSPI_FLR"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexspiSlv {
    ptr: *mut u8,
}
unsafe impl Send for FlexspiSlv {}
unsafe impl Sync for FlexspiSlv {}
impl FlexspiSlv {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Module Control"]
    #[inline(always)]
    pub const fn module_control(
        self,
    ) -> crate::common::Reg<regs::ModuleControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Read Command Control"]
    #[inline(always)]
    pub const fn read_command_control(
        self,
    ) -> crate::common::Reg<regs::ReadCommandControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Read Register Command Setting"]
    #[inline(always)]
    pub const fn read_register_command0(
        self,
    ) -> crate::common::Reg<regs::ReadRegisterCommand0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Read Command 1 setting"]
    #[inline(always)]
    pub const fn read_command1(self) -> crate::common::Reg<regs::ReadCommand1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Read Command 2 setting"]
    #[inline(always)]
    pub const fn read_command2(self) -> crate::common::Reg<regs::ReadCommand2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Write Command Control"]
    #[inline(always)]
    pub const fn write_command_control(
        self,
    ) -> crate::common::Reg<regs::WriteCommandControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Write Register Command 0 Setting"]
    #[inline(always)]
    pub const fn write_register_command0(
        self,
    ) -> crate::common::Reg<regs::WriteRegisterCommand0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Write Command 1 Setting"]
    #[inline(always)]
    pub const fn write_command1(
        self,
    ) -> crate::common::Reg<regs::WriteCommand1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Write Command 2 Setting"]
    #[inline(always)]
    pub const fn write_command2(
        self,
    ) -> crate::common::Reg<regs::WriteCommand2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Read Write Command Address Base"]
    #[inline(always)]
    pub const fn rw_command_base(
        self,
    ) -> crate::common::Reg<regs::RwCommandBase, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Command Suit 1 Range"]
    #[inline(always)]
    pub const fn cmd1_range(self) -> crate::common::Reg<regs::Cmd1Range, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Command Suit 2 Range"]
    #[inline(always)]
    pub const fn cmd2_range(self) -> crate::common::Reg<regs::Cmd2Range, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Module Status"]
    #[inline(always)]
    pub const fn module_status(self) -> crate::common::Reg<regs::ModuleStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "SPI FLR interrupt"]
    #[inline(always)]
    pub const fn module_int(self) -> crate::common::Reg<regs::ModuleInt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "SPI FLR Interrupt Enable"]
    #[inline(always)]
    pub const fn module_inten(self) -> crate::common::Reg<regs::ModuleInten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "SPI Mailbox control"]
    #[inline(always)]
    pub const fn spi_mail_ctrl(self) -> crate::common::Reg<regs::SpiMailCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "SPI Mail Interrupt"]
    #[inline(always)]
    pub const fn spimail(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
        assert!(n < 9usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize + n * 4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
