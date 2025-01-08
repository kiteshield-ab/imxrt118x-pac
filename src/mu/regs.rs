#[doc = "Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc = "MU Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn mur(&self) -> super::vals::Mur {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Mur::from_bits(val as u8)
    }
    #[doc = "MU Reset"]
    #[inline(always)]
    pub const fn set_mur(&mut self, val: super::vals::Mur) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "MUA Reset Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn murie(&self) -> super::vals::Murie {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Murie::from_bits(val as u8)
    }
    #[doc = "MUA Reset Interrupt Enable"]
    #[inline(always)]
    pub const fn set_murie(&mut self, val: super::vals::Murie) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for Cr {
    #[inline(always)]
    fn default() -> Cr {
        Cr(0u64 as u32)
    }
}
impl core::fmt::Debug for Cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr")
            .field("mur", &self.mur())
            .field("murie", &self.murie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cr {
            mur: super::vals::Mur,
            murie: super::vals::Murie,
        }
        let proxy = Cr {
            mur: self.mur(),
            murie: self.murie(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Flag Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcr(pub u32);
impl Fcr {
    #[doc = "MUA to MUB Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn f0(&self) -> super::vals::FcrF0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::FcrF0::from_bits(val as u8)
    }
    #[doc = "MUA to MUB Flag"]
    #[inline(always)]
    pub const fn set_f0(&mut self, val: super::vals::FcrF0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "MUA to MUB Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn f1(&self) -> super::vals::FcrF1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::FcrF1::from_bits(val as u8)
    }
    #[doc = "MUA to MUB Flag"]
    #[inline(always)]
    pub const fn set_f1(&mut self, val: super::vals::FcrF1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "MUA to MUB Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn f2(&self) -> super::vals::FcrF2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::FcrF2::from_bits(val as u8)
    }
    #[doc = "MUA to MUB Flag"]
    #[inline(always)]
    pub const fn set_f2(&mut self, val: super::vals::FcrF2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
}
impl Default for Fcr {
    #[inline(always)]
    fn default() -> Fcr {
        Fcr(0u64 as u32)
    }
}
impl core::fmt::Debug for Fcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fcr")
            .field("f0", &self.f0())
            .field("f1", &self.f1())
            .field("f2", &self.f2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Fcr {
            f0: super::vals::FcrF0,
            f1: super::vals::FcrF1,
            f2: super::vals::FcrF2,
        }
        let proxy = Fcr {
            f0: self.f0(),
            f1: self.f1(),
            f2: self.f2(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Flag Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fsr(pub u32);
impl Fsr {
    #[doc = "MUB to MUA-Side Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn f0(&self) -> super::vals::FsrF0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::FsrF0::from_bits(val as u8)
    }
    #[doc = "MUB to MUA-Side Flag"]
    #[inline(always)]
    pub const fn set_f0(&mut self, val: super::vals::FsrF0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "MUB to MUA-Side Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn f1(&self) -> super::vals::FsrF1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::FsrF1::from_bits(val as u8)
    }
    #[doc = "MUB to MUA-Side Flag"]
    #[inline(always)]
    pub const fn set_f1(&mut self, val: super::vals::FsrF1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "MUB to MUA-Side Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn f2(&self) -> super::vals::FsrF2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::FsrF2::from_bits(val as u8)
    }
    #[doc = "MUB to MUA-Side Flag"]
    #[inline(always)]
    pub const fn set_f2(&mut self, val: super::vals::FsrF2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
}
impl Default for Fsr {
    #[inline(always)]
    fn default() -> Fsr {
        Fsr(0u64 as u32)
    }
}
impl core::fmt::Debug for Fsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fsr")
            .field("f0", &self.f0())
            .field("f1", &self.f1())
            .field("f2", &self.f2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fsr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Fsr {
            f0: super::vals::FsrF0,
            f1: super::vals::FsrF1,
            f2: super::vals::FsrF2,
        }
        let proxy = Fsr {
            f0: self.f0(),
            f1: self.f1(),
            f2: self.f2(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "General-Purpose Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gcr(pub u32);
impl Gcr {
    #[doc = "MUA General-Purpose Interrupt Request"]
    #[must_use]
    #[inline(always)]
    pub const fn gir0(&self) -> super::vals::Gir0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Gir0::from_bits(val as u8)
    }
    #[doc = "MUA General-Purpose Interrupt Request"]
    #[inline(always)]
    pub const fn set_gir0(&mut self, val: super::vals::Gir0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "MUA General-Purpose Interrupt Request"]
    #[must_use]
    #[inline(always)]
    pub const fn gir1(&self) -> super::vals::Gir1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Gir1::from_bits(val as u8)
    }
    #[doc = "MUA General-Purpose Interrupt Request"]
    #[inline(always)]
    pub const fn set_gir1(&mut self, val: super::vals::Gir1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "MUA General-Purpose Interrupt Request"]
    #[must_use]
    #[inline(always)]
    pub const fn gir2(&self) -> super::vals::Gir2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Gir2::from_bits(val as u8)
    }
    #[doc = "MUA General-Purpose Interrupt Request"]
    #[inline(always)]
    pub const fn set_gir2(&mut self, val: super::vals::Gir2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "MUA General-Purpose Interrupt Request"]
    #[must_use]
    #[inline(always)]
    pub const fn gir3(&self) -> super::vals::Gir3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Gir3::from_bits(val as u8)
    }
    #[doc = "MUA General-Purpose Interrupt Request"]
    #[inline(always)]
    pub const fn set_gir3(&mut self, val: super::vals::Gir3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for Gcr {
    #[inline(always)]
    fn default() -> Gcr {
        Gcr(0u64 as u32)
    }
}
impl core::fmt::Debug for Gcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gcr")
            .field("gir0", &self.gir0())
            .field("gir1", &self.gir1())
            .field("gir2", &self.gir2())
            .field("gir3", &self.gir3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gcr {
            gir0: super::vals::Gir0,
            gir1: super::vals::Gir1,
            gir2: super::vals::Gir2,
            gir3: super::vals::Gir3,
        }
        let proxy = Gcr {
            gir0: self.gir0(),
            gir1: self.gir1(),
            gir2: self.gir2(),
            gir3: self.gir3(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "General-Purpose Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gier(pub u32);
impl Gier {
    #[doc = "MUA General-purpose Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gie0(&self) -> super::vals::Gie0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Gie0::from_bits(val as u8)
    }
    #[doc = "MUA General-purpose Interrupt Enable"]
    #[inline(always)]
    pub const fn set_gie0(&mut self, val: super::vals::Gie0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "MUA General-purpose Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gie1(&self) -> super::vals::Gie1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Gie1::from_bits(val as u8)
    }
    #[doc = "MUA General-purpose Interrupt Enable"]
    #[inline(always)]
    pub const fn set_gie1(&mut self, val: super::vals::Gie1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "MUA General-purpose Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gie2(&self) -> super::vals::Gie2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Gie2::from_bits(val as u8)
    }
    #[doc = "MUA General-purpose Interrupt Enable"]
    #[inline(always)]
    pub const fn set_gie2(&mut self, val: super::vals::Gie2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "MUA General-purpose Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gie3(&self) -> super::vals::Gie3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Gie3::from_bits(val as u8)
    }
    #[doc = "MUA General-purpose Interrupt Enable"]
    #[inline(always)]
    pub const fn set_gie3(&mut self, val: super::vals::Gie3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for Gier {
    #[inline(always)]
    fn default() -> Gier {
        Gier(0u64 as u32)
    }
}
impl core::fmt::Debug for Gier {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gier")
            .field("gie0", &self.gie0())
            .field("gie1", &self.gie1())
            .field("gie2", &self.gie2())
            .field("gie3", &self.gie3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gier {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gier {
            gie0: super::vals::Gie0,
            gie1: super::vals::Gie1,
            gie2: super::vals::Gie2,
            gie3: super::vals::Gie3,
        }
        let proxy = Gier {
            gie0: self.gie0(),
            gie1: self.gie1(),
            gie2: self.gie2(),
            gie3: self.gie3(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "General-purpose Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gsr(pub u32);
impl Gsr {
    #[doc = "MUA General-Purpose Interrupt Request Pending"]
    #[must_use]
    #[inline(always)]
    pub const fn gip0(&self) -> super::vals::Gip0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Gip0::from_bits(val as u8)
    }
    #[doc = "MUA General-Purpose Interrupt Request Pending"]
    #[inline(always)]
    pub const fn set_gip0(&mut self, val: super::vals::Gip0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "MUA General-Purpose Interrupt Request Pending"]
    #[must_use]
    #[inline(always)]
    pub const fn gip1(&self) -> super::vals::Gip1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Gip1::from_bits(val as u8)
    }
    #[doc = "MUA General-Purpose Interrupt Request Pending"]
    #[inline(always)]
    pub const fn set_gip1(&mut self, val: super::vals::Gip1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "MUA General-Purpose Interrupt Request Pending"]
    #[must_use]
    #[inline(always)]
    pub const fn gip2(&self) -> super::vals::Gip2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Gip2::from_bits(val as u8)
    }
    #[doc = "MUA General-Purpose Interrupt Request Pending"]
    #[inline(always)]
    pub const fn set_gip2(&mut self, val: super::vals::Gip2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "MUA General-Purpose Interrupt Request Pending"]
    #[must_use]
    #[inline(always)]
    pub const fn gip3(&self) -> super::vals::Gip3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Gip3::from_bits(val as u8)
    }
    #[doc = "MUA General-Purpose Interrupt Request Pending"]
    #[inline(always)]
    pub const fn set_gip3(&mut self, val: super::vals::Gip3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for Gsr {
    #[inline(always)]
    fn default() -> Gsr {
        Gsr(0u64 as u32)
    }
}
impl core::fmt::Debug for Gsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gsr")
            .field("gip0", &self.gip0())
            .field("gip1", &self.gip1())
            .field("gip2", &self.gip2())
            .field("gip3", &self.gip3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gsr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gsr {
            gip0: super::vals::Gip0,
            gip1: super::vals::Gip1,
            gip2: super::vals::Gip2,
            gip3: super::vals::Gip3,
        }
        let proxy = Gsr {
            gip0: self.gip0(),
            gip1: self.gip1(),
            gip2: self.gip2(),
            gip3: self.gip3(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Parameter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Par(pub u32);
impl Par {
    #[doc = "Transmit Register Number"]
    #[must_use]
    #[inline(always)]
    pub const fn tr_num(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Transmit Register Number"]
    #[inline(always)]
    pub const fn set_tr_num(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Receive Register Number"]
    #[must_use]
    #[inline(always)]
    pub const fn rr_num(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Receive Register Number"]
    #[inline(always)]
    pub const fn set_rr_num(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "General-Purpose Interrupt Request Number"]
    #[must_use]
    #[inline(always)]
    pub const fn gir_num(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "General-Purpose Interrupt Request Number"]
    #[inline(always)]
    pub const fn set_gir_num(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Flag Width"]
    #[must_use]
    #[inline(always)]
    pub const fn flag_width(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Flag Width"]
    #[inline(always)]
    pub const fn set_flag_width(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Par {
    #[inline(always)]
    fn default() -> Par {
        Par(50594820u64 as u32)
    }
}
impl core::fmt::Debug for Par {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Par")
            .field("tr_num", &self.tr_num())
            .field("rr_num", &self.rr_num())
            .field("gir_num", &self.gir_num())
            .field("flag_width", &self.flag_width())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Par {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Par {
            tr_num: u8,
            rr_num: u8,
            gir_num: u8,
            flag_width: u8,
        }
        let proxy = Par {
            tr_num: self.tr_num(),
            rr_num: self.rr_num(),
            gir_num: self.gir_num(),
            flag_width: self.flag_width(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Receive Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcr(pub u32);
impl Rcr {
    #[doc = "MUA Receive Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rie0(&self) -> super::vals::Rie0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Rie0::from_bits(val as u8)
    }
    #[doc = "MUA Receive Interrupt Enable"]
    #[inline(always)]
    pub const fn set_rie0(&mut self, val: super::vals::Rie0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "MUA Receive Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rie1(&self) -> super::vals::Rie1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Rie1::from_bits(val as u8)
    }
    #[doc = "MUA Receive Interrupt Enable"]
    #[inline(always)]
    pub const fn set_rie1(&mut self, val: super::vals::Rie1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "MUA Receive Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rie2(&self) -> super::vals::Rie2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Rie2::from_bits(val as u8)
    }
    #[doc = "MUA Receive Interrupt Enable"]
    #[inline(always)]
    pub const fn set_rie2(&mut self, val: super::vals::Rie2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "MUA Receive Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rie3(&self) -> super::vals::Rie3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Rie3::from_bits(val as u8)
    }
    #[doc = "MUA Receive Interrupt Enable"]
    #[inline(always)]
    pub const fn set_rie3(&mut self, val: super::vals::Rie3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for Rcr {
    #[inline(always)]
    fn default() -> Rcr {
        Rcr(0u64 as u32)
    }
}
impl core::fmt::Debug for Rcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rcr")
            .field("rie0", &self.rie0())
            .field("rie1", &self.rie1())
            .field("rie2", &self.rie2())
            .field("rie3", &self.rie3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Rcr {
            rie0: super::vals::Rie0,
            rie1: super::vals::Rie1,
            rie2: super::vals::Rie2,
            rie3: super::vals::Rie3,
        }
        let proxy = Rcr {
            rie0: self.rie0(),
            rie1: self.rie1(),
            rie2: self.rie2(),
            rie3: self.rie3(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Receive Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsr(pub u32);
impl Rsr {
    #[doc = "MUA Receive Register Full"]
    #[must_use]
    #[inline(always)]
    pub const fn rf0(&self) -> super::vals::Rf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Rf0::from_bits(val as u8)
    }
    #[doc = "MUA Receive Register Full"]
    #[inline(always)]
    pub const fn set_rf0(&mut self, val: super::vals::Rf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "MUA Receive Register Full"]
    #[must_use]
    #[inline(always)]
    pub const fn rf1(&self) -> super::vals::Rf1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Rf1::from_bits(val as u8)
    }
    #[doc = "MUA Receive Register Full"]
    #[inline(always)]
    pub const fn set_rf1(&mut self, val: super::vals::Rf1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "MUA Receive Register Full"]
    #[must_use]
    #[inline(always)]
    pub const fn rf2(&self) -> super::vals::Rf2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Rf2::from_bits(val as u8)
    }
    #[doc = "MUA Receive Register Full"]
    #[inline(always)]
    pub const fn set_rf2(&mut self, val: super::vals::Rf2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "MUA Receive Register Full"]
    #[must_use]
    #[inline(always)]
    pub const fn rf3(&self) -> super::vals::Rf3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Rf3::from_bits(val as u8)
    }
    #[doc = "MUA Receive Register Full"]
    #[inline(always)]
    pub const fn set_rf3(&mut self, val: super::vals::Rf3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for Rsr {
    #[inline(always)]
    fn default() -> Rsr {
        Rsr(0u64 as u32)
    }
}
impl core::fmt::Debug for Rsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsr")
            .field("rf0", &self.rf0())
            .field("rf1", &self.rf1())
            .field("rf2", &self.rf2())
            .field("rf3", &self.rf3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Rsr {
            rf0: super::vals::Rf0,
            rf1: super::vals::Rf1,
            rf2: super::vals::Rf2,
            rf3: super::vals::Rf3,
        }
        let proxy = Rsr {
            rf0: self.rf0(),
            rf1: self.rf1(),
            rf2: self.rf2(),
            rf3: self.rf3(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "MUA and MUB Reset State"]
    #[must_use]
    #[inline(always)]
    pub const fn murs(&self) -> super::vals::Murs {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Murs::from_bits(val as u8)
    }
    #[doc = "MUA and MUB Reset State"]
    #[inline(always)]
    pub const fn set_murs(&mut self, val: super::vals::Murs) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "MU Reset Interrupt Pending Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn murip(&self) -> super::vals::Murip {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Murip::from_bits(val as u8)
    }
    #[doc = "MU Reset Interrupt Pending Flag"]
    #[inline(always)]
    pub const fn set_murip(&mut self, val: super::vals::Murip) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "MUA Side Event Pending"]
    #[must_use]
    #[inline(always)]
    pub const fn ep(&self) -> super::vals::Ep {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Ep::from_bits(val as u8)
    }
    #[doc = "MUA Side Event Pending"]
    #[inline(always)]
    pub const fn set_ep(&mut self, val: super::vals::Ep) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "MUA Flag Update Pending"]
    #[must_use]
    #[inline(always)]
    pub const fn fup(&self) -> super::vals::Fup {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Fup::from_bits(val as u8)
    }
    #[doc = "MUA Flag Update Pending"]
    #[inline(always)]
    pub const fn set_fup(&mut self, val: super::vals::Fup) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "MUA General-Purpose Interrupt Pending"]
    #[must_use]
    #[inline(always)]
    pub const fn girp(&self) -> super::vals::Girp {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Girp::from_bits(val as u8)
    }
    #[doc = "MUA General-Purpose Interrupt Pending"]
    #[inline(always)]
    pub const fn set_girp(&mut self, val: super::vals::Girp) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "MUA Transmit Empty Pending"]
    #[must_use]
    #[inline(always)]
    pub const fn tep(&self) -> super::vals::Tep {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Tep::from_bits(val as u8)
    }
    #[doc = "MUA Transmit Empty Pending"]
    #[inline(always)]
    pub const fn set_tep(&mut self, val: super::vals::Tep) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "MUA Receive Full Pending"]
    #[must_use]
    #[inline(always)]
    pub const fn rfp(&self) -> super::vals::Rfp {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Rfp::from_bits(val as u8)
    }
    #[doc = "MUA Receive Full Pending"]
    #[inline(always)]
    pub const fn set_rfp(&mut self, val: super::vals::Rfp) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
}
impl Default for Sr {
    #[inline(always)]
    fn default() -> Sr {
        Sr(0u64 as u32)
    }
}
impl core::fmt::Debug for Sr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sr")
            .field("murs", &self.murs())
            .field("murip", &self.murip())
            .field("ep", &self.ep())
            .field("fup", &self.fup())
            .field("girp", &self.girp())
            .field("tep", &self.tep())
            .field("rfp", &self.rfp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sr {
            murs: super::vals::Murs,
            murip: super::vals::Murip,
            ep: super::vals::Ep,
            fup: super::vals::Fup,
            girp: super::vals::Girp,
            tep: super::vals::Tep,
            rfp: super::vals::Rfp,
        }
        let proxy = Sr {
            murs: self.murs(),
            murip: self.murip(),
            ep: self.ep(),
            fup: self.fup(),
            girp: self.girp(),
            tep: self.tep(),
            rfp: self.rfp(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Transmit Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcr(pub u32);
impl Tcr {
    #[doc = "MUA Transmit Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tie0(&self) -> super::vals::Tie0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Tie0::from_bits(val as u8)
    }
    #[doc = "MUA Transmit Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tie0(&mut self, val: super::vals::Tie0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "MUA Transmit Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tie1(&self) -> super::vals::Tie1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Tie1::from_bits(val as u8)
    }
    #[doc = "MUA Transmit Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tie1(&mut self, val: super::vals::Tie1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "MUA Transmit Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tie2(&self) -> super::vals::Tie2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Tie2::from_bits(val as u8)
    }
    #[doc = "MUA Transmit Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tie2(&mut self, val: super::vals::Tie2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "MUA Transmit Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tie3(&self) -> super::vals::Tie3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Tie3::from_bits(val as u8)
    }
    #[doc = "MUA Transmit Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tie3(&mut self, val: super::vals::Tie3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for Tcr {
    #[inline(always)]
    fn default() -> Tcr {
        Tcr(0u64 as u32)
    }
}
impl core::fmt::Debug for Tcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcr")
            .field("tie0", &self.tie0())
            .field("tie1", &self.tie1())
            .field("tie2", &self.tie2())
            .field("tie3", &self.tie3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Tcr {
            tie0: super::vals::Tie0,
            tie1: super::vals::Tie1,
            tie2: super::vals::Tie2,
            tie3: super::vals::Tie3,
        }
        let proxy = Tcr {
            tie0: self.tie0(),
            tie1: self.tie1(),
            tie2: self.tie2(),
            tie3: self.tie3(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Transmit Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsr(pub u32);
impl Tsr {
    #[doc = "MUA Transmit Empty"]
    #[must_use]
    #[inline(always)]
    pub const fn te0(&self) -> super::vals::Te0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Te0::from_bits(val as u8)
    }
    #[doc = "MUA Transmit Empty"]
    #[inline(always)]
    pub const fn set_te0(&mut self, val: super::vals::Te0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "MUA Transmit Empty"]
    #[must_use]
    #[inline(always)]
    pub const fn te1(&self) -> super::vals::Te1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Te1::from_bits(val as u8)
    }
    #[doc = "MUA Transmit Empty"]
    #[inline(always)]
    pub const fn set_te1(&mut self, val: super::vals::Te1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "MUA Transmit Empty"]
    #[must_use]
    #[inline(always)]
    pub const fn te2(&self) -> super::vals::Te2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Te2::from_bits(val as u8)
    }
    #[doc = "MUA Transmit Empty"]
    #[inline(always)]
    pub const fn set_te2(&mut self, val: super::vals::Te2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "MUA Transmit Empty"]
    #[must_use]
    #[inline(always)]
    pub const fn te3(&self) -> super::vals::Te3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Te3::from_bits(val as u8)
    }
    #[doc = "MUA Transmit Empty"]
    #[inline(always)]
    pub const fn set_te3(&mut self, val: super::vals::Te3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for Tsr {
    #[inline(always)]
    fn default() -> Tsr {
        Tsr(15u64 as u32)
    }
}
impl core::fmt::Debug for Tsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tsr")
            .field("te0", &self.te0())
            .field("te1", &self.te1())
            .field("te2", &self.te2())
            .field("te3", &self.te3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tsr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Tsr {
            te0: super::vals::Te0,
            te1: super::vals::Te1,
            te2: super::vals::Te2,
            te3: super::vals::Te3,
        }
        let proxy = Tsr {
            te0: self.te0(),
            te1: self.te1(),
            te2: self.te2(),
            te3: self.te3(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Version ID"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ver(pub u32);
impl Ver {
    #[doc = "Feature Set Number"]
    #[must_use]
    #[inline(always)]
    pub const fn feature(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Feature Set Number"]
    #[inline(always)]
    pub const fn set_feature(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
impl Default for Ver {
    #[inline(always)]
    fn default() -> Ver {
        Ver(50921487u64 as u32)
    }
}
impl core::fmt::Debug for Ver {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ver")
            .field("feature", &self.feature())
            .field("minor", &self.minor())
            .field("major", &self.major())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ver {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ver {
            feature: u16,
            minor: u8,
            major: u8,
        }
        let proxy = Ver {
            feature: self.feature(),
            minor: self.minor(),
            major: self.major(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
