#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aicsa {
    #[doc = "Bit clock 0"]
    Bitclk0 = 0x0,
    #[doc = "Bit clock 1"]
    Bitclk1 = 0x01,
    #[doc = "Bit clock 2"]
    Bitclk2 = 0x02,
    #[doc = "Bit clock 3"]
    Bitclk3 = 0x03,
    #[doc = "Bit clock 4"]
    Bitclk4 = 0x04,
    #[doc = "Bit clock 5"]
    Bitclk5 = 0x05,
    #[doc = "Bit clock 6"]
    Bitclk6 = 0x06,
    #[doc = "Bit clock 7"]
    Bitclk7 = 0x07,
    #[doc = "Bit clock 8"]
    Bitclk8 = 0x08,
    #[doc = "Bit clock 9"]
    Bitclk9 = 0x09,
    #[doc = "Bit clock A"]
    Bitclka = 0x0a,
    #[doc = "Bit clock B"]
    Bitclkb = 0x0b,
    #[doc = "Bit clock C"]
    Bitclkc = 0x0c,
    #[doc = "Bit clock D"]
    Bitclkd = 0x0d,
    #[doc = "Bit clock E"]
    Bitclke = 0x0e,
    #[doc = "Clock disabled, connected to zero"]
    ClkDisabled = 0x0f,
}
impl Aicsa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aicsa {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aicsa {
    #[inline(always)]
    fn from(val: u8) -> Aicsa {
        Aicsa::from_bits(val)
    }
}
impl From<Aicsa> for u8 {
    #[inline(always)]
    fn from(val: Aicsa) -> u8 {
        Aicsa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aicsb {
    #[doc = "Bit clock 0"]
    Bitclk0 = 0x0,
    #[doc = "Bit clock 1"]
    Bitclk1 = 0x01,
    #[doc = "Bit clock 2"]
    Bitclk2 = 0x02,
    #[doc = "Bit clock 3"]
    Bitclk3 = 0x03,
    #[doc = "Bit clock 4"]
    Bitclk4 = 0x04,
    #[doc = "Bit clock 5"]
    Bitclk5 = 0x05,
    #[doc = "Bit clock 6"]
    Bitclk6 = 0x06,
    #[doc = "Bit clock 7"]
    Bitclk7 = 0x07,
    #[doc = "Bit clock 8"]
    Bitclk8 = 0x08,
    #[doc = "Bit clock 9"]
    Bitclk9 = 0x09,
    #[doc = "Bit clock A"]
    Bitclka = 0x0a,
    #[doc = "Bit clock B"]
    Bitclkb = 0x0b,
    #[doc = "Bit clock C"]
    Bitclkc = 0x0c,
    #[doc = "Bit clock D"]
    Bitclkd = 0x0d,
    #[doc = "Bit clock E"]
    Bitclke = 0x0e,
    #[doc = "Clock disabled, connected to zero"]
    ClkDisabled = 0x0f,
}
impl Aicsb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aicsb {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aicsb {
    #[inline(always)]
    fn from(val: u8) -> Aicsb {
        Aicsb::from_bits(val)
    }
}
impl From<Aicsb> for u8 {
    #[inline(always)]
    fn from(val: Aicsb) -> u8 {
        Aicsb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aicsc {
    #[doc = "Bit clock 0"]
    Bitclk0 = 0x0,
    #[doc = "Bit clock 1"]
    Bitclk1 = 0x01,
    #[doc = "Bit clock 2"]
    Bitclk2 = 0x02,
    #[doc = "Bit clock 3"]
    Bitclk3 = 0x03,
    #[doc = "Bit clock 4"]
    Bitclk4 = 0x04,
    #[doc = "Bit clock 5"]
    Bitclk5 = 0x05,
    #[doc = "Bit clock 6"]
    Bitclk6 = 0x06,
    #[doc = "Bit clock 7"]
    Bitclk7 = 0x07,
    #[doc = "Bit clock 8"]
    Bitclk8 = 0x08,
    #[doc = "Bit clock 9"]
    Bitclk9 = 0x09,
    #[doc = "Bit clock A"]
    Bitclka = 0x0a,
    #[doc = "Bit clock B"]
    Bitclkb = 0x0b,
    #[doc = "Bit clock C"]
    Bitclkc = 0x0c,
    #[doc = "Bit clock D"]
    Bitclkd = 0x0d,
    #[doc = "Bit clock E"]
    Bitclke = 0x0e,
    #[doc = "Clock disabled, connected to zero"]
    ClkDisabled = 0x0f,
}
impl Aicsc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aicsc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aicsc {
    #[inline(always)]
    fn from(val: u8) -> Aicsc {
        Aicsc::from_bits(val)
    }
}
impl From<Aicsc> for u8 {
    #[inline(always)]
    fn from(val: Aicsc) -> u8 {
        Aicsc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aidea {
    #[doc = "The threshold has been met and no data input A interrupt is generated"]
    ThreshMet = 0x0,
    #[doc = "When AIDEA is set, the ASRC generates data input A interrupt request to the processor if ASRIER\\[AIDEA\\] = 1"]
    LessthanThresh = 0x01,
}
impl Aidea {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aidea {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aidea {
    #[inline(always)]
    fn from(val: u8) -> Aidea {
        Aidea::from_bits(val)
    }
}
impl From<Aidea> for u8 {
    #[inline(always)]
    fn from(val: Aidea) -> u8 {
        Aidea::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aideb {
    #[doc = "The threshold has been met and no data input B interrupt is generated"]
    ThreshMet = 0x0,
    #[doc = "When AIDEB is set, the ASRC generates data input B interrupt request to the processor if ASRIER\\[AIDEB\\] = 1"]
    LessthanThresh = 0x01,
}
impl Aideb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aideb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aideb {
    #[inline(always)]
    fn from(val: u8) -> Aideb {
        Aideb::from_bits(val)
    }
}
impl From<Aideb> for u8 {
    #[inline(always)]
    fn from(val: Aideb) -> u8 {
        Aideb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aidec {
    #[doc = "The threshold has been met and no data input C interrupt is generated"]
    ThreshMet = 0x0,
    #[doc = "When AIDEC is set, the ASRC generates data input C interrupt request to the processor if ASRIER\\[AIDEC\\] = 1"]
    LessthanThresh = 0x01,
}
impl Aidec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aidec {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aidec {
    #[inline(always)]
    fn from(val: u8) -> Aidec {
        Aidec::from_bits(val)
    }
}
impl From<Aidec> for u8 {
    #[inline(always)]
    fn from(val: Aidec) -> u8 {
        Aidec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Anca {
    #[doc = "0 channels in A (Pair A is disabled)"]
    ZeroChan = 0x0,
    #[doc = "1 channel in A"]
    OneChan = 0x01,
    #[doc = "2 channels in A"]
    TwoChan = 0x02,
    #[doc = "3 channels in A"]
    ThreeChan = 0x03,
    #[doc = "4 channels in A"]
    FourChan = 0x04,
    #[doc = "5 channels in A"]
    FiveChan = 0x05,
    #[doc = "6 channels in A"]
    SixChan = 0x06,
    #[doc = "7 channels in A"]
    SevenChan = 0x07,
    #[doc = "8 channels in A"]
    EightChan = 0x08,
    #[doc = "9 channels in A"]
    NineChan = 0x09,
    #[doc = "10 channels in A"]
    TenChan = 0x0a,
    #[doc = "Should not be used."]
    NotUsed11 = 0x0b,
    #[doc = "Should not be used."]
    NotUsed12 = 0x0c,
    #[doc = "Should not be used."]
    NotUsed13 = 0x0d,
    #[doc = "Should not be used."]
    NotUsed14 = 0x0e,
    #[doc = "Should not be used."]
    NotUsed15 = 0x0f,
}
impl Anca {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Anca {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Anca {
    #[inline(always)]
    fn from(val: u8) -> Anca {
        Anca::from_bits(val)
    }
}
impl From<Anca> for u8 {
    #[inline(always)]
    fn from(val: Anca) -> u8 {
        Anca::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ancb {
    #[doc = "0 channels in B (Pair B is disabled)"]
    ZeroChan = 0x0,
    #[doc = "1 channel in B"]
    OneChan = 0x01,
    #[doc = "2 channels in B"]
    TwoChan = 0x02,
    #[doc = "3 channels in B"]
    ThreeChan = 0x03,
    #[doc = "4 channels in B"]
    FourChan = 0x04,
    #[doc = "5 channels in B"]
    FiveChan = 0x05,
    #[doc = "6 channels in B"]
    SixChan = 0x06,
    #[doc = "7 channels in B"]
    SevenChan = 0x07,
    #[doc = "8 channels in B"]
    EightChan = 0x08,
    #[doc = "9 channels in B"]
    NineChan = 0x09,
    #[doc = "10 channels in B"]
    TenChan = 0x0a,
    #[doc = "Should not be used."]
    NotUsed11 = 0x0b,
    #[doc = "Should not be used."]
    NotUsed12 = 0x0c,
    #[doc = "Should not be used."]
    NotUsed13 = 0x0d,
    #[doc = "Should not be used."]
    NotUsed14 = 0x0e,
    #[doc = "Should not be used."]
    NotUsed15 = 0x0f,
}
impl Ancb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ancb {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ancb {
    #[inline(always)]
    fn from(val: u8) -> Ancb {
        Ancb::from_bits(val)
    }
}
impl From<Ancb> for u8 {
    #[inline(always)]
    fn from(val: Ancb) -> u8 {
        Ancb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ancc {
    #[doc = "0 channels in C (Pair C is disabled)"]
    ZeroChan = 0x0,
    #[doc = "1 channel in C"]
    OneChan = 0x01,
    #[doc = "2 channels in C"]
    TwoChan = 0x02,
    #[doc = "3 channels in C"]
    ThreeChan = 0x03,
    #[doc = "4 channels in C"]
    FourChan = 0x04,
    #[doc = "5 channels in C"]
    FiveChan = 0x05,
    #[doc = "6 channels in C"]
    SixChan = 0x06,
    #[doc = "7 channels in C"]
    SevenChan = 0x07,
    #[doc = "8 channels in C"]
    EightChan = 0x08,
    #[doc = "9 channels in C"]
    NineChan = 0x09,
    #[doc = "10 channels in C"]
    TenChan = 0x0a,
    #[doc = "Should not be used."]
    NotUsed11 = 0x0b,
    #[doc = "Should not be used."]
    NotUsed12 = 0x0c,
    #[doc = "Should not be used."]
    NotUsed13 = 0x0d,
    #[doc = "Should not be used."]
    NotUsed14 = 0x0e,
    #[doc = "Should not be used."]
    NotUsed15 = 0x0f,
}
impl Ancc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ancc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ancc {
    #[inline(always)]
    fn from(val: u8) -> Ancc {
        Ancc::from_bits(val)
    }
}
impl From<Ancc> for u8 {
    #[inline(always)]
    fn from(val: Ancc) -> u8 {
        Ancc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aocsa {
    #[doc = "Bit clock 0"]
    Bitclk0 = 0x0,
    #[doc = "Bit clock 1"]
    Bitclk1 = 0x01,
    #[doc = "Bit clock 2"]
    Bitclk2 = 0x02,
    #[doc = "Bit clock 3"]
    Bitclk3 = 0x03,
    #[doc = "Bit clock 4"]
    Bitclk4 = 0x04,
    #[doc = "Bit clock 5"]
    Bitclk5 = 0x05,
    #[doc = "Bit clock 6"]
    Bitclk6 = 0x06,
    #[doc = "Bit clock 7"]
    Bitclk7 = 0x07,
    #[doc = "Bit clock 8"]
    Bitclk8 = 0x08,
    #[doc = "Bit clock 9"]
    Bitclk9 = 0x09,
    #[doc = "Bit clock A"]
    Bitclka = 0x0a,
    #[doc = "Bit clock B"]
    Bitclkb = 0x0b,
    #[doc = "Bit clock C"]
    Bitclkc = 0x0c,
    #[doc = "Bit clock D"]
    Bitclkd = 0x0d,
    #[doc = "Bit clock E"]
    Bitclke = 0x0e,
    #[doc = "Clock disabled, connected to zero"]
    ClkDisabled = 0x0f,
}
impl Aocsa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aocsa {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aocsa {
    #[inline(always)]
    fn from(val: u8) -> Aocsa {
        Aocsa::from_bits(val)
    }
}
impl From<Aocsa> for u8 {
    #[inline(always)]
    fn from(val: Aocsa) -> u8 {
        Aocsa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aocsb {
    #[doc = "Bit clock 0"]
    Bitclk0 = 0x0,
    #[doc = "Bit clock 1"]
    Bitclk1 = 0x01,
    #[doc = "Bit clock 2"]
    Bitclk2 = 0x02,
    #[doc = "Bit clock 3"]
    Bitclk3 = 0x03,
    #[doc = "Bit clock 4"]
    Bitclk4 = 0x04,
    #[doc = "Bit clock 5"]
    Bitclk5 = 0x05,
    #[doc = "Bit clock 6"]
    Bitclk6 = 0x06,
    #[doc = "Bit clock 7"]
    Bitclk7 = 0x07,
    #[doc = "Bit clock 8"]
    Bitclk8 = 0x08,
    #[doc = "Bit clock 9"]
    Bitclk9 = 0x09,
    #[doc = "Bit clock A"]
    Bitclka = 0x0a,
    #[doc = "Bit clock B"]
    Bitclkb = 0x0b,
    #[doc = "Bit clock C"]
    Bitclkc = 0x0c,
    #[doc = "Bit clock D"]
    Bitclkd = 0x0d,
    #[doc = "Bit clock E"]
    Bitclke = 0x0e,
    #[doc = "Clock disabled, connected to zero"]
    ClkDisabled = 0x0f,
}
impl Aocsb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aocsb {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aocsb {
    #[inline(always)]
    fn from(val: u8) -> Aocsb {
        Aocsb::from_bits(val)
    }
}
impl From<Aocsb> for u8 {
    #[inline(always)]
    fn from(val: Aocsb) -> u8 {
        Aocsb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aocsc {
    #[doc = "Bit clock 0"]
    Bitclk0 = 0x0,
    #[doc = "Bit clock 1"]
    Bitclk1 = 0x01,
    #[doc = "Bit clock 2"]
    Bitclk2 = 0x02,
    #[doc = "Bit clock 3"]
    Bitclk3 = 0x03,
    #[doc = "Bit clock 4"]
    Bitclk4 = 0x04,
    #[doc = "Bit clock 5"]
    Bitclk5 = 0x05,
    #[doc = "Bit clock 6"]
    Bitclk6 = 0x06,
    #[doc = "Bit clock 7"]
    Bitclk7 = 0x07,
    #[doc = "Bit clock 8"]
    Bitclk8 = 0x08,
    #[doc = "Bit clock 9"]
    Bitclk9 = 0x09,
    #[doc = "Bit clock A"]
    Bitclka = 0x0a,
    #[doc = "Bit clock B"]
    Bitclkb = 0x0b,
    #[doc = "Bit clock C"]
    Bitclkc = 0x0c,
    #[doc = "Bit clock D"]
    Bitclkd = 0x0d,
    #[doc = "Bit clock E"]
    Bitclke = 0x0e,
    #[doc = "Clock disabled, connected to zero"]
    ClkDisabled = 0x0f,
}
impl Aocsc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aocsc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aocsc {
    #[inline(always)]
    fn from(val: u8) -> Aocsc {
        Aocsc::from_bits(val)
    }
}
impl From<Aocsc> for u8 {
    #[inline(always)]
    fn from(val: Aocsc) -> u8 {
        Aocsc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aodfa {
    #[doc = "The threshold has not yet been met and no data output A interrupt is generated"]
    ThreshNotmet = 0x0,
    #[doc = "When AODFA is set, the ASRC generates data output A interrupt request to the processor if ASRIER\\[ADOEA\\] = 1"]
    GreaterthanThresh = 0x01,
}
impl Aodfa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aodfa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aodfa {
    #[inline(always)]
    fn from(val: u8) -> Aodfa {
        Aodfa::from_bits(val)
    }
}
impl From<Aodfa> for u8 {
    #[inline(always)]
    fn from(val: Aodfa) -> u8 {
        Aodfa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aodfb {
    #[doc = "The threshold has not yet been met and no data output B interrupt is generated"]
    ThreshNotmet = 0x0,
    #[doc = "When AODFB is set, the ASRC generates data output B interrupt request to the processor if ASRIER\\[ADOEB\\] = 1"]
    GreaterthanThresh = 0x01,
}
impl Aodfb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aodfb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aodfb {
    #[inline(always)]
    fn from(val: u8) -> Aodfb {
        Aodfb::from_bits(val)
    }
}
impl From<Aodfb> for u8 {
    #[inline(always)]
    fn from(val: Aodfb) -> u8 {
        Aodfb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aodfc {
    #[doc = "The threshold has not yet been met and no data output C interrupt is generated"]
    ThreshNotmet = 0x0,
    #[doc = "When AODFC is set, the ASRC generates data output C interrupt request to the processor if ASRIER\\[ADOEC\\] = 1"]
    GreaterthanThresh = 0x01,
}
impl Aodfc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aodfc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aodfc {
    #[inline(always)]
    fn from(val: u8) -> Aodfc {
        Aodfc::from_bits(val)
    }
}
impl From<Aodfc> for u8 {
    #[inline(always)]
    fn from(val: Aodfc) -> u8 {
        Aodfc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aole {
    #[doc = "No overload"]
    TaskOk = 0x0,
    #[doc = "Task rate is too high"]
    TooHigh = 0x01,
}
impl Aole {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aole {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aole {
    #[inline(always)]
    fn from(val: u8) -> Aole {
        Aole::from_bits(val)
    }
}
impl From<Aole> for u8 {
    #[inline(always)]
    fn from(val: Aole) -> u8 {
        Aole::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dslcnt {
    #[doc = "New DSL counter information is in the process of storage into the internal ASRC FIFO"]
    DslcntProc = 0x0,
    #[doc = "New DSL counter information is stored in the internal ASRC FIFO"]
    DslcntStored = 0x01,
}
impl Dslcnt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dslcnt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dslcnt {
    #[inline(always)]
    fn from(val: u8) -> Dslcnt {
        Dslcnt::from_bits(val)
    }
}
impl From<Dslcnt> for u8 {
    #[inline(always)]
    fn from(val: Dslcnt) -> u8 {
        Dslcnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Extthrsha {
    #[doc = "Use default thresholds."]
    UseDefaultThresh = 0x0,
    #[doc = "Use external defined thresholds."]
    UseExtThresh = 0x01,
}
impl Extthrsha {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Extthrsha {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Extthrsha {
    #[inline(always)]
    fn from(val: u8) -> Extthrsha {
        Extthrsha::from_bits(val)
    }
}
impl From<Extthrsha> for u8 {
    #[inline(always)]
    fn from(val: Extthrsha) -> u8 {
        Extthrsha::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Extthrshb {
    #[doc = "Use default thresholds."]
    UseDefaultThresh = 0x0,
    #[doc = "Use external defined thresholds."]
    UseExtThresh = 0x01,
}
impl Extthrshb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Extthrshb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Extthrshb {
    #[inline(always)]
    fn from(val: u8) -> Extthrshb {
        Extthrshb::from_bits(val)
    }
}
impl From<Extthrshb> for u8 {
    #[inline(always)]
    fn from(val: Extthrshb) -> u8 {
        Extthrshb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Extthrshc {
    #[doc = "Use default thresholds."]
    UseDefaultThresh = 0x0,
    #[doc = "Use external defined thresholds."]
    UseExtThresh = 0x01,
}
impl Extthrshc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Extthrshc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Extthrshc {
    #[inline(always)]
    fn from(val: u8) -> Extthrshc {
        Extthrshc::from_bits(val)
    }
}
impl From<Extthrshc> for u8 {
    #[inline(always)]
    fn from(val: Extthrshc) -> u8 {
        Extthrshc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Idra {
    #[doc = "ASRC internal measured ratio is used"]
    IdraMeasured = 0x0,
    #[doc = "Ideal ratio from the interface register ASRIDRHA, ASRIDRLA is used"]
    IdraIdeal = 0x01,
}
impl Idra {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Idra {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Idra {
    #[inline(always)]
    fn from(val: u8) -> Idra {
        Idra::from_bits(val)
    }
}
impl From<Idra> for u8 {
    #[inline(always)]
    fn from(val: Idra) -> u8 {
        Idra::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Idrb {
    #[doc = "ASRC internal measured ratio is used"]
    IdraMeasured = 0x0,
    #[doc = "Ideal ratio from the interface register ASRIDRHB, ASRIDRLB is used"]
    IdraIdeal = 0x01,
}
impl Idrb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Idrb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Idrb {
    #[inline(always)]
    fn from(val: u8) -> Idrb {
        Idrb::from_bits(val)
    }
}
impl From<Idrb> for u8 {
    #[inline(always)]
    fn from(val: Idrb) -> u8 {
        Idrb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Idrc {
    #[doc = "ASRC internal measured ratio is used"]
    IdraMeasured = 0x0,
    #[doc = "Ideal ratio from the interface register ASRIDRHC, ASRIDRLC is used"]
    IdraIdeal = 0x01,
}
impl Idrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Idrc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Idrc {
    #[inline(always)]
    fn from(val: u8) -> Idrc {
        Idrc::from_bits(val)
    }
}
impl From<Idrc> for u8 {
    #[inline(always)]
    fn from(val: Idrc) -> u8 {
        Idrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Imsb {
    #[doc = "LSB aligned"]
    LsbAligned = 0x0,
    #[doc = "MSB aligned"]
    MsbAligned = 0x01,
}
impl Imsb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Imsb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Imsb {
    #[inline(always)]
    fn from(val: u8) -> Imsb {
        Imsb::from_bits(val)
    }
}
impl From<Imsb> for u8 {
    #[inline(always)]
    fn from(val: Imsb) -> u8 {
        Imsb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Inirqa {
    #[doc = "Initialization for Conversion Pair A not served"]
    InitNotserved = 0x0,
    #[doc = "Initialization for Conversion Pair A served"]
    InitServed = 0x01,
}
impl Inirqa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Inirqa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Inirqa {
    #[inline(always)]
    fn from(val: u8) -> Inirqa {
        Inirqa::from_bits(val)
    }
}
impl From<Inirqa> for u8 {
    #[inline(always)]
    fn from(val: Inirqa) -> u8 {
        Inirqa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Inirqb {
    #[doc = "Initialization for Conversion Pair B not served"]
    InitNotserved = 0x0,
    #[doc = "Initialization for Conversion Pair B served"]
    InitServed = 0x01,
}
impl Inirqb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Inirqb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Inirqb {
    #[inline(always)]
    fn from(val: u8) -> Inirqb {
        Inirqb::from_bits(val)
    }
}
impl From<Inirqb> for u8 {
    #[inline(always)]
    fn from(val: Inirqb) -> u8 {
        Inirqb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Inirqc {
    #[doc = "Initialization for Conversion Pair C not served"]
    InitNotserved = 0x0,
    #[doc = "Initialization for Conversion Pair C served"]
    InitServed = 0x01,
}
impl Inirqc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Inirqc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Inirqc {
    #[inline(always)]
    fn from(val: u8) -> Inirqc {
        Inirqc::from_bits(val)
    }
}
impl From<Inirqc> for u8 {
    #[inline(always)]
    fn from(val: Inirqc) -> u8 {
        Inirqc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iwd {
    #[doc = "24-bit audio data."]
    Audiodata24bit = 0x0,
    #[doc = "16-bit audio data."]
    Audiodata16bit = 0x01,
    #[doc = "8-bit audio data."]
    Audiodata8bit = 0x02,
    _RESERVED_3 = 0x03,
}
impl Iwd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iwd {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iwd {
    #[inline(always)]
    fn from(val: u8) -> Iwd {
        Iwd::from_bits(val)
    }
}
impl From<Iwd> for u8 {
    #[inline(always)]
    fn from(val: Iwd) -> u8 {
        Iwd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ndpra {
    #[doc = "Use default parameters for RAM-stored parameters. Override any parameters already in RAM."]
    UseDefault = 0x0,
    #[doc = "Don't use default parameters for RAM-stored parameters. Use the parameters already stored in RAM."]
    NotDefault = 0x01,
}
impl Ndpra {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ndpra {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ndpra {
    #[inline(always)]
    fn from(val: u8) -> Ndpra {
        Ndpra::from_bits(val)
    }
}
impl From<Ndpra> for u8 {
    #[inline(always)]
    fn from(val: Ndpra) -> u8 {
        Ndpra::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ndprb {
    #[doc = "Use default parameters for RAM-stored parameters. Override any parameters already in RAM."]
    UseDefault = 0x0,
    #[doc = "Don't use default parameters for RAM-stored parameter. Use the parameters already stored in RAM."]
    NotDefault = 0x01,
}
impl Ndprb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ndprb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ndprb {
    #[inline(always)]
    fn from(val: u8) -> Ndprb {
        Ndprb::from_bits(val)
    }
}
impl From<Ndprb> for u8 {
    #[inline(always)]
    fn from(val: Ndprb) -> u8 {
        Ndprb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ndprc {
    #[doc = "Use default parameters for RAM-stored parameters. Override any parameters already in RAM."]
    UseDefault = 0x0,
    #[doc = "Don't use default parameters for RAM-stored parameters. Use the parameters already stored in RAM."]
    NotDefault = 0x01,
}
impl Ndprc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ndprc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ndprc {
    #[inline(always)]
    fn from(val: u8) -> Ndprc {
        Ndprc::from_bits(val)
    }
}
impl From<Ndprc> for u8 {
    #[inline(always)]
    fn from(val: Ndprc) -> u8 {
        Ndprc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Omsb {
    #[doc = "LSB aligned"]
    LsbAligned = 0x0,
    #[doc = "MSB aligned"]
    MsbAligned = 0x01,
}
impl Omsb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Omsb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Omsb {
    #[inline(always)]
    fn from(val: u8) -> Omsb {
        Omsb::from_bits(val)
    }
}
impl From<Omsb> for u8 {
    #[inline(always)]
    fn from(val: Omsb) -> u8 {
        Omsb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ow16 {
    #[doc = "24-bit output data"]
    Out24bit = 0x0,
    #[doc = "16-bit output data"]
    Out16bit = 0x01,
}
impl Ow16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ow16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ow16 {
    #[inline(always)]
    fn from(val: u8) -> Ow16 {
        Ow16::from_bits(val)
    }
}
impl From<Ow16> for u8 {
    #[inline(always)]
    fn from(val: Ow16) -> u8 {
        Ow16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Postmoda {
    #[doc = "Select Upsampling-by-2"]
    Upsamp2 = 0x0,
    #[doc = "Select Direct-Connection"]
    DirectConnect = 0x01,
    #[doc = "Select Downsampling-by-2"]
    Downsamp2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Postmoda {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Postmoda {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Postmoda {
    #[inline(always)]
    fn from(val: u8) -> Postmoda {
        Postmoda::from_bits(val)
    }
}
impl From<Postmoda> for u8 {
    #[inline(always)]
    fn from(val: Postmoda) -> u8 {
        Postmoda::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Postmodb {
    #[doc = "Select Upsampling-by-2"]
    Upsamp2 = 0x0,
    #[doc = "Select Direct-Connection"]
    DirectConnect = 0x01,
    #[doc = "Select Downsampling-by-2"]
    Downsamp2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Postmodb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Postmodb {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Postmodb {
    #[inline(always)]
    fn from(val: u8) -> Postmodb {
        Postmodb::from_bits(val)
    }
}
impl From<Postmodb> for u8 {
    #[inline(always)]
    fn from(val: Postmodb) -> u8 {
        Postmodb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Postmodc {
    #[doc = "Select Upsampling-by-2 as defined in Signal Processing Flow."]
    Upsamp2 = 0x0,
    #[doc = "Select Direct-Connection as defined in Signal Processing Flow."]
    DirectConnect = 0x01,
    #[doc = "Select Downsampling-by-2 as defined in Signal Processing Flow."]
    Downsamp2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Postmodc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Postmodc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Postmodc {
    #[inline(always)]
    fn from(val: u8) -> Postmodc {
        Postmodc::from_bits(val)
    }
}
impl From<Postmodc> for u8 {
    #[inline(always)]
    fn from(val: Postmodc) -> u8 {
        Postmodc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Premoda {
    #[doc = "Select Upsampling-by-2"]
    Upsamp2 = 0x0,
    #[doc = "Select Direct-Connection"]
    DirectConnect = 0x01,
    #[doc = "Select Downsampling-by-2"]
    Downsamp2 = 0x02,
    #[doc = "Select passthrough mode. In this case, POSTMODA\\[1:0\\] have no use."]
    Passthru = 0x03,
}
impl Premoda {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Premoda {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Premoda {
    #[inline(always)]
    fn from(val: u8) -> Premoda {
        Premoda::from_bits(val)
    }
}
impl From<Premoda> for u8 {
    #[inline(always)]
    fn from(val: Premoda) -> u8 {
        Premoda::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Premodb {
    #[doc = "Select Upsampling-by-2"]
    Upsamp2 = 0x0,
    #[doc = "Select Direct-Connection"]
    DirectConnect = 0x01,
    #[doc = "Select Downsampling-by-2"]
    Downsamp2 = 0x02,
    #[doc = "Select passthrough mode. In this case, POSTMODB\\[1:0\\] have no use."]
    Passthru = 0x03,
}
impl Premodb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Premodb {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Premodb {
    #[inline(always)]
    fn from(val: u8) -> Premodb {
        Premodb::from_bits(val)
    }
}
impl From<Premodb> for u8 {
    #[inline(always)]
    fn from(val: Premodb) -> u8 {
        Premodb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Premodc {
    #[doc = "Select Upsampling-by-2"]
    Upsamp2 = 0x0,
    #[doc = "Select Direct-Connection"]
    DirectConnect = 0x01,
    #[doc = "Select Downsampling-by-2"]
    Downsamp2 = 0x02,
    #[doc = "Select passthrough mode. In this case, POSTMODC\\[1:0\\] have no use."]
    Passthru = 0x03,
}
impl Premodc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Premodc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Premodc {
    #[inline(always)]
    fn from(val: u8) -> Premodc {
        Premodc::from_bits(val)
    }
}
impl From<Premodc> for u8 {
    #[inline(always)]
    fn from(val: Premodc) -> u8 {
        Premodc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Srst {
    #[doc = "ASRC Software reset cleared"]
    Cleared = 0x0,
    #[doc = "ASRC Software reset generated. NOTE: This is a self-clear bit"]
    Reset = 0x01,
}
impl Srst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Srst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Srst {
    #[inline(always)]
    fn from(val: u8) -> Srst {
        Srst::from_bits(val)
    }
}
impl From<Srst> for u8 {
    #[inline(always)]
    fn from(val: Srst) -> u8 {
        Srst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usra {
    #[doc = "Do not use ratio as the input to ASRC for pair A"]
    UseRatioNo = 0x0,
    #[doc = "Use ratio as the input to ASRC for pair A"]
    UseRatio = 0x01,
}
impl Usra {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usra {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usra {
    #[inline(always)]
    fn from(val: u8) -> Usra {
        Usra::from_bits(val)
    }
}
impl From<Usra> for u8 {
    #[inline(always)]
    fn from(val: Usra) -> u8 {
        Usra::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usrb {
    #[doc = "Do not use ratio as the input to ASRC for pair B"]
    UseRatioNo = 0x0,
    #[doc = "Use ratio as the input to ASRC for pair B"]
    UseRatio = 0x01,
}
impl Usrb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usrb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usrb {
    #[inline(always)]
    fn from(val: u8) -> Usrb {
        Usrb::from_bits(val)
    }
}
impl From<Usrb> for u8 {
    #[inline(always)]
    fn from(val: Usrb) -> u8 {
        Usrb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usrc {
    #[doc = "Do not use ratio as the input to ASRC for pair C"]
    UseRatioNo = 0x0,
    #[doc = "Use ratio as the input to ASRC for pair C"]
    UseRatio = 0x01,
}
impl Usrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usrc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usrc {
    #[inline(always)]
    fn from(val: u8) -> Usrc {
        Usrc::from_bits(val)
    }
}
impl From<Usrc> for u8 {
    #[inline(always)]
    fn from(val: Usrc) -> u8 {
        Usrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Zerobufa {
    #[doc = "Zeroize the buffer"]
    ZeroBuf = 0x0,
    #[doc = "Don't zeroize the buffer"]
    DoNotZeroBuf = 0x01,
}
impl Zerobufa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Zerobufa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Zerobufa {
    #[inline(always)]
    fn from(val: u8) -> Zerobufa {
        Zerobufa::from_bits(val)
    }
}
impl From<Zerobufa> for u8 {
    #[inline(always)]
    fn from(val: Zerobufa) -> u8 {
        Zerobufa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Zerobufb {
    #[doc = "Zeroize the buffer"]
    ZeroBuf = 0x0,
    #[doc = "Don't zeroize the buffer"]
    DoNotZeroBuf = 0x01,
}
impl Zerobufb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Zerobufb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Zerobufb {
    #[inline(always)]
    fn from(val: u8) -> Zerobufb {
        Zerobufb::from_bits(val)
    }
}
impl From<Zerobufb> for u8 {
    #[inline(always)]
    fn from(val: Zerobufb) -> u8 {
        Zerobufb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Zerobufc {
    #[doc = "Zeroize the buffer"]
    ZeroBuf = 0x0,
    #[doc = "Don't zeroize the buffer"]
    DoNotZeroBuf = 0x01,
}
impl Zerobufc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Zerobufc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Zerobufc {
    #[inline(always)]
    fn from(val: u8) -> Zerobufc {
        Zerobufc::from_bits(val)
    }
}
impl From<Zerobufc> for u8 {
    #[inline(always)]
    fn from(val: Zerobufc) -> u8 {
        Zerobufc::to_bits(val)
    }
}
