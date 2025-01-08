#[doc = "SRC MIX SLICE"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SrcMixSlice {
    ptr: *mut u8,
}
unsafe impl Send for SrcMixSlice {}
unsafe impl Sync for SrcMixSlice {}
impl SrcMixSlice {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Authentication Control"]
    #[inline(always)]
    pub const fn authen_ctrl(self) -> crate::common::Reg<regs::AuthenCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "SLICE Software Control"]
    #[inline(always)]
    pub const fn slice_sw_ctrl(self) -> crate::common::Reg<regs::SliceSwCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Function Status"]
    #[inline(always)]
    pub const fn func_stat(self) -> crate::common::Reg<regs::FuncStat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "UPI Status 0"]
    #[inline(always)]
    pub const fn upi_stat_0(self) -> crate::common::Reg<regs::UpiStat0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "UPI Status 1"]
    #[inline(always)]
    pub const fn upi_stat_1(self) -> crate::common::Reg<regs::UpiStat1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Low Power Mode Setting 0"]
    #[inline(always)]
    pub const fn lpm_setting_0(self) -> crate::common::Reg<regs::LpmSetting0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Low Power Mode Setting 1"]
    #[inline(always)]
    pub const fn lpm_setting_1(self) -> crate::common::Reg<regs::LpmSetting1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "Low Power Mode Setting 2"]
    #[inline(always)]
    pub const fn lpm_setting_2(self) -> crate::common::Reg<regs::LpmSetting2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "Edgelock Handshake Control"]
    #[inline(always)]
    pub const fn edgelock_hdsk_ctrl(
        self,
    ) -> crate::common::Reg<regs::EdgelockHdskCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "Edgelock Handshake Status"]
    #[inline(always)]
    pub const fn edgelock_hdsk_stat(
        self,
    ) -> crate::common::Reg<regs::EdgelockHdskStat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "Edgelock Handshake Counter Config"]
    #[inline(always)]
    pub const fn edgelock_hdsk_cnt_cfg(
        self,
    ) -> crate::common::Reg<regs::EdgelockHdskCntCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "Edgelock Handshake Counter Status"]
    #[inline(always)]
    pub const fn edgelock_hdsk_cnt_stat(
        self,
    ) -> crate::common::Reg<regs::EdgelockHdskCntStat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "ISO Delay Pre control"]
    #[inline(always)]
    pub const fn iso_dly_pre(self) -> crate::common::Reg<regs::IsoDlyPre, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "PSW Delay Pre for HF"]
    #[inline(always)]
    pub const fn psw_dly_pre_hf(self) -> crate::common::Reg<regs::PswDlyPreHf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "PSW Delay Pre for LF"]
    #[inline(always)]
    pub const fn psw_dly_pre_lf(self) -> crate::common::Reg<regs::PswDlyPreLf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "PSW Control"]
    #[inline(always)]
    pub const fn psw_ctrl(self) -> crate::common::Reg<regs::PswCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "PSW Status"]
    #[inline(always)]
    pub const fn psw_stat(self) -> crate::common::Reg<regs::PswStat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "PSW Counter Config for HF"]
    #[inline(always)]
    pub const fn psw_cnt_cfg_hf(self) -> crate::common::Reg<regs::PswCntCfgHf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "PSW Counter Status for HF"]
    #[inline(always)]
    pub const fn psw_cnt_stat_hf(self) -> crate::common::Reg<regs::PswCntStatHf, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "PSW Counter Config for LF"]
    #[inline(always)]
    pub const fn psw_cnt_cfg_lf(self) -> crate::common::Reg<regs::PswCntCfgLf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "PSW Counter Status for LF"]
    #[inline(always)]
    pub const fn psw_cnt_stat_lf(self) -> crate::common::Reg<regs::PswCntStatLf, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "Memory Low Power Level Trigger Control"]
    #[inline(always)]
    pub const fn mlpl_trig_ctrl(self) -> crate::common::Reg<regs::MlplTrigCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "Memory Low Power Level Config"]
    #[inline(always)]
    pub const fn mlpl_cfg(self) -> crate::common::Reg<regs::MlplCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "Memory Low Power Level Status"]
    #[inline(always)]
    pub const fn mlpl_stat(self) -> crate::common::Reg<regs::MlplStat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
}
pub mod regs;
pub mod vals;
