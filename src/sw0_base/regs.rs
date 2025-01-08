#[doc = "Buffer pool capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bpcapr(pub u32);
impl Bpcapr {
    #[doc = "Number of buffer pools supported."]
    #[must_use]
    #[inline(always)]
    pub const fn num_bp(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Number of buffer pools supported."]
    #[inline(always)]
    pub const fn set_num_bp(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Number of shared buffer pools supported."]
    #[must_use]
    #[inline(always)]
    pub const fn num_spb(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Number of shared buffer pools supported."]
    #[inline(always)]
    pub const fn set_num_spb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for Bpcapr {
    #[inline(always)]
    fn default() -> Bpcapr {
        Bpcapr(131080u64 as u32)
    }
}
impl core::fmt::Debug for Bpcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bpcapr")
            .field("num_bp", &self.num_bp())
            .field("num_spb", &self.num_spb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bpcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Bpcapr {
            num_bp: u8,
            num_spb: u8,
        }
        let proxy = Bpcapr {
            num_bp: self.num_bp(),
            num_spb: self.num_spb(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Bridge capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Brcapr(pub u32);
impl Brcapr {
    #[doc = "L2 IPv4 multicast filtering supported."]
    #[must_use]
    #[inline(always)]
    pub const fn ipv4mflt(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "L2 IPv4 multicast filtering supported."]
    #[inline(always)]
    pub const fn set_ipv4mflt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Station Move Disable supported"]
    #[must_use]
    #[inline(always)]
    pub const fn stamvd(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Station Move Disable supported"]
    #[inline(always)]
    pub const fn set_stamvd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Storm Control supported."]
    #[must_use]
    #[inline(always)]
    pub const fn strmctrl(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Storm Control supported."]
    #[inline(always)]
    pub const fn set_strmctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Source port pruning disable supported"]
    #[must_use]
    #[inline(always)]
    pub const fn srcpprnd(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Source port pruning disable supported"]
    #[inline(always)]
    pub const fn set_srcpprnd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Egress VLAN translation supported"]
    #[must_use]
    #[inline(always)]
    pub const fn evlanxlate(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Egress VLAN translation supported"]
    #[inline(always)]
    pub const fn set_evlanxlate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Number of Spanning Tree Groups supported"]
    #[must_use]
    #[inline(always)]
    pub const fn num_stg(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Number of Spanning Tree Groups supported"]
    #[inline(always)]
    pub const fn set_num_stg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
}
impl Default for Brcapr {
    #[inline(always)]
    fn default() -> Brcapr {
        Brcapr(4157u64 as u32)
    }
}
impl core::fmt::Debug for Brcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Brcapr")
            .field("ipv4mflt", &self.ipv4mflt())
            .field("stamvd", &self.stamvd())
            .field("strmctrl", &self.strmctrl())
            .field("srcpprnd", &self.srcpprnd())
            .field("evlanxlate", &self.evlanxlate())
            .field("num_stg", &self.num_stg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Brcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Brcapr {
            ipv4mflt: bool,
            stamvd: bool,
            strmctrl: bool,
            srcpprnd: bool,
            evlanxlate: bool,
            num_stg: u8,
        }
        let proxy = Brcapr {
            ipv4mflt: self.ipv4mflt(),
            stamvd: self.stamvd(),
            strmctrl: self.strmctrl(),
            srcpprnd: self.srcpprnd(),
            evlanxlate: self.evlanxlate(),
            num_stg: self.num_stg(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Command BDR base address register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cbdrbar0(pub u32);
impl Cbdrbar0 {
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
impl Default for Cbdrbar0 {
    #[inline(always)]
    fn default() -> Cbdrbar0 {
        Cbdrbar0(0u64 as u32)
    }
}
impl core::fmt::Debug for Cbdrbar0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cbdrbar0")
            .field("addrl", &self.addrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cbdrbar0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cbdrbar0 {
            addrl: u32,
        }
        let proxy = Cbdrbar0 {
            addrl: self.addrl(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Command BDR a consumer index register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cbdrcir(pub u32);
impl Cbdrcir {
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
impl Default for Cbdrcir {
    #[inline(always)]
    fn default() -> Cbdrcir {
        Cbdrcir(0u64 as u32)
    }
}
impl core::fmt::Debug for Cbdrcir {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cbdrcir")
            .field("bdr_index", &self.bdr_index())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cbdrcir {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cbdrcir {
            bdr_index: u16,
        }
        let proxy = Cbdrcir {
            bdr_index: self.bdr_index(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Command BDR a interrupt detect register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cbdridr(pub u32);
impl Cbdridr {
    #[doc = "Command BD completed"]
    #[must_use]
    #[inline(always)]
    pub const fn cbdc(&self) -> super::vals::Cbdc {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Cbdc::from_bits(val as u8)
    }
    #[doc = "Command BD completed"]
    #[inline(always)]
    pub const fn set_cbdc(&mut self, val: super::vals::Cbdc) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Cbdridr {
    #[inline(always)]
    fn default() -> Cbdridr {
        Cbdridr(0u64 as u32)
    }
}
impl core::fmt::Debug for Cbdridr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cbdridr")
            .field("cbdc", &self.cbdc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cbdridr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cbdridr {
            cbdc: super::vals::Cbdc,
        }
        let proxy = Cbdridr { cbdc: self.cbdc() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Command BDR a interrupt enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cbdrier(pub u32);
impl Cbdrier {
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
impl Default for Cbdrier {
    #[inline(always)]
    fn default() -> Cbdrier {
        Cbdrier(0u64 as u32)
    }
}
impl core::fmt::Debug for Cbdrier {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cbdrier")
            .field("cbdcie", &self.cbdcie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cbdrier {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cbdrier {
            cbdcie: bool,
        }
        let proxy = Cbdrier {
            cbdcie: self.cbdcie(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Command BDR a length register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cbdrlenr(pub u32);
impl Cbdrlenr {
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
impl Default for Cbdrlenr {
    #[inline(always)]
    fn default() -> Cbdrlenr {
        Cbdrlenr(0u64 as u32)
    }
}
impl core::fmt::Debug for Cbdrlenr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cbdrlenr")
            .field("length", &self.length())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cbdrlenr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cbdrlenr {
            length: u8,
        }
        let proxy = Cbdrlenr {
            length: self.length(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Command BDR a mode register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cbdrmr(pub u32);
impl Cbdrmr {
    #[doc = "Enable command buffer descriptor ring"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable command buffer descriptor ring"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Cbdrmr {
    #[inline(always)]
    fn default() -> Cbdrmr {
        Cbdrmr(0u64 as u32)
    }
}
impl core::fmt::Debug for Cbdrmr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cbdrmr").field("en", &self.en()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cbdrmr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cbdrmr {
            en: bool,
        }
        let proxy = Cbdrmr { en: self.en() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Command BDR a MSI-X vector register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cbdrmsivr(pub u32);
impl Cbdrmsivr {
    #[doc = "Index into MSI-X address/data table"]
    #[must_use]
    #[inline(always)]
    pub const fn vector(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Index into MSI-X address/data table"]
    #[inline(always)]
    pub const fn set_vector(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Cbdrmsivr {
    #[inline(always)]
    fn default() -> Cbdrmsivr {
        Cbdrmsivr(0u64 as u32)
    }
}
impl core::fmt::Debug for Cbdrmsivr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cbdrmsivr")
            .field("vector", &self.vector())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cbdrmsivr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cbdrmsivr {
            vector: u8,
        }
        let proxy = Cbdrmsivr {
            vector: self.vector(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Command BDR a producer index register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cbdrpir(pub u32);
impl Cbdrpir {
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
impl Default for Cbdrpir {
    #[inline(always)]
    fn default() -> Cbdrpir {
        Cbdrpir(0u64 as u32)
    }
}
impl core::fmt::Debug for Cbdrpir {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cbdrpir")
            .field("bdr_index", &self.bdr_index())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cbdrpir {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cbdrpir {
            bdr_index: u16,
        }
        let proxy = Cbdrpir {
            bdr_index: self.bdr_index(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Command BDR a status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cbdrsr(pub u32);
impl Cbdrsr {
    #[doc = "Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn busy(&self) -> super::vals::Busy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Busy::from_bits(val as u8)
    }
    #[doc = "Busy."]
    #[inline(always)]
    pub const fn set_busy(&mut self, val: super::vals::Busy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Cbdrsr {
    #[inline(always)]
    fn default() -> Cbdrsr {
        Cbdrsr(0u64 as u32)
    }
}
impl core::fmt::Debug for Cbdrsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cbdrsr")
            .field("busy", &self.busy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cbdrsr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cbdrsr {
            busy: super::vals::Busy,
        }
        let proxy = Cbdrsr { busy: self.busy() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Command cache attribute register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccar(pub u32);
impl Ccar {
    #[doc = "Command buffer descriptor write cache type This is the cache attribute setting used when switch writes the command buffer descriptor in host memory"]
    #[must_use]
    #[inline(always)]
    pub const fn cbd_wrcache(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Command buffer descriptor write cache type This is the cache attribute setting used when switch writes the command buffer descriptor in host memory"]
    #[inline(always)]
    pub const fn set_cbd_wrcache(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Command buffer descriptor write domain This is the domain attribute setting used when switch writes the command buffer descriptor in host memory"]
    #[must_use]
    #[inline(always)]
    pub const fn cbd_wrdomain(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Command buffer descriptor write domain This is the domain attribute setting used when switch writes the command buffer descriptor in host memory"]
    #[inline(always)]
    pub const fn set_cbd_wrdomain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Command buffer descriptor write snoop This is the snoop attribute setting used when switch writes the command buffer descriptor in host memory"]
    #[must_use]
    #[inline(always)]
    pub const fn cbd_wrsnp(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Command buffer descriptor write snoop This is the snoop attribute setting used when switch writes the command buffer descriptor in host memory"]
    #[inline(always)]
    pub const fn set_cbd_wrsnp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Write data cache type This is the cache attribute setting used when switch writes command data to host memory"]
    #[must_use]
    #[inline(always)]
    pub const fn cwrcache(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Write data cache type This is the cache attribute setting used when switch writes command data to host memory"]
    #[inline(always)]
    pub const fn set_cwrcache(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Write data domain This is the domain attribute setting used when switch writes command data to host memory"]
    #[must_use]
    #[inline(always)]
    pub const fn cwrdomain(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Write data domain This is the domain attribute setting used when switch writes command data to host memory"]
    #[inline(always)]
    pub const fn set_cwrdomain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Write data snoop This is the snoop attribute setting used when switch writes command data to host memory"]
    #[must_use]
    #[inline(always)]
    pub const fn cwrsnp(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Write data snoop This is the snoop attribute setting used when switch writes command data to host memory"]
    #[inline(always)]
    pub const fn set_cwrsnp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Command buffer descriptor read cache type This is the cache attribute setting used when switch reads the command buffer descriptor from host memory"]
    #[must_use]
    #[inline(always)]
    pub const fn cbd_rdcache(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Command buffer descriptor read cache type This is the cache attribute setting used when switch reads the command buffer descriptor from host memory"]
    #[inline(always)]
    pub const fn set_cbd_rdcache(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Command buffer descriptor read domain This is the domain attribute setting used when switch reads the command buffer descriptor from host memory"]
    #[must_use]
    #[inline(always)]
    pub const fn cbd_rddomain(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Command buffer descriptor read domain This is the domain attribute setting used when switch reads the command buffer descriptor from host memory"]
    #[inline(always)]
    pub const fn set_cbd_rddomain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "Command descriptor read snoop See System Interface Read Transaction Attribute Definitions table in for valid settings"]
    #[must_use]
    #[inline(always)]
    pub const fn cbd_rdsnp(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Command descriptor read snoop See System Interface Read Transaction Attribute Definitions table in for valid settings"]
    #[inline(always)]
    pub const fn set_cbd_rdsnp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Read data cache type This is the cache attribute setting used when switch reads command data from host memory"]
    #[must_use]
    #[inline(always)]
    pub const fn crdcache(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Read data cache type This is the cache attribute setting used when switch reads command data from host memory"]
    #[inline(always)]
    pub const fn set_crdcache(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Read data domain This is the domain attribute setting used when switch reads command data from host memory"]
    #[must_use]
    #[inline(always)]
    pub const fn crddomain(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "Read data domain This is the domain attribute setting used when switch reads command data from host memory"]
    #[inline(always)]
    pub const fn set_crddomain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "Read data snoop This is the snoop attribute setting used when switch reads command data from host memory"]
    #[must_use]
    #[inline(always)]
    pub const fn crdsnp(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Read data snoop This is the snoop attribute setting used when switch reads command data from host memory"]
    #[inline(always)]
    pub const fn set_crdsnp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Ccar {
    #[inline(always)]
    fn default() -> Ccar {
        Ccar(33686018u64 as u32)
    }
}
impl core::fmt::Debug for Ccar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccar")
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
impl defmt::Format for Ccar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ccar {
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
        let proxy = Ccar {
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
#[doc = "Forwarding capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcapr(pub u32);
impl Fcapr {
    #[doc = "802.1Q bridge forwarding support."]
    #[must_use]
    #[inline(always)]
    pub const fn br(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "802.1Q bridge forwarding support."]
    #[inline(always)]
    pub const fn set_br(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Stream forwarding supported"]
    #[must_use]
    #[inline(always)]
    pub const fn sf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Stream forwarding supported"]
    #[inline(always)]
    pub const fn set_sf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Fcapr {
    #[inline(always)]
    fn default() -> Fcapr {
        Fcapr(3u64 as u32)
    }
}
impl core::fmt::Debug for Fcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fcapr")
            .field("br", &self.br())
            .field("sf", &self.sf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Fcapr {
            br: bool,
            sf: bool,
        }
        let proxy = Fcapr {
            br: self.br(),
            sf: self.sf(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FDB hash table capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fdbhtcapr(pub u32);
impl Fdbhtcapr {
    #[doc = "Number of guaranteed MAC addresses which can be added to the FDB table if an entry cannot be added due to collision limit being exceeded"]
    #[must_use]
    #[inline(always)]
    pub const fn num_gmac(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Number of guaranteed MAC addresses which can be added to the FDB table if an entry cannot be added due to collision limit being exceeded"]
    #[inline(always)]
    pub const fn set_num_gmac(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    #[must_use]
    #[inline(always)]
    pub const fn access_meth(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    #[inline(always)]
    pub const fn set_access_meth(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
}
impl Default for Fdbhtcapr {
    #[inline(always)]
    fn default() -> Fdbhtcapr {
        Fdbhtcapr(11534336u64 as u32)
    }
}
impl core::fmt::Debug for Fdbhtcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fdbhtcapr")
            .field("num_gmac", &self.num_gmac())
            .field("access_meth", &self.access_meth())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fdbhtcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Fdbhtcapr {
            num_gmac: u16,
            access_meth: u8,
        }
        let proxy = Fdbhtcapr {
            num_gmac: self.num_gmac(),
            access_meth: self.access_meth(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FDB hash table memory configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fdbhtmcr(pub u32);
impl Fdbhtmcr {
    #[doc = "This field specifies the maximum number of dynamic entries allowed in the FDB table for the entire switch"]
    #[must_use]
    #[inline(always)]
    pub const fn dyn_limit(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This field specifies the maximum number of dynamic entries allowed in the FDB table for the entire switch"]
    #[inline(always)]
    pub const fn set_dyn_limit(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Fdbhtmcr {
    #[inline(always)]
    fn default() -> Fdbhtmcr {
        Fdbhtmcr(0u64 as u32)
    }
}
impl core::fmt::Debug for Fdbhtmcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fdbhtmcr")
            .field("dyn_limit", &self.dyn_limit())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fdbhtmcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Fdbhtmcr {
            dyn_limit: u16,
        }
        let proxy = Fdbhtmcr {
            dyn_limit: self.dyn_limit(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FDB hash table operational register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fdbhtor0(pub u32);
impl Fdbhtor0 {
    #[doc = "Number of static entries in-use in the FDB table."]
    #[must_use]
    #[inline(always)]
    pub const fn static_entries(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of static entries in-use in the FDB table."]
    #[inline(always)]
    pub const fn set_static_entries(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "The number of guaranteed MAC entries in-use in the FDB table"]
    #[must_use]
    #[inline(always)]
    pub const fn num_gentries(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x01ff;
        val as u16
    }
    #[doc = "The number of guaranteed MAC entries in-use in the FDB table"]
    #[inline(always)]
    pub const fn set_num_gentries(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
    }
}
impl Default for Fdbhtor0 {
    #[inline(always)]
    fn default() -> Fdbhtor0 {
        Fdbhtor0(0u64 as u32)
    }
}
impl core::fmt::Debug for Fdbhtor0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fdbhtor0")
            .field("static_entries", &self.static_entries())
            .field("num_gentries", &self.num_gentries())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fdbhtor0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Fdbhtor0 {
            static_entries: u16,
            num_gentries: u16,
        }
        let proxy = Fdbhtor0 {
            static_entries: self.static_entries(),
            num_gentries: self.num_gentries(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FDB hash table operational register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fdbhtor1(pub u32);
impl Fdbhtor1 {
    #[doc = "Number of dynamic entries in-use in the FDB table."]
    #[must_use]
    #[inline(always)]
    pub const fn dyn_entries(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of dynamic entries in-use in the FDB table."]
    #[inline(always)]
    pub const fn set_dyn_entries(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "High water mark of dynamic entries in-use in the FDB table. Value reset to DYN_ENTRIES when read."]
    #[must_use]
    #[inline(always)]
    pub const fn hwm_dyn_entries(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "High water mark of dynamic entries in-use in the FDB table. Value reset to DYN_ENTRIES when read."]
    #[inline(always)]
    pub const fn set_hwm_dyn_entries(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Fdbhtor1 {
    #[inline(always)]
    fn default() -> Fdbhtor1 {
        Fdbhtor1(0u64 as u32)
    }
}
impl core::fmt::Debug for Fdbhtor1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fdbhtor1")
            .field("dyn_entries", &self.dyn_entries())
            .field("hwm_dyn_entries", &self.hwm_dyn_entries())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fdbhtor1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Fdbhtor1 {
            dyn_entries: u16,
            hwm_dyn_entries: u16,
        }
        let proxy = Fdbhtor1 {
            dyn_entries: self.dyn_entries(),
            hwm_dyn_entries: self.hwm_dyn_entries(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ingress mirror destination configuration register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Imdcr0(pub u32);
impl Imdcr0 {
    #[doc = "Mirror enable."]
    #[must_use]
    #[inline(always)]
    pub const fn miren(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Mirror enable."]
    #[inline(always)]
    pub const fn set_miren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates the mirror destination"]
    #[must_use]
    #[inline(always)]
    pub const fn mirdest(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the mirror destination"]
    #[inline(always)]
    pub const fn set_mirdest(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Mirrored packet's IPV."]
    #[must_use]
    #[inline(always)]
    pub const fn ipv(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x07;
        val as u8
    }
    #[doc = "Mirrored packet's IPV."]
    #[inline(always)]
    pub const fn set_ipv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
    }
    #[doc = "Mirrored packet's DR (drop resilience)."]
    #[must_use]
    #[inline(always)]
    pub const fn dr(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Mirrored packet's DR (drop resilience)."]
    #[inline(always)]
    pub const fn set_dr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Port where ingress mirrored frames are sent. Valid if MIRDEST=0."]
    #[must_use]
    #[inline(always)]
    pub const fn port(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Port where ingress mirrored frames are sent. Valid if MIRDEST=0."]
    #[inline(always)]
    pub const fn set_port(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
}
impl Default for Imdcr0 {
    #[inline(always)]
    fn default() -> Imdcr0 {
        Imdcr0(192u64 as u32)
    }
}
impl core::fmt::Debug for Imdcr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Imdcr0")
            .field("miren", &self.miren())
            .field("mirdest", &self.mirdest())
            .field("ipv", &self.ipv())
            .field("dr", &self.dr())
            .field("port", &self.port())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Imdcr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Imdcr0 {
            miren: bool,
            mirdest: bool,
            ipv: u8,
            dr: u8,
            port: u8,
        }
        let proxy = Imdcr0 {
            miren: self.miren(),
            mirdest: self.mirdest(),
            ipv: self.ipv(),
            dr: self.dr(),
            port: self.port(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ingress mirror destination configuration register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Imdcr1(pub u32);
impl Imdcr1 {
    #[doc = "Egress Frame Modification Entry Id Only applicable if MIRDEST=0"]
    #[must_use]
    #[inline(always)]
    pub const fn efmeid(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Egress Frame Modification Entry Id Only applicable if MIRDEST=0"]
    #[inline(always)]
    pub const fn set_efmeid(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Egress Frame Modification Frame Length change in 2s complement notation"]
    #[must_use]
    #[inline(always)]
    pub const fn efm_len_change(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "Egress Frame Modification Frame Length change in 2s complement notation"]
    #[inline(always)]
    pub const fn set_efm_len_change(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
}
impl Default for Imdcr1 {
    #[inline(always)]
    fn default() -> Imdcr1 {
        Imdcr1(0u64 as u32)
    }
}
impl core::fmt::Debug for Imdcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Imdcr1")
            .field("efmeid", &self.efmeid())
            .field("efm_len_change", &self.efm_len_change())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Imdcr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Imdcr1 {
            efmeid: u16,
            efm_len_change: u8,
        }
        let proxy = Imdcr1 {
            efmeid: self.efmeid(),
            efm_len_change: self.efm_len_change(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "IP multicast filter hash table capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipmfhtcapr(pub u32);
impl Ipmfhtcapr {
    #[doc = "Indicates which configuration access methods are supported: xxx1: Index xx1x: EntryId x1xx: Search 1xxx: Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn access_meth(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: Index xx1x: EntryId x1xx: Search 1xxx: Reserved"]
    #[inline(always)]
    pub const fn set_access_meth(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
}
impl Default for Ipmfhtcapr {
    #[inline(always)]
    fn default() -> Ipmfhtcapr {
        Ipmfhtcapr(7340032u64 as u32)
    }
}
impl core::fmt::Debug for Ipmfhtcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipmfhtcapr")
            .field("access_meth", &self.access_meth())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipmfhtcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ipmfhtcapr {
            access_meth: u8,
        }
        let proxy = Ipmfhtcapr {
            access_meth: self.access_meth(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "IPv4 multicast filter hash table operation register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipv4mfhtor(pub u32);
impl Ipv4mfhtor {
    #[doc = "Number of IPV4 Any Source Multicast (ASM) Multicast Filter table entries in-use."]
    #[must_use]
    #[inline(always)]
    pub const fn asm_entries(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of IPV4 Any Source Multicast (ASM) Multicast Filter table entries in-use."]
    #[inline(always)]
    pub const fn set_asm_entries(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Number of IPV4 Source Specific Multicast (SSM) Multicast Filter table entries in-use."]
    #[must_use]
    #[inline(always)]
    pub const fn ssm_entries(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of IPV4 Source Specific Multicast (SSM) Multicast Filter table entries in-use."]
    #[inline(always)]
    pub const fn set_ssm_entries(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Ipv4mfhtor {
    #[inline(always)]
    fn default() -> Ipv4mfhtor {
        Ipv4mfhtor(0u64 as u32)
    }
}
impl core::fmt::Debug for Ipv4mfhtor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipv4mfhtor")
            .field("asm_entries", &self.asm_entries())
            .field("ssm_entries", &self.ssm_entries())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipv4mfhtor {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ipv4mfhtor {
            asm_entries: u16,
            ssm_entries: u16,
        }
        let proxy = Ipv4mfhtor {
            asm_entries: self.asm_entries(),
            ssm_entries: self.ssm_entries(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Management port configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mpcr(pub u32);
impl Mpcr {
    #[doc = "Switch Management Port"]
    #[must_use]
    #[inline(always)]
    pub const fn port(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Switch Management Port"]
    #[inline(always)]
    pub const fn set_port(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for Mpcr {
    #[inline(always)]
    fn default() -> Mpcr {
        Mpcr(4u64 as u32)
    }
}
impl core::fmt::Debug for Mpcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mpcr").field("port", &self.port()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mpcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Mpcr {
            port: u8,
        }
        let proxy = Mpcr { port: self.port() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCP to PCP mapping profile a register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcp2pcpmpr(pub u32);
impl Pcp2pcpmpr {
    #[doc = "PCP0 priority code point mapped value."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "PCP0 priority code point mapped value."]
    #[inline(always)]
    pub const fn set_pcp0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "PCP1 priority code point mapped value."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp1(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "PCP1 priority code point mapped value."]
    #[inline(always)]
    pub const fn set_pcp1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "PCP2 priority code point mapped value."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "PCP2 priority code point mapped value."]
    #[inline(always)]
    pub const fn set_pcp2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "PCP3 priority code point mapped value."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp3(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "PCP3 priority code point mapped value."]
    #[inline(always)]
    pub const fn set_pcp3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "PCP4 priority code point mapped value."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp4(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "PCP4 priority code point mapped value."]
    #[inline(always)]
    pub const fn set_pcp4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "PCP5 priority code point mapped value."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp5(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "PCP5 priority code point mapped value."]
    #[inline(always)]
    pub const fn set_pcp5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    #[doc = "PCP6 priority code point mapped value."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp6(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "PCP6 priority code point mapped value."]
    #[inline(always)]
    pub const fn set_pcp6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "PCP7 priority code point mapped value."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp7(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x07;
        val as u8
    }
    #[doc = "PCP7 priority code point mapped value."]
    #[inline(always)]
    pub const fn set_pcp7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
    }
}
impl Default for Pcp2pcpmpr {
    #[inline(always)]
    fn default() -> Pcp2pcpmpr {
        Pcp2pcpmpr(0u64 as u32)
    }
}
impl core::fmt::Debug for Pcp2pcpmpr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcp2pcpmpr")
            .field("pcp0", &self.pcp0())
            .field("pcp1", &self.pcp1())
            .field("pcp2", &self.pcp2())
            .field("pcp3", &self.pcp3())
            .field("pcp4", &self.pcp4())
            .field("pcp5", &self.pcp5())
            .field("pcp6", &self.pcp6())
            .field("pcp7", &self.pcp7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcp2pcpmpr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pcp2pcpmpr {
            pcp0: u8,
            pcp1: u8,
            pcp2: u8,
            pcp3: u8,
            pcp4: u8,
            pcp5: u8,
            pcp6: u8,
            pcp7: u8,
        }
        let proxy = Pcp2pcpmpr {
            pcp0: self.pcp0(),
            pcp1: self.pcp1(),
            pcp2: self.pcp2(),
            pcp3: self.pcp3(),
            pcp4: self.pcp4(),
            pcp5: self.pcp5(),
            pcp6: self.pcp6(),
            pcp7: self.pcp7(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "QoS to VLAN mapping profile a register b"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qosvlanmpr(pub u32);
impl Qosvlanmpr {
    #[doc = "The IPV0DR0 field is defined as a 4-bit field as follows: Bit 3 - Reserved Bit 2:0 - Priority Code Point (PCP)"]
    #[must_use]
    #[inline(always)]
    pub const fn ipv0_dr0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "The IPV0DR0 field is defined as a 4-bit field as follows: Bit 3 - Reserved Bit 2:0 - Priority Code Point (PCP)"]
    #[inline(always)]
    pub const fn set_ipv0_dr0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "The IPV0DR1 field is defined as a 4-bit field as follows: Bit 3 - Reserved Bit 2:0 - Priority Code Point (PCP)"]
    #[must_use]
    #[inline(always)]
    pub const fn ipv0_dr1(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "The IPV0DR1 field is defined as a 4-bit field as follows: Bit 3 - Reserved Bit 2:0 - Priority Code Point (PCP)"]
    #[inline(always)]
    pub const fn set_ipv0_dr1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "The IPV0DR2 field is defined as a 4-bit field as follows: Bit 3 - Reserved Bit 2:0 - Priority Code Point (PCP)"]
    #[must_use]
    #[inline(always)]
    pub const fn ipv0_dr2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "The IPV0DR2 field is defined as a 4-bit field as follows: Bit 3 - Reserved Bit 2:0 - Priority Code Point (PCP)"]
    #[inline(always)]
    pub const fn set_ipv0_dr2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "The IPV0DR3 field is defined as a 4-bit field as follows: Bit 3 - Reserved Bit 2:0 - Priority Code Point (PCP)"]
    #[must_use]
    #[inline(always)]
    pub const fn ipv0_dr3(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "The IPV0DR3 field is defined as a 4-bit field as follows: Bit 3 - Reserved Bit 2:0 - Priority Code Point (PCP)"]
    #[inline(always)]
    pub const fn set_ipv0_dr3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "The IPV1DR0 field is defined as a 4-bit field as follows: Bit 3 - Reserved Bit 2:0 - Priority Code Point (PCP)"]
    #[must_use]
    #[inline(always)]
    pub const fn ipv1_dr0(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "The IPV1DR0 field is defined as a 4-bit field as follows: Bit 3 - Reserved Bit 2:0 - Priority Code Point (PCP)"]
    #[inline(always)]
    pub const fn set_ipv1_dr0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "The IPV1DR1 field is defined as a 4-bit field as follows: Bit 3 - Reserved Bit 2:0 - Priority Code Point (PCP)"]
    #[must_use]
    #[inline(always)]
    pub const fn ipv1_dr1(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "The IPV1DR1 field is defined as a 4-bit field as follows: Bit 3 - Reserved Bit 2:0 - Priority Code Point (PCP)"]
    #[inline(always)]
    pub const fn set_ipv1_dr1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "The IPV1DR2 field is defined as a 4-bit field as follows: Bit 3 - Reserved Bit 2:0 - Priority Code Point (PCP)"]
    #[must_use]
    #[inline(always)]
    pub const fn ipv1_dr2(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "The IPV1DR2 field is defined as a 4-bit field as follows: Bit 3 - Reserved Bit 2:0 - Priority Code Point (PCP)"]
    #[inline(always)]
    pub const fn set_ipv1_dr2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "The IPV1DR3 field is defined as a 4-bit field as follows: Bit 3 - Reserved Bit 2:0 - Priority Code Point (PCP)"]
    #[must_use]
    #[inline(always)]
    pub const fn ipv1_dr3(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "The IPV1DR3 field is defined as a 4-bit field as follows: Bit 3 - Reserved Bit 2:0 - Priority Code Point (PCP)"]
    #[inline(always)]
    pub const fn set_ipv1_dr3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Qosvlanmpr {
    #[inline(always)]
    fn default() -> Qosvlanmpr {
        Qosvlanmpr(0u64 as u32)
    }
}
impl core::fmt::Debug for Qosvlanmpr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qosvlanmpr")
            .field("ipv0_dr0", &self.ipv0_dr0())
            .field("ipv0_dr1", &self.ipv0_dr1())
            .field("ipv0_dr2", &self.ipv0_dr2())
            .field("ipv0_dr3", &self.ipv0_dr3())
            .field("ipv1_dr0", &self.ipv1_dr0())
            .field("ipv1_dr1", &self.ipv1_dr1())
            .field("ipv1_dr2", &self.ipv1_dr2())
            .field("ipv1_dr3", &self.ipv1_dr3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qosvlanmpr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Qosvlanmpr {
            ipv0_dr0: u8,
            ipv0_dr1: u8,
            ipv0_dr2: u8,
            ipv0_dr3: u8,
            ipv1_dr0: u8,
            ipv1_dr1: u8,
            ipv1_dr2: u8,
            ipv1_dr3: u8,
        }
        let proxy = Qosvlanmpr {
            ipv0_dr0: self.ipv0_dr0(),
            ipv0_dr1: self.ipv0_dr1(),
            ipv0_dr2: self.ipv0_dr2(),
            ipv0_dr3: self.ipv0_dr3(),
            ipv1_dr0: self.ipv1_dr0(),
            ipv1_dr1: self.ipv1_dr1(),
            ipv1_dr2: self.ipv1_dr2(),
            ipv1_dr3: self.ipv1_dr3(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Switch capability register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scapr0(pub u32);
impl Scapr0 {
    #[doc = "Number of ports supported"]
    #[must_use]
    #[inline(always)]
    pub const fn num_port(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Number of ports supported"]
    #[inline(always)]
    pub const fn set_num_port(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Number of IPVs supported 0: 8 IPV 1: Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn num_ipv(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Number of IPVs supported 0: 8 IPV 1: Reserved"]
    #[inline(always)]
    pub const fn set_num_ipv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Number of MSI-X vectors supported by switch function"]
    #[must_use]
    #[inline(always)]
    pub const fn num_msix(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of MSI-X vectors supported by switch function"]
    #[inline(always)]
    pub const fn set_num_msix(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Number of VLAN PCP to PCP mapping profiles supported"]
    #[must_use]
    #[inline(always)]
    pub const fn num_pcpmp(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of VLAN PCP to PCP mapping profiles supported"]
    #[inline(always)]
    pub const fn set_num_pcpmp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Number of QoS to VLAN PCP mapping profiles supported"]
    #[must_use]
    #[inline(always)]
    pub const fn num_qvmp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of QoS to VLAN PCP mapping profiles supported"]
    #[inline(always)]
    pub const fn set_num_qvmp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for Scapr0 {
    #[inline(always)]
    fn default() -> Scapr0 {
        Scapr0(35979269u64 as u32)
    }
}
impl core::fmt::Debug for Scapr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scapr0")
            .field("num_port", &self.num_port())
            .field("num_ipv", &self.num_ipv())
            .field("num_msix", &self.num_msix())
            .field("num_pcpmp", &self.num_pcpmp())
            .field("num_qvmp", &self.num_qvmp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scapr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Scapr0 {
            num_port: u8,
            num_ipv: bool,
            num_msix: u8,
            num_pcpmp: u8,
            num_qvmp: u8,
        }
        let proxy = Scapr0 {
            num_port: self.num_port(),
            num_ipv: self.num_ipv(),
            num_msix: self.num_msix(),
            num_pcpmp: self.num_pcpmp(),
            num_qvmp: self.num_qvmp(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Switch capability register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scapr1(pub u32);
impl Scapr1 {
    #[doc = "Functional safety capability supported."]
    #[must_use]
    #[inline(always)]
    pub const fn fs(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Functional safety capability supported."]
    #[inline(always)]
    pub const fn set_fs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Cut-through forwarding supported. 0: Not supported 1: Supported"]
    #[must_use]
    #[inline(always)]
    pub const fn ctf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Cut-through forwarding supported. 0: Not supported 1: Supported"]
    #[inline(always)]
    pub const fn set_ctf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Time capture capability supported. 0: Not supported 1: Supported"]
    #[must_use]
    #[inline(always)]
    pub const fn timcap(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Time capture capability supported. 0: Not supported 1: Supported"]
    #[inline(always)]
    pub const fn set_timcap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Ingress mirroring functionality supported. 0: Not supported 1: Supported"]
    #[must_use]
    #[inline(always)]
    pub const fn imir(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Ingress mirroring functionality supported. 0: Not supported 1: Supported"]
    #[inline(always)]
    pub const fn set_imir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Indicates which FRER Sequence Tags are supported xxxx1: 802"]
    #[must_use]
    #[inline(always)]
    pub const fn sq_tags(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Indicates which FRER Sequence Tags are supported xxxx1: 802"]
    #[inline(always)]
    pub const fn set_sq_tags(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for Scapr1 {
    #[inline(always)]
    fn default() -> Scapr1 {
        Scapr1(458780u64 as u32)
    }
}
impl core::fmt::Debug for Scapr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scapr1")
            .field("fs", &self.fs())
            .field("ctf", &self.ctf())
            .field("timcap", &self.timcap())
            .field("imir", &self.imir())
            .field("sq_tags", &self.sq_tags())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scapr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Scapr1 {
            fs: bool,
            ctf: bool,
            timcap: bool,
            imir: bool,
            sq_tags: u8,
        }
        let proxy = Scapr1 {
            fs: self.fs(),
            ctf: self.ctf(),
            timcap: self.timcap(),
            imir: self.imir(),
            sq_tags: self.sq_tags(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Shared memory buffer capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smbcapr(pub u32);
impl Smbcapr {
    #[doc = "Number of words available for the switch frame buffering"]
    #[must_use]
    #[inline(always)]
    pub const fn num_words(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Number of words available for the switch frame buffering"]
    #[inline(always)]
    pub const fn set_num_words(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "Word size in bytes."]
    #[must_use]
    #[inline(always)]
    pub const fn word_size(&self) -> super::vals::WordSize {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::WordSize::from_bits(val as u8)
    }
    #[doc = "Word size in bytes."]
    #[inline(always)]
    pub const fn set_word_size(&mut self, val: super::vals::WordSize) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "Indicates memory location"]
    #[must_use]
    #[inline(always)]
    pub const fn mloc(&self) -> super::vals::Mloc {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::Mloc::from_bits(val as u8)
    }
    #[doc = "Indicates memory location"]
    #[inline(always)]
    pub const fn set_mloc(&mut self, val: super::vals::Mloc) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for Smbcapr {
    #[inline(always)]
    fn default() -> Smbcapr {
        Smbcapr(3138u64 as u32)
    }
}
impl core::fmt::Debug for Smbcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smbcapr")
            .field("num_words", &self.num_words())
            .field("word_size", &self.word_size())
            .field("mloc", &self.mloc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smbcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Smbcapr {
            num_words: u32,
            word_size: super::vals::WordSize,
            mloc: super::vals::Mloc,
        }
        let proxy = Smbcapr {
            num_words: self.num_words(),
            word_size: self.word_size(),
            mloc: self.mloc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Shared memory buffer operational register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smbor0(pub u32);
impl Smbor0 {
    #[doc = "Number of words in use for frame buffering."]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Number of words in use for frame buffering."]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Smbor0 {
    #[inline(always)]
    fn default() -> Smbor0 {
        Smbor0(0u64 as u32)
    }
}
impl core::fmt::Debug for Smbor0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smbor0")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smbor0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Smbor0 {
            count: u32,
        }
        let proxy = Smbor0 {
            count: self.count(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Shared memory buffer operational register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smbor1(pub u32);
impl Smbor1 {
    #[doc = "High watermark of words in use for frame buffering since the last read"]
    #[must_use]
    #[inline(always)]
    pub const fn watermark(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "High watermark of words in use for frame buffering since the last read"]
    #[inline(always)]
    pub const fn set_watermark(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Smbor1 {
    #[inline(always)]
    fn default() -> Smbor1 {
        Smbor1(0u64 as u32)
    }
}
impl core::fmt::Debug for Smbor1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smbor1")
            .field("watermark", &self.watermark())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smbor1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Smbor1 {
            watermark: u32,
        }
        let proxy = Smbor1 {
            watermark: self.watermark(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "VLAN filter hash table capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vfhtcapr(pub u32);
impl Vfhtcapr {
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    #[must_use]
    #[inline(always)]
    pub const fn access_meth(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    #[inline(always)]
    pub const fn set_access_meth(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
}
impl Default for Vfhtcapr {
    #[inline(always)]
    fn default() -> Vfhtcapr {
        Vfhtcapr(11534336u64 as u32)
    }
}
impl core::fmt::Debug for Vfhtcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vfhtcapr")
            .field("access_meth", &self.access_meth())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vfhtcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Vfhtcapr {
            access_meth: u8,
        }
        let proxy = Vfhtcapr {
            access_meth: self.access_meth(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "VLAN Filter (hash) table default entry configuration registers 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vfhtdecr0(pub u32);
impl Vfhtdecr0 {
    #[doc = "Port n"]
    #[must_use]
    #[inline(always)]
    pub const fn port0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port n"]
    #[inline(always)]
    pub const fn set_port0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Port n"]
    #[must_use]
    #[inline(always)]
    pub const fn port1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port n"]
    #[inline(always)]
    pub const fn set_port1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port n"]
    #[must_use]
    #[inline(always)]
    pub const fn port2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port n"]
    #[inline(always)]
    pub const fn set_port2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Port n"]
    #[must_use]
    #[inline(always)]
    pub const fn port3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Port n"]
    #[inline(always)]
    pub const fn set_port3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Port n"]
    #[must_use]
    #[inline(always)]
    pub const fn port4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Port n"]
    #[inline(always)]
    pub const fn set_port4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Spanning Tree Group Member ID"]
    #[must_use]
    #[inline(always)]
    pub const fn stg_id(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Spanning Tree Group Member ID"]
    #[inline(always)]
    pub const fn set_stg_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "IP Multicast Filtering Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ipmfe(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "IP Multicast Filtering Enable"]
    #[inline(always)]
    pub const fn set_ipmfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "IP Multicast Flooding Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ipmfle(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "IP Multicast Flooding Enable"]
    #[inline(always)]
    pub const fn set_ipmfle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Vfhtdecr0 {
    #[inline(always)]
    fn default() -> Vfhtdecr0 {
        Vfhtdecr0(0u64 as u32)
    }
}
impl core::fmt::Debug for Vfhtdecr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vfhtdecr0")
            .field("port0", &self.port0())
            .field("port1", &self.port1())
            .field("port2", &self.port2())
            .field("port3", &self.port3())
            .field("port4", &self.port4())
            .field("stg_id", &self.stg_id())
            .field("ipmfe", &self.ipmfe())
            .field("ipmfle", &self.ipmfle())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vfhtdecr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Vfhtdecr0 {
            port0: bool,
            port1: bool,
            port2: bool,
            port3: bool,
            port4: bool,
            stg_id: u8,
            ipmfe: bool,
            ipmfle: bool,
        }
        let proxy = Vfhtdecr0 {
            port0: self.port0(),
            port1: self.port1(),
            port2: self.port2(),
            port3: self.port3(),
            port4: self.port4(),
            stg_id: self.stg_id(),
            ipmfe: self.ipmfe(),
            ipmfle: self.ipmfle(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "VLAN filter hash table default entry configuration registers 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vfhtdecr1(pub u32);
impl Vfhtdecr1 {
    #[doc = "Filtering ID"]
    #[must_use]
    #[inline(always)]
    pub const fn fid(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Filtering ID"]
    #[inline(always)]
    pub const fn set_fid(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "VLAN Learning Mode 0: Independent VLAN learning: FID is set to the VID assigned to the frame 1: Shared VLAN learning: Use the FID specified in this register Used to determine the FID when doing a lookup in the FDB table"]
    #[must_use]
    #[inline(always)]
    pub const fn vl_mode(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "VLAN Learning Mode 0: Independent VLAN learning: FID is set to the VID assigned to the frame 1: Shared VLAN learning: Use the FID specified in this register Used to determine the FID when doing a lookup in the FDB table"]
    #[inline(always)]
    pub const fn set_vl_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Base Egress Treatment Entry ID Refer to the VLAN Filter table entry BASE_ET_EID field description, for more details"]
    #[must_use]
    #[inline(always)]
    pub const fn base_eteid(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Base Egress Treatment Entry ID Refer to the VLAN Filter table entry BASE_ET_EID field description, for more details"]
    #[inline(always)]
    pub const fn set_base_eteid(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Vfhtdecr1 {
    #[inline(always)]
    fn default() -> Vfhtdecr1 {
        Vfhtdecr1(4294901760u64 as u32)
    }
}
impl core::fmt::Debug for Vfhtdecr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vfhtdecr1")
            .field("fid", &self.fid())
            .field("vl_mode", &self.vl_mode())
            .field("base_eteid", &self.base_eteid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vfhtdecr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Vfhtdecr1 {
            fid: u16,
            vl_mode: bool,
            base_eteid: u16,
        }
        let proxy = Vfhtdecr1 {
            fid: self.fid(),
            vl_mode: self.vl_mode(),
            base_eteid: self.base_eteid(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "VLAN filter hash table default entry configuration registers 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vfhtdecr2(pub u32);
impl Vfhtdecr2 {
    #[doc = "Egress Treatment Applicability Port n."]
    #[must_use]
    #[inline(always)]
    pub const fn et_port0(&self) -> super::vals::EtPort0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::EtPort0::from_bits(val as u8)
    }
    #[doc = "Egress Treatment Applicability Port n."]
    #[inline(always)]
    pub const fn set_et_port0(&mut self, val: super::vals::EtPort0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Egress Treatment Applicability Port n."]
    #[must_use]
    #[inline(always)]
    pub const fn et_port1(&self) -> super::vals::EtPort1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::EtPort1::from_bits(val as u8)
    }
    #[doc = "Egress Treatment Applicability Port n."]
    #[inline(always)]
    pub const fn set_et_port1(&mut self, val: super::vals::EtPort1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Egress Treatment Applicability Port n."]
    #[must_use]
    #[inline(always)]
    pub const fn et_port2(&self) -> super::vals::EtPort2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::EtPort2::from_bits(val as u8)
    }
    #[doc = "Egress Treatment Applicability Port n."]
    #[inline(always)]
    pub const fn set_et_port2(&mut self, val: super::vals::EtPort2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Egress Treatment Applicability Port n."]
    #[must_use]
    #[inline(always)]
    pub const fn et_port3(&self) -> super::vals::EtPort3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::EtPort3::from_bits(val as u8)
    }
    #[doc = "Egress Treatment Applicability Port n."]
    #[inline(always)]
    pub const fn set_et_port3(&mut self, val: super::vals::EtPort3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Egress Treatment Applicability Port n."]
    #[must_use]
    #[inline(always)]
    pub const fn et_port4(&self) -> super::vals::EtPort4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::EtPort4::from_bits(val as u8)
    }
    #[doc = "Egress Treatment Applicability Port n."]
    #[inline(always)]
    pub const fn set_et_port4(&mut self, val: super::vals::EtPort4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "MAC learning options: 0: Reserved 1: Disable MAC learning"]
    #[must_use]
    #[inline(always)]
    pub const fn mlo(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "MAC learning options: 0: Reserved 1: Disable MAC learning"]
    #[inline(always)]
    pub const fn set_mlo(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "MAC forwarding options: 0: Reserved 1: No FDB lookup is performed, the frame is flooded"]
    #[must_use]
    #[inline(always)]
    pub const fn mfo(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "MAC forwarding options: 0: Reserved 1: No FDB lookup is performed, the frame is flooded"]
    #[inline(always)]
    pub const fn set_mfo(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
}
impl Default for Vfhtdecr2 {
    #[inline(always)]
    fn default() -> Vfhtdecr2 {
        Vfhtdecr2(301989888u64 as u32)
    }
}
impl core::fmt::Debug for Vfhtdecr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vfhtdecr2")
            .field("et_port0", &self.et_port0())
            .field("et_port1", &self.et_port1())
            .field("et_port2", &self.et_port2())
            .field("et_port3", &self.et_port3())
            .field("et_port4", &self.et_port4())
            .field("mlo", &self.mlo())
            .field("mfo", &self.mfo())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vfhtdecr2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Vfhtdecr2 {
            et_port0: super::vals::EtPort0,
            et_port1: super::vals::EtPort1,
            et_port2: super::vals::EtPort2,
            et_port3: super::vals::EtPort3,
            et_port4: super::vals::EtPort4,
            mlo: u8,
            mfo: u8,
        }
        let proxy = Vfhtdecr2 {
            et_port0: self.et_port0(),
            et_port1: self.et_port1(),
            et_port2: self.et_port2(),
            et_port3: self.et_port3(),
            et_port4: self.et_port4(),
            mlo: self.mlo(),
            mfo: self.mfo(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "VLAN filter hash table operational register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vfhtor(pub u32);
impl Vfhtor {
    #[doc = "Number of entries in-use by the VLAN Filter table."]
    #[must_use]
    #[inline(always)]
    pub const fn num_entries(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Number of entries in-use by the VLAN Filter table."]
    #[inline(always)]
    pub const fn set_num_entries(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for Vfhtor {
    #[inline(always)]
    fn default() -> Vfhtor {
        Vfhtor(0u64 as u32)
    }
}
impl core::fmt::Debug for Vfhtor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vfhtor")
            .field("num_entries", &self.num_entries())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vfhtor {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Vfhtor {
            num_entries: u16,
        }
        let proxy = Vfhtor {
            num_entries: self.num_entries(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
