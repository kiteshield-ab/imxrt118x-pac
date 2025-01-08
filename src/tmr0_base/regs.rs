#[doc = "Timer Alarm Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TmrAlarmCtrl(pub u32);
impl TmrAlarmCtrl {
    #[doc = "ALARM 1 pulse width selector"]
    #[must_use]
    #[inline(always)]
    pub const fn alarm1_pw(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "ALARM 1 pulse width selector"]
    #[inline(always)]
    pub const fn set_alarm1_pw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Alarm1 pulse generation time"]
    #[must_use]
    #[inline(always)]
    pub const fn pg1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Alarm1 pulse generation time"]
    #[inline(always)]
    pub const fn set_pg1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "ALARM 2 pulse width selector"]
    #[must_use]
    #[inline(always)]
    pub const fn alarm2_pw(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "ALARM 2 pulse width selector"]
    #[inline(always)]
    pub const fn set_alarm2_pw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Alarm2 pulse generation time"]
    #[must_use]
    #[inline(always)]
    pub const fn pg2(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Alarm2 pulse generation time"]
    #[inline(always)]
    pub const fn set_pg2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for TmrAlarmCtrl {
    #[inline(always)]
    fn default() -> TmrAlarmCtrl {
        TmrAlarmCtrl(0u64 as u32)
    }
}
impl core::fmt::Debug for TmrAlarmCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TmrAlarmCtrl")
            .field("alarm1_pw", &self.alarm1_pw())
            .field("pg1", &self.pg1())
            .field("alarm2_pw", &self.alarm2_pw())
            .field("pg2", &self.pg2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TmrAlarmCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TmrAlarmCtrl {
            alarm1_pw: u8,
            pg1: bool,
            alarm2_pw: u8,
            pg2: bool,
        }
        let proxy = TmrAlarmCtrl {
            alarm1_pw: self.alarm1_pw(),
            pg1: self.pg1(),
            alarm2_pw: self.alarm2_pw(),
            pg2: self.pg2(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Timer Capability Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TmrCapr(pub u32);
impl TmrCapr {
    #[doc = "IEEE 1722 support 0 Not supported 1 Supported"]
    #[must_use]
    #[inline(always)]
    pub const fn ieee_1722(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "IEEE 1722 support 0 Not supported 1 Supported"]
    #[inline(always)]
    pub const fn set_ieee_1722(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enhanced 1588 nanosecond (ns) Timer Adjustment supported"]
    #[must_use]
    #[inline(always)]
    pub const fn ecadj(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enhanced 1588 nanosecond (ns) Timer Adjustment supported"]
    #[inline(always)]
    pub const fn set_ecadj(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Support 802.1AS-2020 by providing both a ns and free-running clock to the Ethernet MACs."]
    #[must_use]
    #[inline(always)]
    pub const fn ieee_8021as_rev(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Support 802.1AS-2020 by providing both a ns and free-running clock to the Ethernet MACs."]
    #[inline(always)]
    pub const fn set_ieee_8021as_rev(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Number of MSI-x Vectors supported"]
    #[must_use]
    #[inline(always)]
    pub const fn num_msix(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Number of MSI-x Vectors supported"]
    #[inline(always)]
    pub const fn set_num_msix(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for TmrCapr {
    #[inline(always)]
    fn default() -> TmrCapr {
        TmrCapr(6u64 as u32)
    }
}
impl core::fmt::Debug for TmrCapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TmrCapr")
            .field("ieee_1722", &self.ieee_1722())
            .field("ecadj", &self.ecadj())
            .field("ieee_8021as_rev", &self.ieee_8021as_rev())
            .field("num_msix", &self.num_msix())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TmrCapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TmrCapr {
            ieee_1722: bool,
            ecadj: bool,
            ieee_8021as_rev: bool,
            num_msix: bool,
        }
        let proxy = TmrCapr {
            ieee_1722: self.ieee_1722(),
            ecadj: self.ecadj(),
            ieee_8021as_rev: self.ieee_8021as_rev(),
            num_msix: self.num_msix(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Timer Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TmrCtrl(pub u32);
impl TmrCtrl {
    #[doc = "1588 timer reference clock source select"]
    #[must_use]
    #[inline(always)]
    pub const fn ck_sel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "1588 timer reference clock source select"]
    #[inline(always)]
    pub const fn set_ck_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "1588 timer enable"]
    #[must_use]
    #[inline(always)]
    pub const fn te(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "1588 timer enable"]
    #[inline(always)]
    pub const fn set_te(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "External oscillator input clock phase"]
    #[must_use]
    #[inline(always)]
    pub const fn ciph(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "External oscillator input clock phase"]
    #[inline(always)]
    pub const fn set_ciph(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Generated clock (TMR_GCLK) output phase."]
    #[must_use]
    #[inline(always)]
    pub const fn coph(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Generated clock (TMR_GCLK) output phase."]
    #[inline(always)]
    pub const fn set_coph(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "External trigger 1 edge polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn etep1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "External trigger 1 edge polarity"]
    #[inline(always)]
    pub const fn set_etep1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "External trigger 2 edge polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn etep2(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "External trigger 2 edge polarity"]
    #[inline(always)]
    pub const fn set_etep2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Mode bit to allow atomic writes to TCLK_PERIOD and TMR_ADD"]
    #[must_use]
    #[inline(always)]
    pub const fn comp_mode(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Mode bit to allow atomic writes to TCLK_PERIOD and TMR_ADD"]
    #[inline(always)]
    pub const fn set_comp_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "1588 timer reference clock period"]
    #[must_use]
    #[inline(always)]
    pub const fn tclk_period(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "1588 timer reference clock period"]
    #[inline(always)]
    pub const fn set_tclk_period(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
    #[doc = "Fiper2 pulse loop back mode enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn pp2l(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Fiper2 pulse loop back mode enabled"]
    #[inline(always)]
    pub const fn set_pp2l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Fiper1 pulse loop back mode enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn pp1l(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Fiper1 pulse loop back mode enabled"]
    #[inline(always)]
    pub const fn set_pp1l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "FIPER start indication"]
    #[must_use]
    #[inline(always)]
    pub const fn fs(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "FIPER start indication"]
    #[inline(always)]
    pub const fn set_fs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "shadow Register disable"]
    #[must_use]
    #[inline(always)]
    pub const fn shadow_dis(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "shadow Register disable"]
    #[inline(always)]
    pub const fn set_shadow_dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Alarm2 output polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn alm2p(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Alarm2 output polarity"]
    #[inline(always)]
    pub const fn set_alm2p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Alarm1 output polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn alm1p(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Alarm1 output polarity"]
    #[inline(always)]
    pub const fn set_alm1p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for TmrCtrl {
    #[inline(always)]
    fn default() -> TmrCtrl {
        TmrCtrl(65537u64 as u32)
    }
}
impl core::fmt::Debug for TmrCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TmrCtrl")
            .field("ck_sel", &self.ck_sel())
            .field("te", &self.te())
            .field("ciph", &self.ciph())
            .field("coph", &self.coph())
            .field("etep1", &self.etep1())
            .field("etep2", &self.etep2())
            .field("comp_mode", &self.comp_mode())
            .field("tclk_period", &self.tclk_period())
            .field("pp2l", &self.pp2l())
            .field("pp1l", &self.pp1l())
            .field("fs", &self.fs())
            .field("shadow_dis", &self.shadow_dis())
            .field("alm2p", &self.alm2p())
            .field("alm1p", &self.alm1p())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TmrCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TmrCtrl {
            ck_sel: u8,
            te: bool,
            ciph: bool,
            coph: bool,
            etep1: bool,
            etep2: bool,
            comp_mode: bool,
            tclk_period: u16,
            pp2l: bool,
            pp1l: bool,
            fs: bool,
            shadow_dis: bool,
            alm2p: bool,
            alm1p: bool,
        }
        let proxy = TmrCtrl {
            ck_sel: self.ck_sel(),
            te: self.te(),
            ciph: self.ciph(),
            coph: self.coph(),
            etep1: self.etep1(),
            etep2: self.etep2(),
            comp_mode: self.comp_mode(),
            tclk_period: self.tclk_period(),
            pp2l: self.pp2l(),
            pp1l: self.pp1l(),
            fs: self.fs(),
            shadow_dis: self.shadow_dis(),
            alm2p: self.alm2p(),
            alm1p: self.alm1p(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Extended timer control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TmrEctrl(pub u32);
impl TmrEctrl {
    #[doc = "External trigger FIFO threshold."]
    #[must_use]
    #[inline(always)]
    pub const fn etff_thr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "External trigger FIFO threshold."]
    #[inline(always)]
    pub const fn set_etff_thr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for TmrEctrl {
    #[inline(always)]
    fn default() -> TmrEctrl {
        TmrEctrl(0u64 as u32)
    }
}
impl core::fmt::Debug for TmrEctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TmrEctrl")
            .field("etff_thr", &self.etff_thr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TmrEctrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TmrEctrl {
            etff_thr: u8,
        }
        let proxy = TmrEctrl {
            etff_thr: self.etff_thr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Timer FIPER Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TmrFiperCtrl(pub u32);
impl TmrFiperCtrl {
    #[doc = "FIPER 1 pulse width selection"]
    #[must_use]
    #[inline(always)]
    pub const fn fiper1_pw(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "FIPER 1 pulse width selection"]
    #[inline(always)]
    pub const fn set_fiper1_pw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "FIPER1 pulse generation select"]
    #[must_use]
    #[inline(always)]
    pub const fn pg1(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "FIPER1 pulse generation select"]
    #[inline(always)]
    pub const fn set_pg1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "FIPER1 disable"]
    #[must_use]
    #[inline(always)]
    pub const fn fiper1_dis(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "FIPER1 disable"]
    #[inline(always)]
    pub const fn set_fiper1_dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "FIPER 2 pulse width selection"]
    #[must_use]
    #[inline(always)]
    pub const fn fiper2_pw(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "FIPER 2 pulse width selection"]
    #[inline(always)]
    pub const fn set_fiper2_pw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "FIPER2 pulse generation time"]
    #[must_use]
    #[inline(always)]
    pub const fn pg2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "FIPER2 pulse generation time"]
    #[inline(always)]
    pub const fn set_pg2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "FIPER2 disable"]
    #[must_use]
    #[inline(always)]
    pub const fn fiper2_dis(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "FIPER2 disable"]
    #[inline(always)]
    pub const fn set_fiper2_dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "FIPER 3 pulse width selection"]
    #[must_use]
    #[inline(always)]
    pub const fn fiper3_pw(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "FIPER 3 pulse width selection"]
    #[inline(always)]
    pub const fn set_fiper3_pw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "FIPER3 pulse generation time"]
    #[must_use]
    #[inline(always)]
    pub const fn pg3(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "FIPER3 pulse generation time"]
    #[inline(always)]
    pub const fn set_pg3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "FIPER3 disable"]
    #[must_use]
    #[inline(always)]
    pub const fn fiper3_dis(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "FIPER3 disable"]
    #[inline(always)]
    pub const fn set_fiper3_dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for TmrFiperCtrl {
    #[inline(always)]
    fn default() -> TmrFiperCtrl {
        TmrFiperCtrl(4276545u64 as u32)
    }
}
impl core::fmt::Debug for TmrFiperCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TmrFiperCtrl")
            .field("fiper1_pw", &self.fiper1_pw())
            .field("pg1", &self.pg1())
            .field("fiper1_dis", &self.fiper1_dis())
            .field("fiper2_pw", &self.fiper2_pw())
            .field("pg2", &self.pg2())
            .field("fiper2_dis", &self.fiper2_dis())
            .field("fiper3_pw", &self.fiper3_pw())
            .field("pg3", &self.pg3())
            .field("fiper3_dis", &self.fiper3_dis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TmrFiperCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TmrFiperCtrl {
            fiper1_pw: u8,
            pg1: bool,
            fiper1_dis: bool,
            fiper2_pw: u8,
            pg2: bool,
            fiper2_dis: bool,
            fiper3_pw: u8,
            pg3: bool,
            fiper3_dis: bool,
        }
        let proxy = TmrFiperCtrl {
            fiper1_pw: self.fiper1_pw(),
            pg1: self.pg1(),
            fiper1_dis: self.fiper1_dis(),
            fiper2_pw: self.fiper2_pw(),
            pg2: self.pg2(),
            fiper2_dis: self.fiper2_dis(),
            fiper3_pw: self.fiper3_pw(),
            pg3: self.pg3(),
            fiper3_dis: self.fiper3_dis(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Module ID Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TmrId(pub u32);
impl TmrId {
    #[doc = "Minor revision"]
    #[must_use]
    #[inline(always)]
    pub const fn rev_mn(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Minor revision"]
    #[inline(always)]
    pub const fn set_rev_mn(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Major revision"]
    #[must_use]
    #[inline(always)]
    pub const fn rev_mj(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Major revision"]
    #[inline(always)]
    pub const fn set_rev_mj(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Timer IP ID"]
    #[must_use]
    #[inline(always)]
    pub const fn tmr_id(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Timer IP ID"]
    #[inline(always)]
    pub const fn set_tmr_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for TmrId {
    #[inline(always)]
    fn default() -> TmrId {
        TmrId(50332161u64 as u32)
    }
}
impl core::fmt::Debug for TmrId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TmrId")
            .field("rev_mn", &self.rev_mn())
            .field("rev_mj", &self.rev_mj())
            .field("tmr_id", &self.tmr_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TmrId {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TmrId {
            rev_mn: u8,
            rev_mj: u8,
            tmr_id: u16,
        }
        let proxy = TmrId {
            rev_mn: self.rev_mn(),
            rev_mj: self.rev_mj(),
            tmr_id: self.tmr_id(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Timer parameter register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TmrParam(pub u32);
impl TmrParam {
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
impl Default for TmrParam {
    #[inline(always)]
    fn default() -> TmrParam {
        TmrParam(0u64 as u32)
    }
}
impl core::fmt::Debug for TmrParam {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TmrParam")
            .field("sync", &self.sync())
            .field("param_val", &self.param_val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TmrParam {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TmrParam {
            sync: bool,
            param_val: u32,
        }
        let proxy = TmrParam {
            sync: self.sync(),
            param_val: self.param_val(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Timer prescale register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TmrPrsc(pub u32);
impl TmrPrsc {
    #[doc = "Output clock division prescale factor."]
    #[must_use]
    #[inline(always)]
    pub const fn prsc_ock(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Output clock division prescale factor."]
    #[inline(always)]
    pub const fn set_prsc_ock(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for TmrPrsc {
    #[inline(always)]
    fn default() -> TmrPrsc {
        TmrPrsc(2u64 as u32)
    }
}
impl core::fmt::Debug for TmrPrsc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TmrPrsc")
            .field("prsc_ock", &self.prsc_ock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TmrPrsc {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TmrPrsc {
            prsc_ock: u16,
        }
        let proxy = TmrPrsc {
            prsc_ock: self.prsc_ock(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Timer status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TmrStat(pub u32);
impl TmrStat {
    #[doc = "External trigger 1 Valid time-stamp 0 all valid external trigger time-stamps have been read 1 external trigger has an unread valid time-stamp value"]
    #[must_use]
    #[inline(always)]
    pub const fn ets1_vld(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "External trigger 1 Valid time-stamp 0 all valid external trigger time-stamps have been read 1 external trigger has an unread valid time-stamp value"]
    #[inline(always)]
    pub const fn set_ets1_vld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "External trigger 2 Valid time-stamp 0 all valid external trigger time-stamps have been read 1 external trigger has an unread valid time-stamp value"]
    #[must_use]
    #[inline(always)]
    pub const fn ets2_vld(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "External trigger 2 Valid time-stamp 0 all valid external trigger time-stamps have been read 1 external trigger has an unread valid time-stamp value"]
    #[inline(always)]
    pub const fn set_ets2_vld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Timer Reference Clock Detected"]
    #[must_use]
    #[inline(always)]
    pub const fn rcd(&self) -> super::vals::Rcd {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Rcd::from_bits(val as u8)
    }
    #[doc = "Timer Reference Clock Detected"]
    #[inline(always)]
    pub const fn set_rcd(&mut self, val: super::vals::Rcd) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for TmrStat {
    #[inline(always)]
    fn default() -> TmrStat {
        TmrStat(2147483648u64 as u32)
    }
}
impl core::fmt::Debug for TmrStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TmrStat")
            .field("ets1_vld", &self.ets1_vld())
            .field("ets2_vld", &self.ets2_vld())
            .field("rcd", &self.rcd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TmrStat {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TmrStat {
            ets1_vld: bool,
            ets2_vld: bool,
            rcd: super::vals::Rcd,
        }
        let proxy = TmrStat {
            ets1_vld: self.ets1_vld(),
            ets2_vld: self.ets2_vld(),
            rcd: self.rcd(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Timer event mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TmrTemask(pub u32);
impl TmrTemask {
    #[doc = "Periodic pulse event 3 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pp3en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Periodic pulse event 3 enable"]
    #[inline(always)]
    pub const fn set_pp3en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Periodic pulse event 2 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pp2en(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Periodic pulse event 2 enable"]
    #[inline(always)]
    pub const fn set_pp2en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Periodic pulse event 1 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pp1en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Periodic pulse event 1 enable"]
    #[inline(always)]
    pub const fn set_pp1en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Timer ALM2 event enable"]
    #[must_use]
    #[inline(always)]
    pub const fn alm1en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Timer ALM2 event enable"]
    #[inline(always)]
    pub const fn set_alm1en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Timer ALM1 event enable"]
    #[must_use]
    #[inline(always)]
    pub const fn alm2en(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Timer ALM1 event enable"]
    #[inline(always)]
    pub const fn set_alm2en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "External trigger 1 timestamp FIFO Threshold Level Hit event enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ets1_thren(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "External trigger 1 timestamp FIFO Threshold Level Hit event enable"]
    #[inline(always)]
    pub const fn set_ets1_thren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "External trigger 2 timestamp FIFO Threshold Level Hit event enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ets2_thren(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "External trigger 2 timestamp FIFO Threshold Level Hit event enable"]
    #[inline(always)]
    pub const fn set_ets2_thren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "External trigger 1 new timestamp sample event available interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ets1en(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "External trigger 1 new timestamp sample event available interrupt enable"]
    #[inline(always)]
    pub const fn set_ets1en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "External trigger 2 new timestamp sample event available interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ets2en(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "External trigger 2 new timestamp sample event available interrupt enable"]
    #[inline(always)]
    pub const fn set_ets2en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "External trigger 1 timestamp FIFO Overflow event interrupt enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn ets1_oven(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "External trigger 1 timestamp FIFO Overflow event interrupt enabled"]
    #[inline(always)]
    pub const fn set_ets1_oven(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "External trigger 2 timestamp FIFO Overflow event interrupt enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn ets2_oven(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "External trigger 2 timestamp FIFO Overflow event interrupt enabled"]
    #[inline(always)]
    pub const fn set_ets2_oven(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for TmrTemask {
    #[inline(always)]
    fn default() -> TmrTemask {
        TmrTemask(0u64 as u32)
    }
}
impl core::fmt::Debug for TmrTemask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TmrTemask")
            .field("pp3en", &self.pp3en())
            .field("pp2en", &self.pp2en())
            .field("pp1en", &self.pp1en())
            .field("alm1en", &self.alm1en())
            .field("alm2en", &self.alm2en())
            .field("ets1_thren", &self.ets1_thren())
            .field("ets2_thren", &self.ets2_thren())
            .field("ets1en", &self.ets1en())
            .field("ets2en", &self.ets2en())
            .field("ets1_oven", &self.ets1_oven())
            .field("ets2_oven", &self.ets2_oven())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TmrTemask {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TmrTemask {
            pp3en: bool,
            pp2en: bool,
            pp1en: bool,
            alm1en: bool,
            alm2en: bool,
            ets1_thren: bool,
            ets2_thren: bool,
            ets1en: bool,
            ets2en: bool,
            ets1_oven: bool,
            ets2_oven: bool,
        }
        let proxy = TmrTemask {
            pp3en: self.pp3en(),
            pp2en: self.pp2en(),
            pp1en: self.pp1en(),
            alm1en: self.alm1en(),
            alm2en: self.alm2en(),
            ets1_thren: self.ets1_thren(),
            ets2_thren: self.ets2_thren(),
            ets1en: self.ets1en(),
            ets2en: self.ets2en(),
            ets1_oven: self.ets1_oven(),
            ets2_oven: self.ets2_oven(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Timer Event Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TmrTevent(pub u32);
impl TmrTevent {
    #[doc = "Periodic pulse event 3 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pp3en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Periodic pulse event 3 enable"]
    #[inline(always)]
    pub const fn set_pp3en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Periodic pulse event 2 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pp2en(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Periodic pulse event 2 enable"]
    #[inline(always)]
    pub const fn set_pp2en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Periodic pulse event 1 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pp1en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Periodic pulse event 1 enable"]
    #[inline(always)]
    pub const fn set_pp1en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Timer ALM2 event enable"]
    #[must_use]
    #[inline(always)]
    pub const fn alm1en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Timer ALM2 event enable"]
    #[inline(always)]
    pub const fn set_alm1en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Timer ALM1 event enable"]
    #[must_use]
    #[inline(always)]
    pub const fn alm2en(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Timer ALM1 event enable"]
    #[inline(always)]
    pub const fn set_alm2en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "External trigger 1 timestamp FIFO Threshold Level Hit"]
    #[must_use]
    #[inline(always)]
    pub const fn ets1_thren(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "External trigger 1 timestamp FIFO Threshold Level Hit"]
    #[inline(always)]
    pub const fn set_ets1_thren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "External trigger 2 timestamp FIFO Threshold Level Hit"]
    #[must_use]
    #[inline(always)]
    pub const fn ets2_thren(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "External trigger 2 timestamp FIFO Threshold Level Hit"]
    #[inline(always)]
    pub const fn set_ets2_thren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "External trigger 1 new timestamp sample event available"]
    #[must_use]
    #[inline(always)]
    pub const fn ets1en(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "External trigger 1 new timestamp sample event available"]
    #[inline(always)]
    pub const fn set_ets1en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "External trigger 2 new timestamp sample event available"]
    #[must_use]
    #[inline(always)]
    pub const fn ets2en(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "External trigger 2 new timestamp sample event available"]
    #[inline(always)]
    pub const fn set_ets2en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "External trigger 1 timestamp FIFO Overflow event occurred"]
    #[must_use]
    #[inline(always)]
    pub const fn ets1_oven(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "External trigger 1 timestamp FIFO Overflow event occurred"]
    #[inline(always)]
    pub const fn set_ets1_oven(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "External trigger 2 timestamp FIFO Overflow event occurred"]
    #[must_use]
    #[inline(always)]
    pub const fn ets2_oven(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "External trigger 2 timestamp FIFO Overflow event occurred"]
    #[inline(always)]
    pub const fn set_ets2_oven(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for TmrTevent {
    #[inline(always)]
    fn default() -> TmrTevent {
        TmrTevent(0u64 as u32)
    }
}
impl core::fmt::Debug for TmrTevent {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TmrTevent")
            .field("pp3en", &self.pp3en())
            .field("pp2en", &self.pp2en())
            .field("pp1en", &self.pp1en())
            .field("alm1en", &self.alm1en())
            .field("alm2en", &self.alm2en())
            .field("ets1_thren", &self.ets1_thren())
            .field("ets2_thren", &self.ets2_thren())
            .field("ets1en", &self.ets1en())
            .field("ets2en", &self.ets2en())
            .field("ets1_oven", &self.ets1_oven())
            .field("ets2_oven", &self.ets2_oven())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TmrTevent {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TmrTevent {
            pp3en: bool,
            pp2en: bool,
            pp1en: bool,
            alm1en: bool,
            alm2en: bool,
            ets1_thren: bool,
            ets2_thren: bool,
            ets1en: bool,
            ets2en: bool,
            ets1_oven: bool,
            ets2_oven: bool,
        }
        let proxy = TmrTevent {
            pp3en: self.pp3en(),
            pp2en: self.pp2en(),
            pp1en: self.pp1en(),
            alm1en: self.alm1en(),
            alm2en: self.alm2en(),
            ets1_thren: self.ets1_thren(),
            ets2_thren: self.ets2_thren(),
            ets1en: self.ets1en(),
            ets2en: self.ets2en(),
            ets1_oven: self.ets1_oven(),
            ets2_oven: self.ets2_oven(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
