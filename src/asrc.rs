#[doc = "ASRC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asrc {
    ptr: *mut u8,
}
unsafe impl Send for Asrc {}
unsafe impl Sync for Asrc {}
impl Asrc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "ASRC Control"]
    #[inline(always)]
    pub const fn asrctr(self) -> crate::common::Reg<regs::Asrctr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "ASRC Interrupt Enable"]
    #[inline(always)]
    pub const fn asrier(self) -> crate::common::Reg<regs::Asrier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "ASRC Channel Number Configuration"]
    #[inline(always)]
    pub const fn asrcncr(self) -> crate::common::Reg<regs::Asrcncr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "ASRC Filter Configuration Status"]
    #[inline(always)]
    pub const fn asrcfg(self) -> crate::common::Reg<regs::Asrcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "ASRC Clock Source"]
    #[inline(always)]
    pub const fn asrcsr(self) -> crate::common::Reg<regs::Asrcsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "ASRC Clock Divider 1"]
    #[inline(always)]
    pub const fn asrcdr1(self) -> crate::common::Reg<regs::Asrcdr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "ASRC Clock Divider 2"]
    #[inline(always)]
    pub const fn asrcdr2(self) -> crate::common::Reg<regs::Asrcdr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "ASRC Status"]
    #[inline(always)]
    pub const fn asrstr(self) -> crate::common::Reg<regs::Asrstr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "ASRC Parameter x"]
    #[inline(always)]
    pub const fn asrpm(self, n: usize) -> crate::common::Reg<regs::Asrpm, crate::common::RW> {
        assert!(n < 5usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize + n * 4usize) as _) }
    }
    #[doc = "ASRC Task Queue FIFO 1"]
    #[inline(always)]
    pub const fn asrtfr1(self) -> crate::common::Reg<regs::Asrtfr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "ASRC Channel Counter"]
    #[inline(always)]
    pub const fn asrccr(self) -> crate::common::Reg<regs::Asrccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "ASRC Data Input for Pair x"]
    #[inline(always)]
    pub const fn asrdia(self) -> crate::common::Reg<regs::Asrdia, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "ASRC Data Output for Pair x"]
    #[inline(always)]
    pub const fn asrdoa(self) -> crate::common::Reg<regs::Asrdoa, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "ASRC Data Input for Pair x"]
    #[inline(always)]
    pub const fn asrdib(self) -> crate::common::Reg<regs::Asrdib, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "ASRC Data Output for Pair x"]
    #[inline(always)]
    pub const fn asrdob(self) -> crate::common::Reg<regs::Asrdob, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "ASRC Data Input for Pair x"]
    #[inline(always)]
    pub const fn asrdic(self) -> crate::common::Reg<regs::Asrdic, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "ASRC Data Output for Pair x"]
    #[inline(always)]
    pub const fn asrdoc(self) -> crate::common::Reg<regs::Asrdoc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "ASRC Ideal Ratio for Pair A-High Part"]
    #[inline(always)]
    pub const fn asridrha(self) -> crate::common::Reg<regs::Asridrha, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "ASRC Ideal Ratio for Pair A -Low Part"]
    #[inline(always)]
    pub const fn asridrla(self) -> crate::common::Reg<regs::Asridrla, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "ASRC Ideal Ratio for Pair B-High Part"]
    #[inline(always)]
    pub const fn asridrhb(self) -> crate::common::Reg<regs::Asridrhb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "ASRC Ideal Ratio for Pair B-Low Part"]
    #[inline(always)]
    pub const fn asridrlb(self) -> crate::common::Reg<regs::Asridrlb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "ASRC Ideal Ratio for Pair C-High Part"]
    #[inline(always)]
    pub const fn asridrhc(self) -> crate::common::Reg<regs::Asridrhc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "ASRC Ideal Ratio for Pair C-Low Part"]
    #[inline(always)]
    pub const fn asridrlc(self) -> crate::common::Reg<regs::Asridrlc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "ASRC 76 kHz Period"]
    #[inline(always)]
    pub const fn asr76k(self) -> crate::common::Reg<regs::Asr76k, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[doc = "ASRC 56 kHz Period"]
    #[inline(always)]
    pub const fn asr56k(self) -> crate::common::Reg<regs::Asr56k, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "ASRC Misc Control for Pair A"]
    #[inline(always)]
    pub const fn asrmcra(self) -> crate::common::Reg<regs::Asrmcra, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "ASRC FIFO Status for Pair A"]
    #[inline(always)]
    pub const fn asrfsta(self) -> crate::common::Reg<regs::Asrfsta, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "ASRC Misc Control for Pair B"]
    #[inline(always)]
    pub const fn asrmcrb(self) -> crate::common::Reg<regs::Asrmcrb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "ASRC FIFO Status for Pair B"]
    #[inline(always)]
    pub const fn asrfstb(self) -> crate::common::Reg<regs::Asrfstb, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[doc = "ASRC Misc Control for Pair C"]
    #[inline(always)]
    pub const fn asrmcrc(self) -> crate::common::Reg<regs::Asrmcrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[doc = "ASRC FIFO Status for Pair C"]
    #[inline(always)]
    pub const fn asrfstc(self) -> crate::common::Reg<regs::Asrfstc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
    }
    #[doc = "ASRC Misc Control 1 for Pair X"]
    #[inline(always)]
    pub const fn asrmcr1(self, n: usize) -> crate::common::Reg<regs::Asrmcr1, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize + n * 4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
