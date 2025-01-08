#[doc = "Rx BDR a base address register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rbbar0(pub u32);
impl Rbbar0 {
    #[doc = "Lower address bits 31-7."]
    #[must_use]
    #[inline(always)]
    pub const fn addrl(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "Lower address bits 31-7."]
    #[inline(always)]
    pub const fn set_addrl(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for Rbbar0 {
    #[inline(always)]
    fn default() -> Rbbar0 {
        Rbbar0(0u64 as u32)
    }
}
impl core::fmt::Debug for Rbbar0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rbbar0")
            .field("addrl", &self.addrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rbbar0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Rbbar0 {
            addrl: u32,
        }
        let proxy = Rbbar0 {
            addrl: self.addrl(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Rx BDR a buffer size register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rbbsr(pub u32);
impl Rbbsr {
    #[doc = "Buffer size Indicates the buffer size for the buffer pool used"]
    #[must_use]
    #[inline(always)]
    pub const fn bsize(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Buffer size Indicates the buffer size for the buffer pool used"]
    #[inline(always)]
    pub const fn set_bsize(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Rbbsr {
    #[inline(always)]
    fn default() -> Rbbsr {
        Rbbsr(0u64 as u32)
    }
}
impl core::fmt::Debug for Rbbsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rbbsr")
            .field("bsize", &self.bsize())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rbbsr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Rbbsr {
            bsize: u16,
        }
        let proxy = Rbbsr {
            bsize: self.bsize(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Rx BDR a consumer index register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rbcir(pub u32);
impl Rbcir {
    #[doc = "Receive buffer descriptor ring consumer index. Range of index depends on ring size RBaLENR\\[LENGTH\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn bdr_index(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Receive buffer descriptor ring consumer index. Range of index depends on ring size RBaLENR\\[LENGTH\\]."]
    #[inline(always)]
    pub const fn set_bdr_index(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Rbcir {
    #[inline(always)]
    fn default() -> Rbcir {
        Rbcir(0u64 as u32)
    }
}
impl core::fmt::Debug for Rbcir {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rbcir")
            .field("bdr_index", &self.bdr_index())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rbcir {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Rbcir {
            bdr_index: u16,
        }
        let proxy = Rbcir {
            bdr_index: self.bdr_index(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Rx BDR a interrupt coalescing register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rbicr0(pub u32);
impl Rbicr0 {
    #[doc = "Interrupt coalescing packet threshold While interrupt coalescing is enabled, ICEN=1, this values determines the minimum number of packets received before raising an interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn icpt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Interrupt coalescing packet threshold While interrupt coalescing is enabled, ICEN=1, this values determines the minimum number of packets received before raising an interrupt"]
    #[inline(always)]
    pub const fn set_icpt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "Interrupt coalescing enable 0 Interrupt coalescing is disabled 1 Interrupt coalescing is enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn icen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt coalescing enable 0 Interrupt coalescing is disabled 1 Interrupt coalescing is enabled"]
    #[inline(always)]
    pub const fn set_icen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Rbicr0 {
    #[inline(always)]
    fn default() -> Rbicr0 {
        Rbicr0(0u64 as u32)
    }
}
impl core::fmt::Debug for Rbicr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rbicr0")
            .field("icpt", &self.icpt())
            .field("icen", &self.icen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rbicr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Rbicr0 {
            icpt: u16,
            icen: bool,
        }
        let proxy = Rbicr0 {
            icpt: self.icpt(),
            icen: self.icen(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Rx BDR a interrupt detect register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rbidr(pub u32);
impl Rbidr {
    #[doc = "Receive threshold 0 No threshold event detected 1 Receive ring holds at least the number of packets specified by RBaICR0\\[ICPT\\] or the threshold timer has expired since last received packet as specified by RBaICR1\\[ICTT\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn rxt(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Receive threshold 0 No threshold event detected 1 Receive ring holds at least the number of packets specified by RBaICR0\\[ICPT\\] or the threshold timer has expired since last received packet as specified by RBaICR1\\[ICTT\\]"]
    #[inline(always)]
    pub const fn set_rxt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Rbidr {
    #[inline(always)]
    fn default() -> Rbidr {
        Rbidr(0u64 as u32)
    }
}
impl core::fmt::Debug for Rbidr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rbidr").field("rxt", &self.rxt()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rbidr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Rbidr {
            rxt: bool,
        }
        let proxy = Rbidr { rxt: self.rxt() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Rx BDR a interrupt enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rbier(pub u32);
impl Rbier {
    #[doc = "Receive threshold interrupt enable 0 Disabled 1 Enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn rxtie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Receive threshold interrupt enable 0 Disabled 1 Enabled"]
    #[inline(always)]
    pub const fn set_rxtie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Rbier {
    #[inline(always)]
    fn default() -> Rbier {
        Rbier(0u64 as u32)
    }
}
impl core::fmt::Debug for Rbier {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rbier")
            .field("rxtie", &self.rxtie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rbier {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Rbier {
            rxtie: bool,
        }
        let proxy = Rbier {
            rxtie: self.rxtie(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Rx BDR a length register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rblenr(pub u32);
impl Rblenr {
    #[doc = "BD ring length Size of ring in sets of 8 BDs"]
    #[must_use]
    #[inline(always)]
    pub const fn length(&self) -> u16 {
        let val = (self.0 >> 3usize) & 0x3fff;
        val as u16
    }
    #[doc = "BD ring length Size of ring in sets of 8 BDs"]
    #[inline(always)]
    pub const fn set_length(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 3usize)) | (((val as u32) & 0x3fff) << 3usize);
    }
}
impl Default for Rblenr {
    #[inline(always)]
    fn default() -> Rblenr {
        Rblenr(0u64 as u32)
    }
}
impl core::fmt::Debug for Rblenr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rblenr")
            .field("length", &self.length())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rblenr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Rblenr {
            length: u16,
        }
        let proxy = Rblenr {
            length: self.length(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Rx BDR a mode register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rbmr(pub u32);
impl Rbmr {
    #[doc = "Header alignment If set, an additional 2-byte alignment is applied to the header in the 1st buffer descriptor"]
    #[must_use]
    #[inline(always)]
    pub const fn al(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Header alignment If set, an additional 2-byte alignment is applied to the header in the 1st buffer descriptor"]
    #[inline(always)]
    pub const fn set_al(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "BD Size A BD ring can use either the standard 16B or extended 32B buffer descriptor format"]
    #[must_use]
    #[inline(always)]
    pub const fn bds(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "BD Size A BD ring can use either the standard 16B or extended 32B buffer descriptor format"]
    #[inline(always)]
    pub const fn set_bds(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Congestion mode Determines the congestion scheme for the receive ring"]
    #[must_use]
    #[inline(always)]
    pub const fn cm(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Congestion mode Determines the congestion scheme for the receive ring"]
    #[inline(always)]
    pub const fn set_cm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "VLAN tag extract enable Controls whether the outer VLAN, as seen by the SI, is extracted"]
    #[must_use]
    #[inline(always)]
    pub const fn vte(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "VLAN tag extract enable Controls whether the outer VLAN, as seen by the SI, is extracted"]
    #[inline(always)]
    pub const fn set_vte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "VLAN tag presentation disable Controls whether the extracted VLAN tag is provided in the Rx BD"]
    #[must_use]
    #[inline(always)]
    pub const fn vtpd(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "VLAN tag presentation disable Controls whether the extracted VLAN tag is provided in the Rx BD"]
    #[inline(always)]
    pub const fn set_vtpd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Indicates if CRC32 (FCS) is to be stripped or preserved at the end of the frame"]
    #[must_use]
    #[inline(always)]
    pub const fn crc(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates if CRC32 (FCS) is to be stripped or preserved at the end of the frame"]
    #[inline(always)]
    pub const fn set_crc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Enable receive buffer descriptor ring 0 Disabled. 1 Enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable receive buffer descriptor ring 0 Disabled. 1 Enabled."]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Rbmr {
    #[inline(always)]
    fn default() -> Rbmr {
        Rbmr(0u64 as u32)
    }
}
impl core::fmt::Debug for Rbmr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rbmr")
            .field("al", &self.al())
            .field("bds", &self.bds())
            .field("cm", &self.cm())
            .field("vte", &self.vte())
            .field("vtpd", &self.vtpd())
            .field("crc", &self.crc())
            .field("en", &self.en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rbmr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Rbmr {
            al: bool,
            bds: bool,
            cm: bool,
            vte: bool,
            vtpd: bool,
            crc: bool,
            en: bool,
        }
        let proxy = Rbmr {
            al: self.al(),
            bds: self.bds(),
            cm: self.cm(),
            vte: self.vte(),
            vtpd: self.vtpd(),
            crc: self.crc(),
            en: self.en(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Rx BDR a producer index register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rbpir(pub u32);
impl Rbpir {
    #[doc = "Receive buffer descriptor ring producer index. Range of index depends on ring size RBaLENR\\[LENGTH\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn bdr_index(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Receive buffer descriptor ring producer index. Range of index depends on ring size RBaLENR\\[LENGTH\\]."]
    #[inline(always)]
    pub const fn set_bdr_index(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Rbpir {
    #[inline(always)]
    fn default() -> Rbpir {
        Rbpir(0u64 as u32)
    }
}
impl core::fmt::Debug for Rbpir {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rbpir")
            .field("bdr_index", &self.bdr_index())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rbpir {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Rbpir {
            bdr_index: u16,
        }
        let proxy = Rbpir {
            bdr_index: self.bdr_index(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Rx BDR a status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rbsr(pub u32);
impl Rbsr {
    #[doc = "Ring is empty Indicates the ring is empty of received BDs. Valid when RBaMR\\[EN\\]=1."]
    #[must_use]
    #[inline(always)]
    pub const fn empty(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Ring is empty Indicates the ring is empty of received BDs. Valid when RBaMR\\[EN\\]=1."]
    #[inline(always)]
    pub const fn set_empty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "System bus error A system bus error has occurred during one or more transactions related to this receive ring, including possibly the receive BD writeback entry itself"]
    #[must_use]
    #[inline(always)]
    pub const fn sbe(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "System bus error A system bus error has occurred during one or more transactions related to this receive ring, including possibly the receive BD writeback entry itself"]
    #[inline(always)]
    pub const fn set_sbe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Rbsr {
    #[inline(always)]
    fn default() -> Rbsr {
        Rbsr(0u64 as u32)
    }
}
impl core::fmt::Debug for Rbsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rbsr")
            .field("empty", &self.empty())
            .field("sbe", &self.sbe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rbsr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Rbsr {
            empty: bool,
            sbe: bool,
        }
        let proxy = Rbsr {
            empty: self.empty(),
            sbe: self.sbe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface buffer cache attribute register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sibcar(pub u32);
impl Sibcar {
    #[doc = "Buffer descriptor write cache type This is the cache attribute setting used when NETC writes the buffer descriptor to memory, which includes receive and transmit BD update"]
    #[must_use]
    #[inline(always)]
    pub const fn bd_wrcache(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Buffer descriptor write cache type This is the cache attribute setting used when NETC writes the buffer descriptor to memory, which includes receive and transmit BD update"]
    #[inline(always)]
    pub const fn set_bd_wrcache(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Buffer descriptor write domain This is the domain attribute setting used when NETC writes the buffer descriptor to memory, which includes receive and transmit BD update"]
    #[must_use]
    #[inline(always)]
    pub const fn bd_wrdomain(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Buffer descriptor write domain This is the domain attribute setting used when NETC writes the buffer descriptor to memory, which includes receive and transmit BD update"]
    #[inline(always)]
    pub const fn set_bd_wrdomain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Buffer descriptor write snoop See table above for valid settings."]
    #[must_use]
    #[inline(always)]
    pub const fn bd_wrsnp(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Buffer descriptor write snoop See table above for valid settings."]
    #[inline(always)]
    pub const fn set_bd_wrsnp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Write data cache type This is the cache attribute setting used when NETC writes frame data to memory on receive"]
    #[must_use]
    #[inline(always)]
    pub const fn wrcache(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Write data cache type This is the cache attribute setting used when NETC writes frame data to memory on receive"]
    #[inline(always)]
    pub const fn set_wrcache(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Write data domain This is the domain attribute setting used when NETC writes frame data to memory on receive"]
    #[must_use]
    #[inline(always)]
    pub const fn wrdomain(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Write data domain This is the domain attribute setting used when NETC writes frame data to memory on receive"]
    #[inline(always)]
    pub const fn set_wrdomain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Write data snoop See table above for valid settings."]
    #[must_use]
    #[inline(always)]
    pub const fn wrsnp(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Write data snoop See table above for valid settings."]
    #[inline(always)]
    pub const fn set_wrsnp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Buffer descriptor read cache type This is the cache attribute setting used when NETC read the buffer descriptor from memory for transmit"]
    #[must_use]
    #[inline(always)]
    pub const fn bd_rdcache(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Buffer descriptor read cache type This is the cache attribute setting used when NETC read the buffer descriptor from memory for transmit"]
    #[inline(always)]
    pub const fn set_bd_rdcache(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Buffer descriptor read domain This is the domain attribute setting used when NETC read the buffer descriptor from memory for transmit"]
    #[must_use]
    #[inline(always)]
    pub const fn bd_rddomain(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Buffer descriptor read domain This is the domain attribute setting used when NETC read the buffer descriptor from memory for transmit"]
    #[inline(always)]
    pub const fn set_bd_rddomain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "Buffer descriptor read snoop See table above for valid settings."]
    #[must_use]
    #[inline(always)]
    pub const fn bd_rdsnp(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Buffer descriptor read snoop See table above for valid settings."]
    #[inline(always)]
    pub const fn set_bd_rdsnp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Read data cache type This is the cache attribute setting used when NETC reads frame data from memory for transmit"]
    #[must_use]
    #[inline(always)]
    pub const fn rdcache(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Read data cache type This is the cache attribute setting used when NETC reads frame data from memory for transmit"]
    #[inline(always)]
    pub const fn set_rdcache(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Read data domain This is the domain attribute setting used when NETC reads frame data from memory for transmit"]
    #[must_use]
    #[inline(always)]
    pub const fn rddomain(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "Read data domain This is the domain attribute setting used when NETC reads frame data from memory for transmit"]
    #[inline(always)]
    pub const fn set_rddomain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "Read data snoop See table above for valid settings."]
    #[must_use]
    #[inline(always)]
    pub const fn rdsnp(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Read data snoop See table above for valid settings."]
    #[inline(always)]
    pub const fn set_rdsnp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Sibcar {
    #[inline(always)]
    fn default() -> Sibcar {
        Sibcar(33686018u64 as u32)
    }
}
impl core::fmt::Debug for Sibcar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sibcar")
            .field("bd_wrcache", &self.bd_wrcache())
            .field("bd_wrdomain", &self.bd_wrdomain())
            .field("bd_wrsnp", &self.bd_wrsnp())
            .field("wrcache", &self.wrcache())
            .field("wrdomain", &self.wrdomain())
            .field("wrsnp", &self.wrsnp())
            .field("bd_rdcache", &self.bd_rdcache())
            .field("bd_rddomain", &self.bd_rddomain())
            .field("bd_rdsnp", &self.bd_rdsnp())
            .field("rdcache", &self.rdcache())
            .field("rddomain", &self.rddomain())
            .field("rdsnp", &self.rdsnp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sibcar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sibcar {
            bd_wrcache: u8,
            bd_wrdomain: u8,
            bd_wrsnp: bool,
            wrcache: u8,
            wrdomain: u8,
            wrsnp: bool,
            bd_rdcache: u8,
            bd_rddomain: u8,
            bd_rdsnp: bool,
            rdcache: u8,
            rddomain: u8,
            rdsnp: bool,
        }
        let proxy = Sibcar {
            bd_wrcache: self.bd_wrcache(),
            bd_wrdomain: self.bd_wrdomain(),
            bd_wrsnp: self.bd_wrsnp(),
            wrcache: self.wrcache(),
            wrdomain: self.wrdomain(),
            wrsnp: self.wrsnp(),
            bd_rdcache: self.bd_rdcache(),
            bd_rddomain: self.bd_rddomain(),
            bd_rdsnp: self.bd_rdsnp(),
            rdcache: self.rdcache(),
            rddomain: self.rddomain(),
            rdsnp: self.rdsnp(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface capability register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sicapr0(pub u32);
impl Sicapr0 {
    #[doc = "Number of transmit buffer descriptor rings assigned to the SI"]
    #[must_use]
    #[inline(always)]
    pub const fn num_tx_bdr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Number of transmit buffer descriptor rings assigned to the SI"]
    #[inline(always)]
    pub const fn set_num_tx_bdr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Number of receive buffer descriptor rings assigned to the SI"]
    #[must_use]
    #[inline(always)]
    pub const fn num_rx_bdr(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Number of receive buffer descriptor rings assigned to the SI"]
    #[inline(always)]
    pub const fn set_num_rx_bdr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Number of MAC addresses Formula: NUM_MAC_ADDR+1"]
    #[must_use]
    #[inline(always)]
    pub const fn num_mac_addr(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of MAC addresses Formula: NUM_MAC_ADDR+1"]
    #[inline(always)]
    pub const fn set_num_mac_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Sicapr0 {
    #[inline(always)]
    fn default() -> Sicapr0 {
        Sicapr0(268697604u64 as u32)
    }
}
impl core::fmt::Debug for Sicapr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sicapr0")
            .field("num_tx_bdr", &self.num_tx_bdr())
            .field("num_rx_bdr", &self.num_rx_bdr())
            .field("num_mac_addr", &self.num_mac_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sicapr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sicapr0 {
            num_tx_bdr: u8,
            num_rx_bdr: u8,
            num_mac_addr: u8,
        }
        let proxy = Sicapr0 {
            num_tx_bdr: self.num_tx_bdr(),
            num_rx_bdr: self.num_rx_bdr(),
            num_mac_addr: self.num_mac_addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface capability register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sicapr1(pub u32);
impl Sicapr1 {
    #[doc = "Max number of receive buffer descriptor ring groups available to the SI"]
    #[must_use]
    #[inline(always)]
    pub const fn num_rx_grp(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Max number of receive buffer descriptor ring groups available to the SI"]
    #[inline(always)]
    pub const fn set_num_rx_grp(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Sicapr1 {
    #[inline(always)]
    fn default() -> Sicapr1 {
        Sicapr1(131072u64 as u32)
    }
}
impl core::fmt::Debug for Sicapr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sicapr1")
            .field("num_rx_grp", &self.num_rx_grp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sicapr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sicapr1 {
            num_rx_grp: u8,
        }
        let proxy = Sicapr1 {
            num_rx_grp: self.num_rx_grp(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface capability register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sicapr2(pub u32);
impl Sicapr2 {
    #[doc = "VLAN types permitted Bit: 0 Standard C-VLAN 0x8100 1 Standard S-VLAN 0x88A8 2 Custom VLAN as defined by CVLANR1\\[ETYPE\\] 3 Custom VLAN as defined by CVLANR2\\[ETYPE\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn vtp(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "VLAN types permitted Bit: 0 Standard C-VLAN 0x8100 1 Standard S-VLAN 0x88A8 2 Custom VLAN as defined by CVLANR1\\[ETYPE\\] 3 Custom VLAN as defined by CVLANR2\\[ETYPE\\]"]
    #[inline(always)]
    pub const fn set_vtp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Sicapr2 {
    #[inline(always)]
    fn default() -> Sicapr2 {
        Sicapr2(0u64 as u32)
    }
}
impl core::fmt::Debug for Sicapr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sicapr2").field("vtp", &self.vtp()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sicapr2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sicapr2 {
            vtp: u8,
        }
        let proxy = Sicapr2 { vtp: self.vtp() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface command BDR base address register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sicbdrbar0(pub u32);
impl Sicbdrbar0 {
    #[doc = "Lower address bits 31-7."]
    #[must_use]
    #[inline(always)]
    pub const fn addrl(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "Lower address bits 31-7."]
    #[inline(always)]
    pub const fn set_addrl(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for Sicbdrbar0 {
    #[inline(always)]
    fn default() -> Sicbdrbar0 {
        Sicbdrbar0(0u64 as u32)
    }
}
impl core::fmt::Debug for Sicbdrbar0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sicbdrbar0")
            .field("addrl", &self.addrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sicbdrbar0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sicbdrbar0 {
            addrl: u32,
        }
        let proxy = Sicbdrbar0 {
            addrl: self.addrl(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface command BDR consumer index register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sicbdrcir(pub u32);
impl Sicbdrcir {
    #[doc = "Command buffer descriptor ring consumer index"]
    #[must_use]
    #[inline(always)]
    pub const fn bdr_index(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Command buffer descriptor ring consumer index"]
    #[inline(always)]
    pub const fn set_bdr_index(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Sicbdrcir {
    #[inline(always)]
    fn default() -> Sicbdrcir {
        Sicbdrcir(0u64 as u32)
    }
}
impl core::fmt::Debug for Sicbdrcir {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sicbdrcir")
            .field("bdr_index", &self.bdr_index())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sicbdrcir {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sicbdrcir {
            bdr_index: u16,
        }
        let proxy = Sicbdrcir {
            bdr_index: self.bdr_index(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface command BDR interrupt detect register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sicbdridr(pub u32);
impl Sicbdridr {
    #[doc = "Command BD completed 0 No BD with CI=1 completed 1 Processed BD with CI=1"]
    #[must_use]
    #[inline(always)]
    pub const fn cbdc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Command BD completed 0 No BD with CI=1 completed 1 Processed BD with CI=1"]
    #[inline(always)]
    pub const fn set_cbdc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Sicbdridr {
    #[inline(always)]
    fn default() -> Sicbdridr {
        Sicbdridr(0u64 as u32)
    }
}
impl core::fmt::Debug for Sicbdridr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sicbdridr")
            .field("cbdc", &self.cbdc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sicbdridr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sicbdridr {
            cbdc: bool,
        }
        let proxy = Sicbdridr { cbdc: self.cbdc() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface command BDR interrupt enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sicbdrier(pub u32);
impl Sicbdrier {
    #[doc = "Command BD completion interrupt enable 0 Disabled 1 Enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn cbdcie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Command BD completion interrupt enable 0 Disabled 1 Enabled"]
    #[inline(always)]
    pub const fn set_cbdcie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Sicbdrier {
    #[inline(always)]
    fn default() -> Sicbdrier {
        Sicbdrier(0u64 as u32)
    }
}
impl core::fmt::Debug for Sicbdrier {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sicbdrier")
            .field("cbdcie", &self.cbdcie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sicbdrier {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sicbdrier {
            cbdcie: bool,
        }
        let proxy = Sicbdrier {
            cbdcie: self.cbdcie(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface command BDR length register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sicbdrlenr(pub u32);
impl Sicbdrlenr {
    #[doc = "BD ring length Size of ring in sets of 8 BDs. Maximum ring size is 1K."]
    #[must_use]
    #[inline(always)]
    pub const fn length(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0xff;
        val as u8
    }
    #[doc = "BD ring length Size of ring in sets of 8 BDs. Maximum ring size is 1K."]
    #[inline(always)]
    pub const fn set_length(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 3usize)) | (((val as u32) & 0xff) << 3usize);
    }
}
impl Default for Sicbdrlenr {
    #[inline(always)]
    fn default() -> Sicbdrlenr {
        Sicbdrlenr(0u64 as u32)
    }
}
impl core::fmt::Debug for Sicbdrlenr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sicbdrlenr")
            .field("length", &self.length())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sicbdrlenr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sicbdrlenr {
            length: u8,
        }
        let proxy = Sicbdrlenr {
            length: self.length(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface command BDR mode register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sicbdrmr(pub u32);
impl Sicbdrmr {
    #[doc = "Enable command buffer descriptor ring 0 Disabled"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable command buffer descriptor ring 0 Disabled"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Sicbdrmr {
    #[inline(always)]
    fn default() -> Sicbdrmr {
        Sicbdrmr(0u64 as u32)
    }
}
impl core::fmt::Debug for Sicbdrmr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sicbdrmr").field("en", &self.en()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sicbdrmr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sicbdrmr {
            en: bool,
        }
        let proxy = Sicbdrmr { en: self.en() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface command BDR producer index register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sicbdrpir(pub u32);
impl Sicbdrpir {
    #[doc = "Command buffer descriptor ring producer index"]
    #[must_use]
    #[inline(always)]
    pub const fn bdr_index(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Command buffer descriptor ring producer index"]
    #[inline(always)]
    pub const fn set_bdr_index(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Sicbdrpir {
    #[inline(always)]
    fn default() -> Sicbdrpir {
        Sicbdrpir(0u64 as u32)
    }
}
impl core::fmt::Debug for Sicbdrpir {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sicbdrpir")
            .field("bdr_index", &self.bdr_index())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sicbdrpir {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sicbdrpir {
            bdr_index: u16,
        }
        let proxy = Sicbdrpir {
            bdr_index: self.bdr_index(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface command BDR status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sicbdrsr(pub u32);
impl Sicbdrsr {
    #[doc = "Busy. The command BD ring is busy processing commands 0 Idle 1 Busy"]
    #[must_use]
    #[inline(always)]
    pub const fn busy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Busy. The command BD ring is busy processing commands 0 Idle 1 Busy"]
    #[inline(always)]
    pub const fn set_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Sicbdrsr {
    #[inline(always)]
    fn default() -> Sicbdrsr {
        Sicbdrsr(0u64 as u32)
    }
}
impl core::fmt::Debug for Sicbdrsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sicbdrsr")
            .field("busy", &self.busy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sicbdrsr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sicbdrsr {
            busy: bool,
        }
        let proxy = Sicbdrsr { busy: self.busy() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface command cache attribute register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Siccar(pub u32);
impl Siccar {
    #[doc = "Command buffer descriptor write cache type This is the cache attribute setting used when NETC writes the buffer descriptor to memory, which includes receive and transmit BD update"]
    #[must_use]
    #[inline(always)]
    pub const fn cbd_wrcache(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Command buffer descriptor write cache type This is the cache attribute setting used when NETC writes the buffer descriptor to memory, which includes receive and transmit BD update"]
    #[inline(always)]
    pub const fn set_cbd_wrcache(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Command buffer descriptor write domain This is the domain attribute setting used when NETC writes the buffer descriptor to memory, which includes receive and transmit BD update"]
    #[must_use]
    #[inline(always)]
    pub const fn cbd_wrdomain(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Command buffer descriptor write domain This is the domain attribute setting used when NETC writes the buffer descriptor to memory, which includes receive and transmit BD update"]
    #[inline(always)]
    pub const fn set_cbd_wrdomain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Command buffer descriptor write snoop See table above for valid settings."]
    #[must_use]
    #[inline(always)]
    pub const fn cbd_wrsnp(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Command buffer descriptor write snoop See table above for valid settings."]
    #[inline(always)]
    pub const fn set_cbd_wrsnp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Write data cache type This is the cache attribute setting used when NETC writes frame data to memory on receive"]
    #[must_use]
    #[inline(always)]
    pub const fn cwrcache(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Write data cache type This is the cache attribute setting used when NETC writes frame data to memory on receive"]
    #[inline(always)]
    pub const fn set_cwrcache(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Write data domain This is the domain attribute setting used when NETC writes frame data to memory on receive"]
    #[must_use]
    #[inline(always)]
    pub const fn cwrdomain(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Write data domain This is the domain attribute setting used when NETC writes frame data to memory on receive"]
    #[inline(always)]
    pub const fn set_cwrdomain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Write data snoop See table above for valid settings."]
    #[must_use]
    #[inline(always)]
    pub const fn cwrsnp(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Write data snoop See table above for valid settings."]
    #[inline(always)]
    pub const fn set_cwrsnp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Command buffer descriptor read cache type This is the cache attribute setting used when NETC read the buffer descriptor from memory for transmit"]
    #[must_use]
    #[inline(always)]
    pub const fn cbd_rdcache(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Command buffer descriptor read cache type This is the cache attribute setting used when NETC read the buffer descriptor from memory for transmit"]
    #[inline(always)]
    pub const fn set_cbd_rdcache(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Command buffer descriptor read domain This is the domain attribute setting used when NETC read the buffer descriptor from memory for transmit"]
    #[must_use]
    #[inline(always)]
    pub const fn cbd_rddomain(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Command buffer descriptor read domain This is the domain attribute setting used when NETC read the buffer descriptor from memory for transmit"]
    #[inline(always)]
    pub const fn set_cbd_rddomain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "Command buffer descriptor read snoop See table above for valid settings."]
    #[must_use]
    #[inline(always)]
    pub const fn cbd_rdsnp(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Command buffer descriptor read snoop See table above for valid settings."]
    #[inline(always)]
    pub const fn set_cbd_rdsnp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Read data cache type This is the cache attribute setting used when NETC reads frame data from memory for transmit"]
    #[must_use]
    #[inline(always)]
    pub const fn crdcache(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Read data cache type This is the cache attribute setting used when NETC reads frame data from memory for transmit"]
    #[inline(always)]
    pub const fn set_crdcache(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Read data domain This is the domain attribute setting used when NETC reads frame data from memory for transmit"]
    #[must_use]
    #[inline(always)]
    pub const fn crddomain(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "Read data domain This is the domain attribute setting used when NETC reads frame data from memory for transmit"]
    #[inline(always)]
    pub const fn set_crddomain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "Read data snoop See table above for valid settings."]
    #[must_use]
    #[inline(always)]
    pub const fn crdsnp(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Read data snoop See table above for valid settings."]
    #[inline(always)]
    pub const fn set_crdsnp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Siccar {
    #[inline(always)]
    fn default() -> Siccar {
        Siccar(33686018u64 as u32)
    }
}
impl core::fmt::Debug for Siccar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Siccar")
            .field("cbd_wrcache", &self.cbd_wrcache())
            .field("cbd_wrdomain", &self.cbd_wrdomain())
            .field("cbd_wrsnp", &self.cbd_wrsnp())
            .field("cwrcache", &self.cwrcache())
            .field("cwrdomain", &self.cwrdomain())
            .field("cwrsnp", &self.cwrsnp())
            .field("cbd_rdcache", &self.cbd_rdcache())
            .field("cbd_rddomain", &self.cbd_rddomain())
            .field("cbd_rdsnp", &self.cbd_rdsnp())
            .field("crdcache", &self.crdcache())
            .field("crddomain", &self.crddomain())
            .field("crdsnp", &self.crdsnp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Siccar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Siccar {
            cbd_wrcache: u8,
            cbd_wrdomain: u8,
            cbd_wrsnp: bool,
            cwrcache: u8,
            cwrdomain: u8,
            cwrsnp: bool,
            cbd_rdcache: u8,
            cbd_rddomain: u8,
            cbd_rdsnp: bool,
            crdcache: u8,
            crddomain: u8,
            crdsnp: bool,
        }
        let proxy = Siccar {
            cbd_wrcache: self.cbd_wrcache(),
            cbd_wrdomain: self.cbd_wrdomain(),
            cbd_wrsnp: self.cbd_wrsnp(),
            cwrcache: self.cwrcache(),
            cwrdomain: self.cwrdomain(),
            cwrsnp: self.cwrsnp(),
            cbd_rdcache: self.cbd_rdcache(),
            cbd_rddomain: self.cbd_rddomain(),
            cbd_rdsnp: self.cbd_rdsnp(),
            crdcache: self.crdcache(),
            crddomain: self.crddomain(),
            crdsnp: self.crdsnp(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface correctable memory error configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sicmecr(pub u32);
impl Sicmecr {
    #[doc = "Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn threshold(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Threshold"]
    #[inline(always)]
    pub const fn set_threshold(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Sicmecr {
    #[inline(always)]
    fn default() -> Sicmecr {
        Sicmecr(0u64 as u32)
    }
}
impl core::fmt::Debug for Sicmecr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sicmecr")
            .field("threshold", &self.threshold())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sicmecr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sicmecr {
            threshold: u8,
        }
        let proxy = Sicmecr {
            threshold: self.threshold(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface correctable memory error count register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sicmectr(pub u32);
impl Sicmectr {
    #[doc = "Count"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Count"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Sicmectr {
    #[inline(always)]
    fn default() -> Sicmectr {
        Sicmectr(0u64 as u32)
    }
}
impl core::fmt::Debug for Sicmectr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sicmectr")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sicmectr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sicmectr {
            count: u8,
        }
        let proxy = Sicmectr {
            count: self.count(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface correctable memory error status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sicmesr(pub u32);
impl Sicmesr {
    #[doc = "Memory ID"]
    #[must_use]
    #[inline(always)]
    pub const fn mem_id(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Memory ID"]
    #[inline(always)]
    pub const fn set_mem_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Single-bit ECC error"]
    #[must_use]
    #[inline(always)]
    pub const fn sbee(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Single-bit ECC error"]
    #[inline(always)]
    pub const fn set_sbee(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Sicmesr {
    #[inline(always)]
    fn default() -> Sicmesr {
        Sicmesr(0u64 as u32)
    }
}
impl core::fmt::Debug for Sicmesr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sicmesr")
            .field("mem_id", &self.mem_id())
            .field("sbee", &self.sbee())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sicmesr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sicmesr {
            mem_id: u8,
            sbee: bool,
        }
        let proxy = Sicmesr {
            mem_id: self.mem_id(),
            sbee: self.sbee(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface command MSI-X vector register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sicmsivr(pub u32);
impl Sicmsivr {
    #[doc = "Vector Index into MSI-X address/data table"]
    #[must_use]
    #[inline(always)]
    pub const fn vector(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Vector Index into MSI-X address/data table"]
    #[inline(always)]
    pub const fn set_vector(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for Sicmsivr {
    #[inline(always)]
    fn default() -> Sicmsivr {
        Sicmsivr(0u64 as u32)
    }
}
impl core::fmt::Debug for Sicmsivr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sicmsivr")
            .field("vector", &self.vector())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sicmsivr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sicmsivr {
            vector: u8,
        }
        let proxy = Sicmsivr {
            vector: self.vector(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface custom VLAN register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sicvlanr1(pub u32);
impl Sicvlanr1 {
    #[doc = "Ethertype"]
    #[must_use]
    #[inline(always)]
    pub const fn etype(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Ethertype"]
    #[inline(always)]
    pub const fn set_etype(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "0 not valid 1 valid"]
    #[must_use]
    #[inline(always)]
    pub const fn v(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "0 not valid 1 valid"]
    #[inline(always)]
    pub const fn set_v(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Sicvlanr1 {
    #[inline(always)]
    fn default() -> Sicvlanr1 {
        Sicvlanr1(0u64 as u32)
    }
}
impl core::fmt::Debug for Sicvlanr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sicvlanr1")
            .field("etype", &self.etype())
            .field("v", &self.v())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sicvlanr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sicvlanr1 {
            etype: u16,
            v: bool,
        }
        let proxy = Sicvlanr1 {
            etype: self.etype(),
            v: self.v(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface custom VLAN register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sicvlanr2(pub u32);
impl Sicvlanr2 {
    #[doc = "Ethertype"]
    #[must_use]
    #[inline(always)]
    pub const fn etype(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Ethertype"]
    #[inline(always)]
    pub const fn set_etype(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "0 not valid 1 valid"]
    #[must_use]
    #[inline(always)]
    pub const fn v(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "0 not valid 1 valid"]
    #[inline(always)]
    pub const fn set_v(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Sicvlanr2 {
    #[inline(always)]
    fn default() -> Sicvlanr2 {
        Sicvlanr2(0u64 as u32)
    }
}
impl core::fmt::Debug for Sicvlanr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sicvlanr2")
            .field("etype", &self.etype())
            .field("v", &self.v())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sicvlanr2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sicvlanr2 {
            etype: u16,
            v: bool,
        }
        let proxy = Sicvlanr2 {
            etype: self.etype(),
            v: self.v(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface IPV to ring mapping register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Siipvbdrmr0(pub u32);
impl Siipvbdrmr0 {
    #[doc = "BD ring used within the group for IPV 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ipv0bdr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "BD ring used within the group for IPV 0."]
    #[inline(always)]
    pub const fn set_ipv0bdr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "BD ring used within the group for IPV 1."]
    #[must_use]
    #[inline(always)]
    pub const fn ipv1bdr(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "BD ring used within the group for IPV 1."]
    #[inline(always)]
    pub const fn set_ipv1bdr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "BD ring used within the group for IPV 2."]
    #[must_use]
    #[inline(always)]
    pub const fn ipv2bdr(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "BD ring used within the group for IPV 2."]
    #[inline(always)]
    pub const fn set_ipv2bdr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "BD ring used within the group for IPV 3."]
    #[must_use]
    #[inline(always)]
    pub const fn ipv3bdr(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "BD ring used within the group for IPV 3."]
    #[inline(always)]
    pub const fn set_ipv3bdr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "BD ring used within the group for IPV 4."]
    #[must_use]
    #[inline(always)]
    pub const fn ipv4bdr(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "BD ring used within the group for IPV 4."]
    #[inline(always)]
    pub const fn set_ipv4bdr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "BD ring used within the group for IPV 5."]
    #[must_use]
    #[inline(always)]
    pub const fn ipv5bdr(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "BD ring used within the group for IPV 5."]
    #[inline(always)]
    pub const fn set_ipv5bdr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    #[doc = "BD ring used within the group for IPV 6."]
    #[must_use]
    #[inline(always)]
    pub const fn ipv6bdr(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "BD ring used within the group for IPV 6."]
    #[inline(always)]
    pub const fn set_ipv6bdr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "BD ring used within the group for IPV 7."]
    #[must_use]
    #[inline(always)]
    pub const fn ipv7bdr(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x07;
        val as u8
    }
    #[doc = "BD ring used within the group for IPV 7."]
    #[inline(always)]
    pub const fn set_ipv7bdr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
    }
}
impl Default for Siipvbdrmr0 {
    #[inline(always)]
    fn default() -> Siipvbdrmr0 {
        Siipvbdrmr0(0u64 as u32)
    }
}
impl core::fmt::Debug for Siipvbdrmr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Siipvbdrmr0")
            .field("ipv0bdr", &self.ipv0bdr())
            .field("ipv1bdr", &self.ipv1bdr())
            .field("ipv2bdr", &self.ipv2bdr())
            .field("ipv3bdr", &self.ipv3bdr())
            .field("ipv4bdr", &self.ipv4bdr())
            .field("ipv5bdr", &self.ipv5bdr())
            .field("ipv6bdr", &self.ipv6bdr())
            .field("ipv7bdr", &self.ipv7bdr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Siipvbdrmr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Siipvbdrmr0 {
            ipv0bdr: u8,
            ipv1bdr: u8,
            ipv2bdr: u8,
            ipv3bdr: u8,
            ipv4bdr: u8,
            ipv5bdr: u8,
            ipv6bdr: u8,
            ipv7bdr: u8,
        }
        let proxy = Siipvbdrmr0 {
            ipv0bdr: self.ipv0bdr(),
            ipv1bdr: self.ipv1bdr(),
            ipv2bdr: self.ipv2bdr(),
            ipv3bdr: self.ipv3bdr(),
            ipv4bdr: self.ipv4bdr(),
            ipv5bdr: self.ipv5bdr(),
            ipv6bdr: self.ipv6bdr(),
            ipv7bdr: self.ipv7bdr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface MAC address filter table capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Simaftcapr(pub u32);
impl Simaftcapr {
    #[doc = "Number of MAC address filter table entries for use by SI"]
    #[must_use]
    #[inline(always)]
    pub const fn num_mac_afte(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Number of MAC address filter table entries for use by SI"]
    #[inline(always)]
    pub const fn set_num_mac_afte(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Simaftcapr {
    #[inline(always)]
    fn default() -> Simaftcapr {
        Simaftcapr(4u64 as u32)
    }
}
impl core::fmt::Debug for Simaftcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Simaftcapr")
            .field("num_mac_afte", &self.num_mac_afte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Simaftcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Simaftcapr {
            num_mac_afte: u8,
        }
        let proxy = Simaftcapr {
            num_mac_afte: self.num_mac_afte(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface mode register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Simr(pub u32);
impl Simr {
    #[doc = "RSS classification enable 0 RSS classification is disabled 1 RSS classification is enabled This field is valid if SIPCAPR0\\[RSS\\] is set to 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rsse(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RSS classification enable 0 RSS classification is disabled 1 RSS classification is enabled This field is valid if SIPCAPR0\\[RSS\\] is set to 1"]
    #[inline(always)]
    pub const fn set_rsse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive no unicast-cast mode Suppresses uni-cast receive for the SI"]
    #[must_use]
    #[inline(always)]
    pub const fn rnum(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive no unicast-cast mode Suppresses uni-cast receive for the SI"]
    #[inline(always)]
    pub const fn set_rnum(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Receive no multi-cast mode Suppresses multi-cast receive for the SI"]
    #[must_use]
    #[inline(always)]
    pub const fn rnmm(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Receive no multi-cast mode Suppresses multi-cast receive for the SI"]
    #[inline(always)]
    pub const fn set_rnmm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Receive no broad-cast mode Suppresses broad-cast receive for the SI"]
    #[must_use]
    #[inline(always)]
    pub const fn rnbm(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Receive no broad-cast mode Suppresses broad-cast receive for the SI"]
    #[inline(always)]
    pub const fn set_rnbm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "VLAN to IPV mapping enable Maps the VLAN PCP/DEI to internal priority value (IPV)"]
    #[must_use]
    #[inline(always)]
    pub const fn v2ipve(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "VLAN to IPV mapping enable Maps the VLAN PCP/DEI to internal priority value (IPV)"]
    #[inline(always)]
    pub const fn set_v2ipve(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Default receive group The default receive group is used when there is no match for RFS and RSS is disabled"]
    #[must_use]
    #[inline(always)]
    pub const fn default_rx_group(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Default receive group The default receive group is used when there is no match for RFS and RSS is disabled"]
    #[inline(always)]
    pub const fn set_default_rx_group(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Enable station interface"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable station interface"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Simr {
    #[inline(always)]
    fn default() -> Simr {
        Simr(2147483648u64 as u32)
    }
}
impl core::fmt::Debug for Simr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Simr")
            .field("rsse", &self.rsse())
            .field("rnum", &self.rnum())
            .field("rnmm", &self.rnmm())
            .field("rnbm", &self.rnbm())
            .field("v2ipve", &self.v2ipve())
            .field("default_rx_group", &self.default_rx_group())
            .field("en", &self.en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Simr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Simr {
            rsse: bool,
            rnum: bool,
            rnmm: bool,
            rnbm: bool,
            v2ipve: bool,
            default_rx_group: bool,
            en: bool,
        }
        let proxy = Simr {
            rsse: self.rsse(),
            rnum: self.rnum(),
            rnmm: self.rnmm(),
            rnbm: self.rnbm(),
            v2ipve: self.v2ipve(),
            default_rx_group: self.default_rx_group(),
            en: self.en(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface MSI-X receive ring a vector register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Simsirrvr(pub u32);
impl Simsirrvr {
    #[doc = "Vector Index into MSI-X address/data table"]
    #[must_use]
    #[inline(always)]
    pub const fn vector(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Vector Index into MSI-X address/data table"]
    #[inline(always)]
    pub const fn set_vector(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for Simsirrvr {
    #[inline(always)]
    fn default() -> Simsirrvr {
        Simsirrvr(0u64 as u32)
    }
}
impl core::fmt::Debug for Simsirrvr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Simsirrvr")
            .field("vector", &self.vector())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Simsirrvr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Simsirrvr {
            vector: u8,
        }
        let proxy = Simsirrvr {
            vector: self.vector(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface MSI-X transmit ring a vector register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Simsitrvr(pub u32);
impl Simsitrvr {
    #[doc = "Vector Index into MSI-X address/data table"]
    #[must_use]
    #[inline(always)]
    pub const fn vector(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Vector Index into MSI-X address/data table"]
    #[inline(always)]
    pub const fn set_vector(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for Simsitrvr {
    #[inline(always)]
    fn default() -> Simsitrvr {
        Simsitrvr(0u64 as u32)
    }
}
impl core::fmt::Debug for Simsitrvr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Simsitrvr")
            .field("vector", &self.vector())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Simsitrvr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Simsitrvr {
            vector: u8,
        }
        let proxy = Simsitrvr {
            vector: self.vector(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface port capability register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sipcapr0(pub u32);
impl Sipcapr0 {
    #[doc = "Receive Flow Steering"]
    #[must_use]
    #[inline(always)]
    pub const fn rfs(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Flow Steering"]
    #[inline(always)]
    pub const fn set_rfs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Frame Preemption"]
    #[must_use]
    #[inline(always)]
    pub const fn fp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Frame Preemption"]
    #[inline(always)]
    pub const fn set_fp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Time Gate Scheduling"]
    #[must_use]
    #[inline(always)]
    pub const fn tgs(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Time Gate Scheduling"]
    #[inline(always)]
    pub const fn set_tgs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Time Specific Departure"]
    #[must_use]
    #[inline(always)]
    pub const fn tsd(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Time Specific Departure"]
    #[inline(always)]
    pub const fn set_tsd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Credit Based Shaping (CBS)"]
    #[must_use]
    #[inline(always)]
    pub const fn cbs(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Credit Based Shaping (CBS)"]
    #[inline(always)]
    pub const fn set_cbs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Receive Side Scaling"]
    #[must_use]
    #[inline(always)]
    pub const fn rss(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Side Scaling"]
    #[inline(always)]
    pub const fn set_rss(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Per-Stream Filtering and Policing (IEEE 802.1Qci)"]
    #[must_use]
    #[inline(always)]
    pub const fn psfp(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Per-Stream Filtering and Policing (IEEE 802.1Qci)"]
    #[inline(always)]
    pub const fn set_psfp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Ingress Port Filtering"]
    #[must_use]
    #[inline(always)]
    pub const fn ipflt(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Ingress Port Filtering"]
    #[inline(always)]
    pub const fn set_ipflt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Rate Policing"]
    #[must_use]
    #[inline(always)]
    pub const fn rp(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Rate Policing"]
    #[inline(always)]
    pub const fn set_rp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Wake-on-LAN"]
    #[must_use]
    #[inline(always)]
    pub const fn wo(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-on-LAN"]
    #[inline(always)]
    pub const fn set_wo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Functional Safety"]
    #[must_use]
    #[inline(always)]
    pub const fn fs(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Functional Safety"]
    #[inline(always)]
    pub const fn set_fs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Sipcapr0 {
    #[inline(always)]
    fn default() -> Sipcapr0 {
        Sipcapr0(11896u64 as u32)
    }
}
impl core::fmt::Debug for Sipcapr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sipcapr0")
            .field("rfs", &self.rfs())
            .field("fp", &self.fp())
            .field("tgs", &self.tgs())
            .field("tsd", &self.tsd())
            .field("cbs", &self.cbs())
            .field("rss", &self.rss())
            .field("psfp", &self.psfp())
            .field("ipflt", &self.ipflt())
            .field("rp", &self.rp())
            .field("wo", &self.wo())
            .field("fs", &self.fs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sipcapr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sipcapr0 {
            rfs: bool,
            fp: bool,
            tgs: bool,
            tsd: bool,
            cbs: bool,
            rss: bool,
            psfp: bool,
            ipflt: bool,
            rp: bool,
            wo: bool,
            fs: bool,
        }
        let proxy = Sipcapr0 {
            rfs: self.rfs(),
            fp: self.fp(),
            tgs: self.tgs(),
            tsd: self.tsd(),
            cbs: self.cbs(),
            rss: self.rss(),
            psfp: self.psfp(),
            ipflt: self.ipflt(),
            rp: self.rp(),
            wo: self.wo(),
            fs: self.fs(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface port capability register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sipcapr1(pub u32);
impl Sipcapr1 {
    #[doc = "Number of traffic classes 0 - 1 Traffic class (0) 1 - 2 Traffic classes (0-1)"]
    #[must_use]
    #[inline(always)]
    pub const fn num_tcs(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Number of traffic classes 0 - 1 Traffic class (0) 1 - 2 Traffic classes (0-1)"]
    #[inline(always)]
    pub const fn set_num_tcs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Number of multicast hash entries per SI 00 64 multicast addresses 01 128 multicast addresses 10 256 multicast addresses 11 512 multicast addresses"]
    #[must_use]
    #[inline(always)]
    pub const fn num_mch(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Number of multicast hash entries per SI 00 64 multicast addresses 01 128 multicast addresses 10 256 multicast addresses 11 512 multicast addresses"]
    #[inline(always)]
    pub const fn set_num_mch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Number of unicast hash entries (1 bit per entry) per SI 00 64 unicast addresses 01 128 unicast addresses 10 256 unicast addresses 11 512 unicast addresses"]
    #[must_use]
    #[inline(always)]
    pub const fn num_uch(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Number of unicast hash entries (1 bit per entry) per SI 00 64 unicast addresses 01 128 unicast addresses 10 256 unicast addresses 11 512 unicast addresses"]
    #[inline(always)]
    pub const fn set_num_uch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Number of MSI-X vectors per physical/virtual function Range: 1..64 Formula: NUM_MSIX+1"]
    #[must_use]
    #[inline(always)]
    pub const fn num_msix(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x3f;
        val as u8
    }
    #[doc = "Number of MSI-X vectors per physical/virtual function Range: 1..64 Formula: NUM_MSIX+1"]
    #[inline(always)]
    pub const fn set_num_msix(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 12usize)) | (((val as u32) & 0x3f) << 12usize);
    }
    #[doc = "Indicates the number of IPVs supported. 0: 8 IPVs 1: 16 IPVs"]
    #[must_use]
    #[inline(always)]
    pub const fn num_ipv(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the number of IPVs supported. 0: 8 IPVs 1: 16 IPVs"]
    #[inline(always)]
    pub const fn set_num_ipv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Sipcapr1 {
    #[inline(always)]
    fn default() -> Sipcapr1 {
        Sipcapr1(45104u64 as u32)
    }
}
impl core::fmt::Debug for Sipcapr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sipcapr1")
            .field("num_tcs", &self.num_tcs())
            .field("num_mch", &self.num_mch())
            .field("num_uch", &self.num_uch())
            .field("num_msix", &self.num_msix())
            .field("num_ipv", &self.num_ipv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sipcapr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sipcapr1 {
            num_tcs: u8,
            num_mch: u8,
            num_uch: u8,
            num_msix: u8,
            num_ipv: bool,
        }
        let proxy = Sipcapr1 {
            num_tcs: self.num_tcs(),
            num_mch: self.num_mch(),
            num_uch: self.num_uch(),
            num_msix: self.num_msix(),
            num_ipv: self.num_ipv(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface primary MAC address register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sipmar1(pub u32);
impl Sipmar1 {
    #[doc = "Primary MAC address This field is defined in network byte order (big-endian)"]
    #[must_use]
    #[inline(always)]
    pub const fn prim_mac_addr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Primary MAC address This field is defined in network byte order (big-endian)"]
    #[inline(always)]
    pub const fn set_prim_mac_addr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Sipmar1 {
    #[inline(always)]
    fn default() -> Sipmar1 {
        Sipmar1(0u64 as u32)
    }
}
impl core::fmt::Debug for Sipmar1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sipmar1")
            .field("prim_mac_addr", &self.prim_mac_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sipmar1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sipmar1 {
            prim_mac_addr: u16,
        }
        let proxy = Sipmar1 {
            prim_mac_addr: self.prim_mac_addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface receive BDR group control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sirbgcr(pub u32);
impl Sirbgcr {
    #[doc = "Number of groups Number of groups used, 0-2"]
    #[must_use]
    #[inline(always)]
    pub const fn num_groups(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Number of groups Number of groups used, 0-2"]
    #[inline(always)]
    pub const fn set_num_groups(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Number of rings per group The number of rings per groups is RINGS_PER_GROUP + 1, e"]
    #[must_use]
    #[inline(always)]
    pub const fn rings_per_group(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Number of rings per group The number of rings per groups is RINGS_PER_GROUP + 1, e"]
    #[inline(always)]
    pub const fn set_rings_per_group(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
}
impl Default for Sirbgcr {
    #[inline(always)]
    fn default() -> Sirbgcr {
        Sirbgcr(0u64 as u32)
    }
}
impl core::fmt::Debug for Sirbgcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sirbgcr")
            .field("num_groups", &self.num_groups())
            .field("rings_per_group", &self.rings_per_group())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sirbgcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sirbgcr {
            num_groups: u8,
            rings_per_group: u8,
        }
        let proxy = Sirbgcr {
            num_groups: self.num_groups(),
            rings_per_group: self.rings_per_group(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface receive interrupt detect register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sirxidr0(pub u32);
impl Sirxidr0 {
    #[doc = "Summary of detected interrupts for receive ring 0 assigned to SI 0 No interrupt detected for receive ring 0 1 Interrupt detected for receive ring 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rx0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Summary of detected interrupts for receive ring 0 assigned to SI 0 No interrupt detected for receive ring 0 1 Interrupt detected for receive ring 0"]
    #[inline(always)]
    pub const fn set_rx0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Summary of detected interrupts for receive ring 1 assigned to SI 0 No interrupt detected for receive ring 1 1 Interrupt detected for receive ring 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rx1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Summary of detected interrupts for receive ring 1 assigned to SI 0 No interrupt detected for receive ring 1 1 Interrupt detected for receive ring 1"]
    #[inline(always)]
    pub const fn set_rx1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Summary of detected interrupts for receive ring 2 assigned to SI 0 No interrupt detected for receive ring 2 1 Interrupt detected for receive ring 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rx2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Summary of detected interrupts for receive ring 2 assigned to SI 0 No interrupt detected for receive ring 2 1 Interrupt detected for receive ring 2"]
    #[inline(always)]
    pub const fn set_rx2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Summary of detected interrupts for receive ring 3 assigned to SI 0 No interrupt detected for receive ring 3 1 Interrupt detected for receive ring 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rx3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Summary of detected interrupts for receive ring 3 assigned to SI 0 No interrupt detected for receive ring 3 1 Interrupt detected for receive ring 3"]
    #[inline(always)]
    pub const fn set_rx3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Summary of detected interrupts for receive ring 4 assigned to SI 0 No interrupt detected for receive ring 4 1 Interrupt detected for receive ring 4"]
    #[must_use]
    #[inline(always)]
    pub const fn rx4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Summary of detected interrupts for receive ring 4 assigned to SI 0 No interrupt detected for receive ring 4 1 Interrupt detected for receive ring 4"]
    #[inline(always)]
    pub const fn set_rx4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Summary of detected interrupts for receive ring 5 assigned to SI 0 No interrupt detected for receive ring 5 1 Interrupt detected for receive ring 5"]
    #[must_use]
    #[inline(always)]
    pub const fn rx5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Summary of detected interrupts for receive ring 5 assigned to SI 0 No interrupt detected for receive ring 5 1 Interrupt detected for receive ring 5"]
    #[inline(always)]
    pub const fn set_rx5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Summary of detected interrupts for receive ring 6 assigned to SI 0 No interrupt detected for receive ring 6 1 Interrupt detected for receive ring 6"]
    #[must_use]
    #[inline(always)]
    pub const fn rx6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Summary of detected interrupts for receive ring 6 assigned to SI 0 No interrupt detected for receive ring 6 1 Interrupt detected for receive ring 6"]
    #[inline(always)]
    pub const fn set_rx6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Summary of detected interrupts for receive ring 7 assigned to SI 0 No interrupt detected for receive ring 7 1 Interrupt detected for receive ring 7"]
    #[must_use]
    #[inline(always)]
    pub const fn rx7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Summary of detected interrupts for receive ring 7 assigned to SI 0 No interrupt detected for receive ring 7 1 Interrupt detected for receive ring 7"]
    #[inline(always)]
    pub const fn set_rx7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Summary of detected interrupts for receive ring 8 assigned to SI 0 No interrupt detected for receive ring 8 1 Interrupt detected for receive ring 8"]
    #[must_use]
    #[inline(always)]
    pub const fn rx8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Summary of detected interrupts for receive ring 8 assigned to SI 0 No interrupt detected for receive ring 8 1 Interrupt detected for receive ring 8"]
    #[inline(always)]
    pub const fn set_rx8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Summary of detected interrupts for receive ring 9 assigned to SI 0 No interrupt detected for receive ring 9 1 Interrupt detected for receive ring 9"]
    #[must_use]
    #[inline(always)]
    pub const fn rx9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Summary of detected interrupts for receive ring 9 assigned to SI 0 No interrupt detected for receive ring 9 1 Interrupt detected for receive ring 9"]
    #[inline(always)]
    pub const fn set_rx9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Summary of detected interrupts for receive ring 10 assigned to SI 0 No interrupt detected for receive ring 10 1 Interrupt detected for receive ring 10"]
    #[must_use]
    #[inline(always)]
    pub const fn rx10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Summary of detected interrupts for receive ring 10 assigned to SI 0 No interrupt detected for receive ring 10 1 Interrupt detected for receive ring 10"]
    #[inline(always)]
    pub const fn set_rx10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Summary of detected interrupts for receive ring 11 assigned to SI 0 No interrupt detected for receive ring 11 1 Interrupt detected for receive ring 11"]
    #[must_use]
    #[inline(always)]
    pub const fn rx11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Summary of detected interrupts for receive ring 11 assigned to SI 0 No interrupt detected for receive ring 11 1 Interrupt detected for receive ring 11"]
    #[inline(always)]
    pub const fn set_rx11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Summary of detected interrupts for receive ring 12 assigned to SI 0 No interrupt detected for receive ring 12 1 Interrupt detected for receive ring 12"]
    #[must_use]
    #[inline(always)]
    pub const fn rx12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Summary of detected interrupts for receive ring 12 assigned to SI 0 No interrupt detected for receive ring 12 1 Interrupt detected for receive ring 12"]
    #[inline(always)]
    pub const fn set_rx12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Summary of detected interrupts for receive ring 13 assigned to SI 0 No interrupt detected for receive ring 13 1 Interrupt detected for receive ring 13"]
    #[must_use]
    #[inline(always)]
    pub const fn rx13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Summary of detected interrupts for receive ring 13 assigned to SI 0 No interrupt detected for receive ring 13 1 Interrupt detected for receive ring 13"]
    #[inline(always)]
    pub const fn set_rx13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
}
impl Default for Sirxidr0 {
    #[inline(always)]
    fn default() -> Sirxidr0 {
        Sirxidr0(0u64 as u32)
    }
}
impl core::fmt::Debug for Sirxidr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sirxidr0")
            .field("rx0", &self.rx0())
            .field("rx1", &self.rx1())
            .field("rx2", &self.rx2())
            .field("rx3", &self.rx3())
            .field("rx4", &self.rx4())
            .field("rx5", &self.rx5())
            .field("rx6", &self.rx6())
            .field("rx7", &self.rx7())
            .field("rx8", &self.rx8())
            .field("rx9", &self.rx9())
            .field("rx10", &self.rx10())
            .field("rx11", &self.rx11())
            .field("rx12", &self.rx12())
            .field("rx13", &self.rx13())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sirxidr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sirxidr0 {
            rx0: bool,
            rx1: bool,
            rx2: bool,
            rx3: bool,
            rx4: bool,
            rx5: bool,
            rx6: bool,
            rx7: bool,
            rx8: bool,
            rx9: bool,
            rx10: bool,
            rx11: bool,
            rx12: bool,
            rx13: bool,
        }
        let proxy = Sirxidr0 {
            rx0: self.rx0(),
            rx1: self.rx1(),
            rx2: self.rx2(),
            rx3: self.rx3(),
            rx4: self.rx4(),
            rx5: self.rx5(),
            rx6: self.rx6(),
            rx7: self.rx7(),
            rx8: self.rx8(),
            rx9: self.rx9(),
            rx10: self.rx10(),
            rx11: self.rx11(),
            rx12: self.rx12(),
            rx13: self.rx13(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sisr(pub u32);
impl Sisr {
    #[doc = "Transmit busy"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_busy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit busy"]
    #[inline(always)]
    pub const fn set_tx_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SI MAC unicast promiscuous"]
    #[must_use]
    #[inline(always)]
    pub const fn mac_up(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SI MAC unicast promiscuous"]
    #[inline(always)]
    pub const fn set_mac_up(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SI MAC multicast promiscuous"]
    #[must_use]
    #[inline(always)]
    pub const fn mac_mp(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SI MAC multicast promiscuous"]
    #[inline(always)]
    pub const fn set_mac_mp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "SI VLAN promiscuous"]
    #[must_use]
    #[inline(always)]
    pub const fn vlan_p(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "SI VLAN promiscuous"]
    #[inline(always)]
    pub const fn set_vlan_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "SI VLAN untagged frames accepted"]
    #[must_use]
    #[inline(always)]
    pub const fn vlan_uta(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "SI VLAN untagged frames accepted"]
    #[inline(always)]
    pub const fn set_vlan_uta(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Sisr {
    #[inline(always)]
    fn default() -> Sisr {
        Sisr(30u64 as u32)
    }
}
impl core::fmt::Debug for Sisr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sisr")
            .field("tx_busy", &self.tx_busy())
            .field("mac_up", &self.mac_up())
            .field("mac_mp", &self.mac_mp())
            .field("vlan_p", &self.vlan_p())
            .field("vlan_uta", &self.vlan_uta())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sisr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sisr {
            tx_busy: bool,
            mac_up: bool,
            mac_mp: bool,
            vlan_p: bool,
            vlan_uta: bool,
        }
        let proxy = Sisr {
            tx_busy: self.tx_busy(),
            mac_up: self.mac_up(),
            mac_mp: self.mac_mp(),
            vlan_p: self.vlan_p(),
            vlan_uta: self.vlan_uta(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface timer interrupt detect register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sitmridr(pub u32);
impl Sitmridr {
    #[doc = "Timer synchronous state change detected when set."]
    #[must_use]
    #[inline(always)]
    pub const fn sync(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Timer synchronous state change detected when set."]
    #[inline(always)]
    pub const fn set_sync(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Sitmridr {
    #[inline(always)]
    fn default() -> Sitmridr {
        Sitmridr(0u64 as u32)
    }
}
impl core::fmt::Debug for Sitmridr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sitmridr")
            .field("sync", &self.sync())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sitmridr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sitmridr {
            sync: bool,
        }
        let proxy = Sitmridr { sync: self.sync() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface timer interrupt enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sitmrier(pub u32);
impl Sitmrier {
    #[doc = "Timer synchronous state change interrupt enable 0 Disabled 1 Enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn synce(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Timer synchronous state change interrupt enable 0 Disabled 1 Enabled"]
    #[inline(always)]
    pub const fn set_synce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Sitmrier {
    #[inline(always)]
    fn default() -> Sitmrier {
        Sitmrier(0u64 as u32)
    }
}
impl core::fmt::Debug for Sitmrier {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sitmrier")
            .field("synce", &self.synce())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sitmrier {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sitmrier {
            synce: bool,
        }
        let proxy = Sitmrier {
            synce: self.synce(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface timer MSI-X vector register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sitmrmsivr(pub u32);
impl Sitmrmsivr {
    #[doc = "Vector Index into MSI-X address/data table"]
    #[must_use]
    #[inline(always)]
    pub const fn vector(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Vector Index into MSI-X address/data table"]
    #[inline(always)]
    pub const fn set_vector(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for Sitmrmsivr {
    #[inline(always)]
    fn default() -> Sitmrmsivr {
        Sitmrmsivr(0u64 as u32)
    }
}
impl core::fmt::Debug for Sitmrmsivr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sitmrmsivr")
            .field("vector", &self.vector())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sitmrmsivr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sitmrmsivr {
            vector: u8,
        }
        let proxy = Sitmrmsivr {
            vector: self.vector(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface timer status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sitsr(pub u32);
impl Sitsr {
    #[doc = "Timer synchronization"]
    #[must_use]
    #[inline(always)]
    pub const fn sync(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Timer synchronization"]
    #[inline(always)]
    pub const fn set_sync(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "User specific parameter values"]
    #[must_use]
    #[inline(always)]
    pub const fn param_val(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "User specific parameter values"]
    #[inline(always)]
    pub const fn set_param_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for Sitsr {
    #[inline(always)]
    fn default() -> Sitsr {
        Sitsr(0u64 as u32)
    }
}
impl core::fmt::Debug for Sitsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sitsr")
            .field("sync", &self.sync())
            .field("param_val", &self.param_val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sitsr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sitsr {
            sync: bool,
            param_val: u32,
        }
        let proxy = Sitsr {
            sync: self.sync(),
            param_val: self.param_val(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface transmit interrupt detect register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sitxidr0(pub u32);
impl Sitxidr0 {
    #[doc = "Summary of detected threshold interrupts for transmit ring 0 assigned to SI 0 No interrupt detected for transmit ring 0 1 Threshold interrupt detected for transmit ring 0"]
    #[must_use]
    #[inline(always)]
    pub const fn txt0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Summary of detected threshold interrupts for transmit ring 0 assigned to SI 0 No interrupt detected for transmit ring 0 1 Threshold interrupt detected for transmit ring 0"]
    #[inline(always)]
    pub const fn set_txt0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Summary of detected threshold interrupts for transmit ring 1 assigned to SI 0 No interrupt detected for transmit ring 1 1 Threshold interrupt detected for transmit ring 1"]
    #[must_use]
    #[inline(always)]
    pub const fn txt1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Summary of detected threshold interrupts for transmit ring 1 assigned to SI 0 No interrupt detected for transmit ring 1 1 Threshold interrupt detected for transmit ring 1"]
    #[inline(always)]
    pub const fn set_txt1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Summary of detected threshold interrupts for transmit ring 2 assigned to SI 0 No interrupt detected for transmit ring 2 1 Threshold interrupt detected for transmit ring 2"]
    #[must_use]
    #[inline(always)]
    pub const fn txt2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Summary of detected threshold interrupts for transmit ring 2 assigned to SI 0 No interrupt detected for transmit ring 2 1 Threshold interrupt detected for transmit ring 2"]
    #[inline(always)]
    pub const fn set_txt2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Summary of detected threshold interrupts for transmit ring 3 assigned to SI 0 No interrupt detected for transmit ring 3 1 Threshold interrupt detected for transmit ring 3"]
    #[must_use]
    #[inline(always)]
    pub const fn txt3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Summary of detected threshold interrupts for transmit ring 3 assigned to SI 0 No interrupt detected for transmit ring 3 1 Threshold interrupt detected for transmit ring 3"]
    #[inline(always)]
    pub const fn set_txt3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Summary of detected threshold interrupts for transmit ring 4 assigned to SI 0 No interrupt detected for transmit ring 4 1 Threshold interrupt detected for transmit ring 4"]
    #[must_use]
    #[inline(always)]
    pub const fn txt4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Summary of detected threshold interrupts for transmit ring 4 assigned to SI 0 No interrupt detected for transmit ring 4 1 Threshold interrupt detected for transmit ring 4"]
    #[inline(always)]
    pub const fn set_txt4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Summary of detected threshold interrupts for transmit ring 5 assigned to SI 0 No interrupt detected for transmit ring 5 1 Threshold interrupt detected for transmit ring 5"]
    #[must_use]
    #[inline(always)]
    pub const fn txt5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Summary of detected threshold interrupts for transmit ring 5 assigned to SI 0 No interrupt detected for transmit ring 5 1 Threshold interrupt detected for transmit ring 5"]
    #[inline(always)]
    pub const fn set_txt5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Summary of detected threshold interrupts for transmit ring 6 assigned to SI 0 No interrupt detected for transmit ring 6 1 Threshold interrupt detected for transmit ring 6"]
    #[must_use]
    #[inline(always)]
    pub const fn txt6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Summary of detected threshold interrupts for transmit ring 6 assigned to SI 0 No interrupt detected for transmit ring 6 1 Threshold interrupt detected for transmit ring 6"]
    #[inline(always)]
    pub const fn set_txt6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Summary of detected threshold interrupts for transmit ring 7 assigned to SI 0 No interrupt detected for transmit ring 7 1 Threshold interrupt detected for transmit ring 7"]
    #[must_use]
    #[inline(always)]
    pub const fn txt7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Summary of detected threshold interrupts for transmit ring 7 assigned to SI 0 No interrupt detected for transmit ring 7 1 Threshold interrupt detected for transmit ring 7"]
    #[inline(always)]
    pub const fn set_txt7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Summary of detected threshold interrupts for transmit ring 8 assigned to SI 0 No interrupt detected for transmit ring 8 1 Threshold interrupt detected for transmit ring 8"]
    #[must_use]
    #[inline(always)]
    pub const fn txt8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Summary of detected threshold interrupts for transmit ring 8 assigned to SI 0 No interrupt detected for transmit ring 8 1 Threshold interrupt detected for transmit ring 8"]
    #[inline(always)]
    pub const fn set_txt8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Summary of detected threshold interrupts for transmit ring 9 assigned to SI 0 No interrupt detected for transmit ring 9 1 Threshold interrupt detected for transmit ring 9"]
    #[must_use]
    #[inline(always)]
    pub const fn txt9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Summary of detected threshold interrupts for transmit ring 9 assigned to SI 0 No interrupt detected for transmit ring 9 1 Threshold interrupt detected for transmit ring 9"]
    #[inline(always)]
    pub const fn set_txt9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Summary of detected threshold interrupts for transmit ring 10 assigned to SI 0 No interrupt detected for transmit ring 10 1 Threshold interrupt detected for transmit ring 10"]
    #[must_use]
    #[inline(always)]
    pub const fn txt10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Summary of detected threshold interrupts for transmit ring 10 assigned to SI 0 No interrupt detected for transmit ring 10 1 Threshold interrupt detected for transmit ring 10"]
    #[inline(always)]
    pub const fn set_txt10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Summary of detected threshold interrupts for transmit ring 11 assigned to SI 0 No interrupt detected for transmit ring 11 1 Threshold interrupt detected for transmit ring 11"]
    #[must_use]
    #[inline(always)]
    pub const fn txt11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Summary of detected threshold interrupts for transmit ring 11 assigned to SI 0 No interrupt detected for transmit ring 11 1 Threshold interrupt detected for transmit ring 11"]
    #[inline(always)]
    pub const fn set_txt11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Summary of detected threshold interrupts for transmit ring 12 assigned to SI 0 No interrupt detected for transmit ring 12 1 Threshold interrupt detected for transmit ring 12"]
    #[must_use]
    #[inline(always)]
    pub const fn txt12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Summary of detected threshold interrupts for transmit ring 12 assigned to SI 0 No interrupt detected for transmit ring 12 1 Threshold interrupt detected for transmit ring 12"]
    #[inline(always)]
    pub const fn set_txt12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Summary of detected threshold interrupts for transmit ring 13 assigned to SI 0 No interrupt detected for transmit ring 13 1 Threshold interrupt detected for transmit ring 13"]
    #[must_use]
    #[inline(always)]
    pub const fn txt13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Summary of detected threshold interrupts for transmit ring 13 assigned to SI 0 No interrupt detected for transmit ring 13 1 Threshold interrupt detected for transmit ring 13"]
    #[inline(always)]
    pub const fn set_txt13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Summary of detected transmit frame interrupts for transmit ring 0 assigned to SI 0 No interrupt detected for transmit ring 0 1 Frame transmit interrupt detected for transmit ring 0"]
    #[must_use]
    #[inline(always)]
    pub const fn txf0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Summary of detected transmit frame interrupts for transmit ring 0 assigned to SI 0 No interrupt detected for transmit ring 0 1 Frame transmit interrupt detected for transmit ring 0"]
    #[inline(always)]
    pub const fn set_txf0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Summary of detected transmit frame interrupts for transmit ring 1 assigned to SI 0 No interrupt detected for transmit ring 1 1 Frame transmit interrupt detected for transmit ring 1"]
    #[must_use]
    #[inline(always)]
    pub const fn txf1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Summary of detected transmit frame interrupts for transmit ring 1 assigned to SI 0 No interrupt detected for transmit ring 1 1 Frame transmit interrupt detected for transmit ring 1"]
    #[inline(always)]
    pub const fn set_txf1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Summary of detected transmit frame interrupts for transmit ring 2 assigned to SI 0 No interrupt detected for transmit ring 2 1 Frame transmit interrupt detected for transmit ring 2"]
    #[must_use]
    #[inline(always)]
    pub const fn txf2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Summary of detected transmit frame interrupts for transmit ring 2 assigned to SI 0 No interrupt detected for transmit ring 2 1 Frame transmit interrupt detected for transmit ring 2"]
    #[inline(always)]
    pub const fn set_txf2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Summary of detected transmit frame interrupts for transmit ring 3 assigned to SI 0 No interrupt detected for transmit ring 3 1 Frame transmit interrupt detected for transmit ring 3"]
    #[must_use]
    #[inline(always)]
    pub const fn txf3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Summary of detected transmit frame interrupts for transmit ring 3 assigned to SI 0 No interrupt detected for transmit ring 3 1 Frame transmit interrupt detected for transmit ring 3"]
    #[inline(always)]
    pub const fn set_txf3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Summary of detected transmit frame interrupts for transmit ring 4 assigned to SI 0 No interrupt detected for transmit ring 4 1 Frame transmit interrupt detected for transmit ring 4"]
    #[must_use]
    #[inline(always)]
    pub const fn txf4(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Summary of detected transmit frame interrupts for transmit ring 4 assigned to SI 0 No interrupt detected for transmit ring 4 1 Frame transmit interrupt detected for transmit ring 4"]
    #[inline(always)]
    pub const fn set_txf4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Summary of detected transmit frame interrupts for transmit ring 5 assigned to SI 0 No interrupt detected for transmit ring 5 1 Frame transmit interrupt detected for transmit ring 5"]
    #[must_use]
    #[inline(always)]
    pub const fn txf5(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Summary of detected transmit frame interrupts for transmit ring 5 assigned to SI 0 No interrupt detected for transmit ring 5 1 Frame transmit interrupt detected for transmit ring 5"]
    #[inline(always)]
    pub const fn set_txf5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Summary of detected transmit frame interrupts for transmit ring 6 assigned to SI 0 No interrupt detected for transmit ring 6 1 Frame transmit interrupt detected for transmit ring 6"]
    #[must_use]
    #[inline(always)]
    pub const fn txf6(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Summary of detected transmit frame interrupts for transmit ring 6 assigned to SI 0 No interrupt detected for transmit ring 6 1 Frame transmit interrupt detected for transmit ring 6"]
    #[inline(always)]
    pub const fn set_txf6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Summary of detected transmit frame interrupts for transmit ring 7 assigned to SI 0 No interrupt detected for transmit ring 7 1 Frame transmit interrupt detected for transmit ring 7"]
    #[must_use]
    #[inline(always)]
    pub const fn txf7(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Summary of detected transmit frame interrupts for transmit ring 7 assigned to SI 0 No interrupt detected for transmit ring 7 1 Frame transmit interrupt detected for transmit ring 7"]
    #[inline(always)]
    pub const fn set_txf7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Summary of detected transmit frame interrupts for transmit ring 8 assigned to SI 0 No interrupt detected for transmit ring 8 1 Frame transmit interrupt detected for transmit ring 8"]
    #[must_use]
    #[inline(always)]
    pub const fn txf8(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Summary of detected transmit frame interrupts for transmit ring 8 assigned to SI 0 No interrupt detected for transmit ring 8 1 Frame transmit interrupt detected for transmit ring 8"]
    #[inline(always)]
    pub const fn set_txf8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Summary of detected transmit frame interrupts for transmit ring 9 assigned to SI 0 No interrupt detected for transmit ring 9 1 Frame transmit interrupt detected for transmit ring 9"]
    #[must_use]
    #[inline(always)]
    pub const fn txf9(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Summary of detected transmit frame interrupts for transmit ring 9 assigned to SI 0 No interrupt detected for transmit ring 9 1 Frame transmit interrupt detected for transmit ring 9"]
    #[inline(always)]
    pub const fn set_txf9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Summary of detected transmit frame interrupts for transmit ring 10 assigned to SI 0 No interrupt detected for transmit ring 10 1 Frame transmit interrupt detected for transmit ring 10"]
    #[must_use]
    #[inline(always)]
    pub const fn txf10(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Summary of detected transmit frame interrupts for transmit ring 10 assigned to SI 0 No interrupt detected for transmit ring 10 1 Frame transmit interrupt detected for transmit ring 10"]
    #[inline(always)]
    pub const fn set_txf10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Summary of detected transmit frame interrupts for transmit ring 11 assigned to SI 0 No interrupt detected for transmit ring 11 1 Frame transmit interrupt detected for transmit ring 11"]
    #[must_use]
    #[inline(always)]
    pub const fn txf11(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Summary of detected transmit frame interrupts for transmit ring 11 assigned to SI 0 No interrupt detected for transmit ring 11 1 Frame transmit interrupt detected for transmit ring 11"]
    #[inline(always)]
    pub const fn set_txf11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Summary of detected transmit frame interrupts for transmit ring 12 assigned to SI 0 No interrupt detected for transmit ring 12 1 Frame transmit interrupt detected for transmit ring 12"]
    #[must_use]
    #[inline(always)]
    pub const fn txf12(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Summary of detected transmit frame interrupts for transmit ring 12 assigned to SI 0 No interrupt detected for transmit ring 12 1 Frame transmit interrupt detected for transmit ring 12"]
    #[inline(always)]
    pub const fn set_txf12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Summary of detected transmit frame interrupts for transmit ring 13 assigned to SI 0 No interrupt detected for transmit ring 13 1 Frame transmit interrupt detected for transmit ring 13"]
    #[must_use]
    #[inline(always)]
    pub const fn txf13(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Summary of detected transmit frame interrupts for transmit ring 13 assigned to SI 0 No interrupt detected for transmit ring 13 1 Frame transmit interrupt detected for transmit ring 13"]
    #[inline(always)]
    pub const fn set_txf13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for Sitxidr0 {
    #[inline(always)]
    fn default() -> Sitxidr0 {
        Sitxidr0(0u64 as u32)
    }
}
impl core::fmt::Debug for Sitxidr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sitxidr0")
            .field("txt0", &self.txt0())
            .field("txt1", &self.txt1())
            .field("txt2", &self.txt2())
            .field("txt3", &self.txt3())
            .field("txt4", &self.txt4())
            .field("txt5", &self.txt5())
            .field("txt6", &self.txt6())
            .field("txt7", &self.txt7())
            .field("txt8", &self.txt8())
            .field("txt9", &self.txt9())
            .field("txt10", &self.txt10())
            .field("txt11", &self.txt11())
            .field("txt12", &self.txt12())
            .field("txt13", &self.txt13())
            .field("txf0", &self.txf0())
            .field("txf1", &self.txf1())
            .field("txf2", &self.txf2())
            .field("txf3", &self.txf3())
            .field("txf4", &self.txf4())
            .field("txf5", &self.txf5())
            .field("txf6", &self.txf6())
            .field("txf7", &self.txf7())
            .field("txf8", &self.txf8())
            .field("txf9", &self.txf9())
            .field("txf10", &self.txf10())
            .field("txf11", &self.txf11())
            .field("txf12", &self.txf12())
            .field("txf13", &self.txf13())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sitxidr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sitxidr0 {
            txt0: bool,
            txt1: bool,
            txt2: bool,
            txt3: bool,
            txt4: bool,
            txt5: bool,
            txt6: bool,
            txt7: bool,
            txt8: bool,
            txt9: bool,
            txt10: bool,
            txt11: bool,
            txt12: bool,
            txt13: bool,
            txf0: bool,
            txf1: bool,
            txf2: bool,
            txf3: bool,
            txf4: bool,
            txf5: bool,
            txf6: bool,
            txf7: bool,
            txf8: bool,
            txf9: bool,
            txf10: bool,
            txf11: bool,
            txf12: bool,
            txf13: bool,
        }
        let proxy = Sitxidr0 {
            txt0: self.txt0(),
            txt1: self.txt1(),
            txt2: self.txt2(),
            txt3: self.txt3(),
            txt4: self.txt4(),
            txt5: self.txt5(),
            txt6: self.txt6(),
            txt7: self.txt7(),
            txt8: self.txt8(),
            txt9: self.txt9(),
            txt10: self.txt10(),
            txt11: self.txt11(),
            txt12: self.txt12(),
            txt13: self.txt13(),
            txf0: self.txf0(),
            txf1: self.txf1(),
            txf2: self.txf2(),
            txf3: self.txf3(),
            txf4: self.txf4(),
            txf5: self.txf5(),
            txf6: self.txf6(),
            txf7: self.txf7(),
            txf8: self.txf8(),
            txf9: self.txf9(),
            txf10: self.txf10(),
            txf11: self.txf11(),
            txf12: self.txf12(),
            txf13: self.txf13(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface uncorrectable fatal memory error configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Siufmecr(pub u32);
impl Siufmecr {
    #[doc = "Report disable"]
    #[must_use]
    #[inline(always)]
    pub const fn rd(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Report disable"]
    #[inline(always)]
    pub const fn set_rd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Siufmecr {
    #[inline(always)]
    fn default() -> Siufmecr {
        Siufmecr(0u64 as u32)
    }
}
impl core::fmt::Debug for Siufmecr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Siufmecr").field("rd", &self.rd()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Siufmecr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Siufmecr {
            rd: bool,
        }
        let proxy = Siufmecr { rd: self.rd() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface uncorrectable fatal memory error status register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Siufmesr0(pub u32);
impl Siufmesr0 {
    #[doc = "Syndrome"]
    #[must_use]
    #[inline(always)]
    pub const fn syndrome(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Syndrome"]
    #[inline(always)]
    pub const fn set_syndrome(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "Memory ID"]
    #[must_use]
    #[inline(always)]
    pub const fn mem_id(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Memory ID"]
    #[inline(always)]
    pub const fn set_mem_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Multiple"]
    #[must_use]
    #[inline(always)]
    pub const fn m(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Multiple"]
    #[inline(always)]
    pub const fn set_m(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Multi-bit ECC error"]
    #[must_use]
    #[inline(always)]
    pub const fn mbee(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Multi-bit ECC error"]
    #[inline(always)]
    pub const fn set_mbee(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Siufmesr0 {
    #[inline(always)]
    fn default() -> Siufmesr0 {
        Siufmesr0(0u64 as u32)
    }
}
impl core::fmt::Debug for Siufmesr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Siufmesr0")
            .field("syndrome", &self.syndrome())
            .field("mem_id", &self.mem_id())
            .field("m", &self.m())
            .field("mbee", &self.mbee())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Siufmesr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Siufmesr0 {
            syndrome: u16,
            mem_id: u8,
            m: bool,
            mbee: bool,
        }
        let proxy = Siufmesr0 {
            syndrome: self.syndrome(),
            mem_id: self.mem_id(),
            m: self.m(),
            mbee: self.mbee(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface uncorrectable fatal system bus error configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Siufsbecr(pub u32);
impl Siufsbecr {
    #[doc = "Report disable"]
    #[must_use]
    #[inline(always)]
    pub const fn rd(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Report disable"]
    #[inline(always)]
    pub const fn set_rd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Siufsbecr {
    #[inline(always)]
    fn default() -> Siufsbecr {
        Siufsbecr(0u64 as u32)
    }
}
impl core::fmt::Debug for Siufsbecr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Siufsbecr").field("rd", &self.rd()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Siufsbecr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Siufsbecr {
            rd: bool,
        }
        let proxy = Siufsbecr { rd: self.rd() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface uncorrectable fatal system bus error status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Siufsbesr(pub u32);
impl Siufsbesr {
    #[doc = "System Bus ID"]
    #[must_use]
    #[inline(always)]
    pub const fn sb_id(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "System Bus ID"]
    #[inline(always)]
    pub const fn set_sb_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Multiple"]
    #[must_use]
    #[inline(always)]
    pub const fn m(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Multiple"]
    #[inline(always)]
    pub const fn set_m(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "System bus error"]
    #[must_use]
    #[inline(always)]
    pub const fn sbe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "System bus error"]
    #[inline(always)]
    pub const fn set_sbe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Siufsbesr {
    #[inline(always)]
    fn default() -> Siufsbesr {
        Siufsbesr(0u64 as u32)
    }
}
impl core::fmt::Debug for Siufsbesr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Siufsbesr")
            .field("sb_id", &self.sb_id())
            .field("m", &self.m())
            .field("sbe", &self.sbe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Siufsbesr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Siufsbesr {
            sb_id: u8,
            m: bool,
            sbe: bool,
        }
        let proxy = Siufsbesr {
            sb_id: self.sb_id(),
            m: self.m(),
            sbe: self.sbe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface uncorrectable non-fatal memory error configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Siunmecr(pub u32);
impl Siunmecr {
    #[doc = "Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn threshold(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Threshold"]
    #[inline(always)]
    pub const fn set_threshold(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Report disable"]
    #[must_use]
    #[inline(always)]
    pub const fn rd(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Report disable"]
    #[inline(always)]
    pub const fn set_rd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Siunmecr {
    #[inline(always)]
    fn default() -> Siunmecr {
        Siunmecr(0u64 as u32)
    }
}
impl core::fmt::Debug for Siunmecr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Siunmecr")
            .field("threshold", &self.threshold())
            .field("rd", &self.rd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Siunmecr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Siunmecr {
            threshold: u8,
            rd: bool,
        }
        let proxy = Siunmecr {
            threshold: self.threshold(),
            rd: self.rd(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface uncorrectable non-fatal memory error count register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Siunmectr(pub u32);
impl Siunmectr {
    #[doc = "Count"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Count"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Siunmectr {
    #[inline(always)]
    fn default() -> Siunmectr {
        Siunmectr(0u64 as u32)
    }
}
impl core::fmt::Debug for Siunmectr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Siunmectr")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Siunmectr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Siunmectr {
            count: u8,
        }
        let proxy = Siunmectr {
            count: self.count(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface uncorrectable non-fatal memory error status register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Siunmesr0(pub u32);
impl Siunmesr0 {
    #[doc = "Syndrome"]
    #[must_use]
    #[inline(always)]
    pub const fn syndrome(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Syndrome"]
    #[inline(always)]
    pub const fn set_syndrome(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "Memory ID"]
    #[must_use]
    #[inline(always)]
    pub const fn mem_id(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Memory ID"]
    #[inline(always)]
    pub const fn set_mem_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Multi-bit ECC error"]
    #[must_use]
    #[inline(always)]
    pub const fn mbee(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Multi-bit ECC error"]
    #[inline(always)]
    pub const fn set_mbee(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Siunmesr0 {
    #[inline(always)]
    fn default() -> Siunmesr0 {
        Siunmesr0(0u64 as u32)
    }
}
impl core::fmt::Debug for Siunmesr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Siunmesr0")
            .field("syndrome", &self.syndrome())
            .field("mem_id", &self.mem_id())
            .field("mbee", &self.mbee())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Siunmesr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Siunmesr0 {
            syndrome: u16,
            mem_id: u8,
            mbee: bool,
        }
        let proxy = Siunmesr0 {
            syndrome: self.syndrome(),
            mem_id: self.mem_id(),
            mbee: self.mbee(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface uncorrectable non-fatal system bus error configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Siunsbecr(pub u32);
impl Siunsbecr {
    #[doc = "Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn threshold(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Threshold"]
    #[inline(always)]
    pub const fn set_threshold(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Siunsbecr {
    #[inline(always)]
    fn default() -> Siunsbecr {
        Siunsbecr(0u64 as u32)
    }
}
impl core::fmt::Debug for Siunsbecr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Siunsbecr")
            .field("threshold", &self.threshold())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Siunsbecr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Siunsbecr {
            threshold: u8,
        }
        let proxy = Siunsbecr {
            threshold: self.threshold(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface uncorrectable non-fatal system bus error count register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Siunsbectr(pub u32);
impl Siunsbectr {
    #[doc = "Count"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Count"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Siunsbectr {
    #[inline(always)]
    fn default() -> Siunsbectr {
        Siunsbectr(0u64 as u32)
    }
}
impl core::fmt::Debug for Siunsbectr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Siunsbectr")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Siunsbectr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Siunsbectr {
            count: u8,
        }
        let proxy = Siunsbectr {
            count: self.count(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface uncorrectable non-fatal system bus error status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Siunsbesr(pub u32);
impl Siunsbesr {
    #[doc = "System Bus ID"]
    #[must_use]
    #[inline(always)]
    pub const fn sb_id(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "System Bus ID"]
    #[inline(always)]
    pub const fn set_sb_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "System bus error"]
    #[must_use]
    #[inline(always)]
    pub const fn sbe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "System bus error"]
    #[inline(always)]
    pub const fn set_sbe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Siunsbesr {
    #[inline(always)]
    fn default() -> Siunsbesr {
        Siunsbesr(0u64 as u32)
    }
}
impl core::fmt::Debug for Siunsbesr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Siunsbesr")
            .field("sb_id", &self.sb_id())
            .field("sbe", &self.sbe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Siunsbesr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Siunsbesr {
            sb_id: u8,
            sbe: bool,
        }
        let proxy = Siunsbesr {
            sb_id: self.sb_id(),
            sbe: self.sbe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface uncorrectable programming error configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Siupecr(pub u32);
impl Siupecr {
    #[doc = "Report disable"]
    #[must_use]
    #[inline(always)]
    pub const fn rd(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Report disable"]
    #[inline(always)]
    pub const fn set_rd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Siupecr {
    #[inline(always)]
    fn default() -> Siupecr {
        Siupecr(0u64 as u32)
    }
}
impl core::fmt::Debug for Siupecr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Siupecr").field("rd", &self.rd()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Siupecr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Siupecr {
            rd: bool,
        }
        let proxy = Siupecr { rd: self.rd() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface uncorrectable programming error status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Siupesr(pub u32);
impl Siupesr {
    #[doc = "Station interface disabled drop error."]
    #[must_use]
    #[inline(always)]
    pub const fn drop_si_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Station interface disabled drop error."]
    #[inline(always)]
    pub const fn set_drop_si_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Station interface ring disabled drop error."]
    #[must_use]
    #[inline(always)]
    pub const fn drop_ring_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Station interface ring disabled drop error."]
    #[inline(always)]
    pub const fn set_drop_ring_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Non-existing receive ring error."]
    #[must_use]
    #[inline(always)]
    pub const fn drop_ring_sel(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Non-existing receive ring error."]
    #[inline(always)]
    pub const fn set_drop_ring_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Multiple"]
    #[must_use]
    #[inline(always)]
    pub const fn m(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Multiple"]
    #[inline(always)]
    pub const fn set_m(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Programming error"]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Programming error"]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Siupesr {
    #[inline(always)]
    fn default() -> Siupesr {
        Siupesr(0u64 as u32)
    }
}
impl core::fmt::Debug for Siupesr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Siupesr")
            .field("drop_si_en", &self.drop_si_en())
            .field("drop_ring_en", &self.drop_ring_en())
            .field("drop_ring_sel", &self.drop_ring_sel())
            .field("m", &self.m())
            .field("pe", &self.pe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Siupesr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Siupesr {
            drop_si_en: bool,
            drop_ring_en: bool,
            drop_ring_sel: bool,
            m: bool,
            pe: bool,
        }
        let proxy = Siupesr {
            drop_si_en: self.drop_si_en(),
            drop_ring_en: self.drop_ring_en(),
            drop_ring_sel: self.drop_ring_sel(),
            m: self.m(),
            pe: self.pe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface VLAN filter table capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sivftcapr(pub u32);
impl Sivftcapr {
    #[doc = "Number of VLAN filter table entries for use by SI"]
    #[must_use]
    #[inline(always)]
    pub const fn num_vlan_fte(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Number of VLAN filter table entries for use by SI"]
    #[inline(always)]
    pub const fn set_num_vlan_fte(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Sivftcapr {
    #[inline(always)]
    fn default() -> Sivftcapr {
        Sivftcapr(4u64 as u32)
    }
}
impl core::fmt::Debug for Sivftcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sivftcapr")
            .field("num_vlan_fte", &self.num_vlan_fte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sivftcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sivftcapr {
            num_vlan_fte: u8,
        }
        let proxy = Sivftcapr {
            num_vlan_fte: self.num_vlan_fte(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface VLAN to IPV mapping register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sivlanipvmr0(pub u32);
impl Sivlanipvmr0 {
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    #[inline(always)]
    pub const fn set_pcp_dei_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_1(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    #[inline(always)]
    pub const fn set_pcp_dei_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    #[inline(always)]
    pub const fn set_pcp_dei_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_3(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    #[inline(always)]
    pub const fn set_pcp_dei_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_4(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    #[inline(always)]
    pub const fn set_pcp_dei_4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_5(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    #[inline(always)]
    pub const fn set_pcp_dei_5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_6(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    #[inline(always)]
    pub const fn set_pcp_dei_6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_7(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    #[inline(always)]
    pub const fn set_pcp_dei_7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Sivlanipvmr0 {
    #[inline(always)]
    fn default() -> Sivlanipvmr0 {
        Sivlanipvmr0(0u64 as u32)
    }
}
impl core::fmt::Debug for Sivlanipvmr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sivlanipvmr0")
            .field("pcp_dei_0", &self.pcp_dei_0())
            .field("pcp_dei_1", &self.pcp_dei_1())
            .field("pcp_dei_2", &self.pcp_dei_2())
            .field("pcp_dei_3", &self.pcp_dei_3())
            .field("pcp_dei_4", &self.pcp_dei_4())
            .field("pcp_dei_5", &self.pcp_dei_5())
            .field("pcp_dei_6", &self.pcp_dei_6())
            .field("pcp_dei_7", &self.pcp_dei_7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sivlanipvmr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sivlanipvmr0 {
            pcp_dei_0: u8,
            pcp_dei_1: u8,
            pcp_dei_2: u8,
            pcp_dei_3: u8,
            pcp_dei_4: u8,
            pcp_dei_5: u8,
            pcp_dei_6: u8,
            pcp_dei_7: u8,
        }
        let proxy = Sivlanipvmr0 {
            pcp_dei_0: self.pcp_dei_0(),
            pcp_dei_1: self.pcp_dei_1(),
            pcp_dei_2: self.pcp_dei_2(),
            pcp_dei_3: self.pcp_dei_3(),
            pcp_dei_4: self.pcp_dei_4(),
            pcp_dei_5: self.pcp_dei_5(),
            pcp_dei_6: self.pcp_dei_6(),
            pcp_dei_7: self.pcp_dei_7(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Station interface VLAN to IPV mapping register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sivlanipvmr1(pub u32);
impl Sivlanipvmr1 {
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_8(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    #[inline(always)]
    pub const fn set_pcp_dei_8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_9(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    #[inline(always)]
    pub const fn set_pcp_dei_9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_10(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    #[inline(always)]
    pub const fn set_pcp_dei_10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_11(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    #[inline(always)]
    pub const fn set_pcp_dei_11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_12(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    #[inline(always)]
    pub const fn set_pcp_dei_12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_13(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    #[inline(always)]
    pub const fn set_pcp_dei_13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_14(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    #[inline(always)]
    pub const fn set_pcp_dei_14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_15(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    #[inline(always)]
    pub const fn set_pcp_dei_15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Sivlanipvmr1 {
    #[inline(always)]
    fn default() -> Sivlanipvmr1 {
        Sivlanipvmr1(0u64 as u32)
    }
}
impl core::fmt::Debug for Sivlanipvmr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sivlanipvmr1")
            .field("pcp_dei_8", &self.pcp_dei_8())
            .field("pcp_dei_9", &self.pcp_dei_9())
            .field("pcp_dei_10", &self.pcp_dei_10())
            .field("pcp_dei_11", &self.pcp_dei_11())
            .field("pcp_dei_12", &self.pcp_dei_12())
            .field("pcp_dei_13", &self.pcp_dei_13())
            .field("pcp_dei_14", &self.pcp_dei_14())
            .field("pcp_dei_15", &self.pcp_dei_15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sivlanipvmr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sivlanipvmr1 {
            pcp_dei_8: u8,
            pcp_dei_9: u8,
            pcp_dei_10: u8,
            pcp_dei_11: u8,
            pcp_dei_12: u8,
            pcp_dei_13: u8,
            pcp_dei_14: u8,
            pcp_dei_15: u8,
        }
        let proxy = Sivlanipvmr1 {
            pcp_dei_8: self.pcp_dei_8(),
            pcp_dei_9: self.pcp_dei_9(),
            pcp_dei_10: self.pcp_dei_10(),
            pcp_dei_11: self.pcp_dei_11(),
            pcp_dei_12: self.pcp_dei_12(),
            pcp_dei_13: self.pcp_dei_13(),
            pcp_dei_14: self.pcp_dei_14(),
            pcp_dei_15: self.pcp_dei_15(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Tx BDR a base address register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tbbar0(pub u32);
impl Tbbar0 {
    #[doc = "Lower address bits 31-7."]
    #[must_use]
    #[inline(always)]
    pub const fn addrl(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "Lower address bits 31-7."]
    #[inline(always)]
    pub const fn set_addrl(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for Tbbar0 {
    #[inline(always)]
    fn default() -> Tbbar0 {
        Tbbar0(0u64 as u32)
    }
}
impl core::fmt::Debug for Tbbar0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tbbar0")
            .field("addrl", &self.addrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tbbar0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Tbbar0 {
            addrl: u32,
        }
        let proxy = Tbbar0 {
            addrl: self.addrl(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Tx BDR a consumer index register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tbcir(pub u32);
impl Tbcir {
    #[doc = "Transmit buffer descriptor ring consumer index"]
    #[must_use]
    #[inline(always)]
    pub const fn bdr_index(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Transmit buffer descriptor ring consumer index"]
    #[inline(always)]
    pub const fn set_bdr_index(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Status identifier"]
    #[must_use]
    #[inline(always)]
    pub const fn stat_id(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Status identifier"]
    #[inline(always)]
    pub const fn set_stat_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Tbcir {
    #[inline(always)]
    fn default() -> Tbcir {
        Tbcir(0u64 as u32)
    }
}
impl core::fmt::Debug for Tbcir {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tbcir")
            .field("bdr_index", &self.bdr_index())
            .field("stat_id", &self.stat_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tbcir {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Tbcir {
            bdr_index: u16,
            stat_id: u16,
        }
        let proxy = Tbcir {
            bdr_index: self.bdr_index(),
            stat_id: self.stat_id(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Tx BDR a interrupt coalescing register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tbicr0(pub u32);
impl Tbicr0 {
    #[doc = "Interrupt coalescing packet threshold While interrupt coalescing is enabled, ICEN=1, this values determines the minimum number of packets transmitted before raising an interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn icpt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Interrupt coalescing packet threshold While interrupt coalescing is enabled, ICEN=1, this values determines the minimum number of packets transmitted before raising an interrupt"]
    #[inline(always)]
    pub const fn set_icpt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Interrupt coalescing enable 0 Interrupt coalescing is disabled 1 Interrupt coalescing is enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn icen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt coalescing enable 0 Interrupt coalescing is disabled 1 Interrupt coalescing is enabled"]
    #[inline(always)]
    pub const fn set_icen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Tbicr0 {
    #[inline(always)]
    fn default() -> Tbicr0 {
        Tbicr0(0u64 as u32)
    }
}
impl core::fmt::Debug for Tbicr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tbicr0")
            .field("icpt", &self.icpt())
            .field("icen", &self.icen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tbicr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Tbicr0 {
            icpt: u8,
            icen: bool,
        }
        let proxy = Tbicr0 {
            icpt: self.icpt(),
            icen: self.icen(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Tx BDR a interrupt detect register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tbidr(pub u32);
impl Tbidr {
    #[doc = "Transmit threshold 0 No threshold event detected 1 Transmit ring has transmitted at least the number of packets specified by TBaICR0\\[ICPT\\] or the threshold timer has expired since last transmitted packet as specified by TBaICR1\\[ICTT\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn txt(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit threshold 0 No threshold event detected 1 Transmit ring has transmitted at least the number of packets specified by TBaICR0\\[ICPT\\] or the threshold timer has expired since last transmitted packet as specified by TBaICR1\\[ICTT\\]"]
    #[inline(always)]
    pub const fn set_txt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit of frame 0 No transmit of frame detected with BD\\[FI\\]=1 1 Transmit of frame detected with BD\\[FI\\]=1"]
    #[must_use]
    #[inline(always)]
    pub const fn txf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit of frame 0 No transmit of frame detected with BD\\[FI\\]=1 1 Transmit of frame detected with BD\\[FI\\]=1"]
    #[inline(always)]
    pub const fn set_txf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Tbidr {
    #[inline(always)]
    fn default() -> Tbidr {
        Tbidr(0u64 as u32)
    }
}
impl core::fmt::Debug for Tbidr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tbidr")
            .field("txt", &self.txt())
            .field("txf", &self.txf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tbidr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Tbidr {
            txt: bool,
            txf: bool,
        }
        let proxy = Tbidr {
            txt: self.txt(),
            txf: self.txf(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Tx BDR a interrupt enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tbier(pub u32);
impl Tbier {
    #[doc = "Transmit threshold interrupt enable 0 Disabled 1 Enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn txtie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit threshold interrupt enable 0 Disabled 1 Enabled"]
    #[inline(always)]
    pub const fn set_txtie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit frame interrupt enable 0 Disabled 1 Enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn txfie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit frame interrupt enable 0 Disabled 1 Enabled"]
    #[inline(always)]
    pub const fn set_txfie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Tbier {
    #[inline(always)]
    fn default() -> Tbier {
        Tbier(0u64 as u32)
    }
}
impl core::fmt::Debug for Tbier {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tbier")
            .field("txtie", &self.txtie())
            .field("txfie", &self.txfie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tbier {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Tbier {
            txtie: bool,
            txfie: bool,
        }
        let proxy = Tbier {
            txtie: self.txtie(),
            txfie: self.txfie(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Tx BDR a length register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tblenr(pub u32);
impl Tblenr {
    #[doc = "BD ring length Size of ring in sets of 8 BDs. Maximum ring size is 64K."]
    #[must_use]
    #[inline(always)]
    pub const fn length(&self) -> u16 {
        let val = (self.0 >> 3usize) & 0x3fff;
        val as u16
    }
    #[doc = "BD ring length Size of ring in sets of 8 BDs. Maximum ring size is 64K."]
    #[inline(always)]
    pub const fn set_length(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 3usize)) | (((val as u32) & 0x3fff) << 3usize);
    }
}
impl Default for Tblenr {
    #[inline(always)]
    fn default() -> Tblenr {
        Tblenr(0u64 as u32)
    }
}
impl core::fmt::Debug for Tblenr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tblenr")
            .field("length", &self.length())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tblenr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Tblenr {
            length: u16,
        }
        let proxy = Tblenr {
            length: self.length(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Tx BDR a mode register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tbmr(pub u32);
impl Tbmr {
    #[doc = "Priority Priority of the transmit buffer descriptor ring"]
    #[must_use]
    #[inline(always)]
    pub const fn prio(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Priority Priority of the transmit buffer descriptor ring"]
    #[inline(always)]
    pub const fn set_prio(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "WRR weight Weight used for arbitration when two or more rings have the same priority within the same VSI"]
    #[must_use]
    #[inline(always)]
    pub const fn wrr(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "WRR weight Weight used for arbitration when two or more rings have the same priority within the same VSI"]
    #[inline(always)]
    pub const fn set_wrr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "User CRC provided Determines if user provides CRC32 (FCS) at the end of the frame"]
    #[must_use]
    #[inline(always)]
    pub const fn crc(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "User CRC provided Determines if user provides CRC32 (FCS) at the end of the frame"]
    #[inline(always)]
    pub const fn set_crc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "VLAN Insert Hint If set (b1) then SW intends to use VLAN insertion offload by providing VLAN tag data in the Tx BD"]
    #[must_use]
    #[inline(always)]
    pub const fn vih(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "VLAN Insert Hint If set (b1) then SW intends to use VLAN insertion offload by providing VLAN tag data in the Tx BD"]
    #[inline(always)]
    pub const fn set_vih(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Enable transmit buffer descriptor ring 0: Disabled"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable transmit buffer descriptor ring 0: Disabled"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Tbmr {
    #[inline(always)]
    fn default() -> Tbmr {
        Tbmr(0u64 as u32)
    }
}
impl core::fmt::Debug for Tbmr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tbmr")
            .field("prio", &self.prio())
            .field("wrr", &self.wrr())
            .field("crc", &self.crc())
            .field("vih", &self.vih())
            .field("en", &self.en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tbmr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Tbmr {
            prio: u8,
            wrr: u8,
            crc: bool,
            vih: bool,
            en: bool,
        }
        let proxy = Tbmr {
            prio: self.prio(),
            wrr: self.wrr(),
            crc: self.crc(),
            vih: self.vih(),
            en: self.en(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Tx BDR a producer index register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tbpir(pub u32);
impl Tbpir {
    #[doc = "Transmit buffer descriptor ring producer index"]
    #[must_use]
    #[inline(always)]
    pub const fn bdr_index(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Transmit buffer descriptor ring producer index"]
    #[inline(always)]
    pub const fn set_bdr_index(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Tbpir {
    #[inline(always)]
    fn default() -> Tbpir {
        Tbpir(0u64 as u32)
    }
}
impl core::fmt::Debug for Tbpir {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tbpir")
            .field("bdr_index", &self.bdr_index())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tbpir {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Tbpir {
            bdr_index: u16,
        }
        let proxy = Tbpir {
            bdr_index: self.bdr_index(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Tx BDR a status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tbsr(pub u32);
impl Tbsr {
    #[doc = "Busy"]
    #[must_use]
    #[inline(always)]
    pub const fn busy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Busy"]
    #[inline(always)]
    pub const fn set_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "System bus error A system bus error has occurred during one or more transactions related to this transmit ring, including possibly the transmit BD writeback entry itself"]
    #[must_use]
    #[inline(always)]
    pub const fn sbe(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "System bus error A system bus error has occurred during one or more transactions related to this transmit ring, including possibly the transmit BD writeback entry itself"]
    #[inline(always)]
    pub const fn set_sbe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Tbsr {
    #[inline(always)]
    fn default() -> Tbsr {
        Tbsr(0u64 as u32)
    }
}
impl core::fmt::Debug for Tbsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tbsr")
            .field("busy", &self.busy())
            .field("sbe", &self.sbe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tbsr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Tbsr {
            busy: bool,
            sbe: bool,
        }
        let proxy = Tbsr {
            busy: self.busy(),
            sbe: self.sbe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
