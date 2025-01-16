#[doc = "Boolean Function Term 0 and 1 Configuration for EVENT0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bfcrt010(pub u16);
impl Bfcrt010 {
    #[doc = "Product Term 1, Input D Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_dc(&self) -> super::vals::Bfcrt010Pt1Dc {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Bfcrt010Pt1Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 1, Input D Configuration"]
    #[inline(always)]
    pub const fn set_pt1_dc(&mut self, val: super::vals::Bfcrt010Pt1Dc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Product Term 1, Input C Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_cc(&self) -> super::vals::Bfcrt010Pt1Cc {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Bfcrt010Pt1Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 1, Input C Configuration"]
    #[inline(always)]
    pub const fn set_pt1_cc(&mut self, val: super::vals::Bfcrt010Pt1Cc) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Product Term 1, Input B Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_bc(&self) -> super::vals::Bfcrt010Pt1Bc {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Bfcrt010Pt1Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 1, Input B Configuration"]
    #[inline(always)]
    pub const fn set_pt1_bc(&mut self, val: super::vals::Bfcrt010Pt1Bc) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Product Term 1, Input A Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_ac(&self) -> super::vals::Bfcrt010Pt1Ac {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Bfcrt010Pt1Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 1, Input A Configuration"]
    #[inline(always)]
    pub const fn set_pt1_ac(&mut self, val: super::vals::Bfcrt010Pt1Ac) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "Product Term 0, Input D Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_dc(&self) -> super::vals::Bfcrt010Pt0Dc {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Bfcrt010Pt0Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 0, Input D Configuration"]
    #[inline(always)]
    pub const fn set_pt0_dc(&mut self, val: super::vals::Bfcrt010Pt0Dc) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "Product Term 0, Input C Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_cc(&self) -> super::vals::Bfcrt010Pt0Cc {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Bfcrt010Pt0Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 0, Input C Configuration"]
    #[inline(always)]
    pub const fn set_pt0_cc(&mut self, val: super::vals::Bfcrt010Pt0Cc) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u16) & 0x03) << 10usize);
    }
    #[doc = "Product Term 0, Input B Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_bc(&self) -> super::vals::Bfcrt010Pt0Bc {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Bfcrt010Pt0Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 0, Input B Configuration"]
    #[inline(always)]
    pub const fn set_pt0_bc(&mut self, val: super::vals::Bfcrt010Pt0Bc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
    #[doc = "Product Term 0, Input A Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_ac(&self) -> super::vals::Bfcrt010Pt0Ac {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Bfcrt010Pt0Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 0, Input A Configuration"]
    #[inline(always)]
    pub const fn set_pt0_ac(&mut self, val: super::vals::Bfcrt010Pt0Ac) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Bfcrt010 {
    #[inline(always)]
    fn default() -> Bfcrt010 {
        Bfcrt010(0u64 as u16)
    }
}
impl core::fmt::Debug for Bfcrt010 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bfcrt010")
            .field("pt1_dc", &self.pt1_dc())
            .field("pt1_cc", &self.pt1_cc())
            .field("pt1_bc", &self.pt1_bc())
            .field("pt1_ac", &self.pt1_ac())
            .field("pt0_dc", &self.pt0_dc())
            .field("pt0_cc", &self.pt0_cc())
            .field("pt0_bc", &self.pt0_bc())
            .field("pt0_ac", &self.pt0_ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bfcrt010 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Bfcrt010 {
            pt1_dc: super::vals::Bfcrt010Pt1Dc,
            pt1_cc: super::vals::Bfcrt010Pt1Cc,
            pt1_bc: super::vals::Bfcrt010Pt1Bc,
            pt1_ac: super::vals::Bfcrt010Pt1Ac,
            pt0_dc: super::vals::Bfcrt010Pt0Dc,
            pt0_cc: super::vals::Bfcrt010Pt0Cc,
            pt0_bc: super::vals::Bfcrt010Pt0Bc,
            pt0_ac: super::vals::Bfcrt010Pt0Ac,
        }
        let proxy = Bfcrt010 {
            pt1_dc: self.pt1_dc(),
            pt1_cc: self.pt1_cc(),
            pt1_bc: self.pt1_bc(),
            pt1_ac: self.pt1_ac(),
            pt0_dc: self.pt0_dc(),
            pt0_cc: self.pt0_cc(),
            pt0_bc: self.pt0_bc(),
            pt0_ac: self.pt0_ac(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Boolean Function Term 0 and 1 Configuration for EVENT1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bfcrt011(pub u16);
impl Bfcrt011 {
    #[doc = "Product Term 1, Input D Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_dc(&self) -> super::vals::Bfcrt011Pt1Dc {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Bfcrt011Pt1Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 1, Input D Configuration"]
    #[inline(always)]
    pub const fn set_pt1_dc(&mut self, val: super::vals::Bfcrt011Pt1Dc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Product Term 1, Input C Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_cc(&self) -> super::vals::Bfcrt011Pt1Cc {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Bfcrt011Pt1Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 1, Input C Configuration"]
    #[inline(always)]
    pub const fn set_pt1_cc(&mut self, val: super::vals::Bfcrt011Pt1Cc) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Product Term 1, Input B Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_bc(&self) -> super::vals::Bfcrt011Pt1Bc {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Bfcrt011Pt1Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 1, Input B Configuration"]
    #[inline(always)]
    pub const fn set_pt1_bc(&mut self, val: super::vals::Bfcrt011Pt1Bc) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Product Term 1, Input A Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_ac(&self) -> super::vals::Bfcrt011Pt1Ac {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Bfcrt011Pt1Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 1, Input A Configuration"]
    #[inline(always)]
    pub const fn set_pt1_ac(&mut self, val: super::vals::Bfcrt011Pt1Ac) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "Product Term 0, Input D Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_dc(&self) -> super::vals::Bfcrt011Pt0Dc {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Bfcrt011Pt0Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 0, Input D Configuration"]
    #[inline(always)]
    pub const fn set_pt0_dc(&mut self, val: super::vals::Bfcrt011Pt0Dc) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "Product Term 0, Input C Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_cc(&self) -> super::vals::Bfcrt011Pt0Cc {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Bfcrt011Pt0Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 0, Input C Configuration"]
    #[inline(always)]
    pub const fn set_pt0_cc(&mut self, val: super::vals::Bfcrt011Pt0Cc) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u16) & 0x03) << 10usize);
    }
    #[doc = "Product Term 0, Input B Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_bc(&self) -> super::vals::Bfcrt011Pt0Bc {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Bfcrt011Pt0Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 0, Input B Configuration"]
    #[inline(always)]
    pub const fn set_pt0_bc(&mut self, val: super::vals::Bfcrt011Pt0Bc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
    #[doc = "Product Term 0, Input A Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_ac(&self) -> super::vals::Bfcrt011Pt0Ac {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Bfcrt011Pt0Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 0, Input A Configuration"]
    #[inline(always)]
    pub const fn set_pt0_ac(&mut self, val: super::vals::Bfcrt011Pt0Ac) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Bfcrt011 {
    #[inline(always)]
    fn default() -> Bfcrt011 {
        Bfcrt011(0u64 as u16)
    }
}
impl core::fmt::Debug for Bfcrt011 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bfcrt011")
            .field("pt1_dc", &self.pt1_dc())
            .field("pt1_cc", &self.pt1_cc())
            .field("pt1_bc", &self.pt1_bc())
            .field("pt1_ac", &self.pt1_ac())
            .field("pt0_dc", &self.pt0_dc())
            .field("pt0_cc", &self.pt0_cc())
            .field("pt0_bc", &self.pt0_bc())
            .field("pt0_ac", &self.pt0_ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bfcrt011 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Bfcrt011 {
            pt1_dc: super::vals::Bfcrt011Pt1Dc,
            pt1_cc: super::vals::Bfcrt011Pt1Cc,
            pt1_bc: super::vals::Bfcrt011Pt1Bc,
            pt1_ac: super::vals::Bfcrt011Pt1Ac,
            pt0_dc: super::vals::Bfcrt011Pt0Dc,
            pt0_cc: super::vals::Bfcrt011Pt0Cc,
            pt0_bc: super::vals::Bfcrt011Pt0Bc,
            pt0_ac: super::vals::Bfcrt011Pt0Ac,
        }
        let proxy = Bfcrt011 {
            pt1_dc: self.pt1_dc(),
            pt1_cc: self.pt1_cc(),
            pt1_bc: self.pt1_bc(),
            pt1_ac: self.pt1_ac(),
            pt0_dc: self.pt0_dc(),
            pt0_cc: self.pt0_cc(),
            pt0_bc: self.pt0_bc(),
            pt0_ac: self.pt0_ac(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Boolean Function Term 0 and 1 Configuration for EVENT2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bfcrt012(pub u16);
impl Bfcrt012 {
    #[doc = "Product Term 1, Input D Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_dc(&self) -> super::vals::Bfcrt012Pt1Dc {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Bfcrt012Pt1Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 1, Input D Configuration"]
    #[inline(always)]
    pub const fn set_pt1_dc(&mut self, val: super::vals::Bfcrt012Pt1Dc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Product Term 1, Input C Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_cc(&self) -> super::vals::Bfcrt012Pt1Cc {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Bfcrt012Pt1Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 1, Input C Configuration"]
    #[inline(always)]
    pub const fn set_pt1_cc(&mut self, val: super::vals::Bfcrt012Pt1Cc) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Product Term 1, Input B Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_bc(&self) -> super::vals::Bfcrt012Pt1Bc {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Bfcrt012Pt1Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 1, Input B Configuration"]
    #[inline(always)]
    pub const fn set_pt1_bc(&mut self, val: super::vals::Bfcrt012Pt1Bc) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Product Term 1, Input A Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_ac(&self) -> super::vals::Bfcrt012Pt1Ac {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Bfcrt012Pt1Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 1, Input A Configuration"]
    #[inline(always)]
    pub const fn set_pt1_ac(&mut self, val: super::vals::Bfcrt012Pt1Ac) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "Product Term 0, Input D Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_dc(&self) -> super::vals::Bfcrt012Pt0Dc {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Bfcrt012Pt0Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 0, Input D Configuration"]
    #[inline(always)]
    pub const fn set_pt0_dc(&mut self, val: super::vals::Bfcrt012Pt0Dc) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "Product Term 0, Input C Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_cc(&self) -> super::vals::Bfcrt012Pt0Cc {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Bfcrt012Pt0Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 0, Input C Configuration"]
    #[inline(always)]
    pub const fn set_pt0_cc(&mut self, val: super::vals::Bfcrt012Pt0Cc) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u16) & 0x03) << 10usize);
    }
    #[doc = "Product Term 0, Input B Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_bc(&self) -> super::vals::Bfcrt012Pt0Bc {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Bfcrt012Pt0Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 0, Input B Configuration"]
    #[inline(always)]
    pub const fn set_pt0_bc(&mut self, val: super::vals::Bfcrt012Pt0Bc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
    #[doc = "Product Term 0, Input A Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_ac(&self) -> super::vals::Bfcrt012Pt0Ac {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Bfcrt012Pt0Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 0, Input A Configuration"]
    #[inline(always)]
    pub const fn set_pt0_ac(&mut self, val: super::vals::Bfcrt012Pt0Ac) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Bfcrt012 {
    #[inline(always)]
    fn default() -> Bfcrt012 {
        Bfcrt012(0u64 as u16)
    }
}
impl core::fmt::Debug for Bfcrt012 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bfcrt012")
            .field("pt1_dc", &self.pt1_dc())
            .field("pt1_cc", &self.pt1_cc())
            .field("pt1_bc", &self.pt1_bc())
            .field("pt1_ac", &self.pt1_ac())
            .field("pt0_dc", &self.pt0_dc())
            .field("pt0_cc", &self.pt0_cc())
            .field("pt0_bc", &self.pt0_bc())
            .field("pt0_ac", &self.pt0_ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bfcrt012 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Bfcrt012 {
            pt1_dc: super::vals::Bfcrt012Pt1Dc,
            pt1_cc: super::vals::Bfcrt012Pt1Cc,
            pt1_bc: super::vals::Bfcrt012Pt1Bc,
            pt1_ac: super::vals::Bfcrt012Pt1Ac,
            pt0_dc: super::vals::Bfcrt012Pt0Dc,
            pt0_cc: super::vals::Bfcrt012Pt0Cc,
            pt0_bc: super::vals::Bfcrt012Pt0Bc,
            pt0_ac: super::vals::Bfcrt012Pt0Ac,
        }
        let proxy = Bfcrt012 {
            pt1_dc: self.pt1_dc(),
            pt1_cc: self.pt1_cc(),
            pt1_bc: self.pt1_bc(),
            pt1_ac: self.pt1_ac(),
            pt0_dc: self.pt0_dc(),
            pt0_cc: self.pt0_cc(),
            pt0_bc: self.pt0_bc(),
            pt0_ac: self.pt0_ac(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Boolean Function Term 0 and 1 Configuration for EVENT3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bfcrt013(pub u16);
impl Bfcrt013 {
    #[doc = "Product Term 1, Input D Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_dc(&self) -> super::vals::Bfcrt013Pt1Dc {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Bfcrt013Pt1Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 1, Input D Configuration"]
    #[inline(always)]
    pub const fn set_pt1_dc(&mut self, val: super::vals::Bfcrt013Pt1Dc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Product Term 1, Input C Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_cc(&self) -> super::vals::Bfcrt013Pt1Cc {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Bfcrt013Pt1Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 1, Input C Configuration"]
    #[inline(always)]
    pub const fn set_pt1_cc(&mut self, val: super::vals::Bfcrt013Pt1Cc) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Product Term 1, Input B Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_bc(&self) -> super::vals::Bfcrt013Pt1Bc {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Bfcrt013Pt1Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 1, Input B Configuration"]
    #[inline(always)]
    pub const fn set_pt1_bc(&mut self, val: super::vals::Bfcrt013Pt1Bc) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Product Term 1, Input A Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_ac(&self) -> super::vals::Bfcrt013Pt1Ac {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Bfcrt013Pt1Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 1, Input A Configuration"]
    #[inline(always)]
    pub const fn set_pt1_ac(&mut self, val: super::vals::Bfcrt013Pt1Ac) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "Product Term 0, Input D Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_dc(&self) -> super::vals::Bfcrt013Pt0Dc {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Bfcrt013Pt0Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 0, Input D Configuration"]
    #[inline(always)]
    pub const fn set_pt0_dc(&mut self, val: super::vals::Bfcrt013Pt0Dc) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "Product Term 0, Input C Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_cc(&self) -> super::vals::Bfcrt013Pt0Cc {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Bfcrt013Pt0Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 0, Input C Configuration"]
    #[inline(always)]
    pub const fn set_pt0_cc(&mut self, val: super::vals::Bfcrt013Pt0Cc) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u16) & 0x03) << 10usize);
    }
    #[doc = "Product Term 0, Input B Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_bc(&self) -> super::vals::Bfcrt013Pt0Bc {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Bfcrt013Pt0Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 0, Input B Configuration"]
    #[inline(always)]
    pub const fn set_pt0_bc(&mut self, val: super::vals::Bfcrt013Pt0Bc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
    #[doc = "Product Term 0, Input A Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_ac(&self) -> super::vals::Bfcrt013Pt0Ac {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Bfcrt013Pt0Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 0, Input A Configuration"]
    #[inline(always)]
    pub const fn set_pt0_ac(&mut self, val: super::vals::Bfcrt013Pt0Ac) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Bfcrt013 {
    #[inline(always)]
    fn default() -> Bfcrt013 {
        Bfcrt013(0u64 as u16)
    }
}
impl core::fmt::Debug for Bfcrt013 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bfcrt013")
            .field("pt1_dc", &self.pt1_dc())
            .field("pt1_cc", &self.pt1_cc())
            .field("pt1_bc", &self.pt1_bc())
            .field("pt1_ac", &self.pt1_ac())
            .field("pt0_dc", &self.pt0_dc())
            .field("pt0_cc", &self.pt0_cc())
            .field("pt0_bc", &self.pt0_bc())
            .field("pt0_ac", &self.pt0_ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bfcrt013 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Bfcrt013 {
            pt1_dc: super::vals::Bfcrt013Pt1Dc,
            pt1_cc: super::vals::Bfcrt013Pt1Cc,
            pt1_bc: super::vals::Bfcrt013Pt1Bc,
            pt1_ac: super::vals::Bfcrt013Pt1Ac,
            pt0_dc: super::vals::Bfcrt013Pt0Dc,
            pt0_cc: super::vals::Bfcrt013Pt0Cc,
            pt0_bc: super::vals::Bfcrt013Pt0Bc,
            pt0_ac: super::vals::Bfcrt013Pt0Ac,
        }
        let proxy = Bfcrt013 {
            pt1_dc: self.pt1_dc(),
            pt1_cc: self.pt1_cc(),
            pt1_bc: self.pt1_bc(),
            pt1_ac: self.pt1_ac(),
            pt0_dc: self.pt0_dc(),
            pt0_cc: self.pt0_cc(),
            pt0_bc: self.pt0_bc(),
            pt0_ac: self.pt0_ac(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Boolean Function Term 2 and 3 Configuration for EVENT0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bfcrt230(pub u16);
impl Bfcrt230 {
    #[doc = "Product Term 3, Input D Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_dc(&self) -> super::vals::Bfcrt230Pt3Dc {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Bfcrt230Pt3Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 3, Input D Configuration"]
    #[inline(always)]
    pub const fn set_pt3_dc(&mut self, val: super::vals::Bfcrt230Pt3Dc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Product Term 3, Input C Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_cc(&self) -> super::vals::Bfcrt230Pt3Cc {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Bfcrt230Pt3Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 3, Input C Configuration"]
    #[inline(always)]
    pub const fn set_pt3_cc(&mut self, val: super::vals::Bfcrt230Pt3Cc) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Product Term 3, Input B Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_bc(&self) -> super::vals::Bfcrt230Pt3Bc {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Bfcrt230Pt3Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 3, Input B Configuration"]
    #[inline(always)]
    pub const fn set_pt3_bc(&mut self, val: super::vals::Bfcrt230Pt3Bc) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Product Term 3, Input A Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_ac(&self) -> super::vals::Bfcrt230Pt3Ac {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Bfcrt230Pt3Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 3, Input A Configuration"]
    #[inline(always)]
    pub const fn set_pt3_ac(&mut self, val: super::vals::Bfcrt230Pt3Ac) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "Product Term 2, Input D Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_dc(&self) -> super::vals::Bfcrt230Pt2Dc {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Bfcrt230Pt2Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 2, Input D Configuration"]
    #[inline(always)]
    pub const fn set_pt2_dc(&mut self, val: super::vals::Bfcrt230Pt2Dc) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "Product Term 2, Input C Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_cc(&self) -> super::vals::Bfcrt230Pt2Cc {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Bfcrt230Pt2Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 2, Input C Configuration"]
    #[inline(always)]
    pub const fn set_pt2_cc(&mut self, val: super::vals::Bfcrt230Pt2Cc) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u16) & 0x03) << 10usize);
    }
    #[doc = "Product Term 2, Input B Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_bc(&self) -> super::vals::Bfcrt230Pt2Bc {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Bfcrt230Pt2Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 2, Input B Configuration"]
    #[inline(always)]
    pub const fn set_pt2_bc(&mut self, val: super::vals::Bfcrt230Pt2Bc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
    #[doc = "Product Term 2, Input A Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_ac(&self) -> super::vals::Bfcrt230Pt2Ac {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Bfcrt230Pt2Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 2, Input A Configuration"]
    #[inline(always)]
    pub const fn set_pt2_ac(&mut self, val: super::vals::Bfcrt230Pt2Ac) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Bfcrt230 {
    #[inline(always)]
    fn default() -> Bfcrt230 {
        Bfcrt230(0u64 as u16)
    }
}
impl core::fmt::Debug for Bfcrt230 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bfcrt230")
            .field("pt3_dc", &self.pt3_dc())
            .field("pt3_cc", &self.pt3_cc())
            .field("pt3_bc", &self.pt3_bc())
            .field("pt3_ac", &self.pt3_ac())
            .field("pt2_dc", &self.pt2_dc())
            .field("pt2_cc", &self.pt2_cc())
            .field("pt2_bc", &self.pt2_bc())
            .field("pt2_ac", &self.pt2_ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bfcrt230 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Bfcrt230 {
            pt3_dc: super::vals::Bfcrt230Pt3Dc,
            pt3_cc: super::vals::Bfcrt230Pt3Cc,
            pt3_bc: super::vals::Bfcrt230Pt3Bc,
            pt3_ac: super::vals::Bfcrt230Pt3Ac,
            pt2_dc: super::vals::Bfcrt230Pt2Dc,
            pt2_cc: super::vals::Bfcrt230Pt2Cc,
            pt2_bc: super::vals::Bfcrt230Pt2Bc,
            pt2_ac: super::vals::Bfcrt230Pt2Ac,
        }
        let proxy = Bfcrt230 {
            pt3_dc: self.pt3_dc(),
            pt3_cc: self.pt3_cc(),
            pt3_bc: self.pt3_bc(),
            pt3_ac: self.pt3_ac(),
            pt2_dc: self.pt2_dc(),
            pt2_cc: self.pt2_cc(),
            pt2_bc: self.pt2_bc(),
            pt2_ac: self.pt2_ac(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Boolean Function Term 2 and 3 Configuration for EVENT1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bfcrt231(pub u16);
impl Bfcrt231 {
    #[doc = "Product Term 3, Input D Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_dc(&self) -> super::vals::Bfcrt231Pt3Dc {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Bfcrt231Pt3Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 3, Input D Configuration"]
    #[inline(always)]
    pub const fn set_pt3_dc(&mut self, val: super::vals::Bfcrt231Pt3Dc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Product Term 3, Input C Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_cc(&self) -> super::vals::Bfcrt231Pt3Cc {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Bfcrt231Pt3Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 3, Input C Configuration"]
    #[inline(always)]
    pub const fn set_pt3_cc(&mut self, val: super::vals::Bfcrt231Pt3Cc) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Product Term 3, Input B Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_bc(&self) -> super::vals::Bfcrt231Pt3Bc {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Bfcrt231Pt3Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 3, Input B Configuration"]
    #[inline(always)]
    pub const fn set_pt3_bc(&mut self, val: super::vals::Bfcrt231Pt3Bc) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Product Term 3, Input A Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_ac(&self) -> super::vals::Bfcrt231Pt3Ac {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Bfcrt231Pt3Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 3, Input A Configuration"]
    #[inline(always)]
    pub const fn set_pt3_ac(&mut self, val: super::vals::Bfcrt231Pt3Ac) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "Product Term 2, Input D Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_dc(&self) -> super::vals::Bfcrt231Pt2Dc {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Bfcrt231Pt2Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 2, Input D Configuration"]
    #[inline(always)]
    pub const fn set_pt2_dc(&mut self, val: super::vals::Bfcrt231Pt2Dc) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "Product Term 2, Input C Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_cc(&self) -> super::vals::Bfcrt231Pt2Cc {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Bfcrt231Pt2Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 2, Input C Configuration"]
    #[inline(always)]
    pub const fn set_pt2_cc(&mut self, val: super::vals::Bfcrt231Pt2Cc) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u16) & 0x03) << 10usize);
    }
    #[doc = "Product Term 2, Input B Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_bc(&self) -> super::vals::Bfcrt231Pt2Bc {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Bfcrt231Pt2Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 2, Input B Configuration"]
    #[inline(always)]
    pub const fn set_pt2_bc(&mut self, val: super::vals::Bfcrt231Pt2Bc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
    #[doc = "Product Term 2, Input A Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_ac(&self) -> super::vals::Bfcrt231Pt2Ac {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Bfcrt231Pt2Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 2, Input A Configuration"]
    #[inline(always)]
    pub const fn set_pt2_ac(&mut self, val: super::vals::Bfcrt231Pt2Ac) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Bfcrt231 {
    #[inline(always)]
    fn default() -> Bfcrt231 {
        Bfcrt231(0u64 as u16)
    }
}
impl core::fmt::Debug for Bfcrt231 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bfcrt231")
            .field("pt3_dc", &self.pt3_dc())
            .field("pt3_cc", &self.pt3_cc())
            .field("pt3_bc", &self.pt3_bc())
            .field("pt3_ac", &self.pt3_ac())
            .field("pt2_dc", &self.pt2_dc())
            .field("pt2_cc", &self.pt2_cc())
            .field("pt2_bc", &self.pt2_bc())
            .field("pt2_ac", &self.pt2_ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bfcrt231 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Bfcrt231 {
            pt3_dc: super::vals::Bfcrt231Pt3Dc,
            pt3_cc: super::vals::Bfcrt231Pt3Cc,
            pt3_bc: super::vals::Bfcrt231Pt3Bc,
            pt3_ac: super::vals::Bfcrt231Pt3Ac,
            pt2_dc: super::vals::Bfcrt231Pt2Dc,
            pt2_cc: super::vals::Bfcrt231Pt2Cc,
            pt2_bc: super::vals::Bfcrt231Pt2Bc,
            pt2_ac: super::vals::Bfcrt231Pt2Ac,
        }
        let proxy = Bfcrt231 {
            pt3_dc: self.pt3_dc(),
            pt3_cc: self.pt3_cc(),
            pt3_bc: self.pt3_bc(),
            pt3_ac: self.pt3_ac(),
            pt2_dc: self.pt2_dc(),
            pt2_cc: self.pt2_cc(),
            pt2_bc: self.pt2_bc(),
            pt2_ac: self.pt2_ac(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Boolean Function Term 2 and 3 Configuration for EVENT2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bfcrt232(pub u16);
impl Bfcrt232 {
    #[doc = "Product Term 3, Input D Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_dc(&self) -> super::vals::Bfcrt232Pt3Dc {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Bfcrt232Pt3Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 3, Input D Configuration"]
    #[inline(always)]
    pub const fn set_pt3_dc(&mut self, val: super::vals::Bfcrt232Pt3Dc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Product Term 3, Input C Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_cc(&self) -> super::vals::Bfcrt232Pt3Cc {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Bfcrt232Pt3Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 3, Input C Configuration"]
    #[inline(always)]
    pub const fn set_pt3_cc(&mut self, val: super::vals::Bfcrt232Pt3Cc) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Product Term 3, Input B Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_bc(&self) -> super::vals::Bfcrt232Pt3Bc {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Bfcrt232Pt3Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 3, Input B Configuration"]
    #[inline(always)]
    pub const fn set_pt3_bc(&mut self, val: super::vals::Bfcrt232Pt3Bc) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Product Term 3, Input A Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_ac(&self) -> super::vals::Bfcrt232Pt3Ac {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Bfcrt232Pt3Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 3, Input A Configuration"]
    #[inline(always)]
    pub const fn set_pt3_ac(&mut self, val: super::vals::Bfcrt232Pt3Ac) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "Product Term 2, Input D Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_dc(&self) -> super::vals::Bfcrt232Pt2Dc {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Bfcrt232Pt2Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 2, Input D Configuration"]
    #[inline(always)]
    pub const fn set_pt2_dc(&mut self, val: super::vals::Bfcrt232Pt2Dc) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "Product Term 2, Input C Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_cc(&self) -> super::vals::Bfcrt232Pt2Cc {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Bfcrt232Pt2Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 2, Input C Configuration"]
    #[inline(always)]
    pub const fn set_pt2_cc(&mut self, val: super::vals::Bfcrt232Pt2Cc) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u16) & 0x03) << 10usize);
    }
    #[doc = "Product Term 2, Input B Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_bc(&self) -> super::vals::Bfcrt232Pt2Bc {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Bfcrt232Pt2Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 2, Input B Configuration"]
    #[inline(always)]
    pub const fn set_pt2_bc(&mut self, val: super::vals::Bfcrt232Pt2Bc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
    #[doc = "Product Term 2, Input A Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_ac(&self) -> super::vals::Bfcrt232Pt2Ac {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Bfcrt232Pt2Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 2, Input A Configuration"]
    #[inline(always)]
    pub const fn set_pt2_ac(&mut self, val: super::vals::Bfcrt232Pt2Ac) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Bfcrt232 {
    #[inline(always)]
    fn default() -> Bfcrt232 {
        Bfcrt232(0u64 as u16)
    }
}
impl core::fmt::Debug for Bfcrt232 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bfcrt232")
            .field("pt3_dc", &self.pt3_dc())
            .field("pt3_cc", &self.pt3_cc())
            .field("pt3_bc", &self.pt3_bc())
            .field("pt3_ac", &self.pt3_ac())
            .field("pt2_dc", &self.pt2_dc())
            .field("pt2_cc", &self.pt2_cc())
            .field("pt2_bc", &self.pt2_bc())
            .field("pt2_ac", &self.pt2_ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bfcrt232 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Bfcrt232 {
            pt3_dc: super::vals::Bfcrt232Pt3Dc,
            pt3_cc: super::vals::Bfcrt232Pt3Cc,
            pt3_bc: super::vals::Bfcrt232Pt3Bc,
            pt3_ac: super::vals::Bfcrt232Pt3Ac,
            pt2_dc: super::vals::Bfcrt232Pt2Dc,
            pt2_cc: super::vals::Bfcrt232Pt2Cc,
            pt2_bc: super::vals::Bfcrt232Pt2Bc,
            pt2_ac: super::vals::Bfcrt232Pt2Ac,
        }
        let proxy = Bfcrt232 {
            pt3_dc: self.pt3_dc(),
            pt3_cc: self.pt3_cc(),
            pt3_bc: self.pt3_bc(),
            pt3_ac: self.pt3_ac(),
            pt2_dc: self.pt2_dc(),
            pt2_cc: self.pt2_cc(),
            pt2_bc: self.pt2_bc(),
            pt2_ac: self.pt2_ac(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Boolean Function Term 2 and 3 Configuration for EVENT3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bfcrt233(pub u16);
impl Bfcrt233 {
    #[doc = "Product Term 3, Input D Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_dc(&self) -> super::vals::Bfcrt233Pt3Dc {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Bfcrt233Pt3Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 3, Input D Configuration"]
    #[inline(always)]
    pub const fn set_pt3_dc(&mut self, val: super::vals::Bfcrt233Pt3Dc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Product Term 3, Input C Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_cc(&self) -> super::vals::Bfcrt233Pt3Cc {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Bfcrt233Pt3Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 3, Input C Configuration"]
    #[inline(always)]
    pub const fn set_pt3_cc(&mut self, val: super::vals::Bfcrt233Pt3Cc) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Product Term 3, Input B Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_bc(&self) -> super::vals::Bfcrt233Pt3Bc {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Bfcrt233Pt3Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 3, Input B Configuration"]
    #[inline(always)]
    pub const fn set_pt3_bc(&mut self, val: super::vals::Bfcrt233Pt3Bc) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Product Term 3, Input A Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_ac(&self) -> super::vals::Bfcrt233Pt3Ac {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Bfcrt233Pt3Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 3, Input A Configuration"]
    #[inline(always)]
    pub const fn set_pt3_ac(&mut self, val: super::vals::Bfcrt233Pt3Ac) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "Product Term 2, Input D Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_dc(&self) -> super::vals::Bfcrt233Pt2Dc {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Bfcrt233Pt2Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 2, Input D Configuration"]
    #[inline(always)]
    pub const fn set_pt2_dc(&mut self, val: super::vals::Bfcrt233Pt2Dc) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "Product Term 2, Input C Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_cc(&self) -> super::vals::Bfcrt233Pt2Cc {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Bfcrt233Pt2Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 2, Input C Configuration"]
    #[inline(always)]
    pub const fn set_pt2_cc(&mut self, val: super::vals::Bfcrt233Pt2Cc) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u16) & 0x03) << 10usize);
    }
    #[doc = "Product Term 2, Input B Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_bc(&self) -> super::vals::Bfcrt233Pt2Bc {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Bfcrt233Pt2Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 2, Input B Configuration"]
    #[inline(always)]
    pub const fn set_pt2_bc(&mut self, val: super::vals::Bfcrt233Pt2Bc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
    #[doc = "Product Term 2, Input A Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_ac(&self) -> super::vals::Bfcrt233Pt2Ac {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Bfcrt233Pt2Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 2, Input A Configuration"]
    #[inline(always)]
    pub const fn set_pt2_ac(&mut self, val: super::vals::Bfcrt233Pt2Ac) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Bfcrt233 {
    #[inline(always)]
    fn default() -> Bfcrt233 {
        Bfcrt233(0u64 as u16)
    }
}
impl core::fmt::Debug for Bfcrt233 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bfcrt233")
            .field("pt3_dc", &self.pt3_dc())
            .field("pt3_cc", &self.pt3_cc())
            .field("pt3_bc", &self.pt3_bc())
            .field("pt3_ac", &self.pt3_ac())
            .field("pt2_dc", &self.pt2_dc())
            .field("pt2_cc", &self.pt2_cc())
            .field("pt2_bc", &self.pt2_bc())
            .field("pt2_ac", &self.pt2_ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bfcrt233 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Bfcrt233 {
            pt3_dc: super::vals::Bfcrt233Pt3Dc,
            pt3_cc: super::vals::Bfcrt233Pt3Cc,
            pt3_bc: super::vals::Bfcrt233Pt3Bc,
            pt3_ac: super::vals::Bfcrt233Pt3Ac,
            pt2_dc: super::vals::Bfcrt233Pt2Dc,
            pt2_cc: super::vals::Bfcrt233Pt2Cc,
            pt2_bc: super::vals::Bfcrt233Pt2Bc,
            pt2_ac: super::vals::Bfcrt233Pt2Ac,
        }
        let proxy = Bfcrt233 {
            pt3_dc: self.pt3_dc(),
            pt3_cc: self.pt3_cc(),
            pt3_bc: self.pt3_bc(),
            pt3_ac: self.pt3_ac(),
            pt2_dc: self.pt2_dc(),
            pt2_cc: self.pt2_cc(),
            pt2_bc: self.pt2_bc(),
            pt2_ac: self.pt2_ac(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
