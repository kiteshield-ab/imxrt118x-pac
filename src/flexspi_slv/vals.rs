#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Allowaxird {
    #[doc = "Denied"]
    DENY = 0x0,
    #[doc = "Allowed"]
    ALLOW = 0x01,
}
impl Allowaxird {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Allowaxird {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Allowaxird {
    #[inline(always)]
    fn from(val: u8) -> Allowaxird {
        Allowaxird::from_bits(val)
    }
}
impl From<Allowaxird> for u8 {
    #[inline(always)]
    fn from(val: Allowaxird) -> u8 {
        Allowaxird::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Allowaxiwr {
    #[doc = "Denied"]
    DENY = 0x0,
    #[doc = "Allowed"]
    ALLOW = 0x01,
}
impl Allowaxiwr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Allowaxiwr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Allowaxiwr {
    #[inline(always)]
    fn from(val: u8) -> Allowaxiwr {
        Allowaxiwr::from_bits(val)
    }
}
impl From<Allowaxiwr> for u8 {
    #[inline(always)]
    fn from(val: Allowaxiwr) -> u8 {
        Allowaxiwr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Axireadidle {
    #[doc = "Busy"]
    BUSY = 0x0,
    #[doc = "Idle"]
    IDLE = 0x01,
}
impl Axireadidle {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Axireadidle {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Axireadidle {
    #[inline(always)]
    fn from(val: u8) -> Axireadidle {
        Axireadidle::from_bits(val)
    }
}
impl From<Axireadidle> for u8 {
    #[inline(always)]
    fn from(val: Axireadidle) -> u8 {
        Axireadidle::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Blknxtrd {
    #[doc = "Allowed"]
    ALLOW = 0x0,
    #[doc = "Blocked"]
    BLOCK = 0x01,
}
impl Blknxtrd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Blknxtrd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Blknxtrd {
    #[inline(always)]
    fn from(val: u8) -> Blknxtrd {
        Blknxtrd::from_bits(val)
    }
}
impl From<Blknxtrd> for u8 {
    #[inline(always)]
    fn from(val: Blknxtrd) -> u8 {
        Blknxtrd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Blknxtwr {
    #[doc = "Allowed"]
    ALLOW = 0x0,
    #[doc = "Blocked"]
    BLOCK = 0x01,
}
impl Blknxtwr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Blknxtwr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Blknxtwr {
    #[inline(always)]
    fn from(val: u8) -> Blknxtwr {
        Blknxtwr::from_bits(val)
    }
}
impl From<Blknxtwr> for u8 {
    #[inline(always)]
    fn from(val: Blknxtwr) -> u8 {
        Blknxtwr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Blkread {
    #[doc = "Allowed"]
    ALLOW = 0x0,
    #[doc = "Blocked"]
    BLOCK = 0x01,
}
impl Blkread {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Blkread {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Blkread {
    #[inline(always)]
    fn from(val: u8) -> Blkread {
        Blkread::from_bits(val)
    }
}
impl From<Blkread> for u8 {
    #[inline(always)]
    fn from(val: Blkread) -> u8 {
        Blkread::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Blkwrite {
    #[doc = "Allowed"]
    ALLOW = 0x0,
    #[doc = "Blocked"]
    BLOCK = 0x01,
}
impl Blkwrite {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Blkwrite {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Blkwrite {
    #[inline(always)]
    fn from(val: u8) -> Blkwrite {
        Blkwrite::from_bits(val)
    }
}
impl From<Blkwrite> for u8 {
    #[inline(always)]
    fn from(val: Blkwrite) -> u8 {
        Blkwrite::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrint {
    #[doc = "Do not clear"]
    NONE = 0x0,
    #[doc = "Clear"]
    CLEAR = 0x01,
}
impl Clrint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrint {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrint {
    #[inline(always)]
    fn from(val: u8) -> Clrint {
        Clrint::from_bits(val)
    }
}
impl From<Clrint> for u8 {
    #[inline(always)]
    fn from(val: Clrint) -> u8 {
        Clrint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Errcmd {
    #[doc = "Not received"]
    NONE = 0x0,
    #[doc = "Received"]
    ERROR = 0x01,
}
impl Errcmd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Errcmd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Errcmd {
    #[inline(always)]
    fn from(val: u8) -> Errcmd {
        Errcmd::from_bits(val)
    }
}
impl From<Errcmd> for u8 {
    #[inline(always)]
    fn from(val: Errcmd) -> u8 {
        Errcmd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iomode {
    #[doc = "SDR*4"]
    SDRX4 = 0x0,
    #[doc = "SDR*8"]
    SDRX8 = 0x01,
    #[doc = "DDR*4"]
    DDRX4 = 0x02,
    #[doc = "DDR*8"]
    DDRX8 = 0x03,
}
impl Iomode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iomode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iomode {
    #[inline(always)]
    fn from(val: u8) -> Iomode {
        Iomode::from_bits(val)
    }
}
impl From<Iomode> for u8 {
    #[inline(always)]
    fn from(val: Iomode) -> u8 {
        Iomode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rdfetchsize {
    #[doc = "256 bytes"]
    SIZE256 = 0x0,
    #[doc = "512 bytes"]
    SIZE512 = 0x01,
    #[doc = "1 KB"]
    SIZE1K = 0x02,
    #[doc = "2 KB"]
    SIZE2K = 0x03,
}
impl Rdfetchsize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rdfetchsize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rdfetchsize {
    #[inline(always)]
    fn from(val: u8) -> Rdfetchsize {
        Rdfetchsize::from_bits(val)
    }
}
impl From<Rdfetchsize> for u8 {
    #[inline(always)]
    fn from(val: Rdfetchsize) -> u8 {
        Rdfetchsize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Regrwidle {
    #[doc = "Busy"]
    BUSY = 0x0,
    #[doc = "Idle"]
    IDLE = 0x01,
}
impl Regrwidle {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Regrwidle {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Regrwidle {
    #[inline(always)]
    fn from(val: u8) -> Regrwidle {
        Regrwidle::from_bits(val)
    }
}
impl From<Regrwidle> for u8 {
    #[inline(always)]
    fn from(val: Regrwidle) -> u8 {
        Regrwidle::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ruf {
    #[doc = "Did not occur"]
    NONE = 0x0,
    #[doc = "Occurred"]
    UNDERFLOW = 0x01,
}
impl Ruf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ruf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ruf {
    #[inline(always)]
    fn from(val: u8) -> Ruf {
        Ruf::from_bits(val)
    }
}
impl From<Ruf> for u8 {
    #[inline(always)]
    fn from(val: Ruf) -> u8 {
        Ruf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Seqidle {
    #[doc = "Busy"]
    BUSY = 0x0,
    #[doc = "Idle"]
    IDLE = 0x01,
}
impl Seqidle {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Seqidle {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Seqidle {
    #[inline(always)]
    fn from(val: u8) -> Seqidle {
        Seqidle::from_bits(val)
    }
}
impl From<Seqidle> for u8 {
    #[inline(always)]
    fn from(val: Seqidle) -> u8 {
        Seqidle::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swreset {
    #[doc = "Finished"]
    FINISH = 0x0,
    #[doc = "Initiate"]
    INITIATE = 0x01,
}
impl Swreset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swreset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swreset {
    #[inline(always)]
    fn from(val: u8) -> Swreset {
        Swreset::from_bits(val)
    }
}
impl From<Swreset> for u8 {
    #[inline(always)]
    fn from(val: Swreset) -> u8 {
        Swreset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wip {
    #[doc = "Not busy"]
    IDLE = 0x0,
    #[doc = "Busy"]
    BUSY = 0x01,
}
impl Wip {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wip {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wip {
    #[inline(always)]
    fn from(val: u8) -> Wip {
        Wip::from_bits(val)
    }
}
impl From<Wip> for u8 {
    #[inline(always)]
    fn from(val: Wip) -> u8 {
        Wip::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wof {
    #[doc = "Did not occur"]
    NONE = 0x0,
    #[doc = "Occurred"]
    OVERFLOW = 0x01,
}
impl Wof {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wof {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wof {
    #[inline(always)]
    fn from(val: u8) -> Wof {
        Wof::from_bits(val)
    }
}
impl From<Wof> for u8 {
    #[inline(always)]
    fn from(val: Wof) -> u8 {
        Wof::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wrwm {
    #[doc = "32 bytes"]
    SIZE32 = 0x0,
    #[doc = "64 bytes"]
    SIZE64 = 0x01,
    #[doc = "128 bytes"]
    SIZE128 = 0x02,
    #[doc = "256 bytes"]
    SIZE256 = 0x03,
}
impl Wrwm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wrwm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wrwm {
    #[inline(always)]
    fn from(val: u8) -> Wrwm {
        Wrwm::from_bits(val)
    }
}
impl From<Wrwm> for u8 {
    #[inline(always)]
    fn from(val: Wrwm) -> u8 {
        Wrwm::to_bits(val)
    }
}
