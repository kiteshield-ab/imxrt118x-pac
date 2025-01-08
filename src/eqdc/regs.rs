#[doc = "Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u16);
impl Ctrl {
    #[doc = "Load Okay"]
    #[must_use]
    #[inline(always)]
    pub const fn ldok(&self) -> super::vals::Ldok {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ldok::from_bits(val as u8)
    }
    #[doc = "Load Okay"]
    #[inline(always)]
    pub const fn set_ldok(&mut self, val: super::vals::Ldok) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmaen(&self) -> super::vals::Dmaen {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Dmaen::from_bits(val as u8)
    }
    #[doc = "DMA Enable"]
    #[inline(always)]
    pub const fn set_dmaen(&mut self, val: super::vals::Dmaen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Watchdog Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wde(&self) -> super::vals::Wde {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Wde::from_bits(val as u8)
    }
    #[doc = "Watchdog Enable"]
    #[inline(always)]
    pub const fn set_wde(&mut self, val: super::vals::Wde) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Watchdog Timeout Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wdie(&self) -> super::vals::Wdie {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Wdie::from_bits(val as u8)
    }
    #[doc = "Watchdog Timeout Interrupt Enable"]
    #[inline(always)]
    pub const fn set_wdie(&mut self, val: super::vals::Wdie) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
    #[doc = "Watchdog Timeout Interrupt Request"]
    #[must_use]
    #[inline(always)]
    pub const fn wdirq(&self) -> super::vals::Wdirq {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Wdirq::from_bits(val as u8)
    }
    #[doc = "Watchdog Timeout Interrupt Request"]
    #[inline(always)]
    pub const fn set_wdirq(&mut self, val: super::vals::Wdirq) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Select Positive/Negative Edge of INDEX/PRESET Pulse"]
    #[must_use]
    #[inline(always)]
    pub const fn xne(&self) -> super::vals::Xne {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Xne::from_bits(val as u8)
    }
    #[doc = "Select Positive/Negative Edge of INDEX/PRESET Pulse"]
    #[inline(always)]
    pub const fn set_xne(&mut self, val: super::vals::Xne) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
    }
    #[doc = "INDEX Triggered Initialization of Position Counters UPOS and LPOS"]
    #[must_use]
    #[inline(always)]
    pub const fn xip(&self) -> super::vals::Xip {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Xip::from_bits(val as u8)
    }
    #[doc = "INDEX Triggered Initialization of Position Counters UPOS and LPOS"]
    #[inline(always)]
    pub const fn set_xip(&mut self, val: super::vals::Xip) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "INDEX/PRESET Pulse Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn xie(&self) -> super::vals::Xie {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Xie::from_bits(val as u8)
    }
    #[doc = "INDEX/PRESET Pulse Interrupt Enable"]
    #[inline(always)]
    pub const fn set_xie(&mut self, val: super::vals::Xie) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "INDEX/PRESET Pulse Interrupt Request"]
    #[must_use]
    #[inline(always)]
    pub const fn xirq(&self) -> super::vals::Xirq {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Xirq::from_bits(val as u8)
    }
    #[doc = "INDEX/PRESET Pulse Interrupt Request"]
    #[inline(always)]
    pub const fn set_xirq(&mut self, val: super::vals::Xirq) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "Enable Single Phase Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn ph1(&self) -> super::vals::Ph1 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Ph1::from_bits(val as u8)
    }
    #[doc = "Enable Single Phase Mode"]
    #[inline(always)]
    pub const fn set_ph1(&mut self, val: super::vals::Ph1) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
    }
    #[doc = "Enable Reverse Direction Counting"]
    #[must_use]
    #[inline(always)]
    pub const fn rev(&self) -> super::vals::Rev {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Rev::from_bits(val as u8)
    }
    #[doc = "Enable Reverse Direction Counting"]
    #[inline(always)]
    pub const fn set_rev(&mut self, val: super::vals::Rev) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u16) & 0x01) << 10usize);
    }
    #[doc = "Software-Triggered Initialization of Position Counters UPOS and LPOS"]
    #[must_use]
    #[inline(always)]
    pub const fn swip(&self) -> super::vals::Swip {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Swip::from_bits(val as u8)
    }
    #[doc = "Software-Triggered Initialization of Position Counters UPOS and LPOS"]
    #[inline(always)]
    pub const fn set_swip(&mut self, val: super::vals::Swip) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u16) & 0x01) << 11usize);
    }
    #[doc = "Use Negative Edge of HOME/ENABLE Input"]
    #[must_use]
    #[inline(always)]
    pub const fn hne(&self) -> super::vals::Hne {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Hne::from_bits(val as u8)
    }
    #[doc = "Use Negative Edge of HOME/ENABLE Input"]
    #[inline(always)]
    pub const fn set_hne(&mut self, val: super::vals::Hne) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
    #[doc = "Enable HOME to Initialize Position Counter UPOS/LPOS"]
    #[must_use]
    #[inline(always)]
    pub const fn hip(&self) -> super::vals::Hip {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Hip::from_bits(val as u8)
    }
    #[doc = "Enable HOME to Initialize Position Counter UPOS/LPOS"]
    #[inline(always)]
    pub const fn set_hip(&mut self, val: super::vals::Hip) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "HOME/ENABLE Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hie(&self) -> super::vals::Hie {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Hie::from_bits(val as u8)
    }
    #[doc = "HOME/ENABLE Interrupt Enable"]
    #[inline(always)]
    pub const fn set_hie(&mut self, val: super::vals::Hie) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "HOME/ENABLE Signal Transition Interrupt Request"]
    #[must_use]
    #[inline(always)]
    pub const fn hirq(&self) -> super::vals::Hirq {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Hirq::from_bits(val as u8)
    }
    #[doc = "HOME/ENABLE Signal Transition Interrupt Request"]
    #[inline(always)]
    pub const fn set_hirq(&mut self, val: super::vals::Hirq) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0u64 as u16)
    }
}
impl core::fmt::Debug for Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl")
            .field("ldok", &self.ldok())
            .field("dmaen", &self.dmaen())
            .field("wde", &self.wde())
            .field("wdie", &self.wdie())
            .field("wdirq", &self.wdirq())
            .field("xne", &self.xne())
            .field("xip", &self.xip())
            .field("xie", &self.xie())
            .field("xirq", &self.xirq())
            .field("ph1", &self.ph1())
            .field("rev", &self.rev())
            .field("swip", &self.swip())
            .field("hne", &self.hne())
            .field("hip", &self.hip())
            .field("hie", &self.hie())
            .field("hirq", &self.hirq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ctrl {
            ldok: super::vals::Ldok,
            dmaen: super::vals::Dmaen,
            wde: super::vals::Wde,
            wdie: super::vals::Wdie,
            wdirq: super::vals::Wdirq,
            xne: super::vals::Xne,
            xip: super::vals::Xip,
            xie: super::vals::Xie,
            xirq: super::vals::Xirq,
            ph1: super::vals::Ph1,
            rev: super::vals::Rev,
            swip: super::vals::Swip,
            hne: super::vals::Hne,
            hip: super::vals::Hip,
            hie: super::vals::Hie,
            hirq: super::vals::Hirq,
        }
        let proxy = Ctrl {
            ldok: self.ldok(),
            dmaen: self.dmaen(),
            wde: self.wde(),
            wdie: self.wdie(),
            wdirq: self.wdirq(),
            xne: self.xne(),
            xip: self.xip(),
            xie: self.xie(),
            xirq: self.xirq(),
            ph1: self.ph1(),
            rev: self.rev(),
            swip: self.swip(),
            hne: self.hne(),
            hip: self.hip(),
            hie: self.hie(),
            hirq: self.hirq(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Control 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl2(pub u16);
impl Ctrl2 {
    #[doc = "Update Hold Registers"]
    #[must_use]
    #[inline(always)]
    pub const fn updhld(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Update Hold Registers"]
    #[inline(always)]
    pub const fn set_updhld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Update Position Registers"]
    #[must_use]
    #[inline(always)]
    pub const fn updpos(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Update Position Registers"]
    #[inline(always)]
    pub const fn set_updpos(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Operation Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn opmode(&self) -> super::vals::Opmode {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Opmode::from_bits(val as u8)
    }
    #[doc = "Operation Mode Select"]
    #[inline(always)]
    pub const fn set_opmode(&mut self, val: super::vals::Opmode) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Buffered Register Load (Update) Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ldmod(&self) -> super::vals::Ldmod {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Ldmod::from_bits(val as u8)
    }
    #[doc = "Buffered Register Load (Update) Mode Select"]
    #[inline(always)]
    pub const fn set_ldmod(&mut self, val: super::vals::Ldmod) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
    #[doc = "Revolution Counter Modulus Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn revmod(&self) -> super::vals::Revmod {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Revmod::from_bits(val as u8)
    }
    #[doc = "Revolution Counter Modulus Enable"]
    #[inline(always)]
    pub const fn set_revmod(&mut self, val: super::vals::Revmod) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "Output Control"]
    #[must_use]
    #[inline(always)]
    pub const fn outctl(&self) -> super::vals::Outctl {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Outctl::from_bits(val as u8)
    }
    #[doc = "Output Control"]
    #[inline(always)]
    pub const fn set_outctl(&mut self, val: super::vals::Outctl) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
    }
    #[doc = "Period measurement function enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pmen(&self) -> super::vals::Pmen {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pmen::from_bits(val as u8)
    }
    #[doc = "Period measurement function enable"]
    #[inline(always)]
    pub const fn set_pmen(&mut self, val: super::vals::Pmen) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u16) & 0x01) << 10usize);
    }
    #[doc = "Initial Position Register"]
    #[must_use]
    #[inline(always)]
    pub const fn initpos(&self) -> super::vals::Initpos {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Initpos::from_bits(val as u8)
    }
    #[doc = "Initial Position Register"]
    #[inline(always)]
    pub const fn set_initpos(&mut self, val: super::vals::Initpos) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
    #[doc = "Count Once"]
    #[must_use]
    #[inline(always)]
    pub const fn once(&self) -> super::vals::Once {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Once::from_bits(val as u8)
    }
    #[doc = "Count Once"]
    #[inline(always)]
    pub const fn set_once(&mut self, val: super::vals::Once) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "Counting Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn cmode(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Counting Mode"]
    #[inline(always)]
    pub const fn set_cmode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u16) & 0x03) << 14usize);
    }
}
impl Default for Ctrl2 {
    #[inline(always)]
    fn default() -> Ctrl2 {
        Ctrl2(0u64 as u16)
    }
}
impl core::fmt::Debug for Ctrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl2")
            .field("updhld", &self.updhld())
            .field("updpos", &self.updpos())
            .field("opmode", &self.opmode())
            .field("ldmod", &self.ldmod())
            .field("revmod", &self.revmod())
            .field("outctl", &self.outctl())
            .field("pmen", &self.pmen())
            .field("initpos", &self.initpos())
            .field("once", &self.once())
            .field("cmode", &self.cmode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ctrl2 {
            updhld: bool,
            updpos: bool,
            opmode: super::vals::Opmode,
            ldmod: super::vals::Ldmod,
            revmod: super::vals::Revmod,
            outctl: super::vals::Outctl,
            pmen: super::vals::Pmen,
            initpos: super::vals::Initpos,
            once: super::vals::Once,
            cmode: u8,
        }
        let proxy = Ctrl2 {
            updhld: self.updhld(),
            updpos: self.updpos(),
            opmode: self.opmode(),
            ldmod: self.ldmod(),
            revmod: self.revmod(),
            outctl: self.outctl(),
            pmen: self.pmen(),
            initpos: self.initpos(),
            once: self.once(),
            cmode: self.cmode(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Input Filter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Filt(pub u16);
impl Filt {
    #[doc = "Input Filter Sample Period"]
    #[must_use]
    #[inline(always)]
    pub const fn filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Filter Sample Period"]
    #[inline(always)]
    pub const fn set_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Input Filter Sample Count"]
    #[must_use]
    #[inline(always)]
    pub const fn filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Filter Sample Count"]
    #[inline(always)]
    pub const fn set_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
    #[doc = "Filter Clock Source selection"]
    #[must_use]
    #[inline(always)]
    pub const fn filt_cs(&self) -> super::vals::FiltCs {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::FiltCs::from_bits(val as u8)
    }
    #[doc = "Filter Clock Source selection"]
    #[inline(always)]
    pub const fn set_filt_cs(&mut self, val: super::vals::FiltCs) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u16) & 0x01) << 11usize);
    }
    #[doc = "Prescaler"]
    #[must_use]
    #[inline(always)]
    pub const fn prsc(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Prescaler"]
    #[inline(always)]
    pub const fn set_prsc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u16) & 0x0f) << 12usize);
    }
}
impl Default for Filt {
    #[inline(always)]
    fn default() -> Filt {
        Filt(0u64 as u16)
    }
}
impl core::fmt::Debug for Filt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Filt")
            .field("filt_per", &self.filt_per())
            .field("filt_cnt", &self.filt_cnt())
            .field("filt_cs", &self.filt_cs())
            .field("prsc", &self.prsc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Filt {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Filt {
            filt_per: u8,
            filt_cnt: u8,
            filt_cs: super::vals::FiltCs,
            prsc: u8,
        }
        let proxy = Filt {
            filt_per: self.filt_per(),
            filt_cnt: self.filt_cnt(),
            filt_cs: self.filt_cs(),
            prsc: self.prsc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Input Monitor Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Imr(pub u16);
impl Imr {
    #[doc = "HOME_ENABLE"]
    #[must_use]
    #[inline(always)]
    pub const fn home_enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "HOME_ENABLE"]
    #[inline(always)]
    pub const fn set_home_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "INDEX_PRESET"]
    #[must_use]
    #[inline(always)]
    pub const fn index_preset(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "INDEX_PRESET"]
    #[inline(always)]
    pub const fn set_index_preset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "PHB"]
    #[must_use]
    #[inline(always)]
    pub const fn phb(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "PHB"]
    #[inline(always)]
    pub const fn set_phb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "PHA"]
    #[must_use]
    #[inline(always)]
    pub const fn pha(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "PHA"]
    #[inline(always)]
    pub const fn set_pha(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "filter operation on HOME/ENABLE input"]
    #[must_use]
    #[inline(always)]
    pub const fn fhom_ena(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "filter operation on HOME/ENABLE input"]
    #[inline(always)]
    pub const fn set_fhom_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "filter operation on INDEX/PRESET input"]
    #[must_use]
    #[inline(always)]
    pub const fn find_pre(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "filter operation on INDEX/PRESET input"]
    #[inline(always)]
    pub const fn set_find_pre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "filter operation on PHASEB input"]
    #[must_use]
    #[inline(always)]
    pub const fn fphb(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "filter operation on PHASEB input"]
    #[inline(always)]
    pub const fn set_fphb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "filter operation on PHASEA input"]
    #[must_use]
    #[inline(always)]
    pub const fn fpha(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "filter operation on PHASEA input"]
    #[inline(always)]
    pub const fn set_fpha(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Position Compare 0 Flag Output"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpf0(&self) -> super::vals::Cmpf0 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Cmpf0::from_bits(val as u8)
    }
    #[doc = "Position Compare 0 Flag Output"]
    #[inline(always)]
    pub const fn set_cmpf0(&mut self, val: super::vals::Cmpf0) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "Position Compare1 Flag Output"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp1f(&self) -> super::vals::Cmp1f {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Cmp1f::from_bits(val as u8)
    }
    #[doc = "Position Compare1 Flag Output"]
    #[inline(always)]
    pub const fn set_cmp1f(&mut self, val: super::vals::Cmp1f) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
    }
    #[doc = "Position Compare2 Flag Output"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp2f(&self) -> super::vals::Cmp2f {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Cmp2f::from_bits(val as u8)
    }
    #[doc = "Position Compare2 Flag Output"]
    #[inline(always)]
    pub const fn set_cmp2f(&mut self, val: super::vals::Cmp2f) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u16) & 0x01) << 10usize);
    }
    #[doc = "Position Compare3 Flag Output"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp3f(&self) -> super::vals::Cmp3f {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Cmp3f::from_bits(val as u8)
    }
    #[doc = "Position Compare3 Flag Output"]
    #[inline(always)]
    pub const fn set_cmp3f(&mut self, val: super::vals::Cmp3f) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u16) & 0x01) << 11usize);
    }
    #[doc = "Count Direction Flag Hold"]
    #[must_use]
    #[inline(always)]
    pub const fn dirh(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Count Direction Flag Hold"]
    #[inline(always)]
    pub const fn set_dirh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "Count Direction Flag Output"]
    #[must_use]
    #[inline(always)]
    pub const fn dir(&self) -> super::vals::Dir {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Dir::from_bits(val as u8)
    }
    #[doc = "Count Direction Flag Output"]
    #[inline(always)]
    pub const fn set_dir(&mut self, val: super::vals::Dir) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
    }
}
impl Default for Imr {
    #[inline(always)]
    fn default() -> Imr {
        Imr(0u64 as u16)
    }
}
impl core::fmt::Debug for Imr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Imr")
            .field("home_enable", &self.home_enable())
            .field("index_preset", &self.index_preset())
            .field("phb", &self.phb())
            .field("pha", &self.pha())
            .field("fhom_ena", &self.fhom_ena())
            .field("find_pre", &self.find_pre())
            .field("fphb", &self.fphb())
            .field("fpha", &self.fpha())
            .field("cmpf0", &self.cmpf0())
            .field("cmp1f", &self.cmp1f())
            .field("cmp2f", &self.cmp2f())
            .field("cmp3f", &self.cmp3f())
            .field("dirh", &self.dirh())
            .field("dir", &self.dir())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Imr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Imr {
            home_enable: bool,
            index_preset: bool,
            phb: bool,
            pha: bool,
            fhom_ena: bool,
            find_pre: bool,
            fphb: bool,
            fpha: bool,
            cmpf0: super::vals::Cmpf0,
            cmp1f: super::vals::Cmp1f,
            cmp2f: super::vals::Cmp2f,
            cmp3f: super::vals::Cmp3f,
            dirh: bool,
            dir: super::vals::Dir,
        }
        let proxy = Imr {
            home_enable: self.home_enable(),
            index_preset: self.index_preset(),
            phb: self.phb(),
            pha: self.pha(),
            fhom_ena: self.fhom_ena(),
            find_pre: self.find_pre(),
            fphb: self.fphb(),
            fpha: self.fpha(),
            cmpf0: self.cmpf0(),
            cmp1f: self.cmp1f(),
            cmp2f: self.cmp2f(),
            cmp3f: self.cmp3f(),
            dirh: self.dirh(),
            dir: self.dir(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intctrl(pub u16);
impl Intctrl {
    #[doc = "Simultaneous PHASEA and PHASEB Change Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sabie(&self) -> super::vals::Sabie {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sabie::from_bits(val as u8)
    }
    #[doc = "Simultaneous PHASEA and PHASEB Change Interrupt Enable"]
    #[inline(always)]
    pub const fn set_sabie(&mut self, val: super::vals::Sabie) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "Simultaneous PHASEA and PHASEB Change Interrupt Request"]
    #[must_use]
    #[inline(always)]
    pub const fn sabirq(&self) -> super::vals::Sabirq {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sabirq::from_bits(val as u8)
    }
    #[doc = "Simultaneous PHASEA and PHASEB Change Interrupt Request"]
    #[inline(always)]
    pub const fn set_sabirq(&mut self, val: super::vals::Sabirq) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Count direction change interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dirie(&self) -> super::vals::Dirie {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Dirie::from_bits(val as u8)
    }
    #[doc = "Count direction change interrupt enable"]
    #[inline(always)]
    pub const fn set_dirie(&mut self, val: super::vals::Dirie) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Count direction change interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn dirirq(&self) -> super::vals::Dirirq {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Dirirq::from_bits(val as u8)
    }
    #[doc = "Count direction change interrupt"]
    #[inline(always)]
    pub const fn set_dirirq(&mut self, val: super::vals::Dirirq) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
    #[doc = "Roll-under Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ruie(&self) -> super::vals::Ruie {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Ruie::from_bits(val as u8)
    }
    #[doc = "Roll-under Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ruie(&mut self, val: super::vals::Ruie) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Roll-under Interrupt Request"]
    #[must_use]
    #[inline(always)]
    pub const fn ruirq(&self) -> super::vals::Ruirq {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Ruirq::from_bits(val as u8)
    }
    #[doc = "Roll-under Interrupt Request"]
    #[inline(always)]
    pub const fn set_ruirq(&mut self, val: super::vals::Ruirq) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
    }
    #[doc = "Roll-over Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn roie(&self) -> super::vals::Roie {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Roie::from_bits(val as u8)
    }
    #[doc = "Roll-over Interrupt Enable"]
    #[inline(always)]
    pub const fn set_roie(&mut self, val: super::vals::Roie) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Roll-over Interrupt Request"]
    #[must_use]
    #[inline(always)]
    pub const fn roirq(&self) -> super::vals::Roirq {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Roirq::from_bits(val as u8)
    }
    #[doc = "Roll-over Interrupt Request"]
    #[inline(always)]
    pub const fn set_roirq(&mut self, val: super::vals::Roirq) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "Compare 0 Interrupt Request"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp0irq(&self) -> super::vals::Cmp0irq {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Cmp0irq::from_bits(val as u8)
    }
    #[doc = "Compare 0 Interrupt Request"]
    #[inline(always)]
    pub const fn set_cmp0irq(&mut self, val: super::vals::Cmp0irq) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
    }
    #[doc = "Compare1 Interrupt Request"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp1irq(&self) -> super::vals::Cmp1irq {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Cmp1irq::from_bits(val as u8)
    }
    #[doc = "Compare1 Interrupt Request"]
    #[inline(always)]
    pub const fn set_cmp1irq(&mut self, val: super::vals::Cmp1irq) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u16) & 0x01) << 11usize);
    }
    #[doc = "Compare2 Interrupt Request"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp2irq(&self) -> super::vals::Cmp2irq {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Cmp2irq::from_bits(val as u8)
    }
    #[doc = "Compare2 Interrupt Request"]
    #[inline(always)]
    pub const fn set_cmp2irq(&mut self, val: super::vals::Cmp2irq) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "Compare3 Interrupt Request"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp3irq(&self) -> super::vals::Cmp3irq {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Cmp3irq::from_bits(val as u8)
    }
    #[doc = "Compare3 Interrupt Request"]
    #[inline(always)]
    pub const fn set_cmp3irq(&mut self, val: super::vals::Cmp3irq) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
    }
}
impl Default for Intctrl {
    #[inline(always)]
    fn default() -> Intctrl {
        Intctrl(0u64 as u16)
    }
}
impl core::fmt::Debug for Intctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intctrl")
            .field("sabie", &self.sabie())
            .field("sabirq", &self.sabirq())
            .field("dirie", &self.dirie())
            .field("dirirq", &self.dirirq())
            .field("ruie", &self.ruie())
            .field("ruirq", &self.ruirq())
            .field("roie", &self.roie())
            .field("roirq", &self.roirq())
            .field("cmp0irq", &self.cmp0irq())
            .field("cmp1irq", &self.cmp1irq())
            .field("cmp2irq", &self.cmp2irq())
            .field("cmp3irq", &self.cmp3irq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intctrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Intctrl {
            sabie: super::vals::Sabie,
            sabirq: super::vals::Sabirq,
            dirie: super::vals::Dirie,
            dirirq: super::vals::Dirirq,
            ruie: super::vals::Ruie,
            ruirq: super::vals::Ruirq,
            roie: super::vals::Roie,
            roirq: super::vals::Roirq,
            cmp0irq: super::vals::Cmp0irq,
            cmp1irq: super::vals::Cmp1irq,
            cmp2irq: super::vals::Cmp2irq,
            cmp3irq: super::vals::Cmp3irq,
        }
        let proxy = Intctrl {
            sabie: self.sabie(),
            sabirq: self.sabirq(),
            dirie: self.dirie(),
            dirirq: self.dirirq(),
            ruie: self.ruie(),
            ruirq: self.ruirq(),
            roie: self.roie(),
            roirq: self.roirq(),
            cmp0irq: self.cmp0irq(),
            cmp1irq: self.cmp1irq(),
            cmp2irq: self.cmp2irq(),
            cmp3irq: self.cmp3irq(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Test Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tst(pub u16);
impl Tst {
    #[doc = "TEST_COUNT"]
    #[must_use]
    #[inline(always)]
    pub const fn test_count(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "TEST_COUNT"]
    #[inline(always)]
    pub const fn set_test_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "TEST_PERIOD"]
    #[must_use]
    #[inline(always)]
    pub const fn test_period(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "TEST_PERIOD"]
    #[inline(always)]
    pub const fn set_test_period(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u16) & 0x1f) << 8usize);
    }
    #[doc = "Quadrature Decoder Negative Signal"]
    #[must_use]
    #[inline(always)]
    pub const fn qdn(&self) -> super::vals::Qdn {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Qdn::from_bits(val as u8)
    }
    #[doc = "Quadrature Decoder Negative Signal"]
    #[inline(always)]
    pub const fn set_qdn(&mut self, val: super::vals::Qdn) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "Test Counter Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tce(&self) -> super::vals::Tce {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Tce::from_bits(val as u8)
    }
    #[doc = "Test Counter Enable"]
    #[inline(always)]
    pub const fn set_tce(&mut self, val: super::vals::Tce) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "Test Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ten(&self) -> super::vals::Ten {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Ten::from_bits(val as u8)
    }
    #[doc = "Test Mode Enable"]
    #[inline(always)]
    pub const fn set_ten(&mut self, val: super::vals::Ten) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
    }
}
impl Default for Tst {
    #[inline(always)]
    fn default() -> Tst {
        Tst(0u64 as u16)
    }
}
impl core::fmt::Debug for Tst {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tst")
            .field("test_count", &self.test_count())
            .field("test_period", &self.test_period())
            .field("qdn", &self.qdn())
            .field("tce", &self.tce())
            .field("ten", &self.ten())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tst {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Tst {
            test_count: u8,
            test_period: u8,
            qdn: super::vals::Qdn,
            tce: super::vals::Tce,
            ten: super::vals::Ten,
        }
        let proxy = Tst {
            test_count: self.test_count(),
            test_period: self.test_period(),
            qdn: self.qdn(),
            tce: self.tce(),
            ten: self.ten(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
