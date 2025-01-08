#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ep {
    #[doc = "Not pending"]
    ZERO = 0x0,
    #[doc = "Pending"]
    ONE = 0x01,
}
impl Ep {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ep {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ep {
    #[inline(always)]
    fn from(val: u8) -> Ep {
        Ep::from_bits(val)
    }
}
impl From<Ep> for u8 {
    #[inline(always)]
    fn from(val: Ep) -> u8 {
        Ep::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FcrF0 {
    #[doc = "Clear MUB_FSR\\[Fn\\]"]
    ZERO = 0x0,
    #[doc = "Set MUB_FSR\\[Fn\\]"]
    ONE = 0x01,
}
impl FcrF0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FcrF0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FcrF0 {
    #[inline(always)]
    fn from(val: u8) -> FcrF0 {
        FcrF0::from_bits(val)
    }
}
impl From<FcrF0> for u8 {
    #[inline(always)]
    fn from(val: FcrF0) -> u8 {
        FcrF0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FcrF1 {
    #[doc = "Clear MUB_FSR\\[Fn\\]"]
    ZERO = 0x0,
    #[doc = "Set MUB_FSR\\[Fn\\]"]
    ONE = 0x01,
}
impl FcrF1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FcrF1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FcrF1 {
    #[inline(always)]
    fn from(val: u8) -> FcrF1 {
        FcrF1::from_bits(val)
    }
}
impl From<FcrF1> for u8 {
    #[inline(always)]
    fn from(val: FcrF1) -> u8 {
        FcrF1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FcrF2 {
    #[doc = "Clear MUB_FSR\\[Fn\\]"]
    ZERO = 0x0,
    #[doc = "Set MUB_FSR\\[Fn\\]"]
    ONE = 0x01,
}
impl FcrF2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FcrF2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FcrF2 {
    #[inline(always)]
    fn from(val: u8) -> FcrF2 {
        FcrF2::from_bits(val)
    }
}
impl From<FcrF2> for u8 {
    #[inline(always)]
    fn from(val: FcrF2) -> u8 {
        FcrF2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FsrF0 {
    #[doc = "MUB_FCR\\[Fn\\] = 0"]
    ZERO = 0x0,
    #[doc = "MUB_FCR\\[Fn\\] = 1"]
    ONE = 0x01,
}
impl FsrF0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FsrF0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FsrF0 {
    #[inline(always)]
    fn from(val: u8) -> FsrF0 {
        FsrF0::from_bits(val)
    }
}
impl From<FsrF0> for u8 {
    #[inline(always)]
    fn from(val: FsrF0) -> u8 {
        FsrF0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FsrF1 {
    #[doc = "MUB_FCR\\[Fn\\] = 0"]
    ZERO = 0x0,
    #[doc = "MUB_FCR\\[Fn\\] = 1"]
    ONE = 0x01,
}
impl FsrF1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FsrF1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FsrF1 {
    #[inline(always)]
    fn from(val: u8) -> FsrF1 {
        FsrF1::from_bits(val)
    }
}
impl From<FsrF1> for u8 {
    #[inline(always)]
    fn from(val: FsrF1) -> u8 {
        FsrF1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FsrF2 {
    #[doc = "MUB_FCR\\[Fn\\] = 0"]
    ZERO = 0x0,
    #[doc = "MUB_FCR\\[Fn\\] = 1"]
    ONE = 0x01,
}
impl FsrF2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FsrF2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FsrF2 {
    #[inline(always)]
    fn from(val: u8) -> FsrF2 {
        FsrF2::from_bits(val)
    }
}
impl From<FsrF2> for u8 {
    #[inline(always)]
    fn from(val: FsrF2) -> u8 {
        FsrF2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fup {
    #[doc = "No pending update flags (initiated by MUA)"]
    ZERO = 0x0,
    #[doc = "Pending update flags (initiated by MUA)"]
    ONE = 0x01,
}
impl Fup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fup {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fup {
    #[inline(always)]
    fn from(val: u8) -> Fup {
        Fup::from_bits(val)
    }
}
impl From<Fup> for u8 {
    #[inline(always)]
    fn from(val: Fup) -> u8 {
        Fup::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gie0 {
    #[doc = "Disable"]
    ZERO = 0x0,
    #[doc = "Enable"]
    ONE = 0x01,
}
impl Gie0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gie0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gie0 {
    #[inline(always)]
    fn from(val: u8) -> Gie0 {
        Gie0::from_bits(val)
    }
}
impl From<Gie0> for u8 {
    #[inline(always)]
    fn from(val: Gie0) -> u8 {
        Gie0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gie1 {
    #[doc = "Disable"]
    ZERO = 0x0,
    #[doc = "Enable"]
    ONE = 0x01,
}
impl Gie1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gie1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gie1 {
    #[inline(always)]
    fn from(val: u8) -> Gie1 {
        Gie1::from_bits(val)
    }
}
impl From<Gie1> for u8 {
    #[inline(always)]
    fn from(val: Gie1) -> u8 {
        Gie1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gie2 {
    #[doc = "Disable"]
    ZERO = 0x0,
    #[doc = "Enable"]
    ONE = 0x01,
}
impl Gie2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gie2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gie2 {
    #[inline(always)]
    fn from(val: u8) -> Gie2 {
        Gie2::from_bits(val)
    }
}
impl From<Gie2> for u8 {
    #[inline(always)]
    fn from(val: Gie2) -> u8 {
        Gie2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gie3 {
    #[doc = "Disable"]
    ZERO = 0x0,
    #[doc = "Enable"]
    ONE = 0x01,
}
impl Gie3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gie3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gie3 {
    #[inline(always)]
    fn from(val: u8) -> Gie3 {
        Gie3::from_bits(val)
    }
}
impl From<Gie3> for u8 {
    #[inline(always)]
    fn from(val: Gie3) -> u8 {
        Gie3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gip0 {
    #[doc = "Not pending"]
    ZERO = 0x0,
    #[doc = "Pending"]
    ONE = 0x01,
}
impl Gip0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gip0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gip0 {
    #[inline(always)]
    fn from(val: u8) -> Gip0 {
        Gip0::from_bits(val)
    }
}
impl From<Gip0> for u8 {
    #[inline(always)]
    fn from(val: Gip0) -> u8 {
        Gip0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gip1 {
    #[doc = "Not pending"]
    ZERO = 0x0,
    #[doc = "Pending"]
    ONE = 0x01,
}
impl Gip1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gip1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gip1 {
    #[inline(always)]
    fn from(val: u8) -> Gip1 {
        Gip1::from_bits(val)
    }
}
impl From<Gip1> for u8 {
    #[inline(always)]
    fn from(val: Gip1) -> u8 {
        Gip1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gip2 {
    #[doc = "Not pending"]
    ZERO = 0x0,
    #[doc = "Pending"]
    ONE = 0x01,
}
impl Gip2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gip2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gip2 {
    #[inline(always)]
    fn from(val: u8) -> Gip2 {
        Gip2::from_bits(val)
    }
}
impl From<Gip2> for u8 {
    #[inline(always)]
    fn from(val: Gip2) -> u8 {
        Gip2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gip3 {
    #[doc = "Not pending"]
    ZERO = 0x0,
    #[doc = "Pending"]
    ONE = 0x01,
}
impl Gip3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gip3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gip3 {
    #[inline(always)]
    fn from(val: u8) -> Gip3 {
        Gip3::from_bits(val)
    }
}
impl From<Gip3> for u8 {
    #[inline(always)]
    fn from(val: Gip3) -> u8 {
        Gip3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gir0 {
    #[doc = "Not requested"]
    ZERO = 0x0,
    #[doc = "Requested"]
    ONE = 0x01,
}
impl Gir0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gir0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gir0 {
    #[inline(always)]
    fn from(val: u8) -> Gir0 {
        Gir0::from_bits(val)
    }
}
impl From<Gir0> for u8 {
    #[inline(always)]
    fn from(val: Gir0) -> u8 {
        Gir0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gir1 {
    #[doc = "Not requested"]
    ZERO = 0x0,
    #[doc = "Requested"]
    ONE = 0x01,
}
impl Gir1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gir1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gir1 {
    #[inline(always)]
    fn from(val: u8) -> Gir1 {
        Gir1::from_bits(val)
    }
}
impl From<Gir1> for u8 {
    #[inline(always)]
    fn from(val: Gir1) -> u8 {
        Gir1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gir2 {
    #[doc = "Not requested"]
    ZERO = 0x0,
    #[doc = "Requested"]
    ONE = 0x01,
}
impl Gir2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gir2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gir2 {
    #[inline(always)]
    fn from(val: u8) -> Gir2 {
        Gir2::from_bits(val)
    }
}
impl From<Gir2> for u8 {
    #[inline(always)]
    fn from(val: Gir2) -> u8 {
        Gir2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gir3 {
    #[doc = "Not requested"]
    ZERO = 0x0,
    #[doc = "Requested"]
    ONE = 0x01,
}
impl Gir3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gir3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gir3 {
    #[inline(always)]
    fn from(val: u8) -> Gir3 {
        Gir3::from_bits(val)
    }
}
impl From<Gir3> for u8 {
    #[inline(always)]
    fn from(val: Gir3) -> u8 {
        Gir3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Girp {
    #[doc = "No request sent"]
    ZERO = 0x0,
    #[doc = "Request sent"]
    ONE = 0x01,
}
impl Girp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Girp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Girp {
    #[inline(always)]
    fn from(val: u8) -> Girp {
        Girp::from_bits(val)
    }
}
impl From<Girp> for u8 {
    #[inline(always)]
    fn from(val: Girp) -> u8 {
        Girp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mur {
    #[doc = "Idle"]
    ZERO = 0x0,
    #[doc = "Reset"]
    ONE = 0x01,
}
impl Mur {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mur {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mur {
    #[inline(always)]
    fn from(val: u8) -> Mur {
        Mur::from_bits(val)
    }
}
impl From<Mur> for u8 {
    #[inline(always)]
    fn from(val: Mur) -> u8 {
        Mur::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Murie {
    #[doc = "Disable"]
    ZERO = 0x0,
    #[doc = "Enable"]
    ONE = 0x01,
}
impl Murie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Murie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Murie {
    #[inline(always)]
    fn from(val: u8) -> Murie {
        Murie::from_bits(val)
    }
}
impl From<Murie> for u8 {
    #[inline(always)]
    fn from(val: Murie) -> u8 {
        Murie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Murip {
    #[doc = "Reset not issued"]
    ZERO = 0x0,
    #[doc = "Reset issued"]
    ONE = 0x01,
}
impl Murip {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Murip {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Murip {
    #[inline(always)]
    fn from(val: u8) -> Murip {
        Murip::from_bits(val)
    }
}
impl From<Murip> for u8 {
    #[inline(always)]
    fn from(val: Murip) -> u8 {
        Murip::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Murs {
    #[doc = "Out of reset"]
    ZERO = 0x0,
    #[doc = "In reset"]
    ONE = 0x01,
}
impl Murs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Murs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Murs {
    #[inline(always)]
    fn from(val: u8) -> Murs {
        Murs::from_bits(val)
    }
}
impl From<Murs> for u8 {
    #[inline(always)]
    fn from(val: Murs) -> u8 {
        Murs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rf0 {
    #[doc = "Not full"]
    ZERO = 0x0,
    #[doc = "Full"]
    ONE = 0x01,
}
impl Rf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rf0 {
    #[inline(always)]
    fn from(val: u8) -> Rf0 {
        Rf0::from_bits(val)
    }
}
impl From<Rf0> for u8 {
    #[inline(always)]
    fn from(val: Rf0) -> u8 {
        Rf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rf1 {
    #[doc = "Not full"]
    ZERO = 0x0,
    #[doc = "Full"]
    ONE = 0x01,
}
impl Rf1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rf1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rf1 {
    #[inline(always)]
    fn from(val: u8) -> Rf1 {
        Rf1::from_bits(val)
    }
}
impl From<Rf1> for u8 {
    #[inline(always)]
    fn from(val: Rf1) -> u8 {
        Rf1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rf2 {
    #[doc = "Not full"]
    ZERO = 0x0,
    #[doc = "Full"]
    ONE = 0x01,
}
impl Rf2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rf2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rf2 {
    #[inline(always)]
    fn from(val: u8) -> Rf2 {
        Rf2::from_bits(val)
    }
}
impl From<Rf2> for u8 {
    #[inline(always)]
    fn from(val: Rf2) -> u8 {
        Rf2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rf3 {
    #[doc = "Not full"]
    ZERO = 0x0,
    #[doc = "Full"]
    ONE = 0x01,
}
impl Rf3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rf3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rf3 {
    #[inline(always)]
    fn from(val: u8) -> Rf3 {
        Rf3::from_bits(val)
    }
}
impl From<Rf3> for u8 {
    #[inline(always)]
    fn from(val: Rf3) -> u8 {
        Rf3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rfp {
    #[doc = "Not pending; MUB is not writing to a Transmit register"]
    ZERO = 0x0,
    #[doc = "Pending; MUB is writing to a Transmit register"]
    ONE = 0x01,
}
impl Rfp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rfp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rfp {
    #[inline(always)]
    fn from(val: u8) -> Rfp {
        Rfp::from_bits(val)
    }
}
impl From<Rfp> for u8 {
    #[inline(always)]
    fn from(val: Rfp) -> u8 {
        Rfp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rie0 {
    #[doc = "Disable"]
    ZERO = 0x0,
    #[doc = "Enable"]
    ONE = 0x01,
}
impl Rie0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rie0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rie0 {
    #[inline(always)]
    fn from(val: u8) -> Rie0 {
        Rie0::from_bits(val)
    }
}
impl From<Rie0> for u8 {
    #[inline(always)]
    fn from(val: Rie0) -> u8 {
        Rie0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rie1 {
    #[doc = "Disable"]
    ZERO = 0x0,
    #[doc = "Enable"]
    ONE = 0x01,
}
impl Rie1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rie1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rie1 {
    #[inline(always)]
    fn from(val: u8) -> Rie1 {
        Rie1::from_bits(val)
    }
}
impl From<Rie1> for u8 {
    #[inline(always)]
    fn from(val: Rie1) -> u8 {
        Rie1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rie2 {
    #[doc = "Disable"]
    ZERO = 0x0,
    #[doc = "Enable"]
    ONE = 0x01,
}
impl Rie2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rie2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rie2 {
    #[inline(always)]
    fn from(val: u8) -> Rie2 {
        Rie2::from_bits(val)
    }
}
impl From<Rie2> for u8 {
    #[inline(always)]
    fn from(val: Rie2) -> u8 {
        Rie2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rie3 {
    #[doc = "Disable"]
    ZERO = 0x0,
    #[doc = "Enable"]
    ONE = 0x01,
}
impl Rie3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rie3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rie3 {
    #[inline(always)]
    fn from(val: u8) -> Rie3 {
        Rie3::from_bits(val)
    }
}
impl From<Rie3> for u8 {
    #[inline(always)]
    fn from(val: Rie3) -> u8 {
        Rie3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Te0 {
    #[doc = "Not empty"]
    ZERO = 0x0,
    #[doc = "Empty"]
    ONE = 0x01,
}
impl Te0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Te0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Te0 {
    #[inline(always)]
    fn from(val: u8) -> Te0 {
        Te0::from_bits(val)
    }
}
impl From<Te0> for u8 {
    #[inline(always)]
    fn from(val: Te0) -> u8 {
        Te0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Te1 {
    #[doc = "Not empty"]
    ZERO = 0x0,
    #[doc = "Empty"]
    ONE = 0x01,
}
impl Te1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Te1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Te1 {
    #[inline(always)]
    fn from(val: u8) -> Te1 {
        Te1::from_bits(val)
    }
}
impl From<Te1> for u8 {
    #[inline(always)]
    fn from(val: Te1) -> u8 {
        Te1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Te2 {
    #[doc = "Not empty"]
    ZERO = 0x0,
    #[doc = "Empty"]
    ONE = 0x01,
}
impl Te2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Te2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Te2 {
    #[inline(always)]
    fn from(val: u8) -> Te2 {
        Te2::from_bits(val)
    }
}
impl From<Te2> for u8 {
    #[inline(always)]
    fn from(val: Te2) -> u8 {
        Te2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Te3 {
    #[doc = "Not empty"]
    ZERO = 0x0,
    #[doc = "Empty"]
    ONE = 0x01,
}
impl Te3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Te3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Te3 {
    #[inline(always)]
    fn from(val: u8) -> Te3 {
        Te3::from_bits(val)
    }
}
impl From<Te3> for u8 {
    #[inline(always)]
    fn from(val: Te3) -> u8 {
        Te3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tep {
    #[doc = "Not pending; MUB is reading no Receive (RRn) register"]
    ZERO = 0x0,
    #[doc = "Pending; MUB is reading a Receive (RRn) register"]
    ONE = 0x01,
}
impl Tep {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tep {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tep {
    #[inline(always)]
    fn from(val: u8) -> Tep {
        Tep::from_bits(val)
    }
}
impl From<Tep> for u8 {
    #[inline(always)]
    fn from(val: Tep) -> u8 {
        Tep::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tie0 {
    #[doc = "Disable"]
    ZERO = 0x0,
    #[doc = "Enable"]
    ONE = 0x01,
}
impl Tie0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tie0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tie0 {
    #[inline(always)]
    fn from(val: u8) -> Tie0 {
        Tie0::from_bits(val)
    }
}
impl From<Tie0> for u8 {
    #[inline(always)]
    fn from(val: Tie0) -> u8 {
        Tie0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tie1 {
    #[doc = "Disable"]
    ZERO = 0x0,
    #[doc = "Enable"]
    ONE = 0x01,
}
impl Tie1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tie1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tie1 {
    #[inline(always)]
    fn from(val: u8) -> Tie1 {
        Tie1::from_bits(val)
    }
}
impl From<Tie1> for u8 {
    #[inline(always)]
    fn from(val: Tie1) -> u8 {
        Tie1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tie2 {
    #[doc = "Disable"]
    ZERO = 0x0,
    #[doc = "Enable"]
    ONE = 0x01,
}
impl Tie2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tie2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tie2 {
    #[inline(always)]
    fn from(val: u8) -> Tie2 {
        Tie2::from_bits(val)
    }
}
impl From<Tie2> for u8 {
    #[inline(always)]
    fn from(val: Tie2) -> u8 {
        Tie2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tie3 {
    #[doc = "Disable"]
    ZERO = 0x0,
    #[doc = "Enable"]
    ONE = 0x01,
}
impl Tie3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tie3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tie3 {
    #[inline(always)]
    fn from(val: u8) -> Tie3 {
        Tie3::from_bits(val)
    }
}
impl From<Tie3> for u8 {
    #[inline(always)]
    fn from(val: Tie3) -> u8 {
        Tie3::to_bits(val)
    }
}
