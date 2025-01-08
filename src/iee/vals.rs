#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Afd {
    #[doc = "No fault detected"]
    AFD_0 = 0x0,
    #[doc = "Fault detected"]
    AFD_1 = 0x01,
}
impl Afd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Afd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Afd {
    #[inline(always)]
    fn from(val: u8) -> Afd {
        Afd::from_bits(val)
    }
}
impl From<Afd> for u8 {
    #[inline(always)]
    fn from(val: Afd) -> u8 {
        Afd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Byp {
    #[doc = "use MD field"]
    BYP_0 = 0x0,
    #[doc = "Bypass AES, no encrypt/decrypt"]
    BYP_1 = 0x01,
}
impl Byp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Byp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Byp {
    #[inline(always)]
    fn from(val: u8) -> Byp {
        Byp::from_bits(val)
    }
}
impl From<Byp> for u8 {
    #[inline(always)]
    fn from(val: Byp) -> u8 {
        Byp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClrMon {
    #[doc = "Do not reset."]
    CLR_MON_0 = 0x0,
    #[doc = "Reset performance counters."]
    CLR_MON_1 = 0x01,
}
impl ClrMon {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClrMon {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClrMon {
    #[inline(always)]
    fn from(val: u8) -> ClrMon {
        ClrMon::from_bits(val)
    }
}
impl From<ClrMon> for u8 {
    #[inline(always)]
    fn from(val: ClrMon) -> u8 {
        ClrMon::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dsr {
    #[doc = "No seed request present"]
    DSR_0 = 0x0,
    #[doc = "Seed request present"]
    DSR_1 = 0x01,
}
impl Dsr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dsr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dsr {
    #[inline(always)]
    fn from(val: u8) -> Dsr {
        Dsr::from_bits(val)
    }
}
impl From<Dsr> for u8 {
    #[inline(always)]
    fn from(val: Dsr) -> u8 {
        Dsr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KeyRdDis {
    #[doc = "Key read enabled. Reading the key registers is allowed."]
    KEY_RD_DIS_0 = 0x0,
    #[doc = "Key read disabled. Reading the key registers is disabled."]
    KEY_RD_DIS_1 = 0x01,
}
impl KeyRdDis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KeyRdDis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KeyRdDis {
    #[inline(always)]
    fn from(val: u8) -> KeyRdDis {
        KeyRdDis::from_bits(val)
    }
}
impl From<KeyRdDis> for u8 {
    #[inline(always)]
    fn from(val: KeyRdDis) -> u8 {
        KeyRdDis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ks {
    #[doc = "128 bits (CTR), 256 bits (XTS)."]
    KS_0 = 0x0,
    #[doc = "256 bits (CTR), 512 bits (XTS)."]
    KS_1 = 0x01,
}
impl Ks {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ks {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ks {
    #[inline(always)]
    fn from(val: u8) -> Ks {
        Ks::from_bits(val)
    }
}
impl From<Ks> for u8 {
    #[inline(always)]
    fn from(val: Ks) -> u8 {
        Ks::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Md {
    #[doc = "None (AXI error if accessed)"]
    MD_0 = 0x0,
    #[doc = "XTS"]
    MD_1 = 0x01,
    #[doc = "CTR w/ address binding"]
    MD_2 = 0x02,
    #[doc = "CTR w/o address binding"]
    MD_3 = 0x03,
    #[doc = "CTR keystream only"]
    MD_4 = 0x04,
    #[doc = "Undefined, AXI error if used"]
    MD_5 = 0x05,
    #[doc = "Undefined, AXI error if used"]
    MD_6 = 0x06,
    #[doc = "Undefined, AXI error if used"]
    MD_7 = 0x07,
}
impl Md {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Md {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Md {
    #[inline(always)]
    fn from(val: u8) -> Md {
        Md::from_bits(val)
    }
}
impl From<Md> for u8 {
    #[inline(always)]
    fn from(val: Md) -> u8 {
        Md::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MonEn {
    #[doc = "Performance monitoring disabled. Writing of the performance counter registers is enabled."]
    MON_EN_0 = 0x0,
    #[doc = "Performance monitoring enabled. Writing of the performance counter registers is disabled."]
    MON_EN_1 = 0x01,
}
impl MonEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MonEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MonEn {
    #[inline(always)]
    fn from(val: u8) -> MonEn {
        MonEn::from_bits(val)
    }
}
impl From<MonEn> for u8 {
    #[inline(always)]
    fn from(val: MonEn) -> u8 {
        MonEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rl0 {
    #[doc = "Unlocked."]
    RL0_0 = 0x0,
    #[doc = "Key, Offset and Attribute registers are locked."]
    RL0_1 = 0x01,
}
impl Rl0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rl0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rl0 {
    #[inline(always)]
    fn from(val: u8) -> Rl0 {
        Rl0::from_bits(val)
    }
}
impl From<Rl0> for u8 {
    #[inline(always)]
    fn from(val: Rl0) -> u8 {
        Rl0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rl1 {
    #[doc = "Unlocked."]
    RL1_0 = 0x0,
    #[doc = "Key, Offset and Attribute registers are locked."]
    RL1_1 = 0x01,
}
impl Rl1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rl1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rl1 {
    #[inline(always)]
    fn from(val: u8) -> Rl1 {
        Rl1::from_bits(val)
    }
}
impl From<Rl1> for u8 {
    #[inline(always)]
    fn from(val: Rl1) -> u8 {
        Rl1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rl2 {
    #[doc = "Unlocked."]
    RL2_0 = 0x0,
    #[doc = "Key, Offset and Attribute registers are locked."]
    RL2_1 = 0x01,
}
impl Rl2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rl2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rl2 {
    #[inline(always)]
    fn from(val: u8) -> Rl2 {
        Rl2::from_bits(val)
    }
}
impl From<Rl2> for u8 {
    #[inline(always)]
    fn from(val: Rl2) -> u8 {
        Rl2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rl3 {
    #[doc = "Unlocked."]
    RL3_0 = 0x0,
    #[doc = "Key, Offset and Attribute registers are locked."]
    RL3_1 = 0x01,
}
impl Rl3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rl3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rl3 {
    #[inline(always)]
    fn from(val: u8) -> Rl3 {
        Rl3::from_bits(val)
    }
}
impl From<Rl3> for u8 {
    #[inline(always)]
    fn from(val: Rl3) -> u8 {
        Rl3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rl4 {
    #[doc = "Unlocked."]
    RL4_0 = 0x0,
    #[doc = "Key, Offset and Attribute registers are locked."]
    RL4_1 = 0x01,
}
impl Rl4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rl4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rl4 {
    #[inline(always)]
    fn from(val: u8) -> Rl4 {
        Rl4::from_bits(val)
    }
}
impl From<Rl4> for u8 {
    #[inline(always)]
    fn from(val: Rl4) -> u8 {
        Rl4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rl5 {
    #[doc = "Unlocked."]
    RL5_0 = 0x0,
    #[doc = "Key, Offset and Attribute registers are locked."]
    RL5_1 = 0x01,
}
impl Rl5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rl5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rl5 {
    #[inline(always)]
    fn from(val: u8) -> Rl5 {
        Rl5::from_bits(val)
    }
}
impl From<Rl5> for u8 {
    #[inline(always)]
    fn from(val: Rl5) -> u8 {
        Rl5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rl6 {
    #[doc = "Unlocked."]
    RL6_0 = 0x0,
    #[doc = "Key, Offset and Attribute registers are locked."]
    RL6_1 = 0x01,
}
impl Rl6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rl6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rl6 {
    #[inline(always)]
    fn from(val: u8) -> Rl6 {
        Rl6::from_bits(val)
    }
}
impl From<Rl6> for u8 {
    #[inline(always)]
    fn from(val: Rl6) -> u8 {
        Rl6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rl7 {
    #[doc = "Unlocked."]
    RL7_0 = 0x0,
    #[doc = "Key, Offset and Attribute registers are locked."]
    RL7_1 = 0x01,
}
impl Rl7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rl7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rl7 {
    #[inline(always)]
    fn from(val: u8) -> Rl7 {
        Rl7::from_bits(val)
    }
}
impl From<Rl7> for u8 {
    #[inline(always)]
    fn from(val: Rl7) -> u8 {
        Rl7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rst {
    #[doc = "Do Not Reset."]
    RST_0 = 0x0,
    #[doc = "Reset IEE."]
    RST_1 = 0x01,
}
impl Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rst {
    #[inline(always)]
    fn from(val: u8) -> Rst {
        Rst::from_bits(val)
    }
}
impl From<Rst> for u8 {
    #[inline(always)]
    fn from(val: Rst) -> u8 {
        Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmcont {
    #[doc = "Do not continue. This is the last block of data for AES."]
    TMCONT_0 = 0x0,
    #[doc = "Continue. Do not initialize AES after this block."]
    TMCONT_1 = 0x01,
}
impl Tmcont {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmcont {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmcont {
    #[inline(always)]
    fn from(val: u8) -> Tmcont {
        Tmcont::from_bits(val)
    }
}
impl From<Tmcont> for u8 {
    #[inline(always)]
    fn from(val: Tmcont) -> u8 {
        Tmcont::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmd {
    #[doc = "Test mode is usable."]
    TMD_0 = 0x0,
    #[doc = "Test mode is disabled."]
    TMD_1 = 0x01,
}
impl Tmd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmd {
    #[inline(always)]
    fn from(val: u8) -> Tmd {
        Tmd::from_bits(val)
    }
}
impl From<Tmd> for u8 {
    #[inline(always)]
    fn from(val: Tmd) -> u8 {
        Tmd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmdone {
    #[doc = "Not Done."]
    TMDONE_0 = 0x0,
    #[doc = "Test Done."]
    TMDONE_1 = 0x01,
}
impl Tmdone {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmdone {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmdone {
    #[inline(always)]
    fn from(val: u8) -> Tmdone {
        Tmdone::from_bits(val)
    }
}
impl From<Tmdone> for u8 {
    #[inline(always)]
    fn from(val: Tmdone) -> u8 {
        Tmdone::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tme {
    #[doc = "Disabled."]
    TME_0 = 0x0,
    #[doc = "Enabled."]
    TME_1 = 0x01,
}
impl Tme {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tme {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tme {
    #[inline(always)]
    fn from(val: u8) -> Tme {
        Tme::from_bits(val)
    }
}
impl From<Tme> for u8 {
    #[inline(always)]
    fn from(val: Tme) -> u8 {
        Tme::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmencr {
    #[doc = "AES Test mode will do decryption."]
    TMENCR_0 = 0x0,
    #[doc = "AES Test mode will do encryption."]
    TMENCR_1 = 0x01,
}
impl Tmencr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmencr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmencr {
    #[inline(always)]
    fn from(val: u8) -> Tmencr {
        Tmencr::from_bits(val)
    }
}
impl From<Tmencr> for u8 {
    #[inline(always)]
    fn from(val: Tmencr) -> u8 {
        Tmencr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmr {
    #[doc = "Not running. May be written if IEE_GCFG\\[TME\\] = 1"]
    TMR_0 = 0x0,
    #[doc = "Run AES Test until TMDONE is indicated."]
    TMR_1 = 0x01,
}
impl Tmr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmr {
    #[inline(always)]
    fn from(val: u8) -> Tmr {
        Tmr::from_bits(val)
    }
}
impl From<Tmr> for u8 {
    #[inline(always)]
    fn from(val: Tmr) -> u8 {
        Tmr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmrdy {
    #[doc = "Not Ready."]
    TMRDY_0 = 0x0,
    #[doc = "Ready."]
    TMRDY_1 = 0x01,
}
impl Tmrdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmrdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmrdy {
    #[inline(always)]
    fn from(val: u8) -> Tmrdy {
        Tmrdy::from_bits(val)
    }
}
impl From<Tmrdy> for u8 {
    #[inline(always)]
    fn from(val: Tmrdy) -> u8 {
        Tmrdy::to_bits(val)
    }
}
