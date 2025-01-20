#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DgsTc0 {
    #[doc = "Closed"]
    Zero = 0x0,
    #[doc = "Open"]
    One = 0x01,
}
impl DgsTc0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DgsTc0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DgsTc0 {
    #[inline(always)]
    fn from(val: u8) -> DgsTc0 {
        DgsTc0::from_bits(val)
    }
}
impl From<DgsTc0> for u8 {
    #[inline(always)]
    fn from(val: DgsTc0) -> u8 {
        DgsTc0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DgsTc1 {
    #[doc = "Closed"]
    Zero = 0x0,
    #[doc = "Open"]
    One = 0x01,
}
impl DgsTc1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DgsTc1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DgsTc1 {
    #[inline(always)]
    fn from(val: u8) -> DgsTc1 {
        DgsTc1::from_bits(val)
    }
}
impl From<DgsTc1> for u8 {
    #[inline(always)]
    fn from(val: DgsTc1) -> u8 {
        DgsTc1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DgsTc2 {
    #[doc = "Closed"]
    Zero = 0x0,
    #[doc = "Open"]
    One = 0x01,
}
impl DgsTc2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DgsTc2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DgsTc2 {
    #[inline(always)]
    fn from(val: u8) -> DgsTc2 {
        DgsTc2::from_bits(val)
    }
}
impl From<DgsTc2> for u8 {
    #[inline(always)]
    fn from(val: DgsTc2) -> u8 {
        DgsTc2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DgsTc3 {
    #[doc = "Closed"]
    Zero = 0x0,
    #[doc = "Open"]
    One = 0x01,
}
impl DgsTc3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DgsTc3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DgsTc3 {
    #[inline(always)]
    fn from(val: u8) -> DgsTc3 {
        DgsTc3::from_bits(val)
    }
}
impl From<DgsTc3> for u8 {
    #[inline(always)]
    fn from(val: DgsTc3) -> u8 {
        DgsTc3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DgsTc4 {
    #[doc = "Closed"]
    Zero = 0x0,
    #[doc = "Open"]
    One = 0x01,
}
impl DgsTc4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DgsTc4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DgsTc4 {
    #[inline(always)]
    fn from(val: u8) -> DgsTc4 {
        DgsTc4::from_bits(val)
    }
}
impl From<DgsTc4> for u8 {
    #[inline(always)]
    fn from(val: DgsTc4) -> u8 {
        DgsTc4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DgsTc5 {
    #[doc = "Closed"]
    Zero = 0x0,
    #[doc = "Open"]
    One = 0x01,
}
impl DgsTc5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DgsTc5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DgsTc5 {
    #[inline(always)]
    fn from(val: u8) -> DgsTc5 {
        DgsTc5::from_bits(val)
    }
}
impl From<DgsTc5> for u8 {
    #[inline(always)]
    fn from(val: DgsTc5) -> u8 {
        DgsTc5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DgsTc6 {
    #[doc = "Closed"]
    Zero = 0x0,
    #[doc = "Open"]
    One = 0x01,
}
impl DgsTc6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DgsTc6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DgsTc6 {
    #[inline(always)]
    fn from(val: u8) -> DgsTc6 {
        DgsTc6::from_bits(val)
    }
}
impl From<DgsTc6> for u8 {
    #[inline(always)]
    fn from(val: DgsTc6) -> u8 {
        DgsTc6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DgsTc7 {
    #[doc = "Closed"]
    Zero = 0x0,
    #[doc = "Open"]
    One = 0x01,
}
impl DgsTc7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DgsTc7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DgsTc7 {
    #[inline(always)]
    fn from(val: u8) -> DgsTc7 {
        DgsTc7::from_bits(val)
    }
}
impl From<DgsTc7> for u8 {
    #[inline(always)]
    fn from(val: DgsTc7) -> u8 {
        DgsTc7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Drme {
    #[doc = "Preserve the DR value in the outer VLAN."]
    Preserve = 0x0,
    #[doc = "Update DR value in the outer VLAN based on DEnDEI field."]
    Update = 0x01,
}
impl Drme {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Drme {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Drme {
    #[inline(always)]
    fn from(val: u8) -> Drme {
        Drme::from_bits(val)
    }
}
impl From<Drme> for u8 {
    #[inline(always)]
    fn from(val: Drme) -> u8 {
        Drme::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fmma {
    #[doc = "Discard the frame and counted against the port's Tx discard count register (PTXDCR) along with the setting of the Frame Modification Misconfiguration Discard Reason (FMMDR) flag to 1 in the port's Tx discard count reason register 0 (PTXDCRR0)."]
    Discard = 0x0,
    #[doc = "Transmit the frame without performing any of the ingress and egress frame modification actions specified."]
    Transmit = 0x01,
}
impl Fmma {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fmma {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fmma {
    #[inline(always)]
    fn from(val: u8) -> Fmma {
        Fmma::from_bits(val)
    }
}
impl From<Fmma> for u8 {
    #[inline(always)]
    fn from(val: Fmma) -> u8 {
        Fmma::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum L3hfp {
    #[doc = "No L3 header present. Indicates to the parser of not parsing the L3 header. Parsing in this case would go as far as the L2 header regardless of whether or not there is an L3 header in the frame. This option should not be used if there are any table lookup entries that contain L3/L4 key fields that could be matched against a frame."]
    L3HdrNotPreset = 0x0,
    #[doc = "Parse L3 header if present in the frame."]
    L3HdrPresent = 0x01,
}
impl L3hfp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> L3hfp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for L3hfp {
    #[inline(always)]
    fn from(val: u8) -> L3hfp {
        L3hfp::from_bits(val)
    }
}
impl From<L3hfp> for u8 {
    #[inline(always)]
    fn from(val: L3hfp) -> u8 {
        L3hfp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum L4hfp {
    #[doc = "No L4 header present. Indicates to the parser of not parsing the L4 header. Parsing in this case would go as far as the L3 header if configured to parse it (L3HFP=1) regardless of whether or not there is an L4 header in the frame. This option should not be used if there are any table lookup entries that contain L4 key fields that could be matched against a frame."]
    L4HdrNotPresent = 0x0,
    #[doc = "Parse L4 header if present in the frame"]
    L4HdrPresent = 0x01,
}
impl L4hfp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> L4hfp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for L4hfp {
    #[inline(always)]
    fn from(val: u8) -> L4hfp {
        L4hfp::from_bits(val)
    }
}
impl From<L4hfp> for u8 {
    #[inline(always)]
    fn from(val: L4hfp) -> u8 {
        L4hfp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RxBusy {
    #[doc = "Idle"]
    Idle = 0x0,
    #[doc = "Busy"]
    Busy = 0x01,
}
impl RxBusy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RxBusy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RxBusy {
    #[inline(always)]
    fn from(val: u8) -> RxBusy {
        RxBusy::from_bits(val)
    }
}
impl From<RxBusy> for u8 {
    #[inline(always)]
    fn from(val: RxBusy) -> u8 {
        RxBusy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxdis {
    #[doc = "Rx path is enabled."]
    Enable = 0x0,
    #[doc = "Rx path is disabled."]
    Disable = 0x01,
}
impl Rxdis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxdis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxdis {
    #[inline(always)]
    fn from(val: u8) -> Rxdis {
        Rxdis::from_bits(val)
    }
}
impl From<Rxdis> for u8 {
    #[inline(always)]
    fn from(val: Rxdis) -> u8 {
        Rxdis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Srcprnd {
    #[doc = "Enabled"]
    Enable = 0x0,
    #[doc = "Disabled"]
    Disable = 0x01,
}
impl Srcprnd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Srcprnd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Srcprnd {
    #[inline(always)]
    fn from(val: u8) -> Srcprnd {
        Srcprnd::from_bits(val)
    }
}
impl From<Srcprnd> for u8 {
    #[inline(always)]
    fn from(val: Srcprnd) -> u8 {
        Srcprnd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TxBusy {
    #[doc = "Idle"]
    Idle = 0x0,
    #[doc = "Busy"]
    Busy = 0x01,
}
impl TxBusy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TxBusy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TxBusy {
    #[inline(always)]
    fn from(val: u8) -> TxBusy {
        TxBusy::from_bits(val)
    }
}
impl From<TxBusy> for u8 {
    #[inline(always)]
    fn from(val: TxBusy) -> u8 {
        TxBusy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txdis {
    #[doc = "Tx path is enabled"]
    Enable = 0x0,
    #[doc = "Tx path is disabled."]
    Disable = 0x01,
}
impl Txdis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txdis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txdis {
    #[inline(always)]
    fn from(val: u8) -> Txdis {
        Txdis::from_bits(val)
    }
}
impl From<Txdis> for u8 {
    #[inline(always)]
    fn from(val: Txdis) -> u8 {
        Txdis::to_bits(val)
    }
}
