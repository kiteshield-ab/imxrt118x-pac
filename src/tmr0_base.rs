#[doc = "Timer"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmr0Base {
    ptr: *mut u8,
}
unsafe impl Send for Tmr0Base {}
unsafe impl Sync for Tmr0Base {}
impl Tmr0Base {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Module ID Register"]
    #[inline(always)]
    pub const fn tmr_id(self) -> crate::common::Reg<regs::TmrId, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Timer Capability Register"]
    #[inline(always)]
    pub const fn tmr_capr(self) -> crate::common::Reg<regs::TmrCapr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Timer free running time low register"]
    #[inline(always)]
    pub const fn tmr_frt_l(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Timer free running time high register"]
    #[inline(always)]
    pub const fn tmr_frt_h(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Timer synchronous time low register"]
    #[inline(always)]
    pub const fn tmr_srt_l(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Timer synchronous time high register."]
    #[inline(always)]
    pub const fn tmr_srt_h(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Default ns timer counter low register"]
    #[inline(always)]
    pub const fn tmr_def_cnt_l(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Default ns timer counter high register"]
    #[inline(always)]
    pub const fn tmr_def_cnt_h(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "Timer Control Register"]
    #[inline(always)]
    pub const fn tmr_ctrl(self) -> crate::common::Reg<regs::TmrCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "Timer Event Register"]
    #[inline(always)]
    pub const fn tmr_tevent(self) -> crate::common::Reg<regs::TmrTevent, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "Timer event mask register"]
    #[inline(always)]
    pub const fn tmr_temask(self) -> crate::common::Reg<regs::TmrTemask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "Timer status register"]
    #[inline(always)]
    pub const fn tmr_stat(self) -> crate::common::Reg<regs::TmrStat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "Timer counter low register"]
    #[inline(always)]
    pub const fn tmr_cnt_l(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[doc = "Timer counter high register"]
    #[inline(always)]
    pub const fn tmr_cnt_h(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "Timer addend register"]
    #[inline(always)]
    pub const fn tmr_add(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "Timer accumulator register"]
    #[inline(always)]
    pub const fn tmr_acc(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "Timer prescale register"]
    #[inline(always)]
    pub const fn tmr_prsc(self) -> crate::common::Reg<regs::TmrPrsc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "Extended timer control register"]
    #[inline(always)]
    pub const fn tmr_ectrl(self) -> crate::common::Reg<regs::TmrEctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[doc = "Timer offset low register"]
    #[inline(always)]
    pub const fn tmroff_l(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[doc = "Timer offset high register"]
    #[inline(always)]
    pub const fn tmroff_h(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
    }
    #[doc = "Alarm 1 time comparator low register"]
    #[inline(always)]
    pub const fn tmr_alarm1_l(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb8usize) as _) }
    }
    #[doc = "Alarm 1 time comparator high register"]
    #[inline(always)]
    pub const fn tmr_alarm1_h(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbcusize) as _) }
    }
    #[doc = "Alarm 2 time comparator low register"]
    #[inline(always)]
    pub const fn tmr_alarm2_l(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[doc = "Alarm 2 time comparator high register"]
    #[inline(always)]
    pub const fn tmr_alarm2_h(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[doc = "Timer Alarm Control Register"]
    #[inline(always)]
    pub const fn tmr_alarm_ctrl(self) -> crate::common::Reg<regs::TmrAlarmCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xccusize) as _) }
    }
    #[doc = "Timer i fixed interval period register"]
    #[inline(always)]
    pub const fn tmr_fiper(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize + n * 4usize) as _) }
    }
    #[doc = "Timer FIPER Control Register"]
    #[inline(always)]
    pub const fn tmr_fiper_ctrl(self) -> crate::common::Reg<regs::TmrFiperCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdcusize) as _) }
    }
    #[doc = "External trigger stamp register"]
    #[inline(always)]
    pub const fn tmr_etts1_l(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe0usize) as _) }
    }
    #[doc = "External trigger stamp register"]
    #[inline(always)]
    pub const fn tmr_etts1_h(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe4usize) as _) }
    }
    #[doc = "External trigger stamp register"]
    #[inline(always)]
    pub const fn tmr_etts2_l(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe8usize) as _) }
    }
    #[doc = "External trigger stamp register"]
    #[inline(always)]
    pub const fn tmr_etts2_h(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xecusize) as _) }
    }
    #[doc = "Timer current time low register"]
    #[inline(always)]
    pub const fn tmr_cur_time_l(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
    }
    #[doc = "Timer current time high register"]
    #[inline(always)]
    pub const fn tmr_cur_time_h(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4usize) as _) }
    }
    #[doc = "Timer parameter register"]
    #[inline(always)]
    pub const fn tmr_param(self) -> crate::common::Reg<regs::TmrParam, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf8usize) as _) }
    }
}
pub mod regs;
pub mod vals;
