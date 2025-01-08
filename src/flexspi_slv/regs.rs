#[doc = "Command Suit 1 Range"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmd1Range(pub u32);
impl Cmd1Range {
    #[doc = "Memory Range"]
    #[must_use]
    #[inline(always)]
    pub const fn range(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "Memory Range"]
    #[inline(always)]
    pub const fn set_range(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 10usize)) | (((val as u32) & 0x003f_ffff) << 10usize);
    }
}
impl Default for Cmd1Range {
    #[inline(always)]
    fn default() -> Cmd1Range {
        Cmd1Range(0u64 as u32)
    }
}
impl core::fmt::Debug for Cmd1Range {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmd1Range")
            .field("range", &self.range())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmd1Range {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cmd1Range {
            range: u32,
        }
        let proxy = Cmd1Range {
            range: self.range(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Command Suit 2 Range"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmd2Range(pub u32);
impl Cmd2Range {
    #[doc = "Memory Range"]
    #[must_use]
    #[inline(always)]
    pub const fn range(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "Memory Range"]
    #[inline(always)]
    pub const fn set_range(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 10usize)) | (((val as u32) & 0x003f_ffff) << 10usize);
    }
}
impl Default for Cmd2Range {
    #[inline(always)]
    fn default() -> Cmd2Range {
        Cmd2Range(0u64 as u32)
    }
}
impl core::fmt::Debug for Cmd2Range {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmd2Range")
            .field("range", &self.range())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmd2Range {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cmd2Range {
            range: u32,
        }
        let proxy = Cmd2Range {
            range: self.range(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Module Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ModuleControl(pub u32);
impl ModuleControl {
    #[doc = "Software Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn swreset(&self) -> super::vals::Swreset {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Swreset::from_bits(val as u8)
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn set_swreset(&mut self, val: super::vals::Swreset) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "SPI IO Mode Control"]
    #[must_use]
    #[inline(always)]
    pub const fn iomode(&self) -> super::vals::Iomode {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::Iomode::from_bits(val as u8)
    }
    #[doc = "SPI IO Mode Control"]
    #[inline(always)]
    pub const fn set_iomode(&mut self, val: super::vals::Iomode) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "DQS Stop Feature"]
    #[must_use]
    #[inline(always)]
    pub const fn dqsstop(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DQS Stop Feature"]
    #[inline(always)]
    pub const fn set_dqsstop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Chip Select Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn csmask(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Chip Select Mask"]
    #[inline(always)]
    pub const fn set_csmask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Block Read"]
    #[must_use]
    #[inline(always)]
    pub const fn blkread(&self) -> super::vals::Blkread {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Blkread::from_bits(val as u8)
    }
    #[doc = "Block Read"]
    #[inline(always)]
    pub const fn set_blkread(&mut self, val: super::vals::Blkread) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Block Write"]
    #[must_use]
    #[inline(always)]
    pub const fn blkwrite(&self) -> super::vals::Blkwrite {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Blkwrite::from_bits(val as u8)
    }
    #[doc = "Block Write"]
    #[inline(always)]
    pub const fn set_blkwrite(&mut self, val: super::vals::Blkwrite) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Block Next Write Command"]
    #[must_use]
    #[inline(always)]
    pub const fn blknxtwr(&self) -> super::vals::Blknxtwr {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Blknxtwr::from_bits(val as u8)
    }
    #[doc = "Block Next Write Command"]
    #[inline(always)]
    pub const fn set_blknxtwr(&mut self, val: super::vals::Blknxtwr) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Block Next Read"]
    #[must_use]
    #[inline(always)]
    pub const fn blknxtrd(&self) -> super::vals::Blknxtrd {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Blknxtrd::from_bits(val as u8)
    }
    #[doc = "Block Next Read"]
    #[inline(always)]
    pub const fn set_blknxtrd(&mut self, val: super::vals::Blknxtrd) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Allow One More Write"]
    #[must_use]
    #[inline(always)]
    pub const fn allowonewr(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Allow One More Write"]
    #[inline(always)]
    pub const fn set_allowonewr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Allow One More Read"]
    #[must_use]
    #[inline(always)]
    pub const fn allowonerd(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Allow One More Read"]
    #[inline(always)]
    pub const fn set_allowonerd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "AXI Command Range Base Update"]
    #[must_use]
    #[inline(always)]
    pub const fn cmdrangebaseupdate(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "AXI Command Range Base Update"]
    #[inline(always)]
    pub const fn set_cmdrangebaseupdate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for ModuleControl {
    #[inline(always)]
    fn default() -> ModuleControl {
        ModuleControl(24u64 as u32)
    }
}
impl core::fmt::Debug for ModuleControl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ModuleControl")
            .field("swreset", &self.swreset())
            .field("iomode", &self.iomode())
            .field("dqsstop", &self.dqsstop())
            .field("csmask", &self.csmask())
            .field("blkread", &self.blkread())
            .field("blkwrite", &self.blkwrite())
            .field("blknxtwr", &self.blknxtwr())
            .field("blknxtrd", &self.blknxtrd())
            .field("allowonewr", &self.allowonewr())
            .field("allowonerd", &self.allowonerd())
            .field("cmdrangebaseupdate", &self.cmdrangebaseupdate())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ModuleControl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ModuleControl {
            swreset: super::vals::Swreset,
            iomode: super::vals::Iomode,
            dqsstop: bool,
            csmask: bool,
            blkread: super::vals::Blkread,
            blkwrite: super::vals::Blkwrite,
            blknxtwr: super::vals::Blknxtwr,
            blknxtrd: super::vals::Blknxtrd,
            allowonewr: bool,
            allowonerd: bool,
            cmdrangebaseupdate: bool,
        }
        let proxy = ModuleControl {
            swreset: self.swreset(),
            iomode: self.iomode(),
            dqsstop: self.dqsstop(),
            csmask: self.csmask(),
            blkread: self.blkread(),
            blkwrite: self.blkwrite(),
            blknxtwr: self.blknxtwr(),
            blknxtrd: self.blknxtrd(),
            allowonewr: self.allowonewr(),
            allowonerd: self.allowonerd(),
            cmdrangebaseupdate: self.cmdrangebaseupdate(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SPI FLR interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ModuleInt(pub u32);
impl ModuleInt {
    #[doc = "Write Overflow Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn wof(&self) -> super::vals::Wof {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Wof::from_bits(val as u8)
    }
    #[doc = "Write Overflow Interrupt"]
    #[inline(always)]
    pub const fn set_wof(&mut self, val: super::vals::Wof) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Read Underflow"]
    #[must_use]
    #[inline(always)]
    pub const fn ruf(&self) -> super::vals::Ruf {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ruf::from_bits(val as u8)
    }
    #[doc = "Read Underflow"]
    #[inline(always)]
    pub const fn set_ruf(&mut self, val: super::vals::Ruf) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Error Command"]
    #[must_use]
    #[inline(always)]
    pub const fn errcmd(&self) -> super::vals::Errcmd {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Errcmd::from_bits(val as u8)
    }
    #[doc = "Error Command"]
    #[inline(always)]
    pub const fn set_errcmd(&mut self, val: super::vals::Errcmd) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
}
impl Default for ModuleInt {
    #[inline(always)]
    fn default() -> ModuleInt {
        ModuleInt(0u64 as u32)
    }
}
impl core::fmt::Debug for ModuleInt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ModuleInt")
            .field("wof", &self.wof())
            .field("ruf", &self.ruf())
            .field("errcmd", &self.errcmd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ModuleInt {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ModuleInt {
            wof: super::vals::Wof,
            ruf: super::vals::Ruf,
            errcmd: super::vals::Errcmd,
        }
        let proxy = ModuleInt {
            wof: self.wof(),
            ruf: self.ruf(),
            errcmd: self.errcmd(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SPI FLR Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ModuleInten(pub u32);
impl ModuleInten {
    #[doc = "Write Overflow Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wofen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write Overflow Interrupt Enable"]
    #[inline(always)]
    pub const fn set_wofen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Read Underflow Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rufen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Read Underflow Interrupt Enable"]
    #[inline(always)]
    pub const fn set_rufen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Error Command Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn errcmden(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Error Command Interrupt Enable"]
    #[inline(always)]
    pub const fn set_errcmden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for ModuleInten {
    #[inline(always)]
    fn default() -> ModuleInten {
        ModuleInten(0u64 as u32)
    }
}
impl core::fmt::Debug for ModuleInten {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ModuleInten")
            .field("wofen", &self.wofen())
            .field("rufen", &self.rufen())
            .field("errcmden", &self.errcmden())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ModuleInten {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ModuleInten {
            wofen: bool,
            rufen: bool,
            errcmden: bool,
        }
        let proxy = ModuleInten {
            wofen: self.wofen(),
            rufen: self.rufen(),
            errcmden: self.errcmden(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Module Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ModuleStatus(pub u32);
impl ModuleStatus {
    #[doc = "Write in Progress"]
    #[must_use]
    #[inline(always)]
    pub const fn wip(&self) -> super::vals::Wip {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Wip::from_bits(val as u8)
    }
    #[doc = "Write in Progress"]
    #[inline(always)]
    pub const fn set_wip(&mut self, val: super::vals::Wip) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "AXI Read Leader Idle"]
    #[must_use]
    #[inline(always)]
    pub const fn axireadidle(&self) -> super::vals::Axireadidle {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Axireadidle::from_bits(val as u8)
    }
    #[doc = "AXI Read Leader Idle"]
    #[inline(always)]
    pub const fn set_axireadidle(&mut self, val: super::vals::Axireadidle) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Register Read Write Idle"]
    #[must_use]
    #[inline(always)]
    pub const fn regrwidle(&self) -> super::vals::Regrwidle {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Regrwidle::from_bits(val as u8)
    }
    #[doc = "Register Read Write Idle"]
    #[inline(always)]
    pub const fn set_regrwidle(&mut self, val: super::vals::Regrwidle) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "SEQ Controller Idle"]
    #[must_use]
    #[inline(always)]
    pub const fn seqidle(&self) -> super::vals::Seqidle {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Seqidle::from_bits(val as u8)
    }
    #[doc = "SEQ Controller Idle"]
    #[inline(always)]
    pub const fn set_seqidle(&mut self, val: super::vals::Seqidle) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Read Out-of-Range Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn rdofr(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Read Out-of-Range Counter"]
    #[inline(always)]
    pub const fn set_rdofr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Write Out-of-Range Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn wrofr(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Write Out-of-Range Counter"]
    #[inline(always)]
    pub const fn set_wrofr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Allow AXI Read Access"]
    #[must_use]
    #[inline(always)]
    pub const fn allowaxird(&self) -> super::vals::Allowaxird {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Allowaxird::from_bits(val as u8)
    }
    #[doc = "Allow AXI Read Access"]
    #[inline(always)]
    pub const fn set_allowaxird(&mut self, val: super::vals::Allowaxird) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Allow AXI Write Access"]
    #[must_use]
    #[inline(always)]
    pub const fn allowaxiwr(&self) -> super::vals::Allowaxiwr {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Allowaxiwr::from_bits(val as u8)
    }
    #[doc = "Allow AXI Write Access"]
    #[inline(always)]
    pub const fn set_allowaxiwr(&mut self, val: super::vals::Allowaxiwr) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
}
impl Default for ModuleStatus {
    #[inline(always)]
    fn default() -> ModuleStatus {
        ModuleStatus(12302u64 as u32)
    }
}
impl core::fmt::Debug for ModuleStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ModuleStatus")
            .field("wip", &self.wip())
            .field("axireadidle", &self.axireadidle())
            .field("regrwidle", &self.regrwidle())
            .field("seqidle", &self.seqidle())
            .field("rdofr", &self.rdofr())
            .field("wrofr", &self.wrofr())
            .field("allowaxird", &self.allowaxird())
            .field("allowaxiwr", &self.allowaxiwr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ModuleStatus {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ModuleStatus {
            wip: super::vals::Wip,
            axireadidle: super::vals::Axireadidle,
            regrwidle: super::vals::Regrwidle,
            seqidle: super::vals::Seqidle,
            rdofr: u8,
            wrofr: u8,
            allowaxird: super::vals::Allowaxird,
            allowaxiwr: super::vals::Allowaxiwr,
        }
        let proxy = ModuleStatus {
            wip: self.wip(),
            axireadidle: self.axireadidle(),
            regrwidle: self.regrwidle(),
            seqidle: self.seqidle(),
            rdofr: self.rdofr(),
            wrofr: self.wrofr(),
            allowaxird: self.allowaxird(),
            allowaxiwr: self.allowaxiwr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Read Command 1 setting"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ReadCommand1(pub u32);
impl ReadCommand1 {
    #[doc = "Read Command 1 Dummy Cycles"]
    #[must_use]
    #[inline(always)]
    pub const fn dummycycles(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Read Command 1 Dummy Cycles"]
    #[inline(always)]
    pub const fn set_dummycycles(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Read Command 1 Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn commandset(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Read Command 1 Setting"]
    #[inline(always)]
    pub const fn set_commandset(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for ReadCommand1 {
    #[inline(always)]
    fn default() -> ReadCommand1 {
        ReadCommand1(33685524u64 as u32)
    }
}
impl core::fmt::Debug for ReadCommand1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ReadCommand1")
            .field("dummycycles", &self.dummycycles())
            .field("commandset", &self.commandset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ReadCommand1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ReadCommand1 {
            dummycycles: u16,
            commandset: u16,
        }
        let proxy = ReadCommand1 {
            dummycycles: self.dummycycles(),
            commandset: self.commandset(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Read Command 2 setting"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ReadCommand2(pub u32);
impl ReadCommand2 {
    #[doc = "Read Command 2 Dummy Cycles"]
    #[must_use]
    #[inline(always)]
    pub const fn dummycycles(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Read Command 2 Dummy Cycles"]
    #[inline(always)]
    pub const fn set_dummycycles(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Read Command 2 Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn commandset(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Read Command 2 Setting"]
    #[inline(always)]
    pub const fn set_commandset(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for ReadCommand2 {
    #[inline(always)]
    fn default() -> ReadCommand2 {
        ReadCommand2(50528276u64 as u32)
    }
}
impl core::fmt::Debug for ReadCommand2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ReadCommand2")
            .field("dummycycles", &self.dummycycles())
            .field("commandset", &self.commandset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ReadCommand2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ReadCommand2 {
            dummycycles: u16,
            commandset: u16,
        }
        let proxy = ReadCommand2 {
            dummycycles: self.dummycycles(),
            commandset: self.commandset(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Read Command Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ReadCommandControl(pub u32);
impl ReadCommandControl {
    #[doc = "Read Fetch Size"]
    #[must_use]
    #[inline(always)]
    pub const fn rdfetchsize(&self) -> super::vals::Rdfetchsize {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Rdfetchsize::from_bits(val as u8)
    }
    #[doc = "Read Fetch Size"]
    #[inline(always)]
    pub const fn set_rdfetchsize(&mut self, val: super::vals::Rdfetchsize) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Read Watermark Level"]
    #[must_use]
    #[inline(always)]
    pub const fn rdwm(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[doc = "Read Watermark Level"]
    #[inline(always)]
    pub const fn set_rdwm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u32) & 0x3f) << 2usize);
    }
    #[doc = "Read Outstanding"]
    #[must_use]
    #[inline(always)]
    pub const fn rdot(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Read Outstanding"]
    #[inline(always)]
    pub const fn set_rdot(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Read Water Mark Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wmen(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Read Water Mark Enable"]
    #[inline(always)]
    pub const fn set_wmen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for ReadCommandControl {
    #[inline(always)]
    fn default() -> ReadCommandControl {
        ReadCommandControl(0u64 as u32)
    }
}
impl core::fmt::Debug for ReadCommandControl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ReadCommandControl")
            .field("rdfetchsize", &self.rdfetchsize())
            .field("rdwm", &self.rdwm())
            .field("rdot", &self.rdot())
            .field("wmen", &self.wmen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ReadCommandControl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ReadCommandControl {
            rdfetchsize: super::vals::Rdfetchsize,
            rdwm: u8,
            rdot: bool,
            wmen: bool,
        }
        let proxy = ReadCommandControl {
            rdfetchsize: self.rdfetchsize(),
            rdwm: self.rdwm(),
            rdot: self.rdot(),
            wmen: self.wmen(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Read Register Command Setting"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ReadRegisterCommand0(pub u32);
impl ReadRegisterCommand0 {
    #[doc = "Read Register Dummy Cycles"]
    #[must_use]
    #[inline(always)]
    pub const fn dummycycles(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Read Register Dummy Cycles"]
    #[inline(always)]
    pub const fn set_dummycycles(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Read Register Command Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn commandset(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Read Register Command Setting"]
    #[inline(always)]
    pub const fn set_commandset(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for ReadRegisterCommand0 {
    #[inline(always)]
    fn default() -> ReadRegisterCommand0 {
        ReadRegisterCommand0(16842772u64 as u32)
    }
}
impl core::fmt::Debug for ReadRegisterCommand0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ReadRegisterCommand0")
            .field("dummycycles", &self.dummycycles())
            .field("commandset", &self.commandset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ReadRegisterCommand0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ReadRegisterCommand0 {
            dummycycles: u16,
            commandset: u16,
        }
        let proxy = ReadRegisterCommand0 {
            dummycycles: self.dummycycles(),
            commandset: self.commandset(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Read Write Command Address Base"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RwCommandBase(pub u32);
impl RwCommandBase {
    #[doc = "Address Base 1"]
    #[must_use]
    #[inline(always)]
    pub const fn addrbase1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Address Base 1"]
    #[inline(always)]
    pub const fn set_addrbase1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Address Base 2"]
    #[must_use]
    #[inline(always)]
    pub const fn addrbase2(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Address Base 2"]
    #[inline(always)]
    pub const fn set_addrbase2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for RwCommandBase {
    #[inline(always)]
    fn default() -> RwCommandBase {
        RwCommandBase(268435456u64 as u32)
    }
}
impl core::fmt::Debug for RwCommandBase {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RwCommandBase")
            .field("addrbase1", &self.addrbase1())
            .field("addrbase2", &self.addrbase2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RwCommandBase {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct RwCommandBase {
            addrbase1: u16,
            addrbase2: u16,
        }
        let proxy = RwCommandBase {
            addrbase1: self.addrbase1(),
            addrbase2: self.addrbase2(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SPI Mailbox control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpiMailCtrl(pub u32);
impl SpiMailCtrl {
    #[doc = "Clear Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn clrint(&self) -> super::vals::Clrint {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Clrint::from_bits(val as u8)
    }
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub const fn set_clrint(&mut self, val: super::vals::Clrint) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "SPI Leader Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn spiinten(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SPI Leader Interrupt Enable"]
    #[inline(always)]
    pub const fn set_spiinten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for SpiMailCtrl {
    #[inline(always)]
    fn default() -> SpiMailCtrl {
        SpiMailCtrl(0u64 as u32)
    }
}
impl core::fmt::Debug for SpiMailCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SpiMailCtrl")
            .field("clrint", &self.clrint())
            .field("spiinten", &self.spiinten())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SpiMailCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SpiMailCtrl {
            clrint: super::vals::Clrint,
            spiinten: bool,
        }
        let proxy = SpiMailCtrl {
            clrint: self.clrint(),
            spiinten: self.spiinten(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Write Command 1 Setting"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WriteCommand1(pub u32);
impl WriteCommand1 {
    #[doc = "Write Command 1 Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn commandset(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Write Command 1 Setting"]
    #[inline(always)]
    pub const fn set_commandset(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for WriteCommand1 {
    #[inline(always)]
    fn default() -> WriteCommand1 {
        WriteCommand1(84213760u64 as u32)
    }
}
impl core::fmt::Debug for WriteCommand1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WriteCommand1")
            .field("commandset", &self.commandset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WriteCommand1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct WriteCommand1 {
            commandset: u16,
        }
        let proxy = WriteCommand1 {
            commandset: self.commandset(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Write Command 2 Setting"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WriteCommand2(pub u32);
impl WriteCommand2 {
    #[doc = "Write Command 2 Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn commandset(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Write Command 2 Setting"]
    #[inline(always)]
    pub const fn set_commandset(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for WriteCommand2 {
    #[inline(always)]
    fn default() -> WriteCommand2 {
        WriteCommand2(101056512u64 as u32)
    }
}
impl core::fmt::Debug for WriteCommand2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WriteCommand2")
            .field("commandset", &self.commandset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WriteCommand2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct WriteCommand2 {
            commandset: u16,
        }
        let proxy = WriteCommand2 {
            commandset: self.commandset(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Write Command Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WriteCommandControl(pub u32);
impl WriteCommandControl {
    #[doc = "Write Watermark"]
    #[must_use]
    #[inline(always)]
    pub const fn wrwm(&self) -> super::vals::Wrwm {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Wrwm::from_bits(val as u8)
    }
    #[doc = "Write Watermark"]
    #[inline(always)]
    pub const fn set_wrwm(&mut self, val: super::vals::Wrwm) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for WriteCommandControl {
    #[inline(always)]
    fn default() -> WriteCommandControl {
        WriteCommandControl(2u64 as u32)
    }
}
impl core::fmt::Debug for WriteCommandControl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WriteCommandControl")
            .field("wrwm", &self.wrwm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WriteCommandControl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct WriteCommandControl {
            wrwm: super::vals::Wrwm,
        }
        let proxy = WriteCommandControl { wrwm: self.wrwm() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Write Register Command 0 Setting"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WriteRegisterCommand0(pub u32);
impl WriteRegisterCommand0 {
    #[doc = "Write Register Command Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn commandset(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Write Register Command Setting"]
    #[inline(always)]
    pub const fn set_commandset(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for WriteRegisterCommand0 {
    #[inline(always)]
    fn default() -> WriteRegisterCommand0 {
        WriteRegisterCommand0(67371008u64 as u32)
    }
}
impl core::fmt::Debug for WriteRegisterCommand0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WriteRegisterCommand0")
            .field("commandset", &self.commandset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WriteRegisterCommand0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct WriteRegisterCommand0 {
            commandset: u16,
        }
        let proxy = WriteRegisterCommand0 {
            commandset: self.commandset(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
