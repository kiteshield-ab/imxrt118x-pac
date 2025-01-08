#[doc = "ARM_PLL_CTRL_REGISTER"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArmPllCtrl(pub u32);
impl ArmPllCtrl {
    #[doc = "DIV_SELECT"]
    #[must_use]
    #[inline(always)]
    pub const fn div_select(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DIV_SELECT"]
    #[inline(always)]
    pub const fn set_div_select(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "PLL Start up initialization"]
    #[must_use]
    #[inline(always)]
    pub const fn hold_ring_off(&self) -> super::vals::ArmPllCtrlHoldRingOff {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::ArmPllCtrlHoldRingOff::from_bits(val as u8)
    }
    #[doc = "PLL Start up initialization"]
    #[inline(always)]
    pub const fn set_hold_ring_off(&mut self, val: super::vals::ArmPllCtrlHoldRingOff) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Power up the PLL"]
    #[must_use]
    #[inline(always)]
    pub const fn powerup(&self) -> super::vals::ArmPllCtrlPowerup {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::ArmPllCtrlPowerup::from_bits(val as u8)
    }
    #[doc = "Power up the PLL"]
    #[inline(always)]
    pub const fn set_powerup(&mut self, val: super::vals::ArmPllCtrlPowerup) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Enable the clock output."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_clk(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the clock output."]
    #[inline(always)]
    pub const fn set_enable_clk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "POST_DIV_SEL"]
    #[must_use]
    #[inline(always)]
    pub const fn post_div_sel(&self) -> super::vals::PostDivSel {
        let val = (self.0 >> 15usize) & 0x03;
        super::vals::PostDivSel::from_bits(val as u8)
    }
    #[doc = "POST_DIV_SEL"]
    #[inline(always)]
    pub const fn set_post_div_sel(&mut self, val: super::vals::PostDivSel) {
        self.0 = (self.0 & !(0x03 << 15usize)) | (((val.to_bits() as u32) & 0x03) << 15usize);
    }
    #[doc = "Bypass the pll."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> super::vals::ArmPllCtrlBypass {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::ArmPllCtrlBypass::from_bits(val as u8)
    }
    #[doc = "Bypass the pll."]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: super::vals::ArmPllCtrlBypass) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "ARM_PLL_STABLE"]
    #[must_use]
    #[inline(always)]
    pub const fn arm_pll_stable(&self) -> super::vals::ArmPllStable {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::ArmPllStable::from_bits(val as u8)
    }
    #[doc = "ARM_PLL_STABLE"]
    #[inline(always)]
    pub const fn set_arm_pll_stable(&mut self, val: super::vals::ArmPllStable) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "ARM_PLL_GATE"]
    #[must_use]
    #[inline(always)]
    pub const fn arm_pll_gate(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "ARM_PLL_GATE"]
    #[inline(always)]
    pub const fn set_arm_pll_gate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "pll_arm_control_mode"]
    #[must_use]
    #[inline(always)]
    pub const fn arm_pll_control_mode(&self) -> super::vals::ArmPllControlMode {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::ArmPllControlMode::from_bits(val as u8)
    }
    #[doc = "pll_arm_control_mode"]
    #[inline(always)]
    pub const fn set_arm_pll_control_mode(&mut self, val: super::vals::ArmPllControlMode) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for ArmPllCtrl {
    #[inline(always)]
    fn default() -> ArmPllCtrl {
        ArmPllCtrl(1073741990u64 as u32)
    }
}
impl core::fmt::Debug for ArmPllCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ArmPllCtrl")
            .field("div_select", &self.div_select())
            .field("hold_ring_off", &self.hold_ring_off())
            .field("powerup", &self.powerup())
            .field("enable_clk", &self.enable_clk())
            .field("post_div_sel", &self.post_div_sel())
            .field("bypass", &self.bypass())
            .field("arm_pll_stable", &self.arm_pll_stable())
            .field("arm_pll_gate", &self.arm_pll_gate())
            .field("arm_pll_control_mode", &self.arm_pll_control_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ArmPllCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ArmPllCtrl {
            div_select: u8,
            hold_ring_off: super::vals::ArmPllCtrlHoldRingOff,
            powerup: super::vals::ArmPllCtrlPowerup,
            enable_clk: bool,
            post_div_sel: super::vals::PostDivSel,
            bypass: super::vals::ArmPllCtrlBypass,
            arm_pll_stable: super::vals::ArmPllStable,
            arm_pll_gate: bool,
            arm_pll_control_mode: super::vals::ArmPllControlMode,
        }
        let proxy = ArmPllCtrl {
            div_select: self.div_select(),
            hold_ring_off: self.hold_ring_off(),
            powerup: self.powerup(),
            enable_clk: self.enable_clk(),
            post_div_sel: self.post_div_sel(),
            bypass: self.bypass(),
            arm_pll_stable: self.arm_pll_stable(),
            arm_pll_gate: self.arm_pll_gate(),
            arm_pll_control_mode: self.arm_pll_control_mode(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PLL_AUDIO_CTRL_REGISTER"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllAudioCtrl(pub u32);
impl PllAudioCtrl {
    #[doc = "ENABLE_CLK"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_clk(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "ENABLE_CLK"]
    #[inline(always)]
    pub const fn set_enable_clk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "PLL_AUDIO_GATE"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_audio_gate(&self) -> super::vals::PllAudioGate {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::PllAudioGate::from_bits(val as u8)
    }
    #[doc = "PLL_AUDIO_GATE"]
    #[inline(always)]
    pub const fn set_pll_audio_gate(&mut self, val: super::vals::PllAudioGate) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "PLL_AUDIO_STABLE"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_audio_stable(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "PLL_AUDIO_STABLE"]
    #[inline(always)]
    pub const fn set_pll_audio_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "pll_audio_control_mode"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_audio_control_mode(&self) -> super::vals::PllAudioControlMode {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PllAudioControlMode::from_bits(val as u8)
    }
    #[doc = "pll_audio_control_mode"]
    #[inline(always)]
    pub const fn set_pll_audio_control_mode(&mut self, val: super::vals::PllAudioControlMode) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PllAudioCtrl {
    #[inline(always)]
    fn default() -> PllAudioCtrl {
        PllAudioCtrl(16384u64 as u32)
    }
}
impl core::fmt::Debug for PllAudioCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllAudioCtrl")
            .field("enable_clk", &self.enable_clk())
            .field("pll_audio_gate", &self.pll_audio_gate())
            .field("pll_audio_stable", &self.pll_audio_stable())
            .field("pll_audio_control_mode", &self.pll_audio_control_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllAudioCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PllAudioCtrl {
            enable_clk: bool,
            pll_audio_gate: super::vals::PllAudioGate,
            pll_audio_stable: bool,
            pll_audio_control_mode: super::vals::PllAudioControlMode,
        }
        let proxy = PllAudioCtrl {
            enable_clk: self.enable_clk(),
            pll_audio_gate: self.pll_audio_gate(),
            pll_audio_stable: self.pll_audio_stable(),
            pll_audio_control_mode: self.pll_audio_control_mode(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SYS_PLL1_CTRL_REGISTER"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SysPll1Ctrl(pub u32);
impl SysPll1Ctrl {
    #[doc = "ENABLE_CLK"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_clk(&self) -> super::vals::SysPll1CtrlEnableClk {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::SysPll1CtrlEnableClk::from_bits(val as u8)
    }
    #[doc = "ENABLE_CLK"]
    #[inline(always)]
    pub const fn set_enable_clk(&mut self, val: super::vals::SysPll1CtrlEnableClk) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "SYS_PLL1_GATE"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_pll1_gate(&self) -> super::vals::SysPll1Gate {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::SysPll1Gate::from_bits(val as u8)
    }
    #[doc = "SYS_PLL1_GATE"]
    #[inline(always)]
    pub const fn set_sys_pll1_gate(&mut self, val: super::vals::SysPll1Gate) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "SYS_PLL1_DIV2"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_pll1_div2(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "SYS_PLL1_DIV2"]
    #[inline(always)]
    pub const fn set_sys_pll1_div2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "SYS_PLL1_DIV5"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_pll1_div5(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "SYS_PLL1_DIV5"]
    #[inline(always)]
    pub const fn set_sys_pll1_div5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "SYS_PLL1_DIV5_CONTROL_MODE"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_pll1_div5_control_mode(&self) -> super::vals::SysPll1Div5ControlMode {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::SysPll1Div5ControlMode::from_bits(val as u8)
    }
    #[doc = "SYS_PLL1_DIV5_CONTROL_MODE"]
    #[inline(always)]
    pub const fn set_sys_pll1_div5_control_mode(
        &mut self,
        val: super::vals::SysPll1Div5ControlMode,
    ) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "SYS_PLL1_DIV2_CONTROL_MODE"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_pll1_div2_control_mode(&self) -> super::vals::SysPll1Div2ControlMode {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::SysPll1Div2ControlMode::from_bits(val as u8)
    }
    #[doc = "SYS_PLL1_DIV2_CONTROL_MODE"]
    #[inline(always)]
    pub const fn set_sys_pll1_div2_control_mode(
        &mut self,
        val: super::vals::SysPll1Div2ControlMode,
    ) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "SYS_PLL1_STABLE"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_pll1_stable(&self) -> super::vals::SysPll1Stable {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::SysPll1Stable::from_bits(val as u8)
    }
    #[doc = "SYS_PLL1_STABLE"]
    #[inline(always)]
    pub const fn set_sys_pll1_stable(&mut self, val: super::vals::SysPll1Stable) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "SYS_PLL1_CONTROL_MODE"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_pll1_control_mode(&self) -> super::vals::SysPll1ControlMode {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SysPll1ControlMode::from_bits(val as u8)
    }
    #[doc = "SYS_PLL1_CONTROL_MODE"]
    #[inline(always)]
    pub const fn set_sys_pll1_control_mode(&mut self, val: super::vals::SysPll1ControlMode) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SysPll1Ctrl {
    #[inline(always)]
    fn default() -> SysPll1Ctrl {
        SysPll1Ctrl(16384u64 as u32)
    }
}
impl core::fmt::Debug for SysPll1Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SysPll1Ctrl")
            .field("enable_clk", &self.enable_clk())
            .field("sys_pll1_gate", &self.sys_pll1_gate())
            .field("sys_pll1_div2", &self.sys_pll1_div2())
            .field("sys_pll1_div5", &self.sys_pll1_div5())
            .field(
                "sys_pll1_div5_control_mode",
                &self.sys_pll1_div5_control_mode(),
            )
            .field(
                "sys_pll1_div2_control_mode",
                &self.sys_pll1_div2_control_mode(),
            )
            .field("sys_pll1_stable", &self.sys_pll1_stable())
            .field("sys_pll1_control_mode", &self.sys_pll1_control_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SysPll1Ctrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SysPll1Ctrl {
            enable_clk: super::vals::SysPll1CtrlEnableClk,
            sys_pll1_gate: super::vals::SysPll1Gate,
            sys_pll1_div2: bool,
            sys_pll1_div5: bool,
            sys_pll1_div5_control_mode: super::vals::SysPll1Div5ControlMode,
            sys_pll1_div2_control_mode: super::vals::SysPll1Div2ControlMode,
            sys_pll1_stable: super::vals::SysPll1Stable,
            sys_pll1_control_mode: super::vals::SysPll1ControlMode,
        }
        let proxy = SysPll1Ctrl {
            enable_clk: self.enable_clk(),
            sys_pll1_gate: self.sys_pll1_gate(),
            sys_pll1_div2: self.sys_pll1_div2(),
            sys_pll1_div5: self.sys_pll1_div5(),
            sys_pll1_div5_control_mode: self.sys_pll1_div5_control_mode(),
            sys_pll1_div2_control_mode: self.sys_pll1_div2_control_mode(),
            sys_pll1_stable: self.sys_pll1_stable(),
            sys_pll1_control_mode: self.sys_pll1_control_mode(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SYS_PLL2_CTRL_REGISTER"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SysPll2Ctrl(pub u32);
impl SysPll2Ctrl {
    #[doc = "Enable Internal PLL Regulator"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_reg_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Internal PLL Regulator"]
    #[inline(always)]
    pub const fn set_pll_reg_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "PLL Start up initialization"]
    #[must_use]
    #[inline(always)]
    pub const fn hold_ring_off(&self) -> super::vals::SysPll2CtrlHoldRingOff {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::SysPll2CtrlHoldRingOff::from_bits(val as u8)
    }
    #[doc = "PLL Start up initialization"]
    #[inline(always)]
    pub const fn set_hold_ring_off(&mut self, val: super::vals::SysPll2CtrlHoldRingOff) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Enable the clock output."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_clk(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the clock output."]
    #[inline(always)]
    pub const fn set_enable_clk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Bypass the pll."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> super::vals::SysPll2CtrlBypass {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::SysPll2CtrlBypass::from_bits(val as u8)
    }
    #[doc = "Bypass the pll."]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: super::vals::SysPll2CtrlBypass) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "DITHER_ENABLE"]
    #[must_use]
    #[inline(always)]
    pub const fn dither_enable(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "DITHER_ENABLE"]
    #[inline(always)]
    pub const fn set_dither_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "PFD_OFFSET_EN"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd_offset_en(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "PFD_OFFSET_EN"]
    #[inline(always)]
    pub const fn set_pfd_offset_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "PLL_DDR_OVERRIDE"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_ddr_override(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "PLL_DDR_OVERRIDE"]
    #[inline(always)]
    pub const fn set_pll_ddr_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Power up the PLL"]
    #[must_use]
    #[inline(always)]
    pub const fn powerup(&self) -> super::vals::SysPll2CtrlPowerup {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::SysPll2CtrlPowerup::from_bits(val as u8)
    }
    #[doc = "Power up the PLL"]
    #[inline(always)]
    pub const fn set_powerup(&mut self, val: super::vals::SysPll2CtrlPowerup) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "SYS_PLL2_STABLE"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_pll2_stable(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "SYS_PLL2_STABLE"]
    #[inline(always)]
    pub const fn set_sys_pll2_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "SYS_PLL2_GATE"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_pll2_gate(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "SYS_PLL2_GATE"]
    #[inline(always)]
    pub const fn set_sys_pll2_gate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "SYS_PLL2_control_mode"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_pll2_control_mode(&self) -> super::vals::SysPll2ControlMode {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SysPll2ControlMode::from_bits(val as u8)
    }
    #[doc = "SYS_PLL2_control_mode"]
    #[inline(always)]
    pub const fn set_sys_pll2_control_mode(&mut self, val: super::vals::SysPll2ControlMode) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SysPll2Ctrl {
    #[inline(always)]
    fn default() -> SysPll2Ctrl {
        SysPll2Ctrl(1073741824u64 as u32)
    }
}
impl core::fmt::Debug for SysPll2Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SysPll2Ctrl")
            .field("pll_reg_en", &self.pll_reg_en())
            .field("hold_ring_off", &self.hold_ring_off())
            .field("enable_clk", &self.enable_clk())
            .field("bypass", &self.bypass())
            .field("dither_enable", &self.dither_enable())
            .field("pfd_offset_en", &self.pfd_offset_en())
            .field("pll_ddr_override", &self.pll_ddr_override())
            .field("powerup", &self.powerup())
            .field("sys_pll2_stable", &self.sys_pll2_stable())
            .field("sys_pll2_gate", &self.sys_pll2_gate())
            .field("sys_pll2_control_mode", &self.sys_pll2_control_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SysPll2Ctrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SysPll2Ctrl {
            pll_reg_en: bool,
            hold_ring_off: super::vals::SysPll2CtrlHoldRingOff,
            enable_clk: bool,
            bypass: super::vals::SysPll2CtrlBypass,
            dither_enable: bool,
            pfd_offset_en: bool,
            pll_ddr_override: bool,
            powerup: super::vals::SysPll2CtrlPowerup,
            sys_pll2_stable: bool,
            sys_pll2_gate: bool,
            sys_pll2_control_mode: super::vals::SysPll2ControlMode,
        }
        let proxy = SysPll2Ctrl {
            pll_reg_en: self.pll_reg_en(),
            hold_ring_off: self.hold_ring_off(),
            enable_clk: self.enable_clk(),
            bypass: self.bypass(),
            dither_enable: self.dither_enable(),
            pfd_offset_en: self.pfd_offset_en(),
            pll_ddr_override: self.pll_ddr_override(),
            powerup: self.powerup(),
            sys_pll2_stable: self.sys_pll2_stable(),
            sys_pll2_gate: self.sys_pll2_gate(),
            sys_pll2_control_mode: self.sys_pll2_control_mode(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SYS_PLL2_MFD_REGISTER"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SysPll2Mfd(pub u32);
impl SysPll2Mfd {
    #[doc = "Denominator"]
    #[must_use]
    #[inline(always)]
    pub const fn mfd(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Denominator"]
    #[inline(always)]
    pub const fn set_mfd(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
}
impl Default for SysPll2Mfd {
    #[inline(always)]
    fn default() -> SysPll2Mfd {
        SysPll2Mfd(268435455u64 as u32)
    }
}
impl core::fmt::Debug for SysPll2Mfd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SysPll2Mfd")
            .field("mfd", &self.mfd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SysPll2Mfd {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SysPll2Mfd {
            mfd: u32,
        }
        let proxy = SysPll2Mfd { mfd: self.mfd() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SYS_PLL2_MFI_REGISTER"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SysPll2Mfi(pub u32);
impl SysPll2Mfi {
    #[doc = "MFI"]
    #[must_use]
    #[inline(always)]
    pub const fn mfi(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "MFI"]
    #[inline(always)]
    pub const fn set_mfi(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
}
impl Default for SysPll2Mfi {
    #[inline(always)]
    fn default() -> SysPll2Mfi {
        SysPll2Mfi(22u64 as u32)
    }
}
impl core::fmt::Debug for SysPll2Mfi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SysPll2Mfi")
            .field("mfi", &self.mfi())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SysPll2Mfi {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SysPll2Mfi {
            mfi: u8,
        }
        let proxy = SysPll2Mfi { mfi: self.mfi() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SYS_PLL2_MFN_REGISTER"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SysPll2Mfn(pub u32);
impl SysPll2Mfn {
    #[doc = "MFN"]
    #[must_use]
    #[inline(always)]
    pub const fn mfn(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "MFN"]
    #[inline(always)]
    pub const fn set_mfn(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
}
impl Default for SysPll2Mfn {
    #[inline(always)]
    fn default() -> SysPll2Mfn {
        SysPll2Mfn(0u64 as u32)
    }
}
impl core::fmt::Debug for SysPll2Mfn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SysPll2Mfn")
            .field("mfn", &self.mfn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SysPll2Mfn {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SysPll2Mfn {
            mfn: u32,
        }
        let proxy = SysPll2Mfn { mfn: self.mfn() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SYS_PLL2_PFD_REGISTER"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SysPll2Pfd(pub u32);
impl SysPll2Pfd {
    #[doc = "PFD0_FRAC"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_frac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "PFD0_FRAC"]
    #[inline(always)]
    pub const fn set_pfd0_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "PFD0_STABLE"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_stable(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "PFD0_STABLE"]
    #[inline(always)]
    pub const fn set_pfd0_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "PFD0_CLKGATE"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_div1_clkgate(&self) -> super::vals::SysPll2PfdPfd0Div1Clkgate {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::SysPll2PfdPfd0Div1Clkgate::from_bits(val as u8)
    }
    #[doc = "PFD0_CLKGATE"]
    #[inline(always)]
    pub const fn set_pfd0_div1_clkgate(&mut self, val: super::vals::SysPll2PfdPfd0Div1Clkgate) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "PFD1_FRAC"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd1_frac(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "PFD1_FRAC"]
    #[inline(always)]
    pub const fn set_pfd1_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "PFD1_STABLE"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd1_stable(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "PFD1_STABLE"]
    #[inline(always)]
    pub const fn set_pfd1_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "PFD1_CLKGATE"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd1_div1_clkgate(&self) -> super::vals::SysPll2PfdPfd1Div1Clkgate {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::SysPll2PfdPfd1Div1Clkgate::from_bits(val as u8)
    }
    #[doc = "PFD1_CLKGATE"]
    #[inline(always)]
    pub const fn set_pfd1_div1_clkgate(&mut self, val: super::vals::SysPll2PfdPfd1Div1Clkgate) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "PFD2_FRAC"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd2_frac(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "PFD2_FRAC"]
    #[inline(always)]
    pub const fn set_pfd2_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "PFD2_STABLE"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd2_stable(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "PFD2_STABLE"]
    #[inline(always)]
    pub const fn set_pfd2_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "PFD2_CLKGATE"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd2_div1_clkgate(&self) -> super::vals::SysPll2PfdPfd2Div1Clkgate {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::SysPll2PfdPfd2Div1Clkgate::from_bits(val as u8)
    }
    #[doc = "PFD2_CLKGATE"]
    #[inline(always)]
    pub const fn set_pfd2_div1_clkgate(&mut self, val: super::vals::SysPll2PfdPfd2Div1Clkgate) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "PFD3_FRAC"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd3_frac(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "PFD3_FRAC"]
    #[inline(always)]
    pub const fn set_pfd3_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
    #[doc = "PFD3_STABLE"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd3_stable(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "PFD3_STABLE"]
    #[inline(always)]
    pub const fn set_pfd3_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "PFD3_CLKGATE"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd3_div1_clkgate(&self) -> super::vals::SysPll2PfdPfd3Div1Clkgate {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SysPll2PfdPfd3Div1Clkgate::from_bits(val as u8)
    }
    #[doc = "PFD3_CLKGATE"]
    #[inline(always)]
    pub const fn set_pfd3_div1_clkgate(&mut self, val: super::vals::SysPll2PfdPfd3Div1Clkgate) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SysPll2Pfd {
    #[inline(always)]
    fn default() -> SysPll2Pfd {
        SysPll2Pfd(2694353051u64 as u32)
    }
}
impl core::fmt::Debug for SysPll2Pfd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SysPll2Pfd")
            .field("pfd0_frac", &self.pfd0_frac())
            .field("pfd0_stable", &self.pfd0_stable())
            .field("pfd0_div1_clkgate", &self.pfd0_div1_clkgate())
            .field("pfd1_frac", &self.pfd1_frac())
            .field("pfd1_stable", &self.pfd1_stable())
            .field("pfd1_div1_clkgate", &self.pfd1_div1_clkgate())
            .field("pfd2_frac", &self.pfd2_frac())
            .field("pfd2_stable", &self.pfd2_stable())
            .field("pfd2_div1_clkgate", &self.pfd2_div1_clkgate())
            .field("pfd3_frac", &self.pfd3_frac())
            .field("pfd3_stable", &self.pfd3_stable())
            .field("pfd3_div1_clkgate", &self.pfd3_div1_clkgate())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SysPll2Pfd {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SysPll2Pfd {
            pfd0_frac: u8,
            pfd0_stable: bool,
            pfd0_div1_clkgate: super::vals::SysPll2PfdPfd0Div1Clkgate,
            pfd1_frac: u8,
            pfd1_stable: bool,
            pfd1_div1_clkgate: super::vals::SysPll2PfdPfd1Div1Clkgate,
            pfd2_frac: u8,
            pfd2_stable: bool,
            pfd2_div1_clkgate: super::vals::SysPll2PfdPfd2Div1Clkgate,
            pfd3_frac: u8,
            pfd3_stable: bool,
            pfd3_div1_clkgate: super::vals::SysPll2PfdPfd3Div1Clkgate,
        }
        let proxy = SysPll2Pfd {
            pfd0_frac: self.pfd0_frac(),
            pfd0_stable: self.pfd0_stable(),
            pfd0_div1_clkgate: self.pfd0_div1_clkgate(),
            pfd1_frac: self.pfd1_frac(),
            pfd1_stable: self.pfd1_stable(),
            pfd1_div1_clkgate: self.pfd1_div1_clkgate(),
            pfd2_frac: self.pfd2_frac(),
            pfd2_stable: self.pfd2_stable(),
            pfd2_div1_clkgate: self.pfd2_div1_clkgate(),
            pfd3_frac: self.pfd3_frac(),
            pfd3_stable: self.pfd3_stable(),
            pfd3_div1_clkgate: self.pfd3_div1_clkgate(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SYS_PLL2_SS_REGISTER"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SysPll2Ss(pub u32);
impl SysPll2Ss {
    #[doc = "STEP"]
    #[must_use]
    #[inline(always)]
    pub const fn step(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "STEP"]
    #[inline(always)]
    pub const fn set_step(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
    #[doc = "ENABLE"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "STOP"]
    #[must_use]
    #[inline(always)]
    pub const fn stop(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "STOP"]
    #[inline(always)]
    pub const fn set_stop(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for SysPll2Ss {
    #[inline(always)]
    fn default() -> SysPll2Ss {
        SysPll2Ss(0u64 as u32)
    }
}
impl core::fmt::Debug for SysPll2Ss {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SysPll2Ss")
            .field("step", &self.step())
            .field("enable", &self.enable())
            .field("stop", &self.stop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SysPll2Ss {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SysPll2Ss {
            step: u16,
            enable: bool,
            stop: u16,
        }
        let proxy = SysPll2Ss {
            step: self.step(),
            enable: self.enable(),
            stop: self.stop(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SYS_PLL2_UPDATE_REGISTER"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SysPll2Update(pub u32);
impl SysPll2Update {
    #[doc = "PFD0_UPDATE"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_update(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "PFD0_UPDATE"]
    #[inline(always)]
    pub const fn set_pfd0_update(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "PFD1_UPDATE"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd1_update(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "PFD1_UPDATE"]
    #[inline(always)]
    pub const fn set_pfd1_update(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "PFD2_UPDATE"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd2_update(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "PFD2_UPDATE"]
    #[inline(always)]
    pub const fn set_pfd2_update(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "PFD3_UPDATE"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd3_update(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "PFD3_UPDATE"]
    #[inline(always)]
    pub const fn set_pfd3_update(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "pfd0_control_mode"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_control_mode(&self) -> super::vals::SysPll2UpdatePfd0ControlMode {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::SysPll2UpdatePfd0ControlMode::from_bits(val as u8)
    }
    #[doc = "pfd0_control_mode"]
    #[inline(always)]
    pub const fn set_pfd0_control_mode(&mut self, val: super::vals::SysPll2UpdatePfd0ControlMode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "pfd1_control_mode"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd1_control_mode(&self) -> super::vals::SysPll2UpdatePfd1ControlMode {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::SysPll2UpdatePfd1ControlMode::from_bits(val as u8)
    }
    #[doc = "pfd1_control_mode"]
    #[inline(always)]
    pub const fn set_pfd1_control_mode(&mut self, val: super::vals::SysPll2UpdatePfd1ControlMode) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "pfd2_control_mode"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd2_control_mode(&self) -> super::vals::SysPll2UpdatePfd2ControlMode {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::SysPll2UpdatePfd2ControlMode::from_bits(val as u8)
    }
    #[doc = "pfd2_control_mode"]
    #[inline(always)]
    pub const fn set_pfd2_control_mode(&mut self, val: super::vals::SysPll2UpdatePfd2ControlMode) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "pfd3_control_mode"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd3_control_mode(&self) -> super::vals::SysPll2UpdatePfd3ControlMode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::SysPll2UpdatePfd3ControlMode::from_bits(val as u8)
    }
    #[doc = "pfd3_control_mode"]
    #[inline(always)]
    pub const fn set_pfd3_control_mode(&mut self, val: super::vals::SysPll2UpdatePfd3ControlMode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
}
impl Default for SysPll2Update {
    #[inline(always)]
    fn default() -> SysPll2Update {
        SysPll2Update(0u64 as u32)
    }
}
impl core::fmt::Debug for SysPll2Update {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SysPll2Update")
            .field("pfd0_update", &self.pfd0_update())
            .field("pfd1_update", &self.pfd1_update())
            .field("pfd2_update", &self.pfd2_update())
            .field("pfd3_update", &self.pfd3_update())
            .field("pfd0_control_mode", &self.pfd0_control_mode())
            .field("pfd1_control_mode", &self.pfd1_control_mode())
            .field("pfd2_control_mode", &self.pfd2_control_mode())
            .field("pfd3_control_mode", &self.pfd3_control_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SysPll2Update {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SysPll2Update {
            pfd0_update: bool,
            pfd1_update: bool,
            pfd2_update: bool,
            pfd3_update: bool,
            pfd0_control_mode: super::vals::SysPll2UpdatePfd0ControlMode,
            pfd1_control_mode: super::vals::SysPll2UpdatePfd1ControlMode,
            pfd2_control_mode: super::vals::SysPll2UpdatePfd2ControlMode,
            pfd3_control_mode: super::vals::SysPll2UpdatePfd3ControlMode,
        }
        let proxy = SysPll2Update {
            pfd0_update: self.pfd0_update(),
            pfd1_update: self.pfd1_update(),
            pfd2_update: self.pfd2_update(),
            pfd3_update: self.pfd3_update(),
            pfd0_control_mode: self.pfd0_control_mode(),
            pfd1_control_mode: self.pfd1_control_mode(),
            pfd2_control_mode: self.pfd2_control_mode(),
            pfd3_control_mode: self.pfd3_control_mode(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SYS_PLL3_CTRL_REGISTER"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SysPll3Ctrl(pub u32);
impl SysPll3Ctrl {
    #[doc = "DIV_SELECT"]
    #[must_use]
    #[inline(always)]
    pub const fn div_select(&self) -> super::vals::DivSelect {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::DivSelect::from_bits(val as u8)
    }
    #[doc = "DIV_SELECT"]
    #[inline(always)]
    pub const fn set_div_select(&mut self, val: super::vals::DivSelect) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "SYS PLL3 DIV2 gate"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_pll3_div2(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "SYS PLL3 DIV2 gate"]
    #[inline(always)]
    pub const fn set_sys_pll3_div2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable Internal PLL Regulator"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_reg_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Internal PLL Regulator"]
    #[inline(always)]
    pub const fn set_pll_reg_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "PLL Start up initialization"]
    #[must_use]
    #[inline(always)]
    pub const fn hold_ring_off(&self) -> super::vals::SysPll3CtrlHoldRingOff {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::SysPll3CtrlHoldRingOff::from_bits(val as u8)
    }
    #[doc = "PLL Start up initialization"]
    #[inline(always)]
    pub const fn set_hold_ring_off(&mut self, val: super::vals::SysPll3CtrlHoldRingOff) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Enable the clock output."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_clk(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the clock output."]
    #[inline(always)]
    pub const fn set_enable_clk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "BYPASS"]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> super::vals::SysPll3CtrlBypass {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::SysPll3CtrlBypass::from_bits(val as u8)
    }
    #[doc = "BYPASS"]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: super::vals::SysPll3CtrlBypass) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Power up the PLL"]
    #[must_use]
    #[inline(always)]
    pub const fn powerup(&self) -> super::vals::SysPll3CtrlPowerup {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::SysPll3CtrlPowerup::from_bits(val as u8)
    }
    #[doc = "Power up the PLL"]
    #[inline(always)]
    pub const fn set_powerup(&mut self, val: super::vals::SysPll3CtrlPowerup) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "SYS_PLL3_DIV2_CONTROL_MODE"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_pll3_div2_control_mode(&self) -> super::vals::SysPll3Div2ControlMode {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::SysPll3Div2ControlMode::from_bits(val as u8)
    }
    #[doc = "SYS_PLL3_DIV2_CONTROL_MODE"]
    #[inline(always)]
    pub const fn set_sys_pll3_div2_control_mode(
        &mut self,
        val: super::vals::SysPll3Div2ControlMode,
    ) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "SYS_PLL3_STABLE"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_pll3_stable(&self) -> super::vals::SysPll3Stable {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::SysPll3Stable::from_bits(val as u8)
    }
    #[doc = "SYS_PLL3_STABLE"]
    #[inline(always)]
    pub const fn set_sys_pll3_stable(&mut self, val: super::vals::SysPll3Stable) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "SYS_PLL3_GATE"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_pll3_gate(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "SYS_PLL3_GATE"]
    #[inline(always)]
    pub const fn set_sys_pll3_gate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "SYS_PLL3_control_mode"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_pll3_control_mode(&self) -> super::vals::SysPll3ControlMode {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SysPll3ControlMode::from_bits(val as u8)
    }
    #[doc = "SYS_PLL3_control_mode"]
    #[inline(always)]
    pub const fn set_sys_pll3_control_mode(&mut self, val: super::vals::SysPll3ControlMode) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SysPll3Ctrl {
    #[inline(always)]
    fn default() -> SysPll3Ctrl {
        SysPll3Ctrl(1073741827u64 as u32)
    }
}
impl core::fmt::Debug for SysPll3Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SysPll3Ctrl")
            .field("div_select", &self.div_select())
            .field("sys_pll3_div2", &self.sys_pll3_div2())
            .field("pll_reg_en", &self.pll_reg_en())
            .field("hold_ring_off", &self.hold_ring_off())
            .field("enable_clk", &self.enable_clk())
            .field("bypass", &self.bypass())
            .field("powerup", &self.powerup())
            .field(
                "sys_pll3_div2_control_mode",
                &self.sys_pll3_div2_control_mode(),
            )
            .field("sys_pll3_stable", &self.sys_pll3_stable())
            .field("sys_pll3_gate", &self.sys_pll3_gate())
            .field("sys_pll3_control_mode", &self.sys_pll3_control_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SysPll3Ctrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SysPll3Ctrl {
            div_select: super::vals::DivSelect,
            sys_pll3_div2: bool,
            pll_reg_en: bool,
            hold_ring_off: super::vals::SysPll3CtrlHoldRingOff,
            enable_clk: bool,
            bypass: super::vals::SysPll3CtrlBypass,
            powerup: super::vals::SysPll3CtrlPowerup,
            sys_pll3_div2_control_mode: super::vals::SysPll3Div2ControlMode,
            sys_pll3_stable: super::vals::SysPll3Stable,
            sys_pll3_gate: bool,
            sys_pll3_control_mode: super::vals::SysPll3ControlMode,
        }
        let proxy = SysPll3Ctrl {
            div_select: self.div_select(),
            sys_pll3_div2: self.sys_pll3_div2(),
            pll_reg_en: self.pll_reg_en(),
            hold_ring_off: self.hold_ring_off(),
            enable_clk: self.enable_clk(),
            bypass: self.bypass(),
            powerup: self.powerup(),
            sys_pll3_div2_control_mode: self.sys_pll3_div2_control_mode(),
            sys_pll3_stable: self.sys_pll3_stable(),
            sys_pll3_gate: self.sys_pll3_gate(),
            sys_pll3_control_mode: self.sys_pll3_control_mode(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SYS_PLL3_PFD_REGISTER"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SysPll3Pfd(pub u32);
impl SysPll3Pfd {
    #[doc = "PFD0_FRAC"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_frac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "PFD0_FRAC"]
    #[inline(always)]
    pub const fn set_pfd0_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "PFD0_STABLE"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_stable(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "PFD0_STABLE"]
    #[inline(always)]
    pub const fn set_pfd0_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "PFD0_CLKGATE"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_div1_clkgate(&self) -> super::vals::SysPll3PfdPfd0Div1Clkgate {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::SysPll3PfdPfd0Div1Clkgate::from_bits(val as u8)
    }
    #[doc = "PFD0_CLKGATE"]
    #[inline(always)]
    pub const fn set_pfd0_div1_clkgate(&mut self, val: super::vals::SysPll3PfdPfd0Div1Clkgate) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "PFD1_FRAC"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd1_frac(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "PFD1_FRAC"]
    #[inline(always)]
    pub const fn set_pfd1_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "PFD1_STABLE"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd1_stable(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "PFD1_STABLE"]
    #[inline(always)]
    pub const fn set_pfd1_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "PFD1_CLKGATE"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd1_div1_clkgate(&self) -> super::vals::SysPll3PfdPfd1Div1Clkgate {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::SysPll3PfdPfd1Div1Clkgate::from_bits(val as u8)
    }
    #[doc = "PFD1_CLKGATE"]
    #[inline(always)]
    pub const fn set_pfd1_div1_clkgate(&mut self, val: super::vals::SysPll3PfdPfd1Div1Clkgate) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "PFD2_FRAC"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd2_frac(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "PFD2_FRAC"]
    #[inline(always)]
    pub const fn set_pfd2_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "PFD2_STABLE"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd2_stable(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "PFD2_STABLE"]
    #[inline(always)]
    pub const fn set_pfd2_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "PFD2_CLKGATE"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd2_div1_clkgate(&self) -> super::vals::SysPll3PfdPfd2Div1Clkgate {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::SysPll3PfdPfd2Div1Clkgate::from_bits(val as u8)
    }
    #[doc = "PFD2_CLKGATE"]
    #[inline(always)]
    pub const fn set_pfd2_div1_clkgate(&mut self, val: super::vals::SysPll3PfdPfd2Div1Clkgate) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "PFD3_FRAC"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd3_frac(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "PFD3_FRAC"]
    #[inline(always)]
    pub const fn set_pfd3_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
    #[doc = "PFD3_STABLE"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd3_stable(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "PFD3_STABLE"]
    #[inline(always)]
    pub const fn set_pfd3_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "PFD3_CLKGATE"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd3_div1_clkgate(&self) -> super::vals::SysPll3PfdPfd3Div1Clkgate {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SysPll3PfdPfd3Div1Clkgate::from_bits(val as u8)
    }
    #[doc = "PFD3_CLKGATE"]
    #[inline(always)]
    pub const fn set_pfd3_div1_clkgate(&mut self, val: super::vals::SysPll3PfdPfd3Div1Clkgate) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SysPll3Pfd {
    #[inline(always)]
    fn default() -> SysPll3Pfd {
        SysPll3Pfd(2359333261u64 as u32)
    }
}
impl core::fmt::Debug for SysPll3Pfd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SysPll3Pfd")
            .field("pfd0_frac", &self.pfd0_frac())
            .field("pfd0_stable", &self.pfd0_stable())
            .field("pfd0_div1_clkgate", &self.pfd0_div1_clkgate())
            .field("pfd1_frac", &self.pfd1_frac())
            .field("pfd1_stable", &self.pfd1_stable())
            .field("pfd1_div1_clkgate", &self.pfd1_div1_clkgate())
            .field("pfd2_frac", &self.pfd2_frac())
            .field("pfd2_stable", &self.pfd2_stable())
            .field("pfd2_div1_clkgate", &self.pfd2_div1_clkgate())
            .field("pfd3_frac", &self.pfd3_frac())
            .field("pfd3_stable", &self.pfd3_stable())
            .field("pfd3_div1_clkgate", &self.pfd3_div1_clkgate())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SysPll3Pfd {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SysPll3Pfd {
            pfd0_frac: u8,
            pfd0_stable: bool,
            pfd0_div1_clkgate: super::vals::SysPll3PfdPfd0Div1Clkgate,
            pfd1_frac: u8,
            pfd1_stable: bool,
            pfd1_div1_clkgate: super::vals::SysPll3PfdPfd1Div1Clkgate,
            pfd2_frac: u8,
            pfd2_stable: bool,
            pfd2_div1_clkgate: super::vals::SysPll3PfdPfd2Div1Clkgate,
            pfd3_frac: u8,
            pfd3_stable: bool,
            pfd3_div1_clkgate: super::vals::SysPll3PfdPfd3Div1Clkgate,
        }
        let proxy = SysPll3Pfd {
            pfd0_frac: self.pfd0_frac(),
            pfd0_stable: self.pfd0_stable(),
            pfd0_div1_clkgate: self.pfd0_div1_clkgate(),
            pfd1_frac: self.pfd1_frac(),
            pfd1_stable: self.pfd1_stable(),
            pfd1_div1_clkgate: self.pfd1_div1_clkgate(),
            pfd2_frac: self.pfd2_frac(),
            pfd2_stable: self.pfd2_stable(),
            pfd2_div1_clkgate: self.pfd2_div1_clkgate(),
            pfd3_frac: self.pfd3_frac(),
            pfd3_stable: self.pfd3_stable(),
            pfd3_div1_clkgate: self.pfd3_div1_clkgate(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SYS_PLL3_UPDATE_REGISTER"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SysPll3Update(pub u32);
impl SysPll3Update {
    #[doc = "PFD0_OVERRIDE"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_update(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "PFD0_OVERRIDE"]
    #[inline(always)]
    pub const fn set_pfd0_update(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "PFD1_OVERRIDE"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd1_update(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "PFD1_OVERRIDE"]
    #[inline(always)]
    pub const fn set_pfd1_update(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "PFD2_OVERRIDE"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd2_update(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "PFD2_OVERRIDE"]
    #[inline(always)]
    pub const fn set_pfd2_update(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "PFD3_UPDATE"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd3_update(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "PFD3_UPDATE"]
    #[inline(always)]
    pub const fn set_pfd3_update(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "pfd0_control_mode"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_control_mode(&self) -> super::vals::SysPll3UpdatePfd0ControlMode {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::SysPll3UpdatePfd0ControlMode::from_bits(val as u8)
    }
    #[doc = "pfd0_control_mode"]
    #[inline(always)]
    pub const fn set_pfd0_control_mode(&mut self, val: super::vals::SysPll3UpdatePfd0ControlMode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "pfd1_control_mode"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd1_control_mode(&self) -> super::vals::SysPll3UpdatePfd1ControlMode {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::SysPll3UpdatePfd1ControlMode::from_bits(val as u8)
    }
    #[doc = "pfd1_control_mode"]
    #[inline(always)]
    pub const fn set_pfd1_control_mode(&mut self, val: super::vals::SysPll3UpdatePfd1ControlMode) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "pdf2_control_mode"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd2_control_mode(&self) -> super::vals::SysPll3UpdatePfd2ControlMode {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::SysPll3UpdatePfd2ControlMode::from_bits(val as u8)
    }
    #[doc = "pdf2_control_mode"]
    #[inline(always)]
    pub const fn set_pfd2_control_mode(&mut self, val: super::vals::SysPll3UpdatePfd2ControlMode) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "pfd3_control_mode"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd3_control_mode(&self) -> super::vals::SysPll3UpdatePfd3ControlMode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::SysPll3UpdatePfd3ControlMode::from_bits(val as u8)
    }
    #[doc = "pfd3_control_mode"]
    #[inline(always)]
    pub const fn set_pfd3_control_mode(&mut self, val: super::vals::SysPll3UpdatePfd3ControlMode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
}
impl Default for SysPll3Update {
    #[inline(always)]
    fn default() -> SysPll3Update {
        SysPll3Update(0u64 as u32)
    }
}
impl core::fmt::Debug for SysPll3Update {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SysPll3Update")
            .field("pfd0_update", &self.pfd0_update())
            .field("pfd1_update", &self.pfd1_update())
            .field("pfd2_update", &self.pfd2_update())
            .field("pfd3_update", &self.pfd3_update())
            .field("pfd0_control_mode", &self.pfd0_control_mode())
            .field("pfd1_control_mode", &self.pfd1_control_mode())
            .field("pfd2_control_mode", &self.pfd2_control_mode())
            .field("pfd3_control_mode", &self.pfd3_control_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SysPll3Update {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SysPll3Update {
            pfd0_update: bool,
            pfd1_update: bool,
            pfd2_update: bool,
            pfd3_update: bool,
            pfd0_control_mode: super::vals::SysPll3UpdatePfd0ControlMode,
            pfd1_control_mode: super::vals::SysPll3UpdatePfd1ControlMode,
            pfd2_control_mode: super::vals::SysPll3UpdatePfd2ControlMode,
            pfd3_control_mode: super::vals::SysPll3UpdatePfd3ControlMode,
        }
        let proxy = SysPll3Update {
            pfd0_update: self.pfd0_update(),
            pfd1_update: self.pfd1_update(),
            pfd2_update: self.pfd2_update(),
            pfd3_update: self.pfd3_update(),
            pfd0_control_mode: self.pfd0_control_mode(),
            pfd1_control_mode: self.pfd1_control_mode(),
            pfd2_control_mode: self.pfd2_control_mode(),
            pfd3_control_mode: self.pfd3_control_mode(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
