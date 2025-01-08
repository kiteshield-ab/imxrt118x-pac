#[doc = "ASRC 56 kHz Period"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asr56k(pub u32);
impl Asr56k {
    #[doc = "Value for the Period of the 56 kHz Sampling Clock"]
    #[must_use]
    #[inline(always)]
    pub const fn asr56k(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0001_ffff;
        val as u32
    }
    #[doc = "Value for the Period of the 56 kHz Sampling Clock"]
    #[inline(always)]
    pub const fn set_asr56k(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
    }
}
impl Default for Asr56k {
    #[inline(always)]
    fn default() -> Asr56k {
        Asr56k(3571u64 as u32)
    }
}
impl core::fmt::Debug for Asr56k {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Asr56k")
            .field("asr56k", &self.asr56k())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Asr56k {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Asr56k {
            asr56k: u32,
        }
        let proxy = Asr56k {
            asr56k: self.asr56k(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ASRC 76 kHz Period"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asr76k(pub u32);
impl Asr76k {
    #[doc = "Value for the Period of the 76 kHz Sampling Clock"]
    #[must_use]
    #[inline(always)]
    pub const fn asr76k(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0001_ffff;
        val as u32
    }
    #[doc = "Value for the Period of the 76 kHz Sampling Clock"]
    #[inline(always)]
    pub const fn set_asr76k(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
    }
}
impl Default for Asr76k {
    #[inline(always)]
    fn default() -> Asr76k {
        Asr76k(2631u64 as u32)
    }
}
impl core::fmt::Debug for Asr76k {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Asr76k")
            .field("asr76k", &self.asr76k())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Asr76k {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Asr76k {
            asr76k: u32,
        }
        let proxy = Asr76k {
            asr76k: self.asr76k(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ASRC Channel Counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asrccr(pub u32);
impl Asrccr {
    #[doc = "Channel Counter for Pair A's Input FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn acia(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel Counter for Pair A's Input FIFO"]
    #[inline(always)]
    pub const fn set_acia(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Channel Counter for Pair B's Input FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn acib(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel Counter for Pair B's Input FIFO"]
    #[inline(always)]
    pub const fn set_acib(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Channel Counter for Pair C's Input FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn acic(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel Counter for Pair C's Input FIFO"]
    #[inline(always)]
    pub const fn set_acic(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Channel Counter for Pair A's Output FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn acoa(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel Counter for Pair A's Output FIFO"]
    #[inline(always)]
    pub const fn set_acoa(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Channel Counter for Pair B's Output FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn acob(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel Counter for Pair B's Output FIFO"]
    #[inline(always)]
    pub const fn set_acob(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Channel Counter for Pair C's Output FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn acoc(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel Counter for Pair C's Output FIFO"]
    #[inline(always)]
    pub const fn set_acoc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
}
impl Default for Asrccr {
    #[inline(always)]
    fn default() -> Asrccr {
        Asrccr(0u64 as u32)
    }
}
impl core::fmt::Debug for Asrccr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Asrccr")
            .field("acia", &self.acia())
            .field("acib", &self.acib())
            .field("acic", &self.acic())
            .field("acoa", &self.acoa())
            .field("acob", &self.acob())
            .field("acoc", &self.acoc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Asrccr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Asrccr {
            acia: u8,
            acib: u8,
            acic: u8,
            acoa: u8,
            acob: u8,
            acoc: u8,
        }
        let proxy = Asrccr {
            acia: self.acia(),
            acib: self.acib(),
            acic: self.acic(),
            acoa: self.acoa(),
            acob: self.acob(),
            acoc: self.acoc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ASRC Clock Divider 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asrcdr1(pub u32);
impl Asrcdr1 {
    #[doc = "Input Clock Prescaler A"]
    #[must_use]
    #[inline(always)]
    pub const fn aicpa(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Input Clock Prescaler A"]
    #[inline(always)]
    pub const fn set_aicpa(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Input Clock Divider A"]
    #[must_use]
    #[inline(always)]
    pub const fn aicda(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Input Clock Divider A"]
    #[inline(always)]
    pub const fn set_aicda(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "Input Clock Prescaler B"]
    #[must_use]
    #[inline(always)]
    pub const fn aicpb(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x07;
        val as u8
    }
    #[doc = "Input Clock Prescaler B"]
    #[inline(always)]
    pub const fn set_aicpb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
    }
    #[doc = "Input Clock Divider B"]
    #[must_use]
    #[inline(always)]
    pub const fn aicdb(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x07;
        val as u8
    }
    #[doc = "Input Clock Divider B"]
    #[inline(always)]
    pub const fn set_aicdb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
    }
    #[doc = "Output Clock Prescaler A"]
    #[must_use]
    #[inline(always)]
    pub const fn aocpa(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "Output Clock Prescaler A"]
    #[inline(always)]
    pub const fn set_aocpa(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Output Clock Divider A"]
    #[must_use]
    #[inline(always)]
    pub const fn aocda(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x07;
        val as u8
    }
    #[doc = "Output Clock Divider A"]
    #[inline(always)]
    pub const fn set_aocda(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
    }
    #[doc = "Output Clock Prescaler B"]
    #[must_use]
    #[inline(always)]
    pub const fn aocpb(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x07;
        val as u8
    }
    #[doc = "Output Clock Prescaler B"]
    #[inline(always)]
    pub const fn set_aocpb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
    }
    #[doc = "Output Clock Divider B"]
    #[must_use]
    #[inline(always)]
    pub const fn aocdb(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x07;
        val as u8
    }
    #[doc = "Output Clock Divider B"]
    #[inline(always)]
    pub const fn set_aocdb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
    }
}
impl Default for Asrcdr1 {
    #[inline(always)]
    fn default() -> Asrcdr1 {
        Asrcdr1(0u64 as u32)
    }
}
impl core::fmt::Debug for Asrcdr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Asrcdr1")
            .field("aicpa", &self.aicpa())
            .field("aicda", &self.aicda())
            .field("aicpb", &self.aicpb())
            .field("aicdb", &self.aicdb())
            .field("aocpa", &self.aocpa())
            .field("aocda", &self.aocda())
            .field("aocpb", &self.aocpb())
            .field("aocdb", &self.aocdb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Asrcdr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Asrcdr1 {
            aicpa: u8,
            aicda: u8,
            aicpb: u8,
            aicdb: u8,
            aocpa: u8,
            aocda: u8,
            aocpb: u8,
            aocdb: u8,
        }
        let proxy = Asrcdr1 {
            aicpa: self.aicpa(),
            aicda: self.aicda(),
            aicpb: self.aicpb(),
            aicdb: self.aicdb(),
            aocpa: self.aocpa(),
            aocda: self.aocda(),
            aocpb: self.aocpb(),
            aocdb: self.aocdb(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ASRC Clock Divider 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asrcdr2(pub u32);
impl Asrcdr2 {
    #[doc = "Input Clock Prescaler C"]
    #[must_use]
    #[inline(always)]
    pub const fn aicpc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Input Clock Prescaler C"]
    #[inline(always)]
    pub const fn set_aicpc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Input Clock Divider C"]
    #[must_use]
    #[inline(always)]
    pub const fn aicdc(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Input Clock Divider C"]
    #[inline(always)]
    pub const fn set_aicdc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "Output Clock Prescaler C"]
    #[must_use]
    #[inline(always)]
    pub const fn aocpc(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x07;
        val as u8
    }
    #[doc = "Output Clock Prescaler C"]
    #[inline(always)]
    pub const fn set_aocpc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
    }
    #[doc = "Output Clock Divider C"]
    #[must_use]
    #[inline(always)]
    pub const fn aocdc(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x07;
        val as u8
    }
    #[doc = "Output Clock Divider C"]
    #[inline(always)]
    pub const fn set_aocdc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
    }
}
impl Default for Asrcdr2 {
    #[inline(always)]
    fn default() -> Asrcdr2 {
        Asrcdr2(0u64 as u32)
    }
}
impl core::fmt::Debug for Asrcdr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Asrcdr2")
            .field("aicpc", &self.aicpc())
            .field("aicdc", &self.aicdc())
            .field("aocpc", &self.aocpc())
            .field("aocdc", &self.aocdc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Asrcdr2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Asrcdr2 {
            aicpc: u8,
            aicdc: u8,
            aocpc: u8,
            aocdc: u8,
        }
        let proxy = Asrcdr2 {
            aicpc: self.aicpc(),
            aicdc: self.aicdc(),
            aocpc: self.aocpc(),
            aocdc: self.aocdc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ASRC Filter Configuration Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asrcfg(pub u32);
impl Asrcfg {
    #[doc = "Pre-Processing Configuration for Conversion Pair A"]
    #[must_use]
    #[inline(always)]
    pub const fn premoda(&self) -> super::vals::Premoda {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Premoda::from_bits(val as u8)
    }
    #[doc = "Pre-Processing Configuration for Conversion Pair A"]
    #[inline(always)]
    pub const fn set_premoda(&mut self, val: super::vals::Premoda) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Post-Processing Configuration for Conversion Pair A"]
    #[must_use]
    #[inline(always)]
    pub const fn postmoda(&self) -> super::vals::Postmoda {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Postmoda::from_bits(val as u8)
    }
    #[doc = "Post-Processing Configuration for Conversion Pair A"]
    #[inline(always)]
    pub const fn set_postmoda(&mut self, val: super::vals::Postmoda) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Pre-Processing Configuration for Conversion Pair B"]
    #[must_use]
    #[inline(always)]
    pub const fn premodb(&self) -> super::vals::Premodb {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Premodb::from_bits(val as u8)
    }
    #[doc = "Pre-Processing Configuration for Conversion Pair B"]
    #[inline(always)]
    pub const fn set_premodb(&mut self, val: super::vals::Premodb) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Post-Processing Configuration for Conversion Pair B"]
    #[must_use]
    #[inline(always)]
    pub const fn postmodb(&self) -> super::vals::Postmodb {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Postmodb::from_bits(val as u8)
    }
    #[doc = "Post-Processing Configuration for Conversion Pair B"]
    #[inline(always)]
    pub const fn set_postmodb(&mut self, val: super::vals::Postmodb) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Pre-Processing Configuration for Conversion Pair C"]
    #[must_use]
    #[inline(always)]
    pub const fn premodc(&self) -> super::vals::Premodc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Premodc::from_bits(val as u8)
    }
    #[doc = "Pre-Processing Configuration for Conversion Pair C"]
    #[inline(always)]
    pub const fn set_premodc(&mut self, val: super::vals::Premodc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Post-Processing Configuration for Conversion Pair C"]
    #[must_use]
    #[inline(always)]
    pub const fn postmodc(&self) -> super::vals::Postmodc {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Postmodc::from_bits(val as u8)
    }
    #[doc = "Post-Processing Configuration for Conversion Pair C"]
    #[inline(always)]
    pub const fn set_postmodc(&mut self, val: super::vals::Postmodc) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Not Use Default Parameters for RAM-stored Parameters For Conversion Pair A"]
    #[must_use]
    #[inline(always)]
    pub const fn ndpra(&self) -> super::vals::Ndpra {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Ndpra::from_bits(val as u8)
    }
    #[doc = "Not Use Default Parameters for RAM-stored Parameters For Conversion Pair A"]
    #[inline(always)]
    pub const fn set_ndpra(&mut self, val: super::vals::Ndpra) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Not Use Default Parameters for RAM-Stored Parameters For Conversion Pair B"]
    #[must_use]
    #[inline(always)]
    pub const fn ndprb(&self) -> super::vals::Ndprb {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Ndprb::from_bits(val as u8)
    }
    #[doc = "Not Use Default Parameters for RAM-Stored Parameters For Conversion Pair B"]
    #[inline(always)]
    pub const fn set_ndprb(&mut self, val: super::vals::Ndprb) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Not Use Default Parameters for RAM-Stored Parameters For Conversion Pair C"]
    #[must_use]
    #[inline(always)]
    pub const fn ndprc(&self) -> super::vals::Ndprc {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Ndprc::from_bits(val as u8)
    }
    #[doc = "Not Use Default Parameters for RAM-Stored Parameters For Conversion Pair C"]
    #[inline(always)]
    pub const fn set_ndprc(&mut self, val: super::vals::Ndprc) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Initialization for Conversion Pair A is served"]
    #[must_use]
    #[inline(always)]
    pub const fn inirqa(&self) -> super::vals::Inirqa {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Inirqa::from_bits(val as u8)
    }
    #[doc = "Initialization for Conversion Pair A is served"]
    #[inline(always)]
    pub const fn set_inirqa(&mut self, val: super::vals::Inirqa) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Initialization for Conversion Pair B is Served"]
    #[must_use]
    #[inline(always)]
    pub const fn inirqb(&self) -> super::vals::Inirqb {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Inirqb::from_bits(val as u8)
    }
    #[doc = "Initialization for Conversion Pair B is Served"]
    #[inline(always)]
    pub const fn set_inirqb(&mut self, val: super::vals::Inirqb) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Initialization for Conversion Pair C is Served"]
    #[must_use]
    #[inline(always)]
    pub const fn inirqc(&self) -> super::vals::Inirqc {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Inirqc::from_bits(val as u8)
    }
    #[doc = "Initialization for Conversion Pair C is Served"]
    #[inline(always)]
    pub const fn set_inirqc(&mut self, val: super::vals::Inirqc) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
}
impl Default for Asrcfg {
    #[inline(always)]
    fn default() -> Asrcfg {
        Asrcfg(0u64 as u32)
    }
}
impl core::fmt::Debug for Asrcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Asrcfg")
            .field("premoda", &self.premoda())
            .field("postmoda", &self.postmoda())
            .field("premodb", &self.premodb())
            .field("postmodb", &self.postmodb())
            .field("premodc", &self.premodc())
            .field("postmodc", &self.postmodc())
            .field("ndpra", &self.ndpra())
            .field("ndprb", &self.ndprb())
            .field("ndprc", &self.ndprc())
            .field("inirqa", &self.inirqa())
            .field("inirqb", &self.inirqb())
            .field("inirqc", &self.inirqc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Asrcfg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Asrcfg {
            premoda: super::vals::Premoda,
            postmoda: super::vals::Postmoda,
            premodb: super::vals::Premodb,
            postmodb: super::vals::Postmodb,
            premodc: super::vals::Premodc,
            postmodc: super::vals::Postmodc,
            ndpra: super::vals::Ndpra,
            ndprb: super::vals::Ndprb,
            ndprc: super::vals::Ndprc,
            inirqa: super::vals::Inirqa,
            inirqb: super::vals::Inirqb,
            inirqc: super::vals::Inirqc,
        }
        let proxy = Asrcfg {
            premoda: self.premoda(),
            postmoda: self.postmoda(),
            premodb: self.premodb(),
            postmodb: self.postmodb(),
            premodc: self.premodc(),
            postmodc: self.postmodc(),
            ndpra: self.ndpra(),
            ndprb: self.ndprb(),
            ndprc: self.ndprc(),
            inirqa: self.inirqa(),
            inirqb: self.inirqb(),
            inirqc: self.inirqc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ASRC Channel Number Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asrcncr(pub u32);
impl Asrcncr {
    #[doc = "Number of A Channels"]
    #[must_use]
    #[inline(always)]
    pub const fn anca(&self) -> super::vals::Anca {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Anca::from_bits(val as u8)
    }
    #[doc = "Number of A Channels"]
    #[inline(always)]
    pub const fn set_anca(&mut self, val: super::vals::Anca) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Number of B Channels"]
    #[must_use]
    #[inline(always)]
    pub const fn ancb(&self) -> super::vals::Ancb {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::Ancb::from_bits(val as u8)
    }
    #[doc = "Number of B Channels"]
    #[inline(always)]
    pub const fn set_ancb(&mut self, val: super::vals::Ancb) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "Number of C Channels"]
    #[must_use]
    #[inline(always)]
    pub const fn ancc(&self) -> super::vals::Ancc {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Ancc::from_bits(val as u8)
    }
    #[doc = "Number of C Channels"]
    #[inline(always)]
    pub const fn set_ancc(&mut self, val: super::vals::Ancc) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
}
impl Default for Asrcncr {
    #[inline(always)]
    fn default() -> Asrcncr {
        Asrcncr(0u64 as u32)
    }
}
impl core::fmt::Debug for Asrcncr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Asrcncr")
            .field("anca", &self.anca())
            .field("ancb", &self.ancb())
            .field("ancc", &self.ancc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Asrcncr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Asrcncr {
            anca: super::vals::Anca,
            ancb: super::vals::Ancb,
            ancc: super::vals::Ancc,
        }
        let proxy = Asrcncr {
            anca: self.anca(),
            ancb: self.ancb(),
            ancc: self.ancc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ASRC Clock Source"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asrcsr(pub u32);
impl Asrcsr {
    #[doc = "Input Clock Source A"]
    #[must_use]
    #[inline(always)]
    pub const fn aicsa(&self) -> super::vals::Aicsa {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Aicsa::from_bits(val as u8)
    }
    #[doc = "Input Clock Source A"]
    #[inline(always)]
    pub const fn set_aicsa(&mut self, val: super::vals::Aicsa) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Input Clock Source B"]
    #[must_use]
    #[inline(always)]
    pub const fn aicsb(&self) -> super::vals::Aicsb {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::Aicsb::from_bits(val as u8)
    }
    #[doc = "Input Clock Source B"]
    #[inline(always)]
    pub const fn set_aicsb(&mut self, val: super::vals::Aicsb) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "Input Clock Source C"]
    #[must_use]
    #[inline(always)]
    pub const fn aicsc(&self) -> super::vals::Aicsc {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Aicsc::from_bits(val as u8)
    }
    #[doc = "Input Clock Source C"]
    #[inline(always)]
    pub const fn set_aicsc(&mut self, val: super::vals::Aicsc) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Output Clock Source A"]
    #[must_use]
    #[inline(always)]
    pub const fn aocsa(&self) -> super::vals::Aocsa {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Aocsa::from_bits(val as u8)
    }
    #[doc = "Output Clock Source A"]
    #[inline(always)]
    pub const fn set_aocsa(&mut self, val: super::vals::Aocsa) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "Output Clock Source B"]
    #[must_use]
    #[inline(always)]
    pub const fn aocsb(&self) -> super::vals::Aocsb {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Aocsb::from_bits(val as u8)
    }
    #[doc = "Output Clock Source B"]
    #[inline(always)]
    pub const fn set_aocsb(&mut self, val: super::vals::Aocsb) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Output Clock Source C"]
    #[must_use]
    #[inline(always)]
    pub const fn aocsc(&self) -> super::vals::Aocsc {
        let val = (self.0 >> 20usize) & 0x0f;
        super::vals::Aocsc::from_bits(val as u8)
    }
    #[doc = "Output Clock Source C"]
    #[inline(always)]
    pub const fn set_aocsc(&mut self, val: super::vals::Aocsc) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
    }
}
impl Default for Asrcsr {
    #[inline(always)]
    fn default() -> Asrcsr {
        Asrcsr(0u64 as u32)
    }
}
impl core::fmt::Debug for Asrcsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Asrcsr")
            .field("aicsa", &self.aicsa())
            .field("aicsb", &self.aicsb())
            .field("aicsc", &self.aicsc())
            .field("aocsa", &self.aocsa())
            .field("aocsb", &self.aocsb())
            .field("aocsc", &self.aocsc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Asrcsr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Asrcsr {
            aicsa: super::vals::Aicsa,
            aicsb: super::vals::Aicsb,
            aicsc: super::vals::Aicsc,
            aocsa: super::vals::Aocsa,
            aocsb: super::vals::Aocsb,
            aocsc: super::vals::Aocsc,
        }
        let proxy = Asrcsr {
            aicsa: self.aicsa(),
            aicsb: self.aicsb(),
            aicsc: self.aicsc(),
            aocsa: self.aocsa(),
            aocsb: self.aocsb(),
            aocsc: self.aocsc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ASRC Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asrctr(pub u32);
impl Asrctr {
    #[doc = "ASRC Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn asrcen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ASRC Enable"]
    #[inline(always)]
    pub const fn set_asrcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ASRC Enable A"]
    #[must_use]
    #[inline(always)]
    pub const fn asrea(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ASRC Enable A"]
    #[inline(always)]
    pub const fn set_asrea(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "ASRC Enable B"]
    #[must_use]
    #[inline(always)]
    pub const fn asreb(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "ASRC Enable B"]
    #[inline(always)]
    pub const fn set_asreb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "ASRC Enable C"]
    #[must_use]
    #[inline(always)]
    pub const fn asrec(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "ASRC Enable C"]
    #[inline(always)]
    pub const fn set_asrec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Software Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn srst(&self) -> super::vals::Srst {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Srst::from_bits(val as u8)
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn set_srst(&mut self, val: super::vals::Srst) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Use Ideal Ratio for Pair A"]
    #[must_use]
    #[inline(always)]
    pub const fn idra(&self) -> super::vals::Idra {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Idra::from_bits(val as u8)
    }
    #[doc = "Use Ideal Ratio for Pair A"]
    #[inline(always)]
    pub const fn set_idra(&mut self, val: super::vals::Idra) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Use Ratio for Pair A"]
    #[must_use]
    #[inline(always)]
    pub const fn usra(&self) -> super::vals::Usra {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Usra::from_bits(val as u8)
    }
    #[doc = "Use Ratio for Pair A"]
    #[inline(always)]
    pub const fn set_usra(&mut self, val: super::vals::Usra) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Use Ideal Ratio for Pair B"]
    #[must_use]
    #[inline(always)]
    pub const fn idrb(&self) -> super::vals::Idrb {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Idrb::from_bits(val as u8)
    }
    #[doc = "Use Ideal Ratio for Pair B"]
    #[inline(always)]
    pub const fn set_idrb(&mut self, val: super::vals::Idrb) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Use Ratio for Pair B"]
    #[must_use]
    #[inline(always)]
    pub const fn usrb(&self) -> super::vals::Usrb {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Usrb::from_bits(val as u8)
    }
    #[doc = "Use Ratio for Pair B"]
    #[inline(always)]
    pub const fn set_usrb(&mut self, val: super::vals::Usrb) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Use Ideal Ratio for Pair C"]
    #[must_use]
    #[inline(always)]
    pub const fn idrc(&self) -> super::vals::Idrc {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Idrc::from_bits(val as u8)
    }
    #[doc = "Use Ideal Ratio for Pair C"]
    #[inline(always)]
    pub const fn set_idrc(&mut self, val: super::vals::Idrc) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Use Ratio for Pair C"]
    #[must_use]
    #[inline(always)]
    pub const fn usrc(&self) -> super::vals::Usrc {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Usrc::from_bits(val as u8)
    }
    #[doc = "Use Ratio for Pair C"]
    #[inline(always)]
    pub const fn set_usrc(&mut self, val: super::vals::Usrc) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "ASRC Pair A Automatic Selection For Processing Options"]
    #[must_use]
    #[inline(always)]
    pub const fn atsa(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "ASRC Pair A Automatic Selection For Processing Options"]
    #[inline(always)]
    pub const fn set_atsa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "ASRC Pair B Automatic Selection For Processing Options"]
    #[must_use]
    #[inline(always)]
    pub const fn atsb(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "ASRC Pair B Automatic Selection For Processing Options"]
    #[inline(always)]
    pub const fn set_atsb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "ASRC Pair C Automatic Selection For Processing Options"]
    #[must_use]
    #[inline(always)]
    pub const fn atsc(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "ASRC Pair C Automatic Selection For Processing Options"]
    #[inline(always)]
    pub const fn set_atsc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
}
impl Default for Asrctr {
    #[inline(always)]
    fn default() -> Asrctr {
        Asrctr(0u64 as u32)
    }
}
impl core::fmt::Debug for Asrctr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Asrctr")
            .field("asrcen", &self.asrcen())
            .field("asrea", &self.asrea())
            .field("asreb", &self.asreb())
            .field("asrec", &self.asrec())
            .field("srst", &self.srst())
            .field("idra", &self.idra())
            .field("usra", &self.usra())
            .field("idrb", &self.idrb())
            .field("usrb", &self.usrb())
            .field("idrc", &self.idrc())
            .field("usrc", &self.usrc())
            .field("atsa", &self.atsa())
            .field("atsb", &self.atsb())
            .field("atsc", &self.atsc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Asrctr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Asrctr {
            asrcen: bool,
            asrea: bool,
            asreb: bool,
            asrec: bool,
            srst: super::vals::Srst,
            idra: super::vals::Idra,
            usra: super::vals::Usra,
            idrb: super::vals::Idrb,
            usrb: super::vals::Usrb,
            idrc: super::vals::Idrc,
            usrc: super::vals::Usrc,
            atsa: bool,
            atsb: bool,
            atsc: bool,
        }
        let proxy = Asrctr {
            asrcen: self.asrcen(),
            asrea: self.asrea(),
            asreb: self.asreb(),
            asrec: self.asrec(),
            srst: self.srst(),
            idra: self.idra(),
            usra: self.usra(),
            idrb: self.idrb(),
            usrb: self.usrb(),
            idrc: self.idrc(),
            usrc: self.usrc(),
            atsa: self.atsa(),
            atsb: self.atsb(),
            atsc: self.atsc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ASRC Data Input for Pair x"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asrdia(pub u32);
impl Asrdia {
    #[doc = "Data"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Asrdia {
    #[inline(always)]
    fn default() -> Asrdia {
        Asrdia(0u64 as u32)
    }
}
impl core::fmt::Debug for Asrdia {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Asrdia")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Asrdia {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Asrdia {
            data: u32,
        }
        let proxy = Asrdia { data: self.data() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ASRC Data Input for Pair x"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asrdib(pub u32);
impl Asrdib {
    #[doc = "Data"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Asrdib {
    #[inline(always)]
    fn default() -> Asrdib {
        Asrdib(0u64 as u32)
    }
}
impl core::fmt::Debug for Asrdib {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Asrdib")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Asrdib {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Asrdib {
            data: u32,
        }
        let proxy = Asrdib { data: self.data() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ASRC Data Input for Pair x"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asrdic(pub u32);
impl Asrdic {
    #[doc = "Data"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Asrdic {
    #[inline(always)]
    fn default() -> Asrdic {
        Asrdic(0u64 as u32)
    }
}
impl core::fmt::Debug for Asrdic {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Asrdic")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Asrdic {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Asrdic {
            data: u32,
        }
        let proxy = Asrdic { data: self.data() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ASRC Data Output for Pair x"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asrdoa(pub u32);
impl Asrdoa {
    #[doc = "Data"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Asrdoa {
    #[inline(always)]
    fn default() -> Asrdoa {
        Asrdoa(0u64 as u32)
    }
}
impl core::fmt::Debug for Asrdoa {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Asrdoa")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Asrdoa {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Asrdoa {
            data: u32,
        }
        let proxy = Asrdoa { data: self.data() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ASRC Data Output for Pair x"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asrdob(pub u32);
impl Asrdob {
    #[doc = "Data"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Asrdob {
    #[inline(always)]
    fn default() -> Asrdob {
        Asrdob(0u64 as u32)
    }
}
impl core::fmt::Debug for Asrdob {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Asrdob")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Asrdob {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Asrdob {
            data: u32,
        }
        let proxy = Asrdob { data: self.data() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ASRC Data Output for Pair x"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asrdoc(pub u32);
impl Asrdoc {
    #[doc = "Data"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Asrdoc {
    #[inline(always)]
    fn default() -> Asrdoc {
        Asrdoc(0u64 as u32)
    }
}
impl core::fmt::Debug for Asrdoc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Asrdoc")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Asrdoc {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Asrdoc {
            data: u32,
        }
        let proxy = Asrdoc { data: self.data() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ASRC FIFO Status for Pair A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asrfsta(pub u32);
impl Asrfsta {
    #[doc = "Fillings for Pair A's Input FIFO per Channel"]
    #[must_use]
    #[inline(always)]
    pub const fn infifo_filla(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Fillings for Pair A's Input FIFO per Channel"]
    #[inline(always)]
    pub const fn set_infifo_filla(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Input FIFO is Near Empty for Pair A"]
    #[must_use]
    #[inline(always)]
    pub const fn iaea(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Input FIFO is Near Empty for Pair A"]
    #[inline(always)]
    pub const fn set_iaea(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Fillings for Pair A's Output FIFO per Channel"]
    #[must_use]
    #[inline(always)]
    pub const fn outfifo_filla(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x7f;
        val as u8
    }
    #[doc = "Fillings for Pair A's Output FIFO per Channel"]
    #[inline(always)]
    pub const fn set_outfifo_filla(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 12usize)) | (((val as u32) & 0x7f) << 12usize);
    }
    #[doc = "Output FIFO is Near Full for Pair A"]
    #[must_use]
    #[inline(always)]
    pub const fn oafa(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Output FIFO is Near Full for Pair A"]
    #[inline(always)]
    pub const fn set_oafa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Asrfsta {
    #[inline(always)]
    fn default() -> Asrfsta {
        Asrfsta(0u64 as u32)
    }
}
impl core::fmt::Debug for Asrfsta {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Asrfsta")
            .field("infifo_filla", &self.infifo_filla())
            .field("iaea", &self.iaea())
            .field("outfifo_filla", &self.outfifo_filla())
            .field("oafa", &self.oafa())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Asrfsta {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Asrfsta {
            infifo_filla: u8,
            iaea: bool,
            outfifo_filla: u8,
            oafa: bool,
        }
        let proxy = Asrfsta {
            infifo_filla: self.infifo_filla(),
            iaea: self.iaea(),
            outfifo_filla: self.outfifo_filla(),
            oafa: self.oafa(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ASRC FIFO Status for Pair B"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asrfstb(pub u32);
impl Asrfstb {
    #[doc = "Fillings for Pair B's Input FIFO per Channel"]
    #[must_use]
    #[inline(always)]
    pub const fn infifo_fillb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Fillings for Pair B's Input FIFO per Channel"]
    #[inline(always)]
    pub const fn set_infifo_fillb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Input FIFO is Near Empty for Pair B"]
    #[must_use]
    #[inline(always)]
    pub const fn iaeb(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Input FIFO is Near Empty for Pair B"]
    #[inline(always)]
    pub const fn set_iaeb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Fillings for Pair B's Output FIFO per Channel"]
    #[must_use]
    #[inline(always)]
    pub const fn outfifo_fillb(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x7f;
        val as u8
    }
    #[doc = "Fillings for Pair B's Output FIFO per Channel"]
    #[inline(always)]
    pub const fn set_outfifo_fillb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 12usize)) | (((val as u32) & 0x7f) << 12usize);
    }
    #[doc = "Output FIFO is Near Full for Pair B"]
    #[must_use]
    #[inline(always)]
    pub const fn oafb(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Output FIFO is Near Full for Pair B"]
    #[inline(always)]
    pub const fn set_oafb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Asrfstb {
    #[inline(always)]
    fn default() -> Asrfstb {
        Asrfstb(0u64 as u32)
    }
}
impl core::fmt::Debug for Asrfstb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Asrfstb")
            .field("infifo_fillb", &self.infifo_fillb())
            .field("iaeb", &self.iaeb())
            .field("outfifo_fillb", &self.outfifo_fillb())
            .field("oafb", &self.oafb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Asrfstb {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Asrfstb {
            infifo_fillb: u8,
            iaeb: bool,
            outfifo_fillb: u8,
            oafb: bool,
        }
        let proxy = Asrfstb {
            infifo_fillb: self.infifo_fillb(),
            iaeb: self.iaeb(),
            outfifo_fillb: self.outfifo_fillb(),
            oafb: self.oafb(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ASRC FIFO Status for Pair C"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asrfstc(pub u32);
impl Asrfstc {
    #[doc = "Fillings for Pair C's Input FIFO per Channel"]
    #[must_use]
    #[inline(always)]
    pub const fn infifo_fillc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Fillings for Pair C's Input FIFO per Channel"]
    #[inline(always)]
    pub const fn set_infifo_fillc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Input FIFO is Near Empty for Pair C"]
    #[must_use]
    #[inline(always)]
    pub const fn iaec(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Input FIFO is Near Empty for Pair C"]
    #[inline(always)]
    pub const fn set_iaec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Fillings for Pair C's Output FIFO per Channel"]
    #[must_use]
    #[inline(always)]
    pub const fn outfifo_fillc(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x7f;
        val as u8
    }
    #[doc = "Fillings for Pair C's Output FIFO per Channel"]
    #[inline(always)]
    pub const fn set_outfifo_fillc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 12usize)) | (((val as u32) & 0x7f) << 12usize);
    }
    #[doc = "Output FIFO is Near Full for Pair C"]
    #[must_use]
    #[inline(always)]
    pub const fn oafc(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Output FIFO is Near Full for Pair C"]
    #[inline(always)]
    pub const fn set_oafc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Asrfstc {
    #[inline(always)]
    fn default() -> Asrfstc {
        Asrfstc(0u64 as u32)
    }
}
impl core::fmt::Debug for Asrfstc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Asrfstc")
            .field("infifo_fillc", &self.infifo_fillc())
            .field("iaec", &self.iaec())
            .field("outfifo_fillc", &self.outfifo_fillc())
            .field("oafc", &self.oafc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Asrfstc {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Asrfstc {
            infifo_fillc: u8,
            iaec: bool,
            outfifo_fillc: u8,
            oafc: bool,
        }
        let proxy = Asrfstc {
            infifo_fillc: self.infifo_fillc(),
            iaec: self.iaec(),
            outfifo_fillc: self.outfifo_fillc(),
            oafc: self.oafc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ASRC Ideal Ratio for Pair A-High Part"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asridrha(pub u32);
impl Asridrha {
    #[doc = "Ideal Ratio A High"]
    #[must_use]
    #[inline(always)]
    pub const fn idratioa_h(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Ideal Ratio A High"]
    #[inline(always)]
    pub const fn set_idratioa_h(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Asridrha {
    #[inline(always)]
    fn default() -> Asridrha {
        Asridrha(0u64 as u32)
    }
}
impl core::fmt::Debug for Asridrha {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Asridrha")
            .field("idratioa_h", &self.idratioa_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Asridrha {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Asridrha {
            idratioa_h: u8,
        }
        let proxy = Asridrha {
            idratioa_h: self.idratioa_h(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ASRC Ideal Ratio for Pair B-High Part"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asridrhb(pub u32);
impl Asridrhb {
    #[doc = "Ideal Ratio B High"]
    #[must_use]
    #[inline(always)]
    pub const fn idratiob_h(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Ideal Ratio B High"]
    #[inline(always)]
    pub const fn set_idratiob_h(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Asridrhb {
    #[inline(always)]
    fn default() -> Asridrhb {
        Asridrhb(0u64 as u32)
    }
}
impl core::fmt::Debug for Asridrhb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Asridrhb")
            .field("idratiob_h", &self.idratiob_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Asridrhb {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Asridrhb {
            idratiob_h: u8,
        }
        let proxy = Asridrhb {
            idratiob_h: self.idratiob_h(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ASRC Ideal Ratio for Pair C-High Part"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asridrhc(pub u32);
impl Asridrhc {
    #[doc = "Ideal Ratio C High"]
    #[must_use]
    #[inline(always)]
    pub const fn idratioc_h(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Ideal Ratio C High"]
    #[inline(always)]
    pub const fn set_idratioc_h(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Asridrhc {
    #[inline(always)]
    fn default() -> Asridrhc {
        Asridrhc(0u64 as u32)
    }
}
impl core::fmt::Debug for Asridrhc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Asridrhc")
            .field("idratioc_h", &self.idratioc_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Asridrhc {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Asridrhc {
            idratioc_h: u8,
        }
        let proxy = Asridrhc {
            idratioc_h: self.idratioc_h(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ASRC Ideal Ratio for Pair A -Low Part"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asridrla(pub u32);
impl Asridrla {
    #[doc = "Ideal Ratio A Low"]
    #[must_use]
    #[inline(always)]
    pub const fn idratioa_l(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Ideal Ratio A Low"]
    #[inline(always)]
    pub const fn set_idratioa_l(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Asridrla {
    #[inline(always)]
    fn default() -> Asridrla {
        Asridrla(0u64 as u32)
    }
}
impl core::fmt::Debug for Asridrla {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Asridrla")
            .field("idratioa_l", &self.idratioa_l())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Asridrla {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Asridrla {
            idratioa_l: u32,
        }
        let proxy = Asridrla {
            idratioa_l: self.idratioa_l(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ASRC Ideal Ratio for Pair B-Low Part"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asridrlb(pub u32);
impl Asridrlb {
    #[doc = "Ideal Ratio B Low"]
    #[must_use]
    #[inline(always)]
    pub const fn idratiob_l(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Ideal Ratio B Low"]
    #[inline(always)]
    pub const fn set_idratiob_l(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Asridrlb {
    #[inline(always)]
    fn default() -> Asridrlb {
        Asridrlb(0u64 as u32)
    }
}
impl core::fmt::Debug for Asridrlb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Asridrlb")
            .field("idratiob_l", &self.idratiob_l())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Asridrlb {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Asridrlb {
            idratiob_l: u32,
        }
        let proxy = Asridrlb {
            idratiob_l: self.idratiob_l(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ASRC Ideal Ratio for Pair C-Low Part"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asridrlc(pub u32);
impl Asridrlc {
    #[doc = "Ideal Ratio C Low"]
    #[must_use]
    #[inline(always)]
    pub const fn idratioc_l(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Ideal Ratio C Low"]
    #[inline(always)]
    pub const fn set_idratioc_l(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Asridrlc {
    #[inline(always)]
    fn default() -> Asridrlc {
        Asridrlc(0u64 as u32)
    }
}
impl core::fmt::Debug for Asridrlc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Asridrlc")
            .field("idratioc_l", &self.idratioc_l())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Asridrlc {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Asridrlc {
            idratioc_l: u32,
        }
        let proxy = Asridrlc {
            idratioc_l: self.idratioc_l(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ASRC Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asrier(pub u32);
impl Asrier {
    #[doc = "Pair A Data Input Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn adiea(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Pair A Data Input Interrupt Enable"]
    #[inline(always)]
    pub const fn set_adiea(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Pair B Data Input Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn adieb(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Pair B Data Input Interrupt Enable"]
    #[inline(always)]
    pub const fn set_adieb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Pair C Data Input Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn adiec(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Pair C Data Input Interrupt Enable"]
    #[inline(always)]
    pub const fn set_adiec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Pair A Data Output Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn adoea(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Pair A Data Output Interrupt Enable"]
    #[inline(always)]
    pub const fn set_adoea(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Pair B Data Output Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn adoeb(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pair B Data Output Interrupt Enable"]
    #[inline(always)]
    pub const fn set_adoeb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pair C Data Output Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn adoec(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pair C Data Output Interrupt Enable"]
    #[inline(always)]
    pub const fn set_adoec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Overload Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn aolie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Overload Interrupt Enable"]
    #[inline(always)]
    pub const fn set_aolie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "FP in Wait State Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn afpwe(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "FP in Wait State Interrupt Enable"]
    #[inline(always)]
    pub const fn set_afpwe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Asrier {
    #[inline(always)]
    fn default() -> Asrier {
        Asrier(0u64 as u32)
    }
}
impl core::fmt::Debug for Asrier {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Asrier")
            .field("adiea", &self.adiea())
            .field("adieb", &self.adieb())
            .field("adiec", &self.adiec())
            .field("adoea", &self.adoea())
            .field("adoeb", &self.adoeb())
            .field("adoec", &self.adoec())
            .field("aolie", &self.aolie())
            .field("afpwe", &self.afpwe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Asrier {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Asrier {
            adiea: bool,
            adieb: bool,
            adiec: bool,
            adoea: bool,
            adoeb: bool,
            adoec: bool,
            aolie: bool,
            afpwe: bool,
        }
        let proxy = Asrier {
            adiea: self.adiea(),
            adieb: self.adieb(),
            adiec: self.adiec(),
            adoea: self.adoea(),
            adoeb: self.adoeb(),
            adoec: self.adoec(),
            aolie: self.aolie(),
            afpwe: self.afpwe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ASRC Misc Control 1 for Pair X"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asrmcr1(pub u32);
impl Asrmcr1 {
    #[doc = "Bit Width Option of the Output FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn ow16(&self) -> super::vals::Ow16 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ow16::from_bits(val as u8)
    }
    #[doc = "Bit Width Option of the Output FIFO"]
    #[inline(always)]
    pub const fn set_ow16(&mut self, val: super::vals::Ow16) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Sign Extension Option of the Output FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn osgn(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Sign Extension Option of the Output FIFO"]
    #[inline(always)]
    pub const fn set_osgn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Data Alignment of the Output FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn omsb(&self) -> super::vals::Omsb {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Omsb::from_bits(val as u8)
    }
    #[doc = "Data Alignment of the Output FIFO"]
    #[inline(always)]
    pub const fn set_omsb(&mut self, val: super::vals::Omsb) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Data Alignment of the Input FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn imsb(&self) -> super::vals::Imsb {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Imsb::from_bits(val as u8)
    }
    #[doc = "Data Alignment of the Input FIFO"]
    #[inline(always)]
    pub const fn set_imsb(&mut self, val: super::vals::Imsb) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Data Width of the Input FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn iwd(&self) -> super::vals::Iwd {
        let val = (self.0 >> 9usize) & 0x03;
        super::vals::Iwd::from_bits(val as u8)
    }
    #[doc = "Data Width of the Input FIFO"]
    #[inline(always)]
    pub const fn set_iwd(&mut self, val: super::vals::Iwd) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val.to_bits() as u32) & 0x03) << 9usize);
    }
}
impl Default for Asrmcr1 {
    #[inline(always)]
    fn default() -> Asrmcr1 {
        Asrmcr1(0u64 as u32)
    }
}
impl core::fmt::Debug for Asrmcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Asrmcr1")
            .field("ow16", &self.ow16())
            .field("osgn", &self.osgn())
            .field("omsb", &self.omsb())
            .field("imsb", &self.imsb())
            .field("iwd", &self.iwd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Asrmcr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Asrmcr1 {
            ow16: super::vals::Ow16,
            osgn: bool,
            omsb: super::vals::Omsb,
            imsb: super::vals::Imsb,
            iwd: super::vals::Iwd,
        }
        let proxy = Asrmcr1 {
            ow16: self.ow16(),
            osgn: self.osgn(),
            omsb: self.omsb(),
            imsb: self.imsb(),
            iwd: self.iwd(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ASRC Misc Control for Pair A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asrmcra(pub u32);
impl Asrmcra {
    #[doc = "Threshold for Pair A's Input FIFO per Channel"]
    #[must_use]
    #[inline(always)]
    pub const fn infifo_thresholda(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Threshold for Pair A's Input FIFO per Channel"]
    #[inline(always)]
    pub const fn set_infifo_thresholda(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Re-sync Output FIFO Channel Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn rsynofa(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Re-sync Output FIFO Channel Counter"]
    #[inline(always)]
    pub const fn set_rsynofa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Re-sync Input FIFO Channel Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn rsynifa(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Re-sync Input FIFO Channel Counter"]
    #[inline(always)]
    pub const fn set_rsynifa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Threshold for Pair A's Output FIFO per Channel"]
    #[must_use]
    #[inline(always)]
    pub const fn outfifo_thresholda(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x3f;
        val as u8
    }
    #[doc = "Threshold for Pair A's Output FIFO per Channel"]
    #[inline(always)]
    pub const fn set_outfifo_thresholda(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 12usize)) | (((val as u32) & 0x3f) << 12usize);
    }
    #[doc = "Bypass Polyphase Filtering for Pair A"]
    #[must_use]
    #[inline(always)]
    pub const fn bypasspolya(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass Polyphase Filtering for Pair A"]
    #[inline(always)]
    pub const fn set_bypasspolya(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Stall Pair A Conversion in Case of Buffer Near Empty/Full Condition"]
    #[must_use]
    #[inline(always)]
    pub const fn bufstalla(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Stall Pair A Conversion in Case of Buffer Near Empty/Full Condition"]
    #[inline(always)]
    pub const fn set_bufstalla(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Use External Thresholds for FIFO Control of Pair A"]
    #[must_use]
    #[inline(always)]
    pub const fn extthrsha(&self) -> super::vals::Extthrsha {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Extthrsha::from_bits(val as u8)
    }
    #[doc = "Use External Thresholds for FIFO Control of Pair A"]
    #[inline(always)]
    pub const fn set_extthrsha(&mut self, val: super::vals::Extthrsha) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Zero Buffer A"]
    #[must_use]
    #[inline(always)]
    pub const fn zerobufa(&self) -> super::vals::Zerobufa {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Zerobufa::from_bits(val as u8)
    }
    #[doc = "Zero Buffer A"]
    #[inline(always)]
    pub const fn set_zerobufa(&mut self, val: super::vals::Zerobufa) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
}
impl Default for Asrmcra {
    #[inline(always)]
    fn default() -> Asrmcra {
        Asrmcra(0u64 as u32)
    }
}
impl core::fmt::Debug for Asrmcra {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Asrmcra")
            .field("infifo_thresholda", &self.infifo_thresholda())
            .field("rsynofa", &self.rsynofa())
            .field("rsynifa", &self.rsynifa())
            .field("outfifo_thresholda", &self.outfifo_thresholda())
            .field("bypasspolya", &self.bypasspolya())
            .field("bufstalla", &self.bufstalla())
            .field("extthrsha", &self.extthrsha())
            .field("zerobufa", &self.zerobufa())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Asrmcra {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Asrmcra {
            infifo_thresholda: u8,
            rsynofa: bool,
            rsynifa: bool,
            outfifo_thresholda: u8,
            bypasspolya: bool,
            bufstalla: bool,
            extthrsha: super::vals::Extthrsha,
            zerobufa: super::vals::Zerobufa,
        }
        let proxy = Asrmcra {
            infifo_thresholda: self.infifo_thresholda(),
            rsynofa: self.rsynofa(),
            rsynifa: self.rsynifa(),
            outfifo_thresholda: self.outfifo_thresholda(),
            bypasspolya: self.bypasspolya(),
            bufstalla: self.bufstalla(),
            extthrsha: self.extthrsha(),
            zerobufa: self.zerobufa(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ASRC Misc Control for Pair B"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asrmcrb(pub u32);
impl Asrmcrb {
    #[doc = "Threshold for Pair B's Input FIFO per Channel"]
    #[must_use]
    #[inline(always)]
    pub const fn infifo_thresholdb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Threshold for Pair B's Input FIFO per Channel"]
    #[inline(always)]
    pub const fn set_infifo_thresholdb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Re-sync Output FIFO Channel Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn rsynofb(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Re-sync Output FIFO Channel Counter"]
    #[inline(always)]
    pub const fn set_rsynofb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Re-sync Input FIFO Channel Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn rsynifb(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Re-sync Input FIFO Channel Counter"]
    #[inline(always)]
    pub const fn set_rsynifb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Threshold for Pair B's Output FIFO per Channel"]
    #[must_use]
    #[inline(always)]
    pub const fn outfifo_thresholdb(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x3f;
        val as u8
    }
    #[doc = "Threshold for Pair B's Output FIFO per Channel"]
    #[inline(always)]
    pub const fn set_outfifo_thresholdb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 12usize)) | (((val as u32) & 0x3f) << 12usize);
    }
    #[doc = "Bypass Polyphase Filtering for Pair B"]
    #[must_use]
    #[inline(always)]
    pub const fn bypasspolyb(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass Polyphase Filtering for Pair B"]
    #[inline(always)]
    pub const fn set_bypasspolyb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Stall Pair B Conversion in Case of Buffer Near Empty/Full Condition"]
    #[must_use]
    #[inline(always)]
    pub const fn bufstallb(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Stall Pair B Conversion in Case of Buffer Near Empty/Full Condition"]
    #[inline(always)]
    pub const fn set_bufstallb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Use External Thresholds for FIFO Control of Pair B"]
    #[must_use]
    #[inline(always)]
    pub const fn extthrshb(&self) -> super::vals::Extthrshb {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Extthrshb::from_bits(val as u8)
    }
    #[doc = "Use External Thresholds for FIFO Control of Pair B"]
    #[inline(always)]
    pub const fn set_extthrshb(&mut self, val: super::vals::Extthrshb) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Zero Buffer B"]
    #[must_use]
    #[inline(always)]
    pub const fn zerobufb(&self) -> super::vals::Zerobufb {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Zerobufb::from_bits(val as u8)
    }
    #[doc = "Zero Buffer B"]
    #[inline(always)]
    pub const fn set_zerobufb(&mut self, val: super::vals::Zerobufb) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
}
impl Default for Asrmcrb {
    #[inline(always)]
    fn default() -> Asrmcrb {
        Asrmcrb(0u64 as u32)
    }
}
impl core::fmt::Debug for Asrmcrb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Asrmcrb")
            .field("infifo_thresholdb", &self.infifo_thresholdb())
            .field("rsynofb", &self.rsynofb())
            .field("rsynifb", &self.rsynifb())
            .field("outfifo_thresholdb", &self.outfifo_thresholdb())
            .field("bypasspolyb", &self.bypasspolyb())
            .field("bufstallb", &self.bufstallb())
            .field("extthrshb", &self.extthrshb())
            .field("zerobufb", &self.zerobufb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Asrmcrb {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Asrmcrb {
            infifo_thresholdb: u8,
            rsynofb: bool,
            rsynifb: bool,
            outfifo_thresholdb: u8,
            bypasspolyb: bool,
            bufstallb: bool,
            extthrshb: super::vals::Extthrshb,
            zerobufb: super::vals::Zerobufb,
        }
        let proxy = Asrmcrb {
            infifo_thresholdb: self.infifo_thresholdb(),
            rsynofb: self.rsynofb(),
            rsynifb: self.rsynifb(),
            outfifo_thresholdb: self.outfifo_thresholdb(),
            bypasspolyb: self.bypasspolyb(),
            bufstallb: self.bufstallb(),
            extthrshb: self.extthrshb(),
            zerobufb: self.zerobufb(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ASRC Misc Control for Pair C"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asrmcrc(pub u32);
impl Asrmcrc {
    #[doc = "Threshold for Pair C's Input FIFO per Channel"]
    #[must_use]
    #[inline(always)]
    pub const fn infifo_thresholdc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Threshold for Pair C's Input FIFO per Channel"]
    #[inline(always)]
    pub const fn set_infifo_thresholdc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Re-sync Output FIFO Channel Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn rsynofc(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Re-sync Output FIFO Channel Counter"]
    #[inline(always)]
    pub const fn set_rsynofc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Re-sync Input FIFO Channel Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn rsynifc(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Re-sync Input FIFO Channel Counter"]
    #[inline(always)]
    pub const fn set_rsynifc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Threshold for Pair C's Output FIFO per Channel"]
    #[must_use]
    #[inline(always)]
    pub const fn outfifo_thresholdc(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x3f;
        val as u8
    }
    #[doc = "Threshold for Pair C's Output FIFO per Channel"]
    #[inline(always)]
    pub const fn set_outfifo_thresholdc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 12usize)) | (((val as u32) & 0x3f) << 12usize);
    }
    #[doc = "Bypass Polyphase Filtering for Pair C"]
    #[must_use]
    #[inline(always)]
    pub const fn bypasspolyc(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass Polyphase Filtering for Pair C"]
    #[inline(always)]
    pub const fn set_bypasspolyc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Stall Pair C Conversion in Case of Buffer Near Empty/Full Condition"]
    #[must_use]
    #[inline(always)]
    pub const fn bufstallc(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Stall Pair C Conversion in Case of Buffer Near Empty/Full Condition"]
    #[inline(always)]
    pub const fn set_bufstallc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Use External Thresholds for FIFO Control of Pair C"]
    #[must_use]
    #[inline(always)]
    pub const fn extthrshc(&self) -> super::vals::Extthrshc {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Extthrshc::from_bits(val as u8)
    }
    #[doc = "Use External Thresholds for FIFO Control of Pair C"]
    #[inline(always)]
    pub const fn set_extthrshc(&mut self, val: super::vals::Extthrshc) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Zero Buffer C"]
    #[must_use]
    #[inline(always)]
    pub const fn zerobufc(&self) -> super::vals::Zerobufc {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Zerobufc::from_bits(val as u8)
    }
    #[doc = "Zero Buffer C"]
    #[inline(always)]
    pub const fn set_zerobufc(&mut self, val: super::vals::Zerobufc) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
}
impl Default for Asrmcrc {
    #[inline(always)]
    fn default() -> Asrmcrc {
        Asrmcrc(0u64 as u32)
    }
}
impl core::fmt::Debug for Asrmcrc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Asrmcrc")
            .field("infifo_thresholdc", &self.infifo_thresholdc())
            .field("rsynofc", &self.rsynofc())
            .field("rsynifc", &self.rsynifc())
            .field("outfifo_thresholdc", &self.outfifo_thresholdc())
            .field("bypasspolyc", &self.bypasspolyc())
            .field("bufstallc", &self.bufstallc())
            .field("extthrshc", &self.extthrshc())
            .field("zerobufc", &self.zerobufc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Asrmcrc {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Asrmcrc {
            infifo_thresholdc: u8,
            rsynofc: bool,
            rsynifc: bool,
            outfifo_thresholdc: u8,
            bypasspolyc: bool,
            bufstallc: bool,
            extthrshc: super::vals::Extthrshc,
            zerobufc: super::vals::Zerobufc,
        }
        let proxy = Asrmcrc {
            infifo_thresholdc: self.infifo_thresholdc(),
            rsynofc: self.rsynofc(),
            rsynifc: self.rsynifc(),
            outfifo_thresholdc: self.outfifo_thresholdc(),
            bypasspolyc: self.bypasspolyc(),
            bufstallc: self.bufstallc(),
            extthrshc: self.extthrshc(),
            zerobufc: self.zerobufc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ASRC Parameter x"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asrpm(pub u32);
impl Asrpm {
    #[doc = "Parameter Value"]
    #[must_use]
    #[inline(always)]
    pub const fn parameter_value(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Parameter Value"]
    #[inline(always)]
    pub const fn set_parameter_value(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Asrpm {
    #[inline(always)]
    fn default() -> Asrpm {
        Asrpm(0u64 as u32)
    }
}
impl core::fmt::Debug for Asrpm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Asrpm")
            .field("parameter_value", &self.parameter_value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Asrpm {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Asrpm {
            parameter_value: u32,
        }
        let proxy = Asrpm {
            parameter_value: self.parameter_value(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ASRC Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asrstr(pub u32);
impl Asrstr {
    #[doc = "Number of Data in Input Data Buffer A is Less than Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn aidea(&self) -> super::vals::Aidea {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Aidea::from_bits(val as u8)
    }
    #[doc = "Number of Data in Input Data Buffer A is Less than Threshold"]
    #[inline(always)]
    pub const fn set_aidea(&mut self, val: super::vals::Aidea) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Number of Data in Input Data Buffer B is Less than Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn aideb(&self) -> super::vals::Aideb {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Aideb::from_bits(val as u8)
    }
    #[doc = "Number of Data in Input Data Buffer B is Less than Threshold"]
    #[inline(always)]
    pub const fn set_aideb(&mut self, val: super::vals::Aideb) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Number of Data in Input Data Buffer C is Less than Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn aidec(&self) -> super::vals::Aidec {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Aidec::from_bits(val as u8)
    }
    #[doc = "Number of Data in Input Data Buffer C is Less than Threshold"]
    #[inline(always)]
    pub const fn set_aidec(&mut self, val: super::vals::Aidec) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Number of Data in Output Data Buffer A is Greater than Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn aodfa(&self) -> super::vals::Aodfa {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Aodfa::from_bits(val as u8)
    }
    #[doc = "Number of Data in Output Data Buffer A is Greater than Threshold"]
    #[inline(always)]
    pub const fn set_aodfa(&mut self, val: super::vals::Aodfa) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Number of data in Output Data Buffer B is Greater than Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn aodfb(&self) -> super::vals::Aodfb {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Aodfb::from_bits(val as u8)
    }
    #[doc = "Number of data in Output Data Buffer B is Greater than Threshold"]
    #[inline(always)]
    pub const fn set_aodfb(&mut self, val: super::vals::Aodfb) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Number of data in Output Data Buffer C is Greater than Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn aodfc(&self) -> super::vals::Aodfc {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Aodfc::from_bits(val as u8)
    }
    #[doc = "Number of data in Output Data Buffer C is Greater than Threshold"]
    #[inline(always)]
    pub const fn set_aodfc(&mut self, val: super::vals::Aodfc) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Overload Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn aole(&self) -> super::vals::Aole {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Aole::from_bits(val as u8)
    }
    #[doc = "Overload Error Flag"]
    #[inline(always)]
    pub const fn set_aole(&mut self, val: super::vals::Aole) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "FP is in Wait States"]
    #[must_use]
    #[inline(always)]
    pub const fn fpwt(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "FP is in Wait States"]
    #[inline(always)]
    pub const fn set_fpwt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Input Data Buffer A has Underflowed"]
    #[must_use]
    #[inline(always)]
    pub const fn aidua(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Input Data Buffer A has Underflowed"]
    #[inline(always)]
    pub const fn set_aidua(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Input Data Buffer B has Underflowed"]
    #[must_use]
    #[inline(always)]
    pub const fn aidub(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Input Data Buffer B has Underflowed"]
    #[inline(always)]
    pub const fn set_aidub(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Input Data Buffer C has Underflowed"]
    #[must_use]
    #[inline(always)]
    pub const fn aiduc(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Input Data Buffer C has Underflowed"]
    #[inline(always)]
    pub const fn set_aiduc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Output Data Buffer A has Overflowed"]
    #[must_use]
    #[inline(always)]
    pub const fn aodoa(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Output Data Buffer A has Overflowed"]
    #[inline(always)]
    pub const fn set_aodoa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Output Data Buffer B has Overflowed"]
    #[must_use]
    #[inline(always)]
    pub const fn aodob(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Output Data Buffer B has Overflowed"]
    #[inline(always)]
    pub const fn set_aodob(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Output Data Buffer C has Overflowed"]
    #[must_use]
    #[inline(always)]
    pub const fn aodoc(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Output Data Buffer C has Overflowed"]
    #[inline(always)]
    pub const fn set_aodoc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Pair A Input Task Overload"]
    #[must_use]
    #[inline(always)]
    pub const fn aiola(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Pair A Input Task Overload"]
    #[inline(always)]
    pub const fn set_aiola(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Pair B Input Task Overload"]
    #[must_use]
    #[inline(always)]
    pub const fn aiolb(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Pair B Input Task Overload"]
    #[inline(always)]
    pub const fn set_aiolb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Pair C Input Task Overload"]
    #[must_use]
    #[inline(always)]
    pub const fn aiolc(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Pair C Input Task Overload"]
    #[inline(always)]
    pub const fn set_aiolc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Pair A Output Task Overload"]
    #[must_use]
    #[inline(always)]
    pub const fn aoola(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Pair A Output Task Overload"]
    #[inline(always)]
    pub const fn set_aoola(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Pair B Output Task Overload"]
    #[must_use]
    #[inline(always)]
    pub const fn aoolb(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Pair B Output Task Overload"]
    #[inline(always)]
    pub const fn set_aoolb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Pair C Output Task Overload"]
    #[must_use]
    #[inline(always)]
    pub const fn aoolc(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Pair C Output Task Overload"]
    #[inline(always)]
    pub const fn set_aoolc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Task Queue FIFO overload"]
    #[must_use]
    #[inline(always)]
    pub const fn atqol(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Task Queue FIFO overload"]
    #[inline(always)]
    pub const fn set_atqol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Digital Servo Loop (DSL) Counter Input to FIFO Ready"]
    #[must_use]
    #[inline(always)]
    pub const fn dslcnt(&self) -> super::vals::Dslcnt {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Dslcnt::from_bits(val as u8)
    }
    #[doc = "Digital Servo Loop (DSL) Counter Input to FIFO Ready"]
    #[inline(always)]
    pub const fn set_dslcnt(&mut self, val: super::vals::Dslcnt) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
}
impl Default for Asrstr {
    #[inline(always)]
    fn default() -> Asrstr {
        Asrstr(0u64 as u32)
    }
}
impl core::fmt::Debug for Asrstr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Asrstr")
            .field("aidea", &self.aidea())
            .field("aideb", &self.aideb())
            .field("aidec", &self.aidec())
            .field("aodfa", &self.aodfa())
            .field("aodfb", &self.aodfb())
            .field("aodfc", &self.aodfc())
            .field("aole", &self.aole())
            .field("fpwt", &self.fpwt())
            .field("aidua", &self.aidua())
            .field("aidub", &self.aidub())
            .field("aiduc", &self.aiduc())
            .field("aodoa", &self.aodoa())
            .field("aodob", &self.aodob())
            .field("aodoc", &self.aodoc())
            .field("aiola", &self.aiola())
            .field("aiolb", &self.aiolb())
            .field("aiolc", &self.aiolc())
            .field("aoola", &self.aoola())
            .field("aoolb", &self.aoolb())
            .field("aoolc", &self.aoolc())
            .field("atqol", &self.atqol())
            .field("dslcnt", &self.dslcnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Asrstr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Asrstr {
            aidea: super::vals::Aidea,
            aideb: super::vals::Aideb,
            aidec: super::vals::Aidec,
            aodfa: super::vals::Aodfa,
            aodfb: super::vals::Aodfb,
            aodfc: super::vals::Aodfc,
            aole: super::vals::Aole,
            fpwt: bool,
            aidua: bool,
            aidub: bool,
            aiduc: bool,
            aodoa: bool,
            aodob: bool,
            aodoc: bool,
            aiola: bool,
            aiolb: bool,
            aiolc: bool,
            aoola: bool,
            aoolb: bool,
            aoolc: bool,
            atqol: bool,
            dslcnt: super::vals::Dslcnt,
        }
        let proxy = Asrstr {
            aidea: self.aidea(),
            aideb: self.aideb(),
            aidec: self.aidec(),
            aodfa: self.aodfa(),
            aodfb: self.aodfb(),
            aodfc: self.aodfc(),
            aole: self.aole(),
            fpwt: self.fpwt(),
            aidua: self.aidua(),
            aidub: self.aidub(),
            aiduc: self.aiduc(),
            aodoa: self.aodoa(),
            aodob: self.aodob(),
            aodoc: self.aodoc(),
            aiola: self.aiola(),
            aiolb: self.aiolb(),
            aiolc: self.aiolc(),
            aoola: self.aoola(),
            aoolb: self.aoolb(),
            aoolc: self.aoolc(),
            atqol: self.atqol(),
            dslcnt: self.dslcnt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ASRC Task Queue FIFO 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asrtfr1(pub u32);
impl Asrtfr1 {
    #[doc = "Base Address for Task Queue FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn tf_base(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x7f;
        val as u8
    }
    #[doc = "Base Address for Task Queue FIFO"]
    #[inline(always)]
    pub const fn set_tf_base(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 6usize)) | (((val as u32) & 0x7f) << 6usize);
    }
    #[doc = "Current Number of Entries in Task Queue FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn tf_fill(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x7f;
        val as u8
    }
    #[doc = "Current Number of Entries in Task Queue FIFO"]
    #[inline(always)]
    pub const fn set_tf_fill(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 13usize)) | (((val as u32) & 0x7f) << 13usize);
    }
}
impl Default for Asrtfr1 {
    #[inline(always)]
    fn default() -> Asrtfr1 {
        Asrtfr1(0u64 as u32)
    }
}
impl core::fmt::Debug for Asrtfr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Asrtfr1")
            .field("tf_base", &self.tf_base())
            .field("tf_fill", &self.tf_fill())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Asrtfr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Asrtfr1 {
            tf_base: u8,
            tf_fill: u8,
        }
        let proxy = Asrtfr1 {
            tf_base: self.tf_base(),
            tf_fill: self.tf_fill(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
