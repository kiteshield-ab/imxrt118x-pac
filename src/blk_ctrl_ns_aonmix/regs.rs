#[doc = "DCDC status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcStatus(pub u32);
impl DcdcStatus {
    #[doc = "DCDC captured status clear"]
    #[must_use]
    #[inline(always)]
    pub const fn dcdc_status_capt_clr(&self) -> super::vals::DcdcStatusCaptClr {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::DcdcStatusCaptClr::from_bits(val as u8)
    }
    #[doc = "DCDC captured status clear"]
    #[inline(always)]
    pub const fn set_dcdc_status_capt_clr(&mut self, val: super::vals::DcdcStatusCaptClr) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "DCDC_IN low voltage detect"]
    #[must_use]
    #[inline(always)]
    pub const fn dcdc_in_low_vol(&self) -> super::vals::DcdcInLowVol {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::DcdcInLowVol::from_bits(val as u8)
    }
    #[doc = "DCDC_IN low voltage detect"]
    #[inline(always)]
    pub const fn set_dcdc_in_low_vol(&mut self, val: super::vals::DcdcInLowVol) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "DCDC output over current alert"]
    #[must_use]
    #[inline(always)]
    pub const fn dcdc_over_cur(&self) -> super::vals::DcdcOverCur {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::DcdcOverCur::from_bits(val as u8)
    }
    #[doc = "DCDC output over current alert"]
    #[inline(always)]
    pub const fn set_dcdc_over_cur(&mut self, val: super::vals::DcdcOverCur) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "DCDC output over voltage alert"]
    #[must_use]
    #[inline(always)]
    pub const fn dcdc_over_vol(&self) -> super::vals::DcdcOverVol {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::DcdcOverVol::from_bits(val as u8)
    }
    #[doc = "DCDC output over voltage alert"]
    #[inline(always)]
    pub const fn set_dcdc_over_vol(&mut self, val: super::vals::DcdcOverVol) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "DCDC status OK"]
    #[must_use]
    #[inline(always)]
    pub const fn dcdc_sts_dc_ok(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "DCDC status OK"]
    #[inline(always)]
    pub const fn set_dcdc_sts_dc_ok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for DcdcStatus {
    #[inline(always)]
    fn default() -> DcdcStatus {
        DcdcStatus(0u64 as u32)
    }
}
impl core::fmt::Debug for DcdcStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DcdcStatus")
            .field("dcdc_status_capt_clr", &self.dcdc_status_capt_clr())
            .field("dcdc_in_low_vol", &self.dcdc_in_low_vol())
            .field("dcdc_over_cur", &self.dcdc_over_cur())
            .field("dcdc_over_vol", &self.dcdc_over_vol())
            .field("dcdc_sts_dc_ok", &self.dcdc_sts_dc_ok())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DcdcStatus {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DcdcStatus {
            dcdc_status_capt_clr: super::vals::DcdcStatusCaptClr,
            dcdc_in_low_vol: super::vals::DcdcInLowVol,
            dcdc_over_cur: super::vals::DcdcOverCur,
            dcdc_over_vol: super::vals::DcdcOverVol,
            dcdc_sts_dc_ok: bool,
        }
        let proxy = DcdcStatus {
            dcdc_status_capt_clr: self.dcdc_status_capt_clr(),
            dcdc_in_low_vol: self.dcdc_in_low_vol(),
            dcdc_over_cur: self.dcdc_over_cur(),
            dcdc_over_vol: self.dcdc_over_vol(),
            dcdc_sts_dc_ok: self.dcdc_sts_dc_ok(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fuse access disable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FuseAccDis(pub u32);
impl FuseAccDis {
    #[doc = "Fuse read disable flag"]
    #[must_use]
    #[inline(always)]
    pub const fn oscca_fuse_read_dis(&self) -> super::vals::OsccaFuseReadDis {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::OsccaFuseReadDis::from_bits(val as u8)
    }
    #[doc = "Fuse read disable flag"]
    #[inline(always)]
    pub const fn set_oscca_fuse_read_dis(&mut self, val: super::vals::OsccaFuseReadDis) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Fuse calibrate flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ocotp_calibrated(&self) -> super::vals::OcotpCalibrated {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::OcotpCalibrated::from_bits(val as u8)
    }
    #[doc = "Fuse calibrate flag"]
    #[inline(always)]
    pub const fn set_ocotp_calibrated(&mut self, val: super::vals::OcotpCalibrated) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "OCOTP busy flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ocotp_busy(&self) -> super::vals::OcotpBusy {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::OcotpBusy::from_bits(val as u8)
    }
    #[doc = "OCOTP busy flag"]
    #[inline(always)]
    pub const fn set_ocotp_busy(&mut self, val: super::vals::OcotpBusy) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
}
impl Default for FuseAccDis {
    #[inline(always)]
    fn default() -> FuseAccDis {
        FuseAccDis(0u64 as u32)
    }
}
impl core::fmt::Debug for FuseAccDis {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FuseAccDis")
            .field("oscca_fuse_read_dis", &self.oscca_fuse_read_dis())
            .field("ocotp_calibrated", &self.ocotp_calibrated())
            .field("ocotp_busy", &self.ocotp_busy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FuseAccDis {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct FuseAccDis {
            oscca_fuse_read_dis: super::vals::OsccaFuseReadDis,
            ocotp_calibrated: super::vals::OcotpCalibrated,
            ocotp_busy: super::vals::OcotpBusy,
        }
        let proxy = FuseAccDis {
            oscca_fuse_read_dis: self.oscca_fuse_read_dis(),
            ocotp_calibrated: self.ocotp_calibrated(),
            ocotp_busy: self.ocotp_busy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "GPC CORE SLEEP Request Select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpcCfg(pub u32);
impl GpcCfg {
    #[doc = "M33 SLEEP Request Select"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_sleep_sel(&self) -> super::vals::M33SleepSel {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::M33SleepSel::from_bits(val as u8)
    }
    #[doc = "M33 SLEEP Request Select"]
    #[inline(always)]
    pub const fn set_m33_sleep_sel(&mut self, val: super::vals::M33SleepSel) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "M7 SLEEP Request Select"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_sleep_sel(&self) -> super::vals::M7SleepSel {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::M7SleepSel::from_bits(val as u8)
    }
    #[doc = "M7 SLEEP Request Select"]
    #[inline(always)]
    pub const fn set_m7_sleep_sel(&mut self, val: super::vals::M7SleepSel) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for GpcCfg {
    #[inline(always)]
    fn default() -> GpcCfg {
        GpcCfg(0u64 as u32)
    }
}
impl core::fmt::Debug for GpcCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GpcCfg")
            .field("m33_sleep_sel", &self.m33_sleep_sel())
            .field("m7_sleep_sel", &self.m7_sleep_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GpcCfg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct GpcCfg {
            m33_sleep_sel: super::vals::M33SleepSel,
            m7_sleep_sel: super::vals::M7SleepSel,
        }
        let proxy = GpcCfg {
            m33_sleep_sel: self.m33_sleep_sel(),
            m7_sleep_sel: self.m7_sleep_sel(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "I3C1 async wakeup control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3c1AsyncWakeupCtrl(pub u32);
impl I3c1AsyncWakeupCtrl {
    #[doc = "Async wakeup interrupt clear"]
    #[must_use]
    #[inline(always)]
    pub const fn irq_clr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Async wakeup interrupt clear"]
    #[inline(always)]
    pub const fn set_irq_clr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Async wakeup interrupt status"]
    #[must_use]
    #[inline(always)]
    pub const fn irq_status(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Async wakeup interrupt status"]
    #[inline(always)]
    pub const fn set_irq_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for I3c1AsyncWakeupCtrl {
    #[inline(always)]
    fn default() -> I3c1AsyncWakeupCtrl {
        I3c1AsyncWakeupCtrl(0u64 as u32)
    }
}
impl core::fmt::Debug for I3c1AsyncWakeupCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3c1AsyncWakeupCtrl")
            .field("irq_clr", &self.irq_clr())
            .field("irq_status", &self.irq_status())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3c1AsyncWakeupCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct I3c1AsyncWakeupCtrl {
            irq_clr: bool,
            irq_status: bool,
        }
        let proxy = I3c1AsyncWakeupCtrl {
            irq_clr: self.irq_clr(),
            irq_status: self.irq_status(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "IPG Debug mask"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IpgDebug(pub u32);
impl IpgDebug {
    #[doc = "Mask bit for CAN1 debug halted mode with M33 core"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_can1(&self) -> super::vals::M33Can1 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::M33Can1::from_bits(val as u8)
    }
    #[doc = "Mask bit for CAN1 debug halted mode with M33 core"]
    #[inline(always)]
    pub const fn set_m33_can1(&mut self, val: super::vals::M33Can1) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Mask bit for EDMA3 debug halted mode with M33 core"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_edma3(&self) -> super::vals::M33Edma3 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::M33Edma3::from_bits(val as u8)
    }
    #[doc = "Mask bit for EDMA3 debug halted mode with M33 core"]
    #[inline(always)]
    pub const fn set_m33_edma3(&mut self, val: super::vals::M33Edma3) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Mask bit for LPI2C1 debug halted mode with M33 core"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_lpi2c1(&self) -> super::vals::M33Lpi2c1 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::M33Lpi2c1::from_bits(val as u8)
    }
    #[doc = "Mask bit for LPI2C1 debug halted mode with M33 core"]
    #[inline(always)]
    pub const fn set_m33_lpi2c1(&mut self, val: super::vals::M33Lpi2c1) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Mask bit for LPI2C2 debug halted mode with M33 core"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_lpi2c2(&self) -> super::vals::M33Lpi2c2 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::M33Lpi2c2::from_bits(val as u8)
    }
    #[doc = "Mask bit for LPI2C2 debug halted mode with M33 core"]
    #[inline(always)]
    pub const fn set_m33_lpi2c2(&mut self, val: super::vals::M33Lpi2c2) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Mask bit for LPIT1 debug halted mode with M33 core"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_lpit1(&self) -> super::vals::M33Lpit1 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::M33Lpit1::from_bits(val as u8)
    }
    #[doc = "Mask bit for LPIT1 debug halted mode with M33 core"]
    #[inline(always)]
    pub const fn set_m33_lpit1(&mut self, val: super::vals::M33Lpit1) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Mask bit for LPSPI1 debug halted mode with M33 core"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_lpspi1(&self) -> super::vals::M33Lpspi1 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::M33Lpspi1::from_bits(val as u8)
    }
    #[doc = "Mask bit for LPSPI1 debug halted mode with M33 core"]
    #[inline(always)]
    pub const fn set_m33_lpspi1(&mut self, val: super::vals::M33Lpspi1) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Mask bit for LPSPI2 debug halted mode with M33 core"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_lpspi2(&self) -> super::vals::M33Lpspi2 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::M33Lpspi2::from_bits(val as u8)
    }
    #[doc = "Mask bit for LPSPI2 debug halted mode with M33 core"]
    #[inline(always)]
    pub const fn set_m33_lpspi2(&mut self, val: super::vals::M33Lpspi2) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Mask bit for LPTMR1 debug halted mode with M33 core"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_lptmr1(&self) -> super::vals::M33Lptmr1 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::M33Lptmr1::from_bits(val as u8)
    }
    #[doc = "Mask bit for LPTMR1 debug halted mode with M33 core"]
    #[inline(always)]
    pub const fn set_m33_lptmr1(&mut self, val: super::vals::M33Lptmr1) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Mask bit for SAI1 debug halted mode with M33 core"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_sai1(&self) -> super::vals::M33Sai1 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::M33Sai1::from_bits(val as u8)
    }
    #[doc = "Mask bit for SAI1 debug halted mode with M33 core"]
    #[inline(always)]
    pub const fn set_m33_sai1(&mut self, val: super::vals::M33Sai1) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Mask bit for TPM1 debug halted mode with M33 core"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_tpm1(&self) -> super::vals::M33Tpm1 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::M33Tpm1::from_bits(val as u8)
    }
    #[doc = "Mask bit for TPM1 debug halted mode with M33 core"]
    #[inline(always)]
    pub const fn set_m33_tpm1(&mut self, val: super::vals::M33Tpm1) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Mask bit for TPM2 debug halted mode with M33 core"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_tpm2(&self) -> super::vals::M33Tpm2 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::M33Tpm2::from_bits(val as u8)
    }
    #[doc = "Mask bit for TPM2 debug halted mode with M33 core"]
    #[inline(always)]
    pub const fn set_m33_tpm2(&mut self, val: super::vals::M33Tpm2) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Mask bit for WDOG1 debug halted mode with M33 core"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_wdog1(&self) -> super::vals::M33Wdog1 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::M33Wdog1::from_bits(val as u8)
    }
    #[doc = "Mask bit for WDOG1 debug halted mode with M33 core"]
    #[inline(always)]
    pub const fn set_m33_wdog1(&mut self, val: super::vals::M33Wdog1) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Mask bit for WDOG2 debug halted mode with M33 core"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_wdog2(&self) -> super::vals::M33Wdog2 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::M33Wdog2::from_bits(val as u8)
    }
    #[doc = "Mask bit for WDOG2 debug halted mode with M33 core"]
    #[inline(always)]
    pub const fn set_m33_wdog2(&mut self, val: super::vals::M33Wdog2) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Mask bit for GPT1 debug halted mode with M33 core"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_gpt1(&self) -> super::vals::M33Gpt1 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::M33Gpt1::from_bits(val as u8)
    }
    #[doc = "Mask bit for GPT1 debug halted mode with M33 core"]
    #[inline(always)]
    pub const fn set_m33_gpt1(&mut self, val: super::vals::M33Gpt1) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Mask bit for CAN3 debug halted mode with M33 core"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_can3(&self) -> super::vals::M33Can3 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::M33Can3::from_bits(val as u8)
    }
    #[doc = "Mask bit for CAN3 debug halted mode with M33 core"]
    #[inline(always)]
    pub const fn set_m33_can3(&mut self, val: super::vals::M33Can3) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Mask bit for I3C1 debug halted mode with M33 core"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_i3c1(&self) -> super::vals::M33I3c1 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::M33I3c1::from_bits(val as u8)
    }
    #[doc = "Mask bit for I3C1 debug halted mode with M33 core"]
    #[inline(always)]
    pub const fn set_m33_i3c1(&mut self, val: super::vals::M33I3c1) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Mask bit for CAN1 debug halted mode with M7 core"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_can1(&self) -> super::vals::M7Can1 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::M7Can1::from_bits(val as u8)
    }
    #[doc = "Mask bit for CAN1 debug halted mode with M7 core"]
    #[inline(always)]
    pub const fn set_m7_can1(&mut self, val: super::vals::M7Can1) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Mask bit for EDMA3 debug halted mode with M7 core"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_edma3(&self) -> super::vals::M7Edma3 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::M7Edma3::from_bits(val as u8)
    }
    #[doc = "Mask bit for EDMA3 debug halted mode with M7 core"]
    #[inline(always)]
    pub const fn set_m7_edma3(&mut self, val: super::vals::M7Edma3) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Mask bit for LPI2C1 debug halted mode with M7 core"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_lpi2c1(&self) -> super::vals::M7Lpi2c1 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::M7Lpi2c1::from_bits(val as u8)
    }
    #[doc = "Mask bit for LPI2C1 debug halted mode with M7 core"]
    #[inline(always)]
    pub const fn set_m7_lpi2c1(&mut self, val: super::vals::M7Lpi2c1) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Mask bit for LPI2C2 debug halted mode with M7 core"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_lpi2c2(&self) -> super::vals::M7Lpi2c2 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::M7Lpi2c2::from_bits(val as u8)
    }
    #[doc = "Mask bit for LPI2C2 debug halted mode with M7 core"]
    #[inline(always)]
    pub const fn set_m7_lpi2c2(&mut self, val: super::vals::M7Lpi2c2) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Mask bit for LPIT1 debug halted mode with M7 core"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_lpit1(&self) -> super::vals::M7Lpit1 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::M7Lpit1::from_bits(val as u8)
    }
    #[doc = "Mask bit for LPIT1 debug halted mode with M7 core"]
    #[inline(always)]
    pub const fn set_m7_lpit1(&mut self, val: super::vals::M7Lpit1) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Mask bit for LPSPI1 debug halted mode with M7 core"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_lpspi1(&self) -> super::vals::M7Lpspi1 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::M7Lpspi1::from_bits(val as u8)
    }
    #[doc = "Mask bit for LPSPI1 debug halted mode with M7 core"]
    #[inline(always)]
    pub const fn set_m7_lpspi1(&mut self, val: super::vals::M7Lpspi1) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Mask bit for LPSPI2 debug halted mode with M7 core"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_lpspi2(&self) -> super::vals::M7Lpspi2 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::M7Lpspi2::from_bits(val as u8)
    }
    #[doc = "Mask bit for LPSPI2 debug halted mode with M7 core"]
    #[inline(always)]
    pub const fn set_m7_lpspi2(&mut self, val: super::vals::M7Lpspi2) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Mask bit for LPTMR1 debug halted mode with M7 core"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_lptmr1(&self) -> super::vals::M7Lptmr1 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::M7Lptmr1::from_bits(val as u8)
    }
    #[doc = "Mask bit for LPTMR1 debug halted mode with M7 core"]
    #[inline(always)]
    pub const fn set_m7_lptmr1(&mut self, val: super::vals::M7Lptmr1) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Mask bit for SAI1 debug halted mode with M7 core"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_sai1(&self) -> super::vals::M7Sai1 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::M7Sai1::from_bits(val as u8)
    }
    #[doc = "Mask bit for SAI1 debug halted mode with M7 core"]
    #[inline(always)]
    pub const fn set_m7_sai1(&mut self, val: super::vals::M7Sai1) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Mask bit for TPM1 debug halted mode with M7 core"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_tpm1(&self) -> super::vals::M7Tpm1 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::M7Tpm1::from_bits(val as u8)
    }
    #[doc = "Mask bit for TPM1 debug halted mode with M7 core"]
    #[inline(always)]
    pub const fn set_m7_tpm1(&mut self, val: super::vals::M7Tpm1) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Mask bit for TPM2 debug halted mode with M7 core"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_tpm2(&self) -> super::vals::M7Tpm2 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::M7Tpm2::from_bits(val as u8)
    }
    #[doc = "Mask bit for TPM2 debug halted mode with M7 core"]
    #[inline(always)]
    pub const fn set_m7_tpm2(&mut self, val: super::vals::M7Tpm2) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Mask bit for WDOG1 debug halted mode with M7 core"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_wdog1(&self) -> super::vals::M7Wdog1 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::M7Wdog1::from_bits(val as u8)
    }
    #[doc = "Mask bit for WDOG1 debug halted mode with M7 core"]
    #[inline(always)]
    pub const fn set_m7_wdog1(&mut self, val: super::vals::M7Wdog1) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Mask bit for WDOG2 debug halted mode with M7 core"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_wdog2(&self) -> super::vals::M7Wdog2 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::M7Wdog2::from_bits(val as u8)
    }
    #[doc = "Mask bit for WDOG2 debug halted mode with M7 core"]
    #[inline(always)]
    pub const fn set_m7_wdog2(&mut self, val: super::vals::M7Wdog2) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Mask bit for GPT1 debug halted mode with M7 core"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_gpt1(&self) -> super::vals::M7Gpt1 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::M7Gpt1::from_bits(val as u8)
    }
    #[doc = "Mask bit for GPT1 debug halted mode with M7 core"]
    #[inline(always)]
    pub const fn set_m7_gpt1(&mut self, val: super::vals::M7Gpt1) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Mask bit for CAN3 debug halted mode with M7 core"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_can3(&self) -> super::vals::M7Can3 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::M7Can3::from_bits(val as u8)
    }
    #[doc = "Mask bit for CAN3 debug halted mode with M7 core"]
    #[inline(always)]
    pub const fn set_m7_can3(&mut self, val: super::vals::M7Can3) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Mask bit for I3C1 debug halted mode with M7 core"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_i3c1(&self) -> super::vals::M7I3c1 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::M7I3c1::from_bits(val as u8)
    }
    #[doc = "Mask bit for I3C1 debug halted mode with M7 core"]
    #[inline(always)]
    pub const fn set_m7_i3c1(&mut self, val: super::vals::M7I3c1) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for IpgDebug {
    #[inline(always)]
    fn default() -> IpgDebug {
        IpgDebug(0u64 as u32)
    }
}
impl core::fmt::Debug for IpgDebug {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IpgDebug")
            .field("m33_can1", &self.m33_can1())
            .field("m33_edma3", &self.m33_edma3())
            .field("m33_lpi2c1", &self.m33_lpi2c1())
            .field("m33_lpi2c2", &self.m33_lpi2c2())
            .field("m33_lpit1", &self.m33_lpit1())
            .field("m33_lpspi1", &self.m33_lpspi1())
            .field("m33_lpspi2", &self.m33_lpspi2())
            .field("m33_lptmr1", &self.m33_lptmr1())
            .field("m33_sai1", &self.m33_sai1())
            .field("m33_tpm1", &self.m33_tpm1())
            .field("m33_tpm2", &self.m33_tpm2())
            .field("m33_wdog1", &self.m33_wdog1())
            .field("m33_wdog2", &self.m33_wdog2())
            .field("m33_gpt1", &self.m33_gpt1())
            .field("m33_can3", &self.m33_can3())
            .field("m33_i3c1", &self.m33_i3c1())
            .field("m7_can1", &self.m7_can1())
            .field("m7_edma3", &self.m7_edma3())
            .field("m7_lpi2c1", &self.m7_lpi2c1())
            .field("m7_lpi2c2", &self.m7_lpi2c2())
            .field("m7_lpit1", &self.m7_lpit1())
            .field("m7_lpspi1", &self.m7_lpspi1())
            .field("m7_lpspi2", &self.m7_lpspi2())
            .field("m7_lptmr1", &self.m7_lptmr1())
            .field("m7_sai1", &self.m7_sai1())
            .field("m7_tpm1", &self.m7_tpm1())
            .field("m7_tpm2", &self.m7_tpm2())
            .field("m7_wdog1", &self.m7_wdog1())
            .field("m7_wdog2", &self.m7_wdog2())
            .field("m7_gpt1", &self.m7_gpt1())
            .field("m7_can3", &self.m7_can3())
            .field("m7_i3c1", &self.m7_i3c1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IpgDebug {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct IpgDebug {
            m33_can1: super::vals::M33Can1,
            m33_edma3: super::vals::M33Edma3,
            m33_lpi2c1: super::vals::M33Lpi2c1,
            m33_lpi2c2: super::vals::M33Lpi2c2,
            m33_lpit1: super::vals::M33Lpit1,
            m33_lpspi1: super::vals::M33Lpspi1,
            m33_lpspi2: super::vals::M33Lpspi2,
            m33_lptmr1: super::vals::M33Lptmr1,
            m33_sai1: super::vals::M33Sai1,
            m33_tpm1: super::vals::M33Tpm1,
            m33_tpm2: super::vals::M33Tpm2,
            m33_wdog1: super::vals::M33Wdog1,
            m33_wdog2: super::vals::M33Wdog2,
            m33_gpt1: super::vals::M33Gpt1,
            m33_can3: super::vals::M33Can3,
            m33_i3c1: super::vals::M33I3c1,
            m7_can1: super::vals::M7Can1,
            m7_edma3: super::vals::M7Edma3,
            m7_lpi2c1: super::vals::M7Lpi2c1,
            m7_lpi2c2: super::vals::M7Lpi2c2,
            m7_lpit1: super::vals::M7Lpit1,
            m7_lpspi1: super::vals::M7Lpspi1,
            m7_lpspi2: super::vals::M7Lpspi2,
            m7_lptmr1: super::vals::M7Lptmr1,
            m7_sai1: super::vals::M7Sai1,
            m7_tpm1: super::vals::M7Tpm1,
            m7_tpm2: super::vals::M7Tpm2,
            m7_wdog1: super::vals::M7Wdog1,
            m7_wdog2: super::vals::M7Wdog2,
            m7_gpt1: super::vals::M7Gpt1,
            m7_can3: super::vals::M7Can3,
            m7_i3c1: super::vals::M7I3c1,
        }
        let proxy = IpgDebug {
            m33_can1: self.m33_can1(),
            m33_edma3: self.m33_edma3(),
            m33_lpi2c1: self.m33_lpi2c1(),
            m33_lpi2c2: self.m33_lpi2c2(),
            m33_lpit1: self.m33_lpit1(),
            m33_lpspi1: self.m33_lpspi1(),
            m33_lpspi2: self.m33_lpspi2(),
            m33_lptmr1: self.m33_lptmr1(),
            m33_sai1: self.m33_sai1(),
            m33_tpm1: self.m33_tpm1(),
            m33_tpm2: self.m33_tpm2(),
            m33_wdog1: self.m33_wdog1(),
            m33_wdog2: self.m33_wdog2(),
            m33_gpt1: self.m33_gpt1(),
            m33_can3: self.m33_can3(),
            m33_i3c1: self.m33_i3c1(),
            m7_can1: self.m7_can1(),
            m7_edma3: self.m7_edma3(),
            m7_lpi2c1: self.m7_lpi2c1(),
            m7_lpi2c2: self.m7_lpi2c2(),
            m7_lpit1: self.m7_lpit1(),
            m7_lpspi1: self.m7_lpspi1(),
            m7_lpspi2: self.m7_lpspi2(),
            m7_lptmr1: self.m7_lptmr1(),
            m7_sai1: self.m7_sai1(),
            m7_tpm1: self.m7_tpm1(),
            m7_tpm2: self.m7_tpm2(),
            m7_wdog1: self.m7_wdog1(),
            m7_wdog2: self.m7_wdog2(),
            m7_gpt1: self.m7_gpt1(),
            m7_can3: self.m7_can3(),
            m7_i3c1: self.m7_i3c1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "M33 NMI interrupt clear register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct M33NmiClr(pub u32);
impl M33NmiClr {
    #[doc = "Clear CM33 NMI holding register"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_nmi_clear(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clear CM33 NMI holding register"]
    #[inline(always)]
    pub const fn set_m33_nmi_clear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for M33NmiClr {
    #[inline(always)]
    fn default() -> M33NmiClr {
        M33NmiClr(0u64 as u32)
    }
}
impl core::fmt::Debug for M33NmiClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M33NmiClr")
            .field("m33_nmi_clear", &self.m33_nmi_clear())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for M33NmiClr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct M33NmiClr {
            m33_nmi_clear: bool,
        }
        let proxy = M33NmiClr {
            m33_nmi_clear: self.m33_nmi_clear(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Miscellaneous control register of IO"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MiscIoCtrl(pub u32);
impl MiscIoCtrl {
    #[doc = "Disable I3C on-chip strong pull for I3C1"]
    #[must_use]
    #[inline(always)]
    pub const fn i3c_on_chip_strong_pull_dis(&self) -> super::vals::I3cOnChipStrongPullDis {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::I3cOnChipStrongPullDis::from_bits(val as u8)
    }
    #[doc = "Disable I3C on-chip strong pull for I3C1"]
    #[inline(always)]
    pub const fn set_i3c_on_chip_strong_pull_dis(
        &mut self,
        val: super::vals::I3cOnChipStrongPullDis,
    ) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "GPIO_AON IO bank supply voltage range selection"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_aon_high_range(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO_AON IO bank supply voltage range selection"]
    #[inline(always)]
    pub const fn set_gpio_aon_high_range(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "GPIO_AON IO bank supply voltage range selection"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_aon_low_range(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO_AON IO bank supply voltage range selection"]
    #[inline(always)]
    pub const fn set_gpio_aon_low_range(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for MiscIoCtrl {
    #[inline(always)]
    fn default() -> MiscIoCtrl {
        MiscIoCtrl(0u64 as u32)
    }
}
impl core::fmt::Debug for MiscIoCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MiscIoCtrl")
            .field(
                "i3c_on_chip_strong_pull_dis",
                &self.i3c_on_chip_strong_pull_dis(),
            )
            .field("gpio_aon_high_range", &self.gpio_aon_high_range())
            .field("gpio_aon_low_range", &self.gpio_aon_low_range())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MiscIoCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MiscIoCtrl {
            i3c_on_chip_strong_pull_dis: super::vals::I3cOnChipStrongPullDis,
            gpio_aon_high_range: bool,
            gpio_aon_low_range: bool,
        }
        let proxy = MiscIoCtrl {
            i3c_on_chip_strong_pull_dis: self.i3c_on_chip_strong_pull_dis(),
            gpio_aon_high_range: self.gpio_aon_high_range(),
            gpio_aon_low_range: self.gpio_aon_low_range(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SAI1 MCLK control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai1MclkCtrl(pub u32);
impl Sai1MclkCtrl {
    #[doc = "SAI1_MCLK IO direction control. IOMUX need select SAI1 MCLK function"]
    #[must_use]
    #[inline(always)]
    pub const fn sai1_mclk_dir(&self) -> super::vals::Sai1MclkDir {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Sai1MclkDir::from_bits(val as u8)
    }
    #[doc = "SAI1_MCLK IO direction control. IOMUX need select SAI1 MCLK function"]
    #[inline(always)]
    pub const fn set_sai1_mclk_dir(&mut self, val: super::vals::Sai1MclkDir) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
}
impl Default for Sai1MclkCtrl {
    #[inline(always)]
    fn default() -> Sai1MclkCtrl {
        Sai1MclkCtrl(0u64 as u32)
    }
}
impl core::fmt::Debug for Sai1MclkCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai1MclkCtrl")
            .field("sai1_mclk_dir", &self.sai1_mclk_dir())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai1MclkCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sai1MclkCtrl {
            sai1_mclk_dir: super::vals::Sai1MclkDir,
        }
        let proxy = Sai1MclkCtrl {
            sai1_mclk_dir: self.sai1_mclk_dir(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SSI Master: AXI Async Bridge from AXIM to NIC400. Low power mode control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssi(pub u32);
impl Ssi {
    #[doc = "AON Domain SSI master pause mode"]
    #[must_use]
    #[inline(always)]
    pub const fn pause_mode(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "AON Domain SSI master pause mode"]
    #[inline(always)]
    pub const fn set_pause_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "AON Domain SSI master blackhole mode"]
    #[must_use]
    #[inline(always)]
    pub const fn blkhole_mode_b(&self) -> super::vals::BlkholeModeB {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::BlkholeModeB::from_bits(val as u8)
    }
    #[doc = "AON Domain SSI master blackhole mode"]
    #[inline(always)]
    pub const fn set_blkhole_mode_b(&mut self, val: super::vals::BlkholeModeB) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for Ssi {
    #[inline(always)]
    fn default() -> Ssi {
        Ssi(2u64 as u32)
    }
}
impl core::fmt::Debug for Ssi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ssi")
            .field("pause_mode", &self.pause_mode())
            .field("blkhole_mode_b", &self.blkhole_mode_b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ssi {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ssi {
            pause_mode: bool,
            blkhole_mode_b: super::vals::BlkholeModeB,
        }
        let proxy = Ssi {
            pause_mode: self.pause_mode(),
            blkhole_mode_b: self.blkhole_mode_b(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
