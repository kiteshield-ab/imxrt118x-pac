#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dfmt {
    #[doc = "Processor-core domain assignment"]
    Cpu = 0x0,
    #[doc = "Non-processor domain assignment"]
    Noncpu = 0x01,
}
impl Dfmt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dfmt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dfmt {
    #[inline(always)]
    fn from(val: u8) -> Dfmt {
        Dfmt::from_bits(val)
    }
}
impl From<Dfmt> for u8 {
    #[inline(always)]
    fn from(val: Dfmt) -> u8 {
        Dfmt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Didb {
    #[doc = "Use MDAn\\[3:0\\] as the domain identifier."]
    Reg = 0x0,
    #[doc = "Use the DID input as the domain identifier."]
    Input = 0x01,
}
impl Didb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Didb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Didb {
    #[inline(always)]
    fn from(val: u8) -> Didb {
        Didb::from_bits(val)
    }
}
impl From<Didb> for u8 {
    #[inline(always)]
    fn from(val: Didb) -> u8 {
        Didb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dids {
    #[doc = "Use MDAm\\[3:0\\] as the domain identifier."]
    Zero = 0x0,
    #[doc = "Use the input DID as the domain identifier."]
    One = 0x01,
    #[doc = "Use MDAm\\[3:2\\] concatenated with the low-order 2 bits of the input DID (DID_in\\[1:0\\]) as the domain identifier."]
    Two = 0x02,
    _RESERVED_3 = 0x03,
}
impl Dids {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dids {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dids {
    #[inline(always)]
    fn from(val: u8) -> Dids {
        Dids::from_bits(val)
    }
}
impl From<Dids> for u8 {
    #[inline(always)]
    fn from(val: Dids) -> u8 {
        Dids::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Eatr {
    #[doc = "Secure user mode, instruction fetch access."]
    Sui = 0x0,
    #[doc = "Secure user mode, data access."]
    Sud = 0x01,
    #[doc = "Secure privileged mode, instruction fetch access."]
    Spi = 0x02,
    #[doc = "Secure privileged mode, data access."]
    Spd = 0x03,
    #[doc = "Nonsecure user mode, instruction fetch access."]
    Nsui = 0x04,
    #[doc = "Nonsecure user mode, data access."]
    Nsud = 0x05,
    #[doc = "Nonsecure privileged mode, instruction fetch access."]
    Nspi = 0x06,
    #[doc = "Nonsecure privileged mode, data access."]
    Nspd = 0x07,
}
impl Eatr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eatr {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eatr {
    #[inline(always)]
    fn from(val: u8) -> Eatr {
        Eatr::from_bits(val)
    }
}
impl From<Eatr> for u8 {
    #[inline(always)]
    fn from(val: Eatr) -> u8 {
        Eatr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Eport {
    #[doc = "mbcxslv0"]
    S0 = 0x0,
    #[doc = "mbcxslv1"]
    S1 = 0x01,
    #[doc = "mbcxslv2"]
    S2 = 0x02,
    #[doc = "mbcxslv3"]
    S3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Eport {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eport {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eport {
    #[inline(always)]
    fn from(val: u8) -> Eport {
        Eport::from_bits(val)
    }
}
impl From<Eport> for u8 {
    #[inline(always)]
    fn from(val: Eport) -> u8 {
        Eport::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Erw {
    #[doc = "Read access"]
    Read = 0x0,
    #[doc = "Write access"]
    Write = 0x01,
}
impl Erw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Erw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Erw {
    #[inline(always)]
    fn from(val: u8) -> Erw {
        Erw::from_bits(val)
    }
}
impl From<Erw> for u8 {
    #[inline(always)]
    fn from(val: Erw) -> u8 {
        Erw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Est {
    #[doc = "No access violation has been detected."]
    Novio0 = 0x0,
    #[doc = "No access violation has been detected."]
    Novio1 = 0x01,
    #[doc = "A single access violation has been detected."]
    Single = 0x02,
    #[doc = "Multiple access violations for this domain have been detected by this submodule instance. Only the address and attribute information for the first error have been captured in DERR_W0_i and DERR_W1_i."]
    Multi = 0x03,
}
impl Est {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Est {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Est {
    #[inline(always)]
    fn from(val: u8) -> Est {
        Est::from_bits(val)
    }
}
impl From<Est> for u8 {
    #[inline(always)]
    fn from(val: Est) -> u8 {
        Est::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lk {
    #[doc = "Register can be written by any secure privileged write."]
    Zero = 0x0,
    #[doc = "Register can be written by any secure privileged write."]
    One = 0x01,
    #[doc = "Register can only be written by a secure privileged write from the bus master that locked the register."]
    Two = 0x02,
    #[doc = "Register is locked (read-only) until the next reset."]
    Three = 0x03,
}
impl Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lk {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lk {
    #[inline(always)]
    fn from(val: u8) -> Lk {
        Lk::from_bits(val)
    }
}
impl From<Lk> for u8 {
    #[inline(always)]
    fn from(val: Lk) -> u8 {
        Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbacsel {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    Glbac7 = 0x07,
}
impl Mbacsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbacsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbacsel {
    #[inline(always)]
    fn from(val: u8) -> Mbacsel {
        Mbacsel::from_bits(val)
    }
}
impl From<Mbacsel> for u8 {
    #[inline(always)]
    fn from(val: Mbacsel) -> u8 {
        Mbacsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mracsel {
    #[doc = "Select MRC_GLBAC0 access control policy"]
    Sel0 = 0x0,
    #[doc = "Select MRC_GLBAC1 access control policy"]
    Sel1 = 0x01,
    #[doc = "Select MRC_GLBAC2 access control policy"]
    Sel2 = 0x02,
    #[doc = "Select MRC_GLBAC3 access control policy"]
    Sel3 = 0x03,
    #[doc = "Select MRC_GLBAC4 access control policy"]
    Sel4 = 0x04,
    #[doc = "Select MRC_GLBAC5 access control policy"]
    Sel5 = 0x05,
    #[doc = "Select MRC_GLBAC6 access control policy"]
    Sel6 = 0x06,
    #[doc = "Select MRC_GLBAC7 access control policy"]
    Sel7 = 0x07,
}
impl Mracsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mracsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mracsel {
    #[inline(always)]
    fn from(val: u8) -> Mracsel {
        Mracsel::from_bits(val)
    }
}
impl From<Mracsel> for u8 {
    #[inline(always)]
    fn from(val: Mracsel) -> u8 {
        Mracsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ncm {
    #[doc = "Bus master is a processor."]
    Cpu = 0x0,
    #[doc = "Bus master is a non-processor."]
    NonCpu = 0x01,
}
impl Ncm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ncm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ncm {
    #[inline(always)]
    fn from(val: u8) -> Ncm {
        Ncm::from_bits(val)
    }
}
impl From<Ncm> for u8 {
    #[inline(always)]
    fn from(val: Ncm) -> u8 {
        Ncm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pa {
    #[doc = "Force the bus attribute for this master to user."]
    Zero = 0x0,
    #[doc = "Force the bus attribute for this master to privileged."]
    One = 0x01,
    #[doc = "Use the bus master's privileged/user attribute directly."]
    Two = 0x02,
    #[doc = "Use the bus master's privileged/user attribute directly."]
    Three = 0x03,
}
impl Pa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pa {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pa {
    #[inline(always)]
    fn from(val: u8) -> Pa {
        Pa::from_bits(val)
    }
}
impl From<Pa> for u8 {
    #[inline(always)]
    fn from(val: Pa) -> u8 {
        Pa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcurrns {
    #[doc = "Processor is in Secure state"]
    Secure = 0x0,
    #[doc = "Processor is in Nonsecure state"]
    Nonsecure = 0x01,
}
impl Pcurrns {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcurrns {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcurrns {
    #[inline(always)]
    fn from(val: u8) -> Pcurrns {
        Pcurrns::from_bits(val)
    }
}
impl From<Pcurrns> for u8 {
    #[inline(always)]
    fn from(val: Pcurrns) -> u8 {
        Pcurrns::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pe {
    #[doc = "No process identifier is included in the domain hit evaluation."]
    Zero = 0x0,
    #[doc = "No process identifier is included in the domain hit evaluation."]
    One = 0x01,
    #[doc = "PE = 2"]
    Two = 0x02,
    #[doc = "PE = 3"]
    Three = 0x03,
}
impl Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pe {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pe {
    #[inline(always)]
    fn from(val: u8) -> Pe {
        Pe::from_bits(val)
    }
}
impl From<Pe> for u8 {
    #[inline(always)]
    fn from(val: Pe) -> u8 {
        Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sa {
    #[doc = "Force the bus attribute for this master to secure."]
    Sec = 0x0,
    #[doc = "Force the bus attribute for this master to nonsecure."]
    Nonsec = 0x01,
    #[doc = "Use the bus master's secure/nonsecure attribute directly."]
    Src0 = 0x02,
    #[doc = "Use the bus master's secure/nonsecure attribute directly."]
    Src1 = 0x03,
}
impl Sa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sa {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sa {
    #[inline(always)]
    fn from(val: u8) -> Sa {
        Sa::from_bits(val)
    }
}
impl From<Sa> for u8 {
    #[inline(always)]
    fn from(val: Sa) -> u8 {
        Sa::to_bits(val)
    }
}
