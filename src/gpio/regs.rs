#[doc = "Global Interrupt Control High Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gichr(pub u32);
impl Gichr {
    #[doc = "Global Interrupt Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn giwe(&self, n: usize) -> bool {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Global Interrupt Write Enable"]
    #[inline(always)]
    pub const fn set_giwe(&mut self, n: usize, val: bool) {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Global Interrupt Write Data"]
    #[must_use]
    #[inline(always)]
    pub const fn giwd(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Global Interrupt Write Data"]
    #[inline(always)]
    pub const fn set_giwd(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Gichr {
    #[inline(always)]
    fn default() -> Gichr {
        Gichr(0u64 as u32)
    }
}
impl core::fmt::Debug for Gichr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gichr")
            .field(
                "giwe",
                &[
                    self.giwe(0usize),
                    self.giwe(1usize),
                    self.giwe(2usize),
                    self.giwe(3usize),
                    self.giwe(4usize),
                    self.giwe(5usize),
                    self.giwe(6usize),
                    self.giwe(7usize),
                    self.giwe(8usize),
                    self.giwe(9usize),
                    self.giwe(10usize),
                    self.giwe(11usize),
                    self.giwe(12usize),
                    self.giwe(13usize),
                    self.giwe(14usize),
                    self.giwe(15usize),
                ],
            )
            .field("giwd", &self.giwd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gichr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gichr {
            giwe: [bool; 16usize],
            giwd: u16,
        }
        let proxy = Gichr {
            giwe: [
                self.giwe(0usize),
                self.giwe(1usize),
                self.giwe(2usize),
                self.giwe(3usize),
                self.giwe(4usize),
                self.giwe(5usize),
                self.giwe(6usize),
                self.giwe(7usize),
                self.giwe(8usize),
                self.giwe(9usize),
                self.giwe(10usize),
                self.giwe(11usize),
                self.giwe(12usize),
                self.giwe(13usize),
                self.giwe(14usize),
                self.giwe(15usize),
            ],
            giwd: self.giwd(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Global Interrupt Control Low Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Giclr(pub u32);
impl Giclr {
    #[doc = "Global Interrupt Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn giwe(&self, n: usize) -> bool {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Global Interrupt Write Enable"]
    #[inline(always)]
    pub const fn set_giwe(&mut self, n: usize, val: bool) {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Global Interrupt Write Data"]
    #[must_use]
    #[inline(always)]
    pub const fn giwd(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Global Interrupt Write Data"]
    #[inline(always)]
    pub const fn set_giwd(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Giclr {
    #[inline(always)]
    fn default() -> Giclr {
        Giclr(0u64 as u32)
    }
}
impl core::fmt::Debug for Giclr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Giclr")
            .field(
                "giwe",
                &[
                    self.giwe(0usize),
                    self.giwe(1usize),
                    self.giwe(2usize),
                    self.giwe(3usize),
                    self.giwe(4usize),
                    self.giwe(5usize),
                    self.giwe(6usize),
                    self.giwe(7usize),
                    self.giwe(8usize),
                    self.giwe(9usize),
                    self.giwe(10usize),
                    self.giwe(11usize),
                    self.giwe(12usize),
                    self.giwe(13usize),
                    self.giwe(14usize),
                    self.giwe(15usize),
                ],
            )
            .field("giwd", &self.giwd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Giclr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Giclr {
            giwe: [bool; 16usize],
            giwd: u16,
        }
        let proxy = Giclr {
            giwe: [
                self.giwe(0usize),
                self.giwe(1usize),
                self.giwe(2usize),
                self.giwe(3usize),
                self.giwe(4usize),
                self.giwe(5usize),
                self.giwe(6usize),
                self.giwe(7usize),
                self.giwe(8usize),
                self.giwe(9usize),
                self.giwe(10usize),
                self.giwe(11usize),
                self.giwe(12usize),
                self.giwe(13usize),
                self.giwe(14usize),
                self.giwe(15usize),
            ],
            giwd: self.giwd(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Control Non-Privilege"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icnp(pub u32);
impl Icnp {
    #[doc = "Non-Privilege Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn npe(&self, n: usize) -> bool {
        assert!(n < 2usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Non-Privilege Enable"]
    #[inline(always)]
    pub const fn set_npe(&mut self, n: usize, val: bool) {
        assert!(n < 2usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Icnp {
    #[inline(always)]
    fn default() -> Icnp {
        Icnp(0u64 as u32)
    }
}
impl core::fmt::Debug for Icnp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icnp")
            .field("npe", &[self.npe(0usize), self.npe(1usize)])
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icnp {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Icnp {
            npe: [bool; 2usize],
        }
        let proxy = Icnp {
            npe: [self.npe(0usize), self.npe(1usize)],
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Control Non-Secure"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icns(pub u32);
impl Icns {
    #[doc = "Non-Secure Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nse(&self, n: usize) -> bool {
        assert!(n < 2usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Non-Secure Enable"]
    #[inline(always)]
    pub const fn set_nse(&mut self, n: usize, val: bool) {
        assert!(n < 2usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Icns {
    #[inline(always)]
    fn default() -> Icns {
        Icns(0u64 as u32)
    }
}
impl core::fmt::Debug for Icns {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icns")
            .field("nse", &[self.nse(0usize), self.nse(1usize)])
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icns {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Icns {
            nse: [bool; 2usize],
        }
        let proxy = Icns {
            nse: [self.nse(0usize), self.nse(1usize)],
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Control Register %s"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr(pub u32);
impl Icr {
    #[doc = "Interrupt Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn irqc(&self) -> super::vals::Irqc {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Irqc::from_bits(val as u8)
    }
    #[doc = "Interrupt Configuration"]
    #[inline(always)]
    pub const fn set_irqc(&mut self, val: super::vals::Irqc) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Interrupt Select"]
    #[must_use]
    #[inline(always)]
    pub const fn irqs(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Select"]
    #[inline(always)]
    pub const fn set_irqs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Lock Register"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Lock Register"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Icr {
    #[inline(always)]
    fn default() -> Icr {
        Icr(0u64 as u32)
    }
}
impl core::fmt::Debug for Icr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr")
            .field("irqc", &self.irqc())
            .field("irqs", &self.irqs())
            .field("lk", &self.lk())
            .field("isf", &self.isf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Icr {
            irqc: super::vals::Irqc,
            irqs: bool,
            lk: bool,
            isf: bool,
        }
        let proxy = Icr {
            irqc: self.irqc(),
            irqs: self.irqs(),
            lk: self.lk(),
            isf: self.isf(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Status Flag Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isfr(pub u32);
impl Isfr {
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn isf(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_isf(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Isfr {
    #[inline(always)]
    fn default() -> Isfr {
        Isfr(0u64 as u32)
    }
}
impl core::fmt::Debug for Isfr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isfr")
            .field(
                "isf",
                &[
                    self.isf(0usize),
                    self.isf(1usize),
                    self.isf(2usize),
                    self.isf(3usize),
                    self.isf(4usize),
                    self.isf(5usize),
                    self.isf(6usize),
                    self.isf(7usize),
                    self.isf(8usize),
                    self.isf(9usize),
                    self.isf(10usize),
                    self.isf(11usize),
                    self.isf(12usize),
                    self.isf(13usize),
                    self.isf(14usize),
                    self.isf(15usize),
                    self.isf(16usize),
                    self.isf(17usize),
                    self.isf(18usize),
                    self.isf(19usize),
                    self.isf(20usize),
                    self.isf(21usize),
                    self.isf(22usize),
                    self.isf(23usize),
                    self.isf(24usize),
                    self.isf(25usize),
                    self.isf(26usize),
                    self.isf(27usize),
                    self.isf(28usize),
                    self.isf(29usize),
                    self.isf(30usize),
                    self.isf(31usize),
                ],
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isfr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Isfr {
            isf: [bool; 32usize],
        }
        let proxy = Isfr {
            isf: [
                self.isf(0usize),
                self.isf(1usize),
                self.isf(2usize),
                self.isf(3usize),
                self.isf(4usize),
                self.isf(5usize),
                self.isf(6usize),
                self.isf(7usize),
                self.isf(8usize),
                self.isf(9usize),
                self.isf(10usize),
                self.isf(11usize),
                self.isf(12usize),
                self.isf(13usize),
                self.isf(14usize),
                self.isf(15usize),
                self.isf(16usize),
                self.isf(17usize),
                self.isf(18usize),
                self.isf(19usize),
                self.isf(20usize),
                self.isf(21usize),
                self.isf(22usize),
                self.isf(23usize),
                self.isf(24usize),
                self.isf(25usize),
                self.isf(26usize),
                self.isf(27usize),
                self.isf(28usize),
                self.isf(29usize),
                self.isf(30usize),
                self.isf(31usize),
            ],
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Lock"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lock(pub u32);
impl Lock {
    #[doc = "Lock PCNS"]
    #[must_use]
    #[inline(always)]
    pub const fn pcns(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Lock PCNS"]
    #[inline(always)]
    pub const fn set_pcns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Lock ICNS"]
    #[must_use]
    #[inline(always)]
    pub const fn icns(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Lock ICNS"]
    #[inline(always)]
    pub const fn set_icns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Lock PCNP"]
    #[must_use]
    #[inline(always)]
    pub const fn pcnp(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Lock PCNP"]
    #[inline(always)]
    pub const fn set_pcnp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Lock ICNP"]
    #[must_use]
    #[inline(always)]
    pub const fn icnp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Lock ICNP"]
    #[inline(always)]
    pub const fn set_icnp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Lock {
    #[inline(always)]
    fn default() -> Lock {
        Lock(0u64 as u32)
    }
}
impl core::fmt::Debug for Lock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lock")
            .field("pcns", &self.pcns())
            .field("icns", &self.icns())
            .field("pcnp", &self.pcnp())
            .field("icnp", &self.icnp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lock {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lock {
            pcns: bool,
            icns: bool,
            pcnp: bool,
            icnp: bool,
        }
        let proxy = Lock {
            pcns: self.pcns(),
            icns: self.icns(),
            pcnp: self.pcnp(),
            icnp: self.icnp(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Parameter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Param(pub u32);
impl Param {
    #[doc = "Interrupt Number"]
    #[must_use]
    #[inline(always)]
    pub const fn irqnum(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Interrupt Number"]
    #[inline(always)]
    pub const fn set_irqnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Param {
    #[inline(always)]
    fn default() -> Param {
        Param(2u64 as u32)
    }
}
impl core::fmt::Debug for Param {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Param")
            .field("irqnum", &self.irqnum())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Param {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Param {
            irqnum: u8,
        }
        let proxy = Param {
            irqnum: self.irqnum(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Pin Control Non-Privilege"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcnp(pub u32);
impl Pcnp {
    #[doc = "Non-Privilege Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn npe(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Non-Privilege Enable"]
    #[inline(always)]
    pub const fn set_npe(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Pcnp {
    #[inline(always)]
    fn default() -> Pcnp {
        Pcnp(0u64 as u32)
    }
}
impl core::fmt::Debug for Pcnp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcnp")
            .field(
                "npe",
                &[
                    self.npe(0usize),
                    self.npe(1usize),
                    self.npe(2usize),
                    self.npe(3usize),
                    self.npe(4usize),
                    self.npe(5usize),
                    self.npe(6usize),
                    self.npe(7usize),
                    self.npe(8usize),
                    self.npe(9usize),
                    self.npe(10usize),
                    self.npe(11usize),
                    self.npe(12usize),
                    self.npe(13usize),
                    self.npe(14usize),
                    self.npe(15usize),
                    self.npe(16usize),
                    self.npe(17usize),
                    self.npe(18usize),
                    self.npe(19usize),
                    self.npe(20usize),
                    self.npe(21usize),
                    self.npe(22usize),
                    self.npe(23usize),
                    self.npe(24usize),
                    self.npe(25usize),
                    self.npe(26usize),
                    self.npe(27usize),
                    self.npe(28usize),
                    self.npe(29usize),
                    self.npe(30usize),
                    self.npe(31usize),
                ],
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcnp {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pcnp {
            npe: [bool; 32usize],
        }
        let proxy = Pcnp {
            npe: [
                self.npe(0usize),
                self.npe(1usize),
                self.npe(2usize),
                self.npe(3usize),
                self.npe(4usize),
                self.npe(5usize),
                self.npe(6usize),
                self.npe(7usize),
                self.npe(8usize),
                self.npe(9usize),
                self.npe(10usize),
                self.npe(11usize),
                self.npe(12usize),
                self.npe(13usize),
                self.npe(14usize),
                self.npe(15usize),
                self.npe(16usize),
                self.npe(17usize),
                self.npe(18usize),
                self.npe(19usize),
                self.npe(20usize),
                self.npe(21usize),
                self.npe(22usize),
                self.npe(23usize),
                self.npe(24usize),
                self.npe(25usize),
                self.npe(26usize),
                self.npe(27usize),
                self.npe(28usize),
                self.npe(29usize),
                self.npe(30usize),
                self.npe(31usize),
            ],
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Pin Control Non-Secure"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcns(pub u32);
impl Pcns {
    #[doc = "Non-Secure Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nse(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Non-Secure Enable"]
    #[inline(always)]
    pub const fn set_nse(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Pcns {
    #[inline(always)]
    fn default() -> Pcns {
        Pcns(0u64 as u32)
    }
}
impl core::fmt::Debug for Pcns {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcns")
            .field(
                "nse",
                &[
                    self.nse(0usize),
                    self.nse(1usize),
                    self.nse(2usize),
                    self.nse(3usize),
                    self.nse(4usize),
                    self.nse(5usize),
                    self.nse(6usize),
                    self.nse(7usize),
                    self.nse(8usize),
                    self.nse(9usize),
                    self.nse(10usize),
                    self.nse(11usize),
                    self.nse(12usize),
                    self.nse(13usize),
                    self.nse(14usize),
                    self.nse(15usize),
                    self.nse(16usize),
                    self.nse(17usize),
                    self.nse(18usize),
                    self.nse(19usize),
                    self.nse(20usize),
                    self.nse(21usize),
                    self.nse(22usize),
                    self.nse(23usize),
                    self.nse(24usize),
                    self.nse(25usize),
                    self.nse(26usize),
                    self.nse(27usize),
                    self.nse(28usize),
                    self.nse(29usize),
                    self.nse(30usize),
                    self.nse(31usize),
                ],
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcns {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pcns {
            nse: [bool; 32usize],
        }
        let proxy = Pcns {
            nse: [
                self.nse(0usize),
                self.nse(1usize),
                self.nse(2usize),
                self.nse(3usize),
                self.nse(4usize),
                self.nse(5usize),
                self.nse(6usize),
                self.nse(7usize),
                self.nse(8usize),
                self.nse(9usize),
                self.nse(10usize),
                self.nse(11usize),
                self.nse(12usize),
                self.nse(13usize),
                self.nse(14usize),
                self.nse(15usize),
                self.nse(16usize),
                self.nse(17usize),
                self.nse(18usize),
                self.nse(19usize),
                self.nse(20usize),
                self.nse(21usize),
                self.nse(22usize),
                self.nse(23usize),
                self.nse(24usize),
                self.nse(25usize),
                self.nse(26usize),
                self.nse(27usize),
                self.nse(28usize),
                self.nse(29usize),
                self.nse(30usize),
                self.nse(31usize),
            ],
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port Clear Output Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcor(pub u32);
impl Pcor {
    #[doc = "Port Clear Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptco(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Port Clear Output"]
    #[inline(always)]
    pub const fn set_ptco(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Pcor {
    #[inline(always)]
    fn default() -> Pcor {
        Pcor(0u64 as u32)
    }
}
impl core::fmt::Debug for Pcor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcor")
            .field(
                "ptco",
                &[
                    self.ptco(0usize),
                    self.ptco(1usize),
                    self.ptco(2usize),
                    self.ptco(3usize),
                    self.ptco(4usize),
                    self.ptco(5usize),
                    self.ptco(6usize),
                    self.ptco(7usize),
                    self.ptco(8usize),
                    self.ptco(9usize),
                    self.ptco(10usize),
                    self.ptco(11usize),
                    self.ptco(12usize),
                    self.ptco(13usize),
                    self.ptco(14usize),
                    self.ptco(15usize),
                    self.ptco(16usize),
                    self.ptco(17usize),
                    self.ptco(18usize),
                    self.ptco(19usize),
                    self.ptco(20usize),
                    self.ptco(21usize),
                    self.ptco(22usize),
                    self.ptco(23usize),
                    self.ptco(24usize),
                    self.ptco(25usize),
                    self.ptco(26usize),
                    self.ptco(27usize),
                    self.ptco(28usize),
                    self.ptco(29usize),
                    self.ptco(30usize),
                    self.ptco(31usize),
                ],
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcor {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pcor {
            ptco: [bool; 32usize],
        }
        let proxy = Pcor {
            ptco: [
                self.ptco(0usize),
                self.ptco(1usize),
                self.ptco(2usize),
                self.ptco(3usize),
                self.ptco(4usize),
                self.ptco(5usize),
                self.ptco(6usize),
                self.ptco(7usize),
                self.ptco(8usize),
                self.ptco(9usize),
                self.ptco(10usize),
                self.ptco(11usize),
                self.ptco(12usize),
                self.ptco(13usize),
                self.ptco(14usize),
                self.ptco(15usize),
                self.ptco(16usize),
                self.ptco(17usize),
                self.ptco(18usize),
                self.ptco(19usize),
                self.ptco(20usize),
                self.ptco(21usize),
                self.ptco(22usize),
                self.ptco(23usize),
                self.ptco(24usize),
                self.ptco(25usize),
                self.ptco(26usize),
                self.ptco(27usize),
                self.ptco(28usize),
                self.ptco(29usize),
                self.ptco(30usize),
                self.ptco(31usize),
            ],
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port Data Direction Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pddr(pub u32);
impl Pddr {
    #[doc = "Port Data Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pdd(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Port Data Direction"]
    #[inline(always)]
    pub const fn set_pdd(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Pddr {
    #[inline(always)]
    fn default() -> Pddr {
        Pddr(0u64 as u32)
    }
}
impl core::fmt::Debug for Pddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pddr")
            .field(
                "pdd",
                &[
                    self.pdd(0usize),
                    self.pdd(1usize),
                    self.pdd(2usize),
                    self.pdd(3usize),
                    self.pdd(4usize),
                    self.pdd(5usize),
                    self.pdd(6usize),
                    self.pdd(7usize),
                    self.pdd(8usize),
                    self.pdd(9usize),
                    self.pdd(10usize),
                    self.pdd(11usize),
                    self.pdd(12usize),
                    self.pdd(13usize),
                    self.pdd(14usize),
                    self.pdd(15usize),
                    self.pdd(16usize),
                    self.pdd(17usize),
                    self.pdd(18usize),
                    self.pdd(19usize),
                    self.pdd(20usize),
                    self.pdd(21usize),
                    self.pdd(22usize),
                    self.pdd(23usize),
                    self.pdd(24usize),
                    self.pdd(25usize),
                    self.pdd(26usize),
                    self.pdd(27usize),
                    self.pdd(28usize),
                    self.pdd(29usize),
                    self.pdd(30usize),
                    self.pdd(31usize),
                ],
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pddr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pddr {
            pdd: [bool; 32usize],
        }
        let proxy = Pddr {
            pdd: [
                self.pdd(0usize),
                self.pdd(1usize),
                self.pdd(2usize),
                self.pdd(3usize),
                self.pdd(4usize),
                self.pdd(5usize),
                self.pdd(6usize),
                self.pdd(7usize),
                self.pdd(8usize),
                self.pdd(9usize),
                self.pdd(10usize),
                self.pdd(11usize),
                self.pdd(12usize),
                self.pdd(13usize),
                self.pdd(14usize),
                self.pdd(15usize),
                self.pdd(16usize),
                self.pdd(17usize),
                self.pdd(18usize),
                self.pdd(19usize),
                self.pdd(20usize),
                self.pdd(21usize),
                self.pdd(22usize),
                self.pdd(23usize),
                self.pdd(24usize),
                self.pdd(25usize),
                self.pdd(26usize),
                self.pdd(27usize),
                self.pdd(28usize),
                self.pdd(29usize),
                self.pdd(30usize),
                self.pdd(31usize),
            ],
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port Data Input Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdir(pub u32);
impl Pdir {
    #[doc = "Port Data Input"]
    #[must_use]
    #[inline(always)]
    pub const fn pdi(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Port Data Input"]
    #[inline(always)]
    pub const fn set_pdi(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Pdir {
    #[inline(always)]
    fn default() -> Pdir {
        Pdir(0u64 as u32)
    }
}
impl core::fmt::Debug for Pdir {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pdir")
            .field(
                "pdi",
                &[
                    self.pdi(0usize),
                    self.pdi(1usize),
                    self.pdi(2usize),
                    self.pdi(3usize),
                    self.pdi(4usize),
                    self.pdi(5usize),
                    self.pdi(6usize),
                    self.pdi(7usize),
                    self.pdi(8usize),
                    self.pdi(9usize),
                    self.pdi(10usize),
                    self.pdi(11usize),
                    self.pdi(12usize),
                    self.pdi(13usize),
                    self.pdi(14usize),
                    self.pdi(15usize),
                    self.pdi(16usize),
                    self.pdi(17usize),
                    self.pdi(18usize),
                    self.pdi(19usize),
                    self.pdi(20usize),
                    self.pdi(21usize),
                    self.pdi(22usize),
                    self.pdi(23usize),
                    self.pdi(24usize),
                    self.pdi(25usize),
                    self.pdi(26usize),
                    self.pdi(27usize),
                    self.pdi(28usize),
                    self.pdi(29usize),
                    self.pdi(30usize),
                    self.pdi(31usize),
                ],
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pdir {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pdir {
            pdi: [bool; 32usize],
        }
        let proxy = Pdir {
            pdi: [
                self.pdi(0usize),
                self.pdi(1usize),
                self.pdi(2usize),
                self.pdi(3usize),
                self.pdi(4usize),
                self.pdi(5usize),
                self.pdi(6usize),
                self.pdi(7usize),
                self.pdi(8usize),
                self.pdi(9usize),
                self.pdi(10usize),
                self.pdi(11usize),
                self.pdi(12usize),
                self.pdi(13usize),
                self.pdi(14usize),
                self.pdi(15usize),
                self.pdi(16usize),
                self.pdi(17usize),
                self.pdi(18usize),
                self.pdi(19usize),
                self.pdi(20usize),
                self.pdi(21usize),
                self.pdi(22usize),
                self.pdi(23usize),
                self.pdi(24usize),
                self.pdi(25usize),
                self.pdi(26usize),
                self.pdi(27usize),
                self.pdi(28usize),
                self.pdi(29usize),
                self.pdi(30usize),
                self.pdi(31usize),
            ],
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port Data Output Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdor(pub u32);
impl Pdor {
    #[doc = "Port Data Output"]
    #[must_use]
    #[inline(always)]
    pub const fn pdo(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Port Data Output"]
    #[inline(always)]
    pub const fn set_pdo(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Pdor {
    #[inline(always)]
    fn default() -> Pdor {
        Pdor(0u64 as u32)
    }
}
impl core::fmt::Debug for Pdor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pdor")
            .field(
                "pdo",
                &[
                    self.pdo(0usize),
                    self.pdo(1usize),
                    self.pdo(2usize),
                    self.pdo(3usize),
                    self.pdo(4usize),
                    self.pdo(5usize),
                    self.pdo(6usize),
                    self.pdo(7usize),
                    self.pdo(8usize),
                    self.pdo(9usize),
                    self.pdo(10usize),
                    self.pdo(11usize),
                    self.pdo(12usize),
                    self.pdo(13usize),
                    self.pdo(14usize),
                    self.pdo(15usize),
                    self.pdo(16usize),
                    self.pdo(17usize),
                    self.pdo(18usize),
                    self.pdo(19usize),
                    self.pdo(20usize),
                    self.pdo(21usize),
                    self.pdo(22usize),
                    self.pdo(23usize),
                    self.pdo(24usize),
                    self.pdo(25usize),
                    self.pdo(26usize),
                    self.pdo(27usize),
                    self.pdo(28usize),
                    self.pdo(29usize),
                    self.pdo(30usize),
                    self.pdo(31usize),
                ],
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pdor {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pdor {
            pdo: [bool; 32usize],
        }
        let proxy = Pdor {
            pdo: [
                self.pdo(0usize),
                self.pdo(1usize),
                self.pdo(2usize),
                self.pdo(3usize),
                self.pdo(4usize),
                self.pdo(5usize),
                self.pdo(6usize),
                self.pdo(7usize),
                self.pdo(8usize),
                self.pdo(9usize),
                self.pdo(10usize),
                self.pdo(11usize),
                self.pdo(12usize),
                self.pdo(13usize),
                self.pdo(14usize),
                self.pdo(15usize),
                self.pdo(16usize),
                self.pdo(17usize),
                self.pdo(18usize),
                self.pdo(19usize),
                self.pdo(20usize),
                self.pdo(21usize),
                self.pdo(22usize),
                self.pdo(23usize),
                self.pdo(24usize),
                self.pdo(25usize),
                self.pdo(26usize),
                self.pdo(27usize),
                self.pdo(28usize),
                self.pdo(29usize),
                self.pdo(30usize),
                self.pdo(31usize),
            ],
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Pin Data Register a"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdr(pub u8);
impl Pdr {
    #[doc = "Pin Data (input and output)"]
    #[must_use]
    #[inline(always)]
    pub const fn pd(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Pin Data (input and output)"]
    #[inline(always)]
    pub const fn set_pd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for Pdr {
    #[inline(always)]
    fn default() -> Pdr {
        Pdr(0u64 as u8)
    }
}
impl core::fmt::Debug for Pdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pdr").field("pd", &self.pd()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pdr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pdr {
            pd: bool,
        }
        let proxy = Pdr { pd: self.pd() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port Input Disable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pidr(pub u32);
impl Pidr {
    #[doc = "Port Input Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn pid(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Port Input Disable"]
    #[inline(always)]
    pub const fn set_pid(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Pidr {
    #[inline(always)]
    fn default() -> Pidr {
        Pidr(0u64 as u32)
    }
}
impl core::fmt::Debug for Pidr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pidr")
            .field(
                "pid",
                &[
                    self.pid(0usize),
                    self.pid(1usize),
                    self.pid(2usize),
                    self.pid(3usize),
                    self.pid(4usize),
                    self.pid(5usize),
                    self.pid(6usize),
                    self.pid(7usize),
                    self.pid(8usize),
                    self.pid(9usize),
                    self.pid(10usize),
                    self.pid(11usize),
                    self.pid(12usize),
                    self.pid(13usize),
                    self.pid(14usize),
                    self.pid(15usize),
                    self.pid(16usize),
                    self.pid(17usize),
                    self.pid(18usize),
                    self.pid(19usize),
                    self.pid(20usize),
                    self.pid(21usize),
                    self.pid(22usize),
                    self.pid(23usize),
                    self.pid(24usize),
                    self.pid(25usize),
                    self.pid(26usize),
                    self.pid(27usize),
                    self.pid(28usize),
                    self.pid(29usize),
                    self.pid(30usize),
                    self.pid(31usize),
                ],
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pidr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pidr {
            pid: [bool; 32usize],
        }
        let proxy = Pidr {
            pid: [
                self.pid(0usize),
                self.pid(1usize),
                self.pid(2usize),
                self.pid(3usize),
                self.pid(4usize),
                self.pid(5usize),
                self.pid(6usize),
                self.pid(7usize),
                self.pid(8usize),
                self.pid(9usize),
                self.pid(10usize),
                self.pid(11usize),
                self.pid(12usize),
                self.pid(13usize),
                self.pid(14usize),
                self.pid(15usize),
                self.pid(16usize),
                self.pid(17usize),
                self.pid(18usize),
                self.pid(19usize),
                self.pid(20usize),
                self.pid(21usize),
                self.pid(22usize),
                self.pid(23usize),
                self.pid(24usize),
                self.pid(25usize),
                self.pid(26usize),
                self.pid(27usize),
                self.pid(28usize),
                self.pid(29usize),
                self.pid(30usize),
                self.pid(31usize),
            ],
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port Set Output Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psor(pub u32);
impl Psor {
    #[doc = "Port Set Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptso(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Port Set Output"]
    #[inline(always)]
    pub const fn set_ptso(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Psor {
    #[inline(always)]
    fn default() -> Psor {
        Psor(0u64 as u32)
    }
}
impl core::fmt::Debug for Psor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Psor")
            .field(
                "ptso",
                &[
                    self.ptso(0usize),
                    self.ptso(1usize),
                    self.ptso(2usize),
                    self.ptso(3usize),
                    self.ptso(4usize),
                    self.ptso(5usize),
                    self.ptso(6usize),
                    self.ptso(7usize),
                    self.ptso(8usize),
                    self.ptso(9usize),
                    self.ptso(10usize),
                    self.ptso(11usize),
                    self.ptso(12usize),
                    self.ptso(13usize),
                    self.ptso(14usize),
                    self.ptso(15usize),
                    self.ptso(16usize),
                    self.ptso(17usize),
                    self.ptso(18usize),
                    self.ptso(19usize),
                    self.ptso(20usize),
                    self.ptso(21usize),
                    self.ptso(22usize),
                    self.ptso(23usize),
                    self.ptso(24usize),
                    self.ptso(25usize),
                    self.ptso(26usize),
                    self.ptso(27usize),
                    self.ptso(28usize),
                    self.ptso(29usize),
                    self.ptso(30usize),
                    self.ptso(31usize),
                ],
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Psor {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Psor {
            ptso: [bool; 32usize],
        }
        let proxy = Psor {
            ptso: [
                self.ptso(0usize),
                self.ptso(1usize),
                self.ptso(2usize),
                self.ptso(3usize),
                self.ptso(4usize),
                self.ptso(5usize),
                self.ptso(6usize),
                self.ptso(7usize),
                self.ptso(8usize),
                self.ptso(9usize),
                self.ptso(10usize),
                self.ptso(11usize),
                self.ptso(12usize),
                self.ptso(13usize),
                self.ptso(14usize),
                self.ptso(15usize),
                self.ptso(16usize),
                self.ptso(17usize),
                self.ptso(18usize),
                self.ptso(19usize),
                self.ptso(20usize),
                self.ptso(21usize),
                self.ptso(22usize),
                self.ptso(23usize),
                self.ptso(24usize),
                self.ptso(25usize),
                self.ptso(26usize),
                self.ptso(27usize),
                self.ptso(28usize),
                self.ptso(29usize),
                self.ptso(30usize),
                self.ptso(31usize),
            ],
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port Toggle Output Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptor(pub u32);
impl Ptor {
    #[doc = "Port Toggle Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ptto(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Port Toggle Output"]
    #[inline(always)]
    pub const fn set_ptto(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Ptor {
    #[inline(always)]
    fn default() -> Ptor {
        Ptor(0u64 as u32)
    }
}
impl core::fmt::Debug for Ptor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ptor")
            .field(
                "ptto",
                &[
                    self.ptto(0usize),
                    self.ptto(1usize),
                    self.ptto(2usize),
                    self.ptto(3usize),
                    self.ptto(4usize),
                    self.ptto(5usize),
                    self.ptto(6usize),
                    self.ptto(7usize),
                    self.ptto(8usize),
                    self.ptto(9usize),
                    self.ptto(10usize),
                    self.ptto(11usize),
                    self.ptto(12usize),
                    self.ptto(13usize),
                    self.ptto(14usize),
                    self.ptto(15usize),
                    self.ptto(16usize),
                    self.ptto(17usize),
                    self.ptto(18usize),
                    self.ptto(19usize),
                    self.ptto(20usize),
                    self.ptto(21usize),
                    self.ptto(22usize),
                    self.ptto(23usize),
                    self.ptto(24usize),
                    self.ptto(25usize),
                    self.ptto(26usize),
                    self.ptto(27usize),
                    self.ptto(28usize),
                    self.ptto(29usize),
                    self.ptto(30usize),
                    self.ptto(31usize),
                ],
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ptor {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ptor {
            ptto: [bool; 32usize],
        }
        let proxy = Ptor {
            ptto: [
                self.ptto(0usize),
                self.ptto(1usize),
                self.ptto(2usize),
                self.ptto(3usize),
                self.ptto(4usize),
                self.ptto(5usize),
                self.ptto(6usize),
                self.ptto(7usize),
                self.ptto(8usize),
                self.ptto(9usize),
                self.ptto(10usize),
                self.ptto(11usize),
                self.ptto(12usize),
                self.ptto(13usize),
                self.ptto(14usize),
                self.ptto(15usize),
                self.ptto(16usize),
                self.ptto(17usize),
                self.ptto(18usize),
                self.ptto(19usize),
                self.ptto(20usize),
                self.ptto(21usize),
                self.ptto(22usize),
                self.ptto(23usize),
                self.ptto(24usize),
                self.ptto(25usize),
                self.ptto(26usize),
                self.ptto(27usize),
                self.ptto(28usize),
                self.ptto(29usize),
                self.ptto(30usize),
                self.ptto(31usize),
            ],
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Version ID"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "Feature Specification Number"]
    #[must_use]
    #[inline(always)]
    pub const fn feature(&self) -> super::vals::Feature {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Feature::from_bits(val as u16)
    }
    #[doc = "Feature Specification Number"]
    #[inline(always)]
    pub const fn set_feature(&mut self, val: super::vals::Feature) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
    #[doc = "Minor Version Number"]
    #[must_use]
    #[inline(always)]
    pub const fn minor(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Minor Version Number"]
    #[inline(always)]
    pub const fn set_minor(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Major Version Number"]
    #[must_use]
    #[inline(always)]
    pub const fn major(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Major Version Number"]
    #[inline(always)]
    pub const fn set_major(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Verid {
    #[inline(always)]
    fn default() -> Verid {
        Verid(33619969u64 as u32)
    }
}
impl core::fmt::Debug for Verid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Verid")
            .field("feature", &self.feature())
            .field("minor", &self.minor())
            .field("major", &self.major())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Verid {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Verid {
            feature: super::vals::Feature,
            minor: u8,
            major: u8,
        }
        let proxy = Verid {
            feature: self.feature(),
            minor: self.minor(),
            major: self.major(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
